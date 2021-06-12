use super::{user::UserCompact, GameMode};
use crate::{request::GetUser, Osu, OsuResult};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{convert::TryFrom, fmt, str::FromStr};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmap {
    pub ar: f32,
    #[serde(deserialize_with = "deserialize_f32_default")]
    pub bpm: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    pub convert: bool,
    pub count_circles: u32,
    pub count_sliders: u32,
    pub count_spinners: u32,
    pub cs: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_times: Option<FailTimes>,
    #[serde(rename = "drain")]
    pub hp: f32,
    pub is_scoreable: bool,
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "id")]
    pub map_id: u32,
    #[serde(
        default,
        rename = "beatmapset",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapset: Option<Beatmapset>,
    #[serde(rename = "beatmapset_id")]
    pub mapset_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    #[serde(rename = "accuracy")]
    pub od: f32,
    pub passcount: u32,
    pub playcount: u32,
    #[serde(rename = "hit_length")]
    pub seconds_drain: u32,
    #[serde(rename = "total_length")]
    pub seconds_total: u32,
    #[serde(rename = "difficulty_rating")]
    pub stars: f32,
    pub status: RankStatus,
    /// Full URL, i.e. `https://osu.ppy.sh/beatmaps/{map_id}`
    pub url: String,
    pub version: String,
}

impl Beatmap {
    #[inline]
    pub fn count_objects(&self) -> u32 {
        self.count_circles + self.count_sliders + self.count_spinners
    }
}

impl PartialEq for Beatmap {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_id == other.map_id && self.last_updated == other.last_updated
    }
}

impl Eq for Beatmap {}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BeatmapCompact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_times: Option<FailTimes>,
    #[serde(rename = "id")]
    pub map_id: u32,
    #[serde(
        default,
        rename = "beatmapset",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapset: Option<BeatmapsetCompact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_combo: Option<u32>,
    pub mode: GameMode,
    #[serde(rename = "total_length")]
    pub seconds_total: u32,
    #[serde(rename = "difficulty_rating")]
    pub stars: f32,
    pub status: RankStatus,
    pub version: String,
}

impl From<Beatmap> for BeatmapCompact {
    fn from(map: Beatmap) -> Self {
        Self {
            checksum: map.checksum,
            fail_times: map.fail_times,
            map_id: map.map_id,
            mapset: map.mapset.map(|ms| ms.into()),
            max_combo: map.max_combo,
            mode: map.mode,
            seconds_total: map.seconds_total,
            stars: map.stars,
            status: map.status,
            version: map.version,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmapset {
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    pub availability: BeatmapsetAvailability,
    #[serde(deserialize_with = "deserialize_f32_default")]
    pub bpm: f32,
    pub can_be_hyped: bool,
    /// Each difficulty's converted map for each mode
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub converts: Option<Vec<Beatmap>>,
    pub covers: BeatmapsetCovers,
    #[serde(default, rename = "user", skip_serializing_if = "Option::is_none")]
    pub creator: Option<UserCompact>,
    #[serde(rename = "creator")]
    pub creator_name: String,
    #[serde(rename = "user_id")]
    pub creator_id: u32,
    #[serde(
        default,
        deserialize_with = "flatten_description",
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    pub discussion_enabled: bool,
    pub discussion_locked: bool,
    pub favourite_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<Genre>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hype: Option<BeatmapsetHype>,
    pub is_scoreable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    pub last_updated: DateTime<Utc>,
    /// Full URL, i.e. `https://osu.ppy.sh/community/forums/topics/{thread_id}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_thread_url: Option<String>,
    #[serde(default, rename = "beatmaps", skip_serializing_if = "Option::is_none")]
    pub maps: Option<Vec<Beatmap>>,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    pub nominations_summary: BeatmapsetNominations,
    pub nsfw: bool,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    /// Full URL, i.e. `b.ppy.sh/preview/{mapset_id}.mp3`
    pub preview_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ratings: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranked_date: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recent_favourites: Option<Vec<UserCompact>>,
    pub source: String,
    pub status: RankStatus,
    pub storyboard: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted_date: Option<DateTime<Utc>>,
    pub tags: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_unicode: Option<String>,
    pub video: bool,
}

#[inline]
fn deserialize_f32_default<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    Option::<f32>::deserialize(d).map(Option::unwrap_or_default)
}

impl Beatmapset {
    #[inline]
    pub fn get_creator<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.creator_id)
    }
}

impl PartialEq for Beatmapset {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mapset_id == other.mapset_id && self.last_updated == other.last_updated
    }
}

impl Eq for Beatmapset {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeatmapsetAvailability {
    pub download_disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_information: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCommentEdit<T> {
    #[serde(flatten)]
    pub comment_id: BeatmapsetCommentId,
    pub old: T,
    pub new: T,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCommentId {
    #[serde(
        default,
        rename = "beatmap_discussion_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_discussion_id: Option<u64>,
    #[serde(
        default,
        rename = "beatmap_discussion_post_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_discussion_post_id: Option<u64>,
    #[serde(
        default,
        rename = "beatmapset_discussion_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapset_discussion_id: Option<u64>,
    #[serde(
        default,
        rename = "beatmapset_discussion_post_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub mapset_discussion_post_id: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCommentKudosuGain {
    #[serde(flatten)]
    pub comment_id: BeatmapsetCommentId,
    pub new_vote: BeatmapsetVote,
    pub votes: Vec<BeatmapsetVote>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCommentNominate {
    pub modes: Vec<GameMode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCommentOwnerChange {
    #[serde(
        default,
        rename = "beatmap_discussion_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_discussion_id: Option<u64>,
    #[serde(
        default,
        rename = "beatmap_discussion_post_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_discussion_post_id: Option<u64>,
    #[serde(rename = "beatmap_id")]
    pub map_id: u32,
    #[serde(rename = "beatmap_version")]
    pub version: String,
    pub new_user_id: u32,
    #[serde(rename = "new_user_username")]
    pub new_username: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCompact {
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    pub covers: BeatmapsetCovers,
    #[serde(rename = "creator")]
    pub creator_name: String,
    #[serde(rename = "user_id")]
    pub creator_id: u32,
    pub favourite_count: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub genre: Option<Genre>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hype: Option<BeatmapsetHype>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(rename = "id")]
    pub mapset_id: u32,
    pub nsfw: bool,
    #[serde(rename = "play_count")]
    pub playcount: u32,
    /// Full URL, i.e. `b.ppy.sh/preview/{mapset_id}.mp3`
    pub preview_url: String,
    pub source: String,
    pub status: RankStatus,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_unicode: Option<String>,
    pub video: bool,
}

impl BeatmapsetCompact {
    #[inline]
    pub fn get_creator<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.creator_id)
    }
}

impl From<Beatmapset> for BeatmapsetCompact {
    fn from(mapset: Beatmapset) -> Self {
        Self {
            artist: mapset.artist,
            artist_unicode: mapset.artist_unicode,
            covers: mapset.covers,
            creator_name: mapset.creator_name,
            creator_id: mapset.creator_id,
            favourite_count: mapset.favourite_count,
            genre: mapset.genre,
            hype: mapset.hype,
            language: mapset.language,
            mapset_id: mapset.mapset_id,
            nsfw: mapset.nsfw,
            playcount: mapset.playcount,
            preview_url: mapset.preview_url,
            source: mapset.source,
            status: mapset.status,
            title: mapset.title,
            title_unicode: mapset.title_unicode,
            video: mapset.video,
        }
    }
}

/// URLs to various sizes of (parts of) the background picture
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCovers {
    /// Lengthy part of the background
    pub cover: String,
    /// Same as `cover` but larger
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    /// Same as `cover` but much smaller
    pub card: String,
    /// Same as `card` but larger
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    /// Tiny preview of full the
    pub list: String,
    /// Small preview of full the background background
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    /// Same as `cover` but much larger
    #[serde(rename = "slimcover")]
    pub slim_cover: String,
    /// Same as `cover` but huge
    #[serde(rename = "slimcover@2x")]
    pub slim_cover_2x: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BeatmapsetDiscussion {
    #[serde(rename = "id")]
    pub discussion_id: u64,
    #[serde(rename = "beatmapset_id")]
    pub mapset_id: u32,
    #[serde(
        default,
        rename = "beatmap_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub map_id: Option<u32>,
    pub user_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_id: Option<u32>,
    pub message_type: String, // TODO
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<u64>,
    pub resolved: bool,
    pub can_be_resolved: bool,
    pub can_grant_kudosu: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
    pub last_post_at: DateTime<Utc>,
    pub kudosu_denied: bool,
    pub starting_post: BeatmapsetPost,
}

impl PartialEq for BeatmapsetDiscussion {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.discussion_id == other.discussion_id && self.updated_at == other.updated_at
    }
}

impl Eq for BeatmapsetDiscussion {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
#[non_exhaustive]
pub enum BeatmapsetEvent {
    Disqualify {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    GenreEdit {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<Genre>,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    IssueReopen {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    IssueResolve {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuDeny {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        created_at: DateTime<Utc>,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuGain {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentKudosuGain,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuLost {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentKudosuGain,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    LanguageEdit {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<Language>,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Love {
        #[serde(rename = "id")]
        event_id: u64,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Nominate {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentNominate,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    NsfwToggle {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<bool>,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    #[serde(rename = "beatmap_owner_change")]
    OwnerChange {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentOwnerChange,
        created_at: DateTime<Utc>,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Rank {
        #[serde(rename = "id")]
        event_id: u64,
        created_at: DateTime<Utc>,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Qualify {
        #[serde(rename = "id")]
        event_id: u64,
        created_at: DateTime<Utc>,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BeatmapsetEvents {
    pub events: Vec<BeatmapsetEvent>,
    #[serde(rename = "reviewsConfig")]
    pub reviews_config: BeatmapsetReviewsConfig,
    pub users: Vec<UserCompact>,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetHype {
    pub current: u32,
    pub required: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetNominations {
    pub current: u32,
    pub required: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetPost {
    #[serde(rename = "id")]
    pub post_id: u64,
    #[serde(rename = "beatmapset_discussion_id")]
    pub discussion_id: u64,
    pub user_id: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_editor_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_by_id: Option<u32>,
    pub system: bool,
    pub message: String,
    pub created_at: DateTime<Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetReviewsConfig {
    pub max_blocks: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct BeatmapsetSearchCursor {
    #[serde(rename = "_id")]
    pub(crate) id: String,
    #[serde(
        default,
        rename = "play_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) playcount: Option<String>,
    #[serde(default, rename = "_score", skip_serializing_if = "Option::is_none")]
    pub(crate) score: Option<f32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum SearchRankStatus {
    Any,
    Specific(RankStatus),
}

const SEARCH_RANK_STATUS_ANY: i8 = 9;

struct SearchRankStatusVisitor;

impl<'de> Visitor<'de> for SearchRankStatusVisitor {
    type Value = SearchRankStatus;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a rank status, `any`, or 9")
    }

    fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
        let visitor = super::EnumVisitor::<RankStatus>::new();

        if let Ok(status) = visitor.visit_str::<E>(s) {
            Ok(SearchRankStatus::Specific(status))
        } else if s == "any" {
            Ok(SearchRankStatus::Any)
        } else {
            Err(Error::invalid_value(
                Unexpected::Str(s),
                &"a rank status or `any`",
            ))
        }
    }

    fn visit_u64<E: Error>(self, u: u64) -> Result<Self::Value, E> {
        let visitor = super::EnumVisitor::<RankStatus>::new();

        if let Ok(status) = visitor.visit_u64::<E>(u) {
            Ok(SearchRankStatus::Specific(status))
        } else if u as i8 == SEARCH_RANK_STATUS_ANY {
            Ok(SearchRankStatus::Any)
        } else {
            Err(Error::invalid_value(
                Unexpected::Unsigned(u),
                &"a rank status or 9",
            ))
        }
    }

    fn visit_i64<E: Error>(self, i: i64) -> Result<Self::Value, E> {
        let visitor = super::EnumVisitor::<RankStatus>::new();

        if let Ok(status) = visitor.visit_i64::<E>(i) {
            Ok(SearchRankStatus::Specific(status))
        } else if i as i8 == SEARCH_RANK_STATUS_ANY {
            Ok(SearchRankStatus::Any)
        } else {
            Err(Error::invalid_value(
                Unexpected::Signed(i),
                &"a rank status or 9",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for SearchRankStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(SearchRankStatusVisitor)
    }
}

impl Serialize for SearchRankStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Any => s.serialize_i8(SEARCH_RANK_STATUS_ANY),
            Self::Specific(status) => s.serialize_i8(*status as i8),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct BeatmapsetSearchParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mode: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<SearchRankStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) genre: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) language: Option<u8>,
    pub(crate) video: bool,
    pub(crate) storyboard: bool,
    pub(crate) nsfw: bool,
    #[serde(rename(serialize = "_sort"))]
    sort: BeatmapsetSearchSort,
    descending: bool,
}

impl Default for BeatmapsetSearchParameters {
    #[inline]
    fn default() -> Self {
        Self {
            query: None,
            mode: None,
            status: None,
            genre: None,
            language: None,
            video: false,
            storyboard: false,
            nsfw: true,
            sort: BeatmapsetSearchSort::default(),
            descending: true,
        }
    }
}

struct BeatmapsetSearchParametersVisitor;

impl<'de> Visitor<'de> for BeatmapsetSearchParametersVisitor {
    type Value = BeatmapsetSearchParameters;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a search struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut params = None;

        let mut query = None;
        let mut mode = None;
        let mut status = None;
        let mut genre = None;
        let mut language = None;
        let mut video = None;
        let mut storyboard = None;
        let mut nsfw = None;
        let mut sort = None;
        let mut descending = None;

        while let Some(key) = map.next_key()? {
            match key {
                "sort" => {
                    let SubSort { sort, descending } = map.next_value()?;

                    params.replace(BeatmapsetSearchParameters {
                        sort,
                        descending,
                        ..Default::default()
                    });
                }
                "query" => query = map.next_value()?,
                "mode" => {
                    mode.replace(map.next_value()?);
                }
                "status" => {
                    status.replace(map.next_value()?);
                }
                "genre" => {
                    genre.replace(map.next_value()?);
                }
                "language" => {
                    language.replace(map.next_value()?);
                }
                "video" => {
                    video.replace(map.next_value()?);
                }
                "storyboard" => {
                    storyboard.replace(map.next_value()?);
                }
                "nsfw" => {
                    nsfw.replace(map.next_value()?);
                }
                "_sort" => {
                    sort.replace(map.next_value()?);
                }
                "descending" => {
                    descending.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        if let Some(params) = params {
            return Ok(params);
        }

        let sort = sort.ok_or_else(|| Error::missing_field("sort or _sort"))?;
        let video = video.ok_or_else(|| Error::missing_field("sort or video"))?;
        let storyboard = storyboard.ok_or_else(|| Error::missing_field("sort or storyboard"))?;
        let nsfw = nsfw.ok_or_else(|| Error::missing_field("sort or nsfw"))?;
        let descending = descending.ok_or_else(|| Error::missing_field("sort or descending"))?;

        let params = BeatmapsetSearchParameters {
            query,
            mode,
            status,
            genre,
            language,
            video,
            storyboard,
            nsfw,
            sort,
            descending,
        };

        Ok(params)
    }
}

impl<'de> Deserialize<'de> for BeatmapsetSearchParameters {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(BeatmapsetSearchParametersVisitor)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct BeatmapsetSearchResult {
    cursor: Option<BeatmapsetSearchCursor>,
    #[serde(rename(serialize = "beatmapsets"))]
    pub mapsets: Vec<Beatmapset>,
    #[serde(rename(serialize = "search"))]
    pub(crate) params: BeatmapsetSearchParameters,
    /// Total amount of mapsets that fit the search query
    pub total: u32,
}

impl BeatmapsetSearchResult {
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If `has_more()` is true, the API can provide the next set of search results and this method will request them.
    /// Otherwise, this method returns `None`.
    pub async fn get_next(&self, osu: &Osu) -> Option<OsuResult<BeatmapsetSearchResult>> {
        let cursor = self.cursor.as_ref()?.to_owned();
        let params = &self.params;

        let mut fut = osu
            .beatmapset_search()
            .cursor(cursor)
            .video(params.video)
            .storyboard(params.storyboard)
            .nsfw(params.nsfw)
            .sort(params.sort, params.descending);

        if let Some(ref query) = params.query {
            fut = fut.query(query);
        }

        if let Some(mode) = params.mode.map(GameMode::from) {
            fut = fut.mode(mode);
        }

        match params.status {
            None => {}
            Some(SearchRankStatus::Specific(status)) => fut = fut.status(status),
            Some(SearchRankStatus::Any) => fut = fut.any_status(),
        }

        if let Some(genre) = params.genre {
            fut = fut.genre(Genre::try_from(genre).unwrap());
        }

        if let Some(language) = params.language {
            fut = fut.language(Language::try_from(language).unwrap());
        }

        Some(fut.await)
    }
}

struct BeatmapsetSearchResultVisitor;

impl<'de> Visitor<'de> for BeatmapsetSearchResultVisitor {
    type Value = BeatmapsetSearchResult;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a BeatmapsetSearchResult struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut mapsets = None;
        let mut cursor = None;
        let mut params = None;
        let mut total = None;

        while let Some(key) = map.next_key()? {
            match key {
                "beatmapsets" => {
                    mapsets.replace(map.next_value()?);
                }
                "cursor" => cursor = map.next_value()?,
                "search" => {
                    params.replace(map.next_value()?);
                }
                "total" => {
                    total.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        let mapsets = mapsets.ok_or_else(|| Error::missing_field("beatmapsets"))?;
        let params = params.unwrap_or_default();
        let total = total.ok_or_else(|| Error::missing_field("total"))?;

        Ok(BeatmapsetSearchResult {
            cursor,
            mapsets,
            params,
            total,
        })
    }
}

impl<'de> Deserialize<'de> for BeatmapsetSearchResult {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(BeatmapsetSearchResultVisitor)
    }
}

macro_rules! search_sort_enum {
    ($($variant:ident => $name:literal,)+) => {
        /// Provides an option to specify a mapset order in a mapset search,
        /// see [`Osu::beatmapset_search`](crate::client::Osu::beatmapset_search).
        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        pub enum BeatmapsetSearchSort {
            $(
                #[serde(rename = $name)]
                $variant,
            )+
        }

        impl fmt::Display for BeatmapsetSearchSort {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Self::$variant => f.write_str($name),)+
                }


            }
        }

        impl FromStr for BeatmapsetSearchSort {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($name => Ok(Self::$variant),)+
                    _ => Err(()),
                }
            }
        }

        impl<'de> Deserialize<'de> for SubSort {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let s: &str = Deserialize::deserialize(d)?;

                let underscore = s.find('_').ok_or_else(|| {
                    Error::invalid_value(Unexpected::Str(s), &"a string containing an underscore")
                })?;

                let sort = s[..underscore].parse().map_err(|_| {
                    Error::invalid_value(
                        Unexpected::Str(&s[..underscore]),
                        &stringify!($($name),+),
                    )
                })?;

                let descending = match s.get(underscore + 1..) {
                    Some("desc") => true,
                    Some("asc") => false,
                    _ => return Err(Error::invalid_value(Unexpected::Str(s), &"*_desc or *_asc")),
                };

                Ok(SubSort { sort, descending })
            }
        }
    }
}

search_sort_enum!(
    Artist => "artist",
    Favourites => "favourites",
    Playcount => "plays",
    RankedDate => "ranked",
    Rating => "rating",
    Relevance => "relevance",
    Stars => "difficulty",
    Title => "title",
);

impl Default for BeatmapsetSearchSort {
    #[inline]
    fn default() -> Self {
        BeatmapsetSearchSort::Relevance
    }
}

struct SubSort {
    sort: BeatmapsetSearchSort,
    descending: bool,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetVote {
    pub user_id: u32,
    pub score: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FailTimes {
    #[serde(
        default,
        deserialize_with = "use_vec_option_visitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub exit: Option<Vec<u32>>,
    #[serde(
        default,
        deserialize_with = "use_vec_option_visitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub fail: Option<Vec<u32>>,
}

fn use_vec_option_visitor<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<u32>>, D::Error> {
    d.deserialize_option(VecOptionVisitor)
}

struct VecOptionVisitor;

impl<'de> Visitor<'de> for VecOptionVisitor {
    type Value = Option<Vec<u32>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "null or a sequence of u32")
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_seq(HundredU32Visitor::new()).map(Some)
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

struct HundredU32Visitor(Vec<u32>);

impl HundredU32Visitor {
    #[inline]
    fn new() -> Self {
        Self(Vec::with_capacity(100))
    }
}

impl<'de> Visitor<'de> for HundredU32Visitor {
    type Value = Vec<u32>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a sequence of u32")
    }

    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<Self::Value, A::Error> {
        while let Some(n) = seq.next_element()? {
            self.0.push(n);
        }

        Ok(self.0)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MostPlayedMap {
    pub count: usize,
    #[serde(rename = "beatmap")]
    pub map: BeatmapCompact,
    #[serde(rename = "beatmap_id")]
    pub map_id: u32,
    #[serde(rename = "beatmapset")]
    pub mapset: BeatmapsetCompact,
}

impl PartialEq for MostPlayedMap {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.map_id == other.map_id && self.count == other.count
    }
}

impl Eq for MostPlayedMap {}

def_enum!(i8 RankStatus {
    Graveyard = -2 ("graveyard"),
    WIP = -1 ("wip"),
    Pending = 0 ("pending"),
    Ranked = 1 ("ranked"),
    Approved = 2 ("approved"),
    Qualified = 3 ("qualified"),
    Loved = 4 ("loved"),
});

def_enum!(u8 Genre {
    Any = 0 ("Any"),
    Unspecified = 1 ("Unspecified"),
    VideoGame = 2 ("Video Game"),
    Anime = 3 ("Anime"),
    Rock = 4 ("Rock"),
    Pop = 5 ("Pop"),
    Other = 6 ("Other"),
    Novelty = 7 ("Novelty"),
    HipHop = 9 ("Hip Hop"),
    Electronic = 10 ("Electronic"),
    Metal = 11 ("Metal"),
    Classical = 12 ("Classical"),
    Folk = 13 ("Folk"),
    Jazz = 14 ("Jazz"),
});

impl Default for Genre {
    fn default() -> Self {
        Self::Any
    }
}

def_enum!(u8 Language {
    Any = 0,
    Other = 1,
    English = 2,
    Japanese = 3,
    Chinese = 4,
    Instrumental = 5,
    Korean = 6,
    French = 7,
    German = 8,
    Swedish = 9,
    Spanish = 10,
    Italian = 11,
    Russian = 12,
    Polish = 13,
    Unspecified = 14,
});

impl Default for Language {
    fn default() -> Self {
        Self::Any
    }
}

struct DescriptionVisitor;

impl<'de> Visitor<'de> for DescriptionVisitor {
    type Value = Option<String>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a string or a map containing a 'description' field")
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Some(v.to_owned()))
    }

    fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut description = None;

        while let Some(key) = map.next_key()? {
            match key {
                "description" => description = map.next_value()?,
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        Ok(description)
    }

    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }

    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

fn flatten_description<'de, D: Deserializer<'de>>(d: D) -> Result<Option<String>, D::Error> {
    d.deserialize_option(DescriptionVisitor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::DeserializeOwned;
    use std::fmt::Debug;

    fn ser_de<T: DeserializeOwned + Serialize + PartialEq + Debug>(val: T) {
        let serialized =
            serde_json::to_string(&val).unwrap_or_else(|e| panic!("Failed to serialize: {}", e));

        let deserialized: T = serde_json::from_str(&serialized)
            .unwrap_or_else(|e| panic!("Failed to deserialize: {}", e));

        assert_eq!(val, deserialized);
    }

    #[test]
    fn ser_de_search_result_any_status() {
        let search_result = BeatmapsetSearchResult {
            cursor: Some(BeatmapsetSearchCursor {
                id: "123".to_owned(),
                score: Some(3.1415),
                playcount: None,
            }),
            mapsets: Vec::new(),
            params: BeatmapsetSearchParameters {
                query: Some("my query".to_owned()),
                mode: Some(1),
                status: Some(SearchRankStatus::Any),
                genre: Some(4),
                language: Some(5),
                video: true,
                storyboard: false,
                nsfw: false,
                sort: BeatmapsetSearchSort::RankedDate,
                descending: false,
            },
            total: 42,
        };

        ser_de(search_result);
    }

    #[test]
    fn ser_de_search_result_specific_status() {
        let search_result = BeatmapsetSearchResult {
            cursor: Some(BeatmapsetSearchCursor {
                id: "123".to_owned(),
                score: None,
                playcount: Some("123".to_owned()),
            }),
            mapsets: Vec::new(),
            params: BeatmapsetSearchParameters {
                query: None,
                mode: Some(1),
                status: Some(SearchRankStatus::Specific(RankStatus::Pending)),
                genre: None,
                language: Some(5),
                video: true,
                storyboard: false,
                nsfw: true,
                sort: BeatmapsetSearchSort::Playcount,
                descending: true,
            },
            total: 42,
        };

        ser_de(search_result);
    }
}
