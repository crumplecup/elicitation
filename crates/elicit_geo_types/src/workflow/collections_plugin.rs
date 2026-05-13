//! `GeoTypesCollectionsPlugin` — MCP tools for collection geo-types.

use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, GeoMultiPolygon, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    Coord, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon,
};

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a line string was successfully created.
#[derive(Prop)]
pub struct LineStringCreated;
impl VerifiedWorkflow for LineStringCreated {}

/// Proposition: a multi-point was successfully created.
#[derive(Prop)]
pub struct MultiPointCreated;
impl VerifiedWorkflow for MultiPointCreated {}

/// Proposition: a multi-line string was successfully created.
#[derive(Prop)]
pub struct MultiLineStringCreated;
impl VerifiedWorkflow for MultiLineStringCreated {}

/// Proposition: a multi-polygon was successfully created.
#[derive(Prop)]
pub struct MultiPolygonCreated;
impl VerifiedWorkflow for MultiPolygonCreated {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for creating a line string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateLineStringParams {
    /// Ordered list of coordinates.
    pub coords: Vec<Coord>,
}

/// Parameters for inspecting a line string (coords count).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineStringCoordsCountParams {
    /// The line string to inspect.
    pub line_string: LineString,
}

/// Parameters for inspecting a line string (is closed).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineStringIsClosedParams {
    /// The line string to inspect.
    pub line_string: LineString,
}

/// Parameters for creating a multi-point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateMultiPointParams {
    /// List of points.
    pub points: Vec<Point>,
}

/// Parameters for inspecting a multi-point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiPointParams {
    /// The multi-point to inspect.
    pub multi_point: MultiPoint,
}

/// Parameters for creating a multi-line string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateMultiLineStringParams {
    /// List of line strings.
    pub lines: Vec<LineString>,
}

/// Parameters for inspecting a multi-line string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiLineStringParams {
    /// The multi-line string to inspect.
    pub multi_line_string: MultiLineString,
}

/// Parameters for creating a multi-polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateMultiPolygonParams {
    /// List of polygons.
    pub polygons: Vec<Polygon>,
}

/// Parameters for inspecting a multi-polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiPolygonParams {
    /// The multi-polygon to inspect.
    pub multi_polygon: MultiPolygon,
}

/// Parameters for inspecting a geometry collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryCollectionParams {
    /// The geometry collection to inspect.
    pub geometry_collection: GeometryCollection,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "create_line_string",
    description = "Create a LineString from an ordered list of coordinates. Establishes: LineStringCreated."
)]
#[instrument]
async fn create_line_string(p: CreateLineStringParams) -> Result<CallToolResult, ErrorData> {
    let ls = LineString::new(p.coords);
    let _proof = Established::<LineStringCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&ls).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "line_string_coords_count",
    description = "Returns the number of coordinates in a LineString."
)]
#[instrument]
async fn line_string_coords_count(
    p: LineStringCoordsCountParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.line_string.coords_count().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "line_string_is_closed",
    description = "Returns true if the LineString is closed (first and last coordinates are equal)."
)]
#[instrument]
async fn line_string_is_closed(p: LineStringIsClosedParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.line_string.is_closed().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "create_multi_point",
    description = "Create a MultiPoint from a list of points. Establishes: MultiPointCreated."
)]
#[instrument]
async fn create_multi_point(p: CreateMultiPointParams) -> Result<CallToolResult, ErrorData> {
    let mp = MultiPoint::new(p.points);
    let _proof = Established::<MultiPointCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&mp).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "multi_point_count",
    description = "Returns the number of points in a MultiPoint."
)]
#[instrument]
async fn multi_point_count(p: MultiPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.multi_point.count().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "create_multi_line_string",
    description = "Create a MultiLineString from a list of line strings. Establishes: MultiLineStringCreated."
)]
#[instrument]
async fn create_multi_line_string(
    p: CreateMultiLineStringParams,
) -> Result<CallToolResult, ErrorData> {
    let mls = MultiLineString::new(p.lines);
    let _proof = Established::<MultiLineStringCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&mls).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "multi_line_string_count",
    description = "Returns the number of line strings in a MultiLineString."
)]
#[instrument]
async fn multi_line_string_count(p: MultiLineStringParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.multi_line_string.count().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "create_multi_polygon",
    description = "Create a MultiPolygon from a list of polygons. Establishes: MultiPolygonCreated."
)]
#[instrument]
async fn create_multi_polygon(p: CreateMultiPolygonParams) -> Result<CallToolResult, ErrorData> {
    let geo_polys = p.polygons.into_iter().map(|poly| (*poly).clone()).collect();
    let mp = MultiPolygon::from(GeoMultiPolygon(geo_polys));
    let _proof = Established::<MultiPolygonCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&mp).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "multi_polygon_count",
    description = "Returns the number of polygons in a MultiPolygon."
)]
#[instrument]
async fn multi_polygon_count(p: MultiPolygonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.multi_polygon.count().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_collections",
    name = "geometry_collection_count",
    description = "Returns the number of geometries in a GeometryCollection."
)]
#[instrument]
async fn geometry_collection_count(
    p: GeometryCollectionParams,
) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.geometry_collection.count().to_string(),
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// The geo-types collections MCP plugin.
///
/// Provides tools for creating and inspecting collection types:
/// `LineString`, `MultiPoint`, `MultiLineString`, `MultiPolygon`, `GeometryCollection`.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_types_collections")]
pub struct GeoTypesCollectionsPlugin;
