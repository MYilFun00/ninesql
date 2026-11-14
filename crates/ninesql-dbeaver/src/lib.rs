//! DBeaver interoperability: read-only workspace import and export to a new data-sources.json (ADR-0006, invariants I1 and I4). ROADMAP: P2-A, P5-D. Security review required for every change.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-dbeaver");
    }
}
