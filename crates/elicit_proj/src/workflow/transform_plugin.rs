//! `ProjTransformPlugin` — create transforms and convert coordinates.

use crate::transform::ProjTransform;
use elicitation::contracts::Established;
use elicitation::{
    ElicitPlugin, GeoCoord, GeoGeometry, ProjArea, Prop, VerifiedWorkflow, elicit_tool,
};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a PROJ transform was successfully created.
#[derive(Prop)]
pub struct ProjCreated;

impl VerifiedWorkflow for ProjCreated {}

/// Parameters for creating a transform from a PROJ string.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateFromProjStringParams {
    /// A PROJ string definition (e.g. `"+proj=utm +zone=32 +datum=WGS84"`).
    pub definition: String,
}

/// Parameters for creating a transform from two known CRS identifiers.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateFromKnownCrsParams {
    /// Source CRS identifier (e.g. `"EPSG:4326"`).
    pub from: String,
    /// Target CRS identifier (e.g. `"EPSG:3857"`).
    pub to: String,
    /// Optional area of use to guide CRS selection.
    pub area: Option<ProjArea>,
}

/// Parameters for converting a single coordinate.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertCoordParams {
    /// The transform to apply.
    pub transform: ProjTransform,
    /// The coordinate to convert.
    pub coord: GeoCoord,
}

/// Parameters for projecting a single coordinate to/from a projection plane.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectCoordParams {
    /// The transform to apply.
    pub transform: ProjTransform,
    /// The coordinate to project (radians for forward, projected units for inverse).
    pub coord: GeoCoord,
    /// When `true`, perform the inverse projection (projected → geodetic in radians).
    pub inverse: bool,
}

/// Parameters for converting all coordinates in a geometry.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ConvertGeometryParams {
    /// The transform to apply.
    pub transform: ProjTransform,
    /// The geometry whose coordinates will be converted.
    pub geometry: GeoGeometry,
}

/// Parameters for transforming a bounding box.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TransformBoundsParams {
    /// The transform to apply.
    pub transform: ProjTransform,
    /// Western boundary in the source CRS.
    pub west: f64,
    /// Southern boundary in the source CRS.
    pub south: f64,
    /// Eastern boundary in the source CRS.
    pub east: f64,
    /// Northern boundary in the source CRS.
    pub north: f64,
    /// Number of densification points along each edge (21 is a reasonable default).
    pub densify_pts: i32,
}

/// Parameters for retrieving the PROJ string definition of a transform.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DefinitionParams {
    /// The transform to inspect.
    pub transform: ProjTransform,
}

/// The transformed bounding box returned by `proj__transform_bounds`.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TransformBoundsResult {
    /// Western boundary in the target CRS.
    pub west: f64,
    /// Southern boundary in the target CRS.
    pub south: f64,
    /// Eastern boundary in the target CRS.
    pub east: f64,
    /// Northern boundary in the target CRS.
    pub north: f64,
}

impl From<[f64; 4]> for TransformBoundsResult {
    fn from([west, south, east, north]: [f64; 4]) -> Self {
        Self {
            west,
            south,
            east,
            north,
        }
    }
}

#[elicit_tool(
    plugin = "proj",
    name = "create_from_proj_string",
    description = "Create a PROJ coordinate transform from a PROJ string definition. \
                   Establishes: ProjCreated."
)]
#[instrument]
async fn create_from_proj_string(
    p: CreateFromProjStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let transform = ProjTransform::from_proj_string(p.definition);
    transform
        .build()
        .map_err(|e: crate::ProjTransformError| ErrorData::invalid_params(e.to_string(), None))?;
    let _proof = Established::<ProjCreated>::assert();
    json_result(&transform)
}

#[elicit_tool(
    plugin = "proj",
    name = "create_from_known_crs",
    description = "Create a PROJ coordinate transform between two known CRS identifiers \
                   (e.g. EPSG:4326 → EPSG:3857).  Pass an optional area of use to narrow \
                   the best transform selection.  Establishes: ProjCreated."
)]
#[instrument]
async fn create_from_known_crs(
    p: CreateFromKnownCrsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let transform = ProjTransform::from_known_crs(p.from, p.to, p.area);
    transform
        .build()
        .map_err(|e: crate::ProjTransformError| ErrorData::invalid_params(e.to_string(), None))?;
    let _proof = Established::<ProjCreated>::assert();
    json_result(&transform)
}

#[elicit_tool(
    plugin = "proj",
    name = "convert_coord",
    description = "Convert a single coordinate from the source CRS to the target CRS \
                   using the given transform."
)]
#[instrument]
async fn convert_coord(p: ConvertCoordParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let result = p
        .transform
        .convert_coord(p.coord)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&result)
}

#[elicit_tool(
    plugin = "proj",
    name = "project_coord",
    description = "Project a coordinate to/from the projection plane.  Forward direction \
                   expects radians; inverse direction expects projected units.  Set \
                   `inverse = true` for the reverse operation."
)]
#[instrument]
async fn project_coord(p: ProjectCoordParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let result = p
        .transform
        .project_coord(p.coord, p.inverse)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&result)
}

#[elicit_tool(
    plugin = "proj",
    name = "convert_geometry",
    description = "Convert all coordinates in a geometry from the source CRS to the target \
                   CRS using the given transform."
)]
#[instrument(skip(p))]
async fn convert_geometry(
    p: ConvertGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let result = p
        .transform
        .convert_geometry(p.geometry)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&result)
}

#[elicit_tool(
    plugin = "proj",
    name = "transform_bounds",
    description = "Transform a bounding box from the source CRS to the target CRS, \
                   densifying edges to account for non-linear curvature.  A `densify_pts` \
                   of 21 is a reasonable default."
)]
#[instrument]
async fn transform_bounds(
    p: TransformBoundsParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bounds = p
        .transform
        .transform_bounds(p.west, p.south, p.east, p.north, p.densify_pts)
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&TransformBoundsResult::from(bounds))
}

#[elicit_tool(
    plugin = "proj",
    name = "definition",
    description = "Return the PROJ string definition of the given transform."
)]
#[instrument]
async fn definition(p: DefinitionParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let def = p
        .transform
        .definition()
        .map_err(|e| ErrorData::internal_error(e.to_string(), None))?;
    json_result(&def)
}

/// MCP plugin for PROJ coordinate transformation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "proj")]
pub struct ProjTransformPlugin;

impl ProjTransformPlugin {
    /// Create a new PROJ transform plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for ProjTransformPlugin {
    fn default() -> Self {
        Self::new()
    }
}
