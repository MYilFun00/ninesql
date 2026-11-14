//! Application core: connection model, services, Tokio driver runtime and the bridge to the UI (ADR-0004). ROADMAP: P0-D, P1-D, P3-C.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

// Dependency graph of docs/02-architecture.md, made explicit until real code uses these crates.
pub use ninesql_dbeaver as dbeaver;
pub use ninesql_driver_api as driver_api;
pub use ninesql_secrets as secrets;
pub use ninesql_ssh as ssh;
pub use ninesql_storage as storage;
pub use ninesql_transfer as transfer;

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-core");
    }
}
