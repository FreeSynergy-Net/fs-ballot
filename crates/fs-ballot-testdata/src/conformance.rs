//! The promises of [`Urn`] as checks. Each check builds a fresh urn with `make(min_group)` and returns
//! `Err(what went wrong)`; [`all`] runs them and names every failure.
//!
//! Anonymity is checked as far as the trait shows it: a spent token leaves no trace in [`Urn::rows`], the
//! token hashes are sorted, `cast` returns nothing, and the order of ballots does not follow the order
//! of arrival (30 trials of 8 ballots: all 30 equal to the arrival order has a chance of (1/8!)^30).
//!
//! Pattern: Template (one list of checks, run against any implementation of the trait).

use fs_ballot::{Error, Urn, hash_token};

/// A failed promise, in words.
pub type Check = Result<(), String>;

/// A check on a urn made by the given maker.
type Maker<'a, U> = &'a dyn Fn(u16) -> U;

/// A named check.
type Named<'a, U> = (&'a str, fn(Maker<'_, U>) -> Check);

fn ensure(ok: bool, what: &str) -> Check {
    if ok { Ok(()) } else { Err(what.to_owned()) }
}

/// Every check; the list of failures is empty when the urn keeps all promises.
///
/// # Errors
/// The failures, one line each.
pub fn all<U: Urn<String>>(make: impl Fn(u16) -> U) -> Result<(), Vec<String>> {
    let checks: [Named<'_, U>; 7] = [
        ("once", once),
        ("unknown token", unknown_token),
        ("no link", no_link),
        ("random order", random_order),
        ("minimum group", minimum_group),
        ("zero minimum counts as one", zero_minimum),
        ("stages", stages),
    ];
    let failures: Vec<String> = checks
        .iter()
        .filter_map(|(name, f)| f(&make).err().map(|e| format!("{name}: {e}")))
        .collect();
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

/// A token buys one ballot.
///
/// # Errors
/// What went wrong.
pub fn once<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(1);
    let t = u.issue_token().map_err(|e| e.to_string())?;
    u.cast(&t, "a".into()).map_err(|e| e.to_string())?;
    ensure(
        u.cast(&t, "b".into()) == Err(Error::TokenUnknownOrSpent),
        "a second cast with the same token was not refused",
    )?;
    ensure(u.held_back() == 1, "the refused cast still added a ballot")
}

/// A token never issued is refused, and the refusal changes nothing.
///
/// # Errors
/// What went wrong.
pub fn unknown_token<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(1);
    let kept = u.issue_token().map_err(|e| e.to_string())?;
    ensure(
        u.cast("not-issued", "x".into()) == Err(Error::TokenUnknownOrSpent),
        "an unknown token was accepted",
    )?;
    ensure(u.held_back() == 0, "the refused cast added a ballot")?;
    ensure(
        u.rows().tokens == [hash_token(&kept)],
        "the refused cast changed the tokens",
    )
}

/// Nothing the urn keeps ties a token to a ballot.
///
/// # Errors
/// What went wrong.
pub fn no_link<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(1);
    let tokens: Vec<String> = (0..12)
        .map(|_| u.issue_token().map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    ensure(
        u.rows()
            .tokens
            .iter()
            .all(|h| h.len() == 64 && !tokens.contains(h)),
        "the urn keeps a token itself, not its hash",
    )?;
    for (i, t) in tokens.iter().take(5).enumerate() {
        u.cast(t, format!("ballot {i}"))
            .map_err(|e| e.to_string())?;
    }
    let rows = u.rows();
    ensure(rows.tokens.len() == 7, "a spent token is still kept")?;
    for t in &tokens[..5] {
        ensure(
            !rows.tokens.contains(&hash_token(t)),
            "the hash of a spent token is still kept",
        )?;
    }
    ensure(
        rows.tokens.windows(2).all(|w| w[0] < w[1]),
        "the token hashes are not sorted (their order would tell who was invited when)",
    )
}

/// The order of ballots does not follow the order of arrival, pending or released.
///
/// # Errors
/// What went wrong.
pub fn random_order<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let arrival: Vec<String> = (0..8).map(|i| i.to_string()).collect();
    let (mut pending_moved, mut released_moved) = (false, false);
    for _ in 0..30 {
        let mut u = make(1);
        for b in &arrival {
            let t = u.issue_token().map_err(|e| e.to_string())?;
            u.cast(&t, b.clone()).map_err(|e| e.to_string())?;
        }
        let p: Vec<String> = u.rows().pending.into_iter().cloned().collect();
        pending_moved |= p != arrival;
        u.release(1).map_err(|e| e.to_string())?;
        let r: Vec<String> = u
            .released(None)
            .into_iter()
            .map(|(_, b)| b.clone())
            .collect();
        ensure(r.len() == 8, "the release lost or invented ballots")?;
        released_moved |= r != arrival;
    }
    ensure(pending_moved, "pending ballots keep the order of arrival")?;
    ensure(released_moved, "released ballots keep the order of arrival")
}

/// Only groups of at least `min_group` come out; a smaller rest stays hidden and counted.
///
/// # Errors
/// What went wrong.
pub fn minimum_group<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(3);
    ensure(u.min_group() == 3, "min_group is not the one given")?;
    for i in 0..2 {
        let t = u.issue_token().map_err(|e| e.to_string())?;
        u.cast(&t, format!("b{i}")).map_err(|e| e.to_string())?;
    }
    ensure(
        u.release(1) == Ok(0),
        "two ballots were released below a minimum of 3",
    )?;
    ensure(
        u.held_back() == 2,
        "held_back does not count the hidden ballots",
    )?;
    ensure(
        u.released(None).is_empty(),
        "a ballot of a too small group is visible",
    )?;
    let t = u.issue_token().map_err(|e| e.to_string())?;
    u.cast(&t, "b2".into()).map_err(|e| e.to_string())?;
    ensure(u.release(1) == Ok(3), "a group of 3 was not released")?;
    ensure(u.held_back() == 0, "released ballots are still held back")
}

/// A minimum of 0 behaves like 1; an empty urn releases nothing.
///
/// # Errors
/// What went wrong.
pub fn zero_minimum<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(0);
    ensure(u.release(1) == Ok(0), "an empty urn released something")?;
    let t = u.issue_token().map_err(|e| e.to_string())?;
    u.cast(&t, "only".into()).map_err(|e| e.to_string())?;
    ensure(
        u.release(1) == Ok(1),
        "a single ballot was not released at minimum 0",
    )
}

/// Later ballots form a later batch; reading by stage gives only that stage.
///
/// # Errors
/// What went wrong.
pub fn stages<U: Urn<String>>(make: &dyn Fn(u16) -> U) -> Check {
    let mut u = make(1);
    for (stage, b) in [(1_u16, "one"), (2, "two")] {
        let t = u.issue_token().map_err(|e| e.to_string())?;
        u.cast(&t, b.into()).map_err(|e| e.to_string())?;
        u.release(stage).map_err(|e| e.to_string())?;
    }
    ensure(u.released(None).len() == 2, "not all ballots are released")?;
    let one: Vec<&String> = u.released(Some(1)).into_iter().map(|(_, b)| b).collect();
    ensure(
        one == [&"one".to_owned()],
        "stage 1 does not hold exactly its ballot",
    )?;
    let batches: Vec<u16> = u.rows().batches.iter().map(|(s, _)| *s).collect();
    ensure(batches == [1, 2], "batches are not in the order of release")
}
