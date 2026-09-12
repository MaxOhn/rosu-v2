//! rosu-v2 is a wrapper for the [osu!api v2].
//! As such, it provides a bunch of additional endpoints and data over [`rosu`] which wraps the [osu!api v1].
//!
//! Feel free to open an issue when things don't work as expected.
//!
//! The branch `rosu-v2/main` should mirror the last published version. Upcoming changes
//! will generally be added to the `rosu-v2/lazer` branch. If you want to stay up-to-date
//! and use the `lazer` branch, you can add this in your `Cargo.toml`:
//!
//! ```toml
//! rosu-v2 = { git = "https://github.com/MaxOhn/rosu-v2", branch = "lazer" }
//! ```
//!
//! ## Authentication
//!
//! Unlike api v1, api v2 does not require an api key by users. Instead, it requires a client id and a client secret.
//!
//! To get those, you must register an application [here](https://osu.ppy.sh/home/account/edit#new-oauth-application).
//! Unless you're interested in logging into the API through an osu! account, the callback URL here does not matter and can be left blank.
//!
//! If you went through the OAuth process for a user, you can provide the callback URL and received code
//! when creating the client in order to make requests on behalf of the authenticated user.
//!
//! ## Endpoints
//!
//! rosu-v2 wraps most of the api's endpoints - but not all.
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
//!     let client_id: u64 = 123;
//!     let client_secret = String::from("my_secret");
//!     let osu = Osu::new(client_id, client_secret).await.unwrap();
//!
//!     // Get peppy's top 10-15 scores in osu!standard.
//!     // Note that the username here can only be used because of the `cache` feature.
//!     // If you are fine with just providing user ids, consider disabling this feature.
//!     let scores: Vec<Score> = osu.user_scores("peppy")
//!         .mode(GameMode::Osu)
//!         .best() // top scores; alternatively .recent(), .pinned(), or .firsts()
//!         .offset(10)
//!         .limit(5)
//!         .await
//!         .unwrap();
//!
//!     // Search non-nsfw loved mania maps matching the given query.
//!     // Note that the order of called methods doesn't matter for any endpoint.
//!     let search_result: BeatmapsetSearchResult = osu.beatmapset_search()
//!         .nsfw(false)
//!         .status(Some(RankStatus::Loved))
//!         .mode(GameMode::Mania)
//!         .query("blue army stars>3")
//!         .await
//!         .unwrap();
//!
//!     // Get the french wiki page on the osu file format
//!     let wiki_page: WikiPage = osu.wiki("fr")
//!         .page("Client/File_formats/osu_%28file_format%29")
//!         .await
//!         .unwrap();
//!     # };
//! }
//! ```
//!
//! ## Features
//!
//! | Flag          | Description                              | Dependencies
//! | ------------- | ---------------------------------------- | ------------
//! | `default`     | Enables the `cache` and `macros` features |
//! | `cache`       | Cache username-userid pairs so that fetching data by username does one instead of two requests | [`dashmap`]
//! | `macros`      | Re-exports `rosu-mods`'s `mods!` macro to easily create mods for a given mode | [`paste`]
//! | `serialize`   | Implement `serde::Serialize` for most types, allowing for manual serialization |
//! | `metrics`     | Uses the global metrics registry to store response time for each endpoint | [`metrics`]
//! | `replay`      | Enables the method `Osu::replay` to parse a replay. Note that `Osu::replay_raw` is available without this feature but provides raw bytes instead of a parsed replay | [`osu-db`]
//! | `local_oauth` | Enables the method `OsuBuilder::with_local_authorization` to perform the full OAuth procedure | `tokio/net` feature
//!
//! [osu!api v2]: https://osu.ppy.sh/docs/index.html
//! [`rosu`]: https://github.com/MaxOhn/rosu
//! [osu!api v1]: https://github.com/ppy/osu-api/wiki
//! [`dashmap`]: https://docs.rs/dashmap
//! [`paste`]: https://docs.rs/paste
//! [`metrics`]: https://docs.rs/metrics
//! [`osu-db`]: https://docs.rs/osu-db

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links, rustdoc::missing_crate_level_docs)]
#![warn(clippy::missing_const_for_fn, clippy::pedantic)]
#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::struct_excessive_bools,
    clippy::match_same_arms,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::explicit_iter_loop,
    clippy::similar_names,
    clippy::cast_possible_wrap,
    clippy::default_trait_access,
    clippy::ignored_unit_patterns
)]

mod client;
mod future;
mod routing;

/// Errors types
pub mod error;

/// All available data types provided by the api
pub mod model;

/// Request related types to fetch from endpoints
pub mod request;

mod metrics;

pub use self::client::{Osu, OsuBuilder};

#[macro_use]
extern crate tracing;

#[cfg(feature = "macros")]
extern crate rosu_mods;

#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use rosu_mods::mods;

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, error::OsuError>;

/// All types except requesting, stuffed into one module
pub mod prelude {
    pub use crate::{
        client::{Scopes, Token},
        error::OsuError,
        model::{
            beatmap::*,
            changelog::*,
            chat::*,
            comments::*,
            event::*,
            forum::*,
            kudosu::*,
            matches::*,
            matchmaking::*,
            mods::{generated_mods::*, Acronym, GameMods, GameModsIntermode, GameModsLegacy},
            multiplayer::*,
            news::*,
            ranking::*,
            score::*,
            seasonal_backgrounds::*,
            user::*,
            wiki::*,
            GameMode, Grade,
        },
        request::UserId,
        Osu, OsuBuilder, OsuResult,
    };

    pub use compact_str::CompactString;
    pub use hyper::StatusCode;

    #[cfg(feature = "macros")]
    #[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
    pub use rosu_mods::mods;
}
