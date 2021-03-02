mod client;
mod error;
pub mod model;
mod ratelimiter;
pub mod request;
mod routing;

pub use client::{Osu, OsuBuilder};
pub use error::{OsuError, OsuResult};

#[macro_use]
extern crate log;
