//! `Polygon` — elicitation-enabled wrapper around `elicitation::WktPolygon`.

use crate::LineString;
use elicitation::{WktPolygon, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktPolygon, as Polygon, serde);

impl Polygon {
    /// Creates a polygon from an exterior ring and interior rings.
    #[instrument]
    pub fn new(exterior: LineString, interiors: Vec<LineString>) -> Self {
        WktPolygon {
            exterior: (*exterior).clone(),
            interiors: interiors.into_iter().map(|ring| (*ring).clone()).collect(),
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

    /// Returns the interior rings.
    #[instrument(skip(self))]
    pub fn interiors(&self) -> Vec<LineString> {
        self.interiors
            .iter()
            .cloned()
            .map(LineString::from)
            .collect()
    }

    /// Returns the number of interior rings.
    #[instrument(skip(self))]
    pub fn interiors_len(&self) -> usize {
        self.interiors.len()
    }
}

mod emit_impls {
    use super::Polygon;

    impl elicitation::emit_code::ToCodeLiteral for Polygon {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Polygon is serializable");
            quote::quote! {
                ::elicit_wkt::Polygon::from(
                    ::serde_json::from_str::<::elicitation::WktPolygon>(#json)
                        .expect("valid Polygon JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Polygon {}
