mod builder;

pub use builder::OsuBuilder;

use crate::{error::OsuError, model::GameMode, ratelimiter::Ratelimiter, request::*, OsuResult};

use bytes::Bytes;
use reqwest::{header::HeaderValue, multipart::Form, Client, Method, Response, StatusCode};
use serde::{de::DeserializeOwned, Deserialize};
use std::{ops::Drop, sync::Arc};
use tokio::sync::{oneshot::Sender, RwLock};

#[cfg(feature = "cache")]
use dashmap::DashMap;

#[cfg(feature = "metrics")]
use crate::metrics::Metrics;

#[cfg(feature = "metrics")]
use prometheus::IntCounterVec;

/// The main osu client.
/// Cheap to clone.
///
/// Must be constructed through [`OsuBuilder`](crate::OsuBuilder).
pub struct Osu {
    pub(crate) inner: Arc<OsuRef>,
    #[cfg(feature = "cache")]
    pub(crate) cache: Arc<DashMap<String, u32>>,
    #[cfg(feature = "metrics")]
    pub(crate) metrics: Arc<Metrics>,
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

    /// Fine-tune building an [`Osu`](crate::Osu) client.
    #[inline]
    pub fn builder() -> OsuBuilder {
        OsuBuilder::default()
    }

    /// Returns an [`IntCounterVec`](crate::prelude::IntCounterVec) from
    /// [prometheus](https://crates.io/crates/prometheus) containing
    /// a counter for each request type.
    #[cfg(feature = "metrics")]
    pub fn metrics(&self) -> IntCounterVec {
        self.metrics.counters.clone()
    }

    /// Get a [`Beatmap`](crate::model::beatmap::Beatmap).
    ///
    /// Filled options will be: `deleted_at` (if deleted), `fail_times`,
    /// `mapset` and `max_combo` (if available for mode).
    ///
    /// The contained [`Beatmapset`](crate::model::beatmap::Beatmapset) will
    /// have these options filled: `legacy_thread_url`, `ratings`,
    /// `ranked_date` (if not unranked) and `submitted_date` (if submitted).
    #[inline]
    pub fn beatmap(&self) -> GetBeatmap {
        GetBeatmap::new(self)
    }

    /// Get a vec of [`Score`](crate::model::score::Score).
    ///
    /// The contained scores will have the following options filled:
    /// `map`, `pp` (if ranked or approved), and `user`.
    ///
    /// The scores' contained [`UserCompact`](crate::model::user::UserCompact)
    /// will have the `country` and `cover` options filled.
    #[inline]
    pub fn beatmap_scores(&self, map_id: u32) -> GetBeatmapScores {
        GetBeatmapScores::new(self, map_id)
    }

    /// Get a [`BeatmapUserScore`](crate::model::score::BeatmapUserScore).
    ///
    /// The contained [`Score`](crate::model::score::Score) will have the
    /// `map` and `user` options filled.
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn beatmap_user_score(&self, map_id: u32, user_id: u32) -> GetBeatmapUserScore {
        GetBeatmapUserScore::new(self, map_id, user_id)
    }

    /// Get a [`BeatmapUserScore`](crate::model::score::BeatmapUserScore).
    ///
    /// The contained [`Score`](crate::model::score::Score) will have the
    /// `map` and `user` options filled.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn beatmap_user_score(
        &self,
        map_id: u32,
        user_id: impl Into<UserId>,
    ) -> GetBeatmapUserScore {
        GetBeatmapUserScore::new(self, map_id, user_id.into())
    }

    /// Get a [`Beatmapset`](crate::model::beatmap::Beatmapset).
    ///
    /// Filled options will be: `artist_unicode`, `converts`, `description`,
    /// `genre`, `language`, `legacy_thread_url`, `maps`, `ratings`,
    /// `ranked_date` (if not unranked), `recent_favourites`,
    /// `submitted_date` (if submitted), and `title_unicode`.
    ///
    /// The contained [`Beatmap`](crate::model::beatmap::Beatmap)s
    /// will contain `Some` in `fail_times`, `max_combo`
    /// (if available for mode), and `deleted_at` (if deleted).
    #[inline]
    pub fn beatmapset(&self, mapset_id: u32) -> GetBeatmapset {
        GetBeatmapset::new(self, mapset_id)
    }

    /// Get a [`BeatmapsetEvents`](crate::model::beatmap::BeatmapsetEvents)
    /// struct containing the most recent mapset events.
    #[inline]
    pub fn beatmapset_events(&self) -> GetBeatmapsetEvents {
        GetBeatmapsetEvents::new(self)
    }

    /// Get a [`BeatmapsetSearchResult`](crate::model::beatmap::BeatmapsetSearchResult)
    /// struct containing the first page of maps that fit the search query.
    ///
    /// The default search parameters are:
    /// - mode: any
    /// - status: has leaderboard (ranked, loved, approved, and qualified)
    /// - genre: any
    /// - language: any
    /// - extra: does neither contain have video nor storyboard
    /// - nsfw: allowed
    /// - sort: by relevance, descending
    ///
    /// The contained [`Beatmapset`](crate::mode::beatmap::Beatmapset)s will have the
    /// following options filled: `artist_unicode`, `legacy_thread_url`, `maps`,
    /// `ranked_date` and `submitted_date` if available, and `title_unicode`.
    ///
    /// The search query allows the following options to be specified: `ar`, `artist`,
    /// `bpm`, `created`, `creator`, `cs`, `dr` (hp drain rate), `keys`, `length`,
    /// `ranked`, `stars`, and `status`.
    ///
    /// ## Example
    ///
    /// ```
    /// // Search for mapsets from Sotarks that have a map with no more than AR 9.
    /// let query = "creator=sotarks ar<9";
    ///
    /// // Loved mapsets from Camellia including at least one map above 8 stars
    /// let query = "status=loved artist=camellia stars>8";
    /// ```
    #[inline]
    pub fn beatmapset_search(&self) -> GetBeatmapsetSearch {
        GetBeatmapsetSearch::new(self)
    }

    /// Get a list of comments and their replies up to two levels deep
    /// in form of a [`CommentBundle`](crate::model::comments::CommentBundle) .
    #[inline]
    pub fn comments(&self) -> GetComments {
        GetComments::new(self)
    }

    /// Get a [`ChartRankings`](crate::model::ranking::ChartRankings) struct
    /// containing a [`Spotlight`](crate::model::ranking::Spotlight), its
    /// [`Beatmapset`](crate::model::beatmap::Beatmapset)s, and participating
    /// [`UserStatistics`](crate::model::user::UserStatistics).
    ///
    /// The mapset will have their `maps` option filled.
    ///
    /// The user statistics contain specific, spotlight related data.
    /// All fields depends only on scores on maps of the spotlight.
    /// The statistics vector is ordered by `ranked_score`.
    /// The `user` option is filled.
    #[inline]
    pub fn chart_rankings(&self, mode: GameMode) -> GetChartRankings {
        GetChartRankings::new(self, mode)
    }

    /// Get a [`CountryRankings`](crate::model::ranking::CountryRankings) struct
    /// containing a vec of [`CountryRanking`](crate::model::ranking::CountryRanking)s
    /// which will be sorted by the country's total pp.
    #[inline]
    pub fn country_rankings(&self, mode: GameMode) -> GetCountryRankings {
        GetCountryRankings::new(self, mode)
    }

    /// Get a [`ForumPosts`](crate::model::forum::ForumPosts) struct for a forum topic
    #[inline]
    pub fn forum_posts(&self, topic_id: u64) -> GetForumPosts {
        GetForumPosts::new(self, topic_id)
    }

    /// Get the kudosu history of a user in form of a vec of
    /// [`KudosuHistory`](crate::model::kudosu::KudosuHistory).
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn kudosu(&self, user_id: u32) -> GetUserKudosu {
        GetUserKudosu::new(self, user_id)
    }

    /// Get the kudosu history of a user in form of a vec of
    /// [`KudosuHistory`](crate::model::kudosu::KudosuHistory).
    #[cfg(feature = "cache")]
    #[inline]
    pub fn kudosu(&self, user_id: impl Into<UserId>) -> GetUserKudosu {
        GetUserKudosu::new(self, user_id.into())
    }

    /// TODO: Documentation
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn multiplayer_score(
        &self,
        room: u32,
        playlist: u32,
        score_id: u32,
    ) -> GetMultiplayerScore {
        GetMultiplayerScore::new(self, room, playlist, score_id)
    }

    /// TODO: Documentation
    #[inline]
    pub fn multiplayer_scores(&self, room: u32, playlist: u32) -> GetMultiplayerScores {
        GetMultiplayerScores::new(self, room, playlist)
    }

    /// TODO: Documentation
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn multiplayer_user_highscore(
        &self,
        room: u32,
        playlist: u32,
        user_id: u32,
    ) -> GetMultiplayerUserHighScore {
        GetMultiplayerUserHighScore::new(self, room, playlist, user_id)
    }

    /// Get [`News`](crate::model::news::News).
    #[inline]
    pub fn news(&self) -> GetNews {
        GetNews::new(self)
    }

    /// Get an [`OsuMatch`](crate::model::matches::OsuMatch).
    #[inline]
    pub fn osu_match(&self, match_id: u32) -> GetMatch {
        GetMatch::new(self, match_id)
    }

    /// Get a [`MatchList`](crate::model::matches::MatchList) containing all
    /// currently open multiplayer lobbies.
    #[inline]
    pub fn osu_matches(&self) -> GetMatches {
        GetMatches::new(self)
    }

    /// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
    /// [`UserStatistics`](crate::model::user::UserStatistics) are sorted
    /// by their pp, i.e. the current pp leaderboard.
    #[inline]
    pub fn performance_rankings(&self, mode: GameMode) -> GetPerformanceRankings {
        GetPerformanceRankings::new(self, mode)
    }

    /// Get the recent activity of a user in form of a vec of
    /// [`RecentEvent`](crate::model::recent_event::RecentEvent)s.
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn recent_events(&self, user_id: u32) -> GetRecentEvents {
        GetRecentEvents::new(self, user_id)
    }

    /// Get the recent activity of a user in form of a vec of
    /// [`RecentEvent`](crate::model::recent_event::RecentEvent)s.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn recent_events(&self, user_id: impl Into<UserId>) -> GetRecentEvents {
        GetRecentEvents::new(self, user_id.into())
    }

    /// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
    /// [`UserStatistics`](crate::model::user::UserStatistics) are sorted
    /// by their ranked score, i.e. the current ranked score leaderboard.
    #[inline]
    pub fn score_rankings(&self, mode: GameMode) -> GetScoreRankings {
        GetScoreRankings::new(self, mode)
    }

    /// Get [`SeasonalBackgrounds`](crate::model::seasonal_backgrounds::SeasonalBackgrounds).
    #[inline]
    pub fn seasonal_backgrounds(&self) -> GetSeasonalBackgrounds {
        GetSeasonalBackgrounds::new(self)
    }

    /// Get the vec of [`Spotlight`](crate::model::ranking::Spotlight).
    #[inline]
    pub fn spotlights(&self) -> GetSpotlights {
        GetSpotlights::new(self)
    }

    /// Get a [`User`](crate::model::user::User).
    ///
    /// The following options will be filled if the user specified them:
    /// `discord`, `interests`, `location`, `occupation`, `playstyle`,
    /// `profile_color`, `skype`, `title`, `title_url`, `website`
    ///
    /// The only `is_*` options that will be filled are `is_active`, `is_bot`,
    /// `is_deleted`, `is_online`, and `is_supporter`, the others won't be.
    ///
    /// All other options will be filled.
    #[inline]
    pub fn user(&self, user_id: impl Into<UserId>) -> GetUser {
        GetUser::new(self, user_id)
    }

    /// Get the [`Beatmapset`](crate::model::beatmap::Beatmapset)s of a user by their id.
    ///
    /// If no map type specified, either manually through
    /// [`map_type`](crate::request::GetUserBeatmapsets::map_type),
    /// or through any of the methods [`loved`](crate::request::GetUserBeatmapsets::loved),
    /// [`favourite`](crate::request::GetUserBeatmapsets::favourite),
    /// [`graveyard`](crate::request::GetUserBeatmapsets::graveyard),
    /// [`ranked_and_approved`](crate::request::GetUserBeatmapsets::ranked_and_approved),
    /// [`unranked`](crate::request::GetUserBeatmapsets::unranked),
    /// it defaults to `ranked_and_approved`.
    ///
    /// Filled options will be: `artist_unicode`, `legacy_thread_url`, `maps`, `title_unicode`.
    ///
    /// All options of the contained [`Beatmap`](crate::model::beatmap::Beatmap)s will be `None`.
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn user_beatmapsets(&self, user_id: u32) -> GetUserBeatmapsets {
        GetUserBeatmapsets::new(self, user_id)
    }

    /// Get a vec of [`Beatmapset`](crate::model::beatmap::Beatmapset)s a user made.
    ///
    /// Filled options will be: `artist_unicode`, `legacy_thread_url`, `maps`, `title_unicode`.
    ///
    /// All options of the contained [`Beatmap`](crate::model::beatmap::Beatmap)s will be `None`.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn user_beatmapsets(&self, user_id: impl Into<UserId>) -> GetUserBeatmapsets {
        GetUserBeatmapsets::new(self, user_id.into())
    }

    /// Get a vec of a user's [`MostPlayedMap`](crate::model::beatmap::MostPlayedMap)s.
    ///
    /// All options of the contained [`BeatmapCompact`](crate::model::beatmap::BeatmapCompact) and
    /// [`BeatmapsetCompact`](crate::model::beatmap::BeatmapsetCompact) will be `None`.
    ///
    /// ## Limit
    ///
    /// The API provides at most 51 results per requests, 100 in total.
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn user_most_played(&self, user_id: u32) -> GetUserMostPlayed {
        GetUserMostPlayed::new(self, user_id)
    }

    /// Get a vec of a user's [`MostPlayedMap`](crate::model::beatmap::MostPlayedMap)s.
    ///
    /// All options of the contained [`BeatmapCompact`](crate::model::beatmap::BeatmapCompact) and
    /// [`BeatmapsetCompact`](crate::model::beatmap::BeatmapsetCompact) will be `None`.
    ///
    /// ## Limit
    ///
    /// The API provides at most 51 results per requests, 100 in total.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn user_most_played(&self, user_id: impl Into<UserId>) -> GetUserMostPlayed {
        GetUserMostPlayed::new(self, user_id.into())
    }

    /// Get either top, global firsts, or recent scores of a user,
    /// i.e. a vec of [`Score`](crate::model::score::Score).
    ///
    /// If no score type is specified by either
    /// [`best`](crate::request::GetUserScores::best),
    /// [`firsts`](crate::request::GetUserScores::firsts),
    /// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
    ///
    /// The resulting scores will have these options filled: `map`, `mapset`, `pp`, `user`
    ///
    /// Additionally, the `best` score type will provide the `weight` option.
    ///
    /// All options of the contained [`Beatmap`](crate::model::beatmap::Beatmap),
    /// [`BeatmapsetCompact`](crate::model::beatmap::Beatmapset), and
    /// [`UserCompact`](crate::model::user::UserCompact) will be `None`.
    ///
    /// ## Note
    ///
    /// - The API provides at most 51 results per requests, 100 in total.
    /// - For the `recent` score type, failed score are excluded by default.
    /// Use [`include_fails`](crate::request::GetUserScores::include_fails)
    /// to include them.
    /// - For the `firsts` score type, `pp` will only be `Some` if the map
    /// is not loved.
    #[cfg(not(feature = "cache"))]
    #[inline]
    pub fn user_scores(&self, user_id: u32) -> GetUserScores {
        GetUserScores::new(self, user_id)
    }

    /// Get either top, global firsts, or recent scores of a user,
    /// i.e. a vec of [`Score`](crate::model::score::Score).
    ///
    /// If no score type is specified by either
    /// [`best`](crate::request::GetUserScores::best),
    /// [`firsts`](crate::request::GetUserScores::firsts),
    /// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
    ///
    /// The resulting scores will have these options filled: `map`, `mapset`, `pp`, `user`
    ///
    /// Additionally, the `best` score type will provide the `weight` option.
    ///
    /// All options of the contained [`Beatmap`](crate::model::beatmap::Beatmap),
    /// [`BeatmapsetCompact`](crate::model::beatmap::Beatmapset), and
    /// [`UserCompact`](crate::model::user::UserCompact) will be `None`.
    ///
    /// ## Note
    ///
    /// - The API provides at most 51 results per requests, 100 in total.
    /// - For the `recent` score type, failed score are excluded by default.
    /// Use [`include_fails`](crate::request::GetUserScores::include_fails)
    /// to include them.
    /// - For the `firsts` score type, `pp` will only be `Some` if the map
    /// is not loved.
    #[cfg(feature = "cache")]
    #[inline]
    pub fn user_scores(&self, user_id: impl Into<UserId>) -> GetUserScores {
        GetUserScores::new(self, user_id.into())
    }

    /// Get a vec of [`UserCompact`](crate::model::user::UserCompact).
    #[deprecated = "The API currently doesn't allow this endpoint for public use"]
    #[inline]
    pub fn users(&self, user_ids: &[u32]) -> GetUsers {
        GetUsers::new(self, user_ids)
    }

    /// Get a [`WikiPage`](crate::model::wiki::WikiPage) or image data.
    ///
    /// `locale` adjusts the language, e.g. `en` for english, `de` for german, ...
    #[inline]
    pub fn wiki(&self, locale: impl Into<String>) -> GetWikiPage {
        GetWikiPage::new(self, locale)
    }

    #[cfg(feature = "cache")]
    pub(crate) async fn cache_user(&self, user_id: UserId) -> OsuResult<u32> {
        match user_id {
            UserId::Id(id) => Ok(id),
            UserId::Name(mut name) => {
                name.make_ascii_lowercase();

                if let Some(id) = self.cache.get(&name) {
                    return Ok(*id.value());
                }

                let user = self.user(UserId::Name(name.clone())).await?;
                self.cache.insert(name, user.user_id);

                Ok(user.user_id)
            }
        }
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
    "Rust API v2 (",
    env!("CARGO_PKG_REPOSITORY"),
    " v",
    env!("CARGO_PKG_VERSION"),
    ")",
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
            Ok(source) => source,
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
            StatusCode::NOT_FOUND => return Err(OsuError::NotFound),
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
            Ok(source) => source,
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
