use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request error.")]
    HttpError(#[from] reqwest::Error),
    #[error("System time error.")]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error("Base64 decode error.")]
    Base64DecodeError(#[from] base64::DecodeError),
    #[error("Client is not authorized. Are API env vars set?")]
    Unauthorized,
}
