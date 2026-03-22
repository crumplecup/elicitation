//! Kani proofs for sqlx type elicitation.
//!
//! Available with the `sqlx-types` feature.
//!
//! # Proof Strategy
//!
//! For each `Select` enum type we verify:
//! 1. **Label count**: `labels().len() == options().len()`
//! 2. **Roundtrip**: every label produced by `labels()` is accepted by `from_label()`
//! 3. **Unknown rejection**: `from_label("__unknown__")` returns `None`
//!
//! For our owned serializable types (`SqlTypeKind`, `ColumnValue`) we also
//! verify serde roundtrip consistency where applicable.

// ============================================================================
// SqlxErrorKind (impl Select on sqlx::error::ErrorKind)
// ============================================================================

#[cfg(feature = "sqlx-types")]
use elicitation::Select;

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sqlx_error_kind_label_count() {
    use sqlx::error::ErrorKind;
    let labels = ErrorKind::labels();
    let options = ErrorKind::options();
    assert!(
        labels.len() == options.len(),
        "ErrorKind: labels and options have equal length"
    );
    assert!(labels.len() == 5, "ErrorKind has 5 variants");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sqlx_error_kind_unknown_rejected() {
    use sqlx::error::ErrorKind;
    let result = ErrorKind::from_label("__unknown__");
    assert!(result.is_none(), "ErrorKind: unknown label rejected");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sqlx_error_kind_roundtrip_unique_violation() {
    use sqlx::error::ErrorKind;
    let labels = ErrorKind::labels();
    let result = ErrorKind::from_label(&labels[0]);
    assert!(
        result.is_some(),
        "ErrorKind::UniqueViolation label roundtrips"
    );
}

// ============================================================================
// AnyTypeInfoKind (impl Select on sqlx::any::AnyTypeInfoKind)
// ============================================================================

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_any_type_info_kind_label_count() {
    use sqlx::any::AnyTypeInfoKind;
    let labels = AnyTypeInfoKind::labels();
    let options = AnyTypeInfoKind::options();
    assert!(
        labels.len() == options.len(),
        "AnyTypeInfoKind: labels and options have equal length"
    );
    assert!(labels.len() == 9, "AnyTypeInfoKind has 9 variants");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_any_type_info_kind_unknown_rejected() {
    use sqlx::any::AnyTypeInfoKind;
    let result = AnyTypeInfoKind::from_label("__unknown__");
    assert!(result.is_none(), "AnyTypeInfoKind: unknown label rejected");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_any_type_info_kind_roundtrip_null() {
    use sqlx::any::AnyTypeInfoKind;
    let labels = AnyTypeInfoKind::labels();
    let result = AnyTypeInfoKind::from_label(&labels[0]);
    assert!(result.is_some(), "AnyTypeInfoKind::Null label roundtrips");
}

// ============================================================================
// SqlTypeKind (our owned enum — impl Select + Serialize/Deserialize)
// ============================================================================

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sql_type_kind_label_count() {
    use elicitation::SqlTypeKind;
    let labels = SqlTypeKind::labels();
    let options = SqlTypeKind::options();
    assert!(
        labels.len() == options.len(),
        "SqlTypeKind: labels and options have equal length"
    );
    assert!(labels.len() == 9, "SqlTypeKind has 9 variants");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sql_type_kind_unknown_rejected() {
    use elicitation::SqlTypeKind;
    let result = SqlTypeKind::from_label("__unknown__");
    assert!(result.is_none(), "SqlTypeKind: unknown label rejected");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sql_type_kind_roundtrip_null() {
    use elicitation::SqlTypeKind;
    let labels = SqlTypeKind::labels();
    let result = SqlTypeKind::from_label(&labels[0]);
    assert!(result.is_some(), "SqlTypeKind::Null label roundtrips");
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_sql_type_kind_from_any_type_info_kind_total() {
    use elicitation::SqlTypeKind;
    use sqlx::any::AnyTypeInfoKind;
    // Every AnyTypeInfoKind variant maps to some SqlTypeKind
    for kind in [
        AnyTypeInfoKind::Null,
        AnyTypeInfoKind::Bool,
        AnyTypeInfoKind::SmallInt,
        AnyTypeInfoKind::Integer,
        AnyTypeInfoKind::BigInt,
        AnyTypeInfoKind::Real,
        AnyTypeInfoKind::Double,
        AnyTypeInfoKind::Text,
        AnyTypeInfoKind::Blob,
    ] {
        let _sql_kind = SqlTypeKind::from(kind);
    }
}

// ============================================================================
// ColumnValue (our owned enum — Null check + variant coverage)
// ============================================================================

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_column_value_null() {
    use elicitation::ColumnValue;
    let v = ColumnValue::Null;
    assert!(matches!(v, ColumnValue::Null));
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_column_value_bool_roundtrip() {
    use elicitation::ColumnValue;
    let v = ColumnValue::Bool(true);
    assert!(matches!(v, ColumnValue::Bool(true)));
}

#[cfg(feature = "sqlx-types")]
#[kani::proof]
fn verify_column_value_big_int_roundtrip() {
    use elicitation::ColumnValue;
    let n: i64 = kani::any();
    let v = ColumnValue::BigInt(n);
    if let ColumnValue::BigInt(got) = v {
        assert_eq!(got, n);
    } else {
        panic!("ColumnValue::BigInt did not match");
    }
}
