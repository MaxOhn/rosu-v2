mod client;
pub mod error;
pub mod model;
mod ratelimiter;
pub mod request;
mod routing;

pub use client::{Osu, OsuBuilder};

#[macro_use]
extern crate log;

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, error::OsuError>;

pub mod prelude {
    pub use crate::{
        error::OsuError,
        model::{
            beatmap::*, comments::*, forum::*, kudosu::*, matches::*, multiplayer::*, news::*,
            ranking::*, recent_event::*, score::*, user::*, wiki::*, GameMode, GameMods, Grade,
        },
        Osu, OsuBuilder, OsuResult,
    };

    pub use reqwest::{Client as ReqwestClient, StatusCode};
}
