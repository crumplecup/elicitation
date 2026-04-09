//! `GeoMeasurementsPlugin` — MCP tools for area, distance, and length measurements.

use elicit_geo_types::{Line, LineString, MultiPolygon, Point, Polygon};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a measurement was computed.
#[derive(Prop)]
pub struct MeasurementComputed;
impl VerifiedWorkflow for MeasurementComputed {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for polygon unsigned area.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonUnsignedAreaParams {
    /// The polygon to measure.
    pub polygon: Polygon,
}

/// Parameters for polygon signed area.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonSignedAreaParams {
    /// The polygon to measure.
    pub polygon: Polygon,
}

/// Parameters for rect area.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectAreaParams {
    /// The rectangle to measure.
    pub rect: elicit_geo_types::Rect,
}

/// Parameters for multipolygon area.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiPolygonAreaParams {
    /// The multipolygon to measure.
    pub multipolygon: MultiPolygon,
}

/// Parameters for line Euclidean length.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineEuclideanLengthParams {
    /// The line to measure.
    pub line: Line,
}

/// Parameters for linestring Euclidean length.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringEuclideanLengthParams {
    /// The linestring to measure.
    pub linestring: LineString,
}

/// Parameters for Euclidean distance between two points.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EuclideanDistancePointPointParams {
    /// The start point.
    pub from: Point,
    /// The end point.
    pub to: Point,
}

/// Parameters for Euclidean distance between a point and a linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EuclideanDistancePointLinestringParams {
    /// The point.
    pub point: Point,
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for Euclidean distance between a point and a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EuclideanDistancePointPolygonParams {
    /// The point.
    pub point: Point,
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for Hausdorff distance between two linestrings.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HausdorffDistanceLinestringsParams {
    /// The first linestring.
    pub linestring1: LineString,
    /// The second linestring.
    pub linestring2: LineString,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_measurements",
    name = "polygon_area",
    description = "Compute the unsigned area of a polygon."
)]
#[instrument]
async fn polygon_area(p: PolygonUnsignedAreaParams) -> Result<CallToolResult, ErrorData> {
    use geo::Area;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw_polygon.unsigned_area();
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "polygon_signed_area",
    description = "Compute the signed area of a polygon (positive = counter-clockwise)."
)]
#[instrument]
async fn polygon_signed_area(p: PolygonSignedAreaParams) -> Result<CallToolResult, ErrorData> {
    use geo::Area;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw_polygon.signed_area();
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "rect_area",
    description = "Compute the unsigned area of a rectangle."
)]
#[instrument]
async fn rect_area(p: RectAreaParams) -> Result<CallToolResult, ErrorData> {
    use geo::Area;
    let raw_rect: geo_types::Rect<f64> = geo_types::Rect::from(*p.rect);
    let result = raw_rect.unsigned_area();
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "multipolygon_area",
    description = "Compute the unsigned area of a multipolygon."
)]
#[instrument]
async fn multipolygon_area(p: MultiPolygonAreaParams) -> Result<CallToolResult, ErrorData> {
    use geo::Area;
    let raw_mp: geo_types::MultiPolygon<f64> =
        geo_types::MultiPolygon::from((*p.multipolygon).clone());
    let result = raw_mp.unsigned_area();
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "line_euclidean_length",
    description = "Compute the Euclidean length of a line segment."
)]
#[instrument]
async fn line_euclidean_length(p: LineEuclideanLengthParams) -> Result<CallToolResult, ErrorData> {
    use geo::{Euclidean, Length};
    let raw_line: geo_types::Line<f64> = geo_types::Line::from(*p.line);
    let result = Euclidean.length(&raw_line);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "linestring_euclidean_length",
    description = "Compute the Euclidean length of a linestring."
)]
#[instrument]
async fn linestring_euclidean_length(
    p: LinestringEuclideanLengthParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Euclidean, Length};
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = Euclidean.length(&raw_ls);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "euclidean_distance_point_point",
    description = "Compute the Euclidean distance between two points."
)]
#[instrument]
async fn euclidean_distance_point_point(
    p: EuclideanDistancePointPointParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Distance, Euclidean};
    let raw_from: geo_types::Point<f64> = geo_types::Point::from(*p.from);
    let raw_to: geo_types::Point<f64> = geo_types::Point::from(*p.to);
    let result = Euclidean.distance(raw_from, raw_to);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "euclidean_distance_point_linestring",
    description = "Compute the Euclidean distance from a point to a linestring."
)]
#[instrument]
async fn euclidean_distance_point_linestring(
    p: EuclideanDistancePointLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Distance, Euclidean};
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let result = Euclidean.distance(&raw_point, &raw_ls);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "euclidean_distance_point_polygon",
    description = "Compute the Euclidean distance from a point to a polygon."
)]
#[instrument]
async fn euclidean_distance_point_polygon(
    p: EuclideanDistancePointPolygonParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Distance, Euclidean};
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = Euclidean.distance(&raw_point, &raw_polygon);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_measurements",
    name = "hausdorff_distance_linestrings",
    description = "Compute the Hausdorff distance between two linestrings."
)]
#[instrument]
async fn hausdorff_distance_linestrings(
    p: HausdorffDistanceLinestringsParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::HausdorffDistance;
    let raw_ls1: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring1).clone());
    let raw_ls2: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring2).clone());
    let result = raw_ls1.hausdorff_distance(&raw_ls2);
    let _proof = Established::<MeasurementComputed>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

/// Plugin exposing geo measurement tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_measurements")]
pub struct GeoMeasurementsPlugin;
