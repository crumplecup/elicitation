//! `SerdePlugin` — MCP tools for JSON serialization and deserialization via `serde_json`.
//!
//! Registered under the `"serde"` namespace. Uses `erased-serde` to erase generic
//! Serializer/Deserializer parameters, exposing concrete JSON operations as MCP tools.

use elicitation::ElicitPlugin;
use futures::future::BoxFuture;
use rmcp::{
    ErrorData,
    model::{CallToolRequestParams, CallToolResult, Content, Tool},
    service::RequestContext,
};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::instrument;

/// Deserialize tool arguments from the call params.
fn parse_args<T: serde::de::DeserializeOwned>(
    params: &CallToolRequestParams,
) -> Result<T, ErrorData> {
    let value = serde_json::Value::Object(params.arguments.clone().unwrap_or_default());
    serde_json::from_value(value).map_err(|e| ErrorData::invalid_params(e.to_string(), None))
}

/// Build a [`Tool`] with a typed input schema.
fn typed_tool<T: JsonSchema + 'static>(name: &'static str, description: &'static str) -> Tool {
    use std::sync::Arc;
    Tool::new(name, description, Arc::new(Default::default())).with_input_schema::<T>()
}

/// Parameters for `serde__serialize`.
#[derive(Debug, Deserialize, JsonSchema)]
struct SerializeParams {
    /// The registered type name (reserved for future type-registry lookup).
    type_name: String,
    /// JSON string representing the value to serialize.
    value_json: String,
}

/// Parameters for `serde__deserialize`.
#[derive(Debug, Deserialize, JsonSchema)]
struct DeserializeParams {
    /// The registered type name (reserved for future type-registry validation).
    type_name: String,
    /// JSON string to parse.
    json: String,
}

/// Parameters for `serde__round_trip_check`.
#[derive(Debug, Deserialize, JsonSchema)]
struct RoundTripParams {
    /// The registered type name (reserved for future type-registry validation).
    type_name: String,
    /// JSON string to round-trip through parse → serialize → parse.
    json: String,
}

/// MCP plugin exposing JSON serialization operations as tools.
///
/// Register under the `"serde"` namespace:
///
/// ```rust,no_run
/// use elicitation::PluginRegistry;
/// use elicit_serde::SerdePlugin;
///
/// let registry = PluginRegistry::new()
///     .register("serde", SerdePlugin);
/// ```
///
/// Exposes four tools: `serde__serialize`, `serde__deserialize`,
/// `serde__round_trip_check`, and `serde__list_formats`.
#[derive(Debug)]
pub struct SerdePlugin;

impl ElicitPlugin for SerdePlugin {
    fn name(&self) -> &'static str {
        "serde"
    }

    fn list_tools(&self) -> Vec<Tool> {
        vec![
            typed_tool::<SerializeParams>(
                "serialize",
                "Parse value_json as a JSON value and re-emit it as canonical compact JSON. \
                 The type_name parameter is reserved for future type-registry lookup.",
            ),
            typed_tool::<DeserializeParams>(
                "deserialize",
                "Parse json as a JSON value and return it. \
                 The type_name parameter is reserved for future type-registry validation.",
            ),
            typed_tool::<RoundTripParams>(
                "round_trip_check",
                "Parse json → re-serialize → re-parse and return true if both parses produce \
                 equal values (i.e., no data is lost in a JSON round-trip).",
            ),
            Tool::new(
                "list_formats",
                "List the serialization formats supported by this plugin.",
                std::sync::Arc::new(Default::default()),
            ),
        ]
    }

    #[instrument(skip(self, _ctx), fields(tool = %params.name))]
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        _ctx: RequestContext<rmcp::RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>> {
        Box::pin(async move {
            match params.name.as_ref() {
                "serialize" => {
                    let p: SerializeParams = parse_args(&params)?;
                    tracing::debug!(type_name = %p.type_name, "serde serialize");
                    match serde_json::from_str::<serde_json::Value>(&p.value_json) {
                        Ok(v) => match serde_json::to_string(&v) {
                            Ok(canonical) => {
                                Ok(CallToolResult::success(vec![Content::text(canonical)]))
                            }
                            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "deserialize" => {
                    let p: DeserializeParams = parse_args(&params)?;
                    tracing::debug!(type_name = %p.type_name, "serde deserialize");
                    match serde_json::from_str::<serde_json::Value>(&p.json) {
                        Ok(v) => match serde_json::to_string(&v) {
                            Ok(s) => Ok(CallToolResult::success(vec![Content::text(s)])),
                            Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                        },
                        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
                    }
                }
                "round_trip_check" => {
                    let p: RoundTripParams = parse_args(&params)?;
                    tracing::debug!(type_name = %p.type_name, "serde round_trip_check");
                    let first = match serde_json::from_str::<serde_json::Value>(&p.json) {
                        Ok(v) => v,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
                        }
                    };
                    let serialized = match serde_json::to_string(&first) {
                        Ok(s) => s,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
                        }
                    };
                    let second = match serde_json::from_str::<serde_json::Value>(&serialized) {
                        Ok(v) => v,
                        Err(e) => {
                            return Ok(CallToolResult::error(vec![Content::text(e.to_string())]));
                        }
                    };
                    Ok(CallToolResult::success(vec![Content::text(
                        (first == second).to_string(),
                    )]))
                }
                "list_formats" => {
                    tracing::debug!("serde list_formats");
                    let formats = serde_json::json!(["json"]);
                    Ok(CallToolResult::success(vec![Content::text(
                        formats.to_string(),
                    )]))
                }
                other => Err(ErrorData::invalid_params(
                    format!("unknown tool: {other}"),
                    None,
                )),
            }
        })
    }
}
