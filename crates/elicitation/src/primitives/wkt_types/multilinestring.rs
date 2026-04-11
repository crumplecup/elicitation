//! Wrapper for [`wkt::types::MultiLineString<f64>`] with full elicitation support.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern, FieldInfo,
    PatternDetails, Prompt, TypeMetadata,
};

use super::linestring::WktLineString;

/// Elicitable representation of [`wkt::types::MultiLineString<f64>`].
///
/// An ordered collection of WKT line strings.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktMultiLineString {
    /// Collection of WKT line strings.
    pub lines: Vec<WktLineString>,
}

impl From<wkt::types::MultiLineString<f64>> for WktMultiLineString {
    fn from(mls: wkt::types::MultiLineString<f64>) -> Self {
        Self {
            lines: mls.0.into_iter().map(WktLineString::from).collect(),
        }
    }
}

impl From<WktMultiLineString> for wkt::types::MultiLineString<f64> {
    fn from(mls: WktMultiLineString) -> Self {
        wkt::types::MultiLineString(
            mls.lines
                .into_iter()
                .map(wkt::types::LineString::from)
                .collect(),
        )
    }
}

crate::default_style!(WktMultiLineString => WktMultiLineStringStyle);

impl Prompt for WktMultiLineString {
    fn prompt() -> Option<&'static str> {
        Some("Build a WKT multi-line-string (add line strings one by one):")
    }
}

impl Elicitation for WktMultiLineString {
    type Style = WktMultiLineStringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktMultiLineString");
        let lines = Vec::<WktLineString>::elicit(communicator).await?;
        Ok(Self { lines })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        Vec::<WktLineString>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        Vec::<WktLineString>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        Vec::<WktLineString>::creusot_proof()
    }
}

impl ElicitIntrospect for WktMultiLineString {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "wkt::types::MultiLineString<f64>",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "lines",
                    type_name: "Vec<WktLineString>",
                    prompt: Some("Line string collection (prompted iteratively):"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktMultiLineString {
    fn prompt_tree() -> crate::PromptTree {
        WktLineString::prompt_tree()
    }
}

impl crate::emit_code::ToCodeLiteral for WktMultiLineString {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let items: Vec<_> = self.lines.iter().map(|ls| ls.to_code_literal()).collect();
        quote::quote! {
            elicitation::WktMultiLineString { lines: vec![#(#items),*] }
        }
    }
}
