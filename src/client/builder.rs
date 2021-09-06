use super::{Authorization, AuthorizationKind, Osu, OsuRef, Token};
use crate::{error::OsuError, ratelimiter::Ratelimiter, OsuResult};

use hyper::client::Builder;
use hyper_rustls::HttpsConnector;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        oneshot::{self, error::TryRecvError, Receiver},
        RwLock,
    },
    time::sleep,
};

#[cfg(feature = "cache")]
use dashmap::DashMap;

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

/// Builder struct for an [`Osu`](crate::Osu) client.
///
/// `client_id` as well as `client_secret` **must** be specified before building.
///
/// For more info, check out https://osu.ppy.sh/docs/index.html#client-credentials-grant
pub struct OsuBuilder {
    auth_kind: Option<AuthorizationKind>,
    client_id: Option<u64>,
    client_secret: Option<String>,
    timeout: Duration,
}

impl Default for OsuBuilder {
    fn default() -> Self {
        Self {
            auth_kind: None,
            client_id: None,
            client_secret: None,
            timeout: Duration::from_secs(10),
        }
    }
}

impl OsuBuilder {
    /// Create a new [`OsuBuilder`](crate::OsuBuilder)
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an [`Osu`](crate::Osu) client.
    ///
    /// To build the client, the client id and secret are being used
    /// to acquire a token from the API which expires after a certain time.
    /// The client will from then on update the token regularly on its own.
    ///
    /// # Errors
    ///
    /// Returns an error if
    ///   - client id was not set
    ///   - client secret was not set
    ///   - API did not provide a token for the given client id and client secret
    pub async fn build(self) -> OsuResult<Osu> {
        let client_id = self.client_id.ok_or(OsuError::BuilderMissingId)?;
        let client_secret = self.client_secret.ok_or(OsuError::BuilderMissingSecret)?;

        let connector = HttpsConnector::with_native_roots();
        let http = Builder::default().build(connector);

        let ratelimiter = Ratelimiter::new(15, 1);
        let (tx, rx) = oneshot::channel();

        let inner = Arc::new(OsuRef {
            client_id,
            client_secret,
            http,
            ratelimiter,
            timeout: self.timeout,
            auth_kind: self.auth_kind.unwrap_or_default(),
            token: RwLock::new(Token::default()),
            token_loop_tx: Some(tx),
        });

        // Acquire the initial API token
        let token = inner
            .request_token()
            .await
            .map_err(Box::new)
            .map_err(|source| OsuError::UpdateToken { source })?;

        let expires_in = token.expires_in;
        inner.token.write().await.update(token);

        // Let an async worker update the token regularly
        token_update_worker(Arc::clone(&inner), expires_in, rx);

        Ok(Osu {
            inner,

            #[cfg(feature = "cache")]
            cache: Arc::new(DashMap::new()),

            #[cfg(feature = "metrics")]
            metrics: Arc::new(Metrics::new()),
        })
    }

    /// Set the client id of the application.
    ///
    /// For more info, check out https://osu.ppy.sh/docs/index.html#client-credentials-grant
    #[inline]
    pub fn client_id(mut self, client_id: u64) -> Self {
        self.client_id.replace(client_id);

        self
    }

    /// Set the client secret of the application.
    ///
    /// For more info, check out https://osu.ppy.sh/docs/index.html#client-credentials-grant
    #[inline]
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret.replace(client_secret.into());

        self
    }

    /// After acquiring the authorization code from a user through oauth,
    /// use this method to provide the given code, and specified redirect uri.
    ///
    /// For more info, check out https://osu.ppy.sh/docs/index.html#authorization-code-grant
    pub fn with_authorization(
        mut self,
        code: impl Into<String>,
        redirect_uri: impl Into<String>,
    ) -> Self {
        let authorization = Authorization {
            code: code.into(),
            redirect_uri: redirect_uri.into(),
        };

        self.auth_kind = Some(AuthorizationKind::User(authorization));

        self
    }

    /// Set the timeout for HTTP requests, defaults to 10 seconds.
    #[inline]
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }
}

#[inline]
fn adjust_token_expire(expires_in: u64) -> u64 {
    expires_in - (expires_in as f64 * 0.05) as u64
}

fn token_update_worker(osu: Arc<OsuRef>, mut actual_expire: u64, mut rx: Receiver<()>) {
    let mut adjusted_expire = adjust_token_expire(actual_expire);

    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(adjusted_expire)).await;

            if matches!(rx.try_recv(), Ok(_) | Err(TryRecvError::Closed)) {
                return debug!("Exiting token update loop");
            }

            // In case acquiring a new token takes too long,
            // remove the previous token as soon as it expires
            // so that new requests will not be sent until
            // a new token has been acquired
            let (expire_tx, expire_rx) = oneshot::channel::<()>();
            let osu_clone = Arc::clone(&osu);

            tokio::spawn(async move {
                tokio::select!(
                    _ = expire_rx => {}
                    _ = sleep(Duration::from_secs(actual_expire)) => {
                        warn!("Acquiring new token took too long, remove current token");
                        osu_clone.token.write().await.access.take();
                    }
                )
            });

            // Acquire a new token through exponential backoff
            let mut backoff = 200;
            info!("API token expired, acquire new one");

            loop {
                match osu.request_token().await {
                    Ok(token) if token.token_type == "Bearer" => {
                        actual_expire = token.expires_in;
                        adjusted_expire = adjust_token_expire(actual_expire);

                        osu.token.write().await.update(token);

                        break;
                    }
                    Ok(token) => {
                        warn!(
                            "Failed to acquire new token, {:?} != \"Bearer\"; retry in {}ms",
                            token.token_type, backoff
                        );
                    }
                    Err(why) => {
                        warn!(
                            "Failed to acquire new token: {}; retry in {}ms",
                            why, backoff
                        );
                    }
                }

                sleep(Duration::from_millis(backoff)).await;
                backoff = (backoff * 2).min(60_000);
            }

            let _ = expire_tx.send(());
        }
    });
}
