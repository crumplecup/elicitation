//! Writer-facing wrappers for the upstream `wkb::writer` module.

use crate::{Endianness, WkbError, WkbResult};
use elicitation::{
    GeoGeometry, GeoGeometryCollection, GeoLine, GeoLineString, GeoMultiLineString, GeoMultiPoint,
    GeoMultiPolygon, GeoPoint, GeoPolygon, GeoRect, GeoTriangle,
};

/// Serializable mirror of [`wkb::writer::WriteOptions`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
pub struct WriteOptions {
    /// The byte order to use when writing WKB bytes.
    pub endianness: Endianness,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            endianness: Endianness::LittleEndian,
        }
    }
}

impl From<WriteOptions> for wkb::writer::WriteOptions {
    fn from(value: WriteOptions) -> Self {
        Self {
            endianness: value.endianness.into(),
        }
    }
}

fn into_wkb_bytes(bytes: Vec<u8>) -> WkbResult<elicitation::WkbBytes> {
    elicitation::WkbBytes::new(bytes).map_err(WkbError::General)
}

/// The number of bytes this geometry will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn geometry_wkb_size(geom: &GeoGeometry) -> usize {
    let geom: geo_types::Geometry<f64> = geom.clone().into();
    wkb::writer::geometry_wkb_size(&geom)
}

/// The number of bytes this point will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn point_wkb_size(geom: &GeoPoint) -> usize {
    let geom: geo_types::Point<f64> = (*geom).into();
    wkb::writer::geometry_wkb_size(&geo_types::Geometry::Point(geom))
}

/// The number of bytes this line string will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn line_string_wkb_size(geom: &GeoLineString) -> usize {
    let geom: geo_types::LineString<f64> = geom.clone().into();
    wkb::writer::line_string_wkb_size(&geom)
}

/// The number of bytes this polygon will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn polygon_wkb_size(geom: &GeoPolygon) -> usize {
    let geom: geo_types::Polygon<f64> = geom.clone().into();
    wkb::writer::polygon_wkb_size(&geom)
}

/// The number of bytes this multi-point will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn multi_point_wkb_size(geom: &GeoMultiPoint) -> usize {
    let geom: geo_types::MultiPoint<f64> = geom.clone().into();
    wkb::writer::multi_point_wkb_size(&geom)
}

/// The number of bytes this multi-line string will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn multi_line_string_wkb_size(geom: &GeoMultiLineString) -> usize {
    let geom: geo_types::MultiLineString<f64> = geom.clone().into();
    wkb::writer::multi_line_string_wkb_size(&geom)
}

/// The number of bytes this multi-polygon will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn multi_polygon_wkb_size(geom: &GeoMultiPolygon) -> usize {
    let geom: geo_types::MultiPolygon<f64> = geom.clone().into();
    wkb::writer::multi_polygon_wkb_size(&geom)
}

/// The number of bytes this geometry collection will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn geometry_collection_wkb_size(geom: &GeoGeometryCollection) -> usize {
    let geom: geo_types::GeometryCollection<f64> = geom.clone().into();
    wkb::writer::geometry_collection_wkb_size(&geom)
}

/// The number of bytes this rect will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn rect_wkb_size(geom: &GeoRect) -> usize {
    let geom: geo_types::Rect<f64> = (*geom).into();
    wkb::writer::rect_wkb_size(&geom)
}

/// The number of bytes this triangle will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn triangle_wkb_size(geom: &GeoTriangle) -> usize {
    let geom: geo_types::Triangle<f64> = (*geom).into();
    wkb::writer::triangle_wkb_size(&geom)
}

/// The number of bytes this line will take up when encoded as WKB.
#[tracing::instrument(skip(geom))]
pub fn line_wkb_size(geom: &GeoLine) -> usize {
    let geom: geo_types::Line<f64> = (*geom).into();
    wkb::writer::line_wkb_size(&geom)
}

macro_rules! write_fn {
    ($name:ident, $upstream:path, $arg:ty, $geo_ty:ty) => {
        #[doc = "Write the geometry encoded as WKB bytes using the provided write options."]
        #[tracing::instrument(skip(geom, options))]
        pub fn $name(geom: &$arg, options: &WriteOptions) -> WkbResult<elicitation::WkbBytes> {
            let geom: $geo_ty = geom.clone().into();
            let options: wkb::writer::WriteOptions = options.clone().into();
            let mut out = Vec::new();
            $upstream(&mut out, &geom, &options).map_err(WkbError::from)?;
            into_wkb_bytes(out)
        }
    };
}

write_fn!(
    write_geometry,
    wkb::writer::write_geometry,
    GeoGeometry,
    geo_types::Geometry<f64>
);
write_fn!(
    write_point,
    wkb::writer::write_point,
    GeoPoint,
    geo_types::Point<f64>
);
write_fn!(
    write_line_string,
    wkb::writer::write_line_string,
    GeoLineString,
    geo_types::LineString<f64>
);
write_fn!(
    write_polygon,
    wkb::writer::write_polygon,
    GeoPolygon,
    geo_types::Polygon<f64>
);
write_fn!(
    write_multi_point,
    wkb::writer::write_multi_point,
    GeoMultiPoint,
    geo_types::MultiPoint<f64>
);
write_fn!(
    write_multi_line_string,
    wkb::writer::write_multi_line_string,
    GeoMultiLineString,
    geo_types::MultiLineString<f64>
);
write_fn!(
    write_multi_polygon,
    wkb::writer::write_multi_polygon,
    GeoMultiPolygon,
    geo_types::MultiPolygon<f64>
);
write_fn!(
    write_geometry_collection,
    wkb::writer::write_geometry_collection,
    GeoGeometryCollection,
    geo_types::GeometryCollection<f64>
);
write_fn!(
    write_rect,
    wkb::writer::write_rect,
    GeoRect,
    geo_types::Rect<f64>
);
write_fn!(
    write_triangle,
    wkb::writer::write_triangle,
    GeoTriangle,
    geo_types::Triangle<f64>
);
write_fn!(
    write_line,
    wkb::writer::write_line,
    GeoLine,
    geo_types::Line<f64>
);
