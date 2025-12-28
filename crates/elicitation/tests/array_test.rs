//! Tests for fixed-size array [T; N] implementation.

use elicitation::{Elicitation, Prompt};

#[test]
fn test_array_has_prompt() {
    type Array3 = [i32; 3];
    assert!(Array3::prompt().is_some());
}

#[test]
fn test_array_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<[i32; 3]>();
    requires_elicitation::<[String; 5]>();
}

#[test]
fn test_array_construction() {
    let array: [i32; 3] = [1, 2, 3];
    assert_eq!(array.len(), 3);
    assert_eq!(array[0], 1);
    assert_eq!(array[2], 3);
}

#[test]
fn test_array_from_vec() {
    let vec = vec![1, 2, 3, 4, 5];
    let array: [i32; 5] = vec.try_into().expect("Wrong size");
    assert_eq!(array.len(), 5);
}

#[test]
fn test_array_various_sizes() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<[i32; 1]>();
    requires_elicitation::<[i32; 2]>();
    requires_elicitation::<[i32; 10]>();
    requires_elicitation::<[i32; 32]>();
}

#[test]
fn test_array_complex_types() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<[String; 3]>();
    requires_elicitation::<[Option<i32>; 5]>();
    requires_elicitation::<[(i32, String); 2]>();
}
