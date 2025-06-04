use crate::{
    model::{ranking::RankingType, GameMode},
    request::{Method, ScoreType, UserId},
};

use std::{borrow::Cow, fmt::Write};

#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub(crate) enum Route {
    GetBeatmap,
    GetBeatmaps,
    PostBeatmapDifficultyAttributes {
        map_id: u32,
    },
    GetBeatmapScores {
        map_id: u32,
    },
    GetBeatmapUserScore {
        user_id: u32,
        map_id: u32,
    },
    GetBeatmapUserScores {
        user_id: u32,
        map_id: u32,
    },
    GetBeatmapset {
        mapset_id: u32,
    },
    GetBeatmapsetFromMapId,
    GetBeatmapsetEvents,
    GetBeatmapsetSearch,
    GetComments,
    GetEvents,
    GetForumPosts {
        topic_id: u64,
    },
    GetFriends,
    GetMatch {
        match_id: Option<u32>,
    },
    GetNews {
        news: Option<()>,
    },
    GetOwnData {
        mode: Option<GameMode>,
    },
    GetRankings {
        mode: GameMode,
        ranking_type: RankingType,
    },
    GetRecentActivity {
        user_id: u32,
    },
    GetReplay {
        mode: Option<GameMode>,
        score_id: u64,
    },
    GetRoom {
        room_id: u64,
    },
    GetRoomEvents {
        room_id: u64,
    },
    GetRoomLeaderboard {
        room_id: u64,
    },
    GetRooms,
    GetScore {
        mode: Option<GameMode>,
        score_id: u64,
    },
    GetScores,
    GetSeasonalBackgrounds,
    GetSpotlights,
    GetUser {
        user_id: UserId,
        mode: Option<GameMode>,
    },
    GetUserBeatmapsets {
        user_id: u32,
        map_type: &'static str,
    },
    GetUserKudosu {
        user_id: u32,
    },
    GetUserScores {
        user_id: u32,
        score_type: ScoreType,
    },
    GetUsers,
    GetWikiPage {
        locale: Box<str>,
        page: Option<Box<str>>,
    },
}

impl Route {
    /// Separate a route into its parts: the HTTP method and the URI path.
    #[allow(clippy::too_many_lines)]
    pub(crate) fn as_parts(&self) -> (Method, Cow<'static, str>) {
        match self {
            Self::GetBeatmap => (Method::Get, "beatmaps/lookup".into()),
            Self::GetBeatmaps => (Method::Get, "beatmaps".into()),
            Self::PostBeatmapDifficultyAttributes { map_id } => {
                (Method::Post, format!("beatmaps/{map_id}/attributes").into())
            }
            Self::GetBeatmapScores { map_id } => {
                (Method::Get, format!("beatmaps/{map_id}/scores").into())
            }
            Self::GetBeatmapUserScore { map_id, user_id } => (
                Method::Get,
                format!("beatmaps/{map_id}/scores/users/{user_id}").into(),
            ),
            Self::GetBeatmapUserScores { map_id, user_id } => (
                Method::Get,
                format!("beatmaps/{map_id}/scores/users/{user_id}/all").into(),
            ),
            Self::GetBeatmapset { mapset_id } => {
                (Method::Get, format!("beatmapsets/{mapset_id}").into())
            }
            Self::GetBeatmapsetFromMapId => (Method::Get, "beatmapsets/lookup".into()),
            Self::GetBeatmapsetEvents => (Method::Get, "beatmapsets/events".into()),
            Self::GetBeatmapsetSearch => (Method::Get, "beatmapsets/search".into()),
            Self::GetComments => (Method::Get, "comments".into()),
            Self::GetEvents => (Method::Get, "events".into()),
            Self::GetForumPosts { topic_id } => {
                (Method::Get, format!("forums/topics/{topic_id}").into())
            }
            Self::GetFriends => (Method::Get, "friends".into()),
            Self::GetMatch { match_id } => {
                let path = match match_id {
                    Some(id) => format!("matches/{id}").into(),
                    None => "matches".into(),
                };

                (Method::Get, path)
            }
            Self::GetNews { news } => {
                let path = match news {
                    Some(_news) => unimplemented!(),
                    None => "news".into(),
                };

                (Method::Get, path)
            }
            Self::GetOwnData { mode } => {
                let path = match mode {
                    Some(mode) => format!("me/{mode}").into(),
                    None => "me".into(),
                };

                (Method::Get, path)
            }
            Self::GetRankings { mode, ranking_type } => (
                Method::Get,
                format!("rankings/{mode}/{}", ranking_type.as_str()).into(),
            ),
            Self::GetRecentActivity { user_id } => (
                Method::Get,
                format!("users/{user_id}/recent_activity").into(),
            ),
            Self::GetReplay { mode, score_id } => {
                let path = match mode {
                    Some(mode) => format!("scores/{mode}/{score_id}/download").into(),
                    None => format!("scores/{score_id}/download").into(),
                };
                (Method::Get, path)
            }
            Self::GetRoom { room_id } => (Method::Get, format!("rooms/{room_id}").into()),
            Self::GetRoomEvents { room_id } => {
                (Method::Get, format!("rooms/{room_id}/events").into())
            }
            Self::GetRoomLeaderboard { room_id } => {
                (Method::Get, format!("rooms/{room_id}/leaderboard").into())
            }
            Self::GetRooms => (Method::Get, "rooms".into()),
            Self::GetScore { mode, score_id } => {
                let path = match mode {
                    Some(mode) => format!("scores/{mode}/{score_id}").into(),
                    None => format!("scores/{score_id}").into(),
                };
                (Method::Get, path)
            }
            Self::GetScores => (Method::Get, "scores".into()),
            Self::GetSeasonalBackgrounds => (Method::Get, "seasonal-backgrounds".into()),
            Self::GetSpotlights => (Method::Get, "spotlights".into()),
            Self::GetUser { user_id, mode } => {
                let mut path = format!("users/{user_id}");

                if let Some(mode) = mode {
                    let _ = write!(path, "/{mode}");
                }

                (Method::Get, path.into())
            }
            Self::GetUserBeatmapsets { user_id, map_type } => (
                Method::Get,
                format!("users/{user_id}/beatmapsets/{map_type}").into(),
            ),
            Self::GetUserKudosu { user_id } => {
                (Method::Get, format!("users/{user_id}/kudosu").into())
            }
            Self::GetUserScores {
                user_id,
                score_type,
            } => (
                Method::Get,
                format!("users/{user_id}/scores/{}", score_type.as_str()).into(),
            ),
            Self::GetUsers => (Method::Get, "users".into()),
            Self::GetWikiPage { locale, page } => {
                let mut path = format!("wiki/{locale}/");

                if let Some(page) = page {
                    path.push_str(page);
                }

                (Method::Get, path.into())
            }
        }
    }

    #[cfg(feature = "metrics")]
    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::GetBeatmap => "GetBeatmap",
            Self::GetBeatmaps => "GetBeatmaps",
            Self::PostBeatmapDifficultyAttributes { .. } => "PostBeatmapDifficultyAttributes",
            Self::GetBeatmapScores { .. } => "GetBeatmapScores",
            Self::GetBeatmapUserScore { .. } => "GetBeatmapUserScore",
            Self::GetBeatmapUserScores { .. } => "GetBeatmapUserScores",
            Self::GetBeatmapset { .. } => "GetBeatmapset",
            Self::GetBeatmapsetFromMapId => "GetBeatmapsetFromMapId",
            Self::GetBeatmapsetEvents => "GetBeatmapsetEvents",
            Self::GetBeatmapsetSearch => "GetBeatmapsetSearch",
            Self::GetComments => "GetComments",
            Self::GetEvents => "GetEvents",
            Self::GetForumPosts { .. } => "GetForumPosts",
            Self::GetFriends => "GetFriends",
            Self::GetMatch { match_id } => match match_id {
                Some(_) => "GetMatch/match_id",
                None => "GetMatch",
            },
            Self::GetNews { .. } => "GetNews",
            Self::GetOwnData { .. } => "GetOwnData",
            Self::GetRankings { ranking_type, .. } => match ranking_type {
                RankingType::Charts => "GetRankings/Charts",
                RankingType::Country => "GetRankings/Country",
                RankingType::Performance => "GetRankings/Performance",
                RankingType::Score => "GetRankings/Score",
                RankingType::Team => "GetRankings/Team",
            },
            Self::GetRecentActivity { .. } => "GetRecentActivity",
            Self::GetReplay { .. } => "GetReplay",
            Self::GetRoom { .. } => "GetRoom",
            Self::GetRoomEvents { .. } => "GetRoomEvents",
            Self::GetRoomLeaderboard { .. } => "GetRoomLeaderboard",
            Self::GetRooms => "GetRooms",
            Self::GetScore { .. } => "GetScore",
            Self::GetScores => "GetScores",
            Self::GetSeasonalBackgrounds => "GetSeasonalBackgrounds",
            Self::GetSpotlights => "GetSpotlights",
            Self::GetUser { .. } => "GetUser",
            Self::GetUserBeatmapsets { .. } => "GetUserBeatmapsets",
            Self::GetUserKudosu { .. } => "GetUserKudosu",
            Self::GetUserScores { score_type, .. } => match score_type {
                ScoreType::Best => "GetUserScores/Best",
                ScoreType::First => "GetUserScores/First",
                ScoreType::Pinned => "GetUserScores/Pinned",
                ScoreType::Recent => "GetUserScores/Recent",
            },
            Self::GetUsers => "GetUsers",
            Self::GetWikiPage { .. } => "GetWikiPage",
        }
    }
}
