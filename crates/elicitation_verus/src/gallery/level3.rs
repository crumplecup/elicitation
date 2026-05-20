//! V3 — Enum with a `u64` counter field.
//!
//! Hypothesis: an arithmetic invariant (`counter > 0`) on a data-carrying
//! variant is preserved by a pure-increment transition.
//! Expected: ✓ proves.

use verus_builtin_macros::verus;

verus! {

// Required for `>` in spec fn bodies — comparison operators use SpecOrd.
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// State machine with an optional active counter.
#[derive(Debug, Clone, Copy)]
pub enum V3State {
    Idle,
    Active { counter: u64 },
}

/// Invariant: the counter must be non-zero when active.
pub open spec fn v3_wf(s: &V3State) -> bool {
    match s {
        V3State::Active { counter } => *counter > 0,
        V3State::Idle => true,
    }
}

/// Start: transition from Idle to Active with counter = 1.
pub fn v3_start() -> (r: V3State)
    ensures
        v3_wf(&r),
        r matches V3State::Active { .. },
{
    V3State::Active { counter: 1 }
}

/// Increment: counter advances by 1, invariant preserved.
pub fn v3_increment(s: V3State) -> (r: V3State)
    requires
        v3_wf(&s),
        s matches V3State::Active { .. },
    ensures
        v3_wf(&r),
        r matches V3State::Active { .. },
{
    match s {
        V3State::Active { counter } => {
            // Overflow guard: saturate instead of wrapping.
            let new_counter = if counter < u64::MAX { counter + 1 } else { counter };
            V3State::Active { counter: new_counter }
        }
        V3State::Idle => s,
    }
}

/// Stop: Active → Idle (invariant trivially satisfied).
pub fn v3_stop(_s: V3State) -> (r: V3State)
    requires v3_wf(&s),
    ensures
        v3_wf(&r),
        r matches V3State::Idle,
{
    V3State::Idle
}

} // verus!
