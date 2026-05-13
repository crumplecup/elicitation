//! Tests for [`FormalMethod`] and the blanket impl.
//!
//! Verifies that:
//! - Any `Fn(In, Established<PIn>) -> (Out, Established<POut>)` satisfies
//!   `FormalMethod` automatically.
//! - Proof tokens flow correctly through composition.
//! - Zero-cost: `Established` tokens contribute no runtime size.

use elicitation::{Established, FormalMethod, Prop, contracts::ProvableFrom};

// ── Propositions used in tests ────────────────────────────────────────────────

#[derive(Prop)]
struct Validated;

#[derive(Prop)]
struct Normalised;

#[derive(Prop)]
struct Stored;

// ── Helper: a credential that mints Validated ─────────────────────────────────

struct ValidInput;
impl ProvableFrom<ValidInput> for Validated {}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn plain_fn_satisfies_formal_method_bound() {
    fn normalise(s: String, _proof: Established<Validated>) -> (String, Established<Normalised>) {
        (s.trim().to_lowercase(), Established::assert())
    }

    // The bound check: normalise must implement FormalMethod<…>
    fn assert_formal<F: FormalMethod<String, Validated, String, Normalised>>(_f: &F) {}
    assert_formal(&normalise);
}

#[test]
fn closure_satisfies_formal_method_bound() {
    let normalise =
        |s: String, _proof: Established<Validated>| -> (String, Established<Normalised>) {
            (s.to_uppercase(), Established::assert())
        };

    fn assert_formal<F: FormalMethod<String, Validated, String, Normalised>>(_f: &F) {}
    assert_formal(&normalise);
}

#[test]
fn call_formal_produces_correct_output() {
    fn process(n: u32, _proof: Established<Validated>) -> (u32, Established<Normalised>) {
        (n * 2, Established::assert())
    }

    let proof_in = Established::<Validated>::prove(&ValidInput);
    let (result, _proof_out) = process.call_formal(21, proof_in);
    assert_eq!(result, 42);
}

#[test]
fn formal_methods_compose_proof_flows_through() {
    // Step 1: Validated → Normalised
    fn normalise(s: String, _proof: Established<Validated>) -> (String, Established<Normalised>) {
        (s.trim().to_string(), Established::assert())
    }

    // Step 2: Normalised → Stored
    fn store(s: String, _proof: Established<Normalised>) -> (usize, Established<Stored>) {
        (s.len(), Established::assert())
    }

    let validated = Established::<Validated>::prove(&ValidInput);
    let (normalised_val, normalised_proof) =
        normalise.call_formal("  hello  ".to_string(), validated);
    let (len, _stored_proof) = store.call_formal(normalised_val, normalised_proof);

    assert_eq!(len, 5);
}

#[test]
fn proof_tokens_are_zero_sized() {
    let p = Established::<Validated>::assert();
    assert_eq!(std::mem::size_of_val(&p), 0);

    let q = Established::<Normalised>::assert();
    assert_eq!(std::mem::size_of_val(&q), 0);
}

#[test]
fn harness_defaults_return_empty_token_stream() {
    type F = fn(String, Established<Validated>) -> (String, Established<Normalised>);
    // Default harness methods return empty streams — no panic expected.
    let _ = <F as FormalMethod<String, Validated, String, Normalised>>::kani_harness();
    let _ = <F as FormalMethod<String, Validated, String, Normalised>>::verus_harness();
    let _ = <F as FormalMethod<String, Validated, String, Normalised>>::creusot_harness();
}
