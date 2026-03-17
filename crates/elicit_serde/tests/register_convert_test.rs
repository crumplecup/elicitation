//! Integration tests for `DynamicToolRegistry::register_convert`.

use elicit_serde::DynamicToolRegistry;
use elicitation::ElicitPlugin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;

fn result_text(result: rmcp::model::CallToolResult) -> String {
    result
        .content
        .first()
        .and_then(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

// ── Test types ─────────────────────────────────────────────────────────────────

/// Source type: integer value field.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ConfigV1 {
    name: String,
    value: i32,
}

/// Target type: wider integer + optional new field (serde default fills it in).
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ConfigV2 {
    name: String,
    value: i64,
    #[serde(default)]
    tag: Option<String>,
}

/// A flat colour struct whose fields match `Point` so round-tripping works.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Coord {
    x: f64,
    y: f64,
}

/// Renamed clone of `Coord` to get a distinct tool name.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct Vertex {
    x: f64,
    y: f64,
}

// ── Tool naming ────────────────────────────────────────────────────────────────

#[test]
fn tool_name_derived_from_type_names() {
    let registry = DynamicToolRegistry::new().register_convert::<ConfigV1, ConfigV2>();
    let tools = registry.list_tools();
    let names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(
        names
            .iter()
            .any(|n| n == "convert__config_v1__to__config_v2"),
        "expected convert tool in {names:?}"
    );
}

#[test]
fn tool_appears_in_list_immediately() {
    let registry = DynamicToolRegistry::new().register_convert::<Coord, Vertex>();
    let tools = registry.list_tools();
    assert!(
        tools.iter().any(|t| t.name == "convert__coord__to__vertex"),
        "convert tool not found in list: {:?}",
        tools.iter().map(|t| &t.name).collect::<Vec<_>>()
    );
}

// ── Input schema ───────────────────────────────────────────────────────────────

#[test]
fn input_schema_is_source_type_schema() {
    let registry = DynamicToolRegistry::new().register_convert::<ConfigV1, ConfigV2>();
    let tools = registry.list_tools();
    let tool = tools
        .iter()
        .find(|t| t.name == "convert__config_v1__to__config_v2")
        .expect("convert tool not found");
    // The schema should mention "name" and "value" (ConfigV1 fields).
    let schema_str = serde_json::to_string(&tool.input_schema).unwrap();
    assert!(
        schema_str.contains("\"name\""),
        "schema missing 'name' field: {schema_str}"
    );
    assert!(
        schema_str.contains("\"value\""),
        "schema missing 'value' field: {schema_str}"
    );
    // Should NOT mention "tag" (that's ConfigV2 only).
    assert!(
        !schema_str.contains("\"tag\""),
        "schema should be ConfigV1, not ConfigV2: {schema_str}"
    );
}

// ── Successful invocation ──────────────────────────────────────────────────────

#[tokio::test]
async fn successful_conversion_widens_integer_and_fills_default() {
    let registry = DynamicToolRegistry::new().register_convert::<ConfigV1, ConfigV2>();
    let params = json!({ "name": "alpha", "value": 42 });
    let result = registry
        .invoke_dynamic("convert__config_v1__to__config_v2", params)
        .await
        .expect("tool not found")
        .expect("invoke should succeed");

    let text = result_text(result);
    let v2: ConfigV2 = serde_json::from_str(&text).expect("result must deserialize as ConfigV2");
    assert_eq!(v2.name, "alpha");
    assert_eq!(v2.value, 42_i64);
    assert_eq!(v2.tag, None, "tag should default to None");
}

#[tokio::test]
async fn round_trip_coord_to_vertex() {
    let registry = DynamicToolRegistry::new().register_convert::<Coord, Vertex>();
    let params = json!({ "x": 1.5, "y": 2.5 });
    let result = registry
        .invoke_dynamic("convert__coord__to__vertex", params)
        .await
        .expect("tool not found")
        .expect("invoke should succeed");

    let text = result_text(result);
    let v: Vertex = serde_json::from_str(&text).unwrap();
    assert!((v.x - 1.5).abs() < f64::EPSILON);
    assert!((v.y - 2.5).abs() < f64::EPSILON);
}

// ── Error paths ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn missing_required_field_returns_error() {
    let registry = DynamicToolRegistry::new().register_convert::<ConfigV1, ConfigV2>();
    // Omit the required `value` field.
    let params = json!({ "name": "broken" });
    let err = registry
        .invoke_dynamic("convert__config_v1__to__config_v2", params)
        .await
        .expect("tool not found")
        .expect_err("should fail with missing required field");
    let msg = err.message.to_lowercase();
    assert!(
        msg.contains("missing") || msg.contains("deserialize") || msg.contains("value"),
        "error message should mention missing field, got: {msg}"
    );
}

#[tokio::test]
async fn incompatible_types_return_error() {
    // ConfigV1 has { name: String, value: i32 }
    // Blob { bytes: Vec<u8> } is structurally incompatible.
    #[derive(Serialize, Deserialize, JsonSchema)]
    struct Blob {
        bytes: Vec<u8>,
    }
    let registry = DynamicToolRegistry::new().register_convert::<ConfigV1, Blob>();
    let params = json!({ "name": "test", "value": 99 });
    let err = registry
        .invoke_dynamic("convert__config_v1__to__blob", params)
        .await
        .expect("tool not found")
        .expect_err("ConfigV1 → Blob should fail structurally");
    let msg = err.message.to_lowercase();
    assert!(
        msg.contains("conversion") || msg.contains("deserializ") || msg.contains("missing"),
        "got: {msg}"
    );
}

// ── Duplicate registration guard ───────────────────────────────────────────────

#[test]
#[should_panic(expected = "already registered")]
fn duplicate_register_panics() {
    DynamicToolRegistry::new()
        .register_convert::<ConfigV1, ConfigV2>()
        .register_convert::<ConfigV1, ConfigV2>();
}
