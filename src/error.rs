use reqwest::{header::InvalidHeaderValue, Error as ReqwestError, StatusCode};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt};

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, OsuError>;

#[derive(Debug, Deserialize)]
/// The API response was of the form `{ "error": ... }`
pub(crate) struct APIError {
    pub(crate) error: String,
}

#[derive(Debug)]
pub enum OsuError {
    /// Failed to build an [`Osu`](crate::Osu) client because no client id was provided
    BuilderMissingID,
    /// Failed to build an [`Osu`](crate::Osu) client because no client secret was provided
    BuilderMissingSecret,
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
    MissingParameter { param: &'static str },
    /// Failed to parse [`GameMods`](crate::model::GameMods) from `&str`
    ModParsingString,
    /// Failed to parse [`GameMods`](crate::model::GameMods) from `u32`
    ModParsingU32(u32),
    /// Attempted to make request without valid token
    NoToken,
    /// Failed to deserialize response
    Parsing { body: String, source: SerdeError },
    /// Failed to parse a value
    ParsingValue { value: &'static str },
    /// Failed to send request
    Request { source: ReqwestError },
    /// API returned an error
    Response {
        body: String,
        source: Option<String>,
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
            Self::BuilderMissingID => None,
            Self::BuilderMissingSecret => None,
            Self::BuildingClient { source } => Some(source),
            Self::ChunkingResponse { source } => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::MissingParameter { .. } => None,
            Self::ModParsingString => None,
            Self::ModParsingU32(_) => None,
            Self::NoToken => None,
            Self::Parsing { source, .. } => Some(source),
            Self::ParsingValue { .. } => None,
            Self::Request { source } => Some(source),
            Self::Response { .. } => None,
            Self::ServiceUnavailable(_) => None,
            Self::UpdateToken { source } => Some(source),
        }
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BuilderMissingID => {
                f.write_str("Failed to build osu client, no client id was provided")
            }
            Self::BuilderMissingSecret => {
                f.write_str("Failed to build osu client, no client secret was provided")
            }
            Self::BuildingClient { .. } => f.write_str("Failed to build reqwest client"),
            Self::ChunkingResponse { .. } => f.write_str("Failed to chunk the response"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "Failed to parse value for header {}", name)
            }
            Self::MissingParameter { param } => {
                write!(f, "Missing parameter for request: {}", param)
            }
            Self::ModParsingString => f.write_str("Failed to parse GameMods from a &str"),
            Self::ModParsingU32(n) => write!(f, "Failed to parse GameMods from {}_u32", n),
            Self::NoToken => f.write_str(
                "The previous API token expired and the client \
                has not yet succeeded in acquiring a new one. \
                Can not send requests until a new token has been acquired. \
                This should only occur during an extended downtime of the osu!api.",
            ),
            Self::Parsing { body, .. } => write!(f, "Failed to deserialize response: {}", body),
            Self::ParsingValue { value } => write!(f, "Failed to parse {:?}", value),
            Self::Request { .. } => f.write_str("Failed to send request"),
            Self::Response { status, source, .. } => match source {
                Some(source) => write!(f, "Response error, status {} | source: {}", status, source),
                None => write!(f, "Response error, status {}", status),
            },
            Self::ServiceUnavailable(body) => write!(
                f,
                "osu!api may be temporarily unavailable (received 503): {}",
                body.as_deref().unwrap_or("error while parsing body")
            ),
            Self::UpdateToken { .. } => f.write_str("Failed to update osu!api token"),
        }
    }
}
