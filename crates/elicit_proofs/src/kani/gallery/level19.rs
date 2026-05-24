//! Gallery level 19: symbolic `String` arguments dropped by value.
//!
//! Levels 17 and 18 focused on Strings stored in state and then dropped via
//! `..` patterns. The failing `apply_filter` canary adds a different shape:
//! the by-value `filter: String` argument itself may be dropped on a
//! passthrough branch (`other => other`).
//!
//! This level isolates whether DFCC cares about:
//! 1. **symbolic contents** in general, or
//! 2. the **allocation provenance** of the `String` heap buffer.
//!
//! ## Hypothesis
//!
//! `char.to_string()` may produce a String whose heap provenance is too opaque
//! for DFCC's "ptr is freeable" check when the argument is dropped by value
//! inside a `#[kani::proof_for_contract]` function.
//!
//! If we instead allocate the backing buffer concretely with
//! `String::with_capacity(...)` and then push symbolic chars into it, DFCC may
//! accept the drop while still leaving the String contents symbolic.
//!
//! ## Run commands
//!
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery19a_arg_drop_to_string
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery19b_arg_drop_capacity_one_char
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery19c_arg_drop_capacity_two_chars
//! ```

/// Contract target: the argument is accepted by value and dropped in the body.
#[cfg_attr(kani, kani::requires(!filter.is_empty()))]
#[cfg_attr(kani, kani::ensures(|_| true))]
pub fn g19_drop_filter_arg(filter: String) {
    let _ = filter.len();
}

#[cfg(kani)]
fn g19_symbolic_one_char_to_string() -> String {
    kani::any::<char>().to_string()
}

#[cfg(kani)]
fn g19_symbolic_one_char_with_capacity() -> String {
    let mut s = String::with_capacity(4);
    s.push(kani::any::<char>());
    s
}

#[cfg(kani)]
fn g19_symbolic_two_chars_with_capacity() -> String {
    let mut s = String::with_capacity(8);
    s.push(kani::any::<char>());
    s.push(kani::any::<char>());
    s
}

/// Baseline: current `KaniCompose`-style one-symbolic-char construction.
///
/// Expected: this reproduces the current DFCC/freeability behavior for a
/// by-value String argument that is dropped inside the function body.
#[cfg(kani)]
#[kani::proof_for_contract(g19_drop_filter_arg)]
fn gallery19a_arg_drop_to_string() {
    let filter = g19_symbolic_one_char_to_string();
    kani::assume(!filter.is_empty());
    g19_drop_filter_arg(filter);
}

/// Hypothesis test: concrete allocation, symbolic one-char contents.
///
/// If this passes while 19a fails, the right fix is not "make Strings
/// concrete"; it is "preserve symbolic contents while making allocation
/// provenance concrete."
#[cfg(kani)]
#[kani::proof_for_contract(g19_drop_filter_arg)]
fn gallery19b_arg_drop_capacity_one_char() {
    let filter = g19_symbolic_one_char_with_capacity();
    kani::assume(!filter.is_empty());
    g19_drop_filter_arg(filter);
}

/// Same hypothesis at depth-2 string length.
///
/// This checks whether the concrete-allocation strategy composes with the
/// second inductive String step as well.
#[cfg(kani)]
#[kani::proof_for_contract(g19_drop_filter_arg)]
fn gallery19c_arg_drop_capacity_two_chars() {
    let filter = g19_symbolic_two_chars_with_capacity();
    kani::assume(!filter.is_empty());
    g19_drop_filter_arg(filter);
}
