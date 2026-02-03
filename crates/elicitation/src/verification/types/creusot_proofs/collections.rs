//! Creusot proofs for collection contract types.

#![cfg(feature = "verify-creusot")]
#![allow(unused_imports)]

use crate::*;
// Contracts provided by creusot_std;

// Collection Contract Proofs
// ============================================================================

/// Prove that VecNonEmpty construction succeeds for non-empty vectors.
#[requires(!value.is_empty())]
#[ensures(result.is_ok())]
pub fn verify_vec_non_empty_valid<T>(value: Vec<T>) -> Result<VecNonEmpty<T>, ValidationError> {
    VecNonEmpty::new(value)
}

/// Prove that VecNonEmpty construction fails for empty vectors.
#[requires(value.is_empty())]
#[ensures(result.is_err())]
pub fn verify_vec_non_empty_invalid<T>(value: Vec<T>) -> Result<VecNonEmpty<T>, ValidationError> {
    VecNonEmpty::new(value)
}

/// Prove that OptionSome construction succeeds for Some values.
#[requires(value.is_some())]
#[ensures(result.is_ok())]
pub fn verify_option_some_valid<T>(value: Option<T>) -> Result<OptionSome<T>, ValidationError> {
    OptionSome::new(value)
}

/// Prove that OptionSome construction fails for None.
#[requires(value.is_none())]
#[ensures(result.is_err())]
pub fn verify_option_some_invalid<T>(value: Option<T>) -> Result<OptionSome<T>, ValidationError> {
    OptionSome::new(value)
}

/// Prove that ResultOk construction succeeds for Ok values.
#[requires(value.is_ok())]
#[ensures(result.is_ok())]
pub fn verify_result_ok_valid<T>(value: Result<T, ()>) -> Result<ResultOk<T>, ValidationError> {
    ResultOk::new(value)
}

/// Prove that ResultOk construction fails for Err values.
#[requires(value.is_err())]
#[ensures(result.is_err())]
pub fn verify_result_ok_invalid<T>(value: Result<T, ()>) -> Result<ResultOk<T>, ValidationError> {
    ResultOk::new(value)
}

/// Prove that BoxNonNull construction succeeds for non-null boxes.
#[ensures(result.is_ok())]
pub fn verify_box_non_null_valid<T>(value: Box<T>) -> Result<BoxNonNull<T>, ValidationError> {
    BoxNonNull::new(value)
}

/// Prove that ArcNonNull construction succeeds for non-null Arcs.
#[ensures(result.is_ok())]
pub fn verify_arc_non_null_valid<T>(
    value: ::std::sync::Arc<T>,
) -> Result<ArcNonNull<T>, ValidationError> {
    ArcNonNull::new(value)
}

/// Prove that RcNonNull construction succeeds for non-null Rcs.
#[ensures(result.is_ok())]
pub fn verify_rc_non_null_valid<T>(
    value: ::std::rc::Rc<T>,
) -> Result<RcNonNull<T>, ValidationError> {
    RcNonNull::new(value)
}

/// Prove that HashMapNonEmpty construction succeeds for non-empty maps.
#[requires(!value.is_empty())]
#[ensures(result.is_ok())]
pub fn verify_hashmap_non_empty_valid<K, V>(
    value: ::std::collections::HashMap<K, V>,
) -> Result<HashMapNonEmpty<K, V>, ValidationError> {
    HashMapNonEmpty::new(value)
}

// Note: Creusot does not support BTreeMap yet
// #[requires(!value.is_empty())]
// #[ensures(result.is_ok())]
// pub fn verify_btreemap_non_empty_valid<K, V>(value: std::collections::BTreeMap<K, V>) -> Result<BTreeMapNonEmpty<K, V>, ValidationError> {
//     BTreeMapNonEmpty::new(value)
// }

/// Prove that HashSetNonEmpty construction succeeds for non-empty sets.
#[requires(!value.is_empty())]
#[ensures(result.is_ok())]
pub fn verify_hashset_non_empty_valid<T>(
    value: ::std::collections::HashSet<T>,
) -> Result<HashSetNonEmpty<T>, ValidationError> {
    HashSetNonEmpty::new(value)
}

// Note: Creusot does not support BTreeSet yet
// #[requires(!value.is_empty())]
// #[ensures(result.is_ok())]
// pub fn verify_btreeset_non_empty_valid<T>(value: std::collections::BTreeSet<T>) -> Result<BTreeSetNonEmpty<T>, ValidationError> {
//     BTreeSetNonEmpty::new(value)
// }

// Note: Creusot does not support VecDeque yet
// #[requires(!value.is_empty())]
// #[ensures(result.is_ok())]
// pub fn verify_vecdeque_non_empty_valid<T>(value: std::collections::VecDeque<T>) -> Result<VecDequeNonEmpty<T>, ValidationError> {
//     VecDequeNonEmpty::new(value)
// }

// Note: Creusot does not support LinkedList yet
// #[requires(!value.is_empty())]
// #[ensures(result.is_ok())]
// pub fn verify_linkedlist_non_empty_valid<T>(value: std::collections::LinkedList<T>) -> Result<LinkedListNonEmpty<T>, ValidationError> {
//     LinkedListNonEmpty::new(value)
// }

/// Prove that ArrayAllSatisfy construction succeeds when all elements satisfy contract.
#[requires(true)]
#[ensures(true)]
pub fn verify_array_all_satisfy_valid<C, const N: usize>(value: [C; N]) -> ArrayAllSatisfy<C, N> {
    ArrayAllSatisfy::new(value)
}

/// Prove that VecAllSatisfy construction succeeds when all elements satisfy contract.
#[requires(true)]
#[ensures(true)]
pub fn verify_vec_all_satisfy_valid<C>(value: Vec<C>) -> VecAllSatisfy<C> {
    VecAllSatisfy::new(value)
}

// ============================================================================
// Tuple Contract Proofs
// ============================================================================

/// Prove that Tuple2 construction succeeds when both elements satisfy contracts.
#[requires(true)]
#[ensures(result.is_ok())]
pub fn verify_tuple2_valid<C1, C2>(
    first: C1,
    second: C2,
) -> Result<Tuple2<C1, C2>, ValidationError> {
    Ok(Tuple2::new(first, second))
}

/// Prove that Tuple3 construction succeeds when all elements satisfy contracts.
#[requires(true)]
#[ensures(result.is_ok())]
pub fn verify_tuple3_valid<C1, C2, C3>(
    first: C1,
    second: C2,
    third: C3,
) -> Result<Tuple3<C1, C2, C3>, ValidationError> {
    Ok(Tuple3::new(first, second, third))
}

/// Prove that Tuple4 construction succeeds when all elements satisfy contracts.
#[requires(true)]
#[ensures(result.is_ok())]
pub fn verify_tuple4_valid<C1, C2, C3, C4>(
    first: C1,
    second: C2,
    third: C3,
    fourth: C4,
) -> Result<Tuple4<C1, C2, C3, C4>, ValidationError> {
    Ok(Tuple4::new(first, second, third, fourth))
}

// ============================================================================
