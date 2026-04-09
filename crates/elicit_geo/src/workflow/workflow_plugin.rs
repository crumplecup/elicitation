//! `GeoWorkflowPlugin` — MCP tools for common geo workflow operations.

use elicit_geo_types::{LineString, Polygon};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

fn json_err(e: impl std::fmt::Display) -> rmcp::ErrorData {
    rmcp::ErrorData::internal_error(e.to_string(), None)
}

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a workflow operation was completed.
#[derive(Prop)]
pub struct WorkflowCompleted;
impl VerifiedWorkflow for WorkflowCompleted {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for point-in-polygon check.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointInPolygonParams {
    /// The point to test.
    pub point: elicit_geo_types::Point,
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for nearest point on linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NearestPointOnLinestringParams {
    /// The query point.
    pub point: elicit_geo_types::Point,
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for line interpolate point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineInterpolatePointParams {
    /// The linestring.
    pub linestring: LineString,
    /// Fraction along the line (0.0–1.0).
    pub fraction: f64,
}

/// Parameters for line locate point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineLocatePointParams {
    /// The linestring.
    pub linestring: LineString,
    /// The point to locate.
    pub point: elicit_geo_types::Point,
}

/// Parameters for polygon exterior length.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonExteriorLengthParams {
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for geodesic length of linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeodesicLengthLinestringParams {
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for haversine length of linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HaversineLengthLinestringParams {
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for Fréchet distance between two linestrings.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FrechetDistanceParams {
    /// The first linestring.
    pub linestring1: LineString,
    /// The second linestring.
    pub linestring2: LineString,
}

/// Parameters for removing repeated points from a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RemoveRepeatedPointsPolygonParams {
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for polygon interior point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonInteriorPointParams {
    /// The polygon.
    pub polygon: Polygon,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_workflow",
    name = "point_in_polygon",
    description = "Check if a point is inside a polygon. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn point_in_polygon(p: PointInPolygonParams) -> Result<CallToolResult, ErrorData> {
    use geo::Contains;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result = raw_polygon.contains(&raw_point);
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "nearest_point_on_linestring",
    description = "Find the nearest point on a linestring to a given point. Returns a JSON Point."
)]
#[instrument]
async fn nearest_point_on_linestring(
    p: NearestPointOnLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::ClosestPoint;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let closest = raw_ls.closest_point(&raw_point);
    let pt = match closest {
        geo::Closest::SinglePoint(pt) | geo::Closest::Intersection(pt) => pt,
        geo::Closest::Indeterminate => raw_point,
    };
    let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(pt));
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "line_interpolate_point",
    description = "Interpolate a point along a linestring at the given fraction (0.0–1.0). Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn line_interpolate_point(
    p: LineInterpolatePointParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::{Euclidean, InterpolateLine};
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: Option<geo_types::Point<f64>> =
        Euclidean.point_at_ratio_from_start(&raw_ls, p.fraction);
    let _proof = Established::<WorkflowCompleted>::assert();
    let text = match result {
        Some(pt) => {
            let wrapped: elicit_geo_types::Point =
                elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&wrapped).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "line_locate_point",
    description = "Locate the fraction (0.0–1.0) along a linestring closest to a point. Returns an f64 string or \"null\"."
)]
#[instrument]
async fn line_locate_point(p: LineLocatePointParams) -> Result<CallToolResult, ErrorData> {
    use geo::LineLocatePoint;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result: Option<f64> = raw_ls.line_locate_point(&raw_point);
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(match result {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    })]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "polygon_exterior_length",
    description = "Compute the Euclidean length of the exterior ring of a polygon."
)]
#[instrument]
async fn polygon_exterior_length(
    p: PolygonExteriorLengthParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Euclidean, Length};
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = Euclidean.length(raw_polygon.exterior());
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "geodesic_length_linestring",
    description = "Compute the geodesic length in meters of a linestring."
)]
#[instrument]
async fn geodesic_length_linestring(
    p: GeodesicLengthLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Geodesic, Length};
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = Geodesic.length(&raw_ls);
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "haversine_length_linestring",
    description = "Compute the haversine length in meters of a linestring."
)]
#[instrument]
async fn haversine_length_linestring(
    p: HaversineLengthLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Haversine, Length};
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = Haversine.length(&raw_ls);
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "frechet_distance_linestrings",
    description = "Compute the Fréchet distance between two linestrings."
)]
#[instrument]
async fn frechet_distance_linestrings(
    p: FrechetDistanceParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Euclidean;
    use geo::line_measures::FrechetDistance;
    let raw_ls1: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring1).clone());
    let raw_ls2: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring2).clone());
    let result = Euclidean.frechet_distance(&raw_ls1, &raw_ls2);
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "remove_repeated_points_polygon",
    description = "Remove repeated consecutive points from a polygon. Returns a JSON Polygon."
)]
#[instrument]
async fn remove_repeated_points_polygon(
    p: RemoveRepeatedPointsPolygonParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::RemoveRepeatedPoints;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: geo_types::Polygon<f64> = raw_polygon.remove_repeated_points();
    let wrapped: elicit_geo_types::Polygon =
        elicit_geo_types::Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<WorkflowCompleted>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_workflow",
    name = "polygon_interior_point",
    description = "Find a guaranteed interior point of a polygon. Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn polygon_interior_point(
    p: PolygonInteriorPointParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::InteriorPoint;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: Option<geo_types::Point<f64>> = raw_polygon.interior_point();
    let _proof = Established::<WorkflowCompleted>::assert();
    let text = match result {
        Some(pt) => {
            let wrapped: elicit_geo_types::Point =
                elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&wrapped).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

/// Plugin exposing common geo workflow tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_workflow")]
pub struct GeoWorkflowPlugin;
