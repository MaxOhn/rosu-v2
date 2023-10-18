use super::OsuRef;

use serde::Deserialize;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::{
    sync::oneshot::{self, Receiver},
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

    pub(super) fn update_worker(osu: Arc<OsuRef>, mut expire: i64, mut dropped_rx: Receiver<()>) {
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
                    token = Self::update_routine(Arc::clone(&osu), expire, expire_rx) => {
                        let _ = expire_tx.send(());
                        debug!("Successfully acquired new token");

                        expire = token.expires_in;
                        osu.token.write().await.update(token);
                    }
                }
            }
        });
    }

    async fn update_routine(
        osu: Arc<OsuRef>,
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

        Token::request_loop(&osu).await
    }

    // Acquire a new token through exponential backoff
    async fn request_loop(osu: &OsuRef) -> TokenResponse {
        let mut backoff = 400;

        loop {
            match osu.request_token().await {
                Ok(token) if token.token_type == "Bearer" => return token,
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
    }
}

#[inline]
fn adjust_token_expire(expires_in: i64) -> i64 {
    expires_in - (expires_in as f64 * 0.05) as i64
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
    pub expires_in: i64,
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
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
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
