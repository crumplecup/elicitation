//! Gallery level C24: depth-bounded inductive closure for recursive types.
//!
//! **Hypothesis**: Pearlite can express and discharge proof obligations that
//! arise from a depth-bounded guard pattern — the same pattern required to
//! compile self-recursive `elicit()` implementations under Creusot's nightly
//! toolchain.
//!
//! ## Background: the `Send` inference problem
//!
//! Creusot compiles using a dedicated nightly toolchain.  This toolchain
//! applies stricter / lower recursion limits when evaluating `Send` bounds on
//! opaque `impl Future` return types than a typical stable/nightly build does.
//! Two distinct failure modes were discovered while building `cargo creusot`
//! support for `elicit_proofs`.
//!
//! ### Failure mode 1 — mutual recursion (GeoGeometry ↔ GeoGeometryCollection)
//!
//! `GeoGeometry` and `GeoGeometryCollection` are mutually recursive:
//! `GeoGeometry::GeometryCollection(GeoGeometryCollection)` and
//! `GeoGeometryCollection(Vec<GeoGeometry>)`.
//!
//! When `GeoGeometry::elicit()` was an `async fn` with `#[tracing::instrument]`,
//! the compiler wrapped its return in a new opaque `impl Future` and then tried
//! to evaluate `Send` on the wrapper.  That evaluation required
//! `GeoGeometryCollection: Send`, which in turn required `Vec<GeoGeometry>: Send`,
//! which required `GeoGeometry: Send`, which required the future returned by
//! `GeoGeometry::elicit()` to be `Send` — infinite regress.
//!
//! **Fix:** convert `GeoGeometryCollection::elicit()` from `async fn` to an
//! explicit `fn` returning `impl Future + Send { Box::pin(async move {…}) }`,
//! and remove `#[tracing::instrument]` from `GeoGeometry::elicit()`.
//!
//! With `Box::pin`, when checking whether `GeoGeometryCollection::elicit()`'s
//! future is `Send`, the compiler looks at the *declared* return type
//! `impl Future + Send` of `GeoGeometry::elicit()` — it trusts the declaration
//! and does not re-enter the body.  Mutual recursion terminates.
//!
//! This fix is **insufficient** for direct self-recursion (see below).
//!
//! ### Failure mode 2 — direct self-recursion (TomlValue)
//!
//! `TomlValue` contains `Vec<TomlValue>` (Array arm) and
//! `Vec<(String, TomlValue)>` (Table arm).  Its `elicit()` called
//! `Vec::<TomlValue>::elicit()`, which is a generic `async fn impl` that
//! awaits `T::elicit()` for each element.  The Send-check chain becomes:
//!
//! ```text
//! TomlValue::elicit() Send?
//!   → Vec::<TomlValue>::elicit() Send?  (async fn body analysis)
//!     → TomlValue::elicit() Send?       (T::elicit() where T=TomlValue)
//!       → Vec::<TomlValue>::elicit()… ∞
//! ```
//!
//! Even with `Box::pin` and no `#[tracing::instrument]`, the chain re-enters
//! `TomlValue` via the generic `Vec<T>` impl.  The opaque `impl Future + Send`
//! return type forces the compiler to re-evaluate the body each time.
//!
//! **Fix:** introduce a free helper function with a **concrete**
//! `Pin<Box<dyn Future<Output = …> + Send>>` return type:
//!
//! ```rust,ignore
//! fn elicit_toml_value_inner<C: ElicitCommunicator>(
//!     comm: C,
//!     depth: usize,
//! ) -> Pin<Box<dyn Future<Output = ElicitResult<TomlValue>> + Send>> {
//!     Box::pin(async move { … })
//! }
//! ```
//!
//! `Pin<Box<dyn … + Send>>` is a *concrete* type.  Its `Send` bound is
//! structural — `Box<dyn Trait + Send>` is always `Send` by the compiler's
//! blanket rule, with no body analysis required.  When the async block inside
//! `elicit_toml_value_inner` awaits another call to `elicit_toml_value_inner`,
//! the compiler sees `Pin<Box<dyn … + Send>>` at the call site, checks `Send`
//! structurally in O(1), and stops.
//!
//! The array and table arms also bypass the generic `Vec<T>::elicit()` impl
//! entirely, using local helpers with the same concrete return type.
//!
//! ### Why the depth parameter?
//!
//! The depth bound makes the recursion well-founded at runtime: if a TOML
//! document is nested deeper than `TOML_VALUE_MAX_DEPTH` (currently 32), the
//! helper returns an error instead of looping.  It also provides a concrete
//! inductive measure for formal reasoning: the `depth > 0 → result is valid`
//! invariant is exactly what this gallery level proves.
//!
//! ### Summary table
//!
//! | Recursion kind | Root cause | Sufficient fix |
//! |----------------|------------|----------------|
//! | Mutual (A ↔ B) | `async fn` / `instrument` wraps return in new opaque future | Remove `instrument`; `Box::pin` on the collection side |
//! | Direct self (A → Vec<A>) | Generic `async fn` body re-enters A via T | Concrete `Pin<Box<dyn Future + Send>>` helper + bypass generic Vec impl |
//!
//! ## New patterns (relative to C1–C23)
//!
//! | Pattern | Question |
//! |---------|----------|
//! | Depth guard (`depth@ > 0`) | Can Pearlite express the entry precondition? |
//! | Depth decrement (`depth - 1`) | Can Pearlite verify a step preserves `depth@ > 0`? |
//! | Error arm (`depth == 0`) | Can Pearlite prove the zero-depth arm produces None? |
//!
//! ## Run
//!
//! ```bash
//! cargo creusot -p elicitation_creusot
//! ```

use creusot_std::prelude::*;

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A result type mirroring the error/ok shape of depth-bounded elicitation.
pub enum C24Result<T> {
    /// Depth limit was hit; no value produced.
    DepthExceeded,
    /// Elicitation succeeded at the given remaining depth.
    Ok(usize, T),
}

/// Represents one node of a hypothetical recursive elicitation graph.
pub struct C24Node {
    /// Remaining depth budget when this node was produced.
    pub remaining_depth: usize,
    /// The elicited leaf value.
    pub value: i32,
}

// ---------------------------------------------------------------------------
// Logic predicates
// ---------------------------------------------------------------------------

/// The result is valid: elicitation completed within the depth budget.
#[logic]
pub fn c24_is_ok<T>(r: &C24Result<T>) -> bool {
    pearlite! {
        match r {
            C24Result::Ok(_, _) => true,
            C24Result::DepthExceeded => false,
        }
    }
}

/// The depth recorded in an Ok result is strictly less than the input depth.
#[logic]
pub fn c24_depth_decreased(r: &C24Result<C24Node>, input_depth: usize) -> bool {
    pearlite! {
        match r {
            C24Result::Ok(remaining, _node) =>
                remaining@ < input_depth@,
            C24Result::DepthExceeded => true,
        }
    }
}

/// When depth is positive, the result must be Ok.
#[logic]
pub fn c24_depth_ok_implies_result(depth: usize, r: &C24Result<C24Node>) -> bool {
    pearlite! {
        depth@ > 0 ==> c24_is_ok(r)
    }
}

// ---------------------------------------------------------------------------
// Functions
// ---------------------------------------------------------------------------

/// Single-level depth-bounded step.
///
/// Precondition: `depth > 0` — mirrors the guard in `elicit_toml_value_inner`.  
/// Postcondition: the result is `Ok` with remaining depth exactly `depth - 1`.
#[requires(depth@ > 0)]
#[ensures(match &result {
    C24Result::Ok(d, _) => d@ == depth@ - 1,
    C24Result::DepthExceeded => false,
})]
pub fn c24_step(depth: usize, leaf: i32) -> C24Result<C24Node> {
    C24Result::Ok(
        depth - 1,
        C24Node {
            remaining_depth: depth - 1,
            value: leaf,
        },
    )
}

/// Zero-depth arm always produces `DepthExceeded`.
///
/// Mirrors the `if depth == 0 { return Err(…) }` guard at the top of
/// `elicit_toml_value_inner`.
#[requires(depth@ == 0)]
#[ensures(!c24_is_ok(&result))]
pub fn c24_at_limit(depth: usize, _leaf: i32) -> C24Result<C24Node> {
    let _ = depth;
    C24Result::DepthExceeded
}

/// Dispatch: calls `c24_step` when depth is positive, `c24_at_limit` otherwise.
///
/// The full contract — `depth > 0 ==> result is Ok` — is the central
/// invariant of the depth-bounded inductive closure pattern.
#[ensures(c24_depth_ok_implies_result(depth, &result))]
pub fn c24_dispatch(depth: usize, leaf: i32) -> C24Result<C24Node> {
    if depth > 0 {
        c24_step(depth, leaf)
    } else {
        c24_at_limit(depth, leaf)
    }
}

/// Depth decreases at every step: chaining two steps yields remaining depth
/// strictly less than the original.
///
/// This is the formal statement of well-foundedness: the inductive measure
/// (`depth`) strictly decreases at each recursive call, so bounded depth
/// guarantees termination.
#[requires(depth@ >= 2)]
#[ensures(match (&result.0, &result.1) {
    (C24Result::Ok(d1, _), C24Result::Ok(d2, _)) =>
        d1@ < depth@ && d2@ < d1@,
    _ => false,
})]
pub fn c24_two_steps(depth: usize, leaf: i32) -> (C24Result<C24Node>, C24Result<C24Node>) {
    let r1 = c24_step(depth, leaf);
    let r2 = match &r1 {
        C24Result::Ok(d1, _) => c24_step(*d1, leaf),
        C24Result::DepthExceeded => C24Result::DepthExceeded,
    };
    (r1, r2)
}
