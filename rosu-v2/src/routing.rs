use crate::{
    model::{ranking_::RankingType, GameMode},
    request::{ScoreType, UserId},
};

use hyper::Method;
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
        mode: GameMode,
        score_id: u64,
    },
    GetScore {
        mode: GameMode,
        score_id: u64,
    },
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
    #[allow(dead_code)]
    GetUsers,
    GetWikiPage {
        locale: Box<str>,
        page: Option<Box<str>>,
    },
}

impl Route {
    /// Separate a route into its parts: the HTTP method and the URI path.
    pub(crate) fn to_parts(&self) -> (Method, Cow<'static, str>) {
        match self {
            Self::GetBeatmap => (Method::GET, "beatmaps/lookup".into()),
            Self::GetBeatmaps => (Method::GET, "beatmaps".into()),
            Self::PostBeatmapDifficultyAttributes { map_id } => {
                (Method::POST, format!("beatmaps/{map_id}/attributes").into())
            }
            Self::GetBeatmapScores { map_id } => {
                (Method::GET, format!("beatmaps/{map_id}/scores").into())
            }
            Self::GetBeatmapUserScore { map_id, user_id } => (
                Method::GET,
                format!("beatmaps/{map_id}/scores/users/{user_id}").into(),
            ),
            Self::GetBeatmapUserScores { map_id, user_id } => (
                Method::GET,
                format!("beatmaps/{map_id}/scores/users/{user_id}/all").into(),
            ),
            Self::GetBeatmapset { mapset_id } => {
                (Method::GET, format!("beatmapsets/{mapset_id}").into())
            }
            Self::GetBeatmapsetFromMapId => (Method::GET, "beatmapsets/lookup".into()),
            Self::GetBeatmapsetEvents => (Method::GET, "beatmapsets/events".into()),
            Self::GetBeatmapsetSearch => (Method::GET, "beatmapsets/search".into()),
            Self::GetComments => (Method::GET, "comments".into()),
            Self::GetEvents => (Method::GET, "events".into()),
            Self::GetForumPosts { topic_id } => {
                (Method::GET, format!("forums/topics/{topic_id}").into())
            }
            Self::GetMatch { match_id } => {
                let path = match match_id {
                    Some(id) => format!("matches/{id}").into(),
                    None => "matches".into(),
                };

                (Method::GET, path)
            }
            Self::GetNews { news } => {
                let path = match news {
                    Some(_news) => unimplemented!(),
                    None => "news".into(),
                };

                (Method::GET, path)
            }
            Self::GetOwnData { mode } => {
                let path = match mode {
                    Some(mode) => format!("me/{mode}").into(),
                    None => "me".into(),
                };

                (Method::GET, path)
            }
            Self::GetRankings { mode, ranking_type } => (
                Method::GET,
                format!("rankings/{mode}/{}", ranking_type.as_str()).into(),
            ),
            Self::GetRecentActivity { user_id } => (
                Method::GET,
                format!("users/{user_id}/recent_activity").into(),
            ),
            Self::GetReplay { mode, score_id } => (
                Method::GET,
                format!("scores/{mode}/{score_id}/download").into(),
            ),
            Self::GetScore { mode, score_id } => {
                (Method::GET, format!("scores/{mode}/{score_id}").into())
            }
            Self::GetSeasonalBackgrounds => (Method::GET, "seasonal-backgrounds".into()),
            Self::GetSpotlights => (Method::GET, "spotlights".into()),
            Self::GetUser { user_id, mode } => {
                let mut path = format!("users/{user_id}");

                if let Some(mode) = mode {
                    let _ = write!(path, "/{mode}");
                }

                (Method::GET, path.into())
            }
            Self::GetUserBeatmapsets { user_id, map_type } => (
                Method::GET,
                format!("users/{user_id}/beatmapsets/{map_type}").into(),
            ),
            Self::GetUserKudosu { user_id } => {
                (Method::GET, format!("users/{user_id}/kudosu").into())
            }
            Self::GetUserScores {
                user_id,
                score_type,
            } => (
                Method::GET,
                format!("users/{user_id}/scores/{}", score_type.as_str()).into(),
            ),
            Self::GetUsers => (Method::GET, "users".into()),
            Self::GetWikiPage { locale, page } => {
                let mut path = format!("wiki/{locale}/");

                if let Some(page) = page {
                    path.push_str(page);
                }

                (Method::GET, path.into())
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
            },
            Self::GetRecentActivity { .. } => "GetRecentActivity",
            Self::GetReplay { .. } => "GetReplay",
            Self::GetScore { .. } => "GetScore",
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
