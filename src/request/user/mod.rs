mod beatmap;
mod events;
mod kudosu;
mod scores;
mod user;
mod users;

pub use beatmap::GetUserBeatmapsets;
pub use events::GetRecentEvents;
pub use kudosu::GetUserKudosu;
pub use scores::GetUserScores;
pub use user::GetUser;
pub use users::GetUsers;

use std::fmt;

#[derive(Debug)]
pub enum UserId {
    Id(u32),
    Name(String),
}

impl From<u32> for UserId {
    #[inline]
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}

impl From<&str> for UserId {
    #[inline]
    fn from(name: &str) -> Self {
        Self::Name(name.to_owned())
    }
}

impl From<&String> for UserId {
    #[inline]
    fn from(name: &String) -> Self {
        Self::Name(name.to_owned())
    }
}

impl From<String> for UserId {
    #[inline]
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Id(id) => write!(f, "{}", id),
            Self::Name(name) => f.write_str(name),
        }
    }
}
