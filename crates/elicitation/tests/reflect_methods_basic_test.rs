//! Basic integration test for #[reflect_methods] proc macro (non-generic).

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

// Create newtype wrapper
elicit_newtype!(String, as MyString);

// Test non-generic methods
#[reflect_methods]
impl MyString {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[test]
fn test_non_generic_method_compiles() {
    // Just verify that the code compiles
    // The #[reflect_methods] macro should have generated:
    // - len_tool() method (no params)
    // - is_empty_tool() method (no params)
}

#[test]
fn test_non_generic_method_delegation() {
    let s = MyString("hello".to_string());

    // Test methods delegate to inner String
    assert_eq!(s.len(), 5);
    assert!(!s.is_empty());
}
