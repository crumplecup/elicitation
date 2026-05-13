//! `MultiLineString` — elicitation-enabled wrapper around `elicitation::WktMultiLineString`.

use crate::LineString;
use elicitation::{WktMultiLineString, elicit_newtype};
use elicitation_derive::reflect_methods;
use tracing::instrument;

elicit_newtype!(WktMultiLineString, as MultiLineString, serde);

impl MultiLineString {
    /// Creates a multi-line string from a list of line strings.
    #[instrument]
    pub fn new(lines: Vec<LineString>) -> Self {
        WktMultiLineString {
            lines: lines.into_iter().map(|line| (*line).clone()).collect(),
        }
        .into()
    }
}

#[reflect_methods]
impl MultiLineString {
    /// Returns the line strings in this multi-line string.
    #[instrument(skip(self))]
    pub fn lines(&self) -> Vec<LineString> {
        self.lines.iter().cloned().map(LineString::from).collect()
    }

    /// Returns the number of line strings.
    #[instrument(skip(self))]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// Returns true if this multi-line string is empty.
    #[instrument(skip(self))]
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}

mod emit_impls {
    use super::MultiLineString;

    impl elicitation::emit_code::ToCodeLiteral for MultiLineString {
        fn to_code_literal(&self) -> proc_macro2::TokenStream {
            let json = serde_json::to_string(self).expect("MultiLineString is serializable");
            quote::quote! {
                ::elicit_wkt::MultiLineString::from(
                    ::serde_json::from_str::<::elicitation::WktMultiLineString>(#json)
                        .expect("valid MultiLineString JSON")
                )
            }
        }
    }
}

impl elicitation::ElicitComplete for MultiLineString {}
