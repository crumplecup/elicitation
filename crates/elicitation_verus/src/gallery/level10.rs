//! V10 — Two-step proof composition with `Tracked<T>`.
//!
//! Hypothesis: a `Tracked<V10Token>` ghost token can thread the invariant
//! witness through a sequence of two V7-style transitions, confirming that
//! composing production VSM companions works before touching the archive.
//! Expected: ✓ proves.

use verus_builtin_macros::verus;

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Minimal state machine: Initial → Loading → Loaded.
#[derive(Debug)]
pub enum V10State {
    Initial,
    Loading { label: String },
    Loaded  { label: String, count: u64 },
}

/// Invariant for the two-step machine.
pub open spec fn v10_wf(s: &V10State) -> bool {
    match s {
        V10State::Initial             => true,
        V10State::Loading { label }   => label@.len() > 0,
        V10State::Loaded { label, count } =>
            label@.len() > 0 && *count > 0,
    }
}

/// Ghost token that witnesses a state satisfies `v10_wf`.
pub tracked struct V10Token {
    pub ghost state: V10State,
}

/// Mint a fresh token for any well-formed state.
pub proof fn v10_mint(s: V10State) -> (tracked tok: V10Token)
    requires v10_wf(&s),
    ensures
        tok.state == s,
{
    V10Token { state: s }
}

/// Advance token to a new well-formed state.
/// The caller supplies `new_s`; we do not compute in proof mode.
pub proof fn v10_advance(tracked tok: V10Token, new_s: V10State) -> (tracked new_tok: V10Token)
    requires
        v10_wf(&tok.state),
        v10_wf(&new_s),
    ensures
        new_tok.state == new_s,
{
    V10Token { state: new_s }
}

/// Step 1: Initial → Loading.
pub fn v10_step1(_state: V10State, label: String) -> (r: V10State)
    requires
        v10_wf(&state),
        state matches V10State::Initial,
        label@.len() > 0,
    ensures
        v10_wf(&r),
        r matches V10State::Loading { .. },
{
    V10State::Loading { label }
}

/// Step 2: Loading → Loaded.
pub fn v10_step2(state: V10State, count: u64) -> (r: V10State)
    requires
        v10_wf(&state),
        state matches V10State::Loading { .. },
        count > 0,
    ensures
        v10_wf(&r),
        r matches V10State::Loaded { .. },
{
    match state {
        V10State::Loading { label } => V10State::Loaded { label, count },
        other => other,
    }
}

/// Compose tokens through two proof transitions (Initial → Loading → Loaded).
/// Each `v10_advance` call consumes the old token and mints a new one.
pub proof fn v10_proof_compose(
    tracked tok: V10Token,
    s1: V10State,
    s2: V10State,
) -> (tracked final_tok: V10Token)
    requires
        tok.state matches V10State::Initial,
        v10_wf(&tok.state),
        v10_wf(&s1),
        s1 matches V10State::Loading { .. },
        v10_wf(&s2),
        s2 matches V10State::Loaded { .. },
    ensures
        final_tok.state == s2,
{
    let tracked tok1 = v10_advance(tok, s1);
    v10_advance(tok1, s2)
}

/// Exec composition: chains two exec transitions without token threading.
/// V7/V10 validates that Verus can follow the invariant through chained calls.
pub fn v10_exec_compose(
    initial: V10State,
    label: String,
    count: u64,
) -> (r: V10State)
    requires
        v10_wf(&initial),
        initial matches V10State::Initial,
        label@.len() > 0,
        count > 0,
    ensures
        v10_wf(&r),
        r matches V10State::Loaded { .. },
{
    let s1 = v10_step1(initial, label);
    v10_step2(s1, count)
}

} // verus!
