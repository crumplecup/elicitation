//! `GeoValidationPlugin` — MCP tools for geometry validation and inspection.

use elicit_geo_types::{Geometry, LineString, Polygon};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a validation check was completed.
#[derive(Prop)]
pub struct ValidationChecked;
impl VerifiedWorkflow for ValidationChecked {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for linestring is_closed.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringIsClosedParams {
    /// The linestring to inspect.
    pub linestring: LineString,
}

/// Parameters for linestring is_convex.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringIsConvexParams {
    /// The linestring to inspect.
    pub linestring: LineString,
}

/// Parameters for linestring coords count.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringCoordsCountParams {
    /// The linestring to inspect.
    pub linestring: LineString,
}

/// Parameters for polygon coords count.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonCoordsCountParams {
    /// The polygon to inspect.
    pub polygon: Polygon,
}

/// Parameters for geometry type inspection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryTypeParams {
    /// The geometry to inspect.
    pub geometry: Geometry,
}

/// Parameters for polygon exterior coords count.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonExteriorCoordsCountParams {
    /// The polygon to inspect.
    pub polygon: Polygon,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_validation",
    name = "linestring_is_closed",
    description = "Check if a linestring is closed (first and last points are equal). Returns \"true\" or \"false\"."
)]
#[instrument]
async fn linestring_is_closed(p: LinestringIsClosedParams) -> Result<CallToolResult, ErrorData> {
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = raw_ls.is_closed();
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_validation",
    name = "linestring_is_convex",
    description = "Check if a linestring is convex. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn linestring_is_convex(p: LinestringIsConvexParams) -> Result<CallToolResult, ErrorData> {
    use geo::IsConvex;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = raw_ls.is_convex();
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_validation",
    name = "linestring_coords_count",
    description = "Count the number of coordinates in a linestring. Returns a usize as a string."
)]
#[instrument]
async fn linestring_coords_count(
    p: LinestringCoordsCountParams,
) -> Result<CallToolResult, ErrorData> {
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = raw_ls.coords().count();
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_validation",
    name = "polygon_coords_count",
    description = "Count the total number of coordinates in a polygon (exterior + all interiors). Returns a usize as a string."
)]
#[instrument]
async fn polygon_coords_count(p: PolygonCoordsCountParams) -> Result<CallToolResult, ErrorData> {
    use geo::CoordsIter;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw_polygon.coords_count();
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_validation",
    name = "geometry_type_name",
    description = "Return the type name of a geometry as a string (e.g. \"Polygon\", \"Point\", \"LineString\")."
)]
#[instrument]
async fn geometry_type_name(p: GeometryTypeParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoGeometry;
    let geo_geom = (*p.geometry).clone();
    let name = match geo_geom {
        GeoGeometry::Point(_) => "Point",
        GeoGeometry::Line(_) => "Line",
        GeoGeometry::LineString(_) => "LineString",
        GeoGeometry::Polygon(_) => "Polygon",
        GeoGeometry::MultiPoint(_) => "MultiPoint",
        GeoGeometry::MultiLineString(_) => "MultiLineString",
        GeoGeometry::MultiPolygon(_) => "MultiPolygon",
        GeoGeometry::GeometryCollection(_) => "GeometryCollection",
        GeoGeometry::Rect(_) => "Rect",
        GeoGeometry::Triangle(_) => "Triangle",
    };
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        name.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_validation",
    name = "polygon_exterior_coords_count",
    description = "Count the number of coordinates in the exterior ring of a polygon. Returns a usize as a string."
)]
#[instrument]
async fn polygon_exterior_coords_count(
    p: PolygonExteriorCoordsCountParams,
) -> Result<CallToolResult, ErrorData> {
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw_polygon.exterior().0.len();
    let _proof = Established::<ValidationChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

/// Plugin exposing geometry validation and inspection tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_validation")]
pub struct GeoValidationPlugin;
