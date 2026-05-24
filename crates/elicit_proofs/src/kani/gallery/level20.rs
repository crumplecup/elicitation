//! Gallery level 20: closure-architecture canary for `apply_filter`-style transitions.
//!
//! This level mirrors the important parts of the production `apply_filter`
//! failure:
//! - enum state with a unit/default base variant,
//! - a `NavReady | NavFiltered` style branch,
//! - a by-value `String` parameter,
//! - a filtered-state invariant that requires `!filter.is_empty()`,
//! - forgive-and-forget closure setup.
//!
//! The goal is to determine whether the gap is in:
//! 1. the shadow-state depth (`depth0` vs `depth1`), or
//! 2. something more specific in the production codegen around `apply_filter`.

use elicitation::KaniCompose;

#[derive(KaniCompose)]
pub enum G20State {
    Unloaded,
    Ready {
        items: Vec<u8>,
        cursor: usize,
        filter: String,
        filter_active: bool,
        show_help: bool,
    },
    Filtered {
        items: Vec<u8>,
        filter: String,
        cursor: usize,
    },
}

pub fn g20_consistent(state: &G20State) -> bool {
    match state {
        G20State::Filtered { filter, .. } => !filter.is_empty(),
        _ => true,
    }
}

#[cfg_attr(kani, kani::requires(g20_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_consistent(r)))]
pub fn g20_apply_filter(state: G20State, filter: String) -> G20State {
    match state {
        G20State::Ready { items, .. } | G20State::Filtered { items, .. } => {
            if filter.is_empty() {
                G20State::Ready {
                    items,
                    cursor: 0,
                    filter: String::new(),
                    filter_active: true,
                    show_help: false,
                }
            } else {
                G20State::Filtered {
                    items,
                    filter,
                    cursor: 0,
                }
            }
        }
        other => other,
    }
}

/// Minimal passthrough: drops `filter` and returns `state` unchanged.
#[cfg_attr(kani, kani::requires(g20_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_consistent(r)))]
pub fn g20_drop_arg_passthrough(state: G20State, filter: String) -> G20State {
    let _ = filter.len();
    state
}

#[derive(KaniCompose)]
pub enum G20MiniState {
    Unloaded,
    Ready,
    Filtered { filter: String },
}

pub fn g20_mini_consistent(state: &G20MiniState) -> bool {
    match state {
        G20MiniState::Filtered { filter } => !filter.is_empty(),
        _ => true,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_apply_filter(state: G20MiniState, filter: String) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            if filter.is_empty() {
                G20MiniState::Ready
            } else {
                G20MiniState::Filtered { filter }
            }
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_if_no_move(state: G20MiniState, filter: String) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            let _ = if filter.is_empty() { 0usize } else { 1usize };
            G20MiniState::Ready
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_move_no_if(state: G20MiniState, filter: String) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            let _tmp = G20MiniState::Filtered { filter };
            G20MiniState::Ready
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_move_to_local_no_if(state: G20MiniState, filter: String) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            let _tmp = filter;
            G20MiniState::Ready
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_move_to_tuple_no_if(state: G20MiniState, filter: String) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            let _tmp = (filter,);
            G20MiniState::Ready
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_apply_filter_kernel(state: G20MiniState, filter: &str) -> G20MiniState {
    match state {
        G20MiniState::Ready | G20MiniState::Filtered { .. } => {
            if filter.is_empty() {
                G20MiniState::Ready
            } else {
                G20MiniState::Filtered {
                    filter: filter.to_owned(),
                }
            }
        }
        other => other,
    }
}

#[cfg_attr(kani, kani::requires(g20_mini_consistent(&state)))]
#[cfg_attr(kani, kani::ensures(|r| g20_mini_consistent(r)))]
pub fn g20_mini_apply_filter_via_kernel(state: G20MiniState, filter: String) -> G20MiniState {
    g20_mini_apply_filter_kernel(state, &filter)
}

/// Mirror the current `kani_gen.rs` closure architecture:
/// witness = depth2, actual call state = depth1, String arg = depth1.
#[cfg(kani)]
#[kani::proof_for_contract(g20_apply_filter)]
fn gallery20a_depth1_shadow() {
    let state = G20State::kani_depth2();
    kani::assume(g20_consistent(&state));
    std::mem::forget(state);

    let state = G20State::kani_depth1();
    let filter = String::kani_depth1();
    let result = g20_apply_filter(state, filter);
    std::mem::forget(result);
}

/// Mirror the older/formal_method closure architecture:
/// witness = depth2, actual call state = depth0, String arg = depth0.
#[cfg(kani)]
#[kani::proof_for_contract(g20_apply_filter)]
fn gallery20b_depth0_shadow() {
    let state = G20State::kani_depth2();
    kani::assume(g20_consistent(&state));
    std::mem::forget(state);

    let state = G20State::kani_depth0();
    let filter = String::kani_depth0();
    let result = g20_apply_filter(state, filter);
    std::mem::forget(result);
}

/// Hybrid test: depth0 state shadow but inductive String argument.
///
/// This isolates whether the String arg alone is enough to trigger DFCC in the
/// passthrough branch when the state is the default/unit variant.
#[cfg(kani)]
#[kani::proof_for_contract(g20_apply_filter)]
fn gallery20c_depth0_state_depth1_string() {
    let state = G20State::kani_depth2();
    kani::assume(g20_consistent(&state));
    std::mem::forget(state);

    let state = G20State::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_apply_filter(state, filter);
    std::mem::forget(result);
}

/// Same closure shape as 20c, but with no `apply_filter` branching logic.
///
/// If this still fails, the closure gap is fundamentally
/// "enum state passthrough + dropped inductive String arg", not the nav logic.
#[cfg(kani)]
#[kani::proof_for_contract(g20_drop_arg_passthrough)]
fn gallery20d_passthrough_depth0_state_depth1_string() {
    let state = G20State::kani_depth2();
    kani::assume(g20_consistent(&state));
    std::mem::forget(state);

    let state = G20State::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_drop_arg_passthrough(state, filter);
    std::mem::forget(result);
}

/// Passthrough control: empty String arg should remain DFCC-safe.
#[cfg(kani)]
#[kani::proof_for_contract(g20_drop_arg_passthrough)]
fn gallery20e_passthrough_depth0_state_depth0_string() {
    let state = G20State::kani_depth2();
    kani::assume(g20_consistent(&state));
    std::mem::forget(state);

    let state = G20State::kani_depth0();
    let filter = String::kani_depth0();
    let result = g20_drop_arg_passthrough(state, filter);
    std::mem::forget(result);
}

/// Shrunk `apply_filter` shape: no Vecs, no extra fields, same control flow.
#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_apply_filter)]
fn gallery20f_mini_depth0_state_depth1_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_apply_filter(state, filter);
    std::mem::forget(result);
}

/// Empty-string control for the mini `apply_filter` shape.
#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_apply_filter)]
fn gallery20g_mini_depth0_state_depth0_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth0();
    let result = g20_mini_apply_filter(state, filter);
    std::mem::forget(result);
}

/// Test whether the `if filter.is_empty()` branch alone is enough to trigger the issue.
#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_if_no_move)]
fn gallery20h_mini_if_no_move_depth1_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_if_no_move(state, filter);
    std::mem::forget(result);
}

/// Test whether moving the String into a result-like enum is enough without the `is_empty` branch.
#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_move_no_if)]
fn gallery20i_mini_move_no_if_depth1_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_move_no_if(state, filter);
    std::mem::forget(result);
}

#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_move_to_local_no_if)]
fn gallery20j_mini_move_to_local_no_if_depth1_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_move_to_local_no_if(state, filter);
    std::mem::forget(result);
}

#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_move_to_tuple_no_if)]
fn gallery20k_mini_move_to_tuple_no_if_depth1_string() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_move_to_tuple_no_if(state, filter);
    std::mem::forget(result);
}

#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_apply_filter_kernel)]
fn gallery20l_kernel_depth0_state_depth1_string_borrow() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_apply_filter_kernel(state, &filter);
    std::mem::forget(result);
    std::mem::forget(filter);
}

#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_apply_filter_kernel)]
fn gallery20m_kernel_depth0_state_depth0_string_borrow() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth0();
    let result = g20_mini_apply_filter_kernel(state, &filter);
    std::mem::forget(result);
    std::mem::forget(filter);
}

#[cfg(kani)]
#[kani::proof_for_contract(g20_mini_apply_filter_via_kernel)]
fn gallery20n_via_kernel_depth0_state_depth1_string_owned() {
    let state = G20MiniState::kani_depth2();
    kani::assume(g20_mini_consistent(&state));
    std::mem::forget(state);

    let state = G20MiniState::kani_depth0();
    let filter = String::kani_depth1();
    let result = g20_mini_apply_filter_via_kernel(state, filter);
    std::mem::forget(result);
}
