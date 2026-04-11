//! `MultiPoint` — elicitation-enabled wrapper around `elicitation::WktMultiPoint`.

use crate::Point;
use elicitation::{WktMultiPoint, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktMultiPoint, as MultiPoint, serde);

impl MultiPoint {
    /// Creates a multi-point from a list of points.
    #[instrument]
    pub fn new(points: Vec<Point>) -> Self {
        WktMultiPoint {
            points: points.into_iter().map(|point| (*point).clone()).collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl MultiPoint {
    /// Returns the points in this multi-point.
    #[instrument(skip(self))]
    pub fn points(&self) -> Vec<Point> {
        self.points.iter().cloned().map(Point::from).collect()
    }

    /// Returns the number of points.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.points.len()
    }

    /// Returns true if this multi-point is empty.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

mod emit_impls {
    use super::MultiPoint;

    impl elicitation::emit_code::ToCodeLiteral for MultiPoint {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiPoint is serializable");
            quote::quote! {
                ::elicit_wkt::MultiPoint::from(
                    ::serde_json::from_str::<::elicitation::WktMultiPoint>(#json)
                        .expect("valid MultiPoint JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiPoint {}
