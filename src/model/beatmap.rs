use super::{user::UserCompact, GameMode};
use crate::{request::GetUser, Osu};

use chrono::{DateTime, Utc};
use serde::{
    de::{Deserializer, Error, IgnoredAny, MapAccess, Visitor},
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmap {
    pub ar: f32,
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
    pub mapset: Option<Mapset>,
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
    pub mapset: Option<Mapset>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Beatmapset {
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    pub availability: BeatmapsetAvailability,
    pub bpm: f32,
    pub can_be_hyped: bool,
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
    pub discussion_id: Option<u64>,
    #[serde(
        default,
        rename = "beatmap_discussion_post_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub discussion_post_id: Option<u64>,
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
pub struct BeatmapsetCompact {
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    pub covers: BeatmapsetCovers,
    pub creator: String,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetCovers {
    pub cover: String,
    #[serde(rename = "cover@2x")]
    pub cover_2x: String,
    pub card: String,
    #[serde(rename = "card@2x")]
    pub card_2x: String,
    pub list: String,
    #[serde(rename = "list@2x")]
    pub list_2x: String,
    #[serde(rename = "slimcover")]
    pub slim_cover: String,
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
pub enum BeatmapsetEvent {
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
    LanguageEdit {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<Language>,
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
    #[serde(rename = "beatmap_discussion_id")]
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

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct BeatmapsetVote {
    pub user_id: u32,
    pub score: u32,
}

// TODO: Allocate with capacity
// TODO: Make these [u32; 100], serde currently only goes up to 32
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FailTimes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Mapset {
    Full(Beatmapset),
    Compact(BeatmapsetCompact),
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

macro_rules! impl_get {
    ($func:ident -> $ret:ident) => {
        #[inline]
        pub fn $func(&self) -> $ret {
            match self {
                Self::Full(set) => set.$func,
                Self::Compact(set) => set.$func,
            }
        }
    };

    ($func:ident -> &$ret:ident) => {
        #[inline]
        pub fn $func(&self) -> &$ret {
            match self {
                Self::Full(set) => &set.$func,
                Self::Compact(set) => &set.$func,
            }
        }
    };
}

impl Mapset {
    #[inline]
    pub fn artist_unicode(&self) -> Option<&str> {
        match self {
            Self::Full(set) => set.artist_unicode.as_deref(),
            Self::Compact(set) => set.artist_unicode.as_deref(),
        }
    }

    #[inline]
    pub fn creator(&self) -> &str {
        match self {
            Self::Full(set) => &set.creator_name,
            Self::Compact(set) => &set.creator,
        }
    }

    #[inline]
    pub fn title_unicode(&self) -> Option<&str> {
        match self {
            Self::Full(set) => set.title_unicode.as_deref(),
            Self::Compact(set) => set.title_unicode.as_deref(),
        }
    }

    #[inline]
    pub fn get_creator<'o>(&self, osu: &'o Osu) -> GetUser<'o> {
        osu.user(self.creator_id())
    }

    impl_get!(artist -> &str);
    impl_get!(covers -> &BeatmapsetCovers);
    impl_get!(creator_id -> u32);
    impl_get!(favourite_count -> u32);
    impl_get!(mapset_id -> u32);
    impl_get!(playcount -> u32);
    impl_get!(preview_url -> &str);
    impl_get!(source -> &str);
    impl_get!(status -> RankStatus);
    impl_get!(title -> &str);
    impl_get!(video -> bool);
}

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
                "description" => {
                    description.replace(map.next_value()?);
                }
                _ => {
                    let _: IgnoredAny = map.next_value()?;
                }
            }
        }

        description
            .ok_or_else(|| Error::missing_field("description"))
            .map(Some)
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
