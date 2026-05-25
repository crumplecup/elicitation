//! Gallery level C58: split-file crate-local module tree.
//!
//! **Hypothesis**: if the inline crate-local module tree in C57 proves cleanly,
//! the remaining downstream ICE may require the same packaging to be spread
//! across real files the way `valinoreth` is: source code in `vsm/combat.rs`,
//! generated code in `proofs/creusot/generated/combat.rs`, plus sibling module
//! declarations and root re-exports.
//!
//! ```bash
//! cargo creusot prove -- -p elicitation_creusot --features gallery-c58-split-module-tree
//! ```

pub mod vsm;

#[cfg(creusot)]
pub mod proofs;

pub use vsm::combat::{C58Consistent, C58State, C58Step, c58_begin, c58_consistent};
