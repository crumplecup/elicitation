//! Collection types: `LineString`, `MultiPoint`, `MultiLineString`, `MultiPolygon`,
//! `GeometryCollection`.

use elicitation::{
    GeoGeometryCollection, GeoLineString, GeoMultiLineString, GeoMultiPoint, GeoMultiPolygon,
    elicit_newtype,
};
use elicitation_derive::reflect_methods;
use tracing::instrument;

use crate::primitives::{Coord, Point};

// ── LineString ────────────────────────────────────────────────────────────────

elicit_newtype!(GeoLineString, as LineString, serde);

impl LineString {
    /// Creates a `LineString` from a list of coordinates.
    #[instrument]
    pub fn new(coords: Vec<Coord>) -> Self {
        GeoLineString(coords.into_iter().map(|c| *c).collect()).into()
    }
}

#[reflect_methods]
impl LineString {
    /// Returns the number of coordinates.
    #[instrument(skip(self))]
    pub fn coords_count(&self) -> usize {
        self.as_ref().0.len()
    }

    /// Returns `true` if the first and last coordinates are equal (closed ring).
    #[instrument(skip(self))]
    pub fn is_closed(&self) -> bool {
        let v = &self.as_ref().0;
        !v.is_empty() && v.first() == v.last()
    }

    /// Returns all coordinates as a list.
    #[instrument(skip(self))]
    pub fn coords(&self) -> Vec<Coord> {
        self.as_ref().0.iter().map(|c| Coord::from(*c)).collect()
    }
}

mod line_string_emit {
    use super::LineString;
    impl elicitation::emit_code::ToCodeLiteral for LineString {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("LineString is serializable");
            quote::quote! {
                ::elicit_geo_types::LineString::from(
                    ::serde_json::from_str::<::elicitation::GeoLineString>(#json)
                        .expect("valid LineString JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for LineString {}

// ── MultiPoint ────────────────────────────────────────────────────────────────

elicit_newtype!(GeoMultiPoint, as MultiPoint, serde);

impl MultiPoint {
    /// Creates a `MultiPoint` from a list of points.
    #[instrument]
    pub fn new(points: Vec<Point>) -> Self {
        GeoMultiPoint(points.into_iter().map(|p| *p).collect()).into()
    }
}

#[reflect_methods]
impl MultiPoint {
    /// Returns the number of points.
    #[instrument(skip(self))]
    pub fn count(&self) -> usize {
        self.as_ref().0.len()
    }

    /// Returns all points as a list.
    #[instrument(skip(self))]
    pub fn points(&self) -> Vec<Point> {
        self.as_ref().0.iter().map(|p| Point::from(*p)).collect()
    }
}

mod multi_point_emit {
    use super::MultiPoint;
    impl elicitation::emit_code::ToCodeLiteral for MultiPoint {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiPoint is serializable");
            quote::quote! {
                ::elicit_geo_types::MultiPoint::from(
                    ::serde_json::from_str::<::elicitation::GeoMultiPoint>(#json)
                        .expect("valid MultiPoint JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiPoint {}

// ── MultiLineString ───────────────────────────────────────────────────────────

elicit_newtype!(GeoMultiLineString, as MultiLineString, serde);

impl MultiLineString {
    /// Creates a `MultiLineString` from a list of line strings.
    #[instrument]
    pub fn new(lines: Vec<LineString>) -> Self {
        GeoMultiLineString(lines.into_iter().map(|ls| (*ls).clone()).collect()).into()
    }
}

#[reflect_methods]
impl MultiLineString {
    /// Returns the number of line strings.
    #[instrument(skip(self))]
    pub fn count(&self) -> usize {
        self.as_ref().0.len()
    }
}

mod multi_line_string_emit {
    use super::MultiLineString;
    impl elicitation::emit_code::ToCodeLiteral for MultiLineString {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiLineString is serializable");
            quote::quote! {
                ::elicit_geo_types::MultiLineString::from(
                    ::serde_json::from_str::<::elicitation::GeoMultiLineString>(#json)
                        .expect("valid MultiLineString JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiLineString {}

// ── MultiPolygon ──────────────────────────────────────────────────────────────

elicit_newtype!(GeoMultiPolygon, as MultiPolygon, serde);

#[reflect_methods]
impl MultiPolygon {
    /// Returns the number of polygons.
    #[instrument(skip(self))]
    pub fn count(&self) -> usize {
        self.as_ref().0.len()
    }
}

mod multi_polygon_emit {
    use super::MultiPolygon;
    impl elicitation::emit_code::ToCodeLiteral for MultiPolygon {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiPolygon is serializable");
            quote::quote! {
                ::elicit_geo_types::MultiPolygon::from(
                    ::serde_json::from_str::<::elicitation::GeoMultiPolygon>(#json)
                        .expect("valid MultiPolygon JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiPolygon {}

// ── GeometryCollection ────────────────────────────────────────────────────────

elicit_newtype!(GeoGeometryCollection, as GeometryCollection, serde);

#[reflect_methods]
impl GeometryCollection {
    /// Returns the number of geometries in the collection.
    #[instrument(skip(self))]
    pub fn count(&self) -> usize {
        self.as_ref().0.len()
    }

    /// Returns `true` if the collection contains no geometries.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.as_ref().0.is_empty()
    }
}

mod geometry_collection_emit {
    use super::GeometryCollection;
    impl elicitation::emit_code::ToCodeLiteral for GeometryCollection {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("GeometryCollection is serializable");
            quote::quote! {
                ::elicit_geo_types::GeometryCollection::from(
                    ::serde_json::from_str::<::elicitation::GeoGeometryCollection>(#json)
                        .expect("valid GeometryCollection JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for GeometryCollection {}
