//! `fs-ballot-testdata`: the test double of `fs-ballot` and the conformance checks of the trait `Urn`.
//!
//! [`MemoryUrn`] keeps the lane in memory. [`conformance`] holds the promises of `Urn` as checks that run
//! against ANY implementation; a backend runs [`conformance::all`] in its own tests.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

pub mod conformance;
mod memory;

pub use memory::MemoryUrn;
