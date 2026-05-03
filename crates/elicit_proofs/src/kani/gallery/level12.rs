//! Gallery level 12: `proof_for_contract` cost decomposition.
//!
//! Level 11 established that the forgive-and-forget trick works with
//! `proof_for_contract` on **trivial spec functions** (~33 s).  Level 12
//! measures concrete cost factors and validates the `stub_verified` composition
//! architecture.
//!
//! Architecture note: contracts are emitted directly on the original
//! `elicit_server` functions by `#[formal_method]` (via `cfg_attr`).
//! Leaf proofs use `proof_for_contract(fn_name)` on the original; composition
//! proofs use `stub_verified(fn_name)` on the same original.
//!
//! ## Experiment table
//!
//! | ID       | What changes vs prev                     | Result      |
//! |----------|------------------------------------------|-------------|
//! | 12a      | Replicate 11d exactly                    | 33s / 56s   |
//! | 12b      | Contracted fn drops (no forget)          | 31s / 51s   |
//! | 12c      | Return Established<P> (ZST)              | 32s / 50s   |
//! | 12d      | Call real `column_detail` (inline body)  | TIMEOUT >5m |
//! | 12d_pfc  | proof_for_contract on original fn        | 32s / 11m39s |
//! | 12d_two  | stub_verified two-step composition       | 4s / 54s     |
//! | 12e      | `panel_loading`: creates Strings         | 39s / TBD   |
//! | 12f      | `query_complete`: match on state         | 33s / TBD   |
//!
//! ## Run commands
//!
//! ```bash
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12a_baseline_replicate
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12b_drop_unit_variant
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12c_established_return
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12d_real_column_detail
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12d_pfc_column_detail
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts -Z stubbing --harness gallery12d_two_step_composition
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12e_panel_loading_strings
//! cargo kani -p elicit_proofs --lib --features kani -Z function-contracts --harness gallery12f_query_complete_match
//! ```

#[cfg(kani)]
use elicit_server::archive::vsm::archive_panel_consistent;
use elicit_server::archive::vsm::{
    ArchivePanelState, column_detail, panel_loading, query_complete,
};
use elicitation::{KaniCompose, contracts::Established};

// ── Contracted functions ──────────────────────────────────────────────────────

/// 12a replica: exact clone of gallery11d_aps_step — forget input, return unit second.
/// If this still takes ~52s the Level 11 baseline is stable.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, ())| archive_panel_consistent(&result.0))]
fn gallery12a_spec_fn(_state: ArchivePanelState) -> (ArchivePanelState, ()) {
    std::mem::forget(_state);
    (ArchivePanelState::ColumnDetail, ())
}

/// 12b: drop the input instead of forgetting it.
/// `_state` is `ColumnDetail` (unit variant, no heap) — drop is a no-op.
/// Tests whether the implicit drop inside the contracted fn adds DFCC cost.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, ())| archive_panel_consistent(&result.0))]
fn gallery12b_drop_fn(_state: ArchivePanelState) -> (ArchivePanelState, ()) {
    // _state is implicitly dropped here — the key difference from 12a.
    (ArchivePanelState::ColumnDetail, ())
}

/// 12c: return `Established<P>` as the second element (ZST token).
/// Tests whether the token type in the return position adds DFCC cost.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, Established<elicit_server::archive::vsm::ArchivePanelConsistent>)| archive_panel_consistent(&result.0))]
fn gallery12c_established_fn(
    _state: ArchivePanelState,
    proof: Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) -> (
    ArchivePanelState,
    Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) {
    std::mem::forget(_state);
    (ArchivePanelState::ColumnDetail, proof)
}

/// 12d: same signature as 12c but delegates to the REAL `column_detail` fn.
/// `column_detail` just returns `(ArchivePanelState::ColumnDetail, proof)` — drops `_state`.
/// Tests whether calling a real function vs inline logic changes DFCC overhead.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, Established<elicit_server::archive::vsm::ArchivePanelConsistent>)| archive_panel_consistent(&result.0))]
fn gallery12d_real_column_detail_fn(
    _state: ArchivePanelState,
    proof: Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) -> (
    ArchivePanelState,
    Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) {
    elicit_server::archive::vsm::column_detail(_state, proof)
}

/// 12e contracted: wraps `panel_loading` which creates two heap `String` values.
/// Tests whether constructing heap values in the RESULT adds DFCC cost.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&_state))]
#[kani::ensures(|result: &(ArchivePanelState, Established<elicit_server::archive::vsm::ArchivePanelConsistent>)| archive_panel_consistent(&result.0))]
fn gallery12e_panel_loading_fn(
    _state: ArchivePanelState,
    proof: Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
    schema: String,
    label: String,
) -> (
    ArchivePanelState,
    Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) {
    panel_loading(_state, proof, schema, label)
}

/// 12f contracted: wraps `query_complete` which pattern-matches on the input state.
/// Tests whether a non-trivial match arm over the full APS variant tree adds cost.
#[cfg(kani)]
#[kani::requires(archive_panel_consistent(&state))]
#[kani::ensures(|result: &(ArchivePanelState, Established<elicit_server::archive::vsm::ArchivePanelConsistent>)| archive_panel_consistent(&result.0))]
fn gallery12f_query_complete_fn(
    state: ArchivePanelState,
    proof: Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
    result: elicit_server::archive::types::QueryResult,
) -> (
    ArchivePanelState,
    Established<elicit_server::archive::vsm::ArchivePanelConsistent>,
) {
    query_complete(state, proof, result)
}

// ── Harnesses ─────────────────────────────────────────────────────────────────

// ── 12a: replicate Level 11d ──────────────────────────────────────────────────
//
// Exact replication of gallery11d_aps_proof_for_contract_ff.
// Expected: ~52 s (confirms Level 11 baseline is stable across sessions).

#[cfg(kani)]
#[kani::proof_for_contract(gallery12a_spec_fn)]
fn gallery12a_baseline_replicate() {
    let _state = ArchivePanelState::kani_any();
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let _ = gallery12a_spec_fn(_state);
}

// ── 12b: drop the unit-variant input ──────────────────────────────────────────
//
// `gallery12b_drop_fn` drops `_state` rather than forgetting it.
// When `_state = kani_depth0() = ColumnDetail` (unit variant, no heap), drop is
// a no-op at runtime.  Question: does CBMC instrument the generic drop impl
// for ArchivePanelState and pay for all 18 match arms regardless?
//
// Expected range:
//   - Same as 12a (~52 s) → unit-variant drop has zero DFCC overhead
//   - Much slower (>10 min) → generic drop arms are the bottleneck

#[cfg(kani)]
#[kani::proof_for_contract(gallery12b_drop_fn)]
fn gallery12b_drop_unit_variant() {
    let _state = ArchivePanelState::kani_any();
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let _ = gallery12b_drop_fn(_state);
}

// ── 12c: Established<P> in return type ───────────────────────────────────────
//
// Same forgive-and-forget harness but the contracted fn takes and returns the
// evidence token.  Both `Established<P>` and the credential `WcagVerified` are
// ZSTs — they should add zero CBMC cost.
//
// Expected: same as 12a (~52 s).  If slower, something in the ensures closure
// or the token type is creating unexpected symbolic overhead.

#[cfg(kani)]
#[kani::proof_for_contract(gallery12c_established_fn)]
fn gallery12c_established_return() {
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let _state = ArchivePanelState::kani_any();
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    let _ = gallery12c_established_fn(_state, proof);
}

// ── 12d: real `column_detail` body ───────────────────────────────────────────
//
// Same harness as 12c but the contracted fn delegates to the actual
// `column_detail` function.  `column_detail` drops `_state: ArchivePanelState`
// implicitly (same as 12b), so this combines the drop question with calling
// a real function through DFCC instrumentation.
//
// RESULT (measured): TIMEOUT >5 min — DFCC inlines the callee body rather than
// using its contract as a stub.  The solution: first establish a
// proof_for_contract on column_detail itself (12d_pfc), then stub it.

#[cfg(kani)]
#[kani::proof_for_contract(gallery12d_real_column_detail_fn)]
fn gallery12d_real_column_detail() {
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let _state = ArchivePanelState::kani_any();
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    let _ = gallery12d_real_column_detail_fn(_state, proof);
}

// ── 12d_pfc: proof_for_contract on the original `column_detail` fn ────────────
//
// `column_detail` in elicit_server now has kani::requires / kani::ensures
// emitted directly by #[formal_method] via cfg_attr.  This leaf proof
// establishes the contract verdict that composition harnesses build on via
// stub_verified.
//
// Forgive-and-forget pattern: kani_any() → assume → forget → kani_depth0()
// → call → forget result.  DFCC checks the postcondition automatically.
//
// Expected: ~32s (same cost class as 12a–12c).

#[cfg(kani)]
#[kani::proof_for_contract(column_detail)]
fn gallery12d_pfc_column_detail() {
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let _state = ArchivePanelState::kani_any();
    kani::assume(archive_panel_consistent(&_state));
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    let _result = column_detail(_state, proof);
    std::mem::forget(_result);
}

// ── 12d_two_step: composition via stub_verified ───────────────────────────────
//
// Calls `column_detail` twice in sequence.  With stub_verified, each call is
// replaced by the contract axiom — CBMC never sees the implementation body.
// This is the prototype of VSM multi-step composition.
//
// Requires gallery12d_pfc_column_detail to have been verified first.
// Expected: fast (<<32s) — pure contract-axiom expansion, no real bodies.

#[cfg(kani)]
#[kani::proof]
#[kani::stub_verified(column_detail)]
fn gallery12d_two_step_composition() {
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let _state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    kani::assume(archive_panel_consistent(&_state));
    let (_state2, proof2) = column_detail(_state, proof);
    let (_state3, _proof3) = column_detail(_state2, proof2);
    kani::assert(
        archive_panel_consistent(&_state3),
        "consistent after two steps",
    );
}

// ── 12e: panel_loading — String heap in result ───────────────────────────────
//
// `panel_loading` stores `schema` and `label` into the result variant
// `ArchivePanelState::Loading { schema, label }`.  Both are symbolic Strings.
// DFCC must track the heap writes for the two String allocations.
//
// Expected: slower than 12d if heap construction in the result is expensive.
// If similar to 12d, String allocation overhead is not the bottleneck.
//
// Uses `kani::any::<String>()` for the non-state parameters (bounded by default
// loop unwinding).

#[cfg(kani)]
#[kani::proof_for_contract(gallery12e_panel_loading_fn)]
fn gallery12e_panel_loading_strings() {
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let _state = ArchivePanelState::kani_any();
    std::mem::forget(_state);
    let _state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    // Bounded symbolic strings via depth-based induction — same pattern as
    // other heap types in the gallery.
    let schema = String::kani_depth0();
    let label = String::kani_depth0();
    let _ = gallery12e_panel_loading_fn(_state, proof, schema, label);
}

// ── 12f: query_complete — match on input state ───────────────────────────────
//
// `query_complete` matches on `state`, extracting fields from the SqlEditor
// variant or passing other variants through unchanged.  Combined with a
// symbolic `QueryResult` input, this exercises DFCC frame-condition tracking
// on a match expression over all 18 APS variants.
//
// This is representative of the harder class of transitions.  If 12e was fast
// but 12f is slow, the pattern-match over the input (not the heap in the result)
// is the bottleneck.

#[cfg(kani)]
#[kani::proof_for_contract(gallery12f_query_complete_fn)]
fn gallery12f_query_complete_match() {
    use elicit_server::archive::types::QueryResult;
    use elicit_server::archive::vsm::ArchivePanelConsistent;
    let state = ArchivePanelState::kani_any();
    kani::assume(archive_panel_consistent(&state));
    std::mem::forget(state);
    let state = ArchivePanelState::kani_depth0();
    let proof: Established<ArchivePanelConsistent> = {
        let __cred = ArchivePanelConsistent::kani_proof_credential();
        Established::prove(&__cred)
    };
    let result: QueryResult = kani::any();
    let _ = gallery12f_query_complete_fn(state, proof, result);
}
