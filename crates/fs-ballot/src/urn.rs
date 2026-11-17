//! The trait of an urn: what every implementation (memory, typed store) has to offer.
//!
//! Pattern: Strategy (an implementation is one way to keep the lane) behind a small interface (ADR 0043).

use crate::error::Result;
use crate::token::TokenHash;

/// A ballot box without a log. A ballot is whatever the caller counts (`P`); the urn never looks into it.
///
/// The promises, checked by `fs-ballot-testdata`:
/// 1. **Once.** A token buys one ballot. A second cast with it is refused with
///    [`Error::TokenUnknownOrSpent`](crate::Error::TokenUnknownOrSpent), as is a token never issued.
/// 2. **No link.** After `cast`, nothing the urn keeps ties the token (or its hash) to the ballot; `cast`
///    gives no receipt; ballots have no time and no id; the order of pending ballots says nothing about
///    the order of arrival; the token hashes are sorted.
/// 3. **Minimum group.** `release` moves pending ballots out only when there are at least `min_group`
///    (a `min_group` of 0 counts as 1); a smaller rest stays hidden and is only counted (`held_back`).
pub trait Urn<P> {
    /// The smallest number of pending ballots that may be released together.
    fn min_group(&self) -> u16;

    /// Issue a token: 256 random bits as hex. Only its hash is kept; the token itself is returned once.
    ///
    /// # Errors
    /// [`Error::Random`](crate::Error::Random).
    fn issue_token(&mut self) -> Result<String>;

    /// Admit a token the caller made (its delivery is the caller's); only the hash is given.
    fn admit(&mut self, hash: TokenHash);

    /// Spend the token and add the ballot at a random position. All or nothing.
    ///
    /// # Errors
    /// [`Error::TokenUnknownOrSpent`](crate::Error::TokenUnknownOrSpent); [`Error::Random`](crate::Error::Random).
    fn cast(&mut self, token: &str, ballot: P) -> Result<()>;

    /// Release the pending ballots as one shuffled batch of `stage` if there are at least `min_group`;
    /// returns how many were released (0 if too few).
    ///
    /// # Errors
    /// [`Error::Random`](crate::Error::Random).
    fn release(&mut self, stage: u16) -> Result<usize>;

    /// Released ballots of `stage` (`None` = all), each with the number of its batch.
    fn released(&self, stage: Option<u16>) -> Vec<(usize, &P)>;

    /// Ballots cast but not released yet (a count, never the ballots).
    fn held_back(&self) -> usize;

    /// Everything the urn keeps, as typed rows. This is the surface a storage backend writes and the
    /// conformance checks inspect: there is no column that holds a token next to a ballot.
    fn rows(&self) -> Rows<'_, P>;
}

/// The stored state of an urn, as typed rows (no JSON, no blob).
#[derive(Debug, PartialEq)]
pub struct Rows<'a, P> {
    /// Rows of `pending`: ballots not released, in random order.
    pub pending: Vec<&'a P>,
    /// Rows of `batch`: `(stage, ballots)`, in the order of release.
    pub batches: Vec<(u16, Vec<&'a P>)>,
    /// Rows of `token`: the unspent hashes, sorted.
    pub tokens: &'a [TokenHash],
}
