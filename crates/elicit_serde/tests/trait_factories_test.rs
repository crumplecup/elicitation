//! Integration tests for `elicit_serde`'s `#[reflect_trait]` factories.
//!
//! Covers both serde wrapper traits:
//! - [`SerializeJson`] — `to_json(&self) -> Result<String, String>`
//! - [`DeserializeJson`] — `from_json(json: &str) -> Result<Self, String>`
//!
//! Each factory is tested at three levels:
//! 1. Inventory registration (link-time submission)
//! 2. Prime → register_type → instantiate lifecycle
//! 3. Handler invocation via `DynamicToolRegistry::invoke_dynamic`

use elicit_serde::{prime_deserialize_json, prime_serialize_json};
use elicitation::{
    DynamicToolRegistry, Elicit, ElicitPlugin, ToolFactoryRegistration,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Test types ────────────────────────────────────────────────────────────────

/// Simple struct for round-trip tests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct Point {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

/// Enum for variant serialization tests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    /// Red variant
    Red,
    /// Green variant
    Green,
    /// Blue variant
    Blue,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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

// ── Inventory registration ────────────────────────────────────────────────────

#[test]
fn serialize_json_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "crate::SerializeJson");
    assert!(found, "SerializeJsonFactory not found in inventory");
}

#[test]
fn deserialize_json_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "crate::DeserializeJson");
    assert!(found, "DeserializeJsonFactory not found in inventory");
}

// ── Lifecycle: prime → register_type → instantiate ───────────────────────────

#[tokio::test]
async fn serialize_json_instantiate_creates_to_json_tool() {
    prime_serialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("geo");
    registry
        .instantiate("crate::SerializeJson", "geo")
        .await
        .expect("SerializeJson instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    assert!(
        names.iter().any(|n| n == "geo__to_json"),
        "geo__to_json not in tool list: {names:?}"
    );
}

#[tokio::test]
async fn deserialize_json_instantiate_creates_from_json_tool() {
    prime_deserialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("geo2");
    registry
        .instantiate("crate::DeserializeJson", "geo2")
        .await
        .expect("DeserializeJson instantiate should succeed");

    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    assert!(
        names.iter().any(|n| n == "geo2__from_json"),
        "geo2__from_json not in tool list: {names:?}"
    );
}

#[tokio::test]
async fn serialize_json_color_instantiate_creates_tool() {
    prime_serialize_json::<Color>();
    let registry = DynamicToolRegistry::new().register_type::<Color>("palette");
    registry
        .instantiate("crate::SerializeJson", "palette")
        .await
        .expect("instantiate should succeed");
    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    assert!(names.iter().any(|n| n == "palette__to_json"));
}

#[tokio::test]
async fn deserialize_json_color_instantiate_creates_tool() {
    prime_deserialize_json::<Color>();
    let registry = DynamicToolRegistry::new().register_type::<Color>("palette2");
    registry
        .instantiate("crate::DeserializeJson", "palette2")
        .await
        .expect("instantiate should succeed");
    let names: Vec<String> = registry
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    assert!(names.iter().any(|n| n == "palette2__from_json"));
}

/// A type only used in `instantiate_fails_without_prime` — never primed.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct NeverPrimed {
    /// Unique field
    pub v: u32,
}

#[tokio::test]
async fn instantiate_fails_without_prime() {
    // NeverPrimed is never passed to prime_serialize_json in any test
    let registry = DynamicToolRegistry::new().register_type::<NeverPrimed>("no_prime");
    let result = registry
        .instantiate("crate::SerializeJson", "no_prime")
        .await;
    assert!(
        result.is_err(),
        "Expected error when factory not primed for NeverPrimed"
    );
}

// ── Handler invocation: SerializeJson ─────────────────────────────────────────

#[tokio::test]
async fn to_json_point_returns_ok_with_json_string() {
    prime_serialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("s1");
    registry
        .instantiate("crate::SerializeJson", "s1")
        .await
        .unwrap();

    let args = serde_json::json!({ "target": { "x": 1.0, "y": 2.5 } });
    let result = registry
        .invoke_dynamic("s1__to_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("valid JSON response");
    let inner = parsed
        .get("Ok")
        .and_then(|v| v.as_str())
        .expect("Ok variant with string");
    let point: Point = serde_json::from_str(inner).expect("valid Point JSON");
    assert_eq!(point, Point { x: 1.0, y: 2.5 });
}

#[tokio::test]
async fn to_json_color_returns_snake_case_string() {
    prime_serialize_json::<Color>();
    let registry = DynamicToolRegistry::new().register_type::<Color>("s2");
    registry
        .instantiate("crate::SerializeJson", "s2")
        .await
        .unwrap();

    let args = serde_json::json!({ "target": "red" });
    let result = registry
        .invoke_dynamic("s2__to_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    let inner = parsed["Ok"].as_str().expect("Ok variant");
    assert_eq!(inner, "\"red\"");
}

// ── Handler invocation: DeserializeJson ──────────────────────────────────────

#[tokio::test]
async fn from_json_point_returns_deserialized_point() {
    prime_deserialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("d1");
    registry
        .instantiate("crate::DeserializeJson", "d1")
        .await
        .unwrap();

    let args = serde_json::json!({ "json": r#"{"x":3.0,"y":4.0}"# });
    let result = registry
        .invoke_dynamic("d1__from_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    let point: Point = serde_json::from_value(parsed["Ok"].clone()).expect("valid Point");
    assert_eq!(point, Point { x: 3.0, y: 4.0 });
}

#[tokio::test]
async fn from_json_color_returns_deserialized_variant() {
    prime_deserialize_json::<Color>();
    let registry = DynamicToolRegistry::new().register_type::<Color>("d2");
    registry
        .instantiate("crate::DeserializeJson", "d2")
        .await
        .unwrap();

    let args = serde_json::json!({ "json": "\"green\"" });
    let result = registry
        .invoke_dynamic("d2__from_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    let color: Color = serde_json::from_value(parsed["Ok"].clone()).unwrap();
    assert_eq!(color, Color::Green);
}

#[tokio::test]
async fn from_json_invalid_input_returns_err_variant() {
    prime_deserialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("d3");
    registry
        .instantiate("crate::DeserializeJson", "d3")
        .await
        .unwrap();

    let args = serde_json::json!({ "json": "not valid json" });
    let result = registry
        .invoke_dynamic("d3__from_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(
        parsed.get("Err").is_some(),
        "Expected Err variant, got: {parsed}"
    );
}

#[tokio::test]
async fn from_json_wrong_shape_returns_err_variant() {
    prime_deserialize_json::<Point>();
    let registry = DynamicToolRegistry::new().register_type::<Point>("d4");
    registry
        .instantiate("crate::DeserializeJson", "d4")
        .await
        .unwrap();

    let args = serde_json::json!({ "json": r#"{"name":"bob"}"# });
    let result = registry
        .invoke_dynamic("d4__from_json", args)
        .await
        .expect("tool exists")
        .expect("no invocation error");
    let text = result_text(result);
    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert!(
        parsed.get("Err").is_some(),
        "Expected Err variant, got: {parsed}"
    );
}

// ── Round-trip: to_json → from_json ──────────────────────────────────────────

#[tokio::test]
async fn to_json_then_from_json_round_trips_point() {
    prime_serialize_json::<Point>();
    prime_deserialize_json::<Point>();
    // Use separate prefixes: instantiate replaces all {prefix}__* tools, so
    // two factories on the same prefix would overwrite each other.
    let registry = DynamicToolRegistry::new()
        .register_type::<Point>("rt_s")
        .register_type::<Point>("rt_d");
    registry
        .instantiate("crate::SerializeJson", "rt_s")
        .await
        .unwrap();
    registry
        .instantiate("crate::DeserializeJson", "rt_d")
        .await
        .unwrap();

    let original = Point { x: -7.5, y: 42.0 };

    // Step 1: serialize via rt_s__to_json
    let to_args = serde_json::json!({ "target": original });
    let to_result = registry
        .invoke_dynamic("rt_s__to_json", to_args)
        .await
        .unwrap()
        .unwrap();
    let to_text = result_text(to_result);
    let to_parsed: serde_json::Value = serde_json::from_str(&to_text).unwrap();
    let json_string = to_parsed["Ok"].as_str().expect("Ok string").to_string();

    // Canonical JSON should match
    assert_eq!(json_string, serde_json::to_string(&original).unwrap());

    // Step 2: deserialize via rt_d__from_json
    let from_args = serde_json::json!({ "json": json_string });
    let from_result = registry
        .invoke_dynamic("rt_d__from_json", from_args)
        .await
        .unwrap()
        .unwrap();
    let from_text = result_text(from_result);
    let from_parsed: serde_json::Value = serde_json::from_str(&from_text).unwrap();
    let roundtripped: Point = serde_json::from_value(from_parsed["Ok"].clone()).unwrap();
    assert_eq!(roundtripped, original);
}

// ── Unknown tool ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn unknown_tool_name_returns_none() {
    let registry = DynamicToolRegistry::new();
    let result = registry
        .invoke_dynamic("nonexistent__to_json", serde_json::json!({}))
        .await;
    assert!(result.is_none(), "Expected None for unknown tool");
}
