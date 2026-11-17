//! One-time tokens: the voter holds the token, the urn keeps only `sha256(token)`.
//!
//! Pattern: Facade. This is the one place that turns a token into what the urn keeps.

use sha2::{Digest, Sha256};

use crate::random::hex;

/// What the urn keeps of a token: `sha256(token)` in lower-case hex.
pub type TokenHash = String;

/// The hash of a token.
#[must_use]
pub fn hash_token(token: &str) -> TokenHash {
    hex(&Sha256::digest(token.as_bytes()))
}
