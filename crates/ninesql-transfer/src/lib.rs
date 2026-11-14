//! Data transfer: streamed export (CSV, JSON, SQL, Markdown, HTML…) and import with bounded memory (invariant I6). ROADMAP: P5-C.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-transfer");
    }
}
