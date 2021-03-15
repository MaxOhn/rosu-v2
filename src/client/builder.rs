use super::{Osu, OsuRef};
use crate::{error::OsuError, ratelimiter::Ratelimiter, OsuResult};

use reqwest::ClientBuilder as ReqwestClientBuilder;
use std::{sync::Arc, time::Duration as StdDuration};
use tokio::{
    sync::{
        oneshot::{self, error::TryRecvError},
        RwLock,
    },
    time::{sleep, Duration},
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
#[derive(Default)]
pub struct OsuBuilder {
    client_id: Option<u64>,
    client_secret: Option<String>,
    reqwest_client: Option<ReqwestClientBuilder>,
    timeout: Option<StdDuration>,
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
        let client_id = self.client_id.ok_or(OsuError::BuilderMissingID)?;
        let client_secret = self.client_secret.ok_or(OsuError::BuilderMissingSecret)?;

        let http = self
            .reqwest_client
            .unwrap_or_else(ReqwestClientBuilder::new)
            .timeout(self.timeout.unwrap_or_else(|| StdDuration::from_secs(10)))
            .build()
            .map_err(|source| OsuError::BuildingClient { source })?;

        let ratelimiter = Ratelimiter::new(15, 1);
        let (tx, mut rx) = oneshot::channel();

        let inner = Arc::new(OsuRef {
            client_id,
            client_secret,
            http,
            ratelimiter,
            token: RwLock::new(None),
            token_loop_tx: Some(tx),
        });

        // Acquire the initial API token
        let token = inner
            .request_token()
            .await
            .map_err(Box::new)
            .map_err(|source| OsuError::UpdateToken { source })?;

        let access_token = format!("Bearer {}", token.access_token);
        inner.token.write().await.replace(access_token);

        // Let an async worker update the token regularly
        let mut actual_expire = token.expires_in;
        let mut adjusted_expire = adjust_token_expire(actual_expire);
        let osu = Arc::clone(&inner);

        tokio::spawn(async move {
            sleep(Duration::from_secs(adjusted_expire)).await;

            loop {
                if matches!(rx.try_recv(), Ok(_) | Err(TryRecvError::Closed)) {
                    debug!("Exiting token update loop");

                    return;
                }

                // In case of acquiring a new token taking too long,
                // remove the previous token as soon as it expires
                // so that new requests will not be sent until
                // a new token has been acquired
                let (expire_tx, expire_rx) = oneshot::channel();
                let osu_clone = Arc::clone(&osu);

                tokio::spawn(async move {
                    tokio::select!(
                        _ = expire_rx => {}
                        _ = sleep(Duration::from_secs(actual_expire - adjusted_expire)) => {
                            warn!("Acquiring new token took too long, remove current token");
                            osu_clone.token.write().await.take();
                        }
                    )
                });

                // Acquire a new token through exponential backoff
                let mut backoff = 200;
                info!("API token expired, acquire new one");

                while {
                    match osu.request_token().await {
                        Ok(token) if token.token_type == "Bearer" => {
                            actual_expire = token.expires_in;
                            adjusted_expire = adjust_token_expire(token.expires_in);
                            let access_token = format!("Bearer {}", token.access_token);
                            osu.token.write().await.replace(access_token);

                            false
                        }
                        Ok(token) => {
                            warn!(
                                "Failed to acquire new token, {:?} != \"Bearer\"; retry in {}ms",
                                token.token_type, backoff
                            );

                            true
                        }
                        Err(why) => {
                            warn!(
                                "Failed to acquire new token: {}; retry in {}ms",
                                why, backoff
                            );

                            true
                        }
                    }
                } {
                    sleep(Duration::from_millis(backoff)).await;
                    backoff = (backoff * 2).min(60_000);
                }

                let _ = expire_tx.send(());
                sleep(Duration::from_secs(adjusted_expire)).await;
            }
        });

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

    /// Set a pre-configured reqwest client builder to build off of.
    ///
    /// The timeout settings in the reqwest client will be overwritten by
    /// those in this builder.
    ///
    /// The default client uses Rustls as its TLS backend.
    #[inline]
    pub fn reqwest_client(mut self, client: ReqwestClientBuilder) -> Self {
        self.reqwest_client.replace(client);

        self
    }

    /// Set the timeout for HTTP requests, defaults to 10 seconds.
    #[inline]
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout.replace(duration);

        self
    }
}

#[inline]
fn adjust_token_expire(expires_in: u64) -> u64 {
    expires_in - (expires_in as f64 * 0.05) as u64
}
