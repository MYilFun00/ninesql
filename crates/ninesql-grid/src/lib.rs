//! Results grid: virtualized GPUI element over Arrow batches, under a memory budget (ADR-0004, invariant I6). ROADMAP: P3-B.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

pub use ninesql_core as core;

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-grid");
    }
}
