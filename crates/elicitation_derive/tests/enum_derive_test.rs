//! Tests for enum derive macro.

use elicitation::{Elicit, Elicitation, Prompt, Select};

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
enum SimpleEnum {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
#[prompt("Choose your favorite color:")]
enum ColorEnum {
    Red,
    Green,
    Blue,
}

// Test tuple variants
#[derive(Debug, Clone, PartialEq, Elicit)]
enum SimpleTuple {
    Value(String),
}

#[derive(Debug, Clone, PartialEq, Elicit)]
enum MultiTuple {
    Pair(String, i32),
    Triple(String, i32, bool),
}

// Test struct variants
#[derive(Debug, Clone, PartialEq, Elicit)]
enum StructVariant {
    Config { host: String, port: u16 },
}

#[derive(Debug, Clone, PartialEq, Elicit)]
enum MultiStruct {
    ServerConfig { host: String, port: u16 },
    ClientConfig { url: String, timeout: u32 },
}

// Test mixed variants
#[derive(Debug, Clone, PartialEq, Elicit)]
enum Mixed {
    Unit,
    Tuple(String),
    Struct { value: i32 },
}

// Test nested enums
#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
enum Inner {
    A,
    B,
}

#[derive(Debug, Clone, PartialEq, Elicit)]
enum Outer {
    Contains(Inner),
    StructContains { inner: Inner },
    JustUnit,
}

#[test]
fn test_simple_enum_has_prompt() {
    let prompt = SimpleEnum::prompt();
    assert!(prompt.is_some());
    assert!(prompt.unwrap().contains("SimpleEnum"));
}

#[test]
fn test_custom_prompt() {
    let prompt = ColorEnum::prompt();
    assert_eq!(prompt, Some("Choose your favorite color:"));
}

#[test]
fn test_select_options() {
    let options = SimpleEnum::options();
    assert_eq!(options.len(), 3);
    assert_eq!(options[0], SimpleEnum::First);
    assert_eq!(options[1], SimpleEnum::Second);
    assert_eq!(options[2], SimpleEnum::Third);
}

#[test]
fn test_select_labels() {
    let labels = SimpleEnum::labels();
    assert_eq!(labels.len(), 3);
    assert_eq!(labels[0], "First");
    assert_eq!(labels[1], "Second");
    assert_eq!(labels[2], "Third");
}

#[test]
fn test_from_label_valid() {
    assert_eq!(SimpleEnum::from_label("First"), Some(SimpleEnum::First));
    assert_eq!(SimpleEnum::from_label("Second"), Some(SimpleEnum::Second));
    assert_eq!(SimpleEnum::from_label("Third"), Some(SimpleEnum::Third));
}

#[test]
fn test_from_label_invalid() {
    assert_eq!(SimpleEnum::from_label("Invalid"), None);
    assert_eq!(SimpleEnum::from_label("first"), None); // Case sensitive
    assert_eq!(SimpleEnum::from_label(""), None);
}

#[test]
fn test_color_enum_select() {
    let labels = ColorEnum::labels();
    assert_eq!(labels, &["Red", "Green", "Blue"]);

    assert_eq!(ColorEnum::from_label("Red"), Some(ColorEnum::Red));
    assert_eq!(ColorEnum::from_label("Green"), Some(ColorEnum::Green));
    assert_eq!(ColorEnum::from_label("Blue"), Some(ColorEnum::Blue));
}

// Compile-time test: verify trait bounds
#[test]
fn test_trait_bounds() {
    fn requires_select<T: Select>() {}
    fn requires_prompt<T: Prompt>() {}

    requires_select::<SimpleEnum>();
    requires_prompt::<SimpleEnum>();
    requires_select::<ColorEnum>();
    requires_prompt::<ColorEnum>();
}

// Tuple variant tests
#[test]
fn test_simple_tuple_compiles() {
    fn requires_elicit<T: Elicitation>() {}
    requires_elicit::<SimpleTuple>();
}

#[test]
fn test_simple_tuple_labels() {
    let labels = SimpleTuple::labels();
    assert_eq!(labels, &["Value"]);
}

#[test]
fn test_multi_tuple_compiles() {
    fn requires_elicit<T: Elicitation>() {}
    requires_elicit::<MultiTuple>();
}

#[test]
fn test_multi_tuple_labels() {
    let labels = MultiTuple::labels();
    assert_eq!(labels.len(), 2);
    assert!(labels.contains(&"Pair"));
    assert!(labels.contains(&"Triple"));
}

// Struct variant tests
#[test]
fn test_struct_variant_compiles() {
    fn requires_select<T: Select>() {}
    fn requires_elicit<T: Elicitation>() {}
    requires_select::<StructVariant>();
    requires_elicit::<StructVariant>();
}

#[test]
fn test_struct_variant_labels() {
    let labels = StructVariant::labels();
    assert_eq!(labels, &["Config"]);
}

#[test]
fn test_multi_struct_labels() {
    let labels = MultiStruct::labels();
    assert_eq!(labels.len(), 2);
    assert!(labels.contains(&"ServerConfig"));
    assert!(labels.contains(&"ClientConfig"));
}

// Mixed variant tests
#[test]
fn test_mixed_variants() {
    let labels = Mixed::labels();
    assert_eq!(labels.len(), 3);
    assert!(labels.contains(&"Unit"));
    assert!(labels.contains(&"Tuple"));
    assert!(labels.contains(&"Struct"));
}

#[test]
fn test_mixed_from_label() {
    // Only unit variants work with from_label
    assert_eq!(Mixed::from_label("Unit"), Some(Mixed::Unit));
    assert_eq!(Mixed::from_label("Tuple"), None);
    assert_eq!(Mixed::from_label("Struct"), None);
}

// Nested enum tests
#[test]
fn test_nested_enum_compiles() {
    fn requires_elicit<T: Elicitation>() {}
    requires_elicit::<Outer>();
    requires_elicit::<Inner>();
}

#[test]
fn test_nested_enum_labels() {
    let labels = Outer::labels();
    assert_eq!(labels.len(), 3);
    assert!(labels.contains(&"Contains"));
    assert!(labels.contains(&"StructContains"));
    assert!(labels.contains(&"JustUnit"));
}

#[test]
fn test_nested_inner_enum() {
    let labels = Inner::labels();
    assert_eq!(labels, &["A", "B"]);

    assert_eq!(Inner::from_label("A"), Some(Inner::A));
    assert_eq!(Inner::from_label("B"), Some(Inner::B));
}
