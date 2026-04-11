//! `Point` — elicitation-enabled wrapper around `elicitation::WktPoint`.

use crate::Coord;
use elicitation::{WktPoint, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktPoint, as Point, serde);

impl Point {
    /// Creates a point from a coordinate.
    #[instrument]
    pub fn new(coord: Coord) -> Self {
        WktPoint {
            coord: Some((*coord).clone()),
        }
        .into()
    }

    /// Creates an empty point.
    #[instrument]
    pub fn empty() -> Self {
        WktPoint { coord: None }.into()
    }
}

#[reflect_methods]
impl Point {
    /// Returns the point coordinate, if present.
    #[instrument(skip(self))]
    pub fn coord(&self) -> Option<Coord> {
        self.coord.clone().map(Coord::from)
    }

    /// Returns true if this is an empty point.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.coord.is_none()
    }
}

mod emit_impls {
    use super::Point;

    impl elicitation::emit_code::ToCodeLiteral for Point {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("Point is serializable");
            quote::quote! {
                ::elicit_wkt::Point::from(
                    ::serde_json::from_str::<::elicitation::WktPoint>(#json)
                        .expect("valid Point JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for Point {}
