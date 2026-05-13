//! Gallery level 14: inline `tracing::debug!()` / `tracing::info!()` and
//! `goto-instrument` hang.
//!
//! Level 13 established that `#[tracing::instrument]` on callee functions
//! causes `goto-instrument --dfcc` to hang: the attribute injects a tracing
//! span (thread-local lookup, atomic ref-count, drop-glue) into every function
//! body, and CBMC's formula for that symbolic machinery is intractable.
//!
//! Level 14 shows that **inline event macros** (`tracing::debug!`,
//! `tracing::info!`, `tracing::warn!`, `tracing::error!`) in callee function
//! bodies suffer the **same failure mode**.  The macros are less invasive than
//! `#[instrument]` (no span allocation), but they still call into the global
//! tracing dispatcher via thread-local storage, and they can trigger heap
//! allocation through `format!("{:?}", value)` for `?field` arguments.
//! Either path gives CBMC unbounded symbolic branches and the formula never
//! closes.
//!
//! The practical trigger that surfaced this in `strictly_blackjack`:
//!
//! ```rust,ignore
//! // ledger.rs — BankrollLedger::settle() — UNGATED
//! tracing::debug!(
//!     bet          = self.bet,
//!     outcome      = ?outcome,   // ← Debug format → format! → heap alloc
//!     "Payout settled"
//! );
//! ```
//!
//! `bj_place_bet` and `bj_dealer_turn` both call `BankrollLedger::settle()`.
//! `goto-instrument --dfcc` inlines `settle()` while building the DFCC
//! instrumentation wrapper, encounters the ungated `tracing::debug!`, and
//! runs at 99 % CPU indefinitely.  The same applies to the three inline
//! `tracing::debug!` calls in `Shoe::new()`, `Shoe::from_ordered()`, and
//! `Shoe::reshuffle()`.
//!
//! Sibling transitions `bj_player_action` and `bj_restart` do **not** call
//! `BankrollLedger::settle()` and pass in ~8 s, confirming the diagnosis.
//!
//! ## Fix
//!
//! Gate every inline tracing event in a function body with `#[cfg(not(kani))]`:
//!
//! ```rust
//! // Before (hangs goto-instrument):
//! tracing::debug!(outcome = ?outcome, "Payout settled");
//!
//! // After (compiles to nothing under kani):
//! #[cfg(not(kani))]
//! tracing::debug!(outcome = ?outcome, "Payout settled");
//! ```
//!
//! This is analogous to gating `#[instrument]` via
//! `#[cfg_attr(not(kani), tracing::instrument(…))]`.
//!
//! ## Why `#[instrument]` was caught first
//!
//! `#[formal_method]` automatically gates `#[instrument]` on all VSM
//! transition functions (see `formal_method.rs`).  That fixed the contracted
//! functions themselves.  Level 13 then caught ungated callees with explicit
//! `#[instrument]` attrs.  Level 14 catches the residual category: callees
//! that never had `#[instrument]` but still emit inline tracing events.
//!
//! ## Experiment table
//!
//! | ID  | Callee body                              | Expected result      |
//! |-----|------------------------------------------|----------------------|
//! | 14a | `tracing::debug!(?val)` ungated          | goto-instrument hang |
//! | 14b | `#[cfg(not(kani))] tracing::debug!(?val)` | ~8 s (fast)         |
//!
//! ## Run commands
//!
//! ```bash
//! # 14a: expect hang — send SIGINT after ~2 minutes to confirm
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery14a_ungated_debug
//!
//! # 14b: expect ~8 s
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery14b_gated_debug
//! ```

// ── State type (reuse the minimal two-variant unit enum from Level 13) ────────

#[cfg(kani)]
use elicitation::KaniCompose as _;

/// Outcome enum with a Debug-formatted field — mirrors `strictly_blackjack::Outcome`.
/// The `?val` field in `tracing::debug!` triggers `format!("{:?}", val)`, which
/// allocates a heap String that CBMC models symbolically.
#[cfg(kani)]
#[derive(Debug, Clone, Copy)]
enum G14Outcome {
    Win,
    Loss,
    Push,
}

#[cfg(kani)]
impl elicitation::KaniCompose for G14Outcome {
    fn kani_depth0() -> Self {
        G14Outcome::Win
    }
    fn kani_depth1() -> Self {
        G14Outcome::Loss
    }
    fn kani_depth2() -> Self {
        G14Outcome::Push
    }
    fn kani_any() -> Self {
        match kani::any::<u8>() % 3 {
            0 => G14Outcome::Win,
            1 => G14Outcome::Loss,
            _ => G14Outcome::Push,
        }
    }
}

/// Trivially-true invariant — keeps contract cost near zero so any slowdown
/// is attributable to the inline tracing event.
#[cfg(kani)]
fn g14_consistent(_outcome: &G14Outcome) -> bool {
    true
}

// ── L14a: callee with ungated tracing::debug!(?val) ──────────────────────────
//
// `g14a_callee` emits a plain `tracing::debug!` with a `?outcome` field.
// The `?` sigil means Debug-format: the macro expands to (roughly):
//
//   if DISPATCHER.is_enabled(Level::DEBUG) {
//       tracing::event!(Level::DEBUG, outcome = ?outcome, "settled");
//   }
//
// The dispatcher check accesses a thread-local `AtomicUsize`; the format path
// calls `format_args!("{:?}", outcome)` which may heap-allocate.  CBMC models
// the dispatcher state symbolically → formula explosion.

#[cfg(kani)]
fn g14a_callee(outcome: G14Outcome) -> G14Outcome {
    let result = match outcome {
        G14Outcome::Win => G14Outcome::Win,
        G14Outcome::Loss => G14Outcome::Loss,
        G14Outcome::Push => G14Outcome::Push,
    };
    tracing::debug!(outcome = ?outcome, "G14a settled");
    result
}

#[cfg(kani)]
#[kani::requires(g14_consistent(&outcome))]
#[kani::ensures(|r: &G14Outcome| g14_consistent(r))]
fn g14a_contracted(outcome: G14Outcome) -> G14Outcome {
    std::mem::forget(outcome);
    g14a_callee(G14Outcome::Win)
}

// ── L14b: callee with #[cfg(not(kani))] tracing::debug!(?val) ─────────────────
//
// Under kani the entire macro invocation is compiled out.  The callee body
// reduces to a trivial match expression — DFCC sees no tracing overhead at all.
//
// RESULT: ~8 s — matches the Level 13b gated-instrument baseline.

#[cfg(kani)]
fn g14b_callee(outcome: G14Outcome) -> G14Outcome {
    let result = match outcome {
        G14Outcome::Win => G14Outcome::Win,
        G14Outcome::Loss => G14Outcome::Loss,
        G14Outcome::Push => G14Outcome::Push,
    };
    #[cfg(not(kani))]
    tracing::debug!(outcome = ?outcome, "G14b settled");
    result
}

#[cfg(kani)]
#[kani::requires(g14_consistent(&outcome))]
#[kani::ensures(|r: &G14Outcome| g14_consistent(r))]
fn g14b_contracted(outcome: G14Outcome) -> G14Outcome {
    std::mem::forget(outcome);
    g14b_callee(G14Outcome::Win)
}

// ── Harnesses ─────────────────────────────────────────────────────────────────

// ── 14a: ungated debug! → goto-instrument hang ───────────────────────────────
//
// RESULT (expected): goto-instrument runs at 99 % CPU indefinitely.
// Confirmed by: `timeout 120 cargo kani … --harness gallery14a_ungated_debug`
// producing no output after 2 minutes.

#[cfg(kani)]
#[kani::proof_for_contract(g14a_contracted)]
fn gallery14a_ungated_debug() {
    let outcome = G14Outcome::kani_any();
    std::mem::forget(outcome);
    let outcome = G14Outcome::kani_depth0();
    let _ = g14a_contracted(outcome);
}

// ── 14b: gated debug! → fast ─────────────────────────────────────────────────
//
// RESULT (expected): ~8 s — negligible callee cost.

#[cfg(kani)]
#[kani::proof_for_contract(g14b_contracted)]
fn gallery14b_gated_debug() {
    let outcome = G14Outcome::kani_any();
    std::mem::forget(outcome);
    let outcome = G14Outcome::kani_depth0();
    let _ = g14b_contracted(outcome);
}
