//! The in-memory urn keeps every promise of the trait.
use fs_ballot_testdata::{MemoryUrn, conformance};

#[test]
fn memory_urn_passes_all_checks() {
    conformance::all(MemoryUrn::<String>::new).unwrap();
}
