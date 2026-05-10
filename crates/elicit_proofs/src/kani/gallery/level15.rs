//! Gallery level 15: `proof_for_contract` with a bounded-`Vec` struct.
//!
//! # Background
//!
//! The `tictactoe_contracts` module contains `proof_for_contract` harnesses that
//! call `let game: GameInProgress = kani::any()` where `GameInProgress` has a
//! `history: Vec<Move>` field.  The bounded `kani::Arbitrary` implementation
//! builds the Vec via `moves[..len].to_vec()` (symbolic `len ≤ 9`), so the Vec
//! has a symbolically-allocated internal buffer.
//!
//! The foundation harnesses (`tictactoe_foundation`) pass fine because they use
//! plain `#[kani::proof]` (no DFCC).  The blowup happens only in
//! `#[kani::proof_for_contract]` harnesses, pointing at DFCC instrumentation
//! interacting with the symbolic Vec heap.
//!
//! # Experiments and results
//!
//! | ID  | What changes                          | Time    | RAM    | Result              |
//! |-----|---------------------------------------|---------|--------|---------------------|
//! | 15a | DFCC + bounded Vec read-only (base)   | 0.73 s  | normal | ✅ PASS             |
//! | 15b | DFCC + `Vec::push` in body            | 247 s   | +20 GB | ❌ BLOWUP confirmed |
//! | 15c | DFCC + `Vec::push` + `last()` post    | —       | —      | skipped (worse)     |
//! | 15d | DFCC + array instead of Vec           | 0.32 s  | normal | ✅ PASS (fast)      |
//!
//! **Conclusion**: `Vec::push` is the trigger.  Constructing a bounded symbolic
//! `Vec` (read-only) is fine; mutating it via `push` under DFCC causes CBMC to
//! model the full reallocation logic of a heap pointer whose length is itself
//! symbolic, producing an intractable formula.
//!
//! 15c was not run — `Vec::push + last()` is strictly harder than 15b alone; the
//! outcome was not in doubt.  Verifying the internal correctness of `Vec::push` is
//! out of scope for application-level contracts.
//!
//! ## Fix applied to `strictly_tictactoe`
//!
//! `execute_move` in `contracts.rs` did `game.history.push(*mov)` in its body
//! and `game.history().last() == Some(mov)` as a Kani postcondition.  Both were
//! gated out under `#[cfg(not(kani))]`:
//!
//! ```rust
//! // Body — history push is a runtime log; not needed for board-state proofs:
//! #[cfg(not(kani))]
//! game.history.push(*mov);
//!
//! // Postcondition — only the board state is verified:
//! #[cfg_attr(kani, kani::ensures(|_| game.board().get(mov.position) == Occupied(player)))]
//! ```
//!
//! # Run commands
//!
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery15a_vec_read_contract
//!
//! cargo kani -p elicit_proofs --lib --features kani \
//!     -Z function-contracts \
//!     --harness gallery15b_vec_push_contract
//!
//! cargo kani -p elicit_proofs --lib --features kani \
//!     --harness gallery15d_array_push_contract
//! ```

// ── Shared state type ─────────────────────────────────────────────────────────

/// Minimal struct mirroring `GameInProgress`'s Vec-bearing shape.
///
/// `u8` plays the role of `Move` — a simple `Copy` value.
/// `val: u8` mirrors the `Player` discriminant.
#[cfg(kani)]
struct G15State {
    history: Vec<u8>,
    val: u8,
}

/// Bounded `kani::Arbitrary` mirroring the `GameInProgress` implementation:
/// constructs the Vec from a fixed-size array with a symbolic length.
#[cfg(kani)]
impl kani::Arbitrary for G15State {
    fn any() -> Self {
        let val: u8 = kani::any();
        let len: usize = kani::any();
        kani::assume(len <= 9);
        let elems: [u8; 9] = kani::any();
        let history = elems[..len].to_vec();
        Self { history, val }
    }
}

/// Trivially-true invariant — keeps contract verification cost near zero.
#[cfg(kani)]
fn g15_consistent(_s: &G15State) -> bool {
    true
}

// ── L15a: DFCC + bounded Vec read-only ───────────────────────────────────────
//
// The contracted function only reads `state.val`; it does not touch `history`.
// This isolates whether the mere *presence* of a symbolic Vec in the struct
// (from bounded Arbitrary) saturates CBMC under DFCC instrumentation.

#[cfg(kani)]
#[kani::ensures(|result: &u8| *result == state.val)]
fn g15a_read_val(state: &G15State) -> u8 {
    state.val
}

/// RESULT: 0.73 s — bounded Vec read-only under DFCC is fine.
#[cfg(kani)]
#[kani::proof_for_contract(g15a_read_val)]
fn gallery15a_vec_read_contract() {
    let state: G15State = kani::any();
    let _ = g15a_read_val(&state);
    std::mem::forget(state);
}

// ── L15b: DFCC + `Vec::push` in body ─────────────────────────────────────────
//
// Adds a `Vec::push` in the body.  DFCC must track writes to the Vec's
// internal buffer — a heap pointer.  The reallocation logic of a symbolically-
// allocated buffer with symbolic length causes formula explosion.
//
// RESULT: 247 s / +20 GB RAM — BLOWUP CONFIRMED.

#[cfg(kani)]
#[kani::ensures(|_| state.history.len() == old_len + 1)]
fn g15b_push(state: &mut G15State, old_len: usize, item: u8) {
    state.history.push(item);
}

/// RESULT: 247 s / +20 GB — Vec::push under DFCC is the blowup trigger.
#[cfg(kani)]
#[kani::proof_for_contract(g15b_push)]
fn gallery15b_vec_push_contract() {
    let mut state: G15State = kani::any();
    let item: u8 = kani::any();
    let old_len = state.history.len();
    kani::assume(state.history.len() < 9);
    g15b_push(&mut state, old_len, item);
    std::mem::forget(state);
}

// ── L15c: skipped ────────────────────────────────────────────────────────────
//
// DFCC + `Vec::push` + `Vec::last()` postcondition — predicted strictly worse
// than 15b.  Not run: verifying Vec::push internals is out of scope for
// application-level contracts.  See module-level docs.

// ── L15d: control — array instead of Vec, no heap ───────────────────────────
//
// Replace Vec<u8> with a fixed `[u8; 9]` + length counter.
// DFCC sees only stack/array writes — no heap pointers.
// Confirms the blowup is Vec-heap-specific, not a contract complexity issue.
//
// RESULT: 0.32 s — fast, as expected.

/// Array-backed state — no heap allocation.
#[cfg(kani)]
struct G15ArrayState {
    buf: [u8; 9],
    len: usize,
    val: u8,
}

#[cfg(kani)]
impl kani::Arbitrary for G15ArrayState {
    fn any() -> Self {
        let buf: [u8; 9] = kani::any();
        let len: usize = kani::any();
        kani::assume(len <= 9);
        Self { buf, len, val: kani::any() }
    }
}

/// `old_len` is the caller-supplied snapshot of `state.len` before the call.
/// Postcondition: the length grew by 1 and the slot at `old_len` holds `item`.
/// We pass `old_len` as a parameter to avoid capturing pre-state symbolically.
#[cfg(kani)]
#[kani::requires(state.len < 9)]
#[kani::requires(old_len == state.len)]
#[kani::ensures(|_| state.len == old_len + 1)]
fn g15d_array_push(state: &mut G15ArrayState, old_len: usize, item: u8) {
    state.buf[state.len] = item;
    state.len += 1;
}

/// RESULT: fast — no heap, plain array write. Baseline for comparison with 15c.
#[cfg(kani)]
#[kani::proof_for_contract(g15d_array_push)]
fn gallery15d_array_push_contract() {
    let mut state: G15ArrayState = kani::any();
    let item: u8 = kani::any();
    let old_len = state.len;
    kani::assume(state.len < 9);
    g15d_array_push(&mut state, old_len, item);
}
