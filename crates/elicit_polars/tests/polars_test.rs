//! Integration tests for `elicit_polars`.

use std::sync::Arc;

use elicit_polars::{
    PolarsDataFramePlugin, PolarsDfCreated, PolarsExprCreated, PolarsExprPlugin,
    PolarsPipelineCreated, PolarsPipelinePlugin, PolarsSqlCreated, PolarsSqlPlugin,
};
use elicitation::{ElicitPlugin, PluginToolRegistration, VerifiedWorkflow};
use rmcp::model::CallToolRequestParams;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn assert_tool_exists(plugin: &dyn ElicitPlugin, name: &str) {
    let tools = plugin.list_tools();
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();
    assert!(
        names.contains(&name),
        "missing tool: {name}\navailable: {names:?}"
    );
}

fn make_params(name: &'static str, args: serde_json::Value) -> CallToolRequestParams {
    CallToolRequestParams::new(name).with_arguments(args.as_object().cloned().unwrap_or_default())
}

async fn dispatch_tool(
    plugin_name: &'static str,
    tool_name: &'static str,
    ctx: Arc<dyn std::any::Any + Send + Sync>,
    args: serde_json::Value,
) -> serde_json::Value {
    let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
        .filter(|r| r.plugin == plugin_name)
        .find(|r| r.name == tool_name)
        .map(|r| (r.constructor)())
        .unwrap_or_else(|| panic!("tool not found: {plugin_name}::{tool_name}"));

    let params = make_params(tool_name, args);
    let result = descriptor.dispatch(ctx, params).await.unwrap();
    let text = result.content[0].as_text().unwrap().text.as_str();
    // Try JSON decode; if it fails, return as a JSON string value.
    serde_json::from_str(text).unwrap_or_else(|_| serde_json::Value::String(text.to_string()))
}

// ── Smoke tests: plugin creation ──────────────────────────────────────────────

#[test]
fn expr_plugin_creates_successfully() {
    let p = PolarsExprPlugin::new();
    assert_eq!(p.name(), "polars_expr");
}

#[test]
fn df_plugin_creates_successfully() {
    let expr = PolarsExprPlugin::new();
    let p = PolarsDataFramePlugin::new(expr.registry());
    assert_eq!(p.name(), "polars_df");
}

#[test]
fn pipeline_plugin_creates_successfully() {
    let p = PolarsPipelinePlugin::new();
    assert_eq!(p.name(), "polars_pipeline");
}

#[test]
fn sql_plugin_creates_with_df_registry() {
    let expr = PolarsExprPlugin::new();
    let df = PolarsDataFramePlugin::new(expr.registry());
    let p = PolarsSqlPlugin::new(df.df_registry());
    assert_eq!(p.name(), "polars_sql");
}

// ── Tool registration ──────────────────────────────────────────────────────────

#[test]
fn expr_plugin_lists_core_tools() {
    let p = PolarsExprPlugin::new();
    for name in &[
        "polars_expr__col",
        "polars_expr__lit_int",
        "polars_expr__lit_float",
        "polars_expr__lit_str",
        "polars_expr__lit_bool",
        "polars_expr__all_columns",
        "polars_expr__eq",
        "polars_expr__gt",
        "polars_expr__lt",
        "polars_expr__and",
        "polars_expr__or",
        "polars_expr__not",
        "polars_expr__sum",
        "polars_expr__mean",
        "polars_expr__alias",
        "polars_expr__cast",
        "polars_expr__is_null",
        "polars_expr__is_not_null",
        "polars_expr__emit",
        "polars_expr__list",
    ] {
        assert_tool_exists(&p, name);
    }
}

#[test]
fn df_plugin_lists_core_tools() {
    let expr = PolarsExprPlugin::new();
    let p = PolarsDataFramePlugin::new(expr.registry());
    for name in &[
        "polars_df__read_csv",
        "polars_df__read_parquet",
        "polars_df__read_json",
        "polars_df__from_json_string",
        "polars_df__schema",
        "polars_df__shape",
        "polars_df__head",
        "polars_df__to_json_string",
        "polars_df__select",
        "polars_df__filter",
        "polars_df__with_columns",
        "polars_df__sort",
        "polars_df__group_by_agg",
        "polars_df__join",
        "polars_df__unique",
        "polars_df__drop_nulls",
        "polars_df__rename_column",
        "polars_df__drop_column",
        "polars_df__write_csv",
        "polars_df__write_parquet",
        "polars_df__write_json",
        "polars_df__write_ipc",
        "polars_df__list",
    ] {
        assert_tool_exists(&p, name);
    }
}

#[test]
fn pipeline_plugin_lists_core_tools() {
    let p = PolarsPipelinePlugin::new();
    for name in &[
        "polars_pipeline__new",
        "polars_pipeline__add_step",
        "polars_pipeline__remove_step",
        "polars_pipeline__clear",
        "polars_pipeline__describe",
        "polars_pipeline__emit_main",
        "polars_pipeline__list",
    ] {
        assert_tool_exists(&p, name);
    }
}

#[test]
fn sql_plugin_lists_core_tools() {
    let expr = PolarsExprPlugin::new();
    let df = PolarsDataFramePlugin::new(expr.registry());
    let p = PolarsSqlPlugin::new(df.df_registry());
    for name in &[
        "polars_sql__new_context",
        "polars_sql__register",
        "polars_sql__execute",
        "polars_sql__describe",
        "polars_sql__list",
    ] {
        assert_tool_exists(&p, name);
    }
}

// ── Proposition proofs ────────────────────────────────────────────────────────

#[test]
fn expr_proposition_proofs_non_empty() {
    assert!(PolarsExprCreated::validate_proofs_non_empty());
}

#[test]
fn df_proposition_proofs_non_empty() {
    assert!(PolarsDfCreated::validate_proofs_non_empty());
}

#[test]
fn pipeline_proposition_proofs_non_empty() {
    assert!(PolarsPipelineCreated::validate_proofs_non_empty());
}

#[test]
fn sql_proposition_proofs_non_empty() {
    assert!(PolarsSqlCreated::validate_proofs_non_empty());
}

// ── Runtime: Expr composition ─────────────────────────────────────────────────

#[tokio::test]
async fn expr_col_then_emit() {
    let plugin = PolarsExprPlugin::new();
    let ctx = plugin.dispatch_ctx();

    let col_val = dispatch_tool(
        "polars_expr",
        "polars_expr__col",
        ctx.clone(),
        serde_json::json!({"name": "age"}),
    )
    .await;
    let expr_id = col_val["expr_id"].as_str().unwrap().to_string();

    let emit_val = dispatch_tool(
        "polars_expr",
        "polars_expr__emit",
        ctx.clone(),
        serde_json::json!({"expr_id": expr_id}),
    )
    .await;

    // emit returns a raw Rust code string
    let code = emit_val
        .as_str()
        .unwrap_or_else(|| panic!("emit should return a string: {emit_val}"));
    assert_eq!(code, "col(\"age\")", "emit should return Rust DSL source");
}

#[tokio::test]
async fn expr_gt_composition() {
    let plugin = PolarsExprPlugin::new();
    let ctx = plugin.dispatch_ctx();

    let col_val = dispatch_tool(
        "polars_expr",
        "polars_expr__col",
        ctx.clone(),
        serde_json::json!({"name": "age"}),
    )
    .await;
    let col_id = col_val["expr_id"].as_str().unwrap().to_string();

    let lit_val = dispatch_tool(
        "polars_expr",
        "polars_expr__lit_int",
        ctx.clone(),
        serde_json::json!({"value": 18}),
    )
    .await;
    let lit_id = lit_val["expr_id"].as_str().unwrap().to_string();

    let gt_val = dispatch_tool(
        "polars_expr",
        "polars_expr__gt",
        ctx.clone(),
        serde_json::json!({"left_id": col_id, "right_id": lit_id}),
    )
    .await;
    let gt_id = gt_val["expr_id"].as_str().unwrap().to_string();

    // Verify the expr is stored
    let list_val = dispatch_tool(
        "polars_expr",
        "polars_expr__list",
        ctx.clone(),
        serde_json::json!({}),
    )
    .await;
    let entries = list_val.as_array().unwrap();
    let gt_entry = entries
        .iter()
        .find(|e| e["expr_id"].as_str() == Some(&gt_id))
        .expect("gt expr should be in list");
    let code = gt_entry["code"].as_str().unwrap();
    assert!(code.contains("gt"), "code should contain gt: {code}");
    assert!(code.contains("18"), "code should contain 18: {code}");
}

// ── Runtime: DataFrame from JSON ─────────────────────────────────────────────

#[tokio::test]
async fn df_from_json_string_schema_head() {
    let expr_plugin = PolarsExprPlugin::new();
    let df_plugin = PolarsDataFramePlugin::new(expr_plugin.registry());
    let ctx = df_plugin.dispatch_ctx();

    let json_data = r#"[{"name":"Alice","age":30},{"name":"Bob","age":25}]"#;
    let create_val = dispatch_tool(
        "polars_df",
        "polars_df__from_json_string",
        ctx.clone(),
        serde_json::json!({"json": json_data}),
    )
    .await;
    let df_id = create_val["df_id"].as_str().unwrap().to_string();

    let schema_val = dispatch_tool(
        "polars_df",
        "polars_df__schema",
        ctx.clone(),
        serde_json::json!({"df_id": df_id}),
    )
    .await;
    let schema_text = schema_val.to_string();
    assert!(
        schema_text.contains("name") || schema_text.contains("age"),
        "schema should list columns: {schema_text}"
    );

    let shape_val = dispatch_tool(
        "polars_df",
        "polars_df__shape",
        ctx.clone(),
        serde_json::json!({"df_id": df_id}),
    )
    .await;
    assert_eq!(shape_val["rows"], 2, "should have 2 rows: {shape_val}");

    let head_val = dispatch_tool(
        "polars_df",
        "polars_df__head",
        ctx.clone(),
        serde_json::json!({"df_id": df_id, "n": 1}),
    )
    .await;
    // df_head returns a JSON array of row objects
    let rows = head_val
        .as_array()
        .expect("head should return a JSON array");
    assert_eq!(rows.len(), 1, "head(1) should have 1 row: {head_val}");
}

// ── Runtime: Pipeline code gen ────────────────────────────────────────────────

#[tokio::test]
async fn pipeline_emit_main_contains_lazycsvreader() {
    let plugin = PolarsPipelinePlugin::new();
    let ctx = plugin.dispatch_ctx();

    let new_val = dispatch_tool(
        "polars_pipeline",
        "polars_pipeline__new",
        ctx.clone(),
        serde_json::json!({"name": "test"}),
    )
    .await;
    let pipeline_id = new_val["pipeline_id"].as_str().unwrap().to_string();

    dispatch_tool(
        "polars_pipeline",
        "polars_pipeline__add_step",
        ctx.clone(),
        serde_json::json!({
            "pipeline_id": pipeline_id,
            "op": {"op": "read_csv", "path": "data.csv", "has_header": true}
        }),
    )
    .await;

    dispatch_tool(
        "polars_pipeline",
        "polars_pipeline__add_step",
        ctx.clone(),
        serde_json::json!({
            "pipeline_id": pipeline_id,
            "op": {"op": "select", "columns": ["a", "b"]}
        }),
    )
    .await;

    // emit_main returns a plain string (Rust source), not JSON
    let descriptor = elicitation::inventory::iter::<PluginToolRegistration>()
        .filter(|r| r.plugin == "polars_pipeline")
        .find(|r| r.name == "polars_pipeline__emit_main")
        .map(|r| (r.constructor)())
        .unwrap();

    let params = make_params(
        "polars_pipeline__emit_main",
        serde_json::json!({"pipeline_id": pipeline_id}),
    );
    let result = descriptor.dispatch(ctx, params).await.unwrap();
    let code = result.content[0].as_text().unwrap().text.as_str();
    assert!(
        code.contains("LazyCsvReader"),
        "emitted code should contain LazyCsvReader: {code}"
    );
    assert!(
        code.contains("data.csv"),
        "emitted code should reference data.csv: {code}"
    );
    assert!(
        code.contains("select"),
        "emitted code should contain select: {code}"
    );
}

// ── Runtime: SQL context ──────────────────────────────────────────────────────

#[tokio::test(flavor = "multi_thread")]
async fn sql_new_context_register_and_query() {
    let expr_plugin = PolarsExprPlugin::new();
    let df_plugin = PolarsDataFramePlugin::new(expr_plugin.registry());
    let sql_plugin = PolarsSqlPlugin::new(df_plugin.df_registry());

    let df_ctx = df_plugin.dispatch_ctx();
    let sql_ctx = sql_plugin.dispatch_ctx();

    let json_data = r#"[{"id":1,"val":10},{"id":2,"val":20}]"#;
    let df_val = dispatch_tool(
        "polars_df",
        "polars_df__from_json_string",
        df_ctx,
        serde_json::json!({"json": json_data}),
    )
    .await;
    let df_id = df_val["df_id"].as_str().unwrap().to_string();

    let ctx_val = dispatch_tool(
        "polars_sql",
        "polars_sql__new_context",
        sql_ctx.clone(),
        serde_json::json!({}),
    )
    .await;
    let ctx_id = ctx_val["ctx_id"].as_str().unwrap().to_string();

    dispatch_tool(
        "polars_sql",
        "polars_sql__register",
        sql_ctx.clone(),
        serde_json::json!({"ctx_id": ctx_id, "table_name": "t", "df_id": df_id}),
    )
    .await;

    let exec_val = dispatch_tool(
        "polars_sql",
        "polars_sql__execute",
        sql_ctx.clone(),
        serde_json::json!({"ctx_id": ctx_id, "query": "SELECT id FROM t WHERE val > 10"}),
    )
    .await;
    assert!(
        exec_val["df_id"].is_string(),
        "execute should return a df_id: {exec_val}"
    );
}
