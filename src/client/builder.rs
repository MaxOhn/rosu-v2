use super::{
    token::AuthorizationBuilder, Authorization, AuthorizationKind, Osu, OsuRef, Scopes, Token,
};
use crate::{error::OsuError, OsuResult};

use hyper::client::Builder;
use hyper_rustls::HttpsConnectorBuilder;
use leaky_bucket_lite::LeakyBucket;
use std::{sync::Arc, time::Duration};
use tokio::sync::{oneshot, RwLock};

#[cfg(feature = "cache")]
use dashmap::DashMap;

/// Builder struct for an [`Osu`](crate::Osu) client.
///
/// `client_id` as well as `client_secret` **must** be specified before building.
///
/// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
#[must_use]
pub struct OsuBuilder {
    auth: Option<AuthorizationBuilder>,
    client_id: Option<u64>,
    client_secret: Option<String>,
    retries: usize,
    timeout: Duration,
    per_second: u32,
}

impl Default for OsuBuilder {
    fn default() -> Self {
        Self {
            auth: None,
            client_id: None,
            client_secret: None,
            retries: 2,
            timeout: Duration::from_secs(10),
            per_second: 15,
        }
    }
}

impl OsuBuilder {
    /// Create a new [`OsuBuilder`](crate::OsuBuilder)
    pub fn new() -> Self {
        Self::default()
    }

    /// Build an [`Osu`] client.
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

        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .build();

        let http = Builder::default().build(connector);

        let ratelimiter = LeakyBucket::builder()
            .max(self.per_second)
            .tokens(self.per_second)
            .refill_interval(Duration::from_millis(1000 / u64::from(self.per_second)))
            .refill_amount(1)
            .build();

        let inner = Arc::new(OsuRef {
            client_id,
            client_secret: client_secret.into_boxed_str(),
            http,
            ratelimiter,
            timeout: self.timeout,
            token: RwLock::new(Token::default()),
            retries: self.retries,
        });

        #[cfg(feature = "metrics")]
        crate::metrics::init_metrics();

        match self.auth {
            Some(AuthorizationBuilder::Kind(kind)) => build_with_refresh(inner, kind).await,
            #[cfg(feature = "local_oauth")]
            Some(AuthorizationBuilder::LocalOauth {
                redirect_uri,
                scopes,
            }) => {
                let auth_kind =
                    AuthorizationBuilder::perform_local_oauth(redirect_uri, client_id, scopes)
                        .await
                        .map(AuthorizationKind::User)?;

                build_with_refresh(inner, auth_kind).await
            }

            Some(AuthorizationBuilder::Given {
                token,
                expires_in: Some(expires_in),
            }) => {
                let (tx, dropped_rx) = oneshot::channel();

                *inner.token.write().await = token;
                let auth_kind = AuthorizationKind::BareToken;

                // Let an async worker update the token regularly
                Token::update_worker(Arc::clone(&inner), auth_kind, expires_in, dropped_rx);

                Ok(Osu {
                    inner,
                    token_loop_tx: Some(tx),

                    #[cfg(feature = "cache")]
                    cache: Box::new(DashMap::new()),
                })
            }
            Some(AuthorizationBuilder::Given { token, .. }) => {
                *inner.token.write().await = token;

                Ok(Osu {
                    inner,
                    token_loop_tx: None,

                    #[cfg(feature = "cache")]
                    cache: Box::new(DashMap::new()),
                })
            }
            None => build_with_refresh(inner, AuthorizationKind::default()).await,
        }
    }

    /// Set the client id of the application.
    ///
    /// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
    pub const fn client_id(mut self, client_id: u64) -> Self {
        self.client_id = Some(client_id);

        self
    }

    /// Set the client secret of the application.
    ///
    /// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());

        self
    }

    /// Upon calling [`build`], `rosu-v2` will print a url to authorize a local
    /// osu! profile.
    ///
    /// Be sure that the specified client id matches the OAuth application's
    /// redirect uri.
    ///
    /// If the authorization code has already been acquired, use
    /// [`with_authorization`] instead.
    ///
    /// For more info, check out
    /// <https://osu.ppy.sh/docs/index.html#authorization-code-grant>
    ///
    /// [`build`]: OsuBuilder::build
    /// [`with_authorization`]: OsuBuilder::with_authorization
    #[cfg(feature = "local_oauth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "local_oauth")))]
    pub fn with_local_authorization(
        mut self,
        redirect_uri: impl Into<String>,
        scopes: Scopes,
    ) -> Self {
        self.auth = Some(AuthorizationBuilder::LocalOauth {
            redirect_uri: redirect_uri.into(),
            scopes,
        });

        self
    }

    /// After acquiring the authorization code from a user through OAuth,
    /// use this method to provide the given code, and specified redirect uri.
    ///
    /// To perform the full OAuth procedure for a local osu! profile, enable the
    /// `local_oauth` feature and use `OsuBuilder::with_local_authorization`
    /// instead.
    ///
    /// For more info, check out
    /// <https://osu.ppy.sh/docs/index.html#authorization-code-grant>
    pub fn with_authorization(
        mut self,
        code: impl Into<String>,
        redirect_uri: impl Into<String>,
        scopes: Scopes,
    ) -> Self {
        let authorization = Authorization {
            code: code.into().into_boxed_str(),
            redirect_uri: redirect_uri.into().into_boxed_str(),
            scopes,
        };

        self.auth = Some(AuthorizationBuilder::Kind(AuthorizationKind::User(
            authorization,
        )));

        self
    }

    /// Instead of acquiring a token upon building the client, use the given
    /// token.
    ///
    /// If `Token::refresh` and `expires_in` are `Some`, the token will be
    /// refreshed automatically.
    ///
    /// For more info, check out
    /// <https://osu.ppy.sh/docs/index.html#authorization-code-grant>
    pub fn with_token(mut self, token: Token, expires_in: Option<i64>) -> Self {
        self.auth = Some(AuthorizationBuilder::Given { token, expires_in });

        self
    }

    /// In case the request times out, retry up to this many times, defaults to 2.
    pub const fn retries(mut self, retries: usize) -> Self {
        self.retries = retries;

        self
    }

    /// Set the timeout for requests, defaults to 10 seconds.
    pub const fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }

    /// Set the amount of requests that can be made in one second, defaults to 15.
    /// The given value will be clamped between 1 and 20.
    ///
    /// Check out the osu!api's [terms of use] for acceptable values.
    ///
    /// [terms of use]: https://osu.ppy.sh/docs/index.html#terms-of-use
    pub fn ratelimit(mut self, reqs_per_sec: u32) -> Self {
        self.per_second = reqs_per_sec.clamp(1, 20);

        self
    }
}

async fn build_with_refresh(inner: Arc<OsuRef>, auth_kind: AuthorizationKind) -> OsuResult<Osu> {
    let (tx, dropped_rx) = oneshot::channel();

    // Acquire the initial API token
    let token = auth_kind
        .request_token(&inner)
        .await
        .map_err(Box::new)
        .map_err(|source| OsuError::UpdateToken { source })?;

    let expires_in = token.expires_in;
    inner
        .token
        .write()
        .await
        .update(token.access_token.as_ref(), token.refresh_token);

    // Let an async worker update the token regularly
    Token::update_worker(Arc::clone(&inner), auth_kind, expires_in, dropped_rx);

    Ok(Osu {
        inner,
        token_loop_tx: Some(tx),

        #[cfg(feature = "cache")]
        cache: Box::new(DashMap::new()),
    })
}
