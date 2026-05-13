//! `WktParsePlugin` — parse and inspect structured WKT items.

use crate::WktItem;
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::{json_result, text_result};

/// Proposition: a WKT string was successfully parsed into a structured value.
#[derive(Prop)]
pub struct WktParsed;

impl VerifiedWorkflow for WktParsed {}

/// Parameters for parsing a WKT string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ParseWktParams {
    /// Input WKT text.
    pub wkt: String,
}

/// Parameters for re-serializing a parsed WKT item.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WktItemStringParams {
    /// Parsed WKT item to serialize.
    pub item: WktItem,
}

/// Parameters for inspecting the geometry type of a parsed WKT item.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WktItemGeometryTypeParams {
    /// Parsed WKT item to inspect.
    pub item: WktItem,
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_item_from_str",
    description = "Parse a WKT string into a structured WKT item. Establishes: WktParsed."
)]
#[instrument]
async fn wkt_item_from_str(p: ParseWktParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let item = p
        .wkt
        .parse::<WktItem>()
        .map_err(|error| ErrorData::invalid_params(error, None))?;
    let _proof = Established::<WktParsed>::assert();
    json_result(&item)
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_item_wkt_string",
    description = "Serialize a structured WKT item back to a WKT string."
)]
#[instrument]
async fn wkt_item_wkt_string(
    p: WktItemStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(text_result(p.item.wkt_string()))
}

#[elicit_tool(
    plugin = "wkt_parse",
    name = "wkt_item_geometry_type",
    description = "Return the geometry type name of a structured WKT item."
)]
#[instrument]
async fn wkt_item_geometry_type(
    p: WktItemGeometryTypeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(text_result(p.item.geometry_type()))
}

/// The WKT parsing MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wkt_parse")]
pub struct WktParsePlugin;

impl WktParsePlugin {
    /// Creates a new WKT parsing plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for WktParsePlugin {
    fn default() -> Self {
        Self::new()
    }
}
