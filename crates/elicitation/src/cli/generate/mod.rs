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

pub mod scanner;

pub use scanner::{
    PropDescriptor, VsmDescriptor, extract_prop_descriptor, extract_vsms_from_file, has_derive,
    scan_vsms,
};
