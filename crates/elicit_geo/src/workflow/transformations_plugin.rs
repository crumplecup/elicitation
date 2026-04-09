//! `GeoTransformationsPlugin` — MCP tools for translate, rotate, scale, simplify, and remove_repeated_points.

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

/// Proposition: a transformation was applied.
#[derive(Prop)]
pub struct TransformationApplied;
impl VerifiedWorkflow for TransformationApplied {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for translating a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TranslatePointParams {
    /// The point to translate.
    pub point: elicit_geo_types::Point,
    /// X offset.
    pub x_offset: f64,
    /// Y offset.
    pub y_offset: f64,
}

/// Parameters for translating a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TranslateLinestringParams {
    /// The linestring to translate.
    pub linestring: LineString,
    /// X offset.
    pub x_offset: f64,
    /// Y offset.
    pub y_offset: f64,
}

/// Parameters for translating a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TranslatePolygonParams {
    /// The polygon to translate.
    pub polygon: Polygon,
    /// X offset.
    pub x_offset: f64,
    /// Y offset.
    pub y_offset: f64,
}

/// Parameters for rotating a polygon around its centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RotatePolygonParams {
    /// The polygon to rotate.
    pub polygon: Polygon,
    /// Rotation angle in degrees.
    pub degrees: f64,
}

/// Parameters for rotating a linestring around its centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RotateLinestringParams {
    /// The linestring to rotate.
    pub linestring: LineString,
    /// Rotation angle in degrees.
    pub degrees: f64,
}

/// Parameters for scaling a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ScalePolygonParams {
    /// The polygon to scale.
    pub polygon: Polygon,
    /// X scale factor.
    pub scale_x: f64,
    /// Y scale factor.
    pub scale_y: f64,
}

/// Parameters for simplifying a linestring with Ramer-Douglas-Peucker.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SimplifyLinestringRdpParams {
    /// The linestring to simplify.
    pub linestring: LineString,
    /// Epsilon tolerance.
    pub epsilon: f64,
}

/// Parameters for simplifying a linestring with Visvalingam-Whyatt.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SimplifyLinestringVwParams {
    /// The linestring to simplify.
    pub linestring: LineString,
    /// Epsilon tolerance.
    pub epsilon: f64,
}

/// Parameters for simplifying a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SimplifyPolygonParams {
    /// The polygon to simplify.
    pub polygon: Polygon,
    /// Epsilon tolerance.
    pub epsilon: f64,
}

/// Parameters for removing repeated points from a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RemoveRepeatedPointsLinestringParams {
    /// The linestring.
    pub linestring: LineString,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_transformations",
    name = "translate_point",
    description = "Translate a point by (x_offset, y_offset). Returns a JSON Point."
)]
#[instrument]
async fn translate_point(p: TranslatePointParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::Translate;
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result: geo_types::Point<f64> = raw_point.translate(p.x_offset, p.y_offset);
    let wrapped: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "translate_linestring",
    description = "Translate a linestring by (x_offset, y_offset). Returns a JSON LineString."
)]
#[instrument]
async fn translate_linestring(p: TranslateLinestringParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::Translate;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: geo_types::LineString<f64> = raw_ls.translate(p.x_offset, p.y_offset);
    let wrapped: elicit_geo_types::LineString =
        elicit_geo_types::LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "translate_polygon",
    description = "Translate a polygon by (x_offset, y_offset). Returns a JSON Polygon."
)]
#[instrument]
async fn translate_polygon(p: TranslatePolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Translate;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: geo_types::Polygon<f64> = raw_polygon.translate(p.x_offset, p.y_offset);
    let wrapped: elicit_geo_types::Polygon =
        elicit_geo_types::Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "rotate_polygon_around_centroid",
    description = "Rotate a polygon around its centroid by the given degrees. Returns a JSON Polygon."
)]
#[instrument]
async fn rotate_polygon_around_centroid(
    p: RotatePolygonParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Rotate;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: geo_types::Polygon<f64> = raw_polygon.rotate_around_centroid(p.degrees);
    let wrapped: elicit_geo_types::Polygon =
        elicit_geo_types::Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "rotate_linestring_around_centroid",
    description = "Rotate a linestring around its centroid by the given degrees. Returns a JSON LineString."
)]
#[instrument]
async fn rotate_linestring_around_centroid(
    p: RotateLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::Rotate;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: geo_types::LineString<f64> = raw_ls.rotate_around_centroid(p.degrees);
    let wrapped: elicit_geo_types::LineString =
        elicit_geo_types::LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "scale_polygon",
    description = "Scale a polygon by separate x and y factors around its centroid. Returns a JSON Polygon."
)]
#[instrument]
async fn scale_polygon(p: ScalePolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Scale;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: geo_types::Polygon<f64> = raw_polygon.scale_xy(p.scale_x, p.scale_y);
    let wrapped: elicit_geo_types::Polygon =
        elicit_geo_types::Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "simplify_linestring",
    description = "Simplify a linestring using the Ramer-Douglas-Peucker algorithm. Returns a JSON LineString."
)]
#[instrument]
async fn simplify_linestring(p: SimplifyLinestringRdpParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::Simplify;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: geo_types::LineString<f64> = raw_ls.simplify(p.epsilon);
    let wrapped: elicit_geo_types::LineString =
        elicit_geo_types::LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "simplify_polygon",
    description = "Simplify a polygon using the Ramer-Douglas-Peucker algorithm. Returns a JSON Polygon."
)]
#[instrument]
async fn simplify_polygon(p: SimplifyPolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Simplify;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result: geo_types::Polygon<f64> = raw_polygon.simplify(p.epsilon);
    let wrapped: elicit_geo_types::Polygon =
        elicit_geo_types::Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "simplify_linestring_vw",
    description = "Simplify a linestring using the Visvalingam-Whyatt algorithm. Returns a JSON LineString."
)]
#[instrument]
async fn simplify_linestring_vw(
    p: SimplifyLinestringVwParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::SimplifyVw;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: geo_types::LineString<f64> = raw_ls.simplify_vw(p.epsilon);
    let wrapped: elicit_geo_types::LineString =
        elicit_geo_types::LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "remove_repeated_points_linestring",
    description = "Remove repeated consecutive points from a linestring. Returns a JSON LineString."
)]
#[instrument]
async fn remove_repeated_points_linestring(
    p: RemoveRepeatedPointsLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::RemoveRepeatedPoints;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result: geo_types::LineString<f64> = raw_ls.remove_repeated_points();
    let wrapped: elicit_geo_types::LineString =
        elicit_geo_types::LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

/// Plugin exposing geometric transformation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_transformations")]
pub struct GeoTransformationsPlugin;
