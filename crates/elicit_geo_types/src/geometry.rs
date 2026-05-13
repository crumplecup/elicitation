//! `Geometry` — the top-level geo-types enum wrapper.

use elicitation::{GeoGeometry, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(GeoGeometry, as Geometry, serde);

#[reflect_methods]
impl Geometry {
    /// Returns the variant name (e.g. `"Point"`, `"Polygon"`, `"GeometryCollection"`).
    #[instrument(skip(self))]
    pub fn geometry_type(&self) -> String {
        match self.as_ref() {
            GeoGeometry::Point(_) => "Point",
            GeoGeometry::Line(_) => "Line",
            GeoGeometry::LineString(_) => "LineString",
            GeoGeometry::Polygon(_) => "Polygon",
            GeoGeometry::MultiPoint(_) => "MultiPoint",
            GeoGeometry::MultiLineString(_) => "MultiLineString",
            GeoGeometry::MultiPolygon(_) => "MultiPolygon",
            GeoGeometry::Rect(_) => "Rect",
            GeoGeometry::Triangle(_) => "Triangle",
            GeoGeometry::GeometryCollection(_) => "GeometryCollection",
        }
        .to_string()
    }

    /// Returns `true` if this geometry is a `Point` variant.
    #[instrument(skip(self))]
    pub fn is_point(&self) -> bool {
        matches!(self.as_ref(), GeoGeometry::Point(_))
    }

    /// Returns `true` if this geometry is a multi- or collection variant.
    #[instrument(skip(self))]
    pub fn is_collection(&self) -> bool {
        matches!(
            self.as_ref(),
            GeoGeometry::GeometryCollection(_)
                | GeoGeometry::MultiPoint(_)
                | GeoGeometry::MultiLineString(_)
                | GeoGeometry::MultiPolygon(_)
        )
    }
}

impl elicitation::ElicitComplete for Geometry {}

mod geometry_emit {
    use super::Geometry;
    impl elicitation::emit_code::ToCodeLiteral for Geometry {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Geometry is serializable");
            quote::quote! {
                ::elicit_geo_types::Geometry::from(
                    ::serde_json::from_str::<::elicitation::GeoGeometry>(#json)
                        .expect("valid Geometry JSON")
                )
            }
        }
    }
}
