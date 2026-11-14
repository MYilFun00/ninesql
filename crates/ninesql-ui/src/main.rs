//! NineSQL desktop application.
//!
//! Status: skeleton. The GPUI window arrives with the spike (ROADMAP P0-A, ADR-0001).
#![forbid(unsafe_code)]

// Dependency graph of docs/02-architecture.md, made explicit until the UI uses these crates.
use ninesql_core as _;
use ninesql_editor as _;
use ninesql_grid as _;

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut out = std::io::stdout().lock();
    let line = format!(
        "ninesql {} (pre-alpha: no UI yet, see docs/ROADMAP.md P0-A)",
        env!("CARGO_PKG_VERSION")
    );
    match writeln!(out, "{line}") {
        Ok(()) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
