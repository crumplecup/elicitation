//! Tests for the elicit_newtype! and elicit_newtypes! macros.

use elicitation::{elicit_newtype, elicit_newtype_traits, elicit_newtypes};
use std::{collections::HashMap, sync::Arc};

// Simple wrapper test
elicit_newtype!(String, as StringWrapper);

#[test]
fn test_simple_wrapper() {
    let s = String::from("hello");
    let wrapper = StringWrapper::from(s.clone());

    // Test Deref - derefs through Arc to String in one step
    assert_eq!(&*wrapper, "hello");
    assert_eq!(wrapper.len(), 5);

    // Test unwrap - extract Arc and try_unwrap
    let arc: Arc<String> = wrapper.into();
    let unwrapped = Arc::try_unwrap(arc).unwrap();
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
        assert!(&*b);
    }
}

// Test with Vec
elicit_newtype!(Vec<String>, as StringVec);

#[test]
fn test_vec_wrapper() {
    let v = vec!["a".to_string(), "b".to_string()];
    let wrapper = StringVec::from(v.clone());

    // Test Deref - derefs through Arc to Vec in one step
    assert_eq!(wrapper.len(), 2);
    assert_eq!(wrapper[0], "a");

    // Test unwrap - extract Arc and try_unwrap
    let arc: Arc<Vec<String>> = wrapper.into();
    let unwrapped = Arc::try_unwrap(arc).unwrap();
    assert_eq!(unwrapped, v);
}

// ── elicit_newtype_traits! tests ─────────────────────────────────────────────

// u32 supports: PartialEq, Eq, Hash, PartialOrd, Ord, Display, FromStr
elicit_newtype!(u32, as U32Wrapper);
elicit_newtype_traits!(U32Wrapper, u32, [cmp, display, from_str]);

#[test]
fn test_traits_eq_hash_ord() {
    let a = U32Wrapper::from(1u32);
    let b = U32Wrapper::from(2u32);
    let a2 = U32Wrapper::from(1u32);

    // PartialEq + Eq
    assert_eq!(a, a2);
    assert_ne!(a, b);

    // Hash — usable as HashMap key
    let mut map: HashMap<U32Wrapper, &str> = HashMap::new();
    map.insert(a2, "one");
    let key = U32Wrapper::from(1u32);
    assert_eq!(map[&key], "one");

    // PartialOrd + Ord
    assert!(a < b);
    assert!(b > a);
    let mut v = vec![b.clone(), a.clone()];
    v.sort();
    assert_eq!(v, vec![a.clone(), b.clone()]);
}

#[test]
fn test_traits_display() {
    let w = U32Wrapper::from(42u32);
    assert_eq!(w.to_string(), "42");
}

#[test]
fn test_traits_from_str() {
    let w: U32Wrapper = "99".parse().unwrap();
    assert_eq!(*w, 99u32);
}

#[test]
fn test_traits_usable_in_derived_structs() {
    // The primary use case: user's struct derives these traits using the wrapper
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct Record {
        id: U32Wrapper,
        name: String,
    }

    let r1 = Record {
        id: U32Wrapper::from(1u32),
        name: "alice".into(),
    };
    let r2 = Record {
        id: U32Wrapper::from(2u32),
        name: "bob".into(),
    };
    assert!(r1 < r2);
    let mut set = std::collections::BTreeSet::new();
    set.insert(r2.clone());
    set.insert(r1.clone());
    assert_eq!(set.iter().next().unwrap(), &r1);
}

// String supports eq_hash but not Ord (String does have Ord, but test eq_hash flag standalone)
elicit_newtype!(i32, as I32Wrapper);
elicit_newtype_traits!(I32Wrapper, i32, [eq_hash]);

#[test]
fn test_traits_eq_hash_only() {
    let a = I32Wrapper::from(-5i32);
    let b = I32Wrapper::from(-5i32);
    assert_eq!(a, b);
    let mut map: HashMap<I32Wrapper, i32> = HashMap::new();
    map.insert(a, 42);
    assert_eq!(map[&b], 42);
}
