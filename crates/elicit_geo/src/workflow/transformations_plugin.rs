//! `GeoTransformationsPlugin` — MCP tools for translate, rotate, scale, skew, simplify,
//! densify, smooth, map_coords, and remove_repeated_points.

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

// ── Skew ──────────────────────────────────────────────────────────────────────

/// Parameters for skewing a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SkewLinestringParams {
    /// The linestring to skew.
    pub linestring: LineString,
    /// Skew angle along the x axis in degrees.
    pub x_degrees: f64,
    /// Skew angle along the y axis in degrees.
    pub y_degrees: f64,
}

/// Parameters for skewing a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SkewPolygonParams {
    /// The polygon to skew.
    pub polygon: Polygon,
    /// Skew angle along the x axis in degrees.
    pub x_degrees: f64,
    /// Skew angle along the y axis in degrees.
    pub y_degrees: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "skew_linestring",
    description = "Skew a linestring by independent x and y shear angles (degrees). Returns a JSON LineString."
)]
#[instrument]
async fn skew_linestring(p: SkewLinestringParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::Skew;
    let raw: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = raw.skew_xy(p.x_degrees, p.y_degrees);
    let wrapped = LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "skew_polygon",
    description = "Skew a polygon by independent x and y shear angles (degrees). Returns a JSON Polygon."
)]
#[instrument]
async fn skew_polygon(p: SkewPolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Skew;
    let raw: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw.skew_xy(p.x_degrees, p.y_degrees);
    let wrapped = Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

// ── ChaikinSmoothing ──────────────────────────────────────────────────────────

/// Parameters for smoothing a linestring via Chaikin's algorithm.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SmoothLinestringParams {
    /// The linestring to smooth.
    pub linestring: LineString,
    /// Number of smoothing iterations. Each iteration doubles the vertex count.
    pub n_iterations: u32,
}

/// Parameters for smoothing a polygon via Chaikin's algorithm.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SmoothPolygonParams {
    /// The polygon to smooth.
    pub polygon: Polygon,
    /// Number of smoothing iterations. Each iteration doubles the vertex count.
    pub n_iterations: u32,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "smooth_linestring",
    description = "Smooth a linestring using Chaikin's algorithm. Each iteration doubles vertex count; \
                   consider simplifying afterward. Returns a JSON LineString."
)]
#[instrument]
async fn smooth_linestring(p: SmoothLinestringParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::ChaikinSmoothing;
    let raw: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = raw.chaikin_smoothing(p.n_iterations as usize);
    let wrapped = LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "smooth_polygon",
    description = "Smooth a polygon using Chaikin's algorithm. Each iteration doubles vertex count; \
                   consider simplifying afterward. Returns a JSON Polygon."
)]
#[instrument]
async fn smooth_polygon(p: SmoothPolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::ChaikinSmoothing;
    let raw: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw.chaikin_smoothing(p.n_iterations as usize);
    let wrapped = Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

// ── Densify ───────────────────────────────────────────────────────────────────

/// Parameters for densifying a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DensifyLinestringParams {
    /// The linestring to densify.
    pub linestring: LineString,
    /// Maximum Euclidean segment length (same units as coordinates). Must be > 0.
    pub max_segment_length: f64,
}

/// Parameters for densifying a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DensifyPolygonParams {
    /// The polygon to densify.
    pub polygon: Polygon,
    /// Maximum Euclidean segment length (same units as coordinates). Must be > 0.
    pub max_segment_length: f64,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "densify_linestring",
    description = "Add intermediate points to a linestring so no segment exceeds max_segment_length \
                   (Euclidean). For geodetic coordinates use max_segment_length in meters and note \
                   that Euclidean is approximate — prefer densify_haversine for lat/lng. \
                   Returns a JSON LineString."
)]
#[instrument]
async fn densify_linestring(p: DensifyLinestringParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoLineString;
    use geo::Densify;
    use geo::line_measures::Euclidean;
    let raw: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = Euclidean.densify(&raw, p.max_segment_length);
    let wrapped = LineString::from(GeoLineString::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "densify_polygon",
    description = "Add intermediate points to a polygon so no edge segment exceeds max_segment_length \
                   (Euclidean). Returns a JSON Polygon."
)]
#[instrument]
async fn densify_polygon(p: DensifyPolygonParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::Densify;
    use geo::line_measures::Euclidean;
    let raw: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = Euclidean.densify(&raw, p.max_segment_length);
    let wrapped = Polygon::from(GeoPolygon::from(result));
    let _proof = Established::<TransformationApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

// ── map_coords (code fragment) ────────────────────────────────────────────────

/// Parameters for emitting a map_coords code fragment for a point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MapCoordsPointParams {
    /// Rust expression for the variable holding the `Point` (e.g. `"my_point"`).
    pub point_var: String,
    /// Rust closure or fn expression transforming a `Coord<f64>` to `Coord<f64>`.
    /// Example: `"|coord: geo::Coord| geo::Coord { x: coord.x * scale, y: coord.y }"`
    pub transform_fn: String,
}

/// Parameters for emitting a map_coords code fragment for a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MapCoordsLinestringParams {
    /// Rust expression for the variable holding the `LineString` (e.g. `"my_ls"`).
    pub linestring_var: String,
    /// Rust closure or fn expression transforming a `Coord<f64>` to `Coord<f64>`.
    /// Example: `"|coord: geo::Coord| geo::Coord { x: coord.x + 1.0, y: coord.y + 1.0 }"`
    pub transform_fn: String,
}

/// Parameters for emitting a map_coords code fragment for a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MapCoordsPolygonParams {
    /// Rust expression for the variable holding the `Polygon` (e.g. `"my_polygon"`).
    pub polygon_var: String,
    /// Rust closure or fn expression transforming a `Coord<f64>` to `Coord<f64>`.
    /// Example: `"|coord: geo::Coord| geo::Coord { x: coord.x.to_radians(), y: coord.y.to_radians() }"`
    pub transform_fn: String,
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "map_coords_point",
    description = "Emit a Rust code fragment applying a coordinate transform closure to a Point. \
                   The transform_fn is a Rust closure `|coord: geo::Coord| -> geo::Coord { ... }`. \
                   Returns a Rust expression string, not a computed value."
)]
#[instrument]
async fn map_coords_point(p: MapCoordsPointParams) -> Result<CallToolResult, ErrorData> {
    let fragment = format!(
        "use geo::MapCoords;\nlet result = {}.map_coords({});",
        p.point_var, p.transform_fn
    );
    Ok(CallToolResult::success(vec![Content::text(fragment)]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "map_coords_linestring",
    description = "Emit a Rust code fragment applying a coordinate transform closure to a LineString. \
                   The transform_fn is a Rust closure `|coord: geo::Coord| -> geo::Coord { ... }`. \
                   Returns a Rust expression string, not a computed value."
)]
#[instrument]
async fn map_coords_linestring(p: MapCoordsLinestringParams) -> Result<CallToolResult, ErrorData> {
    let fragment = format!(
        "use geo::MapCoords;\nlet result = {}.map_coords({});",
        p.linestring_var, p.transform_fn
    );
    Ok(CallToolResult::success(vec![Content::text(fragment)]))
}

#[elicit_tool(
    plugin = "geo_transformations",
    name = "map_coords_polygon",
    description = "Emit a Rust code fragment applying a coordinate transform closure to a Polygon. \
                   The transform_fn is a Rust closure `|coord: geo::Coord| -> geo::Coord { ... }`. \
                   Returns a Rust expression string, not a computed value."
)]
#[instrument]
async fn map_coords_polygon(p: MapCoordsPolygonParams) -> Result<CallToolResult, ErrorData> {
    let fragment = format!(
        "use geo::MapCoords;\nlet result = {}.map_coords({});",
        p.polygon_var, p.transform_fn
    );
    Ok(CallToolResult::success(vec![Content::text(fragment)]))
}

/// Plugin exposing geometric transformation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_transformations")]
pub struct GeoTransformationsPlugin;
