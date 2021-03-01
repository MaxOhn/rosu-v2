use reqwest::{header::InvalidHeaderValue, Error as ReqwestError, StatusCode};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt};

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, OsuError>;

#[derive(Debug, Deserialize)]
/// The API response was of the form `{ "error": ... }`
pub struct APIError {
    error: String,
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.error.is_empty() {
            f.write_str("Empty error message")
        } else {
            f.write_str(&self.error)
        }
    }
}

impl StdError for APIError {}

/// Error while building [`crate::Osu`] client
#[derive(Debug)]
pub enum BuilderError {
    ClientId,
    ClientSecret,
}

impl StdError for BuilderError {}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientId => f.write_str("no client id was provided"),
            Self::ClientSecret => f.write_str("no client secret was provided"),
        }
    }
}

#[derive(Debug)]
/// Failed to parse [`crate::model::GameMods`] either from `u32` or `&str`.
pub enum ModError {
    U32(u32),
    Str,
}

impl fmt::Display for ModError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::U32(n) => write!(f, "can not parse u32 `{}` into GameMods", n),
            Self::Str => f.write_str("error while parsing string into GameMods"),
        }
    }
}

impl StdError for ModError {}

#[derive(Debug)]
pub enum ValueEnum {
    MapType,
    RankStatus,
    RankingType,
    ScoreType,
}

/// Main error enum
#[derive(Debug)]
pub enum OsuError {
    /// Failed to build an [`crate::Osu`] client
    Builder { source: BuilderError },
    /// Reqwest failed to build its client.
    BuildingClient { source: ReqwestError },
    /// Error while handling response from the API
    ChunkingResponse { source: ReqwestError },
    /// Failed to create a request header
    CreatingHeader {
        name: &'static str,
        source: InvalidHeaderValue,
    },
    /// Failed to request because of missing parameter
    MissingParameter { param: ValueEnum }, // TODO: Remove
    /// Failed to parse [`crate::model::GameMods`] either from `u32` or `&str`
    ModParsing { source: ModError },
    /// Attempted to make request without valid token
    NoToken,
    /// Failed to deserialize response
    Parsing { body: String, source: SerdeError },
    /// Failed to parse a value
    ParsingValue { value: ValueEnum },
    /// Failed to send request
    Request { source: ReqwestError },
    /// API returned an error
    Response {
        body: String,
        source: APIError,
        status: StatusCode,
    },
    /// Temporal (?) downtime of the osu API
    ServiceUnavailable(Option<String>),
    /// Failed to update token
    UpdateToken { source: Box<OsuError> },
}

impl StdError for OsuError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Builder { source } => Some(source),
            Self::BuildingClient { source } => Some(source),
            Self::ChunkingResponse { source } => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::MissingParameter { .. } => None,
            Self::ModParsing { source } => Some(source),
            Self::NoToken => None,
            Self::Parsing { source, .. } => Some(source),
            Self::ParsingValue { .. } => None,
            Self::Request { source } => Some(source),
            Self::Response { source, .. } => Some(source),
            Self::ServiceUnavailable(_) => None,
            Self::UpdateToken { source } => Some(source),
        }
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Builder { .. } => f.write_str("Failed to build osu client"),
            Self::BuildingClient { .. } => f.write_str("Failed to build reqwest client"),
            Self::ChunkingResponse { .. } => f.write_str("Failed to chunk the response"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "Failed to parse value for header {}", name)
            }
            Self::MissingParameter { param } => {
                write!(f, "Missing parameter for request: {:?}", param)
            }
            Self::ModParsing { .. } => f.write_str("Failed to parse GameMods"),
            Self::NoToken => f.write_str(
                "The previous API token expired and the client \
                has not yet succeeded in acquiring a new one. \
                Can not send requests until a new token has been acquired. \
                This should only occur during an extended downtime of the osu!api.",
            ),
            Self::Parsing { body, .. } => write!(f, "Failed to deserialize response: {}", body),
            Self::ParsingValue { value } => write!(f, "Failed to parse {:?}", value),
            Self::Request { .. } => f.write_str("Failed to send request"),
            Self::Response { status, .. } => write!(f, "Response error, status {}", status),
            Self::ServiceUnavailable(body) => write!(
                f,
                "osu!api may be temporarily unavailable (received 503): {}",
                body.as_deref().unwrap_or("error while parsing body")
            ),
            Self::UpdateToken { .. } => f.write_str("Failed to update osu!api token"),
        }
    }
}
