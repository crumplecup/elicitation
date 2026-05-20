//! V8 — `#[verifier::type_invariant]` on a struct.
//!
//! Hypothesis: Verus automatically checks the declared `type_invariant`
//! at every construction and field assignment.  The pattern requires all
//! fields to be private; getters expose them.
//! Expected: ✓ proves.
//!
//! Note: compared to V7's `requires`/`ensures` approach, type invariants
//! provide automatic enforcement but impose stricter encapsulation.

use verus_builtin_macros::verus;
use vstd::prelude::*;

verus! {

#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Phase discriminant for the V8 machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V8Phase {
    Idle,
    Active,
}

/// A struct whose fields are all private — required by `type_invariant`.
#[derive(Debug)]
pub struct V8Machine {
    phase: V8Phase,
    count: u64,
    name:  String,
}

impl V8Machine {
    /// The type invariant: auto-checked at every construction/mutation.
    #[verifier::type_invariant]
    pub closed spec fn wf(self) -> bool {
        match self.phase {
            V8Phase::Active => self.count > 0 && self.name@.len() > 0,
            V8Phase::Idle   => true,
        }
    }

    // Spec accessors: `closed` so the body (which accesses private fields)
    // is not visible outside this module.  Callers can still call these in
    // their own spec expressions; they just cannot see the field value directly.
    pub closed spec fn phase_of(m: &V8Machine) -> V8Phase { m.phase }
    pub closed spec fn count_of(m: &V8Machine) -> u64      { m.count }

    /// Build an Idle machine.  Trivially satisfies the invariant.
    pub fn new_idle() -> (r: V8Machine)
        ensures r.wf(),
    {
        V8Machine { phase: V8Phase::Idle, count: 0, name: String::new() }
    }

    /// Activate: transition to Active with a non-empty name and positive count.
    /// Use a single struct-literal assignment so the invariant is checked at
    /// the end of the function, not after each individual field write.
    pub fn activate(&mut self, name: String, count: u64)
        requires
            name@.len() > 0,
            count > 0,
        ensures self.wf(),
    {
        *self = V8Machine { phase: V8Phase::Active, name, count };
    }

    /// Deactivate: return to Idle.
    pub fn deactivate(&mut self)
        ensures self.wf(),
    {
        *self = V8Machine { phase: V8Phase::Idle, count: 0, name: String::new() };
    }

    // Getters — postconditions use spec accessor fns, not raw field access.
    pub fn phase(&self) -> (r: V8Phase)
        ensures r == Self::phase_of(self),
    { self.phase }

    pub fn count(&self) -> (r: u64)
        ensures r == Self::count_of(self),
    { self.count }
}

} // verus!
