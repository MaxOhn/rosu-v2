mod builder;
mod scopes;
mod token;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper_util::client::legacy::{connect::HttpConnector, Client as HyperClient};
use token::{Authorization, AuthorizationKind, TokenResponse};

pub use self::{builder::OsuBuilder, scopes::Scopes, token::Token};

#[allow(clippy::wildcard_imports)]
use crate::{
    error::OsuError,
    model::{user::UserBeatmapsetsKind, GameMode},
    request::*,
    OsuResult,
};

use hyper::{
    body::Incoming,
    header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT},
    Method, Request as HyperRequest, Response, StatusCode,
};
use hyper_rustls::HttpsConnector;
use leaky_bucket_lite::LeakyBucket;
use serde::de::DeserializeOwned;
use std::{ops::Drop, sync::Arc, time::Duration};
use tokio::sync::{oneshot::Sender, RwLock};
use url::Url;

/// The main osu client.
pub struct Osu {
    pub(crate) inner: Arc<OsuRef>,
    #[cfg(feature = "cache")]
    pub(crate) cache: Box<dashmap::DashMap<crate::prelude::Username, u32>>,
    token_loop_tx: Option<Sender<()>>,
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

    /// Return the [`Token`] that is being used when requesting data.
    #[inline]
    pub async fn token(&self) -> Token {
        self.inner.token.read().await.to_owned()
    }

    /// Get a [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended).
    ///
    /// Filled options will be: `deleted_at` (if deleted), `fail_times`,
    /// `mapset` and `max_combo` (if available for mode).
    ///
    /// The contained [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended) will
    /// have these options filled: `legacy_thread_url`, `ratings`,
    /// `ranked_date` (if not unranked) and `submitted_date` (if submitted).
    #[inline]
    pub const fn beatmap(&self) -> GetBeatmap<'_> {
        GetBeatmap::new(self)
    }

    /// Get a vec of at most 50 [`Beatmap`](crate::model::beatmap::Beatmap)s.
    ///
    /// The contained maps will have these options filled: `mapset`,
    /// `fail_times`, and `max_combo` (if available for mode).
    #[inline]
    pub fn beatmaps<I>(&self, map_ids: I) -> GetBeatmaps<'_>
    where
        I: IntoIterator<Item = u32>,
    {
        GetBeatmaps::new(self, map_ids)
    }

    /// Get a vec of [`Score`](crate::model::score::Score).
    ///
    /// The contained scores will have the following options filled:
    /// `pp` (if ranked or approved), and `user`.
    ///
    /// The scores' contained [`User`](crate::model::user::User)
    /// will have the `country` and `cover` options filled.
    #[inline]
    pub const fn beatmap_scores(&self, map_id: u32) -> GetBeatmapScores<'_> {
        GetBeatmapScores::new(self, map_id)
    }

    /// Get the [`BeatmapDifficultyAttributes`](crate::model::beatmap::BeatmapDifficultyAttributes) for a map.
    #[inline]
    pub fn beatmap_difficulty_attributes(&self, map_id: u32) -> GetBeatmapDifficultyAttributes<'_> {
        GetBeatmapDifficultyAttributes::new(self, map_id)
    }

    /// Get a [`BeatmapUserScore`](crate::model::score::BeatmapUserScore).
    ///
    /// The contained [`Score`](crate::model::score::Score) will have the
    /// `map` and `user` options filled.
    #[inline]
    pub fn beatmap_user_score(
        &self,
        map_id: u32,
        user_id: impl Into<UserId>,
    ) -> GetBeatmapUserScore<'_> {
        GetBeatmapUserScore::new(self, map_id, user_id.into())
    }

    /// Get the top score for each mod combination a user has on a
    /// map in form of a vec of [`Score`](crate::model::score::Score)s.
    ///
    /// The contained scores won't have any Options filled except
    /// for `pp` in case of a ranked map.
    #[inline]
    pub fn beatmap_user_scores(
        &self,
        map_id: u32,
        user_id: impl Into<UserId>,
    ) -> GetBeatmapUserScores<'_> {
        GetBeatmapUserScores::new(self, map_id, user_id.into())
    }

    /// Get a [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended).
    ///
    /// Filled options will be: `artist_unicode`, `converts`, `description`,
    /// `genre`, `language`, `legacy_thread_url`, `maps`, `ratings`,
    /// `ranked_date` (if not unranked), `recent_favourites`,
    /// `submitted_date` (if submitted), and `title_unicode`.
    ///
    /// The contained [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended)s
    /// will contain `Some` in `fail_times`, `max_combo`
    /// (if available for mode), and `deleted_at` (if deleted).
    #[inline]
    pub fn beatmapset(&self, mapset_id: u32) -> GetBeatmapset<'_> {
        GetBeatmapset::new(self, mapset_id)
    }

    /// Get a [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended) from a map ID.
    ///
    /// Filled options will be: `artist_unicode`, `converts`, `description`,
    /// `genre`, `language`, `legacy_thread_url`, `maps`, `ratings`,
    /// `ranked_date` (if not unranked), `recent_favourites`,
    /// `submitted_date` (if submitted), and `title_unicode`.
    ///
    /// The contained [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended)s
    /// will contain `Some` in `fail_times`, `max_combo`
    /// (if available for mode), and `deleted_at` (if deleted).
    #[inline]
    pub fn beatmapset_from_map_id(&self, map_id: u32) -> GetBeatmapsetFromMapId<'_> {
        GetBeatmapsetFromMapId::new(self, map_id)
    }

    /// Get a [`BeatmapsetEvents`](crate::model::beatmap::BeatmapsetEvents)
    /// struct containing the most recent mapset events.
    #[inline]
    pub fn beatmapset_events(&self) -> GetBeatmapsetEvents<'_> {
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
    /// - extra: contains neither video nor storyboard
    /// - nsfw: allowed
    /// - sort: by relevance, descending
    ///
    /// The contained [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s will have the
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
    pub const fn beatmapset_search(&self) -> GetBeatmapsetSearch<'_> {
        GetBeatmapsetSearch::new(self)
    }

    /// Get a list of comments and their replies up to two levels deep
    /// in form of a [`CommentBundle`](crate::model::comments::CommentBundle) .
    #[inline]
    pub const fn comments(&self) -> GetComments<'_> {
        GetComments::new(self)
    }

    /// Get a [`ChartRankings`](crate::model::ranking::ChartRankings) struct
    /// containing a [`Spotlight`](crate::model::ranking::Spotlight), its
    /// [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s, and participating
    /// [`User`](crate::model::user::User).
    ///
    /// The mapset will have their `maps` option filled.
    ///
    /// The user statistics contain specific, spotlight related data.
    /// All fields depends only on scores on maps of the spotlight.
    /// The statistics vector is ordered by `ranked_score`.
    /// The `user` option is filled.
    #[inline]
    pub const fn chart_rankings(&self, mode: GameMode) -> GetChartRankings<'_> {
        GetChartRankings::new(self, mode)
    }

    /// Get a [`CountryRankings`](crate::model::ranking::CountryRankings) struct
    /// containing a vec of [`CountryRanking`](crate::model::ranking::CountryRanking)s
    /// which will be sorted by the country's total pp.
    #[inline]
    pub const fn country_rankings(&self, mode: GameMode) -> GetCountryRankings<'_> {
        GetCountryRankings::new(self, mode)
    }

    /// Get a vec of [`Event`](crate::model::event::Event).
    #[inline]
    pub const fn events(&self) -> GetEvents<'_> {
        GetEvents::new(self)
    }

    /// Get a [`ForumPosts`](crate::model::forum::ForumPosts) struct for a forum topic
    #[inline]
    pub const fn forum_posts(&self, topic_id: u64) -> GetForumPosts<'_> {
        GetForumPosts::new(self, topic_id)
    }

    /// Get all friends of the authenticated user as a vec of [`User`](crate::model::user::User).
    ///
    /// Note that the client has to be initialized with the `FriendsRead` scope
    /// through the OAuth process in order for this endpoint to not return an error.
    ///
    /// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
    #[inline]
    pub const fn friends(&self) -> GetFriends<'_> {
        GetFriends::new(self)
    }

    /// Get the kudosu history of a user in form of a vec of
    /// [`KudosuHistory`](crate::model::kudosu::KudosuHistory).
    #[inline]
    pub fn kudosu(&self, user_id: impl Into<UserId>) -> GetUserKudosu<'_> {
        GetUserKudosu::new(self, user_id.into())
    }

    /// Get [`News`](crate::model::news::News).
    #[inline]
    pub fn news(&self) -> GetNews<'_> {
        GetNews::new(self)
    }

    /// Get an [`OsuMatch`](crate::model::matches::OsuMatch).
    #[inline]
    pub const fn osu_match(&self, match_id: u32) -> GetMatch<'_> {
        GetMatch::new(self, match_id)
    }

    /// Get a [`MatchList`](crate::model::matches::MatchList) containing all
    /// currently open multiplayer lobbies.
    #[inline]
    pub fn osu_matches(&self) -> GetMatches<'_> {
        GetMatches::new(self)
    }

    /// Get the [`UserExtended`](crate::model::user::UserExtended) of the authenticated user.
    ///
    /// Note that the client has to be initialized with the `identify` scope
    /// through the OAuth process in order for this endpoint to not return an error.
    ///
    /// See [`OsuBuilder::with_authorization`](crate::OsuBuilder::with_authorization).
    #[inline]
    pub const fn own_data(&self) -> GetOwnData<'_> {
        GetOwnData::new(self)
    }

    /// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
    /// [`User`](crate::model::user::User)s are sorted
    /// by their pp, i.e. the current pp leaderboard.
    #[inline]
    pub const fn performance_rankings(&self, mode: GameMode) -> GetPerformanceRankings<'_> {
        GetPerformanceRankings::new(self, mode)
    }

    /// Get the recent activity of a user in form of a vec of
    /// [`Event`](crate::model::event::Event)s.
    #[inline]
    pub fn recent_activity(&self, user_id: impl Into<UserId>) -> GetRecentActivity<'_> {
        GetRecentActivity::new(self, user_id.into())
    }

    /// Get the replay of a score in form of a [`Replay`](osu_db::Replay).
    #[cfg(feature = "replay")]
    #[cfg_attr(docsrs, doc(cfg(feature = "replay")))]
    #[inline]
    pub fn replay(&self, score_id: u64) -> GetReplay<'_> {
        GetReplay::new(self, score_id)
    }

    /// Get the bytes of a replay of a score in form of a `Vec<u8>`.
    #[inline]
    pub fn replay_raw(&self, score_id: u64) -> GetReplayRaw<'_> {
        GetReplayRaw::new(self, score_id)
    }

    /// Get a [`Score`](crate::model::score::Score) struct.
    ///
    /// The contained score will have the following options filled:
    /// `map` (will contain `checksum` and `max_combo`), `mapset`
    /// (will contain `artist_unicode` and `title_unicode`), `pp`
    /// (if ranked), `rank_global` (if on leaderboard map) and `user`
    /// (will contain `last_visited`, `country`, `cover` and `groups`)
    #[inline]
    pub const fn score(&self, score_id: u64) -> GetScore<'_> {
        GetScore::new(self, score_id)
    }

    /// Get a list of recently processed [`Score`](crate::model::score::Score)
    /// structs.
    #[inline]
    pub const fn scores(&self) -> GetScores<'_> {
        GetScores::new(self)
    }

    /// Get a [`Rankings`](crate::model::ranking::Rankings) struct whose
    /// [`User`](crate::model::user::User)s are sorted
    /// by their ranked score, i.e. the current ranked score leaderboard.
    #[inline]
    pub const fn score_rankings(&self, mode: GameMode) -> GetScoreRankings<'_> {
        GetScoreRankings::new(self, mode)
    }

    /// Get [`SeasonalBackgrounds`](crate::model::seasonal_backgrounds::SeasonalBackgrounds).
    #[inline]
    pub fn seasonal_backgrounds(&self) -> GetSeasonalBackgrounds<'_> {
        GetSeasonalBackgrounds::new(self)
    }

    /// Get the vec of [`Spotlight`](crate::model::ranking::Spotlight).
    #[inline]
    pub fn spotlights(&self) -> GetSpotlights<'_> {
        GetSpotlights::new(self)
    }

    /// Get a [`TeamRankings`](crate::model::ranking::TeamRankings) struct whose
    /// entries are sorted by pp.
    #[inline]
    pub const fn team_rankings(&self, mode: GameMode) -> GetTeamRankings<'_> {
        GetTeamRankings::new(self, mode)
    }

    /// Get a [`UserExtended`](crate::model::user::UserExtended).
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
    pub fn user(&self, user_id: impl Into<UserId>) -> GetUser<'_> {
        GetUser::new(self, user_id)
    }

    /// Get a vec of [`BeatmapsetExtended`](crate::model::beatmap::BeatmapsetExtended)s a user made.
    ///
    /// Filled options will be: `artist_unicode`, `legacy_thread_url`, `maps`, `title_unicode`.
    ///
    /// All options of the contained [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended)s will be `None`.
    #[inline]
    pub fn user_beatmapsets(
        &self,
        user_id: impl Into<UserId>,
        kind: UserBeatmapsetsKind,
    ) -> GetUserBeatmapsets<'_> {
        GetUserBeatmapsets::new(self, user_id.into(), kind)
    }

    /// Get a vec of a user's [`MostPlayedMap`](crate::model::beatmap::MostPlayedMap)s.
    ///
    /// All options of the contained [`Beatmap`](crate::model::beatmap::Beatmap) and
    /// [`Beatmapset`](crate::model::beatmap::Beatmapset) will be `None`.
    ///
    /// ## Limit
    ///
    /// The API provides at most 100 results, defaults to 5.
    #[inline]
    pub fn user_most_played(&self, user_id: impl Into<UserId>) -> GetUserMostPlayed<'_> {
        GetUserMostPlayed::new(self, user_id.into())
    }

    /// Get either top, global firsts, pinned, or recent scores of a user,
    /// i.e. a vec of [`Score`](crate::model::score::Score).
    ///
    /// If no score type is specified by either
    /// [`best`](crate::request::GetUserScores::best),
    /// [`firsts`](crate::request::GetUserScores::firsts),
    /// [`pinned`](crate::request::GetUserScores::pinned),
    /// or [`recent`](crate::request::GetUserScores::recent), it defaults to `best`.
    ///
    /// The resulting scores will have these options filled: `map`, `mapset`, `pp`, `user`
    ///
    /// Additionally, the `best` score type will provide the `weight` option.
    ///
    /// All options of the contained [`BeatmapExtended`](crate::model::beatmap::BeatmapExtended),
    /// [`Beatmapset`](crate::model::beatmap::Beatmapset), and
    /// [`User`](crate::model::user::User) will be `None`.
    ///
    /// ## Note
    ///
    /// - The API provides at most 100 results per requests and defaults to 5.
    /// - For the `recent` score type, failed score are excluded by default.
    ///   Use [`include_fails`](crate::request::GetUserScores::include_fails)
    ///   to include them.
    /// - For the `firsts` score type, `pp` will only be `Some` if the map
    ///   is not loved.
    #[inline]
    pub fn user_scores(&self, user_id: impl Into<UserId>) -> GetUserScores<'_> {
        GetUserScores::new(self, user_id.into())
    }

    /// Get a vec of [`User`](crate::model::user::User).
    #[inline]
    pub fn users<I>(&self, user_ids: I) -> GetUsers<'_>
    where
        I: IntoIterator<Item = u32>,
    {
        GetUsers::new(self, user_ids)
    }

    /// Get a [`WikiPage`](crate::model::wiki::WikiPage) or image data.
    ///
    /// `locale` adjusts the language, e.g. `en` for english, `de` for german, ...
    #[inline]
    pub fn wiki(&self, locale: impl Into<String>) -> GetWikiPage<'_> {
        GetWikiPage::new(self, locale)
    }

    pub(crate) async fn request<T: DeserializeOwned>(&self, req: Request) -> OsuResult<T> {
        self.inner.request(req).await
    }

    pub(crate) async fn request_raw(&self, req: Request) -> OsuResult<Bytes> {
        self.inner.request_raw(req).await
    }
}

#[cfg(feature = "cache")]
impl Osu {
    pub(crate) async fn cache_user(&self, user_id: UserId) -> OsuResult<u32> {
        match user_id {
            UserId::Id(id) => Ok(id),
            UserId::Name(mut name) => {
                // osu! usernames are ASCII-only
                name.make_ascii_lowercase();

                if let Some(id) = self.cache.get(&name) {
                    return Ok(*id.value());
                }

                let user = self.user(UserId::Name(name.clone())).await?;
                self.cache.insert(name, user.user_id);

                #[cfg(feature = "metrics")]
                // ! BUG: It's possible to increment twice for the same name due to
                // ! concurrent function calls but since `DashMap::len` is a non-trivial
                // ! method to call and `cache_user` is called frequently, it's hopefully
                // ! fine to just naively increment here and ignore double-countings.
                ::metrics::counter!("osu_username_cache_size").increment(1);

                Ok(user.user_id)
            }
        }
    }

    pub(crate) fn update_cache(&self, user_id: u32, username: &crate::prelude::Username) {
        let mut name = username.to_owned();
        name.make_ascii_lowercase();
        self.cache.insert(name, user_id);
    }
}

impl Drop for Osu {
    #[inline]
    fn drop(&mut self) {
        if let Some(tx) = self.token_loop_tx.take() {
            let _ = tx.send(());
        }
    }
}

type Body = Full<Bytes>;

pub(crate) struct OsuRef {
    client_id: u64,
    client_secret: Box<str>,
    http: HyperClient<HttpsConnector<HttpConnector>, Body>,
    timeout: Duration,
    ratelimiter: LeakyBucket,
    token: RwLock<Token>,
    retries: usize,
}

static MY_USER_AGENT: &str = concat!(
    "Rust API v2 (",
    env!("CARGO_PKG_REPOSITORY"),
    " v",
    env!("CARGO_PKG_VERSION"),
    ")",
);

const APPLICATION_JSON: &str = "application/json";
const X_API_VERSION: &str = "x-api-version";

impl OsuRef {
    async fn request_client_token(&self) -> OsuResult<TokenResponse> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "client_credentials");
        let mut scopes = String::new();
        Scopes::Public.format(&mut scopes, ' ');
        body.push_str("scope", &scopes);

        self.finish_token_request(body).await
    }

    async fn request_user_token(&self, auth: &Authorization) -> OsuResult<TokenResponse> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "authorization_code");
        body.push_str("redirect_uri", &auth.redirect_uri);
        body.push_str("code", &auth.code);
        let mut scopes = String::new();
        auth.scopes.format(&mut scopes, ' ');
        body.push_str("scope", &scopes);

        self.finish_token_request(body).await
    }

    async fn request_refresh_token(&self, refresh: &str) -> OsuResult<TokenResponse> {
        let mut body = JsonBody::new();

        body.push_str("grant_type", "refresh_token");
        body.push_str("refresh_token", refresh);

        self.finish_token_request(body).await
    }

    async fn finish_token_request(&self, mut body: JsonBody) -> OsuResult<TokenResponse> {
        body.push_int("client_id", self.client_id);
        body.push_str("client_secret", &self.client_secret);

        let bytes = body.into_bytes();
        let len = bytes.len();
        let body = Full::from(bytes);
        let url = "https://osu.ppy.sh/oauth/token";

        let req = HyperRequest::post(url)
            .header(USER_AGENT, MY_USER_AGENT)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_TYPE, APPLICATION_JSON)
            .header(CONTENT_LENGTH, len)
            .body(body)?;

        let resp = self.send_request(req).await?;
        let bytes = self.handle_status(resp).await?;

        parse_bytes(&bytes)
    }

    async fn request<T: DeserializeOwned>(&self, req: Request) -> OsuResult<T> {
        let bytes = self.request_raw(req).await?;

        // let text = String::from_utf8_lossy(&bytes);
        // println!("Response:\n{text}");

        parse_bytes(&bytes)
    }

    async fn request_raw(&self, req: Request) -> OsuResult<Bytes> {
        let Request {
            query,
            route,
            body,
            api_version,
        } = req;

        let (method, path) = route.to_parts();

        #[cfg(feature = "metrics")]
        let start = std::time::Instant::now();

        let mut url = format!("https://osu.ppy.sh/api/v2/{path}");

        if let Some(ref query) = query {
            url.push('?');
            url.push_str(query);
        }

        let resp = self.raw(url, method, body, api_version).await?;
        let bytes = self.handle_status(resp).await?;

        #[cfg(feature = "metrics")]
        ::metrics::histogram!("osu_response_time", "route" => route.name()).record(start.elapsed());

        Ok(bytes)
    }

    async fn raw(
        &self,
        url: String,
        method: Method,
        body: JsonBody,
        api_version: u32,
    ) -> OsuResult<Response<Incoming>> {
        let url = Url::parse(&url).map_err(|source| OsuError::Url { source, url })?;
        debug!("URL: {url}");

        let Some(ref token) = self.token.read().await.access else {
            return Err(OsuError::NoToken);
        };

        let value = HeaderValue::from_str(token)
            .map_err(|source| OsuError::CreatingTokenHeader { source })?;

        let bytes = body.into_bytes();
        let len = bytes.len();
        let body = Body::from(bytes);

        let mut req_builder = HyperRequest::builder()
            .method(method)
            .uri(url.as_str())
            .header(AUTHORIZATION, value)
            .header(USER_AGENT, MY_USER_AGENT)
            .header(X_API_VERSION, api_version)
            .header(ACCEPT, APPLICATION_JSON)
            .header(CONTENT_LENGTH, len);

        if len > 0 {
            req_builder = req_builder.header(CONTENT_TYPE, APPLICATION_JSON);
        }

        let req = req_builder.body(body)?;

        self.send_request(req).await
    }

    async fn send_request(&self, req: HyperRequest<Body>) -> OsuResult<Response<Incoming>> {
        self.ratelimiter.acquire_one().await;

        let mut attempt = 0;

        loop {
            let req = clone_req(&req);

            match tokio::time::timeout(self.timeout, self.http.request(req)).await {
                Ok(res) => return res.map_err(|source| OsuError::Request { source }),
                Err(_) if attempt < self.retries => {
                    warn!("Timed out on attempt {attempt}, retry...");
                    attempt += 1;
                }
                Err(_) => return Err(OsuError::RequestTimeout),
            }
        }
    }

    async fn handle_status(&self, resp: Response<Incoming>) -> OsuResult<Bytes> {
        let status = resp.status();

        let bytes = resp
            .into_body()
            .collect()
            .await
            .map_err(|source| OsuError::ChunkingResponse { source })?
            .to_bytes();

        match status {
            StatusCode::OK => return Ok(bytes),
            StatusCode::NOT_FOUND => return Err(OsuError::NotFound),
            StatusCode::SERVICE_UNAVAILABLE => {
                let body = String::from_utf8_lossy(&bytes).into_owned();

                return Err(OsuError::ServiceUnavailable(body));
            }
            StatusCode::TOO_MANY_REQUESTS => warn!("Got a 429 response"),
            _ => {}
        }

        let body = String::from_utf8_lossy(&bytes).into_owned();

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
}

#[inline]
fn parse_bytes<T: DeserializeOwned>(bytes: &Bytes) -> OsuResult<T> {
    serde_json::from_slice(bytes).map_err(|source| {
        let body = String::from_utf8_lossy(bytes).into_owned();

        OsuError::Parsing { body, source }
    })
}

fn clone_req(req: &HyperRequest<Body>) -> HyperRequest<Body> {
    let mut builder = HyperRequest::builder().method(req.method()).uri(req.uri());

    if let Some(headers) = builder.headers_mut() {
        req.headers().clone_into(headers);
    }

    builder.body(req.body().to_owned()).unwrap()
}
