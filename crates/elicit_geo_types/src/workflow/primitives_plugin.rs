//! `GeoTypesPrimitivesPlugin` — MCP tools for primitive geo-types.

use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Coord, Line, Point, Triangle};

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a coordinate was successfully created.
#[derive(Prop)]
pub struct CoordCreated;
impl VerifiedWorkflow for CoordCreated {}

/// Proposition: a point was successfully created.
#[derive(Prop)]
pub struct PointCreated;
impl VerifiedWorkflow for PointCreated {}

/// Proposition: a line segment was successfully created.
#[derive(Prop)]
pub struct LineCreated;
impl VerifiedWorkflow for LineCreated {}

/// Proposition: a triangle was successfully created.
#[derive(Prop)]
pub struct TriangleCreated;
impl VerifiedWorkflow for TriangleCreated {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for creating a coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateCoordParams {
    /// X coordinate (longitude for geographic data).
    pub x: f64,
    /// Y coordinate (latitude for geographic data).
    pub y: f64,
}

/// Parameters for creating a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreatePointParams {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// Parameters for creating a line segment.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateLineParams {
    /// Start X.
    pub x1: f64,
    /// Start Y.
    pub y1: f64,
    /// End X.
    pub x2: f64,
    /// End Y.
    pub y2: f64,
}

/// Parameters for creating a triangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateTriangleParams {
    /// First vertex X.
    pub x1: f64,
    /// First vertex Y.
    pub y1: f64,
    /// Second vertex X.
    pub x2: f64,
    /// Second vertex Y.
    pub y2: f64,
    /// Third vertex X.
    pub x3: f64,
    /// Third vertex Y.
    pub y3: f64,
}

/// Parameters for getting the x component of a coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoordXParams {
    /// The coordinate to inspect.
    pub coord: Coord,
}

/// Parameters for getting the y component of a coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoordYParams {
    /// The coordinate to inspect.
    pub coord: Coord,
}

/// Parameters for getting the x component of a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointXParams {
    /// The point to inspect.
    pub point: Point,
}

/// Parameters for getting the y component of a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointYParams {
    /// The point to inspect.
    pub point: Point,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "create_coord",
    description = "Create a 2D coordinate from x and y values. Establishes: CoordCreated."
)]
#[instrument]
async fn create_coord(p: CreateCoordParams) -> Result<CallToolResult, ErrorData> {
    let coord = Coord::new(p.x, p.y);
    let _proof = Established::<CoordCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&coord)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "create_point",
    description = "Create a geographic point from x and y values. Establishes: PointCreated."
)]
#[instrument]
async fn create_point(p: CreatePointParams) -> Result<CallToolResult, ErrorData> {
    let point = Point::new(p.x, p.y);
    let _proof = Established::<PointCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&point)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "create_line",
    description = "Create a line segment from two endpoints. Establishes: LineCreated."
)]
#[instrument]
async fn create_line(p: CreateLineParams) -> Result<CallToolResult, ErrorData> {
    let start = Coord::new(p.x1, p.y1);
    let end = Coord::new(p.x2, p.y2);
    let line = Line::new(start, end);
    let _proof = Established::<LineCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&line).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "create_triangle",
    description = "Create a triangle from three vertex coordinates. Establishes: TriangleCreated."
)]
#[instrument]
async fn create_triangle(p: CreateTriangleParams) -> Result<CallToolResult, ErrorData> {
    let v1 = Coord::new(p.x1, p.y1);
    let v2 = Coord::new(p.x2, p.y2);
    let v3 = Coord::new(p.x3, p.y3);
    let tri = Triangle::new(v1, v2, v3);
    let _proof = Established::<TriangleCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&tri).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "coord_x",
    description = "Returns the x (longitude) component of a coordinate."
)]
#[instrument]
async fn coord_x(p: CoordXParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.coord.x().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "coord_y",
    description = "Returns the y (latitude) component of a coordinate."
)]
#[instrument]
async fn coord_y(p: CoordYParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.coord.y().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "point_x",
    description = "Returns the x component of a point."
)]
#[instrument]
async fn point_x(p: PointXParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.point.x().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_primitives",
    name = "point_y",
    description = "Returns the y component of a point."
)]
#[instrument]
async fn point_y(p: PointYParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.point.y().to_string(),
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// The geo-types primitives MCP plugin.
///
/// Provides tools for creating and inspecting primitive geometric types:
/// `Coord`, `Point`, `Line`, and `Triangle`.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_types_primitives")]
pub struct GeoTypesPrimitivesPlugin;
