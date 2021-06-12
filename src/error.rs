use hyper::{
    header::InvalidHeaderValue, http::Error as HttpError, Error as HyperError, StatusCode,
};
use serde::Deserialize;
use serde_json::Error as SerdeError;
use std::{error::Error as StdError, fmt};
use url::ParseError;

/// The API response was of the form `{ "error": ... }`
#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub error: Option<String>,
}

impl StdError for ApiError {}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error {
            Some(ref msg) => f.write_str(msg),
            None => f.write_str("empty error message"),
        }
    }
}

/// The main error type
#[derive(Debug)]
#[non_exhaustive]
pub enum OsuError {
    /// Failed to create a request body
    BodyError { source: HttpError },
    /// Failed to build an [`Osu`](crate::Osu) client because no client id was provided
    BuilderMissingId,
    /// Failed to build an [`Osu`](crate::Osu) client because no client secret was provided
    BuilderMissingSecret,
    /// Error while handling response from the API
    ChunkingResponse { source: HyperError },
    /// Failed to create a request header
    CreatingHeader {
        name: &'static str,
        source: InvalidHeaderValue,
    },
    /// The API returned a 404
    NotFound,
    /// Attempted to make request without valid token
    NoToken,
    /// Failed to deserialize response
    Parsing { body: String, source: SerdeError },
    /// Failed to parse a value
    ParsingValue { source: ParsingError },
    /// Failed to send request
    Request { source: HyperError },
    /// Timeout while requesting from API
    RequestTimeout,
    /// API returned an error
    Response {
        body: String,
        source: ApiError,
        status: StatusCode,
    },
    /// Temporal (?) downtime of the osu API
    ServiceUnavailable(Option<String>),
    /// The client's authentication is not sufficient for the endpoint
    UnavailableEndpoint,
    /// Failed to update token
    UpdateToken { source: Box<OsuError> },
    /// Failed to parse the URL for a request
    Url { source: ParseError, url: String },
}

impl StdError for OsuError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::BodyError { source } => Some(source),
            Self::BuilderMissingId => None,
            Self::BuilderMissingSecret => None,
            Self::ChunkingResponse { source } => Some(source),
            Self::CreatingHeader { source, .. } => Some(source),
            Self::NotFound => None,
            Self::NoToken => None,
            Self::Parsing { source, .. } => Some(source),
            Self::ParsingValue { source } => Some(source),
            Self::Request { source } => Some(source),
            Self::RequestTimeout => None,
            Self::Response { source, .. } => Some(source),
            Self::ServiceUnavailable(_) => None,
            Self::UnavailableEndpoint => None,
            Self::UpdateToken { source } => Some(source),
            Self::Url { source, .. } => Some(source),
        }
    }
}

impl fmt::Display for OsuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::BodyError { .. } => f.write_str("failed to create request body"),
            Self::BuilderMissingId => {
                f.write_str("failed to build osu client, no client id was provided")
            }
            Self::BuilderMissingSecret => {
                f.write_str("failed to build osu client, no client secret was provided")
            }
            Self::ChunkingResponse { .. } => f.write_str("failed to chunk the response"),
            Self::CreatingHeader { name, .. } => {
                write!(f, "failed to parse value for header {}", name)
            }
            Self::NotFound => f.write_str(
                "the osu!api returned a 404 implying a missing score, incorrect name, id, etc",
            ),
            Self::NoToken => f.write_str(
                "The previous osu!api token expired and the client \
                has not yet succeeded in acquiring a new one. \
                Can not send requests until a new token has been acquired. \
                This should only occur during an extended downtime of the osu!api.",
            ),
            Self::Parsing { body, .. } => write!(f, "failed to deserialize response: {}", body),
            Self::ParsingValue { .. } => f.write_str("failed to parse value"),
            Self::Request { .. } => f.write_str("failed to send request"),
            Self::RequestTimeout => f.write_str("osu!api did not respond in time"),
            Self::Response { status, .. } => write!(f, "response error, status {}", status),
            Self::ServiceUnavailable(body) => write!(
                f,
                "osu!api may be temporarily unavailable (received 503): {}",
                body.as_deref().unwrap_or("error while parsing body")
            ),
            Self::UnavailableEndpoint => {
                f.write_str("the endpoint is not available for the client's authorization level")
            }
            Self::UpdateToken { .. } => f.write_str("failed to update osu!api token"),
            Self::Url { url, .. } => write!(f, "failed to parse URL of a request; url: `{}`", url),
        }
    }
}

impl From<HttpError> for OsuError {
    fn from(e: HttpError) -> Self {
        Self::BodyError { source: e }
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
            Self::Genre(n) => write!(f, "failed to parse {} into Genre", n),
            Self::Grade(s) => write!(f, "failed to parse `{}` into Grade", s),
            Self::Language(n) => write!(f, "failed to parse {} into Language", n),
            Self::ModsU32(n) => write!(f, "failed to parse {} into GameMods", n),
            Self::ModsStr(s) => write!(f, "failed to parse `{}` into GameMods", s),
            Self::RankStatus(n) => write!(f, "failed to parse {} into RankStatus", n),
            Self::ScoringType(n) => write!(f, "failed to parse {} into ScoringType", n),
            Self::Team(n) => write!(f, "failed to parse {} into Team", n),
            Self::TeamType(n) => write!(f, "failed to parse {} into TeamType", n),
        }
    }
}
