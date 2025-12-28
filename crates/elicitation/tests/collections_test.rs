//! Tests for collection type implementations.

use elicitation::{Elicitation, Prompt};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

#[test]
fn test_hashmap_has_prompt() {
    type TestMap = HashMap<String, i32>;
    assert!(TestMap::prompt().is_some());
}

#[test]
fn test_btreemap_has_prompt() {
    type TestMap = BTreeMap<String, i32>;
    assert!(TestMap::prompt().is_some());
}

#[test]
fn test_hashset_has_prompt() {
    type TestSet = HashSet<String>;
    assert!(TestSet::prompt().is_some());
}

#[test]
fn test_btreeset_has_prompt() {
    type TestSet = BTreeSet<String>;
    assert!(TestSet::prompt().is_some());
}

#[test]
fn test_vecdeque_has_prompt() {
    type TestDeque = VecDeque<String>;
    assert!(TestDeque::prompt().is_some());
}

#[test]
fn test_linkedlist_has_prompt() {
    type TestList = LinkedList<String>;
    assert!(TestList::prompt().is_some());
}

#[test]
fn test_hashmap_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<HashMap<String, i32>>();
}

#[test]
fn test_btreemap_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<BTreeMap<String, i32>>();
}

#[test]
fn test_hashset_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<HashSet<String>>();
}

#[test]
fn test_btreeset_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<BTreeSet<String>>();
}

#[test]
fn test_vecdeque_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<VecDeque<String>>();
}

#[test]
fn test_linkedlist_trait_bounds() {
    fn requires_elicitation<T: Elicitation>() {}
    requires_elicitation::<LinkedList<String>>();
}
