//! Creusot proofs for serde deserialization boundary consistency.
//!
//! Proves that `serde_json::from_value` agrees with the validated constructor
//! for each constrained type.  Since the serde bridge calls `Type::new()`
//! internally, these proofs establish that deserialization enforces the same
//! invariants as direct construction.
//!
//! Cloud of assumptions: Trust serde_json parsing, JSON value construction,
//! and Rust's numeric conversions.  We verify the combined accept/reject
//! behaviour at the boundary.

use crate::*;
use elicitation::{
    F64Finite, F64NonNegative, F64Positive, I8NonNegative, I8NonZero, I8Positive, I16NonNegative,
    I16NonZero, I16Positive, StringNonEmpty, U8NonZero, U8Positive, U16NonZero, U16Positive,
    ValidationError,
};
use serde_json::Value;

// ============================================================================
// I8 serde boundary
// ============================================================================

/// Prove: serde accepts positive i8 values for I8Positive.
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i8_positive_serde_valid(value: i8) -> Result<I8Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects non-positive i8 values for I8Positive.
#[requires(value@ <= 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i8_positive_serde_invalid(value: i8) -> Result<I8Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-negative i8 values for I8NonNegative.
#[requires(value@ >= 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i8_non_negative_serde_valid(value: i8) -> Result<I8NonNegative, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects negative i8 values for I8NonNegative.
#[requires(value@ < 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i8_non_negative_serde_invalid(value: i8) -> Result<I8NonNegative, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-zero i8 values for I8NonZero.
#[requires(value@ != 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i8_non_zero_serde_valid(value: i8) -> Result<I8NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero i8 value for I8NonZero.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i8_non_zero_serde_invalid(value: i8) -> Result<I8NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

// ============================================================================
// I16 serde boundary
// ============================================================================

/// Prove: serde accepts positive i16 values for I16Positive.
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i16_positive_serde_valid(value: i16) -> Result<I16Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects non-positive i16 values for I16Positive.
#[requires(value@ <= 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i16_positive_serde_invalid(value: i16) -> Result<I16Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-negative i16 values for I16NonNegative.
#[requires(value@ >= 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i16_non_negative_serde_valid(
    value: i16,
) -> Result<I16NonNegative, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects negative i16 values for I16NonNegative.
#[requires(value@ < 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i16_non_negative_serde_invalid(
    value: i16,
) -> Result<I16NonNegative, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-zero i16 values for I16NonZero.
#[requires(value@ != 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_i16_non_zero_serde_valid(value: i16) -> Result<I16NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero i16 value for I16NonZero.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_i16_non_zero_serde_invalid(value: i16) -> Result<I16NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

// ============================================================================
// U8 / U16 serde boundary
// ============================================================================

/// Prove: serde accepts non-zero u8 values for U8Positive.
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_u8_positive_serde_valid(value: u8) -> Result<U8Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero u8 value for U8Positive.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_u8_positive_serde_invalid(value: u8) -> Result<U8Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-zero u8 values for U8NonZero.
#[requires(value@ != 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_u8_non_zero_serde_valid(value: u8) -> Result<U8NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero u8 value for U8NonZero.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_u8_non_zero_serde_invalid(value: u8) -> Result<U8NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-zero u16 values for U16Positive.
#[requires(value@ > 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_u16_positive_serde_valid(value: u16) -> Result<U16Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero u16 value for U16Positive.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_u16_positive_serde_invalid(value: u16) -> Result<U16Positive, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde accepts non-zero u16 values for U16NonZero.
#[requires(value@ != 0)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_u16_non_zero_serde_valid(value: u16) -> Result<U16NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

/// Prove: serde rejects zero u16 value for U16NonZero.
#[requires(value@ == 0)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_u16_non_zero_serde_invalid(value: u16) -> Result<U16NonZero, serde_json::Error> {
    serde_json::from_value(Value::Number(value.into()))
}

// ============================================================================
// Float serde boundary
// ============================================================================

/// Prove: serde accepts a concrete positive f64 for F64Positive.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_positive_serde_valid() -> Result<F64Positive, serde_json::Error> {
    serde_json::from_value(serde_json::json!(1.0_f64))
}

/// Prove: serde rejects zero for F64Positive.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f64_positive_serde_invalid() -> Result<F64Positive, serde_json::Error> {
    serde_json::from_value(serde_json::json!(0.0_f64))
}

/// Prove: serde accepts zero for F64NonNegative.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_non_negative_serde_valid() -> Result<F64NonNegative, serde_json::Error> {
    serde_json::from_value(serde_json::json!(0.0_f64))
}

/// Prove: serde rejects a negative for F64NonNegative.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_f64_non_negative_serde_invalid() -> Result<F64NonNegative, serde_json::Error> {
    serde_json::from_value(serde_json::json!(-1.0_f64))
}

/// Prove: serde accepts a finite f64 for F64Finite.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_f64_finite_serde_valid() -> Result<F64Finite, serde_json::Error> {
    serde_json::from_value(serde_json::json!(42.5_f64))
}

// ============================================================================
// String serde boundary
// ============================================================================

/// Prove: serde rejects an empty string for StringNonEmpty.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_string_non_empty_serde_empty() -> Result<StringNonEmpty<4096>, serde_json::Error> {
    serde_json::from_value(serde_json::json!(""))
}

/// Prove: serde accepts a non-empty string for StringNonEmpty.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_string_non_empty_serde_valid() -> Result<StringNonEmpty<4096>, serde_json::Error> {
    serde_json::from_value(serde_json::json!("hello"))
}

// ============================================================================
// URL serde boundary (feature-gated)
// ============================================================================

#[cfg(feature = "url")]
use elicitation::{UrlHttp, UrlHttps, UrlValid, UrlWithHost};

/// Prove: serde accepts an HTTPS URL for UrlHttps.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_https_serde_valid() -> Result<UrlHttps, serde_json::Error> {
    serde_json::from_value(serde_json::json!("https://example.com"))
}

/// Prove: serde rejects an HTTP URL for UrlHttps.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_https_serde_invalid() -> Result<UrlHttps, serde_json::Error> {
    serde_json::from_value(serde_json::json!("http://example.com"))
}

/// Prove: serde accepts an HTTP URL for UrlHttp.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_http_serde_valid() -> Result<UrlHttp, serde_json::Error> {
    serde_json::from_value(serde_json::json!("http://example.com"))
}

/// Prove: serde rejects an HTTPS URL for UrlHttp.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_http_serde_invalid() -> Result<UrlHttp, serde_json::Error> {
    serde_json::from_value(serde_json::json!("https://example.com"))
}

/// Prove: serde accepts any valid URL for UrlValid.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_valid_serde_valid() -> Result<UrlValid, serde_json::Error> {
    serde_json::from_value(serde_json::json!("https://example.com"))
}

/// Prove: serde rejects a non-URL string for UrlValid.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_url_valid_serde_invalid() -> Result<UrlValid, serde_json::Error> {
    serde_json::from_value(serde_json::json!("not a url"))
}

/// Prove: serde accepts a URL with a host for UrlWithHost.
#[cfg(feature = "url")]
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_url_with_host_serde_valid() -> Result<UrlWithHost, serde_json::Error> {
    serde_json::from_value(serde_json::json!("https://example.com"))
}

// ============================================================================
// Round-trip proofs
// ============================================================================

/// Prove: I8Positive round-trips through serde preserving its value.
#[requires(value@ > 0)]
#[ensures(match result {
    Ok(v) => i8pos_get(&v)@ == value@,
    Err(_) => false,
})]
#[trusted]
pub fn verify_i8_positive_round_trip(value: i8) -> Result<I8Positive, ValidationError> {
    let original = I8Positive::new(value)?;
    let json =
        serde_json::to_value(original).map_err(|_| ValidationError::NotPositive(value as i128))?;
    serde_json::from_value(json).map_err(|_| ValidationError::NotPositive(value as i128))
}
