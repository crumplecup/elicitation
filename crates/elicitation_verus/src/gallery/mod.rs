//! Verus proof gallery — learning curriculum for the VSM companion pattern.
//!
//! Each level targets one specific Verus feature, building toward the full
//! `VerifiedStateMachine` companion shape in [`level7`] and beyond.
//!
//! | Level | Focus |
//! |-------|-------|
//! | [`level1`] | Unit type baseline — confirms toolchain works |
//! | [`level2`] | Two-variant enum — Z3 ADT discriminant |
//! | [`level3`] | Enum with `u64` field — arithmetic invariant |
//! | [`level4`] | `open`/`closed` spec fn visibility tiers |
//! | [`level5`] | String fields with vstd `View` specs |
//! | [`level6`] | `Tracked<WfToken>` linear ghost token |
//! | [`level7`] | Full VSM pattern — state + exec + proof |
//! | [`level8`] | `#[verifier::type_invariant]` on private-field struct |
//! | [`level9`] | `assume_specification` for external functions |
//! | [`level10`] | Proof composition — chaining two ghost-token transitions |
//! | [`level11`] | Leaf `proof fn` lemmas — per-transition invariant atoms |
//! | [`level12`] | Composition dispatch — O(n) leaves + O(1) composition |
//! | [`level13`] | `assume_specification` in the multi-tool trust model |

pub mod level1;
pub mod level10;
pub mod level11;
pub mod level12;
pub mod level13;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
pub mod level7;
pub mod level8;
pub mod level9;
