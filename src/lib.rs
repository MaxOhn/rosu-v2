//! [![crates.io](https://img.shields.io/crates/v/rosu-v2.svg)](https://crates.io/crates/rosu-v2) [![docs](https://docs.rs/rosu-v2/badge.svg)](https://docs.rs/rosu-v2)
//!
//! # rosu-v2
//!
//! rosu-v2 is a wrapper for the [osu!api v2](https://osu.ppy.sh/docs/index.html).
//! As such, it provides a bunch of additional endpoints and data over [rosu](https://github.com/MaxOhn/rosu) which wraps the [osu!api v1](https://github.com/ppy/osu-api/wiki).
//!
//! However, osu!'s api v2 is still very much a WIP and also weakly documented. Hence, there is a chance that some things might break either because of changes in the api or because the response is not being parsed properly.
//!
//! Feel free to open an issue when things don't work as expected.
//!
//! ## Authentication
//!
//! Unlike api v1, api v2 does not require an api key by users. Instead, it requires a client id and a client secret.
//!
//! To get those, you must register an application [here](https://osu.ppy.sh/home/account/edit#new-oauth-application).
//! Since rosu-v2 only supports client credentials and not authorization code, the callback URL here does not matter.
//!
//! If you went through the OAuth process for a user, you can also provide the redirect url and received code
//! when creating the client in order to make requests on behalf of the authenticated user.
//!
//! ## Endpoints
//!
//! The following endpoints are currently supported:
//!
//! - `beatmaps/lookup`: A specific beatmap including its beatmapset
//! - `beatmaps`: Up to 50 beatmaps at once including their beatmapsets.
//! - `beatmaps/{map_id}/scores`: The global score leaderboard for a beatmap
//! - `beatmaps/{map_id}/scores/users/{user_id}`: The single top score of a user on a beatmap. Defaults to the play with the __max score__, not pp
//! - `beatmapsets/{mapset_id}`: The beatmapset including all of its difficulty beatmaps
//! - `beatmapsets/events`: Various events around a beatmapset such as status, genre, or language updates, kudosu transfers, or new issues
//! - `beatmapsets/search`: Search for beatmapsets; the same search as on the osu! website
//! - `comments`: Most recent comments and their replies up to two levels deep
//! - `forums/topics/{topic_id}`: A forum topic and its posts
//! - `matches`: List of currently open multiplayer lobbies
//! - `matches/{match_id}`: More specific data about a specific multiplayer lobby including participating players and occured events
//! - `me[/{mode}]`: Detailed info about the authenticated user [in the specified mode] (requires OAuth)
//! - `news`: Recent news
//! - `rankings/{mode}/{ranking_type}`: The global leaderboard of either performance points, ranked score, countries, or a spotlight
//! - `users/{user_id}/{recent_activity}`: List of a user's recent events like achieved medals, ranks on a beatmaps, username changes, supporter status updates, beatmapset status updates, ...
//! - `scores/{mode}/{score_id}`: A specific score including its beatmap, beatmapset, and user
//! - `seasonal-backgrounds`: List of seasonal backgrounds i.e. their URL and artists
//! - `spotlights`: List of overviews of all spotlights
//! - `users/{user_id}[/{mode}]`: Detailed info about a user [in the specified mode]
//! - `users/{user_id}/{beatmapsets/{map_type}`: List of beatmapsets either created, favourited, or most played by the user
//! - `users/{user_id}/kudosu`: A user's recent kudosu transfers
//! - `users/{user_id}/scores/{score_type}`: Either top, recent, pinned, or global #1 scores of a user
//! - `wiki/{locale}[/{path}]`: The general wiki page or a specific topic if the path is specified
//!
//! The api itself provides a bunch more endpoints which are not yet implemented because they're really niche and/or missing any documentation.
//!
//! If you find an endpoint on the [api page](https://osu.ppy.sh/docs/index.html) that you want to use but is missing in rosu-v2, feel free to open an issue.
//!
//! ## Usage
//!
//! ```no_run
//! // For convenience sake, all types can be found in the prelude module
//! use rosu_v2::prelude::*;
//!
//! # fn main() {
//! # /*
//! #[tokio::main]
//! async fn main() {
//! # */
//!     # let _ = async {
//!     // Initialize the client
//!     let client_id: u64 = 0;
//!     let client_secret: String = String::from("");
//!     let osu: Osu = match Osu::new(client_id, client_secret).await {
//!         Ok(client) => client,
//!         Err(why) => panic!(
//!             "Failed to create client or make initial osu!api interaction: {}",
//!             why
//!         ),
//!     };
//!
//!     // Get peppy's top 10-15 scores in osu!standard.
//!     // Note that the username here can only be used because of the `cache` feature.
//!     // If you are fine with just providing user ids, consider disabling this feature.
//!     let scores: Vec<Score> = osu.user_scores("peppy")
//!         .mode(GameMode::STD)
//!         .best() // top scores; alternatively .recent() or .firsts()
//!         .offset(10)
//!         .limit(5)
//!         .await
//!         .unwrap_or_else(|why| panic!("Failed to get scores: {}", why));
//!
//!     // Search non-nsfw loved mania maps matching the given query.
//!     // Note that the order of called methods doesn't matter for any endpoint.
//!     let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
//!         .nsfw(false)
//!         .status(RankStatus::Loved)
//!         .mode(GameMode::MNA)
//!         .query("blue army stars>3")
//!         .await
//!         .unwrap_or_else(|why| panic!("Failed to search mapsets: {}", why));
//!
//!     // Get the german wiki page on hit objects
//!     let wiki_page: WikiPage = osu.wiki("de")
//!         .page("Hit_object")
//!         .await
//!         .unwrap_or_else(|why| panic!("Failed to get wiki page: {}", why));
//!     # };
//! }
//! ```
//!
//! ## Features
//!
//! | Flag | Description | deps
//! |-----|-----|-----|
//! | `default` | Enable the `cache` feature |
//! | `cache` | Cache username-user_id pairs so that usernames can be used on all user endpoints instead of only user ids | [dashmap](https://github.com/xacrimon/dashmap)
//! | `metrics` | Provide a count of all request types the client makes with the function `Osu::metrics` returning a `prometheus::IntCounterVec` | [prometheus](https://github.com/tikv/rust-prometheus)

// #![deny(missing_docs)] // TODO
#![deny(clippy::all, nonstandard_style, rust_2018_idioms, unused, warnings)]

mod client;
mod ratelimiter;
mod routing;

/// rosu-specific errors
pub mod error;
/// All available data types provided by the api
pub mod model;
/// Requesting-structs that implement [`Future`](std::future::Future) for each endpoint
pub mod request;

#[cfg(feature = "metrics")]
mod metrics;

pub use client::{Osu, OsuBuilder};

#[macro_use]
extern crate log;

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, error::OsuError>;

/// All types except requesting, stuffed into one module
pub mod prelude {
    pub use crate::{
        client::Scope,
        error::OsuError,
        model::{
            beatmap::*, comments::*, forum::*, kudosu::*, matches::*, news::*, ranking::*,
            recent_event::*, score::*, seasonal_backgrounds::*, user::*, wiki::*, Cursor, GameMode,
            GameMods, Grade,
        },
        request::UserId,
        Osu, OsuBuilder, OsuResult,
    };

    pub use hyper::StatusCode;
    pub use smallstr::SmallString;

    #[cfg(feature = "metrics")]
    pub use prometheus::IntCounterVec;
}
