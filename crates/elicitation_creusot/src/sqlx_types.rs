//! Creusot proofs for sqlx type elicitation.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that the sqlx crate correctly defines its enum variants.
//! We verify our own business logic: that every label produced by `labels()`
//! is accepted by `from_label()` (roundtrip completeness), that unknown labels
//! are rejected, and that our owned `SqlTypeKind` conversion from
//! `AnyTypeInfoKind` is total.
//!
//! # Why all functions are `#[trusted]`
//!
//! **String literal opacity wall** (blocks roundtrip + rejection proofs):
//! `str::view()` is `#[logic(opaque)]` in creusot-std — the SMT solver cannot
//! inspect the content of string literals like `"Null"`.
//! Even with `extern_spec!` contracts on `from_label`, the solver cannot prove
//! that a specific string literal is accepted or rejected without symbolic
//! string reasoning.
//!
//! # Partial de-trusting opportunity
//!
//! The `label_count` proofs are the best candidates for de-trusting.
//! An `extern_spec!` block specifying `#[ensures(result@.len() == N)]` for
//! `labels()` and `options()` would let Alt-Ergo discharge `N == N` without
//! `#[trusted]`.

#![cfg(feature = "sqlx-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// sqlx::error::ErrorKind — 5 variants
// ============================================================================

/// Verify that a known ErrorKind label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_known_label_accepted() -> bool {
    sqlx::error::ErrorKind::from_label("UniqueViolation").is_some()
}

/// Verify that all ErrorKind labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_all_labels_roundtrip() -> bool {
    sqlx::error::ErrorKind::labels()
        .iter()
        .all(|label| sqlx::error::ErrorKind::from_label(label).is_some())
}

/// Verify that an unknown ErrorKind label is rejected.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_error_kind_unknown_rejected() -> bool {
    sqlx::error::ErrorKind::from_label("__unknown__").is_none()
}

// ============================================================================
// sqlx::any::AnyTypeInfoKind — 9 variants
// ============================================================================

/// Verify that a known AnyTypeInfoKind label is accepted by from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_any_type_info_kind_known_label_accepted() -> bool {
    sqlx::any::AnyTypeInfoKind::from_label("Null").is_some()
}

/// Verify that all AnyTypeInfoKind labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_any_type_info_kind_all_labels_roundtrip() -> bool {
    sqlx::any::AnyTypeInfoKind::labels()
        .iter()
        .all(|label| sqlx::any::AnyTypeInfoKind::from_label(label).is_some())
}

/// Verify that an unknown AnyTypeInfoKind label is rejected.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_any_type_info_kind_unknown_rejected() -> bool {
    sqlx::any::AnyTypeInfoKind::from_label("__unknown__").is_none()
}

// ============================================================================
// elicitation::SqlTypeKind — 9 variants (our owned type)
// ============================================================================

/// Verify that all SqlTypeKind labels round-trip through from_label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_sql_type_kind_all_labels_roundtrip() -> bool {
    elicitation::SqlTypeKind::labels()
        .iter()
        .all(|label| elicitation::SqlTypeKind::from_label(label).is_some())
}

/// Verify that an unknown SqlTypeKind label is rejected.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_sql_type_kind_unknown_rejected() -> bool {
    elicitation::SqlTypeKind::from_label("__unknown__").is_none()
}

/// Verify that the SqlTypeKind::from(AnyTypeInfoKind) conversion is total.
///
/// Every AnyTypeInfoKind variant produces a valid SqlTypeKind.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_sql_type_kind_from_any_type_info_kind_total() -> bool {
    use elicitation::SqlTypeKind;
    use sqlx::any::AnyTypeInfoKind;
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Null);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Bool);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::SmallInt);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Integer);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::BigInt);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Real);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Double);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Text);
    let _ = SqlTypeKind::from(AnyTypeInfoKind::Blob);
    true
}
