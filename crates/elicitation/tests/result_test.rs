//! Tests for Result<T, E> implementation.

use elicitation::{Elicitation, Prompt};

#[test]
fn test_result_has_prompt() {
    type TestResult = Result<String, i32>;
    assert!(TestResult::prompt().is_some());
}

#[test]
fn test_result_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Result<String, i32>>();
    requires_elicitation::<Result<i32, String>>();
}

#[test]
fn test_result_ok_variant() {
    let result: Result<i32, String> = Ok(42);
    assert!(result.is_ok());
    if let Ok(value) = result {
        assert_eq!(value, 42);
    }
}

#[test]
fn test_result_err_variant() {
    let result: Result<i32, String> = Err("error".to_string());
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err, "error");
    }
}

#[test]
fn test_result_operations() {
    let ok_result: Result<i32, String> = Ok(10);
    let mapped = ok_result.map(|x| x * 2);
    assert_eq!(mapped, Ok(20));

    let err_result: Result<i32, String> = Err("fail".to_string());
    let mapped = err_result.map(|x| x * 2);
    assert!(mapped.is_err());
}

#[test]
fn test_result_with_complex_types() {
    // Test with complex Ok type
    type ComplexResult = Result<Vec<String>, i32>;
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<ComplexResult>();
}
