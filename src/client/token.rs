use super::OsuRef;

use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{
    sync::oneshot::{self, error::TryRecvError, Receiver},
    time::sleep,
};

#[derive(Debug, Default)]
pub(super) struct Token {
    pub access: Option<String>,
    pub refresh: Option<String>,
}

impl Token {
    pub(super) fn update(&mut self, response: TokenResponse) {
        self.access = Some(format!("Bearer {}", response.access_token));
        self.refresh = response.refresh_token;
    }

    pub(super) fn update_worker(osu: Arc<OsuRef>, mut expire: u64, mut close_rx: Receiver<()>) {
        let mut adjusted_expire = adjust_token_expire(expire);

        tokio::spawn(async move {
            loop {
                info!("Acquire new API token in {} seconds", adjusted_expire);
                sleep(Duration::from_secs(adjusted_expire)).await;

                if matches!(close_rx.try_recv(), Ok(_) | Err(TryRecvError::Closed)) {
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
                        _ = sleep(Duration::from_secs(expire)) => {
                            warn!("Acquiring new token took too long, remove current token");
                            osu_clone.token.write().await.access.take();
                        }
                    )
                });

                // Acquire a new token through exponential backoff
                let mut backoff = 400;
                info!("API token expired, acquire new one");

                loop {
                    match osu.request_token().await {
                        Ok(token) if token.token_type == "Bearer" => {
                            info!("Successfully acquired new token");

                            expire = token.expires_in;
                            adjusted_expire = adjust_token_expire(expire);

                            osu.token.write().await.update(token);

                            break;
                        }
                        Ok(token) => {
                            warn!(
                                r#"Failed to acquire new token, "{}" != "Bearer"; retry in {}ms"#,
                                token.token_type, backoff
                            );
                        }
                        Err(why) => {
                            warn!(
                                "Failed to acquire new token: {}; retry in {}ms",
                                why, backoff
                            );

                            let mut err: &dyn Error = &why;

                            while let Some(src) = err.source() {
                                warn!("  - caused by: {}", src);
                                err = src;
                            }
                        }
                    }

                    sleep(Duration::from_millis(backoff)).await;
                    backoff = (backoff * 2).min(60_000);
                }

                let _ = expire_tx.send(());
            }
        });
    }
}

#[inline]
fn adjust_token_expire(expires_in: u64) -> u64 {
    expires_in - (expires_in as f64 * 0.05) as u64
}

pub(super) enum AuthorizationKind {
    User(Authorization),
    Client(Scope),
}

impl Default for AuthorizationKind {
    fn default() -> Self {
        Self::Client(Scope::Public)
    }
}

pub(super) struct Authorization {
    pub code: String,
    pub redirect_uri: String,
}

#[derive(Deserialize)]
pub(super) struct TokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    #[serde(default)]
    pub refresh_token: Option<String>,
    pub token_type: String,
}

#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    ChatWrite,
    Delegate,
    ForumWrite,
    FriendsRead,
    Identify,
    Lazer,
    Public,
}

impl Display for Scope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Scope::ChatWrite => f.write_str("chat.write"),
            Scope::Delegate => f.write_str("delegate"),
            Scope::ForumWrite => f.write_str("forum.write"),
            Scope::FriendsRead => f.write_str("friends.read"),
            Scope::Identify => f.write_str("identify"),
            Scope::Lazer => f.write_str("lazer"),
            Scope::Public => f.write_str("public"),
        }
    }
}
