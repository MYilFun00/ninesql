//! Secrets: Secret Service backend, Argon2id + XChaCha20-Poly1305 vault, secret handles (ADR-0005, invariant I1). ROADMAP: P0-D, P1-D. Security review required for every change.
//!
//! Status: skeleton. See the ROADMAP task named below before adding code.
#![forbid(unsafe_code)]

/// Crate name, used by tests and diagnostics until real items exist.
pub const CRATE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
mod tests {
    #[test]
    fn crate_name_is_set() {
        assert_eq!(super::CRATE, "ninesql-secrets");
    }
}
