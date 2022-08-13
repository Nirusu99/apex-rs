use thiserror::Error;

#[derive(Error, Debug)]
enum ApexError {
    #[error("Try again in a few minutes.")]
    APITimeout,
    #[error("Unauthorized / Unknown API key.")]
    InvalidAPIKey,
    #[error("Rate limit reached.")]
    RateLimited,
    #[error("Unknown error/internal error occured")]
    Unknown,
}
