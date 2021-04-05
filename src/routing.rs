use crate::{
    model::{ranking::RankingType, GameMode},
    request::{ScoreType, UserId},
};

use reqwest::Method;
use std::{borrow::Cow, fmt::Write};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum Route {
    GetBeatmap,
    GetBeatmapScores {
        map_id: u32,
    },
    GetBeatmapUserScore {
        user_id: u32,
        map_id: u32,
    },
    GetBeatmapset {
        mapset_id: u32,
    },
    GetBeatmapsetEvents,
    GetBeatmapsetSearch,
    GetComments,
    GetForumPosts {
        topic_id: u64,
    },
    GetMatch {
        match_id: Option<u32>,
    },
    GetMultiplayerScore {
        room: u32,
        playlist: u32,
        score_id: u32,
    },
    GetMultiplayerScores {
        room: u32,
        playlist: u32,
    },
    GetMultiplayerUserHighScore {
        room: u32,
        playlist: u32,
        user_id: u32,
    },
    GetNews {
        news: Option<()>,
    },
    GetRankings {
        mode: GameMode,
        ranking_type: RankingType,
    },
    GetRecentEvents {
        user_id: u32,
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
    GetUsers,
    GetWikiPage {
        locale: String,
        page: Option<String>,
    },
}

impl Route {
    /// Separate a route into its parts: the HTTP method and the URI path.
    pub(crate) fn into_parts(self) -> (Method, Cow<'static, str>) {
        match self {
            Self::GetBeatmap => (Method::GET, "beatmaps/lookup".into()),
            Self::GetBeatmapScores { map_id } => {
                (Method::GET, format!("beatmaps/{}/scores", map_id).into())
            }
            Self::GetBeatmapUserScore { map_id, user_id } => (
                Method::GET,
                format!("beatmaps/{}/scores/users/{}", map_id, user_id).into(),
            ),
            Self::GetBeatmapset { mapset_id } => {
                (Method::GET, format!("beatmapsets/{}", mapset_id).into())
            }
            Self::GetBeatmapsetEvents => (Method::GET, "beatmapsets/events".into()),
            Self::GetBeatmapsetSearch => (Method::GET, "beatmapsets/search".into()),
            Self::GetComments => (Method::GET, "comments".into()),
            Self::GetForumPosts { topic_id } => {
                (Method::GET, format!("forums/topics/{}", topic_id).into())
            }
            Self::GetMatch { match_id } => {
                let path = match match_id {
                    Some(id) => format!("matches/{}", id).into(),
                    None => "matches".into(),
                };

                (Method::GET, path)
            }
            Self::GetMultiplayerScore {
                room,
                playlist,
                score_id,
            } => (
                Method::GET,
                format!("rooms/{}/playlist/{}/scores/{}", room, playlist, score_id).into(),
            ),
            Self::GetMultiplayerScores { room, playlist } => (
                Method::GET,
                format!("rooms/{}/playlist/{}/scores", room, playlist).into(),
            ),
            Self::GetMultiplayerUserHighScore {
                room,
                playlist,
                user_id,
            } => {
                let path = format!(
                    "rooms/{}/playlist/{}/scores/users/{}",
                    room, playlist, user_id
                );

                (Method::GET, path.into())
            }
            Self::GetNews { news } => {
                let path = match news {
                    Some(_news) => unimplemented!(),
                    None => "news".into(),
                };

                (Method::GET, path)
            }
            Self::GetRankings { mode, ranking_type } => (
                Method::GET,
                format!("rankings/{}/{}", mode, ranking_type).into(),
            ),
            Self::GetRecentEvents { user_id } => (
                Method::GET,
                format!("users/{}/recent_activity", user_id).into(),
            ),
            Self::GetSeasonalBackgrounds => (Method::GET, "seasonal-backgrounds".into()),
            Self::GetSpotlights => (Method::GET, "spotlights".into()),
            Self::GetUser { user_id, mode } => {
                let mut path = format!("users/{}", user_id);

                if let Some(mode) = mode {
                    let _ = write!(path, "/{}", mode);
                }

                (Method::GET, path.into())
            }
            Self::GetUserBeatmapsets { user_id, map_type } => (
                Method::GET,
                format!("users/{}/beatmapsets/{}", user_id, map_type).into(),
            ),
            Self::GetUserKudosu { user_id } => {
                (Method::GET, format!("users/{}/kudosu", user_id).into())
            }
            Self::GetUserScores {
                user_id,
                score_type,
            } => (
                Method::GET,
                format!("users/{}/scores/{}", user_id, score_type).into(),
            ),
            Self::GetUsers => (Method::GET, "users".into()),
            Self::GetWikiPage { locale, page } => {
                let mut path = format!("wiki/{}/", locale);

                if let Some(page) = page {
                    path.push_str(&page);
                }

                (Method::GET, path.into())
            }
        }
    }
}
