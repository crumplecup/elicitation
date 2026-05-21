//! URL type implementation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use url::{SyntaxViolation};

// Generate default-only style enum
crate::default_style!(url::Url => UrlStyle);

impl Prompt for url::Url {
    fn prompt() -> Option<&'static str> {
        Some("Please enter a URL:")
    }
}

impl Elicitation for url::Url {
    type Style = UrlStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::verification::types::UrlValid;

        tracing::debug!("Eliciting Url via UrlValid wrapper");

        // Use verification wrapper internally
        let wrapper = UrlValid::elicit(communicator).await?;

        // Unwrap to primitive
        Ok(wrapper.into_inner())
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        use crate::verification::types::UrlValid;
        <UrlValid as crate::Elicitation>::kani_proof()
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        use crate::verification::types::UrlValid;
        <UrlValid as crate::Elicitation>::verus_proof()
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        use crate::verification::types::UrlValid;
        <UrlValid as crate::Elicitation>::creusot_proof()
    }
}

impl ElicitIntrospect for url::Url {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "url::Url",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// ============================================================================
// url::SyntaxViolation
// ============================================================================

impl Prompt for SyntaxViolation {
    fn prompt() -> Option<&'static str> {
        Some("Choose a URL syntax violation kind:")
    }
}

impl Select for SyntaxViolation {
    fn options() -> Vec<Self> {
        vec![
            SyntaxViolation::Backslash,
            SyntaxViolation::C0SpaceIgnored,
            SyntaxViolation::EmbeddedCredentials,
            SyntaxViolation::ExpectedDoubleSlash,
            SyntaxViolation::ExpectedFileDoubleSlash,
            SyntaxViolation::FileWithHostAndWindowsDrive,
            SyntaxViolation::NonUrlCodePoint,
            SyntaxViolation::NullInFragment,
            SyntaxViolation::PercentDecode,
            SyntaxViolation::TabOrNewlineIgnored,
            SyntaxViolation::UnencodedAtSign,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "Backslash".to_string(),
            "C0SpaceIgnored".to_string(),
            "EmbeddedCredentials".to_string(),
            "ExpectedDoubleSlash".to_string(),
            "ExpectedFileDoubleSlash".to_string(),
            "FileWithHostAndWindowsDrive".to_string(),
            "NonUrlCodePoint".to_string(),
            "NullInFragment".to_string(),
            "PercentDecode".to_string(),
            "TabOrNewlineIgnored".to_string(),
            "UnencodedAtSign".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Backslash" => Some(SyntaxViolation::Backslash),
            "C0SpaceIgnored" => Some(SyntaxViolation::C0SpaceIgnored),
            "EmbeddedCredentials" => Some(SyntaxViolation::EmbeddedCredentials),
            "ExpectedDoubleSlash" => Some(SyntaxViolation::ExpectedDoubleSlash),
            "ExpectedFileDoubleSlash" => Some(SyntaxViolation::ExpectedFileDoubleSlash),
            "FileWithHostAndWindowsDrive" => Some(SyntaxViolation::FileWithHostAndWindowsDrive),
            "NonUrlCodePoint" => Some(SyntaxViolation::NonUrlCodePoint),
            "NullInFragment" => Some(SyntaxViolation::NullInFragment),
            "PercentDecode" => Some(SyntaxViolation::PercentDecode),
            "TabOrNewlineIgnored" => Some(SyntaxViolation::TabOrNewlineIgnored),
            "UnencodedAtSign" => Some(SyntaxViolation::UnencodedAtSign),
            _ => None,
        }
    }
}

crate::default_style!(url::SyntaxViolation => SyntaxViolationStyle);

impl Elicitation for SyntaxViolation {
    type Style = SyntaxViolationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting url::SyntaxViolation");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose SyntaxViolation:"), &Self::labels());
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid url::SyntaxViolation: {label}"
            )))
        })
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "url::SyntaxViolation",
            "Backslash",
        )
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "url::SyntaxViolation",
            "Backslash",
        )
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "url::SyntaxViolation",
            "Backslash",
        )
    }
}

impl ElicitIntrospect for SyntaxViolation {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "url::SyntaxViolation",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata {
                        label,
                        fields: vec![],
                    })
                    .collect(),
            },
        }
    }
}

crate::select_trenchcoat!(url::SyntaxViolation, as SyntaxViolationSelect);
crate::select_trenchcoat_traits!(SyntaxViolationSelect, url::SyntaxViolation, [copy, eq]);
