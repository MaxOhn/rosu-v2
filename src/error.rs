use reqwest::{header::InvalidHeaderValue, Error as ReqwestError, StatusCode};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt};

/// `Result<_, OsuError>`
pub type OsuResult<T> = Result<T, OsuError>;

#[derive(Debug, Deserialize)]
/// The API response was of the form `{ "error": ... }`
pub struct APIError {
    pub error: Option<String>,
}

impl StdError for APIError {}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error {
            Some(ref msg) => f.write_str(msg),
            None => f.write_str("Empty error message"),
        }
    }
}

/// The main error type
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
    /// Attempted to make request without valid token
    NoToken,
    /// Failed to deserialize response
    Parsing { body: String, source: SerdeError },
    /// Failed to parse a value
    ParsingValue { source: ParsingError },
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
            Self::BuilderMissingID => None,
            Self::BuilderMissingSecret => None,
            Self::BuildingClient { source } => Some(source),
            Self::ChunkingResponse { source } => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::MissingParameter { .. } => None,
            Self::NoToken => None,
            Self::Parsing { source, .. } => Some(source),
            Self::ParsingValue { source } => Some(source),
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
            Self::NoToken => f.write_str(
                "The previous API token expired and the client \
                has not yet succeeded in acquiring a new one. \
                Can not send requests until a new token has been acquired. \
                This should only occur during an extended downtime of the osu!api.",
            ),
            Self::Parsing { body, .. } => write!(f, "Failed to deserialize response: {}", body),
            Self::ParsingValue { .. } => f.write_str("Failed to parse value"),
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

impl From<ParsingError> for OsuError {
    fn from(e: ParsingError) -> Self {
        Self::ParsingValue { source: e }
    }
}

/// Failed some TryFrom parsing
#[derive(Debug)]
pub enum ParsingError {
    Genre(u8),
    Grade(String),
    Language(u8),
    ModsU32(u32),
    ModsStr(String),
    RankStatus(i8),
    ScoringType(u8),
    Team(u8),
    TeamType(u8),
}

impl StdError for ParsingError {}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Genre(n) => write!(f, "Failed to parse {} into Genre", n),
            Self::Grade(s) => write!(f, "Failed to parse `{}` into Grade", s),
            Self::Language(n) => write!(f, "Failed to parse {} into Language", n),
            Self::ModsU32(n) => write!(f, "Failed to parse {} into GameMods", n),
            Self::ModsStr(s) => write!(f, "Failed to parse `{}` into GameMods", s),
            Self::RankStatus(n) => write!(f, "Failed to parse {} into RankStatus", n),
            Self::ScoringType(n) => write!(f, "Failed to parse {} into ScoringType", n),
            Self::Team(n) => write!(f, "Failed to parse {} into Team", n),
            Self::TeamType(n) => write!(f, "Failed to parse {} into TeamType", n),
        }
    }
}
