//! `GeoTypesGeometryPlugin` — MCP tools for the top-level `Geometry` enum.

use elicitation::contracts::Established;
use elicitation::{
    ElicitPlugin, GeoCoord, GeoGeometry, GeoGeometryCollection, GeoPoint, GeoRect, Prop,
    VerifiedWorkflow, elicit_tool,
};
use rmcp::{
    ErrorData,
    model::{CallToolResult, Content},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::Geometry;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a geometry value was successfully created.
#[derive(Prop)]
pub struct GeometryCreated;
impl VerifiedWorkflow for GeometryCreated {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for creating a point geometry.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreatePointGeometryParams {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// Parameters for creating a rect geometry.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateRectGeometryParams {
    /// Minimum corner X.
    pub min_x: f64,
    /// Minimum corner Y.
    pub min_y: f64,
    /// Maximum corner X.
    pub max_x: f64,
    /// Maximum corner Y.
    pub max_y: f64,
}

/// Parameters for getting the type of a geometry.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryTypeParams {
    /// The geometry to inspect.
    pub geometry: Geometry,
}

/// Parameters for checking if a geometry is a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IsPointParams {
    /// The geometry to inspect.
    pub geometry: Geometry,
}

/// Parameters for checking if a geometry is a collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IsCollectionParams {
    /// The geometry to inspect.
    pub geometry: Geometry,
}

/// Parameters for creating an empty geometry collection (no fields required).
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateEmptyCollectionParams {}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "create_point_geometry",
    description = "Create a Geometry::Point variant from x and y values. Establishes: GeometryCreated."
)]
#[instrument]
async fn create_point_geometry(p: CreatePointGeometryParams) -> Result<CallToolResult, ErrorData> {
    let inner = GeoGeometry::Point(GeoPoint {
        coord: GeoCoord { x: p.x, y: p.y },
    });
    let g = Geometry::from(inner);
    let _proof = Established::<GeometryCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&g).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "create_rect_geometry",
    description = "Create a Geometry::Rect variant from min/max corners. Establishes: GeometryCreated."
)]
#[instrument]
async fn create_rect_geometry(p: CreateRectGeometryParams) -> Result<CallToolResult, ErrorData> {
    let inner = GeoGeometry::Rect(GeoRect {
        min: GeoCoord {
            x: p.min_x,
            y: p.min_y,
        },
        max: GeoCoord {
            x: p.max_x,
            y: p.max_y,
        },
    });
    let g = Geometry::from(inner);
    let _proof = Established::<GeometryCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&g).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "geometry_type",
    description = "Returns the variant name of a geometry (e.g. \"Point\", \"Polygon\")."
)]
#[instrument]
async fn geometry_type(p: GeometryTypeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.geometry.geometry_type(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "is_point",
    description = "Returns true if the geometry is a Point variant."
)]
#[instrument]
async fn is_point(p: IsPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.geometry.is_point().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "is_collection",
    description = "Returns true if the geometry is a multi- or collection variant."
)]
#[instrument]
async fn is_collection(p: IsCollectionParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(
        p.geometry.is_collection().to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_types_geometry",
    name = "create_empty_collection",
    description = "Create an empty Geometry::GeometryCollection. Establishes: GeometryCreated."
)]
#[instrument]
async fn create_empty_collection(
    _p: CreateEmptyCollectionParams,
) -> Result<CallToolResult, ErrorData> {
    let inner = GeoGeometry::GeometryCollection(GeoGeometryCollection(vec![]));
    let g = Geometry::from(inner);
    let _proof = Established::<GeometryCreated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&g).map_err(|e| ErrorData::internal_error(e.to_string(), None))?,
    )]))
}

// ── Plugin ────────────────────────────────────────────────────────────────────

/// The geo-types geometry MCP plugin.
///
/// Provides tools for creating and inspecting `Geometry` enum values.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_types_geometry")]
pub struct GeoTypesGeometryPlugin;
