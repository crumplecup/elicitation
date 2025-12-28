//! Tests for struct derive macro.

use elicitation::{Elicit, FieldInfo, Prompt, Select, Survey};

#[derive(Debug, Clone, Copy, PartialEq, Elicit)]
enum Status {
    Active,
    Inactive,
}

#[derive(Debug, Elicit)]
struct SimpleStruct {
    name: String,
    age: u8,
}

#[derive(Debug, Elicit)]
#[prompt("Let's configure your settings:")]
struct ConfigStruct {
    timeout: u32,
    retries: i32,
}

#[derive(Debug, Elicit)]
struct FieldPromptStruct {
    #[prompt("What is your username?")]
    username: String,
    #[prompt("Enter your age:")]
    age: u8,
}

#[derive(Debug)]
struct SkipFieldStruct {
    name: String,
    #[allow(dead_code)]
    internal_id: u64,
}

// Manual impl to test #[skip] behavior would work
// For now, we test it via the fields() check
#[derive(Debug, Elicit)]
struct PartialSkipStruct {
    name: String,
    age: u8,
    #[skip]
    _internal: String,
}

#[derive(Debug, Elicit)]
struct NestedStruct {
    name: String,
    status: Status,
    count: Option<i32>,
}

#[test]
fn test_simple_struct_has_prompt() {
    let prompt = SimpleStruct::prompt();
    assert!(prompt.is_some());
    assert!(prompt.unwrap().contains("SimpleStruct"));
}

#[test]
fn test_custom_struct_prompt() {
    let prompt = ConfigStruct::prompt();
    assert_eq!(prompt, Some("Let's configure your settings:"));
}

#[test]
fn test_survey_fields_simple() {
    let fields = SimpleStruct::fields();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "name");
    assert!(fields[0].type_name.contains("String"));
    assert_eq!(fields[0].prompt, None);

    assert_eq!(fields[1].name, "age");
    assert!(fields[1].type_name.contains("u8"));
    assert_eq!(fields[1].prompt, None);
}

#[test]
fn test_survey_field_prompts() {
    let fields = FieldPromptStruct::fields();
    assert_eq!(fields.len(), 2);

    assert_eq!(fields[0].name, "username");
    assert_eq!(fields[0].prompt, Some("What is your username?"));

    assert_eq!(fields[1].name, "age");
    assert_eq!(fields[1].prompt, Some("Enter your age:"));
}

#[test]
fn test_skip_attribute() {
    let fields = PartialSkipStruct::fields();
    assert_eq!(fields.len(), 2); // _internal should be skipped

    assert_eq!(fields[0].name, "name");
    assert!(fields[0].type_name.contains("String"));

    assert_eq!(fields[1].name, "age");
    assert!(fields[1].type_name.contains("u8"));
}

#[test]
fn test_nested_struct_fields() {
    let fields = NestedStruct::fields();
    assert_eq!(fields.len(), 3);

    assert_eq!(fields[0].name, "name");
    assert_eq!(fields[1].name, "status");
    assert_eq!(fields[2].name, "count");

    // Verify nested types appear
    assert!(fields[1].type_name.contains("Status"));
    assert!(fields[2].type_name.contains("Option"));
}

// Compile-time test: verify trait bounds
#[test]
fn test_trait_bounds() {
    fn requires_survey<T: Survey>() {}
    fn requires_prompt<T: Prompt>() {}

    requires_survey::<SimpleStruct>();
    requires_prompt::<SimpleStruct>();
    requires_survey::<ConfigStruct>();
    requires_prompt::<ConfigStruct>();
    requires_survey::<NestedStruct>();
    requires_prompt::<NestedStruct>();
}

#[test]
fn test_field_info_construction() {
    let info = FieldInfo {
        name: "test",
        prompt: Some("Test prompt"),
        type_name: "String",
    };

    assert_eq!(info.name, "test");
    assert_eq!(info.prompt, Some("Test prompt"));
    assert_eq!(info.type_name, "String");
}
