//! Gallery level 18: sound inductive proof for String-bearing VSMs.
//!
//! The current `KaniCompose for String` implementation is **unsound**:
//! `kani_depth1()` returns `String::new()` — identical to depth-0.
//! Any VSM invariant that requires `!label.is_empty()` has its `requires`
//! clause vacuously satisfied (via `kani::assume(false)` = path pruned),
//! so the proof never exercises the non-empty case at all.
//!
//! This level isolates hypotheses for a sound fix without `unsafe`.
//!
//! ## The generated harness shape
//!
//! The generator emits a single closure harness per transition:
//!
//! ```rust,ignore
//! fn my_transition_kani_closure() {
//!     // Witness: depth2 state, forgotten after assume-check
//!     let witness = MyState::kani_depth2();
//!     kani::assume(my_consistent(&witness));   // ← requires must be sat here
//!     std::mem::forget(witness);
//!     // Actual input: depth1 state, passed to function
//!     let state = MyState::kani_depth1();      // ← if String::new(), vacuous!
//!     let _result = my_transition(state, ...);
//!     std::mem::forget(_result);
//! }
//! ```
//!
//! The `kani_depth2()` witness state is forgotten — its Strings are never
//! dropped.  The `kani_depth1()` state is passed to the function and may
//! be dropped inside it.
//!
//! ## Hypotheses
//!
//! | ID   | `kani_depth1()` for String     | Expected |
//! |------|--------------------------------|----------|
//! | 18a  | `String::new()` (current)      | VACUOUS (no output from kani — vacuity not detected automatically) |
//! | 18b  | `String::from("a")` (lit)      | PASS — non-vacuous, concrete, no DFCC issue |
//! | 18c  | `String::from("a")` + drop     | PASS — 17b established concrete literals survive DFCC drop |
//! | 18d  | `kani_depth2()` = symbolic     | FAIL? — symbolic heap freed inside fn triggers DFCC |
//! | 18e  | `kani_depth2()` = "ab" lit     | PASS — two concrete chars |
//!
//! ## Conclusions sought
//!
//! If 18b PASSes and 18a is vacuous: fix `kani_depth1()` → `String::from("a")`.
//! If 18e PASSes: fix `kani_depth2()` → `String::from("ab")`.
//! If 18d FAILs with DFCC: confirms symbolic strings cannot be dropped in pfc.
//!
//! ## Run commands
//!
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery18a_vacuous_empty 2>&1 | tee /tmp/g18a.log
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery18b_literal_depth1_nonempty 2>&1 | tee /tmp/g18b.log
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery18c_literal_depth1_drop 2>&1 | tee /tmp/g18c.log
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery18d_symbolic_depth2_drop 2>&1 | tee /tmp/g18d.log
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing \
//!   --harness gallery18e_literal_depth2_nonempty 2>&1 | tee /tmp/g18e.log
//! ```

use std::mem::forget;

// ── Minimal VSM with a non-empty String invariant ────────────────────────────

/// A minimal state machine where `Active` requires a non-empty label.
/// This invariant is the common case: most VSM string fields represent
/// names, schemas, labels — never empty when present.
pub enum G18State {
    Idle,
    Active { label: String },
}

/// Invariant: `Active` variant must have a non-empty label.
pub fn g18_consistent(s: &G18State) -> bool {
    match s {
        G18State::Active { label } => !label.is_empty(),
        G18State::Idle => true,
    }
}

/// Transition: rename the label in `Active`.  Drops old label via `..`.
/// Requires: pre-state consistent, new_label non-empty.
/// Ensures:  result consistent.
#[cfg_attr(kani, kani::requires(g18_consistent(&state) && !new_label.is_empty()))]
#[cfg_attr(kani, kani::ensures(|r| g18_consistent(r)))]
pub fn g18_rename(state: G18State, new_label: String) -> G18State {
    match state {
        G18State::Active { .. } => G18State::Active { label: new_label },
        G18State::Idle => G18State::Active { label: new_label },
    }
}

// ── 18a: current approach — depth1 = String::new() → VACUOUS ────────────────

/// Uses `String::new()` at depth1 (current KaniCompose behaviour).
/// `g18_consistent(Active { label: "" })` = `false` → `kani::assume(false)` →
/// this path is pruned.  Kani reports PASS but the proof is vacuous: the
/// non-empty-label branch was never explored.
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18a_vacuous_empty() {
    // Witness (depth2-equivalent): active with empty label — assume will prune
    let witness = G18State::Active {
        label: String::new(),
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    // Actual input (depth1-equivalent): also empty — same problem
    let state = G18State::Active {
        label: String::new(),
    };
    let new_label = String::from("b");
    let result = g18_rename(state, new_label);
    forget(result);
}

// ── 18b: hypothesis — depth1 = String::from("a") → non-vacuous PASS? ────────

/// Uses `String::from("a")` at depth1.
/// `g18_consistent(Active { label: "a" })` = `true` → assume passes.
/// The transition receives a real non-empty label and must preserve consistency.
/// Expected: PASS and non-vacuous (kani should report ≥ 1 reachable check).
///
/// No DFCC issue expected: `g18_rename` moves `new_label` into the result;
/// the old `Active { label: "a" }` is dropped via `..` (concrete heap ptr).
/// Level 17b established that concrete literal drops pass DFCC.
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18b_literal_depth1_nonempty() {
    // Witness: depth2-equivalent, non-empty so assume passes, then forgotten
    let witness = G18State::Active {
        label: String::from("ab"),
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    // Actual input: depth1-equivalent with concrete non-empty string
    let state = G18State::Active {
        label: String::from("a"),
    };
    let new_label = String::from("b");
    let result = g18_rename(state, new_label);
    forget(result);
}

// ── 18c: explicit drop variant — confirm literal string survives DFCC drop ───

/// Same as 18b but `result` is NOT forgotten — it is dropped normally.
/// The result is `Active { label: "b" }` — a concrete string is freed on drop.
/// Expected: PASS — concrete heap ptrs are freeable (DFCC can verify this).
///
/// This confirms that even when the String leaves via the result and is then
/// dropped at the end of the harness, there is no DFCC issue with literals.
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18c_literal_depth1_drop() {
    let witness = G18State::Active {
        label: String::from("ab"),
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    let state = G18State::Active {
        label: String::from("a"),
    };
    let new_label = String::from("b");
    // result is dropped here (not forgotten) — exercises DFCC's free check
    let _result = g18_rename(state, new_label);
}

// ── 18d: symbolic depth2 drop — expect DFCC FAIL ────────────────────────────

/// Uses a two-symbolic-char String at depth1 (what kani_depth2 currently does).
/// The old `Active` variant is dropped inside `g18_rename` via `..`.
/// A symbolically-addressed heap buffer cannot be freed by DFCC.
/// Expected: FAIL with "Check that ptr is freeable" (confirms level17c finding).
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18d_symbolic_depth2_drop() {
    let witness = G18State::Active {
        label: String::from("ab"),
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    // Symbolic two-char string — same construction as current kani_depth2
    let c1: char = kani::any();
    let c2: char = kani::any();
    let mut sym_label = c1.to_string();
    sym_label.push(c2);
    kani::assume(!sym_label.is_empty()); // always true for two chars, but explicit
    let state = G18State::Active { label: sym_label };
    let new_label = String::from("b");
    let result = g18_rename(state, new_label);
    forget(result);
}

// ── 18e: literal depth2 = "ab" — confirms two-char literal is fine ───────────

/// Uses `String::from("ab")` at depth2 (proposed fix for kani_depth2).
/// Two concrete chars, non-empty, dropped via `..` inside the function.
/// Expected: PASS — same argument as 18b/18c.
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18e_literal_depth2_nonempty() {
    let witness = G18State::Active {
        label: String::from("abc"),
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    let state = G18State::Active {
        label: String::from("ab"),
    };
    let new_label = String::from("c");
    let result = g18_rename(state, new_label);
    forget(result);
}

// ── 18f: one symbolic char at depth1 — the TRUE inductive fix ────────────────

/// Uses a single symbolic char as depth1 (proposed fix for `kani_depth1()`).
/// CBMC explores ALL single-char strings, proving the invariant for any
/// non-empty one-char label.  Combined with depth0 (empty, base case) and
/// depth2 (two chars), this gives a proper inductive chain.
///
/// Key insight from 18d: symbolic-heap Strings can be dropped inside
/// `proof_for_contract` without DFCC failure.  The prior belief that
/// `String::new()` was required at depth1 for DFCC safety was wrong.
///
/// Expected: PASS — one symbolic char satisfies `!is_empty()`, assume passes,
/// CBMC explores all char values.
#[cfg(kani)]
#[kani::proof_for_contract(g18_rename)]
fn gallery18f_symbolic_depth1_one_char() {
    // Witness (depth2 equivalent): two symbolic chars, forgotten after assume
    let c1: char = kani::any();
    let c2: char = kani::any();
    let mut witness_label = c1.to_string();
    witness_label.push(c2);
    let witness = G18State::Active {
        label: witness_label,
    };
    kani::assume(g18_consistent(&witness));
    forget(witness);
    // Actual input: ONE symbolic char — the true inductive depth1 step
    let one_char: char = kani::any();
    let state = G18State::Active {
        label: one_char.to_string(),
    };
    let new_label = String::from("b");
    let result = g18_rename(state, new_label);
    forget(result);
}

// ── Summary ──────────────────────────────────────────────────────────────────
//
// | ID   | What                              | Result    |
// |------|-----------------------------------|-----------|
// | 18a  | String::new() at depth1           | VACUOUS   |
// | 18b  | String::from("a") at depth1       | PASS 0.6s |
// | 18c  | String::from("a"), result dropped | PASS 0.7s |
// | 18d  | Two symbolic chars, dropped       | PASS 2.8s |
// | 18e  | String::from("ab") at depth2      | PASS      |
// | 18f  | One symbolic char at depth1       | PASS?     |
//
// Conclusion: symbolic Strings CAN be dropped inside proof_for_contract.
// The ONLY problem was depth1 = String::new() making proofs vacuous.
//
// Fix for `kani_compose.rs`:
//   kani_depth1() → one symbolic char  (covers all 1-char strings)
//   kani_depth2() → current two-symbolic-char impl — already correct
