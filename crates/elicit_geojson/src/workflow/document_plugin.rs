//! `GeoJsonDocumentPlugin` — parse and inspect top-level GeoJSON documents.

use crate::GeoJson;
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::{json_result, text_result};

/// Proposition: a GeoJSON document was successfully parsed or constructed.
#[derive(Prop)]
pub struct GeoJsonDocumentParsed;

impl VerifiedWorkflow for GeoJsonDocumentParsed {}

/// Parameters for parsing a GeoJSON document from text.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonFromStrParams {
    /// Input GeoJSON text.
    pub geojson: String,
}

/// Parameters for constructing a GeoJSON document from a JSON value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonFromJsonValueParams {
    /// JSON value containing a GeoJSON document.
    pub value: serde_json::Value,
}

/// Parameters for converting a GeoJSON document to a JSON value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonToJsonValueParams {
    /// GeoJSON document to serialize.
    pub geojson: GeoJson,
}

/// Parameters for pretty-printing a GeoJSON document.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonToStringPrettyParams {
    /// GeoJSON document to format.
    pub geojson: GeoJson,
}

/// Parameters for reporting the top-level GeoJSON variant name.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeoJsonVariantParams {
    /// GeoJSON document to inspect.
    pub geojson: GeoJson,
}

#[elicit_tool(
    plugin = "geojson_document",
    name = "geojson_from_str",
    description = "Parse a GeoJSON document from text. Establishes: GeoJsonDocumentParsed."
)]
#[instrument]
async fn geojson_from_str(
    p: GeoJsonFromStrParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geojson = p
        .geojson
        .parse::<GeoJson>()
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonDocumentParsed>::assert();
    json_result(&geojson)
}

#[elicit_tool(
    plugin = "geojson_document",
    name = "geojson_from_json_value",
    description = "Construct a GeoJSON document from a JSON value. Establishes: GeoJsonDocumentParsed."
)]
#[instrument]
async fn geojson_from_json_value(
    p: GeoJsonFromJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geojson = GeoJson::from_json_value(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonDocumentParsed>::assert();
    json_result(&geojson)
}

#[elicit_tool(
    plugin = "geojson_document",
    name = "geojson_to_json_value",
    description = "Convert a GeoJSON document to a JSON value."
)]
#[instrument]
async fn geojson_to_json_value(
    p: GeoJsonToJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.geojson.to_json_value())
}

#[elicit_tool(
    plugin = "geojson_document",
    name = "geojson_to_string_pretty",
    description = "Serialize a GeoJSON document to pretty-printed JSON."
)]
#[instrument]
async fn geojson_to_string_pretty(
    p: GeoJsonToStringPrettyParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let text = p
        .geojson
        .to_string_pretty()
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    Ok(text_result(text))
}

#[elicit_tool(
    plugin = "geojson_document",
    name = "geojson_variant_name",
    description = "Return the top-level variant name of a GeoJSON document."
)]
#[instrument]
async fn geojson_variant_name(
    p: GeoJsonVariantParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let name = match p.geojson.as_ref() {
        geojson::GeoJson::Geometry(_) => "Geometry",
        geojson::GeoJson::Feature(_) => "Feature",
        geojson::GeoJson::FeatureCollection(_) => "FeatureCollection",
    };
    Ok(text_result(name))
}

/// The GeoJSON document MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geojson_document")]
pub struct GeoJsonDocumentPlugin;

impl GeoJsonDocumentPlugin {
    /// Creates a new GeoJSON document plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoJsonDocumentPlugin {
    fn default() -> Self {
        Self::new()
    }
}
