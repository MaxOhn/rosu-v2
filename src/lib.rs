mod client;
pub mod error;
pub mod model;
mod ratelimiter;
pub mod request;
mod routing;

pub use client::{Osu, OsuBuilder};

#[macro_use]
extern crate log;
