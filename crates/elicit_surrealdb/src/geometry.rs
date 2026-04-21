//! [`surrealdb_types::Geometry`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

use crate::Value;

elicit_newtype!(surrealdb_types::Geometry, as Geometry, forward_serde);
elicit_newtype_traits!(Geometry, surrealdb_types::Geometry, [eq, display]);

#[reflect_methods]
impl Geometry {
    /// Returns `true` if all coordinates satisfy valid lat/lng bounds.
    #[tracing::instrument(skip(self))]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Returns the GeoJSON geometry type name (e.g. `"Point"`, `"Polygon"`).
    #[tracing::instrument(skip(self))]
    pub fn as_type(&self) -> String {
        self.0.as_type().to_string()
    }

    /// Returns the coordinate data as a nested array Value.
    #[tracing::instrument(skip(self))]
    pub fn as_coordinates(&self) -> Value {
        self.0.as_coordinates().into()
    }
}

impl elicitation::ElicitComplete for Geometry {}

mod emit_impls {
    use super::Geometry;
    use elicitation::emit_code::ToCodeLiteral;
    use proc_macro2::TokenStream;

    impl ToCodeLiteral for Geometry {
        fn to_code_literal(&self) -> TokenStream {
            let json = serde_json::to_string(self.0.as_ref()).expect("Geometry is Serialize");
            quote::quote! {
                ::serde_json::from_str::<::surrealdb_types::Geometry>(#json)
                    .expect("valid Geometry JSON")
                    .into()
            }
        }
    }
}
