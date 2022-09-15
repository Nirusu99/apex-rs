use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApexError {
    #[error("Try again in a few minutes.")]
    APITimeout,
    #[error("Unauthorized / Unknown API key.")]
    InvalidAPIKey,
    #[error("Rate limit reached.")]
    RateLimited,
    #[error("Unknown error/internal error occured.")]
    Unknown,
    #[error("Couldn't parse url.")]
    URLParseError(#[from] url::ParseError),
    #[error("Couldn't make http request.")]
    RequestError(#[from] reqwest::Error),
    #[error("Couldn't parse response from the apex api.")]
    ResponseParseError(#[from] serde_json::Error),
    #[error("Currently unavailable")]
    Unavailable,
}

unsafe impl Send for ApexError {}
unsafe impl Sync for ApexError {}
