//! Mutants: urns that break one promise each. The checks must catch every one (and only that one).
use fs_ballot::{Result, Rows, TokenHash, Urn, hash_token};
use fs_ballot_testdata::{MemoryUrn, conformance};

#[derive(Clone, Copy, PartialEq)]
enum Fault {
    /// No fault: the control.
    None,
    /// Spending does not take the token away.
    NoSpend,
    /// Ignores the minimum group.
    NoMinimum,
    /// Gives ballots back in the order of arrival.
    Ordered,
    /// Keeps all hashes in the order of issue, spent ones too.
    HashLog,
}

struct Faulty {
    fault: Fault,
    asked_min: u16,
    inner: MemoryUrn<String>,
    log: Vec<TokenHash>,
    arrival: Vec<String>,
    arrival_view: Vec<(usize, String)>,
}

impl Faulty {
    fn new(fault: Fault, min: u16) -> Self {
        let inner_min = if fault == Fault::NoMinimum { 1 } else { min };
        Self {
            fault,
            asked_min: min,
            inner: MemoryUrn::new(inner_min),
            log: Vec::new(),
            arrival: Vec::new(),
            arrival_view: Vec::new(),
        }
    }
}

impl Urn<String> for Faulty {
    fn min_group(&self) -> u16 {
        self.asked_min
    }
    fn issue_token(&mut self) -> Result<String> {
        let t = self.inner.issue_token()?;
        self.log.push(hash_token(&t));
        Ok(t)
    }
    fn admit(&mut self, hash: TokenHash) {
        self.inner.admit(hash);
    }
    fn cast(&mut self, token: &str, ballot: String) -> Result<()> {
        self.inner.cast(token, ballot.clone())?;
        self.arrival.push(ballot);
        if self.fault == Fault::NoSpend {
            self.inner.admit(hash_token(token));
        }
        Ok(())
    }
    fn release(&mut self, stage: u16) -> Result<usize> {
        let n = self.inner.release(stage)?;
        if n > 0 {
            self.arrival_view = self.arrival.drain(..).map(|b| (0, b)).collect();
        }
        Ok(n)
    }
    fn released(&self, stage: Option<u16>) -> Vec<(usize, &String)> {
        if self.fault == Fault::Ordered {
            return self.arrival_view.iter().map(|(i, b)| (*i, b)).collect();
        }
        self.inner.released(stage)
    }
    fn held_back(&self) -> usize {
        self.inner.held_back()
    }
    fn rows(&self) -> Rows<'_, String> {
        let mut rows = self.inner.rows();
        if self.fault == Fault::HashLog {
            rows.tokens = &self.log;
        }
        if self.fault == Fault::Ordered {
            rows.pending = self.arrival.iter().collect();
        }
        rows
    }
}

fn failures(fault: Fault) -> Vec<String> {
    conformance::all(|m| Faulty::new(fault, m)).expect_err("the mutant must be caught")
}

#[test]
fn the_wrapper_without_a_fault_passes() {
    // Control: only the fault breaks a promise, not the wrapper.
    conformance::all(|m| Faulty::new(Fault::None, m)).unwrap();
}

#[test]
fn a_token_that_is_not_spent_is_caught() {
    let f = failures(Fault::NoSpend);
    assert!(f.iter().any(|l| l.starts_with("once")), "{f:?}");
}

#[test]
fn an_ignored_minimum_group_is_caught() {
    let f = failures(Fault::NoMinimum);
    assert!(f.iter().any(|l| l.starts_with("minimum group")), "{f:?}");
}

#[test]
fn ballots_in_order_of_arrival_are_caught() {
    let f = failures(Fault::Ordered);
    assert!(f.iter().any(|l| l.starts_with("random order")), "{f:?}");
}

#[test]
fn a_log_of_token_hashes_is_caught() {
    let f = failures(Fault::HashLog);
    assert!(f.iter().any(|l| l.starts_with("no link")), "{f:?}");
}
