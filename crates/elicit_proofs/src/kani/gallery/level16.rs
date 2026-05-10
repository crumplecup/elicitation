//! Gallery level 16: vacuous passes from all-String-field enums.
//!
//! **Finding**: When all enum variants carry `String` fields and the
//! invariant requires non-empty strings, the `KaniCompose` derive
//! returns `String::new()` at every depth (the `field_exprs` bug in
//! `derive_kani_compose.rs`).
//!
//! The forgive-and-forget harness pattern calls
//! `kani::assume(consistent(&kani_depth2()))`.  When `kani_depth2()`
//! returns a state with `label = ""`, `consistent()` returns `false`,
//! and `assume(false)` kills every symbolic path.  `proof_for_contract`
//! then reports PASS vacuously — no counterexample, but zero real checks.
//!
//! ## Experiments
//!
//! ### 16a — Vacuous PASS (current `field_exprs` String behaviour)
//!
//! State type where all variants have `label: String`.  The manual
//! `KaniCompose` impl mirrors the buggy derive: `String::new()` at
//! every depth.  `assume(consistent(&kani_depth2()))` = `assume(false)`.
//! Zero assertions are reachable → Kani reports PASS.
//!
//! ### 16b — Genuine FAIL (depth-aware `KaniCompose`)
//!
//! Same state but the `KaniCompose` impl uses depth-appropriate strings:
//! depth 1 = one symbolic `char`, depth 2 = two symbolic `char`s.
//! `assume(consistent(&kani_depth2()))` = `assume(true)` (non-empty).
//! The transition receives `String::new()` as label → result violates
//! `consistent` → Kani reports FAIL (real counterexample).
//!
//! ### 16c — Genuine PASS (depth-aware `KaniCompose`, depth1 label)
//!
//! Same depth-aware type, but the transition receives
//! `String::kani_depth1()` (one symbolic char) as label.  Non-empty
//! label passes through → Kani reports PASS (sound, non-vacuous).
//!
//! Run:
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery16a_vacuous
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery16b_genuine_fail
//! cargo kani -p elicit_proofs --lib --features kani --harness gallery16c_genuine_pass
//! ```
//!
//! ## Conclusions and fixes
//!
//! 1. **`derive_kani_compose.rs`** `field_exprs` must use depth-aware String:
//!    - depth 0 → `String::new()`
//!    - depth 1 → `<String as KaniCompose>::kani_depth1()` (1 symbolic char)
//!    - depth 2 → `<String as KaniCompose>::kani_depth2()` (2 symbolic chars)
//!
//! 2. **`kani_gen.rs`** generated harnesses must use `kani_depth1()` for the
//!    pre-state passed to the actual function call (not `kani_depth0()`),
//!    because `kani_depth0()` may violate the invariant (empty String).
//!
//! 3. **`kani_gen.rs`** must emit `<String as KaniCompose>::kani_depth1()`
//!    instead of `String::new()` for `String`-typed transition parameters,
//!    so the symbolic argument satisfies any `!param.is_empty()` requires.

use std::mem::forget;

// ── State type ────────────────────────────────────────────────────────────────

/// Three-variant enum where EVERY variant carries `label: String`.
///
/// Mirrors `LedgerPeriodState` (no unit variant, all variants have String).
#[cfg(kani)]
#[derive(Clone)]
enum G16State {
    Open { label: String },
    Adjusting { label: String },
    Closed { label: String },
}

/// Invariant: every state has a non-empty label.
///
/// Mirrors `ledger_period_consistent`.
#[cfg(kani)]
fn g16_consistent(s: &G16State) -> bool {
    match s {
        G16State::Open { label } | G16State::Adjusting { label } | G16State::Closed { label } => {
            !label.is_empty()
        }
    }
}

/// Identity-like transition: move to `Open` with the supplied label.
///
/// Mirrors `open_period`: the invariant requires the result to have a
/// non-empty label, so the caller must supply a non-empty label.
#[cfg(kani)]
fn g16_transition(pre: G16State, label: String) -> G16State {
    drop(pre);
    G16State::Open { label }
}

// ── 16-Bug: KaniCompose that simulates the current derive bug ─────────────────
//
// `field_exprs` returns `String::new()` at EVERY depth, so all three
// `kani_depth{0,1,2}()` calls return `Open { label: "" }` — violating
// `g16_consistent`.

#[cfg(kani)]
struct G16Bug;

#[cfg(kani)]
impl G16Bug {
    /// Simulates the buggy `kani_depth2()`: returns `Open { label: "" }`.
    fn depth2() -> G16State {
        G16State::Open {
            label: String::new(), // String::new() at every depth — the bug
        }
    }

    /// Simulates the buggy `kani_depth1()` for the pre-state call.
    fn depth1() -> G16State {
        G16State::Adjusting {
            label: String::new(),
        }
    }

    /// Simulates the buggy `kani_depth0()` for the pre-state call.
    fn depth0() -> G16State {
        G16State::Open {
            label: String::new(),
        }
    }
}

// ── 16-Fix: KaniCompose with depth-aware String construction ──────────────────
//
// The corrected `field_exprs` returns:
//   depth 0 → `String::new()`      (still empty — for structs/depth0 only)
//   depth 1 → `String::kani_depth1()` (one symbolic char — non-empty)
//   depth 2 → `String::kani_depth2()` (two symbolic chars — non-empty)
//
// With this fix, `kani_depth1/2()` for G16State return non-empty labels,
// so `g16_consistent(&kani_depth2())` = `true`.

#[cfg(kani)]
struct G16Fix;

#[cfg(kani)]
impl G16Fix {
    /// Fixed `kani_depth2()`: two symbolic chars → non-empty label.
    fn depth2() -> G16State {
        G16State::Open {
            label: <String as elicitation::KaniCompose>::kani_depth2(),
        }
    }

    /// Fixed `kani_depth1()`: one symbolic char → non-empty label.
    fn depth1() -> G16State {
        G16State::Adjusting {
            label: <String as elicitation::KaniCompose>::kani_depth1(),
        }
    }

    /// depth0 is still empty — but NOT used in the actual call in the fix.
    #[allow(dead_code)]
    fn depth0() -> G16State {
        G16State::Open {
            label: String::new(),
        }
    }
}

// ── 16a: vacuous PASS ─────────────────────────────────────────────────────────
//
// The buggy harness pattern:
//   assume(g16_consistent(&kani_depth2()))
//     = assume(g16_consistent(&Open { label: "" }))
//     = assume(false)                         ← kills ALL paths
//
// Kani reports PASS because there are zero reachable assertions.
// The kani::assert is never reached — this PASS is unsound.

/// Expected: PASS (vacuously — `assume(false)` eliminates all paths).
#[cfg(kani)]
#[kani::proof]
fn gallery16a_vacuous() {
    // forgive-and-forget: tell CBMC a valid state "exists" (but here it doesn't!)
    let _pre = G16Bug::depth2(); // Open { label: "" } — violates invariant
    kani::assume(g16_consistent(&_pre)); // assume(false) — kills everything
    forget(_pre);

    // actual call (unreachable because all paths were killed above)
    let pre = G16Bug::depth0();
    let result = g16_transition(pre, String::new());
    // This assertion is NEVER checked — Kani says PASS vacuously.
    kani::assert(
        g16_consistent(&result),
        "16a: vacuous (should NOT pass soundly)",
    );
    forget(result);
}

// ── 16b: genuine FAIL ─────────────────────────────────────────────────────────
//
// Fixed forgive-and-forget: depth2() = 2-char symbolic string → non-empty.
//   assume(g16_consistent(&G16Fix::depth2()))
//     = assume(!<2-char>.is_empty())
//     = assume(true)                          ← paths survive!
//
// Actual call pre-state: depth1() = 1-char symbolic string → non-empty.
// Transition parameter: String::new() → result has empty label.
// Postcondition: g16_consistent(&Open { label: "" }) = false → FAIL.

/// Expected: FAIL (genuine postcondition violation — empty label in result).
#[cfg(kani)]
#[kani::proof]
fn gallery16b_genuine_fail() {
    // forgive-and-forget with non-empty depth2 string
    let _pre = G16Fix::depth2(); // Open { label: <2 symbolic chars> }
    kani::assume(g16_consistent(&_pre)); // assume(true) — paths survive!
    forget(_pre);

    // actual call: pre-state has non-empty label (satisfies invariant)
    let pre = G16Fix::depth1(); // Adjusting { label: <1 symbolic char> }
    kani::assume(g16_consistent(&pre)); // extra safety: explicitly assume valid pre

    // transition receives empty label — violates postcondition
    let result = g16_transition(pre, String::new());
    kani::assert(
        g16_consistent(&result),
        "16b: should FAIL — empty label in result",
    );
    forget(result);
}

// ── 16c: genuine PASS ─────────────────────────────────────────────────────────
//
// Same depth-aware setup, but the transition receives
// `String::kani_depth1()` (one symbolic char) as label.
// Result: `Open { label: <1 char> }` — non-empty → PASS.
//
// This demonstrates the CORRECT generated harness pattern:
//   - forgive-and-forget uses kani_depth2() with non-empty strings
//   - actual call uses kani_depth1() (non-empty pre-state)
//   - String parameters use kani_depth1() (non-empty symbolic char)

/// Expected: PASS (sound — non-empty label passes through to result).
#[cfg(kani)]
#[kani::proof]
fn gallery16c_genuine_pass() {
    // forgive-and-forget with non-empty depth2 string
    let _pre = G16Fix::depth2();
    kani::assume(g16_consistent(&_pre));
    forget(_pre);

    // actual call: depth1 pre-state (non-empty label)
    let pre = G16Fix::depth1();
    kani::assume(g16_consistent(&pre));

    // transition receives a 1-char symbolic label — never empty
    let label = <String as elicitation::KaniCompose>::kani_depth1();
    let result = g16_transition(pre, label);
    kani::assert(
        g16_consistent(&result),
        "16c: should PASS — non-empty label in result",
    );
    forget(result);
}
