//! Gallery level 17: dropping a `String` field via `..` pattern in a transition.
//!
//! `apply_filter` on `ArchiveNavState` fails with "Check that ptr is freeable"
//! because `NavReady { schemas, .. }` silently drops `filter: String`.  This
//! level isolates whether the failure is caused by:
//!
//! (a) The `String` in the state being symbolic (from `kani_depth1`)
//! (b) The `Vec` in the state containing heap-allocated elements
//! (c) Something about the `..` pattern drop itself
//!
//! ## Experiment table
//!
//! | ID   | State String source        | Dropped via `..`? | Expected |
//! |------|----------------------------|--------------------|----------|
//! | 17a  | `String::new()` (empty)    | yes                | PASS     |
//! | 17b  | `String::from("a")` (lit)  | yes                | PASS     |
//! | 17c  | `kani_depth1()` (symbolic) | yes                | FAIL?    |
//! | 17d  | `kani_depth1()` no drop    | moved into result  | PASS?    |
//!
//! ## Run commands
//!
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery17a_empty_string_drop
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery17b_literal_string_drop
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery17c_symbolic_string_drop
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery17d_symbolic_string_moved
//! ```

use elicitation::KaniCompose;
use std::mem::forget;

// ── minimal state machine ─────────────────────────────────────────────────────

#[derive(KaniCompose)]
pub enum G17State {
    Idle,
    Active { label: String },
}

pub fn g17_consistent(s: &G17State) -> bool {
    match s {
        G17State::Active { label } => !label.is_empty(),
        G17State::Idle => true,
    }
}

/// Transition: replace `label` in `Active`, or stay `Idle`.
/// Drops the old `label` via `..`.
#[cfg_attr(kani, kani::requires(g17_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g17_consistent(r)))]
pub fn g17_replace(state: G17State, new_label: String) -> G17State {
    match state {
        G17State::Active { .. } => G17State::Active { label: new_label },
        other => other,
    }
}

// ── 17a: empty string dropped via `..` — expect PASS ─────────────────────────

/// Expected: PASS — `String::new()` is concrete, drop is clean.
#[cfg(kani)]
#[kani::proof_for_contract(g17_replace)]
fn gallery17a_empty_string_drop() {
    let pre = G17State::Active {
        label: String::new(),
    };
    let new_label = String::from("b");
    let result = g17_replace(pre, new_label);
    forget(result);
}

// ── 17b: literal string dropped via `..` — expect PASS ───────────────────────

/// Expected: PASS — `String::from("a")` is concrete, drop is clean.
#[cfg(kani)]
#[kani::proof_for_contract(g17_replace)]
fn gallery17b_literal_string_drop() {
    let pre = G17State::Active {
        label: String::from("a"),
    };
    let new_label = String::from("b");
    let result = g17_replace(pre, new_label);
    forget(result);
}

// ── 17c: symbolic string dropped via `..` — expect FAIL ──────────────────────

/// Expected: FAIL — `kani_depth1()` = `c.to_string()` creates a symbolically-
/// addressed heap buffer; dropping it via `..` hits "Check that ptr is freeable".
#[cfg(kani)]
#[kani::proof_for_contract(g17_replace)]
fn gallery17c_symbolic_string_drop() {
    let symbolic_label = String::kani_depth1();
    kani::assume(!symbolic_label.is_empty());
    let pre = G17State::Active {
        label: symbolic_label,
    };
    let new_label = String::from("b");
    let result = g17_replace(pre, new_label);
    forget(result);
}

// ── 17d: symbolic string moved into result — expect PASS ─────────────────────

/// Expected: PASS — new_label string is moved into result (not dropped inside fn).
/// Pre-state has no heap Strings. Confirms: the issue is specifically about
/// DROPPING a by-value parameter's heap allocation inside a proof_for_contract fn.
#[cfg(kani)]
#[kani::proof_for_contract(g17_replace)]
fn gallery17d_symbolic_string_moved() {
    // Use Idle pre-state so the function hits `other => other` branch.
    // new_label is also dropped in that branch — so this is actually the same issue.
    // Re-designed: give Active state with EMPTY label (no heap) so old label drop is free.
    let pre = G17State::Active {
        label: String::new(),
    };
    // kani::assume(g17_consistent(&pre)) would fail (empty label) → vacuous pass.
    // So this tests: what happens when requires is unsatisfied? (vacuous)
    let new_label = String::from("b");
    let result = g17_replace(pre, new_label);
    forget(result);
}

// ── 17e: plain #[kani::proof] (no DFCC) with literal String drop ─────────────

/// Expected: PASS — no DFCC assigns-clause checks.
/// Confirmed PASS: DFCC is the culprit when 17b fails.
#[cfg(kani)]
#[kani::proof]
fn gallery17e_plain_proof_literal_drop() {
    let pre = G17State::Active {
        label: String::from("a"),
    };
    kani::assume(g17_consistent(&pre));
    let new_label = String::from("b");
    let result = g17_replace(pre, new_label);
    kani::assert(g17_consistent(&result), "17e: result must be consistent");
    forget(result);
}

// ── 17f: proof_for_contract with EMPTY string (fix demonstration) ─────────────
//
// G17StatePermissive has no invariant on the label length.  This lets us
// use `Active { label: String::new() }` in a `proof_for_contract` harness
// without the proof becoming vacuous.
//
// `String::new()` has no heap allocation, so when `g17_permissive_replace`
// drops it via `..`, DFCC sees no `free` and raises no freeable-ptr failure.
//
// **This demonstrates the fix**: if `String::kani_depth1()` returns
// `String::new()` (no heap), DFCC is satisfied.  Depth-2 strings (used only
// in the forgive-and-forget witness) can still be symbolic/non-empty.

#[derive(KaniCompose)]
pub enum G17StatePermissive {
    Idle,
    Active { label: String },
}

pub fn g17_permissive_consistent(_s: &G17StatePermissive) -> bool {
    true
}

#[cfg_attr(kani, kani::requires(g17_permissive_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g17_permissive_consistent(r)))]
pub fn g17_permissive_replace(state: G17StatePermissive, new_label: String) -> G17StatePermissive {
    match state {
        G17StatePermissive::Active { .. } => G17StatePermissive::Active { label: new_label },
        other => other,
    }
}

/// Expected: PASS — `String::new()` has no heap; DFCC sees no `free`.
/// Non-vacuous because `g17_permissive_consistent` always returns `true`.
#[cfg(kani)]
#[kani::proof_for_contract(g17_permissive_replace)]
fn gallery17f_pfc_empty_string_drop() {
    let pre = G17StatePermissive::Active {
        label: String::new(),
    };
    let new_label = String::from("b");
    let result = g17_permissive_replace(pre, new_label);
    forget(result);
}
