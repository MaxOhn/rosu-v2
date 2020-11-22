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

/// Main error enum
#[derive(Debug)]
pub enum OsuError {
    /// Failed to build an [`crate::Osu`] client
    Builder(BuilderError),
    /// Reqwest failed to build its client.
    BuildingClient(ReqwestError),
    /// Error while handling response from the API
    ChunkingResponse(ReqwestError),
    /// Failed to create a request header
    CreatingHeader {
        name: &'static str,
        source: InvalidHeaderValue,
    },
    /// Attempted to make request without valid token
    NoToken,
    /// Failed to deserialize response
    Parsing { body: String, source: SerdeError },
    /// Failed to send request
    Request(ReqwestError),
    /// API returned an error
    Response {
        body: String,
        source: APIError,
        status: StatusCode,
    },
    /// Temporal (?) downtime of the osu API
    ServiceUnavailable(Option<String>),
    /// Failed to update token
    UpdateToken(Box<OsuError>),
}

impl StdError for OsuError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Builder(source) => Some(source),
            Self::BuildingClient(source) => Some(source),
            Self::ChunkingResponse(source) => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::NoToken => None,
            Self::Parsing { source, .. } => Some(source),
            Self::Request(source) => Some(source),
            Self::Response { source, .. } => Some(source),
            Self::ServiceUnavailable(_) => None,
            Self::UpdateToken(source) => Some(source),
        }
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Builder(_) => f.write_str("Failed to build osu client"),
            Self::BuildingClient(_) => f.write_str("Failed to build reqwest client"),
            Self::ChunkingResponse(_) => f.write_str("Failed to chunk the response"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "Failed to parse value for header {}", name)
            }
            Self::NoToken => f.write_str(
                "The previous API token expired and the client \
                has not yet succeeded acquiring a new one. \
                Can not send requests until a new token has been acquired. \
                This should only occur during an extended downtime of the osu API.",
            ),
            Self::Parsing { body, .. } => write!(f, "Failed to deserialize response: {}", body),
            Self::Request(_) => f.write_str("Failed to send request"),
            Self::Response { status, .. } => write!(f, "Response error, status {}", status),
            Self::ServiceUnavailable(body) => write!(
                f,
                "API may be temporarily unavailable (received 503): {}",
                body.as_deref().unwrap_or("error while parsing body")
            ),
            Self::UpdateToken(_) => f.write_str("Failed to update API token"),
        }
    }
}
