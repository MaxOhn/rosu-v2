use super::{Authorization, AuthorizationKind, Osu, OsuRef, Token};
use crate::{error::OsuError, OsuResult};

use hyper::client::Builder;
use hyper_rustls::HttpsConnectorBuilder;
use leaky_bucket_lite::LeakyBucket;
use std::{sync::Arc, time::Duration};
use tokio::sync::{oneshot, RwLock};

#[cfg(feature = "cache")]
use dashmap::DashMap;

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

/// Builder struct for an [`Osu`](crate::Osu) client.
///
/// `client_id` as well as `client_secret` **must** be specified before building.
///
/// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
pub struct OsuBuilder {
    auth_kind: Option<AuthorizationKind>,
    client_id: Option<u64>,
    client_secret: Option<String>,
    retries: usize,
    timeout: Duration,
    per_second: u32,
}

impl Default for OsuBuilder {
    #[inline]
    fn default() -> Self {
        Self {
            auth_kind: None,
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
            .refill_interval(Duration::from_millis(1000 / self.per_second as u64))
            .refill_amount(1)
            .build();

        let (tx, dropped_rx) = oneshot::channel();

        let inner = Arc::new(OsuRef {
            client_id,
            client_secret,
            http,
            ratelimiter,
            timeout: self.timeout,
            auth_kind: self.auth_kind.unwrap_or_default(),
            token: RwLock::new(Token::default()),
            retries: self.retries,
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
        Token::update_worker(Arc::clone(&inner), expires_in, dropped_rx);

        Ok(Osu {
            inner,
            token_loop_tx: Some(tx),

            #[cfg(feature = "cache")]
            cache: Box::new(DashMap::new()),

            #[cfg(feature = "metrics")]
            metrics: Box::new(Metrics::new()),
        })
    }

    /// Set the client id of the application.
    ///
    /// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
    #[inline]
    pub fn client_id(mut self, client_id: u64) -> Self {
        self.client_id.replace(client_id);

        self
    }

    /// Set the client secret of the application.
    ///
    /// For more info, check out <https://osu.ppy.sh/docs/index.html#client-credentials-grant>
    #[inline]
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret.replace(client_secret.into());

        self
    }

    /// After acquiring the authorization code from a user through OAuth,
    /// use this method to provide the given code, and specified redirect uri.
    ///
    /// For more info, check out <https://osu.ppy.sh/docs/index.html#authorization-code-grant>
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

    /// In case the request times out, retry up to this many times, defaults to 2.
    #[inline]
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = retries;

        self
    }

    /// Set the timeout for requests, defaults to 10 seconds.
    #[inline]
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }

    /// Set the amount of requests that can be made in one second, defaults to 15.
    /// The given value will be clamped between 1 and 20.
    ///
    /// Check out the osu!api's [terms of use] for acceptable values.
    ///
    /// [terms of use]: https://osu.ppy.sh/docs/index.html#terms-of-use

    #[inline]
    pub fn ratelimit(mut self, reqs_per_sec: u32) -> Self {
        self.per_second = reqs_per_sec.clamp(1, 20);

        self
    }
}
