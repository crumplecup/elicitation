//! Generate subcommand: scan source trees and emit proof companion files.
//!
//! The `elicitation generate` subcommand replaces hand-maintained `build.rs`
//! proof generation.  It scans Rust source files with `syn`, extracts
//! `VsmDescriptor`s from `#[derive(VerifiedStateMachine)]` structs and their
//! companions, then writes toolchain-specific proof files.
//!
//! Toolchain targets:
//! - `kani`    — Kani harnesses (`#[kani::proof]`)
//! - `verus`   — Verus leaf lemmas + composition (vargo-compatible)
//! - `creusot` — Creusot `#[cfg(creusot)]` companions
//! - `all`     — all three targets

pub mod creusot_gen;
pub mod kani_gen;
pub mod scanner;
pub mod verus_gen;

pub use scanner::{
    ArgDescriptor, ArgKind, PropDescriptor, TransitionFn, VsmDescriptor, extract_prop_descriptor,
    extract_vsms_from_file, has_derive, scan_vsms,
};

/// Walk up from `search_from` to find the nearest `Cargo.toml` and return its
/// `[package] name` value.
///
/// Falls back to `"crate"` when no manifest is found (e.g. in unit tests).
pub fn find_crate_name(search_from: &std::path::Path) -> String {
    let mut dir: &std::path::Path = search_from;
    loop {
        let manifest = dir.join("Cargo.toml");
        if manifest.exists() {
            if let Ok(content) = std::fs::read_to_string(&manifest) {
                for line in content.lines() {
                    let t = line.trim();
                    // Match `name = "foo"` or `name = 'foo'`, skip workspace
                    // re-declarations like `name.workspace = true`.
                    if t.starts_with("name") && t.contains('=') && !t.contains('.') {
                        if let Some(val) = t.splitn(2, '=').nth(1) {
                            let name = val.trim().trim_matches('"').trim_matches('\'').trim();
                            if !name.is_empty() && !name.contains('{') {
                                return name.to_string();
                            }
                        }
                    }
                }
            }
        }
        match dir.parent() {
            Some(p) => dir = p,
            None => return "crate".to_string(),
        }
    }
}
