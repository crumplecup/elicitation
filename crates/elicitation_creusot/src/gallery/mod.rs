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
//! | Level        | Subject                              | Key question                                    |
//! |--------------|--------------------------------------|-------------------------------------------------|
//! | [`level1`]  | Unit type, trivial invariant         | Does basic `#[logic]` / `#[requires]` work?    |
//! | [`level2`]  | Integer bounds (`@` model)           | Can Pearlite do arithmetic?                     |
//! | [`level3`]  | Unit enum, `match` in `#[logic]`    | Can predicates match on enum variants?          |
//! | [`level4`]  | String length via `@.len()`          | Can Pearlite reason about String?               |
//! | [`level5`]  | Data-carrying enum (String payload)  | Invariant over data variant field?              |
//! | [`level6`]  | Composition (postcond → precond)    | Is composition free (no `stub_verified`)?       |
//! | [`level7`]  | Named struct fields in variants      | Can predicates access named enum struct fields? |
//! | [`level8`]  | Machine wrapper struct               | Invariant over state enum + numeric metadata?   |
//! | [`level9`]  | Contract chains vs proof tokens      | Is `Established<P>` unnecessary in Creusot?     |
//! | [`level10`] | Full mini connection machine         | Complete 4-state VSM lifecycle provable?        |
//! | [`level11`] | Panel machine with nested enum       | Nested enum field access in pearlite?           |
//! | [`level12`] | Two-machine composition + gating     | Cross-machine invariant (panel gates on conn)?  |
//! | [`level13`] | Machine wrapper + transition counter | Exact counter postconditions chain `below_max`? |
//!
//! ## Run all levels
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```
//!
//! The output is WhyML in `verif/elicitation_creusot_rlib/`.
//! Each level's functions appear as separate Why3 modules.
//!
//! ## Prove gallery
//!
//! ```bash
//! just verify-creusot-gallery
//! ```

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;
pub mod level10;
pub mod level11;
pub mod level12;
pub mod level13;
