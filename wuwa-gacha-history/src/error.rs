use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Database error: {0}")]
    Db(#[from] toasty::Error),

    #[error("API error (code {code}): {message}")]
    Api { code: u32, message: String },
}
