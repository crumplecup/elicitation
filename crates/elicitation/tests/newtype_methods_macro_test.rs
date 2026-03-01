//! Tests for the elicit_newtype_methods! macro.

use elicitation::elicit_newtype_methods;

// Simple test with String wrapper - only non-mutating methods for now
elicit_newtype_methods! {
    StringClient => String,
    fn len() -> usize;
    fn is_empty() -> bool;
    fn to_uppercase() -> String;
}

#[test]
fn test_delegating_methods() {
    let client = StringClient::from("hello".to_string());

    // Test len method
    assert_eq!(client.len(), 5);

    // Test is_empty method
    assert!(!client.is_empty());

    // Test to_uppercase method
    assert_eq!(client.to_uppercase(), "HELLO");
}

#[test]
fn test_tool_wrapper_methods() {
    use rmcp::handler::server::wrapper::Json;

    let client = StringClient::from("hello".to_string());

    // Test len_tool (no params)
    let result = client.len_tool();
    assert!(result.is_ok());
    let Json(len) = result.unwrap();
    assert_eq!(len, 5);

    // Test is_empty_tool (no params)
    let result = client.is_empty_tool();
    assert!(result.is_ok());
    let Json(empty) = result.unwrap();
    assert!(!empty);

    // Test to_uppercase_tool (no params)
    let result = client.to_uppercase_tool();
    assert!(result.is_ok());
    let Json(upper) = result.unwrap();
    assert_eq!(upper, "HELLO");
}

// Test with method that has parameters
elicit_newtype_methods! {
    Calculator => i32,
    fn saturating_add(rhs: i32) -> i32;
    fn saturating_sub(rhs: i32) -> i32;
}

#[test]
fn test_methods_with_params() {
    let calc = Calculator::from(10);

    assert_eq!(calc.saturating_add(5), 15);
    assert_eq!(calc.saturating_sub(3), 7);
}

#[test]
fn test_param_struct_generated() {
    // Verify SaturatingAddParams was generated
    let params = SaturatingAddParams { rhs: 5 };
    assert_eq!(params.rhs, 5);
}

#[test]
fn test_tool_wrappers_with_params() {
    use rmcp::handler::server::wrapper::{Json, Parameters};

    let calc = Calculator::from(10);

    // Test saturating_add_tool
    let params = Parameters(SaturatingAddParams { rhs: 5 });
    let result = calc.saturating_add_tool(params);
    assert!(result.is_ok());
    let Json(sum) = result.unwrap();
    assert_eq!(sum, 15);

    // Test saturating_sub_tool
    let params = Parameters(SaturatingSubParams { rhs: 3 });
    let result = calc.saturating_sub_tool(params);
    assert!(result.is_ok());
    let Json(diff) = result.unwrap();
    assert_eq!(diff, 7);
}
