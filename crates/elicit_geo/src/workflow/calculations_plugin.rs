//! `GeoCalculationsPlugin` — MCP tools for centroid, bounding rect, and convex hull.

use elicit_geo_types::{LineString, MultiPoint, MultiPolygon, Polygon};
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

/// Proposition: a geometric calculation was completed.
#[derive(Prop)]
pub struct GeometricCalculation;
impl VerifiedWorkflow for GeometricCalculation {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for polygon centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonCentroidParams {
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for polygon bounding rect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonBoundingRectParams {
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for polygon convex hull.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonConvexHullParams {
    /// The polygon.
    pub polygon: Polygon,
}

/// Parameters for linestring centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringCentroidParams {
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for linestring bounding rect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringBoundingRectParams {
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for linestring convex hull.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LinestringConvexHullParams {
    /// The linestring.
    pub linestring: LineString,
}

/// Parameters for multipoint centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipointCentroidParams {
    /// The multipoint.
    pub multipoint: MultiPoint,
}

/// Parameters for multipoint bounding rect.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipointBoundingRectParams {
    /// The multipoint.
    pub multipoint: MultiPoint,
}

/// Parameters for multipoint convex hull.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipointConvexHullParams {
    /// The multipoint.
    pub multipoint: MultiPoint,
}

/// Parameters for multipolygon centroid.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipolygonCentroidParams {
    /// The multipolygon.
    pub multipolygon: MultiPolygon,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_calculations",
    name = "polygon_centroid",
    description = "Compute the centroid of a polygon. Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn polygon_centroid(p: PolygonCentroidParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::Centroid;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let c: Option<geo_types::Point<f64>> = raw_polygon.centroid();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match c {
        Some(pt) => {
            let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "linestring_centroid",
    description = "Compute the centroid of a linestring. Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn linestring_centroid(p: LinestringCentroidParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::Centroid;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let c: Option<geo_types::Point<f64>> = raw_ls.centroid();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match c {
        Some(pt) => {
            let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "multipoint_centroid",
    description = "Compute the centroid of a multipoint. Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn multipoint_centroid(p: MultipointCentroidParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::Centroid;
    let raw_mp: geo_types::MultiPoint<f64> = geo_types::MultiPoint::from((*p.multipoint).clone());
    let c: Option<geo_types::Point<f64>> = raw_mp.centroid();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match c {
        Some(pt) => {
            let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "multipolygon_centroid",
    description = "Compute the centroid of a multipolygon. Returns a JSON Point or \"null\"."
)]
#[instrument]
async fn multipolygon_centroid(p: MultipolygonCentroidParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::Centroid;
    let raw_mp: geo_types::MultiPolygon<f64> =
        geo_types::MultiPolygon::from((*p.multipolygon).clone());
    let c: Option<geo_types::Point<f64>> = raw_mp.centroid();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match c {
        Some(pt) => {
            let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(pt));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "polygon_bounding_rect",
    description = "Compute the bounding rectangle of a polygon. Returns a JSON Rect or \"null\"."
)]
#[instrument]
async fn polygon_bounding_rect(p: PolygonBoundingRectParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoRect;
    use geo::BoundingRect;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let r: Option<geo_types::Rect<f64>> = raw_polygon.bounding_rect();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match r {
        Some(rect) => {
            let result: elicit_geo_types::Rect = elicit_geo_types::Rect::from(GeoRect::from(rect));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "linestring_bounding_rect",
    description = "Compute the bounding rectangle of a linestring. Returns a JSON Rect or \"null\"."
)]
#[instrument]
async fn linestring_bounding_rect(
    p: LinestringBoundingRectParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoRect;
    use geo::BoundingRect;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let r: Option<geo_types::Rect<f64>> = raw_ls.bounding_rect();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match r {
        Some(rect) => {
            let result: elicit_geo_types::Rect = elicit_geo_types::Rect::from(GeoRect::from(rect));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "multipoint_bounding_rect",
    description = "Compute the bounding rectangle of a multipoint. Returns a JSON Rect or \"null\"."
)]
#[instrument]
async fn multipoint_bounding_rect(
    p: MultipointBoundingRectParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoRect;
    use geo::BoundingRect;
    let raw_mp: geo_types::MultiPoint<f64> = geo_types::MultiPoint::from((*p.multipoint).clone());
    let r: Option<geo_types::Rect<f64>> = raw_mp.bounding_rect();
    let _proof = Established::<GeometricCalculation>::assert();
    let text = match r {
        Some(rect) => {
            let result: elicit_geo_types::Rect = elicit_geo_types::Rect::from(GeoRect::from(rect));
            serde_json::to_string(&result).map_err(json_err)?
        }
        None => "null".to_string(),
    };
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "polygon_convex_hull",
    description = "Compute the convex hull of a polygon. Returns a JSON Polygon."
)]
#[instrument]
async fn polygon_convex_hull(p: PolygonConvexHullParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::ConvexHull;
    let raw_polygon: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon).clone());
    let hull: geo_types::Polygon<f64> = raw_polygon.convex_hull();
    let result: elicit_geo_types::Polygon = elicit_geo_types::Polygon::from(GeoPolygon::from(hull));
    let _proof = Established::<GeometricCalculation>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "multipoint_convex_hull",
    description = "Compute the convex hull of a multipoint. Returns a JSON Polygon."
)]
#[instrument]
async fn multipoint_convex_hull(
    p: MultipointConvexHullParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::ConvexHull;
    let raw_mp: geo_types::MultiPoint<f64> = geo_types::MultiPoint::from((*p.multipoint).clone());
    let hull: geo_types::Polygon<f64> = raw_mp.convex_hull();
    let result: elicit_geo_types::Polygon = elicit_geo_types::Polygon::from(GeoPolygon::from(hull));
    let _proof = Established::<GeometricCalculation>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_calculations",
    name = "linestring_convex_hull",
    description = "Compute the convex hull of a linestring. Returns a JSON Polygon."
)]
#[instrument]
async fn linestring_convex_hull(
    p: LinestringConvexHullParams,
) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPolygon;
    use geo::ConvexHull;
    let raw_ls: geo_types::LineString<f64> = geo_types::LineString::from((*p.linestring).clone());
    let hull: geo_types::Polygon<f64> = raw_ls.convex_hull();
    let result: elicit_geo_types::Polygon = elicit_geo_types::Polygon::from(GeoPolygon::from(hull));
    let _proof = Established::<GeometricCalculation>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing centroid, bounding rect, and convex hull tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_calculations")]
pub struct GeoCalculationsPlugin;
