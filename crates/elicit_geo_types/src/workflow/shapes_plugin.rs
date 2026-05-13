//! `GeoTypesShapesPlugin` — MCP tools for shape geo-types.

use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{Coord, LineString, Polygon, Rect};

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a rectangle was successfully created.
#[derive(Prop)]
pub struct RectCreated;
impl VerifiedWorkflow for RectCreated {}

/// Proposition: a polygon was successfully created.
#[derive(Prop)]
pub struct PolygonCreated;
impl VerifiedWorkflow for PolygonCreated {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for creating a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRectParams {
    /// Minimum corner X.
    pub min_x: f64,
    /// Minimum corner Y.
    pub min_y: f64,
    /// Maximum corner X.
    pub max_x: f64,
    /// Maximum corner Y.
    pub max_y: f64,
}

/// Parameters for getting the width of a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectWidthParams {
    /// The rectangle to inspect.
    pub rect: Rect,
}

/// Parameters for getting the height of a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectHeightParams {
    /// The rectangle to inspect.
    pub rect: Rect,
}

/// Parameters for getting the center of a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectCenterParams {
    /// The rectangle to inspect.
    pub rect: Rect,
}

/// Parameters for getting the minimum corner of a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectMinParams {
    /// The rectangle to inspect.
    pub rect: Rect,
}

/// Parameters for getting the maximum corner of a rectangle.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectMaxParams {
    /// The rectangle to inspect.
    pub rect: Rect,
}

/// Parameters for creating a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreatePolygonParams {
    /// The exterior ring.
    pub exterior: LineString,
    /// Interior rings (holes), if any.
    pub interiors: Vec<LineString>,
}

/// Parameters for inspecting a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonParams {
    /// The polygon to inspect.
    pub polygon: Polygon,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "create_rect",
    description = "Create an axis-aligned rectangle from min/max corners. Establishes: RectCreated."
)]
#[instrument]
async fn create_rect(p: CreateRectParams) -> Result<CallToolResult, ErrorData> {
    let min = Coord::new(p.min_x, p.min_y);
    let max = Coord::new(p.max_x, p.max_y);
    let rect = Rect::new(min, max);
    let _proof = Established::<RectCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&rect).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "rect_width",
    description = "Returns the width (x-extent) of a rectangle."
)]
#[instrument]
async fn rect_width(p: RectWidthParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.rect.width().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "rect_height",
    description = "Returns the height (y-extent) of a rectangle."
)]
#[instrument]
async fn rect_height(p: RectHeightParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.rect.height().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "rect_center",
    description = "Returns the center coordinate of a rectangle."
)]
#[instrument]
async fn rect_center(p: RectCenterParams) -> Result<CallToolResult, ErrorData> {
    let center = p.rect.center();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&center)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "rect_min",
    description = "Returns the minimum (south-west) corner of a rectangle."
)]
#[instrument]
async fn rect_min(p: RectMinParams) -> Result<CallToolResult, ErrorData> {
    let min = p.rect.min();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&min).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "rect_max",
    description = "Returns the maximum (north-east) corner of a rectangle."
)]
#[instrument]
async fn rect_max(p: RectMaxParams) -> Result<CallToolResult, ErrorData> {
    let max = p.rect.max();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&max).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "create_polygon",
    description = "Create a polygon from an exterior ring and optional interior rings (holes). \
                   Establishes: PolygonCreated."
)]
#[instrument]
async fn create_polygon(p: CreatePolygonParams) -> Result<CallToolResult, ErrorData> {
    let polygon = Polygon::new(p.exterior, p.interiors);
    let _proof = Established::<PolygonCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&polygon)
            .map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_shapes",
    name = "polygon_interiors_count",
    description = "Returns the number of interior rings (holes) in a polygon."
)]
#[instrument]
async fn polygon_interiors_count(p: PolygonParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.polygon.interiors_count().to_string(),
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// The geo-types shapes MCP plugin.
///
/// Provides tools for creating and inspecting `Rect` and `Polygon` shapes.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_types_shapes")]
pub struct GeoTypesShapesPlugin;
