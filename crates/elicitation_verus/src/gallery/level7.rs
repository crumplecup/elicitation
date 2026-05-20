//! V7 — Full VSM pattern.
//!
//! Hypothesis: a multi-variant enum with `String` + `u64` fields is fully
//! covered by a single `pub open spec fn wf` pattern-match, and multiple
//! transitions all satisfy that invariant — no per-variant boilerplate.
//! This is the canonical shape for production VSM companions.
//! Expected: ✓ proves.

#[allow(unused_imports)]
use vstd::prelude::*;
use verus_builtin_macros::verus;

verus! {

// Required for `>` in spec fn bodies on u64.
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// A four-phase state machine with labeled loading and a result count.
#[derive(Debug)]
pub enum V7State {
    Initial,
    Loading { label: String },
    Loaded  { label: String, count: u64 },
    Failed  { message: String },
}

/// Single `pub open spec fn` covers all variants — the VSM production shape.
pub open spec fn v7_wf(s: &V7State) -> bool {
    match s {
        V7State::Initial             => true,
        V7State::Loading { label }   => label@.len() > 0,
        V7State::Loaded { label, count } =>
            label@.len() > 0 && *count > 0,
        V7State::Failed { message }  => message@.len() > 0,
    }
}

/// Begin loading: Initial → Loading.
pub fn v7_begin_loading(state: V7State, label: String) -> (r: V7State)
    requires
        v7_wf(&state),
        state matches V7State::Initial,
        label@.len() > 0,
    ensures
        v7_wf(&r),
        r matches V7State::Loading { .. },
{
    let _ = state;
    V7State::Loading { label }
}

/// Finish loading: Loading → Loaded.
pub fn v7_finish_loading(state: V7State, count: u64) -> (r: V7State)
    requires
        v7_wf(&state),
        state matches V7State::Loading { .. },
        count > 0,
    ensures
        v7_wf(&r),
        r matches V7State::Loaded { .. },
{
    match state {
        V7State::Loading { label } => V7State::Loaded { label, count },
        other => other,
    }
}

/// Fail: any valid state → Failed with a non-empty message.
pub fn v7_fail(state: V7State, message: String) -> (r: V7State)
    requires
        v7_wf(&state),
        message@.len() > 0,
    ensures
        v7_wf(&r),
        r matches V7State::Failed { .. },
{
    let _ = state;
    V7State::Failed { message }
}

/// Reset: any valid state → Initial.
pub fn v7_reset(state: V7State) -> (r: V7State)
    requires v7_wf(&state),
    ensures
        v7_wf(&r),
        r matches V7State::Initial,
{
    let _ = state;
    V7State::Initial
}

} // verus!
