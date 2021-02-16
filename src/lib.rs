mod client;
pub mod error;
pub mod model;
mod ratelimiter;
pub mod request;
mod routing;
mod serde;

pub use client::{Osu, OsuBuilder};
pub use error::{OsuError, OsuResult};

#[macro_use]
extern crate log;
