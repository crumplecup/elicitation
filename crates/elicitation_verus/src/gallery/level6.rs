//! V6 — `Tracked<WfToken>` linear ghost token.
//!
//! Hypothesis: a `proof fn` can mint a `Tracked<V6Token>` witness that
//! a state satisfies `v3_wf`, and another proof fn can extract the state
//! back out, proving the invariant holds.
//! Expected: ✓ proves — confirms the ghost token pattern before applying to
//! production VSM companions.

use vstd::prelude::*;
use verus_builtin_macros::verus;

verus! {

// Required for `>` in spec fn bodies — comparison operators use SpecOrd.
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Ghost invariant witness for a u64-counter state machine.
pub tracked struct V6Token {
    pub ghost counter: u64,
}

/// Invariant on the ghost token.
pub open spec fn v6_tok_wf(tok: &V6Token) -> bool {
    tok.counter > 0
}

/// Mint a fresh token witnessing that the counter is positive.
pub proof fn v6_mint(counter: u64) -> (tracked tok: V6Token)
    requires counter > 0,
    ensures
        v6_tok_wf(&tok),
        tok.counter == counter,
{
    V6Token { counter }
}

/// Advance token: caller supplies the new counter value (must be positive).
/// This avoids proof-mode integer arithmetic (`int` vs `u64` mismatch).
pub proof fn v6_advance(tracked tok: V6Token, new_counter: u64) -> (tracked new_tok: V6Token)
    requires
        v6_tok_wf(&tok),
        new_counter > 0,
    ensures
        v6_tok_wf(&new_tok),
        new_tok.counter == new_counter,
{
    V6Token { counter: new_counter }
}

/// Consume token and recover the witnessed counter value.
pub proof fn v6_consume(tracked tok: V6Token) -> (counter: u64)
    requires v6_tok_wf(&tok),
    ensures counter > 0,
{
    tok.counter
}

} // verus!
