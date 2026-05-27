//! Test whether Kani can handle a function body containing a manually-injected
//! `tracing::info_span!` call — the proposed replacement for `#[instrument]`.
//!
//! If Kani can verify `verify_manual_span` successfully, the V9 approach
//! (strip `#[instrument]`, inject span inline) is viable for `#[formal_method]`.

/// A simple function with a manually-injected tracing span (no #[instrument]).
/// This simulates what `formal_method_test_v9` emits for downstream users.
pub fn fn_with_manual_span(x: u32) -> u32 {
    let _tracing_span = tracing::info_span!("fn_with_manual_span").entered();
    x.saturating_add(1)
}

#[kani::proof]
fn verify_manual_span() {
    let x: u32 = kani::any();
    kani::assume(x < u32::MAX);
    let result = fn_with_manual_span(x);
    assert!(result == x + 1);
}
