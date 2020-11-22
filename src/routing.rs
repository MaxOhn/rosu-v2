use crate::model::GameMode;

use reqwest::Method;
use std::{borrow::Cow, fmt::Write};

#[derive(Debug)]
#[non_exhaustive]
pub(crate) enum Route {
    // GetBeatmap { map_id: u32 },
    GetUser {
        user_id: u32,
        mode: Option<GameMode>,
    },
    GetUserKudosu {
        user_id: u32,
    },
    GetUsers,
}

impl Route {
    /// Separate a route into its parts: the HTTP method and the URI path.
    pub(crate) fn into_parts(self) -> (Method, Cow<'static, str>) {
        match self {
            // Self::GetBeatmap { map_id } => (Method::GET, format!("beatmaps/{}", map_id).into()),
            Self::GetUser { user_id, mode } => {
                let mut uri = format!("users/{}", user_id);

                if let Some(mode) = mode {
                    let _ = write!(uri, "/{}", mode);
                }

                (Method::GET, uri.into())
            }
            Self::GetUserKudosu { user_id } => {
                (Method::GET, format!("users/{}/kudosu", user_id).into())
            }
            Self::GetUsers => (Method::GET, "users".into()),
        }
    }
}
