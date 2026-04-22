//! Tests for the `#[formal_method]` attribute macro.
//!
//! Verifies that:
//! - `#[formal_method]` can be applied to a free function with no args.
//! - `#[formal_method(contracts = [...])]` lists contracts in the doc annotation.
//! - The annotated function still satisfies the `FormalMethod` bound.
//! - The function behaves identically at runtime.
//! - The macro accepts multiple contract types.

use elicitation::{
    Established, FormalMethod, Prop, formal_method,
    contracts::ProvableFrom,
};

// ── Propositions ──────────────────────────────────────────────────────────────

#[derive(Prop)]
struct InputValid;

#[derive(Prop)]
struct OutputNormalised;

// ── Credentials ───────────────────────────────────────────────────────────────

struct InputCredential;
impl ProvableFrom<InputCredential> for InputValid {}

// ── Annotated functions ───────────────────────────────────────────────────────

/// No contracts listed — minimal usage.
#[formal_method]
fn bare_method(
    n: u32,
    _proof: Established<InputValid>,
) -> (u32, Established<OutputNormalised>) {
    (n + 1, Established::assert())
}

/// Single contract declared.
#[formal_method(contracts = [InputValid])]
fn single_contract(
    s: String,
    _proof: Established<InputValid>,
) -> (String, Established<OutputNormalised>) {
    (s.to_uppercase(), Established::assert())
}

/// Multiple contracts declared — both input and output propositions listed.
#[formal_method(contracts = [InputValid, OutputNormalised])]
fn multi_contract(
    n: u32,
    _proof: Established<InputValid>,
) -> (u32, Established<OutputNormalised>) {
    (n * 2, Established::assert())
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[test]
fn annotated_fn_satisfies_formal_method_bound() {
    fn assert_formal<F: FormalMethod<u32, InputValid, u32, OutputNormalised>>(_f: &F) {}
    assert_formal(&bare_method);
    assert_formal(&multi_contract);
}

#[test]
fn annotated_fn_with_contract_satisfies_bound() {
    fn assert_formal<F: FormalMethod<String, InputValid, String, OutputNormalised>>(_f: &F) {}
    assert_formal(&single_contract);
}

#[test]
fn annotated_fn_runs_correctly() {
    let proof = Established::<InputValid>::prove(&InputCredential);
    let (result, _) = bare_method.call_formal(41, proof);
    assert_eq!(result, 42);
}

#[test]
fn multi_contract_fn_runs_correctly() {
    let proof = Established::<InputValid>::prove(&InputCredential);
    let (result, _) = multi_contract.call_formal(21, proof);
    assert_eq!(result, 42);
}

#[test]
fn string_contract_fn_runs_correctly() {
    let proof = Established::<InputValid>::prove(&InputCredential);
    let (result, _) = single_contract.call_formal("hello".to_string(), proof);
    assert_eq!(result, "HELLO");
}
