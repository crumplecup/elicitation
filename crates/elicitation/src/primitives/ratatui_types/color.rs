//! [`ratatui::style::Color`] elicitation.
//!
//! Color uses a two-step elicitation: first the user selects a color category
//! (named, RGB, or indexed), then provides the specific value.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use ratatui::style::Color;

/// Named colors available for direct selection.
const NAMED_COLORS: &[(&str, Color)] = &[
    ("Reset", Color::Reset),
    ("Black", Color::Black),
    ("Red", Color::Red),
    ("Green", Color::Green),
    ("Yellow", Color::Yellow),
    ("Blue", Color::Blue),
    ("Magenta", Color::Magenta),
    ("Cyan", Color::Cyan),
    ("Gray", Color::Gray),
    ("DarkGray", Color::DarkGray),
    ("LightRed", Color::LightRed),
    ("LightGreen", Color::LightGreen),
    ("LightYellow", Color::LightYellow),
    ("LightBlue", Color::LightBlue),
    ("LightMagenta", Color::LightMagenta),
    ("LightCyan", Color::LightCyan),
    ("White", Color::White),
    ("RGB (custom)…", Color::Reset),   // sentinel
    ("Indexed (0–255)…", Color::Reset), // sentinel
];

impl Prompt for Color {
    fn prompt() -> Option<&'static str> {
        Some("Choose a terminal colour:")
    }
}

impl Select for Color {
    fn options() -> Vec<Self> {
        NAMED_COLORS.iter().map(|(_, c)| *c).collect()
    }

    fn labels() -> Vec<String> {
        NAMED_COLORS.iter().map(|(l, _)| (*l).to_string()).collect()
    }

    fn from_label(label: &str) -> Option<Self> {
        NAMED_COLORS
            .iter()
            .find(|(l, _)| *l == label)
            .map(|(_, c)| *c)
    }
}

crate::default_style!(ratatui::style::Color => ColorStyle);

impl Elicitation for Color {
    type Style = ColorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::style::Color");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose colour:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        match label.as_str() {
            "RGB (custom)…" => {
                let rgb_params =
                    mcp::text_params("Enter RGB as #RRGGBB (e.g. #FF00AA):");
                let rgb_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(rgb_params),
                    )
                    .await?;
                let hex = mcp::parse_string(mcp::extract_value(rgb_result)?)?;
                let trimmed = hex.trim().trim_start_matches('#');
                if trimmed.len() != 6 {
                    return Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "Expected 6 hex digits, got: {trimmed}"
                    ))));
                }
                let r =
                    u8::from_str_radix(&trimmed[0..2], 16).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(e.to_string()))
                    })?;
                let g =
                    u8::from_str_radix(&trimmed[2..4], 16).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(e.to_string()))
                    })?;
                let b =
                    u8::from_str_radix(&trimmed[4..6], 16).map_err(|e| {
                        ElicitError::new(ElicitErrorKind::ParseError(e.to_string()))
                    })?;
                Ok(Color::Rgb(r, g, b))
            }
            "Indexed (0–255)…" => {
                let idx_params = mcp::text_params("Enter palette index (0–255):");
                let idx_result = communicator
                    .call_tool(
                        rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                            .with_arguments(idx_params),
                    )
                    .await?;
                let idx_str = mcp::parse_string(mcp::extract_value(idx_result)?)?;
                let index: u8 = idx_str.trim().parse().map_err(|e: std::num::ParseIntError| {
                    ElicitError::new(ElicitErrorKind::ParseError(e.to_string()))
                })?;
                Ok(Color::Indexed(index))
            }
            other => Self::from_label(other).ok_or_else(|| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Invalid ratatui::style::Color: {other}"
                )))
            }),
        }
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("ratatui::style::Color", "Red")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("ratatui::style::Color", "Red")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper("ratatui::style::Color", "Red")
    }
}

impl ElicitIntrospect for Color {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::style::Color",
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
