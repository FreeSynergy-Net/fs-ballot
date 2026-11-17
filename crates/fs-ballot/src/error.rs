//! Errors of the urn.
//!
//! Pattern: error enum, one variant per refusal.

/// Why the urn refused.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// The token was never issued here, or is spent. The two cases are not told apart on purpose.
    #[error("the token is unknown or already spent")]
    TokenUnknownOrSpent,
    /// The system gave no randomness.
    #[error("no randomness: {0}")]
    Random(String),
}

/// Result type of this crate.
pub type Result<T> = std::result::Result<T, Error>;
