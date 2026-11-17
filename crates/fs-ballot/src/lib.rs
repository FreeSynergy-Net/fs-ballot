//! `fs-ballot`: an anonymous ballot box without a log, with one-time tokens and a minimum group.
//!
//! The voter holds a token; the urn keeps only `sha256(token)` (a sorted list, apart from the ballots).
//! A ballot goes in at a random position, without person, time or id. Ballots come out only in shuffled
//! batches of at least `min_group`. [`Lane`] is the pure logic, [`Urn`] the trait a backend implements;
//! counting lives in `fs-tally`.
//!
//! Limits, stated: whoever runs the machine can read memory while a ballot passes; a group of one
//! voter shows that voter's ballot (hence the minimum group); the network address and the time of
//! arrival at the caller are the caller's to protect.

#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod error;
mod lane;
pub mod random;
mod token;
mod urn;

pub use error::{Error, Result};
pub use lane::{Batch, Lane, Meta};
pub use token::{TokenHash, hash_token};
pub use urn::{Rows, Urn};
