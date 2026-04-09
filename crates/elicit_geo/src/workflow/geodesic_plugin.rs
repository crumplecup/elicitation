//! `GeoGeodesicPlugin` — MCP tools for geodesic/haversine distances, bearings, and destinations.

use elicit_geo_types::Point;
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

/// Proposition: a geodesic calculation was completed.
#[derive(Prop)]
pub struct GeodesicCalculated;
impl VerifiedWorkflow for GeodesicCalculated {}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for haversine distance.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HaversineDistanceParams {
    /// The origin point.
    pub from: Point,
    /// The destination point.
    pub to: Point,
}

/// Parameters for geodesic distance.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeodesicDistanceParams {
    /// The origin point.
    pub from: Point,
    /// The destination point.
    pub to: Point,
}

/// Parameters for haversine bearing.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HaversineBearingParams {
    /// The origin point.
    pub from: Point,
    /// The destination point.
    pub to: Point,
}

/// Parameters for geodesic bearing.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeodesicBearingParams {
    /// The origin point.
    pub from: Point,
    /// The destination point.
    pub to: Point,
}

/// Parameters for a haversine destination calculation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct HaversineDestinationParams {
    /// The origin point.
    pub origin: Point,
    /// Bearing in degrees.
    pub bearing_degrees: f64,
    /// Distance in meters.
    pub distance_meters: f64,
}

/// Parameters for a geodesic destination calculation.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeodesicDestinationParams {
    /// The origin point.
    pub origin: Point,
    /// Bearing in degrees.
    pub bearing_degrees: f64,
    /// Distance in meters.
    pub distance_meters: f64,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "haversine_distance_points",
    description = "Compute the haversine distance in meters between two geographic points."
)]
#[instrument]
async fn haversine_distance_points(
    p: HaversineDistanceParams,
) -> Result<CallToolResult, ErrorData> {
    use geo::{Distance, Haversine};
    let raw_from: geo_types::Point<f64> = geo_types::Point::from(*p.from);
    let raw_to: geo_types::Point<f64> = geo_types::Point::from(*p.to);
    let result = Haversine.distance(raw_from, raw_to);
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "geodesic_distance_points",
    description = "Compute the geodesic distance in meters between two geographic points."
)]
#[instrument]
async fn geodesic_distance_points(p: GeodesicDistanceParams) -> Result<CallToolResult, ErrorData> {
    use geo::{Distance, Geodesic};
    let raw_from: geo_types::Point<f64> = geo_types::Point::from(*p.from);
    let raw_to: geo_types::Point<f64> = geo_types::Point::from(*p.to);
    let result = Geodesic.distance(raw_from, raw_to);
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "haversine_bearing",
    description = "Compute the haversine bearing in degrees from one point to another."
)]
#[instrument]
async fn haversine_bearing(p: HaversineBearingParams) -> Result<CallToolResult, ErrorData> {
    use geo::{Bearing, Haversine};
    let raw_from: geo_types::Point<f64> = geo_types::Point::from(*p.from);
    let raw_to: geo_types::Point<f64> = geo_types::Point::from(*p.to);
    let result = Haversine.bearing(raw_from, raw_to);
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "geodesic_bearing",
    description = "Compute the geodesic bearing in degrees from one point to another."
)]
#[instrument]
async fn geodesic_bearing(p: GeodesicBearingParams) -> Result<CallToolResult, ErrorData> {
    use geo::{Bearing, Geodesic};
    let raw_from: geo_types::Point<f64> = geo_types::Point::from(*p.from);
    let raw_to: geo_types::Point<f64> = geo_types::Point::from(*p.to);
    let result = Geodesic.bearing(raw_from, raw_to);
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        result.to_string(),
    )]))
}

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "haversine_destination",
    description = "Compute the haversine destination point given an origin, bearing (degrees), and distance (meters). Returns the destination as a JSON Point."
)]
#[instrument]
async fn haversine_destination(p: HaversineDestinationParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::{Destination, Haversine};
    let raw_origin: geo_types::Point<f64> = geo_types::Point::from(*p.origin);
    let dest: geo_types::Point<f64> =
        Haversine.destination(raw_origin, p.bearing_degrees, p.distance_meters);
    let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(dest));
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

#[elicit_tool(
    plugin = "geo_geodesic",
    name = "geodesic_destination",
    description = "Compute the geodesic destination point given an origin, bearing (degrees), and distance (meters). Returns the destination as a JSON Point."
)]
#[instrument]
async fn geodesic_destination(p: GeodesicDestinationParams) -> Result<CallToolResult, ErrorData> {
    use elicitation::GeoPoint;
    use geo::{Destination, Geodesic};
    let raw_origin: geo_types::Point<f64> = geo_types::Point::from(*p.origin);
    let dest: geo_types::Point<f64> =
        Geodesic.destination(raw_origin, p.bearing_degrees, p.distance_meters);
    let result: elicit_geo_types::Point = elicit_geo_types::Point::from(GeoPoint::from(dest));
    let _proof = Established::<GeodesicCalculated>::assert();
    Ok(CallToolResult::success(vec![Content::text(
        serde_json::to_string(&result).map_err(json_err)?,
    )]))
}

/// Plugin exposing geodesic and haversine tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "geo_geodesic")]
pub struct GeoGeodesicPlugin;
