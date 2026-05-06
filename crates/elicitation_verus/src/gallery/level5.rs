//! V5 — String fields with vstd `View` specs.
//!
//! Hypothesis: `s@.len() > 0` and `s@.len() <= 100` in `requires`/`ensures`
//! clauses work out of the box via `vstd`'s `impl View for String`.
//! No `extern_spec!` needed.
//! Expected: ✓ proves.

use vstd::prelude::*;
use verus_builtin_macros::verus;

verus! {

/// A named state requiring a non-empty, bounded name.
pub struct V5State {
    pub name: String,
}

/// Invariant: name is non-empty and at most 100 characters.
pub open spec fn v5_wf(s: &V5State) -> bool {
    s.name@.len() > 0 && s.name@.len() <= 100
}

/// Constructor: caller must supply a valid name.
pub fn v5_new(name: String) -> (r: V5State)
    requires name@.len() > 0 && name@.len() <= 100,
    ensures
        v5_wf(&r),
        r.name@ == name@,
{
    V5State { name }
}

/// Rename: replace name, preserving the invariant.
pub fn v5_rename(s: V5State, new_name: String) -> (r: V5State)
    requires
        v5_wf(&s),
        new_name@.len() > 0 && new_name@.len() <= 100,
    ensures
        v5_wf(&r),
        r.name@ == new_name@,
{
    V5State { name: new_name }
}

} // verus!
