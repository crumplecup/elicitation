//! Tests for enum derive macro.

use elicitation::{DeriveElicit, Prompt, Select};

#[derive(Debug, Clone, Copy, PartialEq, DeriveElicit)]
enum SimpleEnum {
    First,
    Second,
    Third,
}

#[derive(Debug, Clone, Copy, PartialEq, DeriveElicit)]
#[prompt("Choose your favorite color:")]
enum ColorEnum {
    Red,
    Green,
    Blue,
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
