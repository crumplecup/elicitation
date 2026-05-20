//! V2 — Simple two-variant enum.
//!
//! Hypothesis: Z3 ADT theory handles an `Off`/`On` discriminant in a
//! `pub open spec fn` pattern-match without per-variant harnesses.
//! Expected: ✓ proves.

use verus_builtin_macros::verus;

verus! {

/// A simple two-state toggle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V2State {
    Off,
    On,
}

/// Both variants are valid.
pub open spec fn v2_wf(s: &V2State) -> bool {
    match s {
        V2State::Off => true,
        V2State::On  => true,
    }
}

/// Turn on: only valid from Off, result is always On.
pub fn v2_turn_on(_s: V2State) -> (r: V2State)
    requires s matches V2State::Off,
    ensures
        r matches V2State::On,
        v2_wf(&r),
{
    V2State::On
}

/// Turn off: only valid from On, result is always Off.
pub fn v2_turn_off(_s: V2State) -> (r: V2State)
    requires s matches V2State::On,
    ensures
        r matches V2State::Off,
        v2_wf(&r),
{
    V2State::Off
}

} // verus!
