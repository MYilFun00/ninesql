//! Driver API: `Driver`, `Session`, `Metadata`, `Dialect`, `Capabilities`, result streams (ADR-0004). ROADMAP: P0-B. Security review required for every change.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-driver-api");
    }
}
