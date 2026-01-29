use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Connection failed: {0}")]
    Connection(String),

    #[error("SSE stream error: {0}")]
    Sse(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),
}

pub type Result<T> = std::result::Result<T, Error>;
