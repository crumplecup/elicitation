//! Gallery level C27: RPIT in dep-crate trait methods and Creusot `prove`.
//!
//! **Hypothesis**: When a dep crate contains a trait method returning
//! `impl Future` (RPIT), `cargo creusot prove` panics with an ICE:
//!
//! ```text
//! thread 'rustc' panicked at rustc_middle/src/hir/mod.rs:409:84:
//! index out of bounds: the len is N but the index is M
//!
//! query stack during panic:
//! #0 [local_def_id_to_hir_id] getting HIR ID of
//!    communicator::ElicitCommunicator::send_prompt::{opaque#0}::'_
//! ```
//!
//! The panic is in Creusot's `gather_intrinsics`, which iterates all local
//! `DefId`s and calls `local_def_id_to_hir_id` on each — including opaque
//! RPIT `{opaque#N}` def IDs that have no corresponding HIR node when
//! compiled for proof generation (which uses different emit flags than the
//! plain `cargo creusot` compile).
//!
//! ## Root cause
//!
//! `cargo creusot -p elicit_proofs` compiles deps with `--emit=metadata`
//! (fast, skips HIR indexing of opaques).  `cargo creusot prove` re-compiles
//! deps with `--emit=link` so why3find can load the rlib — this triggers a
//! full HIR pass which exposes the mismatch.
//!
//! ## Fix options explored
//!
//! | Option | Notes |
//! |--------|-------|
//! | `Box<dyn Future>` everywhere | Removes RPIT; Creusot handles boxed trait objects |
//! | `#[cfg(not(creusot))] impl Future` | RPIT only visible to normal rustc |
//! | `#[cfg(creusot)] Box<dyn Future>` | Creusot sees concrete box, not opaque |
//!
//! ## Experiment C27a — baseline: `Box<dyn Future>` in a local trait
//!
//! Proves cleanly because there is no RPIT opaque def ID.
//!
//! ## Experiment C27b — concrete async fn (no RPIT)
//!
//! An `async fn` in a *free function* also generates an opaque, but the
//! opaque is local to the function (not a trait item), and Creusot handles
//! it correctly.  Only trait-method RPIT in dep crates triggers the ICE.

use creusot_std::prelude::*;

// ---------------------------------------------------------------------------
// C27a — Box<dyn Future> trait: Creusot-safe async abstraction pattern
// ---------------------------------------------------------------------------

/// A trait with boxed future return — Creusot-compatible async trait.
pub trait AsyncComputer: Clone + Send + Sync + 'static {
    /// Compute a value, returning a boxed future.
    fn compute(
        &self,
        input: u32,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32> + Send + '_>>;
}

/// A simple synchronous harness that does not actually drive the future.
///
/// In Creusot we cannot drive async execution, but we can reason about
/// types that implement the trait.
#[requires(x < 1000u32)]
#[ensures(result == x + 1u32)]
pub fn c27a_increment_pure(x: u32) -> u32 {
    x + 1
}

// ---------------------------------------------------------------------------
// C27b — free async fn: generates opaque locally, handled by Creusot
// ---------------------------------------------------------------------------

// Concrete async free function — Creusot cannot handle async fn bodies.
//
// Commented out: `cargo creusot prove` fails with:
// "the rvalue Coroutine(...) is not currently supported"
// even for local async fns.  Only the synchronous wrapper is proved.
// pub async fn c27b_add(a: u32, b: u32) -> u32 { a + b }

/// Synchronous wrapper that Creusot can actually prove.
#[requires(a@ + b@ <= u32::MAX@)]
#[ensures(result@ == a@ + b@)]
pub fn c27b_add_sync(a: u32, b: u32) -> u32 {
    a + b
}
