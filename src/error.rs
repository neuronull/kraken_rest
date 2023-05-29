use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug, Clone, Deserialize)]
pub enum Error {
    #[error("HTTP request error: {err}")]
    HttpError { err: String, status: Option<u16> },
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::HttpError {
            err: e.to_string(),
            status: e.status().map(|c| c.as_u16()),
        }
    }
}
