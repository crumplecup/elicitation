//! Generic wrapper type for MCP tool outputs.
//!
//! MCP specification requires tool output schemas to have `"type": "object"`.
//! Enum types generate `"enum": [...]` schemas without a type field, causing
//! validation failures. This wrapper ensures ALL types produce object schemas.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Wrapper that ensures MCP-compliant object schemas for any type.
///
/// # Purpose
///
/// The MCP specification mandates that tool output schemas must be objects
/// with `"type": "object"` at the root level. However, certain Rust types
/// generate schemas that don't meet this requirement:
///
/// - **Enums**: Generate `{"enum": ["Variant1", "Variant2"]}` (no type field)
/// - **Primitives**: Generate `{"type": "string"}` or `{"type": "integer"}` (not objects)
/// - **Tuples**: Generate `{"type": "array"}` (not objects)
///
/// This wrapper solves the problem by wrapping ANY type in a struct with a
/// single field, which always generates an object schema:
///
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "value": { /* inner type schema */ }
///   },
///   "required": ["value"]
/// }
/// ```
///
/// # Usage
///
/// This type is used automatically by the `#[elicit_tools]` macro.
/// You typically don't construct it manually:
///
/// ```rust,ignore
/// #[elicit_tools(MyEnum, MyStruct)]
/// impl MyServer {
///     // Generated methods return ElicitToolOutput<T>
/// }
/// ```
///
/// # Example
///
/// ```rust
/// use elicitation::ElicitToolOutput;
/// use schemars::schema_for;
///
/// #[derive(serde::Serialize, schemars::JsonSchema)]
/// enum Priority { Low, Medium, High }
///
/// let wrapped = ElicitToolOutput { value: Priority::High };
///
/// // Schema is now MCP-compliant object
/// let schema = schema_for!(ElicitToolOutput<Priority>);
/// assert!(schema.as_object().is_some());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ElicitToolOutput<T> {
    /// The elicited value
    pub value: T,
}

impl<T> ElicitToolOutput<T> {
    /// Create a new wrapper around a value.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Extract the wrapped value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> From<T> for ElicitToolOutput<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> AsRef<T> for ElicitToolOutput<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> AsMut<T> for ElicitToolOutput<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
    enum TestEnum {
        A,
        B,
        C,
    }

    #[test]
    fn test_wrapper_construction() {
        let wrapped = ElicitToolOutput::new(TestEnum::A);
        assert_eq!(wrapped.value, TestEnum::A);
    }

    #[test]
    fn test_wrapper_into_inner() {
        let wrapped = ElicitToolOutput::new(42);
        assert_eq!(wrapped.into_inner(), 42);
    }

    #[test]
    fn test_wrapper_from() {
        let wrapped: ElicitToolOutput<i32> = 42.into();
        assert_eq!(wrapped.value, 42);
    }

    #[test]
    fn test_wrapper_as_ref() {
        let wrapped = ElicitToolOutput::new(TestEnum::B);
        assert_eq!(wrapped.as_ref(), &TestEnum::B);
    }

    #[test]
    fn test_wrapper_serialization() {
        let wrapped = ElicitToolOutput::new(TestEnum::C);
        let json = serde_json::to_value(&wrapped).expect("Serialize");

        assert!(json.is_object());
        assert!(json.get("value").is_some());
    }

    #[test]
    fn test_wrapper_schema_is_object() {
        use schemars::schema_for;

        let schema = schema_for!(ElicitToolOutput<TestEnum>);

        // Must be an object schema (not enum/boolean/etc)
        assert!(
            schema.as_object().is_some(),
            "Wrapper schema must be object type"
        );

        // Verify it has a "value" property
        let obj = schema.as_object().unwrap();
        assert!(obj.get("properties").is_some());
    }
}
