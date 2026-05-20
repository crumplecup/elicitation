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
//! | Case | Pattern                                        | Warning? |
//! |------|------------------------------------------------|----------|
//! | A    | No allow                                       | YES      |
//! | B    | `#[allow]` + `#[cfg(foo)]` on **same** item   | YES !!   |
//! | C    | `#[allow]` on separate preceding const         | YES      |
//! | D    | `#[allow]` on outer `const _: () = { ... }`   | **NO**   |
//! | E    | `cfg_attr(foo, ...)` with no allow             | YES      |
//! | F    | `#[allow]` + `cfg_attr` on **same** item       | YES !!   |
//!
//! **KEY FINDING**: Placing `#[allow(unexpected_cfgs)]` directly before
//! `#[cfg(foo)]` on the SAME item does NOT suppress the lint when emitted
//! from a proc macro. The ONLY working pattern is to wrap the cfg'd item
//! inside `#[allow(unexpected_cfgs)] const _: () = { ... }` (Case D).

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

#[test]
fn gallery_compiles() {
    // Compilation is the test. All derives above must expand without errors.
    let _a = GalleryA;
    let _b = GalleryB;
    let _c = GalleryC;
    let _d = GalleryD;
    let _e = GalleryE;
    let _f = GalleryF;
}
