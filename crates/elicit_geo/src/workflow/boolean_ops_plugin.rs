//! `GeoBooleanOpsPlugin` — MCP tools for boolean polygon operations (union, intersection, difference, xor).

use elicit_geo_types::{MultiPolygon, Polygon};
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

/// Proposition: a boolean operation was applied.
#[derive(Prop)]
pub struct BooleanOpApplied;
impl VerifiedWorkflow for BooleanOpApplied {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for polygon union.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonUnionParams {
    /// The first polygon.
    pub polygon1: Polygon,
    /// The second polygon.
    pub polygon2: Polygon,
}

/// Parameters for polygon intersection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonIntersectionParams {
    /// The first polygon.
    pub polygon1: Polygon,
    /// The second polygon.
    pub polygon2: Polygon,
}

/// Parameters for polygon difference.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonDifferenceParams {
    /// The first polygon.
    pub polygon1: Polygon,
    /// The second polygon.
    pub polygon2: Polygon,
}

/// Parameters for polygon xor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonXorParams {
    /// The first polygon.
    pub polygon1: Polygon,
    /// The second polygon.
    pub polygon2: Polygon,
}

/// Parameters for multipolygon union.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipolygonUnionParams {
    /// The first multipolygon.
    pub mp1: MultiPolygon,
    /// The second multipolygon.
    pub mp2: MultiPolygon,
}

/// Parameters for multipolygon intersection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipolygonIntersectionParams {
    /// The first multipolygon.
    pub mp1: MultiPolygon,
    /// The second multipolygon.
    pub mp2: MultiPolygon,
}

/// Parameters for multipolygon difference.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipolygonDifferenceParams {
    /// The first multipolygon.
    pub mp1: MultiPolygon,
    /// The second multipolygon.
    pub mp2: MultiPolygon,
}

/// Parameters for multipolygon xor.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultipolygonXorParams {
    /// The first multipolygon.
    pub mp1: MultiPolygon,
    /// The second multipolygon.
    pub mp2: MultiPolygon,
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn wrap_multipolygon(
    raw: geo_types::MultiPolygon<f64>,
) -> Result<elicit_geo_types::MultiPolygon, ErrorData> {
    use elicitation::GeoMultiPolygon;
    let wrapped: elicit_geo_types::MultiPolygon =
        elicit_geo_types::MultiPolygon::from(GeoMultiPolygon::from(raw));
    serde_json::to_string(&wrapped)
        .map(|_| wrapped)
        .map_err(json_err)
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "polygon_union",
    description = "Compute the union of two polygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn polygon_union(p: PolygonUnionParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_p1: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon1).clone());
    let raw_p2: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_p1.union(&raw_p2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "polygon_intersection",
    description = "Compute the intersection of two polygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn polygon_intersection(p: PolygonIntersectionParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_p1: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon1).clone());
    let raw_p2: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_p1.intersection(&raw_p2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "polygon_difference",
    description = "Compute the set difference of two polygons (polygon1 minus polygon2). Returns a JSON MultiPolygon."
)]
#[instrument]
async fn polygon_difference(p: PolygonDifferenceParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_p1: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon1).clone());
    let raw_p2: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_p1.difference(&raw_p2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "polygon_xor",
    description = "Compute the symmetric difference (XOR) of two polygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn polygon_xor(p: PolygonXorParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_p1: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon1).clone());
    let raw_p2: geo_types::Polygon<f64> = geo_types::Polygon::from((*p.polygon2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_p1.xor(&raw_p2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "multipolygon_union",
    description = "Compute the union of two multipolygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn multipolygon_union(p: MultipolygonUnionParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_mp1: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp1).clone());
    let raw_mp2: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_mp1.union(&raw_mp2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "multipolygon_intersection",
    description = "Compute the intersection of two multipolygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn multipolygon_intersection(
    p: MultipolygonIntersectionParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_mp1: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp1).clone());
    let raw_mp2: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_mp1.intersection(&raw_mp2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "multipolygon_difference",
    description = "Compute the set difference of two multipolygons (mp1 minus mp2). Returns a JSON MultiPolygon."
)]
#[instrument]
async fn multipolygon_difference(
    p: MultipolygonDifferenceParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_mp1: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp1).clone());
    let raw_mp2: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_mp1.difference(&raw_mp2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_boolean_ops",
    name = "multipolygon_xor",
    description = "Compute the symmetric difference (XOR) of two multipolygons. Returns a JSON MultiPolygon."
)]
#[instrument]
async fn multipolygon_xor(p: MultipolygonXorParams) -> Result<CallToolResult, ErrorData> {
    use geo::BooleanOps;
    let raw_mp1: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp1).clone());
    let raw_mp2: geo_types::MultiPolygon<f64> = geo_types::MultiPolygon::from((*p.mp2).clone());
    let raw_result: geo_types::MultiPolygon<f64> = raw_mp1.xor(&raw_mp2);
    let wrapped = wrap_multipolygon(raw_result)?;
    let _proof = Established::<BooleanOpApplied>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&wrapped).map_err(json_err)?,
    )]))
}

/// Plugin exposing boolean polygon operation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_boolean_ops")]
pub struct GeoBooleanOpsPlugin;
