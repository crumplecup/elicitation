//! Creusot proof gallery for VSM invariant expressibility.
//!
//! Each level tests a specific hypothesis about what Pearlite can express,
//! mirroring the methodology used in the Kani gallery.  Unlike Kani (which
//! runs CBMC and measures wall-clock time), Creusot gallery levels are
//! validated by:
//!
//! 1. **Compilation** (`cargo creusot`): annotations parse and type-check
//!    in Pearlite.  Failure here means the predicate is inexpressible.
//! 2. **Goal discharge** (Why3 + Alt-Ergo/Z3): the generated WhyML proof
//!    obligations close.  For trivial goals this takes < 1 s.
//!
//! ## Gallery levels
//!
//! | Level   | Subject                              | Key question                          |
//! |---------|--------------------------------------|---------------------------------------|
//! | [`level1`] | Unit type, trivial invariant      | Does basic `#[logic]` / `#[requires]` work? |
//! | [`level2`] | Integer bounds (`@` model)        | Can Pearlite do arithmetic?           |
//! | [`level3`] | Unit enum, `match` in `#[logic]`  | Can predicates match on enum variants? |
//! | [`level4`] | String length via `@.len()`       | Can Pearlite reason about String?     |
//! | [`level5`] | Data-carrying enum (String payload) | Invariant over data variant field?   |
//! | [`level6`] | Composition (postcond → precond)  | Is composition free (no `stub_verified`)? |
//!
//! ## Run all levels
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```
//!
//! The output is WhyML in `verif/elicitation_creusot_rlib/`.
//! Each level's functions appear as separate Why3 modules.

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
