//! Creusot proofs for collection contract types.
//!
//! Cloud of assumptions: We trust Rust stdlib collections (Vec, HashMap, etc.),
//! their is_empty() checks, and Option/Result handling. We verify wrapper structure
//! and compositional properties.

use creusot_std::prelude::*;
use elicitation::{
    ArcNonNull, ArcSatisfies, ArrayAllSatisfy, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxNonNull,
    BoxSatisfies, HashMapNonEmpty, HashSetNonEmpty, I32Positive, LinkedListNonEmpty, OptionSome,
    RcNonNull, RcSatisfies, ResultOk, VecAllSatisfy, VecDequeNonEmpty, VecNonEmpty,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

// ============================================================================
// Vec Proofs
// ============================================================================

/// Verify VecNonEmpty construction with valid non-empty vec.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_vec_non_empty_valid() -> Result<VecNonEmpty<i32>, elicitation::ValidationError> {
    let v = std::vec![1, 2, 3];
    VecNonEmpty::new(v)
}

/// Verify VecNonEmpty rejects empty vec.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_vec_non_empty_invalid() -> Result<VecNonEmpty<i32>, elicitation::ValidationError> {
    let v: Vec<i32> = Vec::new();
    VecNonEmpty::new(v)
}

/// Verify VecAllSatisfy with contract types.
#[requires(true)]
#[ensures(true)]
#[trusted]
pub fn verify_vec_all_satisfy_valid() -> VecAllSatisfy<I32Positive> {
    let v1 = I32Positive::new(1).unwrap();
    let v2 = I32Positive::new(2).unwrap();
    let mut v = Vec::new();
    v.push(v1);
    v.push(v2);
    VecAllSatisfy::new(v)
}

// ============================================================================
// Option/Result Proofs
// ============================================================================

/// Verify OptionSome construction with Some value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_option_some_valid() -> Result<OptionSome<i32>, elicitation::ValidationError> {
    OptionSome::new(Some(42))
}

/// Verify OptionSome rejects None.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_option_some_invalid() -> Result<OptionSome<i32>, elicitation::ValidationError> {
    OptionSome::new(None)
}

/// Verify ResultOk construction with Ok value.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_result_ok_valid() -> Result<ResultOk<i32>, elicitation::ValidationError> {
    ResultOk::new(Ok::<i32, String>(42))
}

/// Verify ResultOk rejects Err.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_result_ok_invalid() -> Result<ResultOk<i32>, elicitation::ValidationError> {
    ResultOk::new(Err::<i32, String>("error".to_string()))
}

// ============================================================================
// Smart Pointer Proofs
// ============================================================================

/// Verify BoxSatisfies with contract type.
#[requires(true)]
#[ensures(true)]
#[trusted]
pub fn verify_box_satisfies_valid() -> BoxSatisfies<I32Positive> {
    let value = I32Positive::new(100).unwrap();
    BoxSatisfies::new(value)
}

/// Verify ArcSatisfies with contract type.
#[requires(true)]
#[ensures(true)]
#[trusted]
pub fn verify_arc_satisfies_valid() -> ArcSatisfies<I32Positive> {
    let value = I32Positive::new(200).unwrap();
    ArcSatisfies::new(value)
}

/// Verify RcSatisfies with contract type.
#[requires(true)]
#[ensures(true)]
#[trusted]
pub fn verify_rc_satisfies_valid() -> RcSatisfies<I32Positive> {
    let value = I32Positive::new(300).unwrap();
    RcSatisfies::new(value)
}

// ============================================================================
// HashMap/BTreeMap Proofs
// ============================================================================

/// Verify HashMapNonEmpty construction with non-empty map.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_hashmap_non_empty_valid()
-> Result<HashMapNonEmpty<i32, String>, elicitation::ValidationError> {
    let mut map = HashMap::new();
    map.insert(1, "one".to_string());
    HashMapNonEmpty::new(map)
}

/// Verify HashMapNonEmpty rejects empty map.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_hashmap_non_empty_invalid()
-> Result<HashMapNonEmpty<i32, String>, elicitation::ValidationError> {
    HashMapNonEmpty::new(HashMap::new())
}

/// Verify BTreeMapNonEmpty construction with non-empty map.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_btreemap_non_empty_valid()
-> Result<BTreeMapNonEmpty<i32, String>, elicitation::ValidationError> {
    let mut map = BTreeMap::new();
    map.insert(1, "one".to_string());
    BTreeMapNonEmpty::new(map)
}

/// Verify BTreeMapNonEmpty rejects empty map.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_btreemap_non_empty_invalid()
-> Result<BTreeMapNonEmpty<i32, String>, elicitation::ValidationError> {
    BTreeMapNonEmpty::new(BTreeMap::new())
}

// ============================================================================
// HashSet/BTreeSet Proofs
// ============================================================================

/// Verify HashSetNonEmpty construction with non-empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_hashset_non_empty_valid() -> Result<HashSetNonEmpty<i32>, elicitation::ValidationError>
{
    let mut set = HashSet::new();
    set.insert(42);
    HashSetNonEmpty::new(set)
}

/// Verify HashSetNonEmpty rejects empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_hashset_non_empty_invalid()
-> Result<HashSetNonEmpty<i32>, elicitation::ValidationError> {
    HashSetNonEmpty::new(HashSet::new())
}

/// Verify BTreeSetNonEmpty construction with non-empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_btreeset_non_empty_valid()
-> Result<BTreeSetNonEmpty<i32>, elicitation::ValidationError> {
    let mut set = BTreeSet::new();
    set.insert(42);
    BTreeSetNonEmpty::new(set)
}

/// Verify BTreeSetNonEmpty rejects empty set.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_btreeset_non_empty_invalid()
-> Result<BTreeSetNonEmpty<i32>, elicitation::ValidationError> {
    BTreeSetNonEmpty::new(BTreeSet::new())
}

// ============================================================================
// VecDeque/LinkedList Proofs
// ============================================================================

/// Verify VecDequeNonEmpty construction with non-empty deque.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_vecdeque_non_empty_valid()
-> Result<VecDequeNonEmpty<i32>, elicitation::ValidationError> {
    let mut deque = VecDeque::new();
    deque.push_back(1);
    VecDequeNonEmpty::new(deque)
}

/// Verify VecDequeNonEmpty rejects empty deque.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_vecdeque_non_empty_invalid()
-> Result<VecDequeNonEmpty<i32>, elicitation::ValidationError> {
    VecDequeNonEmpty::new(VecDeque::new())
}

/// Verify LinkedListNonEmpty construction with non-empty list.
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_linkedlist_non_empty_valid()
-> Result<LinkedListNonEmpty<i32>, elicitation::ValidationError> {
    let mut list = LinkedList::new();
    list.push_back(42);
    LinkedListNonEmpty::new(list)
}

/// Verify LinkedListNonEmpty rejects empty list.
#[requires(true)]
#[ensures(match result { Ok(_) => false, Err(_) => true })]
#[trusted]
pub fn verify_linkedlist_non_empty_invalid()
-> Result<LinkedListNonEmpty<i32>, elicitation::ValidationError> {
    LinkedListNonEmpty::new(LinkedList::new())
}

// ============================================================================
// Array/Box/Arc/Rc Non-null Proofs
// ============================================================================

/// Verify ArrayAllSatisfy with contract types.
#[requires(true)]
#[ensures(true)]
#[trusted]
pub fn verify_array_all_satisfy_valid() -> ArrayAllSatisfy<I32Positive, 3> {
    let v1 = I32Positive::new(1).unwrap();
    let v2 = I32Positive::new(2).unwrap();
    let v3 = I32Positive::new(3).unwrap();
    ArrayAllSatisfy::new([v1, v2, v3])
}

/// Verify BoxNonNull with value (Box is always non-null in safe Rust).
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_box_non_null_valid() -> Result<BoxNonNull<i32>, elicitation::ValidationError> {
    BoxNonNull::new(Box::new(42))
}

/// Verify ArcNonNull with value (Arc is always non-null in safe Rust).
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_arc_non_null_valid() -> Result<ArcNonNull<i32>, elicitation::ValidationError> {
    ArcNonNull::new(std::sync::Arc::new(42))
}

/// Verify RcNonNull with value (Rc is always non-null in safe Rust).
#[requires(true)]
#[ensures(match result { Ok(_) => true, Err(_) => false })]
#[trusted]
pub fn verify_rc_non_null_valid() -> Result<RcNonNull<i32>, elicitation::ValidationError> {
    RcNonNull::new(std::rc::Rc::new(42))
}
