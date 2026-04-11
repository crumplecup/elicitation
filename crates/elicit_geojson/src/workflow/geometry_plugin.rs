//! `GeoJsonGeometryPlugin` — construct and inspect GeoJSON geometry values.

use crate::{Geometry, Value};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::{json_result, text_result};

/// Proposition: a GeoJSON geometry or geometry value was successfully created.
#[derive(Prop)]
pub struct GeoJsonGeometryCreated;

impl VerifiedWorkflow for GeoJsonGeometryCreated {}

/// Parameters for constructing a geometry object.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryNewParams {
    /// Geometry payload value.
    pub value: Value,
}

/// Parameters for constructing a point geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValuePointParams {
    /// Point coordinates.
    pub coordinates: Vec<f64>,
}

/// Parameters for constructing a multi-point geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueMultiPointParams {
    /// Multi-point coordinates.
    pub coordinates: Vec<Vec<f64>>,
}

/// Parameters for constructing a line string geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueLineStringParams {
    /// Line string coordinates.
    pub coordinates: Vec<Vec<f64>>,
}

/// Parameters for constructing a multi-line string geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueMultiLineStringParams {
    /// Multi-line string coordinates.
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

/// Parameters for constructing a polygon geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValuePolygonParams {
    /// Polygon coordinates.
    pub coordinates: Vec<Vec<Vec<f64>>>,
}

/// Parameters for constructing a multi-polygon geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueMultiPolygonParams {
    /// Multi-polygon coordinates.
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

/// Parameters for constructing a geometry-collection geometry value.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueGeometryCollectionParams {
    /// Geometries included in the collection.
    pub geometries: Vec<Geometry>,
}

/// Parameters for reporting the geometry-value variant name.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueTypeNameParams {
    /// Geometry value to inspect.
    pub value: Value,
}

/// Parameters for constructing a geometry from raw JSON.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryFromJsonValueParams {
    /// JSON value containing a geometry object.
    pub value: serde_json::Value,
}

/// Parameters for constructing a geometry value from raw JSON.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ValueFromJsonValueParams {
    /// JSON value containing a geometry object or geometry fragment.
    pub value: serde_json::Value,
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "geometry_new",
    description = "Create a GeoJSON geometry object from a geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn geometry_new(p: GeometryNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = Geometry::new(p.value);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_point",
    description = "Create a GeoJSON Point geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_point(p: ValuePointParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::point(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_multi_point",
    description = "Create a GeoJSON MultiPoint geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_multi_point(
    p: ValueMultiPointParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::multi_point(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_line_string",
    description = "Create a GeoJSON LineString geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_line_string(
    p: ValueLineStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::line_string(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_multi_line_string",
    description = "Create a GeoJSON MultiLineString geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_multi_line_string(
    p: ValueMultiLineStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::multi_line_string(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_polygon",
    description = "Create a GeoJSON Polygon geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_polygon(p: ValuePolygonParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::polygon(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_multi_polygon",
    description = "Create a GeoJSON MultiPolygon geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_multi_polygon(
    p: ValueMultiPolygonParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::multi_polygon(p.coordinates);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_geometry_collection",
    description = "Create a GeoJSON GeometryCollection geometry value. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_geometry_collection(
    p: ValueGeometryCollectionParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::geometry_collection(p.geometries);
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_type_name",
    description = "Return the geometry variant name of a GeoJSON value."
)]
#[instrument]
async fn value_type_name(p: ValueTypeNameParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    Ok(text_result(p.value.type_name()))
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "geometry_from_json_value",
    description = "Construct a GeoJSON geometry object from raw JSON. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn geometry_from_json_value(
    p: GeometryFromJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let geometry = Geometry::from_json_value(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&geometry)
}

#[elicit_tool(
    plugin = "geojson_geometry",
    name = "value_from_json_value",
    description = "Construct a GeoJSON geometry value from raw JSON. Establishes: GeoJsonGeometryCreated."
)]
#[instrument]
async fn value_from_json_value(
    p: ValueFromJsonValueParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let value = Value::from_json_value(p.value)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<GeoJsonGeometryCreated>::assert();
    json_result(&value)
}

/// The GeoJSON geometry MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geojson_geometry")]
pub struct GeoJsonGeometryPlugin;

impl GeoJsonGeometryPlugin {
    /// Creates a new GeoJSON geometry plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for GeoJsonGeometryPlugin {
    fn default() -> Self {
        Self::new()
    }
}
