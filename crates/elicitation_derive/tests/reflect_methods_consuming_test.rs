//! Tests for consuming methods in #[reflect_methods] proc macro.
//!
//! This test verifies that consuming methods (taking `self` instead of `&self`)
//! work correctly with the hybrid Arc unwrap-or-clone strategy.

use elicitation::elicit_newtype;
use elicitation_derive::reflect_methods;

// Test with a Clone type
#[derive(Debug, Clone, PartialEq)]
pub struct Inner {
    value: String,
}

impl Inner {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn consume_and_get(self) -> String {
        self.value
    }
}

elicit_newtype!(Inner, as Wrapper);

#[reflect_methods]
impl Wrapper {
    pub fn consume_and_get(self) -> String {
        // Hybrid unwrap-or-clone strategy for Arc-based newtypes
        let inner = ::std::sync::Arc::try_unwrap(self.0)
            .unwrap_or_else(|arc| (*arc).clone());
        inner.consume_and_get()
    }
}

#[test]
fn test_consuming_method_single_use() {
    let wrapper = Wrapper::from(Inner::new("hello".to_string()));
    let result = wrapper.consume_and_get();
    assert_eq!(result, "hello");
}

#[test]
fn test_consuming_method_with_clone() {
    let wrapper = Wrapper::from(Inner::new("test".to_string()));
    let wrapper2 = wrapper.clone();  // Arc refcount = 2

    // Both should work (inner Inner is Clone)
    let result1 = wrapper.consume_and_get();   // Clones inner
    let result2 = wrapper2.consume_and_get();  // Clones inner

    assert_eq!(result1, "test");
    assert_eq!(result2, "test");
}

// Test with a non-Clone type (like reqwest::RequestBuilder)
#[derive(Debug)]
pub struct NonCloneInner {
    value: String,
}

impl NonCloneInner {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn append(self, suffix: &str) -> Self {
        Self {
            value: format!("{}{}", self.value, suffix),
        }
    }

    fn finish(self) -> String {
        self.value
    }
}

elicit_newtype!(NonCloneInner, as NonCloneBuilder);

#[reflect_methods]
impl NonCloneBuilder {
    pub fn append(self, suffix: &str) -> Self {
        // Hybrid unwrap-or-clone strategy
        // For NonCloneInner, this will panic if Arc refcount > 1 (correct behavior!)
        let inner = ::std::sync::Arc::try_unwrap(self.0)
            .expect("Cannot append with shared references to non-Clone type");
        Self::from(inner.append(suffix))
    }

    pub fn finish(self) -> String {
        // Hybrid unwrap-or-clone strategy
        let inner = ::std::sync::Arc::try_unwrap(self.0)
            .expect("Cannot finish with shared references to non-Clone type");
        inner.finish()
    }
}

#[test]
fn test_non_clone_consuming() {
    let builder = NonCloneBuilder::from(NonCloneInner::new("hello".to_string()));
    let result = builder
        .append(" ")
        .append("world")
        .finish();

    assert_eq!(result, "hello world");
}

// This should fail to compile if uncommented (NonCloneInner: !Clone)
// #[test]
// fn test_non_clone_with_clone_fails() {
//     let builder = NonCloneBuilder::from(NonCloneInner::new("test".to_string()));
//     let builder2 = builder.clone();  // Arc refcount = 2
//     let result = builder.finish();   // Would try to clone NonCloneInner - compile error!
// }
