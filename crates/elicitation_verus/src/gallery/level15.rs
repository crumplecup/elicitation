//! V15 — Verus companion is isolated from exec function body (including tracing spans).
//!
//! **Hypothesis**: When `formal_method` injects a `tracing::info_span!` call
//! into the public function body (V9 pattern), the generated `__verus` companion
//! is a separate function that delegates via `fn_name(args)` — Verus never
//! inlines or inspects the exec body.
//!
//! This test simulates the pattern without the `tracing` dep (unavailable in
//! the Verus standalone toolchain invocation) by using a plain exec body.
//! The structural isolation is identical: companion calls exec, does not inline it.
//!
//! Expected: ✓ proves.

use verus_builtin_macros::verus;
#[allow(unused_imports)]
use vstd::prelude::SpecOrd;

/// Exec function with an opaque body — simulates a function whose body
/// contains a tracing span. Marked external so Verus ignores the body.
#[verifier::external]
pub fn v15_exec(x: u64) -> u64 {
    // In production this would be: let _span = tracing::info_span!(...).entered();
    x.saturating_add(1)
}

verus! {

/// Attach a trusted spec — mirrors what formal_method __verus companion does.
/// No requires needed: just ensures the result is positive.
pub assume_specification[v15_exec](x: u64) -> (r: u64)
    ensures r > 0;

/// Companion caller: Verus verifies this using the spec alone.
/// It never sees v15_exec's body (which would contain the tracing span).
pub fn v15_companion(x: u64) -> (r: u64)
    ensures r > 0,
{
    v15_exec(x)
}

} // verus!
