//! `MultiPolygon` — elicitation-enabled wrapper around `elicitation::WktMultiPolygon`.

use crate::Polygon;
use elicitation::{WktMultiPolygon, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktMultiPolygon, as MultiPolygon, serde);

impl MultiPolygon {
    /// Creates a multi-polygon from a list of polygons.
    #[instrument]
    pub fn new(polygons: Vec<Polygon>) -> Self {
        WktMultiPolygon {
            polygons: polygons
                .into_iter()
                .map(|polygon| (*polygon).clone())
                .collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl MultiPolygon {
    /// Returns the polygons in this multi-polygon.
    #[instrument(skip(self))]
    pub fn polygons(&self) -> Vec<Polygon> {
        self.polygons.iter().cloned().map(Polygon::from).collect()
    }

    /// Returns the number of polygons.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.polygons.len()
    }

    /// Returns true if this multi-polygon is empty.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.polygons.is_empty()
    }
}

mod emit_impls {
    use super::MultiPolygon;

    impl elicitation::emit_code::ToCodeLiteral for MultiPolygon {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiPolygon is serializable");
            quote::quote! {
                ::elicit_wkt::MultiPolygon::from(
                    ::serde_json::from_str::<::elicitation::WktMultiPolygon>(#json)
                        .expect("valid MultiPolygon JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiPolygon {}
