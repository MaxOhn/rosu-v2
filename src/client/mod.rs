mod builder;

pub use builder::OsuBuilder;

use crate::{
    error::{APIError, OsuError, OsuResult},
    model::GameMode,
    ratelimiter::Ratelimiter,
    request::*,
};

use bytes::Bytes;
use reqwest::{header::HeaderValue, multipart::Form, Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use std::{ops::Drop, sync::Arc};
use tokio::sync::{oneshot::Sender, RwLock};

/// The main osu client.
/// Cheap to clone.
///
/// Must be constructed through [`OsuBuilder`](crate::OsuBuilder).
pub struct Osu {
    pub(crate) inner: Arc<OsuRef>,
}

impl Osu {
    /// Create a new default [`Osu`](crate::Osu) client.
    ///
    /// Errors if the API did not provide a token for the given client id and client secret.
    pub async fn new(client_id: u64, client_secret: impl Into<String>) -> OsuResult<Self> {
        Self::builder()
            .client_id(client_id)
            .client_secret(client_secret)
            .build()
            .await
    }

    /// Fine-tune building an [`Osu`](crate::Osu) client
    #[inline]
    pub fn builder() -> OsuBuilder {
        OsuBuilder::default()
    }

    /// Get a [`Beatmap`](crate::model::Beatmap).
    #[inline]
    pub fn beatmap(&self) -> GetBeatmap {
        GetBeatmap::new(self)
    }

    /// Get a [`BeatmapScores`](crate::model::BeatmapScores).
    #[inline]
    pub fn beatmap_scores(&self, map_id: u32) -> GetBeatmapScores {
        GetBeatmapScores::new(self, map_id)
    }

    /// Get a [`BeatmapUserScore`](crate::model::BeatmapUserScore).
    #[inline]
    pub fn beatmap_user_score(
        &self,
        map_id: u32,
        user_id: impl Into<UserId>,
    ) -> GetBeatmapUserScore {
        GetBeatmapUserScore::new(self, map_id, user_id)
    }

    /// Get a list of comments and their replies up to two levels deep.
    #[inline]
    pub fn comments(&self) -> GetComments {
        GetComments::new(self)
    }

    /// Get the kudosu history of a user.
    #[inline]
    pub fn kudosu(&self, user_id: impl Into<UserId>) -> GetUserKudosu {
        GetUserKudosu::new(self, user_id)
    }

    /// TODO: Documentation
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn multiplayer_score(&self, room: u32, playlist: u32, score_id: u32) -> GetScore {
        GetScore::new(self, room, playlist, score_id)
    }

    /// TODO: Documentation
    #[inline]
    pub fn multiplayer_scores(&self, room: u32, playlist: u32) -> GetScores {
        GetScores::new(self, room, playlist)
    }

    /// TODO: Documentation
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn multiplayer_user_highscore(
        &self,
        room: u32,
        playlist: u32,
        user_id: u32,
    ) -> GetUserHighScore {
        GetUserHighScore::new(self, room, playlist, user_id)
    }

    /// Get [`News`](crate::model::News)
    #[inline]
    pub fn news(&self) -> GetNews {
        GetNews::new(self)
    }

    /// Get the current ranking for the specified type and mode.
    #[inline]
    pub fn rankings(&self, mode: GameMode) -> GetRankings {
        GetRankings::new(self, mode)
    }

    /// Get the recent activity of a user.
    #[inline]
    pub fn recent_events(&self, user_id: impl Into<UserId>) -> GetRecentEvents {
        GetRecentEvents::new(self, user_id)
    }

    /// Get the list of spotlights
    #[inline]
    pub fn spotlights(&self) -> GetSpotlights {
        GetSpotlights::new(self)
    }

    /// Get a [`User`](crate::model::User).
    #[inline]
    pub fn user(&self, user_id: impl Into<UserId>) -> GetUser {
        GetUser::new(self, user_id)
    }

    /// Get the beatmapsets of a user.
    #[inline]
    pub fn user_beatmapsets(&self, user_id: impl Into<UserId>) -> GetUserBeatmapsets {
        GetUserBeatmapsets::new(self, user_id)
    }

    /// Get the beatmapsets of a user.
    #[inline]
    pub fn user_most_played(&self, user_id: impl Into<UserId>) -> GetUserMostPlayed {
        GetUserMostPlayed::new(self, user_id)
    }

    /// Get either top, global firsts, or recent scores of a user.
    #[inline]
    pub fn user_scores(&self, user_id: impl Into<UserId>) -> GetUserScores {
        GetUserScores::new(self, user_id)
    }

    /// Get a vec of [`UserCompact`](crate::model::UserCompact).
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn users<I: Into<UserId>>(&self, user_ids: impl Iterator<Item = I>) -> GetUsers {
        let user_ids = user_ids.take(50).map(I::into).collect();

        GetUsers::new(self, user_ids)
    }

    /// Get a wiki article or image data
    ///
    /// `locale` adjusts the language, e.g. `en` for english, `de` for german, ...
    #[inline]
    pub fn wiki(&self, locale: impl Into<String>) -> GetWikiPage {
        GetWikiPage::new(self, locale)
    }
}

pub(crate) struct OsuRef {
    pub(crate) client_id: u64,
    pub(crate) client_secret: String,
    pub(crate) http: Client,
    pub(crate) ratelimiter: Ratelimiter,
    pub(crate) token: RwLock<Option<String>>,
    pub(crate) token_loop_tx: Option<Sender<()>>,
}

static USER_AGENT: &str = concat!(
    "(",
    env!("CARGO_PKG_HOMEPAGE"),
    ", ",
    env!("CARGO_PKG_VERSION"),
    ") rosu-v2",
);

impl OsuRef {
    pub(crate) async fn request_token(&self) -> OsuResult<Token> {
        let form = Form::new()
            .text("client_id", self.client_id.to_string())
            .text("client_secret", self.client_secret.to_owned())
            .text("grant_type", "client_credentials")
            .text("scope", "public");

        let user_agent = HeaderValue::from_static(USER_AGENT);
        let url = "https://osu.ppy.sh/oauth/token";

        let builder = self
            .http
            .request(Method::POST, url)
            .multipart(form)
            .header("User-Agent", user_agent);

        self.ratelimiter.await_access().await;

        let resp = builder
            .send()
            .await
            .map_err(|source| OsuError::Request { source })?;

        let status = resp.status();

        match status {
            StatusCode::OK => {
                let bytes = resp
                    .bytes()
                    .await
                    .map_err(|source| OsuError::ChunkingResponse { source })?;
                return serde_json::from_slice(&bytes).map_err(|source| {
                    let body = String::from_utf8_lossy(&bytes).into();
                    OsuError::Parsing { body, source }
                });
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                let body = resp.text().await.ok();
                return Err(OsuError::ServiceUnavailable(body));
            }
            StatusCode::TOO_MANY_REQUESTS => warn!("429 response: {:?}", resp),
            _ => {}
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|source| OsuError::ChunkingResponse { source })?;

        let body = String::from_utf8_lossy(bytes.as_ref()).into_owned();

        let source = match serde_json::from_slice(&bytes) {
            Ok(APIError { error }) => error.filter(|e| !e.is_empty()),
            Err(source) => return Err(OsuError::Parsing { body, source }),
        };

        Err(OsuError::Response {
            body,
            source,
            status,
        })
    }

    pub(crate) async fn request<T: DeserializeOwned>(&self, req: Request) -> OsuResult<T> {
        let bytes = self.request_bytes(req).await?;

        // let text = String::from_utf8_lossy(&bytes);
        // println!("{}", text);

        serde_json::from_slice(&bytes).map_err(|source| {
            let body = String::from_utf8_lossy(&bytes).into();
            OsuError::Parsing { body, source }
        })
    }

    pub(crate) async fn request_bytes(&self, req: Request) -> OsuResult<Bytes> {
        let resp = self.make_request(req).await?;

        resp.bytes()
            .await
            .map_err(|source| OsuError::ChunkingResponse { source })
    }

    async fn make_request(&self, req: Request) -> OsuResult<Response> {
        let resp = self.raw(req).await?;
        let status = resp.status();

        match status {
            StatusCode::OK => return Ok(resp),
            StatusCode::SERVICE_UNAVAILABLE => {
                let body = resp.text().await.ok();

                return Err(OsuError::ServiceUnavailable(body));
            }
            StatusCode::TOO_MANY_REQUESTS => warn!("429 response: {:?}", resp),
            _ => {}
        }

        let bytes = resp
            .bytes()
            .await
            .map_err(|source| OsuError::ChunkingResponse { source })?;

        let body = String::from_utf8_lossy(bytes.as_ref()).into_owned();

        let source = match serde_json::from_slice(&bytes) {
            Ok(APIError { error }) => error.filter(|e| !e.is_empty()),
            Err(source) => return Err(OsuError::Parsing { body, source }),
        };

        Err(OsuError::Response {
            body,
            source,
            status,
        })
    }

    async fn raw(&self, req: Request) -> OsuResult<Response> {
        let Request {
            query,
            method,
            path,
        } = req;

        // println!("Path: {}", path);

        let url = format!("https://osu.ppy.sh/api/v2/{}", path);
        debug!("URL: {}", url);
        let mut builder = self.http.request(method.clone(), &url);

        if !query.is_empty() {
            builder = builder.query(&query);
        }

        if let Some(token) = self.token.read().await.as_ref() {
            let value =
                HeaderValue::from_str(token).map_err(|source| OsuError::CreatingHeader {
                    name: "Authorization",
                    source,
                })?;

            builder = builder.header("Authorization", value);
        } else {
            return Err(OsuError::NoToken);
        }

        if matches!(method, Method::PUT | Method::POST | Method::PATCH) {
            builder = builder.header("content-length", 0);
        }

        let user_agent = HeaderValue::from_static(USER_AGENT);
        builder = builder.header("User-Agent", user_agent);

        self.ratelimiter.await_access().await;

        let resp = builder
            .send()
            .await
            .map_err(|source| OsuError::Request { source })?;

        Ok(resp)
    }
}

impl Drop for OsuRef {
    fn drop(&mut self) {
        let tx = self.token_loop_tx.take().unwrap();
        let _ = tx.send(());
    }
}

#[derive(Deserialize)]
pub(crate) struct Token {
    token_type: String,
    expires_in: u64,
    access_token: String,
}
