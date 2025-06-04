mod builder;
mod scopes;
mod token;

use bytes::Bytes;
use http_body_util::Full;
use hyper_util::client::legacy::{connect::HttpConnector, Client as HyperClient};

pub use self::{builder::OsuBuilder, scopes::Scopes, token::Token};

pub(crate) use self::token::{Authorization, TokenResponse};

use self::token::{AuthorizationKind, CurrentToken};

#[allow(clippy::wildcard_imports)]
use crate::{
    model::{user::UserBeatmapsetsKind, GameMode},
    request::*,
    OsuResult,
};

use hyper_rustls::HttpsConnector;
use leaky_bucket::RateLimiter;
use std::{ops::Drop, sync::Arc, time::Duration};
use tokio::sync::oneshot::Sender;

/// The main osu client.
pub struct Osu {
    pub(crate) inner: Arc<OsuInner>,
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
    pub fn token(&self) -> Token {
        self.inner.token.get(Token::to_owned)
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

    /// Get [`BeatmapScores`](crate::model::beatmap::BeatmapScores).
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
    pub const fn beatmap_difficulty_attributes(
        &self,
        map_id: u32,
    ) -> GetBeatmapDifficultyAttributes<'_> {
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
    pub const fn beatmapset(&self, mapset_id: u32) -> GetBeatmapset<'_> {
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
    pub const fn beatmapset_from_map_id(&self, map_id: u32) -> GetBeatmapsetFromMapId<'_> {
        GetBeatmapsetFromMapId::new(self, map_id)
    }

    /// Get a [`BeatmapsetEvents`](crate::model::beatmap::BeatmapsetEvents)
    /// struct containing the most recent mapset events.
    #[inline]
    pub const fn beatmapset_events(&self) -> GetBeatmapsetEvents<'_> {
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
    pub const fn news(&self) -> GetNews<'_> {
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
    pub const fn osu_matches(&self) -> GetMatches<'_> {
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
    pub const fn replay(&self, score_id: u64) -> GetReplay<'_> {
        GetReplay::new(self, score_id)
    }

    /// Get the bytes of a replay of a score in form of a `Vec<u8>`.
    #[inline]
    pub const fn replay_raw(&self, score_id: u64) -> GetReplayRaw<'_> {
        GetReplayRaw::new(self, score_id)
    }

    /// Get a [`Room`](crate::model::multiplayer::Room).
    #[inline]
    pub const fn room(&self, room_id: u64) -> GetRoom<'_> {
        GetRoom::new(self, room_id)
    }

    /// Get [`RoomEvents`](crate::model::multiplayer::RoomEvents).
    #[inline]
    pub const fn room_events(&self, room_id: u64) -> GetRoomEvents<'_> {
        GetRoomEvents::new(self, room_id)
    }

    /// Get a [`RoomLeaderboard`](crate::model::multiplayer::RoomLeaderboard).
    #[inline]
    pub const fn room_leaderboard(&self, room_id: u64) -> GetRoomLeaderboard<'_> {
        GetRoomLeaderboard::new(self, room_id)
    }

    /// Get a vec of [`Room`](crate::model::multiplayer::Room).
    #[inline]
    pub const fn rooms(&self) -> GetRooms<'_> {
        GetRooms::new(self)
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
    pub const fn seasonal_backgrounds(&self) -> GetSeasonalBackgrounds<'_> {
        GetSeasonalBackgrounds::new(self)
    }

    /// Get the vec of [`Spotlight`](crate::model::ranking::Spotlight).
    #[inline]
    pub const fn spotlights(&self) -> GetSpotlights<'_> {
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
        GetUser::new(self, user_id.into())
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
}

impl Drop for Osu {
    fn drop(&mut self) {
        if let Some(tx) = self.token_loop_tx.take() {
            let _ = tx.send(());
        }
    }
}

pub(crate) struct OsuInner {
    pub(crate) client_id: u64,
    pub(crate) client_secret: Box<str>,
    pub(crate) http: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    pub(crate) timeout: Duration,
    pub(crate) ratelimiter: Arc<RateLimiter>,
    pub(crate) token: CurrentToken,
    pub(crate) retries: u8,
    #[cfg(feature = "cache")]
    pub(crate) cache: dashmap::DashMap<crate::prelude::Username, u32>,
}

#[cfg(feature = "cache")]
impl OsuInner {
    pub(crate) fn update_cache(&self, user_id: u32, username: &crate::prelude::Username) {
        let mut name = username.to_owned();
        name.make_ascii_lowercase();
        self.cache.insert(name, user_id);
    }
}
