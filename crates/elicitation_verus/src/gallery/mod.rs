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

pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod level6;
