//! `WktTypesPlugin` — explicit constructor tools for WKT wrapper types.

use crate::{
    Coord, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon, WktItem,
};
use elicitation::contracts::Established;
use elicitation::{ElicitPlugin, Prop, VerifiedWorkflow, elicit_tool};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a concrete WKT wrapper value was successfully created.
#[derive(Prop)]
pub struct WktTypeCreated;

impl VerifiedWorkflow for WktTypeCreated {}

/// Parameters for constructing a 2D WKT coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoordNewParams {
    /// X ordinate.
    pub x: f64,
    /// Y ordinate.
    pub y: f64,
}

/// Parameters for constructing a 3D WKT coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoordNew3dParams {
    /// X ordinate.
    pub x: f64,
    /// Y ordinate.
    pub y: f64,
    /// Z ordinate.
    pub z: f64,
}

/// Parameters for constructing a measured WKT coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CoordNewWithMParams {
    /// X ordinate.
    pub x: f64,
    /// Y ordinate.
    pub y: f64,
    /// M ordinate.
    pub m: f64,
}

/// Parameters for constructing a point from a coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointNewParams {
    /// Coordinate to wrap as a point.
    pub coord: Coord,
}

/// Parameters for constructing an empty point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmptyPointParams {}

/// Parameters for constructing a line string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LineStringNewParams {
    /// Coordinates in order.
    pub coords: Vec<Coord>,
}

/// Parameters for constructing a polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PolygonNewParams {
    /// Exterior ring.
    pub exterior: LineString,
    /// Interior rings.
    pub interiors: Vec<LineString>,
}

/// Parameters for constructing a multi-point.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiPointNewParams {
    /// Points to include.
    pub points: Vec<Point>,
}

/// Parameters for constructing a multi-line string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiLineStringNewParams {
    /// Lines to include.
    pub lines: Vec<LineString>,
}

/// Parameters for constructing a multi-polygon.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MultiPolygonNewParams {
    /// Polygons to include.
    pub polygons: Vec<Polygon>,
}

/// Parameters for constructing a geometry collection.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GeometryCollectionNewParams {
    /// Parsed geometries to include.
    pub geometries: Vec<WktItem>,
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "coord_new",
    description = "Create a 2D WKT coordinate. Establishes: WktTypeCreated."
)]
#[instrument]
async fn coord_new(p: CoordNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let coord = Coord::new(p.x, p.y);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&coord)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "coord_new_3d",
    description = "Create a 3D WKT coordinate. Establishes: WktTypeCreated."
)]
#[instrument]
async fn coord_new_3d(p: CoordNew3dParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let coord = Coord::new_3d(p.x, p.y, p.z);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&coord)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "coord_new_with_m",
    description = "Create a measured WKT coordinate. Establishes: WktTypeCreated."
)]
#[instrument]
async fn coord_new_with_m(
    p: CoordNewWithMParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let coord = Coord::new_with_m(p.x, p.y, p.m);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&coord)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "point_new",
    description = "Create a WKT point from a coordinate. Establishes: WktTypeCreated."
)]
#[instrument]
async fn point_new(p: PointNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let point = Point::new(p.coord);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&point)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "point_empty",
    description = "Create an empty WKT point. Establishes: WktTypeCreated."
)]
#[instrument]
async fn point_empty(_p: EmptyPointParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let point = Point::empty();
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&point)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "linestring_new",
    description = "Create a WKT line string from coordinates. Establishes: WktTypeCreated."
)]
#[instrument]
async fn linestring_new(p: LineStringNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let line = LineString::new(p.coords);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&line)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "polygon_new",
    description = "Create a WKT polygon from an exterior ring and interior rings. Establishes: WktTypeCreated."
)]
#[instrument]
async fn polygon_new(p: PolygonNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let polygon = Polygon::new(p.exterior, p.interiors);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&polygon)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "multipoint_new",
    description = "Create a WKT multi-point from points. Establishes: WktTypeCreated."
)]
#[instrument]
async fn multipoint_new(p: MultiPointNewParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let multipoint = MultiPoint::new(p.points);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&multipoint)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "multilinestring_new",
    description = "Create a WKT multi-line string from line strings. Establishes: WktTypeCreated."
)]
#[instrument]
async fn multilinestring_new(
    p: MultiLineStringNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let multilinestring = MultiLineString::new(p.lines);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&multilinestring)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "multipolygon_new",
    description = "Create a WKT multi-polygon from polygons. Establishes: WktTypeCreated."
)]
#[instrument]
async fn multipolygon_new(
    p: MultiPolygonNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let multipolygon = MultiPolygon::new(p.polygons);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&multipolygon)
}

#[elicit_tool(
    plugin = "wkt_types",
    name = "geometry_collection_new",
    description = "Create a WKT geometry collection from parsed geometries. Establishes: WktTypeCreated."
)]
#[instrument]
async fn geometry_collection_new(
    p: GeometryCollectionNewParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let collection = GeometryCollection::new(p.geometries);
    let _proof = Established::<WktTypeCreated>::assert();
    json_result(&collection)
}

/// The WKT constructor MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wkt_types")]
pub struct WktTypesPlugin;

impl WktTypesPlugin {
    /// Creates a new WKT constructor plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for WktTypesPlugin {
    fn default() -> Self {
        Self::new()
    }
}
