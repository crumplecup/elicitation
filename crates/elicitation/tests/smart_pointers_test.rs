//! Tests for smart pointer implementations.

use elicitation::{Elicitation, Prompt};
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn test_box_has_prompt() {
    type BoxedString = Box<String>;
    // Box should use inner type's prompt
    assert_eq!(BoxedString::prompt(), String::prompt());
}

#[test]
fn test_rc_has_prompt() {
    type RcString = Rc<String>;
    assert_eq!(RcString::prompt(), String::prompt());
}

#[test]
fn test_arc_has_prompt() {
    type ArcString = Arc<String>;
    assert_eq!(ArcString::prompt(), String::prompt());
}

#[test]
fn test_box_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Box<String>>();
    requires_elicitation::<Box<i32>>();
}

#[test]
fn test_rc_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Rc<String>>();
    requires_elicitation::<Rc<i32>>();
}

#[test]
fn test_arc_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Arc<String>>();
    requires_elicitation::<Arc<i32>>();
}

#[test]
fn test_box_construction() {
    let boxed = Box::new(42);
    assert_eq!(*boxed, 42);
}

#[test]
fn test_rc_construction() {
    let rc = Rc::new(String::from("hello"));
    assert_eq!(*rc, "hello");
}

#[test]
fn test_arc_construction() {
    let arc = Arc::new(vec![1, 2, 3]);
    assert_eq!(*arc, vec![1, 2, 3]);
}

#[test]
fn test_smart_pointer_with_complex_types() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<Box<Vec<i32>>>();
    requires_elicitation::<Rc<Option<String>>>();
    requires_elicitation::<Arc<(i32, String)>>();
}
