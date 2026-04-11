//! `WkbWriterPlugin` — write geo wrappers to WKB bytes.

use crate::{
    WriteOptions, geometry_collection_wkb_size, geometry_wkb_size, line_string_wkb_size,
    line_wkb_size, multi_line_string_wkb_size, multi_point_wkb_size, multi_polygon_wkb_size,
    point_wkb_size, polygon_wkb_size, rect_wkb_size, triangle_wkb_size, write_geometry,
    write_geometry_collection, write_line, write_line_string, write_multi_line_string,
    write_multi_point, write_multi_polygon, write_point, write_polygon, write_rect, write_triangle,
};
use elicitation::contracts::Established;
use elicitation::{
    ElicitPlugin, GeoGeometry, GeoGeometryCollection, GeoLine, GeoLineString, GeoMultiLineString,
    GeoMultiPoint, GeoMultiPolygon, GeoPoint, GeoPolygon, GeoRect, GeoTriangle, Prop,
    VerifiedWorkflow, elicit_tool,
};
use rmcp::ErrorData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use super::json_result;

/// Proposition: a geometry value was successfully written to WKB bytes.
#[derive(Prop)]
pub struct WkbWritten;

impl VerifiedWorkflow for WkbWritten {}

macro_rules! write_params {
    ($name:ident, $geom_ty:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        pub struct $name {
            /// Geometry to encode as WKB.
            pub geom: $geom_ty,
            /// Byte-order options for writing.
            pub options: WriteOptions,
        }
    };
}

macro_rules! size_params {
    ($name:ident, $geom_ty:ty, $doc:literal) => {
        #[doc = $doc]
        #[derive(Debug, Serialize, Deserialize, JsonSchema)]
        pub struct $name {
            /// Geometry to measure.
            pub geom: $geom_ty,
        }
    };
}

write_params!(
    WkbWriteGeometryParams,
    GeoGeometry,
    "Parameters for write_geometry."
);
write_params!(WkbWritePointParams, GeoPoint, "Parameters for write_point.");
write_params!(
    WkbWriteLineStringParams,
    GeoLineString,
    "Parameters for write_line_string."
);
write_params!(
    WkbWritePolygonParams,
    GeoPolygon,
    "Parameters for write_polygon."
);
write_params!(
    WkbWriteMultiPointParams,
    GeoMultiPoint,
    "Parameters for write_multi_point."
);
write_params!(
    WkbWriteMultiLineStringParams,
    GeoMultiLineString,
    "Parameters for write_multi_line_string."
);
write_params!(
    WkbWriteMultiPolygonParams,
    GeoMultiPolygon,
    "Parameters for write_multi_polygon."
);
write_params!(
    WkbWriteGeometryCollectionParams,
    GeoGeometryCollection,
    "Parameters for write_geometry_collection."
);
write_params!(WkbWriteRectParams, GeoRect, "Parameters for write_rect.");
write_params!(
    WkbWriteTriangleParams,
    GeoTriangle,
    "Parameters for write_triangle."
);
write_params!(WkbWriteLineParams, GeoLine, "Parameters for write_line.");

size_params!(
    GeometrySizeParams,
    GeoGeometry,
    "Parameters for geometry_wkb_size."
);
size_params!(PointSizeParams, GeoPoint, "Parameters for point_wkb_size.");
size_params!(
    LineStringSizeParams,
    GeoLineString,
    "Parameters for line_string_wkb_size."
);
size_params!(
    PolygonSizeParams,
    GeoPolygon,
    "Parameters for polygon_wkb_size."
);
size_params!(
    MultiPointSizeParams,
    GeoMultiPoint,
    "Parameters for multi_point_wkb_size."
);
size_params!(
    MultiLineStringSizeParams,
    GeoMultiLineString,
    "Parameters for multi_line_string_wkb_size."
);
size_params!(
    MultiPolygonSizeParams,
    GeoMultiPolygon,
    "Parameters for multi_polygon_wkb_size."
);
size_params!(
    GeometryCollectionSizeParams,
    GeoGeometryCollection,
    "Parameters for geometry_collection_wkb_size."
);
size_params!(RectSizeParams, GeoRect, "Parameters for rect_wkb_size.");
size_params!(
    TriangleSizeParams,
    GeoTriangle,
    "Parameters for triangle_wkb_size."
);
size_params!(LineSizeParams, GeoLine, "Parameters for line_wkb_size.");

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_geometry",
    description = "Write a geometry to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_geometry_tool(
    p: WkbWriteGeometryParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_geometry(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_point",
    description = "Write a point to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_point_tool(
    p: WkbWritePointParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_point(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_line_string",
    description = "Write a line string to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_line_string_tool(
    p: WkbWriteLineStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_line_string(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_polygon",
    description = "Write a polygon to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_polygon_tool(
    p: WkbWritePolygonParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_polygon(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_multi_point",
    description = "Write a multi-point to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_multi_point_tool(
    p: WkbWriteMultiPointParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_multi_point(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_multi_line_string",
    description = "Write a multi-line string to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_multi_line_string_tool(
    p: WkbWriteMultiLineStringParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_multi_line_string(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_multi_polygon",
    description = "Write a multi-polygon to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_multi_polygon_tool(
    p: WkbWriteMultiPolygonParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_multi_polygon(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_geometry_collection",
    description = "Write a geometry collection to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_geometry_collection_tool(
    p: WkbWriteGeometryCollectionParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_geometry_collection(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_rect",
    description = "Write a rect to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_rect_tool(p: WkbWriteRectParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_rect(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_triangle",
    description = "Write a triangle to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_triangle_tool(
    p: WkbWriteTriangleParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_triangle(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "write_line",
    description = "Write a line to WKB bytes. Establishes: WkbWritten."
)]
#[instrument]
async fn write_line_tool(p: WkbWriteLineParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    let bytes = write_line(&p.geom, &p.options)
        .map_err(|error| ErrorData::invalid_params(error.to_string(), None))?;
    let _proof = Established::<WkbWritten>::assert();
    json_result(&bytes)
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "geometry_wkb_size",
    description = "Return the number of bytes a geometry will occupy when encoded as WKB."
)]
#[instrument]
async fn geometry_wkb_size_tool(
    p: GeometrySizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&geometry_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "point_wkb_size",
    description = "Return the number of bytes a point will occupy when encoded as WKB."
)]
#[instrument]
async fn point_wkb_size_tool(p: PointSizeParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&point_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "line_string_wkb_size",
    description = "Return the number of bytes a line string will occupy when encoded as WKB."
)]
#[instrument]
async fn line_string_wkb_size_tool(
    p: LineStringSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&line_string_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "polygon_wkb_size",
    description = "Return the number of bytes a polygon will occupy when encoded as WKB."
)]
#[instrument]
async fn polygon_wkb_size_tool(
    p: PolygonSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&polygon_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "multi_point_wkb_size",
    description = "Return the number of bytes a multi-point will occupy when encoded as WKB."
)]
#[instrument]
async fn multi_point_wkb_size_tool(
    p: MultiPointSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&multi_point_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "multi_line_string_wkb_size",
    description = "Return the number of bytes a multi-line string will occupy when encoded as WKB."
)]
#[instrument]
async fn multi_line_string_wkb_size_tool(
    p: MultiLineStringSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&multi_line_string_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "multi_polygon_wkb_size",
    description = "Return the number of bytes a multi-polygon will occupy when encoded as WKB."
)]
#[instrument]
async fn multi_polygon_wkb_size_tool(
    p: MultiPolygonSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&multi_polygon_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "geometry_collection_wkb_size",
    description = "Return the number of bytes a geometry collection will occupy when encoded as WKB."
)]
#[instrument]
async fn geometry_collection_wkb_size_tool(
    p: GeometryCollectionSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&geometry_collection_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "rect_wkb_size",
    description = "Return the number of bytes a rect will occupy when encoded as WKB."
)]
#[instrument]
async fn rect_wkb_size_tool(p: RectSizeParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&rect_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "triangle_wkb_size",
    description = "Return the number of bytes a triangle will occupy when encoded as WKB."
)]
#[instrument]
async fn triangle_wkb_size_tool(
    p: TriangleSizeParams,
) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&triangle_wkb_size(&p.geom))
}

#[elicit_tool(
    plugin = "wkb_writer",
    name = "line_wkb_size",
    description = "Return the number of bytes a line will occupy when encoded as WKB."
)]
#[instrument]
async fn line_wkb_size_tool(p: LineSizeParams) -> Result<rmcp::model::CallToolResult, ErrorData> {
    json_result(&line_wkb_size(&p.geom))
}

/// The WKB writer MCP plugin.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "wkb_writer")]
pub struct WkbWriterPlugin;

impl WkbWriterPlugin {
    /// Creates a new WKB writer plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for WkbWriterPlugin {
    fn default() -> Self {
        Self::new()
    }
}
