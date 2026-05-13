//! Wrapper for raw WKT strings with validation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, FieldInfo, PatternDetails, Prompt, TypeMetadata,
};
use std::str::FromStr;

/// Builder type for validated WKT strings.
///
/// Elicitation prompts for a raw WKT string and validates it by parsing
/// with `wkt::Wkt::<f64>::from_str`. Invalid strings are rejected.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct WktString {
    /// The raw WKT string.
    pub wkt: String,
}

crate::default_style!(WktString => WktStringStyle);

impl Prompt for WktString {
    fn prompt() -> Option<&'static str> {
        Some("Enter a WKT geometry string (e.g. POINT(1 2)):")
    }
}

impl Elicitation for WktString {
    type Style = WktStringStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting WktString");
        let raw = String::elicit(communicator).await?;
        wkt::Wkt::<f64>::from_str(&raw).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid WKT string: {e}"
            )))
        })?;
        Ok(Self { wkt: raw })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        // WktString delegates to String — parsing is third-party logic (trusted).
        String::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        String::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        String::creusot_proof()
    }
}

impl ElicitIntrospect for WktString {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "WktString",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![FieldInfo {
                    name: "wkt",
                    type_name: "String",
                    prompt: Some("WKT geometry string:"),
                }],
            },
        }
    }
}

impl crate::ElicitPromptTree for WktString {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(|s| s.to_string()),
            type_name: "WktString".to_string(),
            fields: vec![("wkt".to_string(), Box::new(String::prompt_tree()))],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for WktString {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let wkt = &self.wkt;
        quote::quote! {
            elicitation::WktString { wkt: #wkt.to_string() }
        }
    }
}
