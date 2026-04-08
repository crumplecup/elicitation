//! Shape types: `Rect`, `Polygon`.

use elicitation::{GeoPolygon, GeoRect, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

use crate::{Coord, LineString};

// ── Rect ─────────────────────────────────────────────────────────────────────

elicit_newtype!(GeoRect, as Rect, serde);

impl Rect {
    /// Creates a rectangle from minimum and maximum corner coordinates.
    #[instrument]
    pub fn new(min: Coord, max: Coord) -> Self {
        GeoRect {
            min: *min,
            max: *max,
        }
        .into()
    }
}

#[reflect_methods]
impl Rect {
    /// Returns the minimum (south-west) corner.
    #[instrument(skip(self))]
    pub fn min(&self) -> Coord {
        Coord::from(self.min)
    }

    /// Returns the maximum (north-east) corner.
    #[instrument(skip(self))]
    pub fn max(&self) -> Coord {
        Coord::from(self.max)
    }

    /// Returns the width (x-extent).
    #[instrument(skip(self))]
    pub fn width(&self) -> f64 {
        self.max.x - self.min.x
    }

    /// Returns the height (y-extent).
    #[instrument(skip(self))]
    pub fn height(&self) -> f64 {
        self.max.y - self.min.y
    }

    /// Returns the center coordinate.
    #[instrument(skip(self))]
    pub fn center(&self) -> Coord {
        Coord::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }
}

impl elicitation::ElicitComplete for Rect {}

mod rect_emit {
    use super::Rect;
    impl elicitation::emit_code::ToCodeLiteral for Rect {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Rect is serializable");
            quote::quote! {
                ::elicit_geo_types::Rect::from(
                    ::serde_json::from_str::<::elicitation::GeoRect>(#json)
                        .expect("valid Rect JSON")
                )
            }
        }
    }
}

// ── Polygon ───────────────────────────────────────────────────────────────────

elicit_newtype!(GeoPolygon, as Polygon, serde);

impl Polygon {
    /// Creates a polygon from an exterior ring and optional interior rings (holes).
    #[instrument]
    pub fn new(exterior: LineString, interiors: Vec<LineString>) -> Self {
        GeoPolygon {
            exterior: (*exterior).clone(),
            interiors: interiors.into_iter().map(|ls| (*ls).clone()).collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl Polygon {
    /// Returns the exterior ring.
    #[instrument(skip(self))]
    pub fn exterior(&self) -> LineString {
        LineString::from(self.exterior.clone())
    }

    /// Returns the interior rings (holes).
    #[instrument(skip(self))]
    pub fn interiors(&self) -> Vec<LineString> {
        self.interiors
            .iter()
            .map(|ls| LineString::from(ls.clone()))
            .collect()
    }

    /// Returns the number of interior rings (holes).
    #[instrument(skip(self))]
    pub fn interiors_count(&self) -> usize {
        self.interiors.len()
    }
}

impl elicitation::ElicitComplete for Polygon {}

mod polygon_emit {
    use super::Polygon;
    impl elicitation::emit_code::ToCodeLiteral for Polygon {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Polygon is serializable");
            quote::quote! {
                ::elicit_geo_types::Polygon::from(
                    ::serde_json::from_str::<::elicitation::GeoPolygon>(#json)
                        .expect("valid Polygon JSON")
                )
            }
        }
    }
}
