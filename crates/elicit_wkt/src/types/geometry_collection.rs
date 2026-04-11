//! `GeometryCollection` — wrapper around `elicitation::WktGeometryCollection`.

use crate::WktItem;
use elicitation::{WktGeom, WktGeometryCollection, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktGeometryCollection, as GeometryCollection, serde);

impl GeometryCollection {
    /// Creates a geometry collection from a list of parsed WKT items.
    #[instrument]
    pub fn new(geometries: Vec<WktItem>) -> Self {
        WktGeometryCollection {
            geometries: geometries
                .into_iter()
                .map(|geometry| {
                    let geom: &WktGeom = geometry.as_ref();
                    geom.clone()
                })
                .collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl GeometryCollection {
    /// Returns the geometries in this collection.
    #[instrument(skip(self))]
    pub fn geometries(&self) -> Vec<WktItem> {
        self.geometries.iter().cloned().map(WktItem::from).collect()
    }

    /// Returns the number of geometries.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.geometries.len()
    }

    /// Returns true if this collection is empty.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.geometries.is_empty()
    }
}

mod emit_impls {
    use super::GeometryCollection;

    impl elicitation::emit_code::ToCodeLiteral for GeometryCollection {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("GeometryCollection is serializable");
            quote::quote! {
                ::elicit_wkt::GeometryCollection::from(
                    ::serde_json::from_str::<::elicitation::WktGeometryCollection>(#json)
                        .expect("valid GeometryCollection JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for GeometryCollection {}
