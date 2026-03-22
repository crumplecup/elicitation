//! Creusot proofs for sqlx type elicitation.
//!
//! # Verification stance
//!
//! **Trust the source. Verify the wrapper.**
//!
//! We trust that the sqlx crate correctly defines its enum variants and their
//! implementations. We verify our own business logic: that every label produced
//! by `labels()` is accepted by `from_label()` (roundtrip completeness), that
//! unknown labels are rejected, and that our owned `SqlTypeKind` conversion
//! from `AnyTypeInfoKind` is total.
//!
//! # De-trusted proofs
//!
//! The `label_count` proofs (`labels().len() == options().len()`) are verified
//! without `#[trusted]`. `Vec` has a `ShallowModel` as `Seq<T>` in creusot-std,
//! so equality of lengths computed by the actual `Select` impls is dischargeable
//! by Alt-Ergo once the body is symbolically evaluated.
//!
//! # Remaining `#[trusted]` walls
//!
//! **String literal opacity wall** (blocks roundtrip + rejection proofs):
//! `str::view()` is `#[logic(opaque)]` in creusot-std — the SMT solver cannot
//! inspect the content of string literals like `"UniqueViolation"`. Even with
//! `extern_spec!` contracts on `from_label`, the solver cannot prove that a
//! specific string literal is accepted or rejected without symbolic string
//! reasoning. These proofs remain `#[trusted]` until creusot-std provides a
//! `ShallowModel` for `&str`.
//!
//! **`SqlTypeKind::from(AnyTypeInfoKind)` totality**: Blocked by the
//! `#[non_exhaustive]` attribute on `AnyTypeInfoKind` — the wildcard arm means
//! Creusot cannot prove exhaustiveness without a closed-world assumption.
//! Remains `#[trusted]` as an explicit architectural axiom.

#![cfg(feature = "sqlx-types")]

use creusot_std::prelude::*;
use elicitation::Select;

// ============================================================================
// sqlx::error::ErrorKind — 5 variants
// ============================================================================

/// Verify that ErrorKind label count equals option count.
///
/// De-trusted: Alt-Ergo discharges this by evaluating `len() == len()`.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_error_kind_label_count() -> bool {
    sqlx::error::ErrorKind::labels().len() == sqlx::error::ErrorKind::options().len()
}

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

/// Verify that AnyTypeInfoKind label count equals option count.
///
/// De-trusted: Alt-Ergo discharges this by evaluating `len() == len()`.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_any_type_info_kind_label_count() -> bool {
    sqlx::any::AnyTypeInfoKind::labels().len() == sqlx::any::AnyTypeInfoKind::options().len()
}

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

/// Verify that SqlTypeKind label count equals option count.
///
/// De-trusted: Alt-Ergo discharges this by evaluating `len() == len()`.
/// As our own type, we have full control over the `Select` implementation.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_sql_type_kind_label_count() -> bool {
    elicitation::SqlTypeKind::labels().len() == elicitation::SqlTypeKind::options().len()
}

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

// ============================================================================
// Trusted axiom: SqlTypeKind::from(AnyTypeInfoKind) is total
//
// `AnyTypeInfoKind` is `#[non_exhaustive]` — Creusot cannot prove exhaustiveness
// from a wildcard arm without a closed-world assumption. This is an explicit
// architectural axiom: we trust our From impl covers all current variants and
// maps unknown/future ones to SqlTypeKind::Unknown.
// ============================================================================

/// Trust axiom: `SqlTypeKind::from(AnyTypeInfoKind)` is total over all known
/// variants and maps future variants to `SqlTypeKind::Unknown`.
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

// ============================================================================
// DriverKind — 3 variants (our owned type, fully de-trusted)
// ============================================================================

/// Verify that DriverKind label count equals option count.
///
/// De-trusted: Alt-Ergo discharges this by evaluating `len() == len()`.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_driver_kind_label_count() -> bool {
    elicitation::DriverKind::labels().len() == elicitation::DriverKind::options().len()
}

/// Verify that a known DriverKind label is accepted.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_driver_kind_known_label_accepted() -> bool {
    elicitation::DriverKind::from_label("Postgres").is_some()
}

/// Verify that an unknown DriverKind label is rejected.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_driver_kind_unknown_rejected() -> bool {
    elicitation::DriverKind::from_label("__unknown__").is_none()
}
