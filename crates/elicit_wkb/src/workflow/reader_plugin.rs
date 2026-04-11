//! `WkbReaderPlugin` — parse and inspect WKB reader values.

use crate::{Wkb, read_wkb};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: WKB bytes were successfully parsed into a reader value.
#[derive(Prop)]
pub struct WkbParsed;

impl VerifiedWorkflow for WkbParsed {}

/// Parameters for parsing WKB bytes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadWkbParams {
    /// Validated WKB bytes to parse.
    pub bytes: elicitation::WkbBytes,
}

/// Parameters for parsing WKB bytes via `Wkb::try_new`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WkbTryNewParams {
    /// Validated WKB bytes to parse.
    pub bytes: elicitation::WkbBytes,
}

/// Parameters for inspecting endianness on a parsed WKB value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WkbEndiannessParams {
    /// Parsed WKB wrapper to inspect.
    pub wkb: Wkb,
}

/// Parameters for inspecting dimension on a parsed WKB value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WkbDimensionParams {
    /// Parsed WKB wrapper to inspect.
    pub wkb: Wkb,
}

/// Parameters for inspecting geometry type on a parsed WKB value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WkbGeometryTypeParams {
    /// Parsed WKB wrapper to inspect.
    pub wkb: Wkb,
}

#[elicit_tool(
    plugin = "wkb_reader",
    name = "read_wkb",
    description = "Parse validated WKB bytes into a Wkb value. Establishes: WkbParsed."
)]
#[instrument]
async fn read_wkb_tool(p: ReadWkbParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let wkb = read_wkb(&p.bytes.bytes)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbParsed>::assert();
    json_result(&wkb)
}

#[elicit_tool(
    plugin = "wkb_reader",
    name = "wkb_try_new",
    description = "Parse validated WKB bytes into a Wkb value via Wkb::try_new. Establishes: WkbParsed."
)]
#[instrument]
async fn wkb_try_new(p: WkbTryNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let wkb = Wkb::try_new(&p.bytes.bytes)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbParsed>::assert();
    json_result(&wkb)
}

#[elicit_tool(
    plugin = "wkb_reader",
    name = "wkb_endianness",
    description = "Return the byte order encoded in a parsed WKB value."
)]
#[instrument]
async fn wkb_endianness(p: WkbEndiannessParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.wkb.endianness())
}

#[elicit_tool(
    plugin = "wkb_reader",
    name = "wkb_dimension",
    description = "Return the coordinate dimension reported by a parsed WKB value."
)]
#[instrument]
async fn wkb_dimension(p: WkbDimensionParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.wkb.dimension())
}

#[elicit_tool(
    plugin = "wkb_reader",
    name = "wkb_geometry_type",
    description = "Return the geometry type reported by a parsed WKB value."
)]
#[instrument]
async fn wkb_geometry_type(
    p: WkbGeometryTypeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&p.wkb.geometry_type())
}

/// The WKB reader MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wkb_reader")]
pub struct WkbReaderPlugin;

impl WkbReaderPlugin {
    /// Creates a new WKB reader plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for WkbReaderPlugin {
    fn default() -> Self {
        Self::new()
    }
}
