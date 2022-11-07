use super::{serde_, user_::UserCompact, Cursor, GameMode};
use crate::{
    error::ParsingError,
    prelude::{CountryCode, OsuError, Username},
    request::{GetBeatmapDifficultyAttributes, GetUser},
    Osu, OsuResult,
};

use serde::{
    de::{
        DeserializeSeed, Deserializer, Error, IgnoredAny, MapAccess, SeqAccess, Unexpected, Visitor,
    },
    ser::Serializer,
    Deserialize, Serialize,
};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter, Result as FmtResult},
    str::FromStr,
};
use time::OffsetDateTime;

#[cfg(feature = "rkyv")]
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
    #[serde(rename = "user_id")]
    pub creator_id: u32,
    pub cs: f32,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub deleted_at: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_times: Option<FailTimes>,
    #[serde(rename = "drain")]
    pub hp: f32,
    pub is_scoreable: bool,
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub last_updated: OffsetDateTime,
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
    /// Return the amount of hit objects in this map.
    #[inline]
    pub fn count_objects(&self) -> u32 {
        self.count_circles + self.count_sliders + self.count_spinners
    }

    /// Request the [`BeatmapDifficultyAttributes`] for this map.
    #[inline]
    pub fn difficulty_attributes<'o>(&self, osu: &'o Osu) -> GetBeatmapDifficultyAttributes<'o> {
        GetBeatmapDifficultyAttributes::new(osu, self.map_id)
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapCompact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "user_id")]
    pub creator_id: u32,
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

impl BeatmapCompact {
    /// Request the [`BeatmapDifficultyAttributes`] for this map.
    #[inline]
    pub fn difficulty_attributes<'o>(&self, osu: &'o Osu) -> GetBeatmapDifficultyAttributes<'o> {
        GetBeatmapDifficultyAttributes::new(osu, self.map_id)
    }
}

impl From<Beatmap> for BeatmapCompact {
    #[inline]
    fn from(map: Beatmap) -> Self {
        Self {
            checksum: map.checksum,
            creator_id: map.creator_id,
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

#[derive(Deserialize)]
pub(crate) struct Beatmaps {
    #[serde(rename = "beatmaps")]
    pub(crate) maps: Vec<BeatmapCompact>,
}

#[derive(Deserialize)]
pub(crate) struct BeatmapDifficultyAttributesWrapper {
    pub attributes: BeatmapDifficultyAttributes,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapDifficultyAttributes {
    pub max_combo: u32,
    #[serde(rename = "star_rating")]
    pub stars: f32,
    #[serde(flatten)]
    pub attrs: GameModeAttributes,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
#[serde(untagged)]
pub enum GameModeAttributes {
    Osu {
        #[serde(rename = "approach_rate")]
        ar: f32,
        #[serde(rename = "overall_difficulty")]
        od: f32,
        aim_difficulty: f32,
        flashlight_difficulty: f32,
        slider_factor: f32,
        speed_difficulty: f32,
    },
    Taiko {
        stamina_difficulty: f32,
        rhythm_difficulty: f32,
        colour_difficulty: f32,
        peak_difficulty: f32,
        great_hit_window: f32,
    },
    Catch {
        #[serde(rename = "approach_rate")]
        ar: f32,
    },
    Mania {
        great_hit_window: f32,
        score_multiplier: f32,
    },
}

/// Represents a beatmapset. This extends [`BeatmapsetCompact`] with additional attributes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(bound(serialize = "__S: rkyv::ser::ScratchSpace + rkyv::ser::Serializer"))
)]
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
    #[cfg_attr(feature = "rkyv", omit_bounds)]
    pub converts: Option<Vec<Beatmap>>,
    pub covers: BeatmapsetCovers,
    /// Username of the mapper at the time of beatmapset creation
    #[serde(
        default,
        rename = "user",
        deserialize_with = "deser_mapset_user",
        skip_serializing_if = "Option::is_none"
    )]
    pub creator: Option<UserCompact>,
    #[serde(rename = "creator")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub creator_name: Username,
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
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub last_updated: OffsetDateTime,
    /// Full URL, i.e. `https://osu.ppy.sh/community/forums/topics/{thread_id}`
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legacy_thread_url: Option<String>,
    #[serde(default, rename = "beatmaps", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", omit_bounds)]
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub ranked_date: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recent_favourites: Option<Vec<UserCompact>>,
    pub source: String,
    pub status: RankStatus,
    pub storyboard: bool,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub submitted_date: Option<OffsetDateTime>,
    pub tags: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_unicode: Option<String>,
    pub video: bool,
}

// Deserialize the creator's `UserCompact` manually for edge cases
// like mapset /s/3 where the user was deleted
fn deser_mapset_user<'de, D: Deserializer<'de>>(d: D) -> Result<Option<UserCompact>, D::Error> {
    struct MapsetUserVisitor;

    impl<'de> Visitor<'de> for MapsetUserVisitor {
        type Value = Option<UserCompact>;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("an optional UserCompact")
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            struct DateSeed;

            impl<'de> DeserializeSeed<'de> for DateSeed {
                type Value = Option<OffsetDateTime>;

                #[inline]
                fn deserialize<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
                    serde_::option_datetime::deserialize(d)
                }
            }

            let mut avatar_url: Option<Option<String>> = None;
            let mut country_code: Option<Option<CountryCode>> = None;
            let mut default_group = None;
            let mut is_active = None;
            let mut is_bot = None;
            let mut is_deleted = None;
            let mut is_online = None;
            let mut is_supporter = None;
            let mut last_visit = None;
            let mut pm_friends_only = None;
            let mut profile_color = None;
            let mut user_id: Option<Option<u32>> = None;
            let mut username = None;

            while let Some(key) = map.next_key()? {
                match key {
                    "avatar_url" => avatar_url = Some(map.next_value()?),
                    "country_code" => country_code = Some(map.next_value()?),
                    "default_group" => default_group = Some(map.next_value()?),
                    "id" => user_id = Some(map.next_value()?),
                    "is_active" => is_active = Some(map.next_value()?),
                    "is_bot" => is_bot = Some(map.next_value()?),
                    "is_deleted" => is_deleted = Some(map.next_value()?),
                    "is_online" => is_online = Some(map.next_value()?),
                    "is_supporter" => is_supporter = Some(map.next_value()?),
                    "last_visit" => last_visit = Some(map.next_value_seed(DateSeed)?),
                    "pm_friends_only" => pm_friends_only = Some(map.next_value()?),
                    "profile_colour" => profile_color = Some(map.next_value()?),
                    "username" => username = Some(map.next_value()?),
                    _ => {
                        let _: IgnoredAny = map.next_value()?;
                    }
                }
            }

            let avatar_url = avatar_url
                .ok_or_else(|| Error::missing_field("avatar_url"))?
                .unwrap_or_default();

            let country_code = country_code
                .ok_or_else(|| Error::missing_field("country_code"))?
                .unwrap_or_else(|| "??".into());

            let default_group =
                default_group.ok_or_else(|| Error::missing_field("default_group"))?;

            let user_id = user_id
                .ok_or_else(|| Error::missing_field("user_id"))?
                .unwrap_or(0);

            let is_active = is_active.ok_or_else(|| Error::missing_field("is_active"))?;
            let is_bot = is_bot.ok_or_else(|| Error::missing_field("is_bot"))?;
            let is_deleted = is_deleted.ok_or_else(|| Error::missing_field("is_deleted"))?;
            let is_online = is_online.ok_or_else(|| Error::missing_field("is_online"))?;
            let is_supporter = is_supporter.ok_or_else(|| Error::missing_field("is_supporter"))?;
            let last_visit = last_visit.ok_or_else(|| Error::missing_field("last_visit"))?;
            let pm_friends_only =
                pm_friends_only.ok_or_else(|| Error::missing_field("pm_friends_only"))?;
            let profile_color =
                profile_color.ok_or_else(|| Error::missing_field("profile_color"))?;
            let username = username.ok_or_else(|| Error::missing_field("username"))?;

            Ok(Some(UserCompact {
                avatar_url,
                country_code,
                default_group,
                is_active,
                is_bot,
                is_deleted,
                is_online,
                is_supporter,
                last_visit,
                pm_friends_only,
                profile_color,
                user_id,
                username,
                account_history: None,
                badges: None,
                beatmap_playcounts_count: None,
                country: None,
                cover: None,
                favourite_mapset_count: None,
                follower_count: None,
                graveyard_mapset_count: None,
                groups: None,
                guest_mapset_count: None,
                highest_rank: None,
                is_admin: None,
                is_bng: None,
                is_full_bn: None,
                is_gmt: None,
                is_limited_bn: None,
                is_moderator: None,
                is_nat: None,
                is_silenced: None,
                loved_mapset_count: None,
                medals: None,
                monthly_playcounts: None,
                page: None,
                previous_usernames: None,
                rank_history: None,
                ranked_mapset_count: None,
                replays_watched_counts: None,
                scores_best_count: None,
                scores_first_count: None,
                scores_recent_count: None,
                statistics: None,
                support_level: None,
                pending_mapset_count: None,
            }))
        }

        #[inline]
        fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
            d.deserialize_map(self)
        }

        #[inline]
        fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }
    }

    d.deserialize_option(MapsetUserVisitor)
}

#[inline]
fn deserialize_f32_default<'de, D: Deserializer<'de>>(d: D) -> Result<f32, D::Error> {
    <Option<f32> as Deserialize>::deserialize(d).map(Option::unwrap_or_default)
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetAvailability {
    pub download_disabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_information: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetCommentEdit<T> {
    #[serde(flatten)]
    pub comment_id: BeatmapsetCommentId,
    pub old: T,
    pub new: T,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetCommentKudosuGain {
    #[serde(flatten)]
    pub comment_id: BeatmapsetCommentId,
    pub new_vote: BeatmapsetVote,
    pub votes: Vec<BeatmapsetVote>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetCommentNominate {
    pub modes: Vec<GameMode>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub new_username: Username,
}

/// Represents a beatmapset.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetCompact {
    pub artist: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    pub covers: BeatmapsetCovers,
    #[serde(rename = "creator")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::UsernameWrapper))]
    pub creator_name: Username,
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
    // TODO: Add ratings
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
    /// Tiny preview of the full background
    pub list: String,
    /// Small preview of the full background
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub deleted_at: Option<OffsetDateTime>,
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub last_post_at: OffsetDateTime,
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
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
#[serde(rename_all = "snake_case", tag = "type")]
#[non_exhaustive]
pub enum BeatmapsetEvent {
    Disqualify {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    GenreEdit {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<Genre>,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    IssueReopen {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    IssueResolve {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuDeny {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentId,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuGain {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentKudosuGain,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    KudosuLost {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentKudosuGain,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
        discussion: BeatmapsetDiscussion,
    },
    LanguageEdit {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<Language>,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Love {
        #[serde(rename = "id")]
        event_id: u64,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Nominate {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentNominate,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    NsfwToggle {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentEdit<bool>,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    #[serde(rename = "beatmap_owner_change")]
    OwnerChange {
        #[serde(rename = "id")]
        event_id: u64,
        comment: BeatmapsetCommentOwnerChange,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        user_id: u32,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Rank {
        #[serde(rename = "id")]
        event_id: u64,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
    Qualify {
        #[serde(rename = "id")]
        event_id: u64,
        #[serde(with = "serde_::datetime")]
        #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
        created_at: OffsetDateTime,
        #[serde(rename = "beatmapset")]
        mapset: BeatmapsetCompact,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetEvents {
    pub events: Vec<BeatmapsetEvent>,
    #[serde(rename = "reviewsConfig")]
    pub reviews_config: BeatmapsetReviewsConfig,
    pub users: Vec<UserCompact>,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct BeatmapsetHype {
    pub current: u32,
    pub required: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct BeatmapsetNominations {
    pub current: u32,
    pub required: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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
    #[serde(with = "serde_::datetime")]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeWrapper))]
    pub created_at: OffsetDateTime,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub updated_at: Option<OffsetDateTime>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "serde_::option_datetime"
    )]
    #[cfg_attr(feature = "rkyv", with(super::rkyv_impls::DateTimeMap))]
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct BeatmapsetReviewsConfig {
    pub max_blocks: u32,
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

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a rank status, \"any\", or `9`")
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
                &"a rank status or \"any\"",
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
                &"a RankStatus i8 or `9`",
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
                &"a RankStatus i8 or `9`",
            ))
        }
    }
}

impl<'de> Deserialize<'de> for SearchRankStatus {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_any(SearchRankStatusVisitor)
    }
}

impl Serialize for SearchRankStatus {
    #[inline]
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

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
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

                    params = Some(BeatmapsetSearchParameters {
                        sort,
                        descending,
                        ..Default::default()
                    });
                }
                "query" => query = map.next_value()?,
                "mode" => mode = Some(map.next_value()?),
                "status" => status = Some(map.next_value()?),
                "genre" => genre = Some(map.next_value()?),
                "language" => language = Some(map.next_value()?),
                "video" => video = Some(map.next_value()?),
                "storyboard" => storyboard = Some(map.next_value()?),
                "nsfw" => nsfw = Some(map.next_value()?),
                "_sort" => sort = Some(map.next_value()?),
                "descending" => descending = Some(map.next_value()?),
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
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(BeatmapsetSearchParametersVisitor)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
// TODO
// #[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct BeatmapsetSearchResult {
    cursor: Option<Cursor>,
    /// All mapsets of the current page
    #[serde(rename(serialize = "beatmapsets"))]
    pub mapsets: Vec<Beatmapset>,
    #[serde(rename(serialize = "search"))]
    pub(crate) params: BeatmapsetSearchParameters,
    /// Total amount of mapsets that fit the search query
    pub total: u32,
}

impl BeatmapsetSearchResult {
    /// Returns whether there is a next page of search results,
    /// retrievable via [`get_next`](BeatmapsetSearchResult::get_next).
    #[inline]
    pub fn has_more(&self) -> bool {
        self.cursor.is_some()
    }

    /// If [`has_more`](BeatmapsetSearchResult::has_more) is true, the API can provide
    /// the next set of search results and this method will request them.
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

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a BeatmapsetSearchResult struct")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut mapsets = None;
        let mut cursor = None;
        let mut params = None;
        let mut total = None;

        while let Some(key) = map.next_key()? {
            match key {
                "beatmapsets" => mapsets = Some(map.next_value()?),
                "cursor" => cursor = map.next_value()?,
                "search" => params = Some(map.next_value()?),
                "total" => total = Some(map.next_value()?),
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
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(BeatmapsetSearchResultVisitor)
    }
}

macro_rules! search_sort_enum {
    ($($variant:ident => $name:literal,)+) => {
        /// Provides an option to specify a mapset order in a mapset search,
        /// see [`Osu::beatmapset_search`](crate::client::Osu::beatmapset_search).
        #[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[cfg_attr(
            feature = "rkyv",
            derive(Archive, RkyvDeserialize, RkyvSerialize),
            archive(as = "Self"))
        ]
        pub enum BeatmapsetSearchSort {
            $(
                #[serde(rename = $name)]
                $variant,
            )+
        }

        impl Display for BeatmapsetSearchSort {
            #[inline]
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Self::$variant => f.write_str($name),)+
                }
            }
        }

        impl FromStr for BeatmapsetSearchSort {
            type Err = ();

            #[inline]
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
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub struct BeatmapsetVote {
    pub user_id: u32,
    pub score: u32,
}

/// All fields are optional but there's always at least one field returned.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
pub struct FailTimes {
    /// List of length 100
    #[serde(
        default,
        deserialize_with = "use_vec_option_visitor",
        skip_serializing_if = "Option::is_none"
    )]
    pub exit: Option<Vec<u32>>,
    /// List of length 100
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

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("null or a sequence of u32")
    }

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_seq(HundredU32Visitor::new()).map(Some)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        self.visit_unit()
    }

    #[inline]
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}

struct HundredU32Visitor(Vec<u32>);

impl HundredU32Visitor {
    fn new() -> Self {
        Self(Vec::with_capacity(100))
    }
}

impl<'de> Visitor<'de> for HundredU32Visitor {
    type Value = Vec<u32>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a sequence of u32")
    }

    #[inline]
    fn visit_seq<A: SeqAccess<'de>>(mut self, mut seq: A) -> Result<Self::Value, A::Error> {
        while let Some(n) = seq.next_element()? {
            self.0.push(n);
        }

        Ok(self.0)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "rkyv", derive(Archive, RkyvDeserialize, RkyvSerialize))]
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

#[allow(clippy::upper_case_acronyms, missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "rkyv",
    derive(Archive, RkyvDeserialize, RkyvSerialize),
    archive(as = "Self")
)]
pub enum RankStatus {
    Graveyard = -2,
    WIP = -1,
    Pending = 0,
    Ranked = 1,
    Approved = 2,
    Qualified = 3,
    Loved = 4,
}

impl<'de> serde::Deserialize<'de> for RankStatus {
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_option(super::EnumVisitor::<RankStatus>::new())
    }
}
impl From<RankStatus> for i8 {
    #[inline]
    fn from(v: RankStatus) -> Self {
        v as i8
    }
}

impl TryFrom<i8> for RankStatus {
    type Error = OsuError;

    #[inline]
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -2 => Ok(Self::Graveyard),
            -1 => Ok(Self::WIP),
            0 => Ok(Self::Pending),
            1 => Ok(Self::Ranked),
            2 => Ok(Self::Approved),
            3 => Ok(Self::Qualified),
            4 => Ok(Self::Loved),
            _ => Err(ParsingError::RankStatus(value).into()),
        }
    }
}

impl Serialize for RankStatus {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i8(*self as i8)
    }
}

impl<'de> Visitor<'de> for super::EnumVisitor<RankStatus> {
    type Value = RankStatus;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("an optional RankStatus i8")
    }

    fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
        match s {
            "graveyard" => Ok(RankStatus::Graveyard),
            "wip" => Ok(RankStatus::WIP),
            "pending" => Ok(RankStatus::Pending),
            "ranked" => Ok(RankStatus::Ranked),
            "approved" => Ok(RankStatus::Approved),
            "qualified" => Ok(RankStatus::Qualified),
            "loved" => Ok(RankStatus::Loved),
            _ => Err(Error::unknown_variant(
                s,
                &[
                    "graveyard",
                    "wip",
                    "pending",
                    "ranked",
                    "approved",
                    "qualified",
                    "loved",
                ],
            )),
        }
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        match v {
            -2 => Ok(RankStatus::Graveyard),
            -1 => Ok(RankStatus::WIP),
            0 => Ok(RankStatus::Pending),
            1 => Ok(RankStatus::Ranked),
            2 => Ok(RankStatus::Approved),
            3 => Ok(RankStatus::Qualified),
            4 => Ok(RankStatus::Loved),
            _ => Err(Error::invalid_value(
                Unexpected::Signed(v),
                &"-2, -1, 0, 1, 2, 3 or 4",
            )),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(RankStatus::Pending),
            1 => Ok(RankStatus::Ranked),
            2 => Ok(RankStatus::Approved),
            3 => Ok(RankStatus::Qualified),
            4 => Ok(RankStatus::Loved),
            _ => Err(Error::invalid_value(
                Unexpected::Unsigned(v),
                &"-2, -1, 0, 1, 2, 3 or 4",
            )),
        }
    }

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }
}

def_enum!(Genre {
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
    #[inline]
    fn default() -> Self {
        Self::Any
    }
}

def_enum!(Language {
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
    #[inline]
    fn default() -> Self {
        Self::Any
    }
}

struct DescriptionVisitor;

impl<'de> Visitor<'de> for DescriptionVisitor {
    type Value = Option<String>;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("a string or a map containing a 'description' field")
    }

    #[inline]
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Some(v.to_owned()))
    }

    #[inline]
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

    #[inline]
    fn visit_some<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        d.deserialize_any(self)
    }

    #[inline]
    fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
        self.visit_unit()
    }

    #[inline]
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
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
            cursor: None,
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
            cursor: None,
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
