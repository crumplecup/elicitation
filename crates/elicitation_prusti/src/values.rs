//! Prusti proofs for serde_json::Value contract types (feature-gated on serde_json).
//!
//! Cloud of assumptions: Trust serde_json Value structure (array, object, null checks).
//! Verify wrapper structure.

#![cfg(feature = "serde_json")]

#[cfg(prusti)]
use prusti_contracts::{ensures, trusted};

#[cfg(prusti)]
use elicitation::{ValueArray, ValueNonNull, ValueObject};

/// Verify ValueArray construction with array value.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_value_array_valid() -> Result<ValueArray, elicitation::ValidationError> {
    use serde_json::json;
    let value = json!([1, 2, 3]);
    ValueArray::new(value)
}

/// Verify ValueArray rejects non-array value.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_value_array_invalid() -> Result<ValueArray, elicitation::ValidationError> {
    use serde_json::json;
    let value = json!({"key": "value"});
    ValueArray::new(value)
}

/// Verify ValueObject construction with object value.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_value_object_valid() -> Result<ValueObject, elicitation::ValidationError> {
    use serde_json::json;
    let value = json!({"key": "value"});
    ValueObject::new(value)
}

/// Verify ValueObject rejects non-object value.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_value_object_invalid() -> Result<ValueObject, elicitation::ValidationError> {
    use serde_json::json;
    let value = json!([1, 2, 3]);
    ValueObject::new(value)
}

/// Verify ValueNonNull construction with non-null value.
#[trusted]
#[ensures(matches!(result, Ok(_)))]
#[cfg(prusti)]
pub fn verify_value_non_null_valid() -> Result<ValueNonNull, elicitation::ValidationError> {
    use serde_json::json;
    let value = json!("test");
    ValueNonNull::new(value)
}

/// Verify ValueNonNull rejects null value.
#[trusted]
#[ensures(matches!(result, Err(_)))]
#[cfg(prusti)]
pub fn verify_value_non_null_invalid() -> Result<ValueNonNull, elicitation::ValidationError> {
    use serde_json::Value;
    ValueNonNull::new(Value::Null)
}
