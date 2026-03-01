//! Tests for the elicit_newtype_methods! macro.
//!
//! This test file demonstrates non-generic method support in the declarative macro.
//!
//! **Generic Method Support:**
//! Generic methods are NOT supported in the `elicit_newtype_methods!` declarative macro
//! due to fundamental parsing limitations in `macro_rules!`. For generic method support,
//! use the `#[reflect_methods]` proc macro from `elicitation_derive`, which has full
//! AST access via `syn`.

use elicitation::elicit_newtype_methods;

// Non-generic method test (fully working)
elicit_newtype_methods! {
    StringWrapper => String,
    fn len() -> usize;
    fn is_empty() -> bool;
}

#[test]
fn test_non_generic_methods() {
    let wrapper = StringWrapper("hello".to_string());
    assert_eq!(wrapper.len(), 5);
    assert!(!wrapper.is_empty());
}

#[test]
fn test_non_generic_tool_wrapper() {
    use rmcp::handler::server::wrapper::Json;

    let wrapper = StringWrapper("test".to_string());
    let result = wrapper.len_tool();
    assert!(result.is_ok());
    let Json(len) = result.unwrap();
    assert_eq!(len, 4);
}
