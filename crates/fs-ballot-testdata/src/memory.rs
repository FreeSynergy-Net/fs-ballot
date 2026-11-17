//! The in-memory urn.
//!
//! Pattern: Adapter (the pure [`Lane`] behind the trait [`Urn`]).

use fs_ballot::{Error, Lane, Result, Rows, TokenHash, Urn, hash_token, random};

/// An urn in memory. Nothing is written anywhere.
#[derive(Debug)]
pub struct MemoryUrn<P> {
    min_group: u16,
    lane: Lane<P>,
}

impl<P> MemoryUrn<P> {
    /// An empty urn that releases only groups of at least `min_group`.
    #[must_use]
    pub fn new(min_group: u16) -> Self {
        Self {
            min_group,
            lane: Lane::new(),
        }
    }
}

impl<P> Urn<P> for MemoryUrn<P> {
    fn min_group(&self) -> u16 {
        self.min_group
    }

    fn issue_token(&mut self) -> Result<String> {
        let token = random::token()?;
        self.lane.add_token(hash_token(&token));
        Ok(token)
    }

    fn admit(&mut self, hash: TokenHash) {
        self.lane.add_token(hash);
    }

    fn cast(&mut self, token: &str, ballot: P) -> Result<()> {
        let hash = hash_token(token);
        if !self.lane.has_token(&hash) {
            return Err(Error::TokenUnknownOrSpent);
        }
        self.lane.add_ballot(ballot)?;
        self.lane.spend(&hash);
        Ok(())
    }

    fn release(&mut self, stage: u16) -> Result<usize> {
        self.lane.release(stage, self.min_group)
    }

    fn released(&self, stage: Option<u16>) -> Vec<(usize, &P)> {
        self.lane.released(stage)
    }

    fn held_back(&self) -> usize {
        self.lane.held_back()
    }

    fn rows(&self) -> Rows<'_, P> {
        Rows {
            pending: self.lane.pending().iter().collect(),
            batches: self
                .lane
                .batches()
                .iter()
                .map(|b| (b.stage(), b.ballots().iter().collect()))
                .collect(),
            tokens: self.lane.tokens(),
        }
    }
}
