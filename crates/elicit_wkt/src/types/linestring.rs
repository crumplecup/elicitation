//! `LineString` — elicitation-enabled wrapper around `elicitation::WktLineString`.

use crate::Coord;
use elicitation::{WktLineString, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktLineString, as LineString, serde);

impl LineString {
    /// Creates a line string from a list of coordinates.
    #[instrument]
    pub fn new(coords: Vec<Coord>) -> Self {
        WktLineString {
            coords: coords.into_iter().map(|coord| (*coord).clone()).collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl LineString {
    /// Returns the coordinates in this line string.
    #[instrument(skip(self))]
    pub fn coords(&self) -> Vec<Coord> {
        self.coords.iter().cloned().map(Coord::from).collect()
    }

    /// Returns the number of coordinates.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.coords.len()
    }

    /// Returns true if this line string has no coordinates.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }
}

mod emit_impls {
    use super::LineString;

    impl elicitation::emit_code::ToCodeLiteral for LineString {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("LineString is serializable");
            quote::quote! {
                ::elicit_wkt::LineString::from(
                    ::serde_json::from_str::<::elicitation::WktLineString>(#json)
                        .expect("valid LineString JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for LineString {}
