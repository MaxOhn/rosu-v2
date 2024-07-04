use hyper::{
    header::InvalidHeaderValue, http::Error as HttpError, Error as HyperError, StatusCode,
};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::fmt;

/// The API response was of the form `{ "error": ... }`
#[derive(Debug, Deserialize, thiserror::Error)]
pub struct ApiError {
    /// Error specified by the API
    pub error: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.error {
            Some(ref msg) => f.write_str(msg),
            None => f.write_str("empty error message"),
        }
    }
}

/// The main error type
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OsuError {
    /// Failed to create a request body
    #[error("failed to create request body")]
    BodyError {
        #[from]
        source: HttpError,
    },
    /// Failed to build an [`Osu`](crate::Osu) client because no client id was provided
    #[error("failed to build osu client, no client id was provided")]
    BuilderMissingId,
    /// Failed to build an [`Osu`](crate::Osu) client because no client secret was provided
    #[error("failed to build osu client, no client secret was provided")]
    BuilderMissingSecret,
    /// Error while handling response from the API
    #[error("failed to chunk the response")]
    ChunkingResponse {
        #[source]
        source: HyperError,
    },
    /// Failed to create the token header for a request
    #[error("failed to parse token for authorization header")]
    CreatingTokenHeader {
        #[from]
        source: InvalidHeaderValue,
    },
    /// The API returned a 404
    #[error("the osu!api returned a 404 implying a missing score, incorrect name, id, etc")]
    NotFound,
    /// Attempted to make request without valid token
    #[error(
        "The previous osu!api token expired and the client \
        has not yet succeeded in acquiring a new one. \
        Can not send requests until a new token has been acquired. \
        This should only occur during an extended downtime of the osu!api."
    )]
    NoToken,
    #[cfg(feature = "replay")]
    /// There was an error while trying to use osu-db
    #[error("osu-db error")]
    OsuDbError {
        #[from]
        source: osu_db::Error,
    },
    /// Failed to deserialize response
    #[error("failed to deserialize response: {}", .body)]
    Parsing {
        body: String,
        #[source]
        source: SerdeError,
    },
    /// Failed to parse a value
    #[error("failed to parse value")]
    ParsingValue {
        #[from]
        source: ParsingError,
    },
    /// Failed to send request
    #[error("failed to send request")]
    Request {
        #[source]
        source: HyperError,
    },
    /// Timeout while requesting from API
    #[error("osu!api did not respond in time")]
    RequestTimeout,
    /// API returned an error
    #[error("response error, status {}", .status)]
    Response {
        body: String,
        #[source]
        source: ApiError,
        status: StatusCode,
    },
    /// Temporal (?) downtime of the osu API
    #[error("osu!api may be temporarily unavailable (received 503): {}", .0)]
    ServiceUnavailable(String),
    /// The client's authentication is not sufficient for the endpoint
    #[error("the endpoint is not available for the client's authorization level")]
    UnavailableEndpoint,
    /// Failed to update token
    #[error("failed to update osu!api token")]
    UpdateToken {
        #[source]
        source: Box<OsuError>,
    },
    /// Failed to parse the URL for a request
    #[error("failed to parse URL of a request; url: `{}`", .url)]
    Url {
        #[source]
        source: url::ParseError,
        /// URL that was attempted to be parsed
        url: String,
    },
}

/// Failed some TryFrom parsing
#[derive(Debug, thiserror::Error)]
pub enum ParsingError {
    /// Failed to parse a str into an [`Acronym`](crate::model::mods::Acronym)
    #[error("failed to parse `{}` into an Acronym", .0)]
    Acronym(Box<str>),
    /// Failed to parse a u8 into a [`Genre`](crate::model::beatmap::Genre)
    #[error("failed to parse {} into Genre", .0)]
    Genre(u8),
    /// Failed to parse a String into a [`Grade`](crate::model::Grade)
    #[error("failed to parse `{}` into Grade", .0)]
    Grade(String), // TODO: make Box<str>
    /// Failed to parse a u8 into a [`Language`](crate::model::beatmap::Language)
    #[error("failed to parse {} into Language", .0)]
    Language(u8),
    /// Failed to parse an i8 into a [`RankStatus`](crate::model::beatmap::RankStatus)
    #[error("failed to parse {} into RankStatus", .0)]
    RankStatus(i8),
    /// Failed to parse a u8 into a [`ScoringType`](crate::model::matches::ScoringType)
    #[error("failed to parse {} into ScoringType", .0)]
    ScoringType(u8),
    /// Failed to parse a u8 into a [`Team`](crate::model::matches::Team)
    #[error("failed to parse {} into Team", .0)]
    Team(u8),
    /// Failed to parse a u8 into a [`TeamType`](crate::model::matches::TeamType)
    #[error("failed to parse {} into TeamType", .0)]
    TeamType(u8),
}
