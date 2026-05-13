//! `GeoPredicatesPlugin` — predicate MCP tools (contains, intersects, within, covers).

use elicit_geo_types::{LineString, Point, Polygon, Rect};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Propositions ──────────────────────────────────────────────────────────────

/// Proposition: a predicate check was completed.
#[derive(Prop)]
pub struct PredicateChecked;
impl VerifiedWorkflow for PredicateChecked {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for rect_contains_point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectContainsPointParams {
    /// The rectangle.
    pub rect: Rect,
    /// The point to test.
    pub point: Point,
}

/// Parameters for polygon_contains_point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonContainsPointParams {
    /// The polygon.
    pub polygon: Polygon,
    /// The point to test.
    pub point: Point,
}

/// Parameters for polygon_contains_linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonContainsLinestringParams {
    /// The polygon.
    pub polygon: Polygon,
    /// The linestring to test.
    pub linestring: LineString,
}

/// Parameters for polygon_contains_polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonContainsPolygonParams {
    /// The container polygon.
    pub container: Polygon,
    /// The geometry to test.
    pub geometry: Polygon,
}

/// Parameters for rect_intersects_rect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RectIntersectsRectParams {
    /// The first rectangle.
    pub rect1: Rect,
    /// The second rectangle.
    pub rect2: Rect,
}

/// Parameters for polygon_intersects_linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonIntersectsLinestringParams {
    /// The polygon.
    pub polygon: Polygon,
    /// The linestring to test.
    pub linestring: LineString,
}

/// Parameters for polygon_intersects_polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonIntersectsPolygonParams {
    /// The first polygon.
    pub polygon1: Polygon,
    /// The second polygon.
    pub polygon2: Polygon,
}

/// Parameters for linestring_intersects_linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringIntersectsLinestringParams {
    /// The first linestring.
    pub linestring1: LineString,
    /// The second linestring.
    pub linestring2: LineString,
}

/// Parameters for point_within_polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointWithinPolygonParams {
    /// The point to test.
    pub point: Point,
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for point_within_rect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointWithinRectParams {
    /// The point to test.
    pub point: Point,
    /// The rectangle.
    pub rect: Rect,
}

/// Parameters for polygon_covers_point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonCoversPointParams {
    /// The polygon.
    pub polygon: Polygon,
    /// The point to test.
    pub point: Point,
}

/// Parameters for polygon_covers_linestring.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonCoversLinestringParams {
    /// The polygon.
    pub polygon: Polygon,
    /// The linestring to test.
    pub linestring: LineString,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_predicates",
    name = "rect_contains_point",
    description = "Check if a rectangle contains a point. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn rect_contains_point(p: RectContainsPointParams) -> Result<CallToolResult, ErrorData> {
    use geo::Contains;
    let raw_rect: geo_types::Rect<f64> = geo_types::Rect::from(*p.rect);
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result = raw_rect.contains(&raw_point);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_contains_point",
    description = "Check if a polygon contains a point. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_contains_point(
    p: PolygonContainsPointParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Contains;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result = raw_polygon.contains(&raw_point);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_contains_linestring",
    description = "Check if a polygon contains a linestring. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_contains_linestring(
    p: PolygonContainsLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Contains;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_linestring: geo_types::LineString<f64> =
        geo_types::LineString::from((*p.linestring).clone());
    let result = raw_polygon.contains(&raw_linestring);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_contains_polygon",
    description = "Check if one polygon contains another polygon. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_contains_polygon(
    p: PolygonContainsPolygonParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Contains;
    let raw_container: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.container).clone());
    let raw_geometry: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.geometry).clone());
    let result = raw_container.contains(&raw_geometry);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "rect_intersects_rect",
    description = "Check if two rectangles intersect. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn rect_intersects_rect(p: RectIntersectsRectParams) -> Result<CallToolResult, ErrorData> {
    use geo::Intersects;
    let raw_rect1: geo_types::Rect<f64> = geo_types::Rect::from(*p.rect1);
    let raw_rect2: geo_types::Rect<f64> = geo_types::Rect::from(*p.rect2);
    let result = raw_rect1.intersects(&raw_rect2);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_intersects_linestring",
    description = "Check if a polygon intersects a linestring. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_intersects_linestring(
    p: PolygonIntersectsLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Intersects;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_linestring: geo_types::LineString<f64> =
        geo_types::LineString::from((*p.linestring).clone());
    let result = raw_polygon.intersects(&raw_linestring);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_intersects_polygon",
    description = "Check if two polygons intersect. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_intersects_polygon(
    p: PolygonIntersectsPolygonParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Intersects;
    let raw_polygon1: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon1).clone());
    let raw_polygon2: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon2).clone());
    let result = raw_polygon1.intersects(&raw_polygon2);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "linestring_intersects_linestring",
    description = "Check if two linestrings intersect. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn linestring_intersects_linestring(
    p: LinestringIntersectsLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Intersects;
    let raw_ls1: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring1).clone());
    let raw_ls2: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring2).clone());
    let result = raw_ls1.intersects(&raw_ls2);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "point_within_polygon",
    description = "Check if a point is within a polygon. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn point_within_polygon(p: PointWithinPolygonParams) -> Result<CallToolResult, ErrorData> {
    use geo::Within;
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let result = raw_point.is_within(&raw_polygon);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "point_within_rect",
    description = "Check if a point is within a rectangle. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn point_within_rect(p: PointWithinRectParams) -> Result<CallToolResult, ErrorData> {
    use geo::Within;
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let raw_rect: geo_types::Rect<f64> = geo_types::Rect::from(*p.rect);
    let result = raw_point.is_within(&raw_rect);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_covers_point",
    description = "Check if a polygon covers a point. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_covers_point(p: PolygonCoversPointParams) -> Result<CallToolResult, ErrorData> {
    use geo::Covers;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_point: geo_types::Point<f64> = geo_types::Point::from(*p.point);
    let result = raw_polygon.covers(&raw_point);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_predicates",
    name = "polygon_covers_linestring",
    description = "Check if a polygon covers a linestring. Returns \"true\" or \"false\"."
)]
#[instrument]
async fn polygon_covers_linestring(
    p: PolygonCoversLinestringParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::Covers;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let raw_linestring: geo_types::LineString<f64> =
        geo_types::LineString::from((*p.linestring).clone());
    let result = raw_polygon.covers(&raw_linestring);
    let _proof = Established::<PredicateChecked>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

/// Plugin exposing geo predicate tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_predicates")]
pub struct GeoPredicatesPlugin;
