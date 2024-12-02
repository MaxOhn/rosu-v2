use crate::OsuResult;

use super::{OsuRef, Scopes};

use serde::Deserialize;
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{
    sync::oneshot::{self, Receiver},
    time::sleep,
};

/// Token to interact with the osu! API.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Token {
    pub(crate) access: Option<Box<str>>,
    pub(crate) refresh: Option<Box<str>>,
}

impl Token {
    /// Value used to access the API.
    ///
    /// `None` if the token has not been refreshed.
    pub fn access(&self) -> Option<&str> {
        self.access.as_deref()
    }

    /// Value used to refresh the token.
    pub fn refresh(&self) -> Option<&str> {
        self.refresh.as_deref()
    }

    /// Create a new [`Token`] with the given values.
    pub fn new(access: &str, refresh: Option<Box<str>>) -> Self {
        let mut token = Self::default();
        token.update(access, refresh);

        token
    }

    pub(super) fn update(&mut self, access: &str, refresh: Option<Box<str>>) {
        let access = if access.starts_with("Bearer ") {
            access.into()
        } else {
            format!("Bearer {access}").into_boxed_str()
        };

        self.access = Some(access);
        self.refresh = refresh;
    }

    pub(super) fn update_worker(
        osu: Arc<OsuRef>,
        auth_kind: AuthorizationKind,
        mut expire: i64,
        mut dropped_rx: Receiver<()>,
    ) {
        tokio::spawn(async move {
            loop {
                // In case acquiring a new token takes too long,
                // remove the previous token as soon as it expires
                // so that new requests will not be sent until
                // a new token has been acquired
                let (expire_tx, expire_rx) = oneshot::channel::<()>();

                tokio::select! {
                    _ = &mut dropped_rx => {
                        let _ = expire_tx.send(());
                        return debug!("Osu dropped; exiting token update loop");
                    }
                    token = Self::update_routine(Arc::clone(&osu), &auth_kind, expire, expire_rx) => {
                        let _ = expire_tx.send(());
                        debug!("Successfully acquired new token");

                        expire = token.expires_in;
                        osu.token.write().await.update(token.access_token.as_ref(), token.refresh_token);
                    }
                }
            }
        });
    }

    async fn update_routine(
        osu: Arc<OsuRef>,
        auth_kind: &AuthorizationKind,
        expire: i64,
        mut expire_rx: Receiver<()>,
    ) -> TokenResponse {
        let osu_clone = Arc::clone(&osu);
        tokio::spawn(async move {
            tokio::select! {
                _ = &mut expire_rx => {}
                _ = sleep(Duration::from_secs(expire.max(0) as u64)) => {
                    warn!("Acquiring new token took too long, removed current token");
                    osu_clone.token.write().await.access.take();
                }
            }
        });

        let adjusted_expire = adjust_token_expire(expire);
        debug!("Acquire new API token in {} seconds", adjusted_expire);

        sleep(Duration::from_secs(adjusted_expire.max(0) as u64)).await;
        debug!("API token expired, acquiring new one...");

        Token::request_loop(&osu, auth_kind).await
    }

    // Acquire a new token through exponential backoff
    async fn request_loop(osu: &OsuRef, auth_kind: &AuthorizationKind) -> TokenResponse {
        let mut backoff = 400;

        loop {
            match auth_kind.request_token(osu).await {
                Ok(token) if token.token_type.as_ref() == "Bearer" => return token,
                Ok(token) => {
                    warn!(
                        r#"Failed to acquire new token, "{}" != "Bearer"; retry in {backoff}ms"#,
                        token.token_type
                    );
                }
                Err(err) => {
                    warn!(?err, "Failed to acquire new token; retry in {backoff}ms");

                    let mut err: &dyn Error = &err;

                    while let Some(src) = err.source() {
                        warn!("  - caused by: {src}");
                        err = src;
                    }
                }
            }

            sleep(Duration::from_millis(backoff)).await;
            backoff = (backoff * 2).min(60_000);
        }
    }
}

#[inline]
fn adjust_token_expire(expires_in: i64) -> i64 {
    expires_in - (expires_in as f64 * 0.05) as i64
}

pub(super) enum AuthorizationBuilder {
    Kind(AuthorizationKind),
    #[cfg(feature = "local_oauth")]
    LocalOauth {
        redirect_uri: String,
        scopes: Scopes,
    },
    Given {
        token: Token,
        expires_in: Option<i64>,
    },
}

impl AuthorizationBuilder {
    #[cfg(feature = "local_oauth")]
    pub(super) async fn perform_local_oauth(
        redirect_uri: String,
        client_id: u64,
        scopes: Scopes,
    ) -> Result<Authorization, crate::error::OAuthError> {
        use std::{
            io::{Error as IoError, ErrorKind},
            str::from_utf8 as str_from_utf8,
        };
        use tokio::{
            io::AsyncWriteExt,
            net::{TcpListener, TcpStream},
        };

        use crate::error::OAuthError;

        let port: u16 = redirect_uri
            .rsplit_once(':')
            .and_then(|(prefix, suffix)| {
                suffix
                    .split('/')
                    .next()
                    .filter(|_| prefix.ends_with("localhost"))
            })
            .map(str::parse)
            .and_then(Result::ok)
            .ok_or(OAuthError::Url)?;

        let listener = TcpListener::bind(("localhost", port))
            .await
            .map_err(OAuthError::Listener)?;

        let mut url = format!(
            "https://osu.ppy.sh/oauth/authorize?\
                client_id={client_id}\
                &redirect_uri={redirect_uri}\
                &response_type=code",
        );

        url.push_str("&scope=");
        scopes.format(&mut url, '+');

        println!("Authorize yourself through the following url:\n{url}");
        info!("Awaiting manual authorization...");

        let (mut stream, _) = listener.accept().await.map_err(OAuthError::Accept)?;
        let mut data = Vec::new();

        loop {
            stream.readable().await.map_err(OAuthError::Read)?;

            match stream.try_read_buf(&mut data) {
                Ok(0) => break,
                Ok(_) => {
                    if data.ends_with(b"\n\n") || data.ends_with(b"\r\n\r\n") {
                        break;
                    }
                }
                Err(ref e) if e.kind() == ErrorKind::WouldBlock => continue,
                Err(e) => return Err(OAuthError::Read(e)),
            }
        }

        let code = str_from_utf8(&data)
            .ok()
            .and_then(|data| {
                const KEY: &str = "code=";

                if let Some(mut start) = data.find(KEY) {
                    start += KEY.len();

                    if let Some(end) = data[start..].find(char::is_whitespace) {
                        return Some(Box::from(&data[start..][..end]));
                    }
                }

                None
            })
            .ok_or(OAuthError::NoCode { data })?;

        info!("Authorization succeeded");

        #[allow(clippy::items_after_statements)]
        async fn respond(stream: &mut TcpStream) -> Result<(), IoError> {
            let response = b"HTTP/1.0 200 OK
Content-Type: text/html

<html><body>
<h2>rosu-v2 authentication succeeded</h2>
You may close this tab
</body></html>";

            stream.writable().await?;
            stream.write_all(response).await?;
            stream.shutdown().await?;

            Ok(())
        }

        respond(&mut stream).await.map_err(OAuthError::Write)?;

        Ok(Authorization {
            code,
            redirect_uri: redirect_uri.into_boxed_str(),
            scopes,
        })
    }
}

pub(super) enum AuthorizationKind {
    User(Authorization),
    Client,
    BareToken,
}

impl AuthorizationKind {
    pub async fn request_token(&self, osu: &OsuRef) -> OsuResult<TokenResponse> {
        match self {
            AuthorizationKind::User(auth) => match osu.token.read().await.refresh {
                Some(ref refresh) => osu.request_refresh_token(refresh).await,
                None => osu.request_user_token(auth).await,
            },
            AuthorizationKind::Client => osu.request_client_token().await,
            AuthorizationKind::BareToken => {
                let Some(ref refresh) = osu.token.read().await.refresh else {
                    error!("Missing refresh token on bare authentication; all future requests will fail");

                    futures::future::pending::<()>().await;
                    unreachable!();
                };

                osu.request_refresh_token(refresh).await
            }
        }
    }
}

impl Default for AuthorizationKind {
    fn default() -> Self {
        Self::Client
    }
}

pub(super) struct Authorization {
    pub code: Box<str>,
    pub redirect_uri: Box<str>,
    pub scopes: Scopes,
}

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub access_token: Box<str>,
    pub expires_in: i64,
    #[serde(default)]
    pub refresh_token: Option<Box<str>>,
    pub token_type: Box<str>,
}
