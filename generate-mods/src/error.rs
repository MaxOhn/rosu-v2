pub type GenResult<T = (), E = Error> = Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("json error")]
    Json(#[from] serde_json::Error),
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
}
