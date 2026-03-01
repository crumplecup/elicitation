//! Tests for the elicit_newtype! and elicit_newtypes! macros.

use elicitation::{elicit_newtype, elicit_newtypes};

// Simple wrapper test
elicit_newtype!(String, as StringWrapper);

#[test]
fn test_simple_wrapper() {
    let s = String::from("hello");
    let wrapper = StringWrapper::from(s.clone());

    // Test Deref
    assert_eq!(&*wrapper, "hello");
    assert_eq!((*wrapper).len(), 5);

    // Test unwrap
    let unwrapped: String = wrapper.into();
    assert_eq!(unwrapped, s);
}

#[test]
fn test_deref_mut() {
    let s = String::from("hello");
    let mut wrapper = StringWrapper::from(s);

    // Test DerefMut
    wrapper.push_str(" world");
    assert_eq!(&*wrapper, "hello world");
}

#[test]
fn test_as_ref() {
    let s = String::from("test");
    let wrapper = StringWrapper::from(s);

    // Test AsRef
    let s_ref: &String = wrapper.as_ref();
    assert_eq!(s_ref, "test");
}

// Test with stdlib collection
elicit_newtype!(std::collections::HashMap<String, i32>, as IntMap);

#[test]
fn test_hashmap_wrapper() {
    let mut map = std::collections::HashMap::new();
    map.insert("answer".to_string(), 42);

    let wrapper = IntMap::from(map);

    // Test Deref - need explicit deref for some methods
    assert_eq!((*wrapper).get("answer"), Some(&42));
    assert_eq!((*wrapper).len(), 1);
}

// Test bulk generation
mod bulk_test {
    use super::*;

    elicit_newtypes! {
        String, as S1;
        i32, as I1;
        bool, as B1;
    }

    #[test]
    fn test_bulk() {
        let s = S1::from(String::from("test"));
        assert_eq!(&*s, "test");

        let i = I1::from(42);
        assert_eq!(*i, 42);

        let b = B1::from(true);
        assert!(*b);
    }
}

// Test with Vec
elicit_newtype!(Vec<String>, as StringVec);

#[test]
fn test_vec_wrapper() {
    let v = vec!["a".to_string(), "b".to_string()];
    let wrapper = StringVec::from(v.clone());

    // Test Deref - need to explicitly deref
    assert_eq!((*wrapper).len(), 2);
    assert_eq!((*wrapper)[0], "a");

    // Test unwrap
    let unwrapped: Vec<String> = wrapper.into();
    assert_eq!(unwrapped, v);
}
