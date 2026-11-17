//! The lane: the pure logic of one urn (pending ballots, released batches, unspent token hashes).
//!
//! Pattern: Aggregate. One value, changed only through its methods. A ballot has no person, no time and
//! no id; it enters at a random position; release shuffles it. The token hashes are a sorted list kept
//! apart from the ballots, so nothing ties a hash to a ballot.

use crate::error::Result;
use crate::random::{insert_random, shuffle};
use crate::token::TokenHash;

/// Counts that travel with the ballots of a batch (for example pattern counts of an open survey): they
/// join the pending ones and are released together with a batch. `()` is "none".
pub trait Meta: Default {
    /// Add the counts of `other` to these.
    fn absorb(&mut self, other: &Self);
}

impl Meta for () {
    fn absorb(&mut self, (): &Self) {}
}

/// Ballots released together at one stage.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(
        serialize = "P: serde::Serialize, M: serde::Serialize",
        deserialize = "P: serde::Deserialize<'de>, M: serde::Deserialize<'de> + Default"
    ))
)]
pub struct Batch<P, M = ()> {
    stage: u16,
    ballots: Vec<P>,
    /// The counts of these ballots, released with them.
    #[cfg_attr(feature = "serde", serde(default, rename = "signals"))]
    meta: M,
}

impl<P, M> Batch<P, M> {
    /// The stage at which the batch was released.
    #[must_use]
    pub fn stage(&self) -> u16 {
        self.stage
    }

    /// The ballots, in shuffled order.
    #[must_use]
    pub fn ballots(&self) -> &[P] {
        &self.ballots
    }

    /// The counts released with the batch.
    #[must_use]
    pub fn meta(&self) -> &M {
        &self.meta
    }
}

/// Everything of one urn.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        default,
        bound(
            serialize = "P: serde::Serialize, M: serde::Serialize",
            deserialize = "P: serde::Deserialize<'de>, M: serde::Deserialize<'de> + Default"
        )
    )
)]
pub struct Lane<P, M = ()> {
    pending: Vec<P>,
    batches: Vec<Batch<P, M>>,
    tokens: Vec<TokenHash>,
    /// Counts of the pending ballots; they join a batch when it is released.
    #[cfg_attr(feature = "serde", serde(rename = "pending_signals"))]
    pending_meta: M,
}

impl<P, M: Default> Default for Lane<P, M> {
    fn default() -> Self {
        Self {
            pending: Vec::new(),
            batches: Vec::new(),
            tokens: Vec::new(),
            pending_meta: M::default(),
        }
    }
}

impl<P, M: Meta> Lane<P, M> {
    /// An empty lane.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a ballot at a random position among the pending ones.
    ///
    /// # Errors
    /// [`Error::Random`](crate::Error::Random) when the system gives no randomness; nothing is added then.
    pub fn add_ballot(&mut self, ballot: P) -> Result<()> {
        insert_random(&mut self.pending, ballot)
    }

    /// Add a ballot together with its counts (kept as counts, not with the ballot).
    ///
    /// # Errors
    /// See [`Lane::add_ballot`].
    pub fn add_ballot_with(&mut self, ballot: P, meta: &M) -> Result<()> {
        self.add_ballot(ballot)?;
        self.pending_meta.absorb(meta);
        Ok(())
    }

    /// Keep a token hash (sorted, so the order says nothing about who was invited when); a hash already
    /// there is not added twice.
    pub fn add_token(&mut self, hash: TokenHash) {
        if let Err(at) = self.tokens.binary_search(&hash) {
            self.tokens.insert(at, hash);
        }
    }

    /// Whether the hash is an unspent token of this lane.
    #[must_use]
    pub fn has_token(&self, hash: &str) -> bool {
        self.tokens
            .binary_search_by(|x| x.as_str().cmp(hash))
            .is_ok()
    }

    /// Remove a token hash; false if it is not there (spent or never issued).
    pub fn spend(&mut self, hash: &str) -> bool {
        self.tokens
            .binary_search_by(|t| t.as_str().cmp(hash))
            .map(|at| self.tokens.remove(at))
            .is_ok()
    }

    /// Release the pending ballots as one shuffled batch of `stage`, if there are at least `min_group` of
    /// them (a `min_group` of 0 counts as 1); a smaller rest stays. Returns how many were released.
    ///
    /// # Errors
    /// See [`Lane::add_ballot`]; nothing is released then.
    pub fn release(&mut self, stage: u16, min_group: u16) -> Result<usize> {
        if self.pending.is_empty() || self.pending.len() < usize::from(min_group) {
            return Ok(0);
        }
        shuffle(&mut self.pending)?;
        let ballots = std::mem::take(&mut self.pending);
        let released = ballots.len();
        let meta = std::mem::take(&mut self.pending_meta);
        self.batches.push(Batch {
            stage,
            ballots,
            meta,
        });
        Ok(released)
    }

    /// Everything of `other` joins this lane (pending, batches, token hashes; the hashes stay sorted).
    /// For a stored form that once kept one lane where there are two now.
    pub fn merge_from(&mut self, mut other: Self) {
        self.pending.append(&mut other.pending);
        self.pending_meta.absorb(&other.pending_meta);
        self.batches.append(&mut other.batches);
        self.tokens.append(&mut other.tokens);
        self.tokens.sort();
        self.tokens.dedup();
    }
}

impl<P, M> Lane<P, M> {
    /// The released batches, in the order of release.
    #[must_use]
    pub fn batches(&self) -> &[Batch<P, M>] {
        &self.batches
    }

    /// Ballots not released yet.
    #[must_use]
    pub fn held_back(&self) -> usize {
        self.pending.len()
    }

    /// The pending ballots, in random order.
    #[must_use]
    pub fn pending(&self) -> &[P] {
        &self.pending
    }

    /// The unspent token hashes, sorted.
    #[must_use]
    pub fn tokens(&self) -> &[TokenHash] {
        &self.tokens
    }

    /// Whether the lane holds any ballot, released or not.
    #[must_use]
    pub fn has_ballots(&self) -> bool {
        !self.pending.is_empty() || !self.batches.is_empty()
    }

    /// Released ballots of `stage` (`None` = all), each with the number of its batch.
    #[must_use]
    pub fn released(&self, stage: Option<u16>) -> Vec<(usize, &P)> {
        self.batches
            .iter()
            .enumerate()
            .filter(|(_, b)| stage.is_none_or(|s| b.stage == s))
            .flat_map(|(i, b)| b.ballots.iter().map(move |x| (i, x)))
            .collect()
    }
}
