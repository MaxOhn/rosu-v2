use crate::{model::GameMode, request::UserId};

use reqwest::Method;
use std::{borrow::Cow, fmt::Write};

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum Route {
    GetBeatmap {
        map_id: u32,
    },
    GetBeatmapScores {
        map_id: u32,
    },
    GetBeatmapUserScore {
        map_id: u32,
        user_id: UserId,
    },
    GetComments,
    #[allow(dead_code)]
    GetNews {
        news_id: Option<u32>,
    },
    GetRankings {
        mode: GameMode,
        ranking_type: &'static str,
    },
    GetRecentEvents {
        user_id: UserId,
    },
    GetScore {
        room: u32,
        playlist: u32,
        score_id: u32,
    },
    GetScores {
        room: u32,
        playlist: u32,
    },
    GetSeasonalBackgrounds,
    GetSpotlights,
    GetUser {
        user_id: UserId,
        mode: Option<GameMode>,
    },
    GetUserBeatmapsets {
        user_id: UserId,
        map_type: &'static str,
    },
    GetUserHighScore {
        room: u32,
        playlist: u32,
        user_id: u32,
    },
    GetUserKudosu {
        user_id: UserId,
    },
    GetUserScores {
        user_id: UserId,
        score_type: &'static str,
    },
    GetUsers,
    GetWikiPage {
        page: Option<String>,
    },
}

impl Route {
    /// Separate a route into its parts: the HTTP method and the URI path.
    pub(crate) fn into_parts(self) -> (Method, Cow<'static, str>) {
        match self {
            Self::GetBeatmap { map_id } => (Method::GET, format!("beatmaps/{}", map_id).into()),
            Self::GetBeatmapScores { map_id } => {
                (Method::GET, format!("beatmaps/{}/scores", map_id).into())
            }
            Self::GetBeatmapUserScore { map_id, user_id } => (
                Method::GET,
                format!("beatmaps/{}/scores/users/{}", map_id, user_id).into(),
            ),
            Self::GetComments => (Method::GET, "comments".into()),
            Self::GetNews { news_id } => {
                let uri = match news_id {
                    Some(id) => format!("news/{}", id).into(),
                    None => "news/".into(),
                };

                (Method::GET, uri)
            }
            Self::GetRankings { mode, ranking_type } => (
                Method::GET,
                format!("rankings/{}/{}", mode, ranking_type).into(),
            ),
            Self::GetRecentEvents { user_id } => (
                Method::GET,
                format!("users/{}/recent_activity", user_id).into(),
            ),
            Self::GetScore {
                room,
                playlist,
                score_id,
            } => (
                Method::GET,
                format!("rooms/{}/playlist/{}/scores/{}", room, playlist, score_id).into(),
            ),
            Self::GetScores { room, playlist } => (
                Method::GET,
                format!("rooms/{}/playlist/{}/scores", room, playlist).into(),
            ),
            Self::GetSeasonalBackgrounds => (Method::GET, "seasonal-backgrounds".into()),
            Self::GetSpotlights => (Method::GET, "spotlights".into()),
            Self::GetUser { user_id, mode } => {
                let mut uri = format!("users/{}", user_id);

                if let Some(mode) = mode {
                    let _ = write!(uri, "/{}", mode);
                }

                (Method::GET, uri.into())
            }
            Self::GetUserBeatmapsets { user_id, map_type } => (
                Method::GET,
                format!("users/{}/beatmapsets/{}", user_id, map_type).into(),
            ),
            Self::GetUserHighScore {
                room,
                playlist,
                user_id,
            } => {
                let uri = format!(
                    "rooms/{}/playlist/{}/scores/users/{}",
                    room, playlist, user_id
                );

                (Method::GET, uri.into())
            }
            Self::GetUserKudosu { user_id } => {
                (Method::GET, format!("users/{}/kudosu", user_id).into())
            }
            Self::GetUserScores {
                user_id,
                score_type,
            } => (
                Method::GET,
                format!("users/{}/scores{}", user_id, score_type).into(),
            ),
            Self::GetUsers => (Method::GET, "users".into()),
            Self::GetWikiPage { page } => {
                let uri = match page {
                    Some(page) => format!("wiki/{}", page).into(),
                    None => "wiki".into(),
                };

                (Method::GET, uri)
            }
        }
    }
}
