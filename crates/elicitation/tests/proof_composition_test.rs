//! Proof composition regression tests.
//!
//! These tests assert that aggregate types' proof outputs contain their
//! constituent types' proof outputs. This catches regressions in manual
//! `impl Elicitation` where a developer refactors the proof body and drops
//! a delegation call.
//!
//! Pattern:
//! ```
//! assert!(Outer::kani_proof_contains::<Inner>(), "message");
//! ```
//!
//! Only covers **manual** impls — derived types get composition for free
//! from the `#[derive(Elicit)]` macro which iterates fields mechanically.

use elicitation::Elicitation;

// ============================================================================
// Helper
// ============================================================================

/// Assert that `Outer`'s Kani proof contains `Inner`'s Kani proof.
#[track_caller]
fn assert_kani_contains<Outer, Inner>(context: &str)
where
    Outer: Elicitation,
    Inner: Elicitation,
{
    assert!(
        {
            let outer = Outer::kani_proof().to_string();
            let inner = Inner::kani_proof().to_string();
            !inner.is_empty() && outer.contains(&inner)
        },
        "{context}: {} kani_proof does not contain {} kani_proof",
        std::any::type_name::<Outer>(),
        std::any::type_name::<Inner>(),
    );
}

// ============================================================================
// Generic stdlib containers — delegate to element type
// ============================================================================

#[test]
fn vec_delegates_to_element() {
    assert_kani_contains::<Vec<bool>, bool>("Vec<bool>");
    assert_kani_contains::<Vec<String>, String>("Vec<String>");
}

#[test]
fn option_delegates_to_inner() {
    assert_kani_contains::<Option<bool>, bool>("Option<bool>");
    assert_kani_contains::<Option<String>, String>("Option<String>");
}

#[test]
fn result_delegates_to_ok_and_err() {
    assert_kani_contains::<Result<bool, String>, bool>("Result ok");
    assert_kani_contains::<Result<bool, String>, String>("Result err");
}

#[test]
fn box_delegates_to_inner() {
    assert_kani_contains::<Box<bool>, bool>("Box<bool>");
}

#[test]
fn arc_delegates_to_inner() {
    assert_kani_contains::<std::sync::Arc<bool>, bool>("Arc<bool>");
}

#[test]
fn rc_delegates_to_inner() {
    assert_kani_contains::<std::rc::Rc<bool>, bool>("Rc<bool>");
}

// ============================================================================
// Verification wrapper types — delegate wrapper harness + inner type
// ============================================================================

use elicitation::verification::types::{
    ArcSatisfies, BTreeMapNonEmpty, BTreeSetNonEmpty, BoxSatisfies, HashMapNonEmpty,
    HashSetNonEmpty, LinkedListNonEmpty, OptionSome, RcSatisfies, ResultOk, VecAllSatisfy,
    VecDequeNonEmpty, VecNonEmpty,
};

#[test]
fn vec_non_empty_delegates_to_element() {
    assert_kani_contains::<VecNonEmpty<bool>, bool>("VecNonEmpty<bool>");
    assert_kani_contains::<VecNonEmpty<String>, String>("VecNonEmpty<String>");
}

#[test]
fn vec_all_satisfy_delegates_to_element() {
    assert_kani_contains::<VecAllSatisfy<bool>, bool>("VecAllSatisfy<bool>");
}

#[test]
fn option_some_delegates_to_inner() {
    assert_kani_contains::<OptionSome<bool>, bool>("OptionSome<bool>");
    assert_kani_contains::<OptionSome<String>, String>("OptionSome<String>");
}

#[test]
fn result_ok_delegates_to_inner() {
    assert_kani_contains::<ResultOk<bool>, bool>("ResultOk<bool>");
    assert_kani_contains::<ResultOk<String>, String>("ResultOk<String>");
}

#[test]
fn box_satisfies_delegates_to_inner() {
    assert_kani_contains::<BoxSatisfies<bool>, bool>("BoxSatisfies<bool>");
}

#[test]
fn arc_satisfies_delegates_to_inner() {
    assert_kani_contains::<ArcSatisfies<bool>, bool>("ArcSatisfies<bool>");
}

#[test]
fn rc_satisfies_delegates_to_inner() {
    assert_kani_contains::<RcSatisfies<bool>, bool>("RcSatisfies<bool>");
}

#[test]
fn hash_map_non_empty_delegates_to_kv() {
    assert_kani_contains::<HashMapNonEmpty<String, bool>, String>("HashMapNonEmpty key");
    assert_kani_contains::<HashMapNonEmpty<String, bool>, bool>("HashMapNonEmpty value");
}

#[test]
fn btree_map_non_empty_delegates_to_kv() {
    assert_kani_contains::<BTreeMapNonEmpty<String, bool>, String>("BTreeMapNonEmpty key");
    assert_kani_contains::<BTreeMapNonEmpty<String, bool>, bool>("BTreeMapNonEmpty value");
}

#[test]
fn hash_set_non_empty_delegates_to_element() {
    assert_kani_contains::<HashSetNonEmpty<bool>, bool>("HashSetNonEmpty<bool>");
}

#[test]
fn btree_set_non_empty_delegates_to_element() {
    assert_kani_contains::<BTreeSetNonEmpty<bool>, bool>("BTreeSetNonEmpty<bool>");
}

#[test]
fn vec_deque_non_empty_delegates_to_element() {
    assert_kani_contains::<VecDequeNonEmpty<bool>, bool>("VecDequeNonEmpty<bool>");
}

#[test]
fn linked_list_non_empty_delegates_to_element() {
    assert_kani_contains::<LinkedListNonEmpty<bool>, bool>("LinkedListNonEmpty<bool>");
}

// ============================================================================
// Primitives — delegate to wrapper types
// ============================================================================

use elicitation::verification::types::{BoolDefault, F64Default, I32Default, I64Default};

#[test]
fn bool_delegates_to_bool_default() {
    assert_kani_contains::<bool, BoolDefault>("bool → BoolDefault");
}

#[test]
fn i32_delegates_to_i32_default() {
    assert_kani_contains::<i32, I32Default>("i32 → I32Default");
}

#[test]
fn i64_delegates_to_i64_default() {
    assert_kani_contains::<i64, I64Default>("i64 → I64Default");
}

#[test]
fn f64_delegates_to_f64_default() {
    assert_kani_contains::<f64, F64Default>("f64 → F64Default");
}

// ============================================================================
// URL — delegates to UrlValid wrapper
// ============================================================================

#[cfg(feature = "url")]
mod url_tests {
    use super::assert_kani_contains;
    use elicitation::verification::types::UrlValid;

    #[test]
    fn url_delegates_to_url_valid() {
        assert_kani_contains::<url::Url, UrlValid>("url::Url → UrlValid");
    }
}
