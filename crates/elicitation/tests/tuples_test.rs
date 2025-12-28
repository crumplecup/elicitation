//! Tests for tuple type implementations.

use elicitation::{Elicitation, Prompt};

#[test]
fn test_tuple2_has_prompt() {
    type Tuple2 = (String, i32);
    assert!(Tuple2::prompt().is_some());
}

#[test]
fn test_tuple3_has_prompt() {
    type Tuple3 = (String, i32, bool);
    assert!(Tuple3::prompt().is_some());
}

#[test]
fn test_tuple2_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<(String, i32)>();
}

#[test]
fn test_tuple3_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<(String, i32, bool)>();
}

#[test]
fn test_tuple4_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<(String, i32, bool, f64)>();
}

#[test]
fn test_tuple_large_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    // Test a larger tuple (arity 8)
    requires_elicitation::<(i32, i32, i32, i32, i32, i32, i32, i32)>();
}

#[test]
fn test_tuple_construction() {
    let tuple2 = (String::from("hello"), 42);
    assert_eq!(tuple2.0, "hello");
    assert_eq!(tuple2.1, 42);

    let tuple3 = (String::from("world"), 100, true);
    assert_eq!(tuple3.0, "world");
    assert_eq!(tuple3.1, 100);
    assert_eq!(tuple3.2, true);
}

#[test]
fn test_tuple_with_complex_types() {
    // Test tuples with Option and Vec
    type ComplexTuple = (Vec<i32>, Option<String>, bool);
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<ComplexTuple>();
}
