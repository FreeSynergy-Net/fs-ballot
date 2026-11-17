//! The only source of randomness: the operating system, through `getrandom`.
//!
//! Pattern: Facade. Insertion positions, Fisher-Yates shuffles and tokens all come from here.

use crate::error::{Error, Result};

/// Fill `buf` from the system.
///
/// # Errors
/// [`Error::Random`] when the system has no randomness to give.
pub fn fill(buf: &mut [u8]) -> Result<()> {
    getrandom::fill(buf).map_err(|e| Error::Random(e.to_string()))
}

/// A uniform number in `0..n` (`n > 0`), by rejection so there is no bias. Crate-internal: the callers
/// (`shuffle`, `insert_random`) always pass at least 1; `n = 0` would divide by zero.
///
/// # Errors
/// See [`fill`].
pub(crate) fn below(n: usize) -> Result<usize> {
    debug_assert!(n > 0);
    let n = n as u64;
    let zone = u64::MAX - (u64::MAX % n);
    loop {
        let mut b = [0u8; 8];
        fill(&mut b)?;
        let x = u64::from_le_bytes(b);
        if x < zone {
            return Ok(usize::try_from(x % n).unwrap_or(0));
        }
    }
}

/// Fisher-Yates shuffle.
///
/// # Errors
/// See [`fill`].
pub fn shuffle<T>(items: &mut [T]) -> Result<()> {
    for i in (1..items.len()).rev() {
        items.swap(i, below(i + 1)?);
    }
    Ok(())
}

/// Insert at a random position, so the order says nothing about the order of arrival.
///
/// # Errors
/// See [`fill`].
pub fn insert_random<T>(items: &mut Vec<T>, item: T) -> Result<()> {
    let at = below(items.len() + 1)?;
    items.insert(at, item);
    Ok(())
}

/// Lower-case hex.
#[must_use]
pub fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// 256 random bits as 64 hex characters: a one-time token.
///
/// # Errors
/// See [`fill`].
pub fn token() -> Result<String> {
    let mut b = [0u8; 32];
    fill(&mut b)?;
    Ok(hex(&b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_callers_never_ask_below_for_zero() {
        // The smallest inputs of the two callers: `below(1)` and `below(2)`, never `below(0)`.
        let mut none: Vec<u8> = vec![];
        insert_random(&mut none, 7).unwrap();
        assert_eq!(none, [7]);
        shuffle(&mut Vec::<u8>::new()).unwrap();
        shuffle(&mut [1u8]).unwrap();
        assert_eq!(below(1).unwrap(), 0);
    }

    #[test]
    fn below_stays_in_range() {
        for n in 1..40 {
            assert!(below(n).unwrap() < n);
        }
    }
}
