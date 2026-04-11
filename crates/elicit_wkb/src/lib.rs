//! `elicit_wkb` — elicitation-enabled wrappers around WKB reader/writer APIs.
//!
//! This crate is intentionally shaped around the real upstream `wkb` surface:
//! `Endianness`, `error`, `reader`, and `writer`.
//!
//! # Workflow plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`WkbReaderPlugin`] | `wkb_reader__*` | Parse and inspect WKB reader values |
//! | [`WkbWriterPlugin`] | `wkb_writer__*` | Write geo-types wrapper values to WKB bytes |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod endianness;
mod error;
mod reader;
mod workflow;
mod writer;

pub use endianness::Endianness;
pub use error::{WkbError, WkbResult};
pub use reader::{Dimension, GeometryType, Wkb, read_wkb};
pub use workflow::{
    GeometryCollectionSizeParams, GeometrySizeParams, LineSizeParams, LineStringSizeParams,
    MultiLineStringSizeParams, MultiPointSizeParams, MultiPolygonSizeParams, PointSizeParams,
    PolygonSizeParams, ReadWkbParams, RectSizeParams, TriangleSizeParams, WkbDimensionParams,
    WkbEndiannessParams, WkbGeometryTypeParams, WkbParsed, WkbReaderPlugin, WkbTryNewParams,
    WkbWriteGeometryCollectionParams, WkbWriteGeometryParams, WkbWriteLineParams,
    WkbWriteLineStringParams, WkbWriteMultiLineStringParams, WkbWriteMultiPointParams,
    WkbWriteMultiPolygonParams, WkbWritePointParams, WkbWritePolygonParams, WkbWriteRectParams,
    WkbWriteTriangleParams, WkbWriterPlugin, WkbWritten,
};
pub use writer::{
    WriteOptions, geometry_collection_wkb_size, geometry_wkb_size, line_string_wkb_size,
    line_wkb_size, multi_line_string_wkb_size, multi_point_wkb_size, multi_polygon_wkb_size,
    point_wkb_size, polygon_wkb_size, rect_wkb_size, triangle_wkb_size, write_geometry,
    write_geometry_collection, write_line, write_line_string, write_multi_line_string,
    write_multi_point, write_multi_polygon, write_point, write_polygon, write_rect, write_triangle,
};
