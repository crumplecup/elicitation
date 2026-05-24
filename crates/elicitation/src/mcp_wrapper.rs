//! Generic wrapper type for MCP tool outputs.
//!
//! MCP specification requires tool output schemas to have `"type": "object"`.
//! Enum types generate `"enum": [...]` schemas without a type field, causing
//! validation failures. This wrapper ensures ALL types produce object schemas.

use proc_macro2::TokenStream;
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

impl<T> crate::Prompt for ElicitToolOutput<T> {
    fn prompt() -> Option<&'static str> {
        Some("Elicit the wrapped value:")
    }
}

/// Default-only style for [`ElicitToolOutput`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ElicitToolOutputStyle;

impl crate::Prompt for ElicitToolOutputStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}
impl crate::Elicitation for ElicitToolOutputStyle {
    type Style = ElicitToolOutputStyle;
    async fn elicit<C: crate::ElicitCommunicator>(_: &C) -> crate::ElicitResult<Self> {
        Ok(Self)
    }
    fn kani_proof() -> TokenStream {
        TokenStream::new()
    }
    fn verus_proof() -> TokenStream {
        TokenStream::new()
    }
    fn creusot_proof() -> TokenStream {
        TokenStream::new()
    }
}
impl crate::style::ElicitationStyle for ElicitToolOutputStyle {}
impl crate::ElicitPromptTree for ElicitToolOutputStyle {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Leaf {
            prompt: "default".to_string(),
            type_name: "ElicitToolOutputStyle".to_string(),
        }
    }
}

impl<T> crate::Elicitation for ElicitToolOutput<T>
where
    T: crate::Elicitation + Send,
{
    type Style = ElicitToolOutputStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: crate::ElicitCommunicator>(communicator: &C) -> crate::ElicitResult<Self> {
        tracing::debug!("Eliciting ElicitToolOutput<T>");
        let value = T::elicit(communicator).await?;
        Ok(Self { value })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        T::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        T::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        T::creusot_proof()
    }
}

impl<T: crate::ElicitIntrospect + Send> crate::ElicitIntrospect for ElicitToolOutput<T> {
    fn pattern() -> crate::ElicitationPattern {
        T::pattern()
    }

    fn metadata() -> crate::TypeMetadata {
        T::metadata()
    }
}

impl<T: crate::ElicitPromptTree> crate::ElicitPromptTree for ElicitToolOutput<T> {
    fn prompt_tree() -> crate::PromptTree {
        T::prompt_tree()
    }
}

impl<T: crate::emit_code::ToCodeLiteral> crate::emit_code::ToCodeLiteral for ElicitToolOutput<T> {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let inner = self.value.to_code_literal();
        quote::quote! { elicitation::ElicitToolOutput { value: #inner } }
    }
}

impl<T: crate::ElicitSpec + 'static> crate::ElicitSpec for ElicitToolOutput<T> {
    fn type_spec() -> crate::TypeSpec {
        T::type_spec()
    }
}

impl<T> crate::ElicitComplete for ElicitToolOutput<T> where T: crate::ElicitComplete + Send {}

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
