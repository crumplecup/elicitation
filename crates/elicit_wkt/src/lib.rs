//! `elicit_wkt` — elicitation-enabled wrappers around WKT types.
//!
//! Provides shadow-crate wrappers over the `elicitation::Wkt*` types plus a
//! parsed [`WktItem`] wrapper for structured `wkt::Wkt<f64>` values.
//!
//! # Workflow plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`WktTypesPlugin`] | `wkt_types__*` | Explicit constructor tools for WKT coordinate and geometry wrappers |
//! | [`WktParsePlugin`] | `wkt_parse__*` | Parse and inspect structured WKT values via [`WktItem`] |
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod trait_factories;
mod types;
mod wkt_item;
mod workflow;

pub use types::{
    Coord, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon, Point,
    Polygon,
};
pub use wkt_item::WktItem;
pub use workflow::{
    CoordNew3dParams, CoordNewParams, CoordNewWithMParams, EmptyPointParams,
    GeometryCollectionNewParams, LineStringNewParams, MultiLineStringNewParams,
    MultiPointNewParams, MultiPolygonNewParams, ParseWktParams, PointNewParams, PolygonNewParams,
    WktItemGeometryTypeParams, WktItemStringParams, WktParsePlugin, WktParsed, WktTypeCreated,
    WktTypesPlugin,
};
