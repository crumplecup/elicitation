//! Gallery for isolating `unexpected_cfgs` suppression patterns.
// Cases A, C, E intentionally trigger unexpected_cfgs to document the behaviour.
#![allow(unexpected_cfgs)]
//!
//! Each case uses a calibrated derive macro that emits `cfg(foo)` — a cfg name
//! not declared in this crate — matching what `cfg(kani)` / `cfg(creusot)` look
//! like in downstream crates. Running `cargo check` on this file reveals which
//! allow placement actually suppresses the lint.
//!
//! Confirmed results (run `cargo test -p elicitation_derive --test cfg_allow_gallery_test`):
//!
//! | Case | Pattern                                                    | Warning? |
//! |------|------------------------------------------------------------|----------|
//! | A    | No allow                                                   | YES      |
//! | B    | `#[allow]` + `#[cfg(foo)]` on **same** item               | YES !!   |
//! | C    | `#[allow]` on separate preceding const                     | YES      |
//! | D    | `#[allow]` on outer `const _: () = { ... }`               | **NO**   |
//! | E    | `cfg_attr(foo, ...)` with no allow                         | YES      |
//! | F    | `#[allow]` + `cfg_attr` on **same** item                   | YES !!   |
//! | G    | attr macro: push allow then cfg_attr at END of func.attrs  | YES      |
//! | H    | attr macro: insert allow at pos 0, push cfg_attr at END    | YES      |
//! | I    | attr macro: emit preceding allow const, then modified fn   | YES      |
//! | J    | attr macro: wrap entire fn in `#[allow] const _: () = {}` | **NO**   |
//! | K    | attr macro: `#[allow] mod { fn }` + `pub use`             | **NO**   |
//!
//! **KEY FINDINGS**:
//! - `#[allow(unexpected_cfgs)]` directly on an item (same or adjacent) does NOT suppress
//!   the lint when emitted from a proc macro into a downstream crate (cases B, F, G, H).
//! - The allow must be on an ENCLOSING item: a `const _: () = {}` wrapper (case D, J)
//!   or a `mod` block (case K).
//! - For derive macros emitting new impl blocks: use `const _: () = {}` (case D).
//! - For attribute macros modifying existing functions: use `#[allow] mod { fn } + pub use`
//!   (case K) — the function remains accessible from the outer scope.

// Case A: no allow → should produce 1 warning: unexpected `cfg` condition name: `foo`
#[derive(elicitation_derive::CfgGalleryA)]
pub struct GalleryA;

// Case B: allow before cfg on same item → should produce NO warning
#[derive(elicitation_derive::CfgGalleryB)]
pub struct GalleryB;

// Case C: allow on separate const before cfg item → should produce 1 warning
#[derive(elicitation_derive::CfgGalleryC)]
pub struct GalleryC;

// Case D: allow on outer const _ = {} wrapping the cfg item → should produce NO warning
#[derive(elicitation_derive::CfgGalleryD)]
pub struct GalleryD;

// Case E: cfg_attr(foo,...) with no allow → should produce 1 warning
#[derive(elicitation_derive::CfgGalleryE)]
pub struct GalleryE;

// Case F: allow before cfg_attr on same item → should produce NO warning
#[derive(elicitation_derive::CfgGalleryF)]
pub struct GalleryF;

// ── Attribute macro cases (G–J): modifying an existing function ──────────────
//
// These test what happens when an attribute macro pushes cfg/cfg_attr attrs
// onto an existing function item — modelling the formal_method pattern.

// Case G: push allow then cfg_attr at END of func.attrs → expected: warning
#[elicitation_derive::cfg_gallery_g]
fn gallery_g_fn() {}

// Case H: INSERT allow at pos 0, push cfg_attr at END → expected: warning
#[elicitation_derive::cfg_gallery_h]
fn gallery_h_fn() {}

// Case I: emit preceding allow const, then modified fn → expected: warning
#[elicitation_derive::cfg_gallery_i]
fn gallery_i_fn() {}

// Case J: wrap entire fn in #[allow] const _: () = { fn ... } → expected: NO warning
// (function inaccessible from outer scope — not viable for formal_method)
#[elicitation_derive::cfg_gallery_j]
#[allow(dead_code)]
fn gallery_j_fn() {}

// Case K: emit fn inside #[allow] mod { ... } then pub use → expected: NO warning
// Candidate fix for formal_method: function remains accessible.
#[elicitation_derive::cfg_gallery_k]
fn gallery_k_fn() {}

#[test]
fn gallery_compiles() {
    // Compilation is the test. All derives above must expand without errors.
    let _a = GalleryA;
    let _b = GalleryB;
    let _c = GalleryC;
    let _d = GalleryD;
    let _e = GalleryE;
    let _f = GalleryF;
    gallery_g_fn();
    gallery_h_fn();
    gallery_i_fn();
    // gallery_j_fn is inside const _ so not callable from here
    gallery_k_fn();
}
