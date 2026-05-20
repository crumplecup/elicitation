//! Diagnostic micro-harnesses for isolating Kani failures.
//!
//! These are NOT part of the VSM suite — they exist to confirm or deny
//! specific theories about which types cause unbounded unwinding.

#[cfg(kani)]
use elicit_db::DbRows;
#[cfg(kani)]
use elicit_server::archive::{display::*, types::*, vsm::*};
#[cfg(kani)]
use elicitation::KaniCompose;

/// Theory A: BTreeMap<String, (f32,f32,f32,f32)> drop causes unbounded unwinding.
#[cfg(kani)]
#[kani::proof]
fn diag_btreemap_drop() {
    let m: std::collections::BTreeMap<String, (f32, f32, f32, f32)> =
        std::collections::BTreeMap::new();
    let _ = m;
}

/// Theory B: kani::any::<Option<ErdLayout>>() hangs.
#[cfg(kani)]
#[kani::proof]
fn diag_option_erd_layout() {
    let _layout: Option<ErdLayout> = kani::any();
}

/// Theory C: kani::any::<ErdDiagramMode>() hangs.
#[cfg(kani)]
#[kani::proof]
fn diag_erd_diagram_mode() {
    let _mode: ErdDiagramMode = kani::any();
}

/// Theory E: Is kani::any::<f32>() itself slow?
#[cfg(kani)]
#[kani::proof]
fn diag_symbolic_f32() {
    let _x: f32 = kani::any();
}

/// Theory F: ErdLayout with concrete floats (no symbolic f32).
#[cfg(kani)]
#[kani::proof]
fn diag_erd_layout_concrete_floats() {
    let _layout = ErdLayout {
        canvas_w: 0.0_f32,
        canvas_h: 0.0_f32,
        boxes: std::collections::BTreeMap::new(),
    };
}

/// Theory G: ErdLayout with ONE symbolic f32.
#[cfg(kani)]
#[kani::proof]
fn diag_erd_layout_one_symbolic_f32() {
    let _layout = ErdLayout {
        canvas_w: kani::any::<f32>(),
        canvas_h: 0.0_f32,
        boxes: std::collections::BTreeMap::new(),
    };
}

/// Theory H: Option<ErdLayout> with manually constructed inner (isolates kani::Arbitrary for Option<T>).
#[cfg(kani)]
#[kani::proof]
fn diag_option_erd_layout_manual() {
    let inner = ErdLayout {
        canvas_w: kani::any::<f32>(),
        canvas_h: kani::any::<f32>(),
        boxes: std::collections::BTreeMap::new(),
    };
    let _layout: Option<ErdLayout> = if kani::any::<bool>() {
        Some(inner)
    } else {
        None
    };
}

/// Theory I: Option<f32> via kani::any() — does kani::Arbitrary for Option<primitive> hang?
#[cfg(kani)]
#[kani::proof]
fn diag_option_f32_arbitrary() {
    let _x: Option<f32> = kani::any();
}

/// Theory J: ExplainNode constructed via kani_depth0() — does the KaniCompose impl hang?
///
/// Previously tested kani::any::<ExplainNode>() but Arbitrary is no longer derived
/// (String fields are not Arbitrary).  kani_depth0() is the correct probe.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_node_arbitrary_alone() {
    use elicitation::KaniCompose;
    let _node: ExplainNode = ExplainNode::kani_depth0();
}

/// Theory K: ExplainNodeMode kani::any() alone — is the enum harmless?
#[cfg(kani)]
#[kani::proof]
fn diag_explain_node_mode_arbitrary() {
    let _mode: ExplainNodeMode = kani::any();
}

/// Theory L: explain_ready with fully concrete inputs (no kani::any on ExplainPlan at all).
///
/// If Theory J confirms kani::any::<ExplainNode>() hangs, this tests whether
/// using concrete inputs bypasses the problem entirely.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_ready_concrete_inputs() {
    let concrete_node = ExplainNode {
        node_type: String::kani_depth1(),
        relation_name: None,
        alias: None,
        startup_cost: 0.0,
        total_cost: 0.0,
        plan_rows: 0,
        plan_width: 0,
        actual_startup_time: None,
        actual_total_time: None,
        actual_rows: None,
        actual_loops: None,
        children: Vec::new(),
    };
    let concrete_plan = ExplainPlan {
        nodes: vec![concrete_node],
        root: 0,
    };
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: concrete_plan.clone(),
        display_mode: ExplainNodeMode::TreeNode,
    };
    let proof = elicitation::contracts::Established::<ArchivePanelConsistent>::assert();
    let _ = explain_ready(
        state,
        proof,
        String::kani_depth1(),
        String::kani_depth1(),
        concrete_plan,
        ExplainNodeMode::TreeNode,
    );
}

// ── ExplainNode field-isolation theories ─────────────────────────────────────
//
// Each theory adds exactly one "symbolic" element to an otherwise fully
// concrete ExplainNode.  Run them in order: the first one that hangs
// identifies the exact field (or interaction) responsible for the timeout.

/// Theory M: ExplainNode with ALL concrete values — zero symbolic inputs.
///
/// This is the baseline.  If it hangs, the issue is CBMC's destructor model
/// for Vec<ExplainNode> itself, regardless of field values.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_node_all_concrete() {
    let _node = ExplainNode {
        node_type: String::kani_depth1(),
        relation_name: None,
        alias: None,
        startup_cost: 0.0_f64,
        total_cost: 0.0_f64,
        plan_rows: 0_i64,
        plan_width: 0_i32,
        actual_startup_time: None,
        actual_total_time: None,
        actual_rows: None,
        actual_loops: None,
        children: Vec::new(),
    };
}

/// Theory N: ExplainNode with one symbolic f64 (`startup_cost`), rest concrete.
///
/// If M passes but N hangs, then symbolic f64 + Vec<ExplainNode> type
/// causes CBMC's destructor analysis to unwind recursively.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_node_one_symbolic_f64() {
    let _node = ExplainNode {
        node_type: String::kani_depth1(),
        relation_name: None,
        alias: None,
        startup_cost: kani::any::<f64>(),
        total_cost: 0.0_f64,
        plan_rows: 0_i64,
        plan_width: 0_i32,
        actual_startup_time: None,
        actual_total_time: None,
        actual_rows: None,
        actual_loops: None,
        children: Vec::new(),
    };
}

/// Theory O: ExplainNode constructed via `kani_depth0()` (symbolic f64/i64/i32).
///
/// If N passes but O hangs, then the issue is inside kani_depth0() itself
/// (e.g. not inlined, CBMC can't propagate Vec::new() through the call).
/// If O also passes, we move to testing the full transition.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_node_kani_depth0() {
    use elicitation::KaniCompose;
    let _node = ExplainNode::kani_depth0();
}

/// Theory P: explain_ready with a non-ExplainView state (takes the `_` arm).
///
/// If this also hangs, the issue is the drop of `ArchivePanelState` (large
/// 18-variant enum).  If it passes, the issue is specific to constructing
/// `ExplainCompare` inside the ExplainView arm.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_ready_non_explain_view() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ColumnDetail;
    let proof = elicitation::contracts::Established::<ArchivePanelConsistent>::assert();
    let plan = ExplainPlan::kani_depth0();
    let mode = ExplainNodeMode::kani_depth0();
    let _ = explain_ready(
        state,
        proof,
        String::kani_depth1(),
        String::kani_depth1(),
        plan,
        mode,
    );
}

/// Theory Q: explain_ready with ExplainView state but drop the result immediately.
///
/// Same as L but assigns to `_` to drop immediately rather than binding the
/// result.  Isolates whether it is the transition body or the result drop.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_ready_explainview_drop_result() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let proof = elicitation::contracts::Established::<ArchivePanelConsistent>::assert();
    let _ = explain_ready(
        state,
        proof,
        String::kani_depth1(),
        String::kani_depth1(),
        ExplainPlan::kani_depth0(),
        ExplainNodeMode::kani_depth0(),
    );
}

/// Theory R: construct and drop ExplainComparison directly (two ExplainPlan fields).
///
/// If this hangs, the problem is CBMC's destructor model for two nested
/// ExplainPlan fields — not the match arm or the function call.
/// If it passes, the explosion is something inside explain_ready specifically.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_comparison_drop() {
    use elicitation::KaniCompose;
    let _cmp = ExplainComparison {
        left: ExplainPlan::kani_depth0(),
        right: ExplainPlan::kani_depth0(),
        label_left: String::kani_depth1(),
        label_right: String::kani_depth1(),
    };
}

/// Theory S: two ExplainPlan values in local scope (not wrapped in a struct).
///
/// Isolates whether it is the struct wrapper or the co-presence of two
/// ExplainPlan values in the same scope that causes unbounded unwinding.
#[cfg(kani)]
#[kani::proof]
fn diag_two_explain_nodes_local() {
    use elicitation::KaniCompose;
    let _a = ExplainPlan::kani_depth0();
    let _b = ExplainPlan::kani_depth0();
}

/// Theory T: inline explain_ready's ExplainView arm directly in the harness.
///
/// R and S pass (ExplainComparison drop is fine). Q hangs (calling explain_ready
/// with ExplainView state hangs). This isolates whether the problem is in the
/// function-call overhead (#[instrument], #[formal_method]) or the match body.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_ready_inlined_body() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let schema = String::kani_depth1();
    let table = String::kani_depth1();
    let root = ExplainPlan::kani_depth0();
    let display_mode = ExplainNodeMode::kani_depth0();
    let _next = match state {
        ArchivePanelState::ExplainView {
            schema: old_schema,
            table: old_table,
            root: old_root,
            ..
        } => ArchivePanelState::ExplainCompare {
            schema: schema.clone(),
            table: table.clone(),
            comparison: ExplainComparison {
                left: old_root,
                right: root,
                label_left: String::kani_depth1(),
                label_right: String::kani_depth1(),
            },
        },
        _ => ArchivePanelState::ExplainView {
            schema,
            table,
            root,
            display_mode,
        },
    };
}

/// Theory U: partial move from ExplainView via if-let, no wildcard arm.
#[cfg(kani)]
#[kani::proof]
fn diag_partial_move_if_let() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let root = ExplainPlan::kani_depth0();
    if let ArchivePanelState::ExplainView { root: old_root, .. } = state {
        let _comparison = ExplainComparison {
            left: old_root,
            right: root,
            label_left: String::kani_depth1(),
            label_right: String::kani_depth1(),
        };
    }
}

/// Theory V: match ExplainView without moving old_root — use fresh ExplainPlans in arm.
#[cfg(kani)]
#[kani::proof]
fn diag_match_no_field_move() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let _next = match state {
        ArchivePanelState::ExplainView { .. } => ArchivePanelState::ExplainCompare {
            schema: String::kani_depth1(),
            table: String::kani_depth1(),
            comparison: ExplainComparison {
                left: ExplainPlan::kani_depth0(),
                right: ExplainPlan::kani_depth0(),
                label_left: String::kani_depth1(),
                label_right: String::kani_depth1(),
            },
        },
        _ => ArchivePanelState::ColumnDetail,
    };
}

/// Theory W: wildcard arm creates ExplainView (with ExplainPlan) — does the wildcard
/// arm having ExplainPlan in it cause the hang even if ExplainView arm is taken?
#[cfg(kani)]
#[kani::proof]
fn diag_wildcard_arm_explain_view() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let fallback_root = ExplainPlan::kani_depth0();
    let _next = match state {
        ArchivePanelState::ExplainView { .. } => ArchivePanelState::ColumnDetail,
        _ => ArchivePanelState::ExplainView {
            schema: String::kani_depth1(),
            table: String::kani_depth1(),
            root: fallback_root,
            display_mode: ExplainNodeMode::kani_depth0(),
        },
    };
}

/// Theory X: directly drop ArchivePanelState::ExplainCompare with two ExplainPlans.
#[cfg(kani)]
#[kani::proof]
fn diag_drop_explain_compare() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ExplainCompare {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        comparison: ExplainComparison {
            left: ExplainPlan::kani_depth0(),
            right: ExplainPlan::kani_depth0(),
            label_left: String::kani_depth1(),
            label_right: String::kani_depth1(),
        },
    };
}

/// Theory Y: directly drop ArchivePanelState::ExplainView with ExplainPlan.
#[cfg(kani)]
#[kani::proof]
fn diag_drop_explain_view_direct() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
}

/// Theory Z: Theory X without unwind bound — should now pass since ExplainPlan is not recursive.
#[cfg(kani)]
#[kani::proof]
fn diag_drop_explain_compare_bounded() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ExplainCompare {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        comparison: ExplainComparison {
            left: ExplainPlan::kani_depth0(),
            right: ExplainPlan::kani_depth0(),
            label_left: String::kani_depth1(),
            label_right: String::kani_depth1(),
        },
    };
}

/// Theory D: fully concrete erd_ready (None layout, concrete diagram, symbolic mode).
#[cfg(kani)]
#[kani::proof]
fn diag_erd_ready_concrete() {
    let state = ArchivePanelState::ColumnDetail;
    let proof = elicitation::contracts::Established::<ArchivePanelConsistent>::assert();
    let diagram = ErdDiagram {
        schema: String::kani_depth1(),
        nodes: vec![],
        edges: vec![],
    };
    let layout: Option<ErdLayout> = None;
    let mode: ErdDiagramMode = kani::any();
    let _ = erd_ready(state, proof, String::kani_depth1(), diagram, layout, mode);
}

/// Theory AA: serde_json::Value::Null — does dropping a concrete Null cause recursive unwind?
/// Isolation: just the type, no collections, no other types.
#[cfg(kani)]
#[kani::proof]
fn diag_serde_json_value_null_drop() {
    let v: serde_json::Value = serde_json::Value::Null;
    let _ = v;
}

/// Theory AB: DbValue::Json — does CBMC's drop analysis for the Json variant cause
/// unbounded unwind? Under kani, Json(String) replaces Json(serde_json::Value) to
/// break the recursive type chain. This harness confirms the fix.
#[cfg(kani)]
#[kani::proof]
fn diag_db_value_json_drop() {
    // Under #[cfg(kani)], DbValue::Json holds a String, not serde_json::Value.
    let v = elicit_db::DbValue::Json("null".to_string());
    let _ = v;
}

/// Theory AC: QueryResult::kani_depth0() — does constructing and dropping this cause timeout?
/// If this hangs, the type chain DbRows→DbRow→DbValue→Json is the source.
/// If this passes, the timeout is caused by something else in the data_grid_ready harness.
#[cfg(kani)]
#[kani::proof]
fn diag_query_result_kani_depth0_drop() {
    use elicitation::KaniCompose;
    let r: QueryResult = QueryResult::kani_depth0();
    let _ = r;
}

/// Theory AD: ArchivePanelState::DataGrid creation and drop at kani_depth0.
/// Isolates whether the DataGrid variant itself is the SAT bottleneck in data_grid_ready.
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_state_drop() {
    use elicitation::KaniCompose;
    let s = ArchivePanelState::DataGrid {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        result: QueryResult::kani_depth0(),
        page: 0,
        grid_row: 0,
        grid_col: 0,
        edit_state: None,
        display_mode: QueryResultMode::kani_depth0(),
    };
    let _ = s;
}

/// Theory AE: Just drop DbRows::kani_depth0() inside the DataGrid context.
/// Theory AC showed QueryResult::kani_depth0() is fast alone.
/// This checks if wrapping in ArchivePanelState::DataGrid adds complexity.
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_minimal() {
    use elicitation::KaniCompose;
    // Minimal DataGrid: everything is the simplest possible value.
    // Under kani, QueryResult is simplified to { row_count: u64 }.
    let result = QueryResult { row_count: 0 };
    let s = ArchivePanelState::DataGrid {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        result,
        page: 0,
        grid_row: 0,
        grid_col: 0,
        edit_state: None,
        display_mode: QueryResultMode::DataGrid,
    };
    let _ = s;
}

/// Theory AF: Check if the 18-variant ArchivePanelState enum itself causes overhead
/// when created as DataGrid, vs. the ExplainView variant.
/// If AF times out but Theory AC passes, the overhead is in ArchivePanelState's type analysis.
#[cfg(kani)]
#[kani::proof]
fn diag_panel_state_expl_view_drop() {
    use elicitation::KaniCompose;
    let s = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: ExplainPlan::kani_depth0(),
        display_mode: ExplainNodeMode::kani_depth0(),
    };
    let _ = s;
}

/// Theory AG: DataGrid without the result field — replace with an empty String to isolate
/// whether the QueryResult type tree is the SAT bottleneck.
/// If this passes fast but Theory AE hangs, the bottleneck is in QueryResult's type.
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_no_result() {
    // Test by dropping a DataGrid-shaped struct without QueryResult.
    // We use a DbRows with zero rows to remove any heap nesting.
    let rows = elicit_db::DbRows {
        rows: Vec::new(),
        affected: 0,
    };
    let _ = rows;
    // Now wrap the other DataGrid fields (no result).
    let s = ArchivePanelState::ColumnDetail; // simplest variant — just proves 18-enum drop is cheap
    let _ = s;
}

/// Theory AH: Vec<(String, DbValue)> drop — direct test of DbRow inner type.
/// This isolates whether the 2-level Vec heap nesting causes CBMC formula explosion.
#[cfg(kani)]
#[kani::proof]
fn diag_db_row_inner_drop() {
    let v: Vec<(String, elicit_db::DbValue)> = Vec::new();
    let _ = v;
}

/// Theory AI: Vec<DbRow> drop — one level up from Theory AH.
#[cfg(kani)]
#[kani::proof]
fn diag_db_rows_vec_drop() {
    let v: Vec<elicit_db::DbRow> = Vec::new();
    let _ = v;
}

// ── open_connection_editor d1/d2 blocker theories ────────────────────────────

/// Theory AJ: ConnectionProfile::kani_depth1() drop alone.
/// Isolates whether multi-Option<String> struct causes SAT explosion at d1.
#[cfg(kani)]
#[kani::proof]
fn diag_connection_profile_depth1() {
    use elicitation::KaniCompose;
    let _p = <ConnectionProfile as KaniCompose>::kani_depth1();
}

/// Theory AK: ConnectionProfile::kani_depth2() drop alone.
/// Confirms whether d2 escalates the issue.
#[cfg(kani)]
#[kani::proof]
fn diag_connection_profile_depth2() {
    use elicitation::KaniCompose;
    let _p = <ConnectionProfile as KaniCompose>::kani_depth2();
}

/// Theory AL: ExplainPlan::kani_depth1() drop alone.
/// Isolates the arena-based ExplainPlan at d1.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_plan_depth1() {
    use elicitation::KaniCompose;
    let _p = <ExplainPlan as KaniCompose>::kani_depth1();
}

/// Theory AM: ExplainPlan::kani_depth2() drop alone.
/// Isolates the arena-based ExplainPlan at d2.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_plan_depth2() {
    use elicitation::KaniCompose;
    let _p = <ExplainPlan as KaniCompose>::kani_depth2();
}

/// Theory AN: Inline open_connection_editor body at d1, bypassing #[instrument].
/// Tests whether tracing span + symbolic heap at d1 causes the hang.
#[cfg(kani)]
#[kani::proof]
fn diag_open_connection_editor_inlined_d1() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let profile = Box::new(<ConnectionProfile as KaniCompose>::kani_depth1());
    let display_mode = <ConnectionProfileMode as KaniCompose>::kani_depth1();
    // Inline the function body — no #[instrument] wrapper
    let _result = (
        ArchivePanelState::ConnectionEdit {
            profile,
            display_mode,
        },
        (),
    );
    let _ = state;
    let _ = _result;
}

/// Theory AO: Same as AN but WITH calling the real function (includes #[instrument]).
/// If AN passes but AO hangs, #[instrument] tracing span is the blocker.
#[cfg(kani)]
#[kani::proof]
fn diag_open_connection_editor_real_d1() {
    use elicitation::KaniCompose;
    let state = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let proof = {
        let cred = ArchivePanelConsistent::kani_proof_credential();
        elicitation::Established::prove(&cred)
    };
    let profile = <ConnectionProfile as KaniCompose>::kani_depth1();
    let display_mode = <ConnectionProfileMode as KaniCompose>::kani_depth1();
    let _result = open_connection_editor(state, proof, profile, display_mode);
}

/// Theory AP: Drop two ArchivePanelState values (ExplainView d1 + ConnectionEdit d1).
/// Tests whether two concurrent symbolic 18-variant drops cause SAT explosion,
/// independent of any function call.
#[cfg(kani)]
#[kani::proof]
fn diag_two_panel_state_drops_d1() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let _s2 = ArchivePanelState::ConnectionEdit {
        profile: Box::new(<ConnectionProfile as KaniCompose>::kani_depth1()),
        display_mode: <ConnectionProfileMode as KaniCompose>::kani_depth1(),
    };
}

/// Theory AQ: Drop ONE ArchivePanelState ExplainView d1 — no second state.
/// If this passes but AP hangs, the issue is two concurrent state drops, not one.
#[cfg(kani)]
#[kani::proof]
fn diag_one_explain_view_drop_d1() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
}

/// Theory AR: ExplainView d1 + ColumnDetail (trivial second variant, no heap).
/// Tests if it's the 18-variant cross-product model OR the specific variant content.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_plus_column_detail_d1() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let _s2 = ArchivePanelState::ColumnDetail;
}

/// Theory AS: ExplainView d0 + ConnectionEdit d1 (concrete input is cheaper).
/// Tests if using depth-0 for the input state avoids the explosion.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_d0_plus_connection_edit_d1() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth0(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth0(),
    };
    let _s2 = ArchivePanelState::ConnectionEdit {
        profile: Box::new(<ConnectionProfile as KaniCompose>::kani_depth1()),
        display_mode: <ConnectionProfileMode as KaniCompose>::kani_depth1(),
    };
}

/// Theory AT: AdminView depth0 alone — no ExplainView.
/// If this hangs, AdminView itself has expensive drop logic.
/// If fast, the cost comes from the combination.
#[cfg(kani)]
#[kani::proof]
fn diag_admin_view_alone_d0() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::AdminView {
        snapshot: <AdminSnapshot as KaniCompose>::kani_depth0(),
        loading: kani::any(),
        display_mode: <AdminSnapshotMode as KaniCompose>::kani_depth0(),
    };
}

/// Theory AU: ExplainView d1 + AdminView d0 directly (no function call).
/// Isolates drop combination cost from transition function overhead.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_d1_plus_admin_view_d0() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let _s2 = ArchivePanelState::AdminView {
        snapshot: <AdminSnapshot as KaniCompose>::kani_depth0(),
        loading: kani::any(),
        display_mode: <AdminSnapshotMode as KaniCompose>::kani_depth0(),
    };
}

/// Theory AV: ExplainView d0 + AdminView d0.
/// If AU hangs but this is fast, d1 input depth is the trigger with AdminView.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_d0_plus_admin_view_d0() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth0(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth0(),
    };
    let _s2 = ArchivePanelState::AdminView {
        snapshot: <AdminSnapshot as KaniCompose>::kani_depth0(),
        loading: kani::any(),
        display_mode: <AdminSnapshotMode as KaniCompose>::kani_depth0(),
    };
}

/// Theory AW: ExplainView d1 + MonitorView d0 directly.
/// MonitorSnapshot has MORE Vec fields but monitor_ready passes.
/// If fast here too, MonitorView is cheaper than AdminView for a structural reason.
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_d1_plus_monitor_view_d0() {
    use elicitation::KaniCompose;
    let _s1 = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let _s2 = ArchivePanelState::MonitorView {
        snapshot: <MonitorSnapshot as KaniCompose>::kani_depth0(),
        loading: kani::any(),
        display_mode: <MonitorSnapshotMode as KaniCompose>::kani_depth0(),
    };
}

// ── Theory AX: ConnectionProfile alone ───────────────────────────────────────
// Isolates whether ConnectionProfile::kani_depth0() drop is bounded.
#[cfg(kani)]
#[kani::proof]
fn diag_connection_profile_d0_alone() {
    use elicitation::KaniCompose;
    let _p = <ConnectionProfile as KaniCompose>::kani_depth0();
}

// ── Theory AY: ExplainPlan d1 + ConnectionProfile d0 ─────────────────────────
// Isolates whether the two-drop combination is bounded (analogous to AU/AV).
#[cfg(kani)]
#[kani::proof]
fn diag_explain_view_d1_plus_connection_profile_d0() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ExplainView {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        root: <ExplainPlan as KaniCompose>::kani_depth1(),
        display_mode: <ExplainNodeMode as KaniCompose>::kani_depth1(),
    };
    let _p = <ConnectionProfile as KaniCompose>::kani_depth0();
}

// ── Theory AZ: ConnectionEdit output state alone ─────────────────────────────
// Isolates whether constructing and dropping the result state is bounded.
// PREVIOUSLY HUNG: ConnectionProfile flat in union + BTree-bearing dead arms.
// AFTER FIX: Box<ConnectionProfile> → union footprint = 8B pointer → EXPECTED PASS.
#[cfg(kani)]
#[kani::proof]
fn diag_connection_edit_state_d0_alone() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ConnectionEdit {
        profile: Box::new(<ConnectionProfile as KaniCompose>::kani_depth0()),
        display_mode: <ConnectionProfileMode as KaniCompose>::kani_depth0(),
    };
}

// ── Theory BA: ErdView with layout: None — inline Option<ErdLayout> in union ─
// If this hangs, the BTree drop loops are triggered by nondeterministic union
// bytes even when Option discriminant is None.  That would confirm that boxing
// is required to cut the reachability.
#[cfg(kani)]
#[kani::proof]
fn diag_erd_view_layout_none() {
    use elicitation::KaniCompose;
    let _s = ArchivePanelState::ErdView {
        schema: String::kani_depth1(),
        diagram: <ErdDiagram as KaniCompose>::kani_depth0(),
        layout: None,
        loading: false,
        display_mode: <ErdDiagramMode as KaniCompose>::kani_depth0(),
    };
}

// ── Theory BB: ColumnDetail (unit variant) alone ────────────────────────────────
// If this hangs, the issue is the enum's drop glue in general (CBMC explores all
// variant arms even with a concrete discriminant).  If it passes, the hang is
// specific to some data-carrying variants.
#[cfg(kani)]
#[kani::proof]
fn diag_column_detail_unit_alone() {
    let _s = ArchivePanelState::ColumnDetail;
}

// ── Theory BC: Loading (2 plain Strings) alone ───────────────────────────────
// Simpler data-carrying variant.  If BB passes but BC hangs, even two Strings
// inside this enum union cause CBMC trouble.
#[cfg(kani)]
#[kani::proof]
fn diag_loading_alone() {
    let _s = ArchivePanelState::Loading {
        schema: String::kani_depth1(),
        label: String::kani_depth1(),
    };
}

// ── Theory BD: ErrorView (1 String, last variant) alone ──────────────────────
// Last variant; smallest data-carrying variant.
#[cfg(kani)]
#[kani::proof]
fn diag_error_view_alone() {
    let _s = ArchivePanelState::ErrorView {
        message: String::kani_depth1(),
    };
}

// ── Theory BE: ConnectionEdit with ALL fields inlined (no kani_depth0 call) ──
// AZ calls kani_depth0() which is a function call that CBMC inlines.  This
// theory constructs the IDENTICAL concrete value inline without any function
// calls.  If AZ hangs but BE passes, the kani_depth0() function call/inlining
// path (or something reachable from it) is the trigger.  If BE also hangs,
// the type structure / union layout itself is the root cause.
// RESULT: HANGS — fully concrete inline still hangs. Root cause is structural.
// #[kani::proof]
// fn diag_connection_edit_inline() { ... }

// ── Theory BF: ConnectionEdit with all Options as Some (not None) ────────────
// RESULT: HANGS (when profile was flat in union). After boxing, this should pass.
// BF is now superseded by AZ (which uses KaniCompose::kani_depth0 + Box).
// #[kani::proof]
// fn diag_connection_edit_options_some() { ... }

// ── Theory BG: SqlEditor (has Option<QueryResult> + Option<String>) ──────────
// Tests whether ANY variant with Option fields hangs.  SqlEditor is simpler
// than ConnectionEdit (fewer fields) but has two Option fields.
#[cfg(kani)]
#[kani::proof]
fn diag_sql_editor_alone() {
    let _s = ArchivePanelState::SqlEditor {
        text: String::kani_depth1(),
        result: None,
        running: false,
        error: None,
    };
}

// ── Theory BH: Bare DataGrid with all concrete-zero / None fields ─────────────
// Does the DataGrid variant itself cause CBMC trouble when fully concrete?
// All scalars are zero, edit_state is None, Strings are empty (no heap).
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_concrete_zero() {
    let _s = ArchivePanelState::DataGrid {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        result: QueryResult { row_count: 0 },
        page: 0_u32,
        grid_row: 0_usize,
        grid_col: 0_usize,
        edit_state: None,
        display_mode: QueryResultMode::DataGrid,
    };
}

// ── Theory BI: DataGrid with kani::any() for all scalar fields ───────────────
// Does symbolic content in the scalars (page/grid_row/grid_col/row_count) cause
// a blowup?
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_symbolic_scalars() {
    let _s = ArchivePanelState::DataGrid {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        result: QueryResult {
            row_count: kani::any(),
        },
        page: kani::any(),
        grid_row: kani::any(),
        grid_col: kani::any(),
        edit_state: None,
        display_mode: QueryResultMode::DataGrid,
    };
}

// ── Theory BJ: column_detail() called with DataGrid input ────────────────────
// Mimics the failing harness: DataGrid state consumed + ColumnDetail returned.
// If BH/BI pass but BJ hangs, the transition call itself (or its drop semantics)
// is the trigger.
#[cfg(kani)]
#[kani::proof]
fn diag_column_detail_with_data_grid_input() {
    use elicitation::contracts::{Established, ProvableFrom};
    let state = ArchivePanelState::DataGrid {
        schema: String::kani_depth1(),
        table: String::kani_depth1(),
        result: QueryResult { row_count: 0 },
        page: 0_u32,
        grid_row: 0_usize,
        grid_col: 0_usize,
        edit_state: None,
        display_mode: QueryResultMode::DataGrid,
    };
    let proof: Established<ArchivePanelConsistent> = Established::assert();
    let _r = column_detail(state, proof);
}

// ── Theory BK: data_grid_ready() called with ColumnDetail input ──────────────
// Mimics the other failing harness: trivial input, DataGrid OUTPUT produced.
// If BH/BI pass but BK hangs, the DataGrid output drop (not input) is the culprit.
#[cfg(kani)]
#[kani::proof]
fn diag_data_grid_ready_with_column_detail_input() {
    use elicitation::contracts::{Established, ProvableFrom};
    let state = ArchivePanelState::ColumnDetail;
    let proof: Established<ArchivePanelConsistent> = Established::assert();
    let result = QueryResult { row_count: 0 };
    let display_mode = QueryResultMode::DataGrid;
    let _r = data_grid_ready(
        state,
        proof,
        String::kani_depth1(),
        String::kani_depth1(),
        result,
        display_mode,
    );
}
