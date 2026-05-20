//! Leptos mode and HTML tag enums.
//!
//! Available with the `leptos-types` feature.

use elicitation_derive::ToCodeLiteral;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Leptos rendering mode.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
    ToCodeLiteral,
)]
pub enum LeptosMode {
    /// Client-side rendering only.
    #[display("csr")]
    Csr,
    /// Server-side rendering with hydration.
    #[display("ssr")]
    Ssr,
    /// Hydration mode (SSR + client hydration).
    #[display("hydrate")]
    Hydrate,
    /// Islands architecture.
    #[display("islands")]
    Islands,
}

/// Common HTML5 element tags.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    strum::EnumIter,
    derive_more::Display,
    ToCodeLiteral,
)]
pub enum LeptosHtmlTag {
    /// Block container.
    #[display("div")]
    Div,
    /// Inline container.
    #[display("span")]
    Span,
    /// Paragraph.
    #[display("p")]
    P,
    /// Anchor/link.
    #[display("a")]
    A,
    /// Button element.
    #[display("button")]
    Button,
    /// Input element.
    #[display("input")]
    Input,
    /// Form element.
    #[display("form")]
    Form,
    /// Heading level 1.
    #[display("h1")]
    H1,
    /// Heading level 2.
    #[display("h2")]
    H2,
    /// Heading level 3.
    #[display("h3")]
    H3,
    /// Unordered list.
    #[display("ul")]
    Ul,
    /// Ordered list.
    #[display("ol")]
    Ol,
    /// List item.
    #[display("li")]
    Li,
    /// Image element.
    #[display("img")]
    Img,
    /// Navigation.
    #[display("nav")]
    Nav,
    /// Main content.
    #[display("main")]
    Main,
    /// Section.
    #[display("section")]
    Section,
    /// Article.
    #[display("article")]
    Article,
    /// Page header.
    #[display("header")]
    Header,
    /// Page footer.
    #[display("footer")]
    Footer,
    /// Aside/sidebar.
    #[display("aside")]
    Aside,
    /// Table.
    #[display("table")]
    Table,
    /// Table row.
    #[display("tr")]
    Tr,
    /// Table data cell.
    #[display("td")]
    Td,
    /// Table header cell.
    #[display("th")]
    Th,
    /// Select dropdown.
    #[display("select")]
    Select,
    /// Option element (named Option_ to avoid keyword clash).
    #[display("option")]
    Option_,
    /// Text area.
    #[display("textarea")]
    Textarea,
    /// Label.
    #[display("label")]
    Label,
    /// Bold/strong.
    #[display("strong")]
    Strong,
    /// Italic/emphasis.
    #[display("em")]
    Em,
    /// Inline code.
    #[display("code")]
    Code,
    /// Preformatted text.
    #[display("pre")]
    Pre,
}

// ============================================================================
// Elicitation traits
// ============================================================================

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use strum::IntoEnumIterator;

// --- LeptosMode --------------------------------------------------------------

impl Prompt for LeptosMode {
    fn prompt() -> Option<&'static str> { Some("Choose the Leptos rendering mode:") }
}

impl Select for LeptosMode {
    fn options() -> Vec<Self> { LeptosMode::iter().collect() }
    fn labels() -> Vec<String> { LeptosMode::iter().map(|v| v.to_string()).collect() }
    fn from_label(label: &str) -> Option<Self> {
        LeptosMode::iter().find(|v| v.to_string() == label)
    }
}

crate::default_style!(LeptosMode => LeptosModeStyle);

impl Elicitation for LeptosMode {
    type Style = LeptosModeStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosMode");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose Leptos mode:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid LeptosMode: {label}")))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("LeptosMode", "Csr")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("LeptosMode", "Csr")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("LeptosMode", "Csr")
    }
}

impl ElicitIntrospect for LeptosMode {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosMode",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for LeptosMode {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose Leptos mode:").to_string(),
            type_name: "LeptosMode".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}

// --- LeptosHtmlTag -----------------------------------------------------------

impl Prompt for LeptosHtmlTag {
    fn prompt() -> Option<&'static str> { Some("Choose an HTML element tag:") }
}

impl Select for LeptosHtmlTag {
    fn options() -> Vec<Self> { LeptosHtmlTag::iter().collect() }
    fn labels() -> Vec<String> { LeptosHtmlTag::iter().map(|v| v.to_string()).collect() }
    fn from_label(label: &str) -> Option<Self> {
        LeptosHtmlTag::iter().find(|v| v.to_string() == label)
    }
}

crate::default_style!(LeptosHtmlTag => LeptosHtmlTagStyle);

impl Elicitation for LeptosHtmlTag {
    type Style = LeptosHtmlTagStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting LeptosHtmlTag");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose HTML tag:"), &Self::labels(),
        );
        let result = communicator.call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        ).await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!("Invalid LeptosHtmlTag: {label}")))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("LeptosHtmlTag", "Div")
    }
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("LeptosHtmlTag", "Div")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("LeptosHtmlTag", "Div")
    }
}

impl ElicitIntrospect for LeptosHtmlTag {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "elicitation::LeptosHtmlTag",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl crate::ElicitPromptTree for LeptosHtmlTag {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Select {
            prompt: Self::prompt().unwrap_or("Choose HTML tag:").to_string(),
            type_name: "LeptosHtmlTag".to_string(),
            options: Self::labels(),
            branches: Self::labels().iter().map(|_| None).collect(),
        }
    }
}
