//! serde_json::Value contract types.
//!
//! Available with the `serde_json` feature.

#[cfg(feature = "serde_json")]
use super::ValidationError;
#[cfg(all(feature = "serde_json", not(kani)))]
use crate::{ElicitClient, ElicitResult, Elicitation, Prompt};
#[cfg(all(feature = "serde_json", not(kani)))]
use elicitation_macros::instrumented_impl;
#[cfg(feature = "serde_json")]
use serde_json::Value;

// ValueObject - JSON Value that is guaranteed to be an object
/// A serde_json::Value that is guaranteed to be an object (not null, bool, string, number, or array).
///
/// Available with the `serde_json` feature.
#[cfg(feature = "serde_json")]
#[derive(Debug, Clone, PartialEq)]
#[cfg(not(kani))]
pub struct ValueObject(Value);

#[cfg(all(feature = "serde_json", kani))]
#[derive(Debug, Clone, PartialEq)]
pub struct ValueObject(std::marker::PhantomData<()>);

#[cfg(feature = "serde_json")]
#[cfg_attr(not(kani), instrumented_impl)]
impl ValueObject {
    /// Create a new ValueObject, validating it's an object.
    #[cfg(not(kani))]
    pub fn new(value: Value) -> Result<Self, ValidationError> {
        if value.is_object() {
            Ok(Self(value))
        } else {
            Err(ValidationError::WrongJsonType {
                expected: "object".to_string(),
                got: value_type_name(&value),
            })
        }
    }

    /// Kani version: trust serde_json, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: Value) -> Result<Self, ValidationError> {
        let is_object: bool = kani::any();
        if is_object {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::WrongJsonType {
                expected: "object".to_string(),
                got: "other".to_string(),
            })
        }
    }

    /// Get the inner Value.
    #[cfg(not(kani))]
    pub fn get(&self) -> &Value {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &Value {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner Value.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> Value {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> Value {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for ValueObject {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a JSON object:")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for ValueObject {
    type Style = <Value as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueObject");
        loop {
            let value = Value::elicit(client).await?;
            match Self::new(value) {
                Ok(valid) => {
                    tracing::debug!("Valid JSON object");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Value not an object, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ValueArray - JSON Value that is guaranteed to be an array
/// A serde_json::Value that is guaranteed to be an array.
///
/// Available with the `serde_json` feature.
#[cfg(feature = "serde_json")]
#[derive(Debug, Clone, PartialEq)]
#[cfg(not(kani))]
pub struct ValueArray(Value);

#[cfg(all(feature = "serde_json", kani))]
#[derive(Debug, Clone, PartialEq)]
pub struct ValueArray(std::marker::PhantomData<()>);

#[cfg(feature = "serde_json")]
#[cfg_attr(not(kani), instrumented_impl)]
impl ValueArray {
    /// Create a new ValueArray, validating it's an array.
    #[cfg(not(kani))]
    pub fn new(value: Value) -> Result<Self, ValidationError> {
        if value.is_array() {
            Ok(Self(value))
        } else {
            Err(ValidationError::WrongJsonType {
                expected: "array".to_string(),
                got: value_type_name(&value),
            })
        }
    }

    /// Kani version: trust serde_json, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: Value) -> Result<Self, ValidationError> {
        let is_array: bool = kani::any();
        if is_array {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::WrongJsonType {
                expected: "array".to_string(),
                got: "other".to_string(),
            })
        }
    }

    /// Get the inner Value.
    #[cfg(not(kani))]
    pub fn get(&self) -> &Value {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &Value {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner Value.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> Value {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> Value {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for ValueArray {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a JSON array:")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for ValueArray {
    type Style = <Value as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueArray");
        loop {
            let value = Value::elicit(client).await?;
            match Self::new(value) {
                Ok(valid) => {
                    tracing::debug!("Valid JSON array");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Value not an array, re-prompting");
                    continue;
                }
            }
        }
    }
}

// ValueNonNull - JSON Value that is guaranteed to not be null
/// A serde_json::Value that is guaranteed to not be null.
///
/// Available with the `serde_json` feature.
#[cfg(feature = "serde_json")]
#[derive(Debug, Clone, PartialEq)]
#[cfg(not(kani))]
pub struct ValueNonNull(Value);

#[cfg(all(feature = "serde_json", kani))]
#[derive(Debug, Clone, PartialEq)]
pub struct ValueNonNull(std::marker::PhantomData<()>);

#[cfg(feature = "serde_json")]
#[cfg_attr(not(kani), instrumented_impl)]
impl ValueNonNull {
    /// Create a new ValueNonNull, validating it's not null.
    #[cfg(not(kani))]
    pub fn new(value: Value) -> Result<Self, ValidationError> {
        if !value.is_null() {
            Ok(Self(value))
        } else {
            Err(ValidationError::JsonIsNull)
        }
    }

    /// Kani version: trust serde_json, verify wrapper logic.
    #[cfg(kani)]
    pub fn new(_value: Value) -> Result<Self, ValidationError> {
        let is_null: bool = kani::any();
        if !is_null {
            Ok(Self(std::marker::PhantomData))
        } else {
            Err(ValidationError::JsonIsNull)
        }
    }

    /// Get the inner Value.
    #[cfg(not(kani))]
    pub fn get(&self) -> &Value {
        &self.0
    }

    #[cfg(kani)]
    pub fn get(&self) -> &Value {
        panic!("get() not supported in Kani verification")
    }

    /// Unwrap into the inner Value.
    #[cfg(not(kani))]
    pub fn into_inner(self) -> Value {
        self.0
    }

    #[cfg(kani)]
    pub fn into_inner(self) -> Value {
        panic!("into_inner() not supported in Kani verification")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Prompt for ValueNonNull {
    fn prompt() -> Option<&'static str> {
        Some("Please provide a non-null JSON value:")
    }
}

#[cfg(feature = "serde_json")]
#[cfg(not(kani))]
#[cfg_attr(not(kani), instrumented_impl)]
impl Elicitation for ValueNonNull {
    type Style = <Value as Elicitation>::Style;

    #[tracing::instrument(skip(client))]
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueNonNull");
        loop {
            let value = Value::elicit(client).await?;
            match Self::new(value) {
                Ok(valid) => {
                    tracing::debug!("Valid non-null JSON value");
                    return Ok(valid);
                }
                Err(e) => {
                    tracing::warn!(error = %e, "Value is null, re-prompting");
                    continue;
                }
            }
        }
    }
}

#[cfg(feature = "serde_json")]
fn value_type_name(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(_) => "boolean".to_string(),
        Value::Number(_) => "number".to_string(),
        Value::String(_) => "string".to_string(),
        Value::Array(_) => "array".to_string(),
        Value::Object(_) => "object".to_string(),
    }
}

#[cfg(all(test, feature = "serde_json"))]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_value_object_valid() {
        let value = json!({"key": "value"});
        let result = ValueObject::new(value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_value_object_array() {
        let value = json!([1, 2, 3]);
        let result = ValueObject::new(value);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_array_valid() {
        let value = json!([1, 2, 3]);
        let result = ValueArray::new(value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_value_array_object() {
        let value = json!({"key": "value"});
        let result = ValueArray::new(value);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_non_null_valid() {
        let value = json!(42);
        let result = ValueNonNull::new(value);
        assert!(result.is_ok());
    }

    #[test]
    fn test_value_non_null_null() {
        let value = Value::Null;
        let result = ValueNonNull::new(value);
        assert!(result.is_err());
    }
}
