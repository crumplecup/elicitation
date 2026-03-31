//! Select-trenchcoat wrappers for ratatui types.
//!
//! Each wrapper adds `JsonSchema` to the corresponding ratatui type,
//! enabling [`ElicitComplete`](crate::ElicitComplete).
//!
//! [`BordersSelect`] wraps the bitflags [`ratatui::widgets::Borders`]
//! with common preset combinations. [`ScrollbarOrientationSelect`]
//! wraps [`ratatui::widgets::ScrollbarOrientation`] with serde delegation.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, VariantMetadata, mcp,
};
use ratatui::widgets::{Borders, ScrollbarOrientation};

// ---------------------------------------------------------------------------
// Borders (bitflags — manual wrapper with common presets)
// ---------------------------------------------------------------------------

/// Select wrapper for [`ratatui::widgets::Borders`].
///
/// Provides common border combinations as selectable presets.
/// Use `into_inner()` to unwrap back to `Borders`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct BordersSelect(Borders);

impl BordersSelect {
    /// Unwrap to the inner `Borders` value.
    pub fn into_inner(self) -> Borders {
        self.0
    }
}

impl From<Borders> for BordersSelect {
    fn from(b: Borders) -> Self {
        Self(b)
    }
}

impl std::ops::Deref for BordersSelect {
    type Target = Borders;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl schemars::JsonSchema for BordersSelect {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "BordersSelect".into()
    }

    fn json_schema(_gen: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let labels = <Borders as Select>::labels();
        let enum_values: Vec<serde_json::Value> =
            labels.into_iter().map(serde_json::Value::String).collect();
        serde_json::from_value(serde_json::json!({
            "type": "string",
            "enum": enum_values
        }))
        .expect("valid schema")
    }
}

impl Prompt for Borders {
    fn prompt() -> Option<&'static str> {
        Some("Choose which borders to show:")
    }
}

impl Select for Borders {
    fn options() -> Vec<Self> {
        vec![
            Borders::NONE,
            Borders::ALL,
            Borders::TOP,
            Borders::BOTTOM,
            Borders::LEFT,
            Borders::RIGHT,
            Borders::TOP | Borders::BOTTOM,
            Borders::LEFT | Borders::RIGHT,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "None".to_string(),
            "All".to_string(),
            "Top".to_string(),
            "Bottom".to_string(),
            "Left".to_string(),
            "Right".to_string(),
            "Top + Bottom".to_string(),
            "Left + Right".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "None" => Some(Borders::NONE),
            "All" => Some(Borders::ALL),
            "Top" => Some(Borders::TOP),
            "Bottom" => Some(Borders::BOTTOM),
            "Left" => Some(Borders::LEFT),
            "Right" => Some(Borders::RIGHT),
            "Top + Bottom" => Some(Borders::TOP | Borders::BOTTOM),
            "Left + Right" => Some(Borders::LEFT | Borders::RIGHT),
            _ => None,
        }
    }
}

crate::default_style!(ratatui::widgets::Borders => BordersStyle);

impl Elicitation for Borders {
    type Style = BordersStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::widgets::Borders");
        let params =
            mcp::select_params(Self::prompt().unwrap_or("Choose borders:"), &Self::labels());
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
                "Invalid ratatui::widgets::Borders: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("ratatui::widgets::Borders", "All")
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper("ratatui::widgets::Borders", "All")
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "ratatui::widgets::Borders",
            "All",
        )
    }
}

impl ElicitIntrospect for Borders {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::widgets::Borders",
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

impl Prompt for BordersSelect {
    fn prompt() -> Option<&'static str> {
        <Borders as Prompt>::prompt()
    }
}

impl ElicitIntrospect for BordersSelect {
    fn pattern() -> ElicitationPattern {
        <Borders as ElicitIntrospect>::pattern()
    }

    fn metadata() -> TypeMetadata {
        <Borders as ElicitIntrospect>::metadata()
    }
}

impl Elicitation for BordersSelect {
    type Style = <Borders as Elicitation>::Style;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        <Borders as Elicitation>::elicit(communicator)
            .await
            .map(Self)
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        <Borders as Elicitation>::kani_proof()
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        <Borders as Elicitation>::verus_proof()
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        <Borders as Elicitation>::creusot_proof()
    }
}

// ---------------------------------------------------------------------------
// ScrollbarOrientation (regular enum with serde)
// ---------------------------------------------------------------------------

crate::select_trenchcoat!(ratatui::widgets::ScrollbarOrientation, as ScrollbarOrientationSelect, serde);
crate::select_trenchcoat_traits!(
    ScrollbarOrientationSelect,
    ratatui::widgets::ScrollbarOrientation,
    [eq, hash]
);

impl Prompt for ScrollbarOrientation {
    fn prompt() -> Option<&'static str> {
        Some("Choose scrollbar orientation:")
    }
}

impl Select for ScrollbarOrientation {
    fn options() -> Vec<Self> {
        vec![
            ScrollbarOrientation::VerticalRight,
            ScrollbarOrientation::VerticalLeft,
            ScrollbarOrientation::HorizontalBottom,
            ScrollbarOrientation::HorizontalTop,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "VerticalRight".to_string(),
            "VerticalLeft".to_string(),
            "HorizontalBottom".to_string(),
            "HorizontalTop".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "VerticalRight" => Some(ScrollbarOrientation::VerticalRight),
            "VerticalLeft" => Some(ScrollbarOrientation::VerticalLeft),
            "HorizontalBottom" => Some(ScrollbarOrientation::HorizontalBottom),
            "HorizontalTop" => Some(ScrollbarOrientation::HorizontalTop),
            _ => None,
        }
    }
}

crate::default_style!(ratatui::widgets::ScrollbarOrientation => ScrollbarOrientationStyle);

impl Elicitation for ScrollbarOrientation {
    type Style = ScrollbarOrientationStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ratatui::widgets::ScrollbarOrientation");
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose scrollbar orientation:"),
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
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid ratatui::widgets::ScrollbarOrientation: {label}"
            )))
        })
    }

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper(
            "ratatui::widgets::ScrollbarOrientation",
            "VerticalRight",
        )
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_select_wrapper(
            "ratatui::widgets::ScrollbarOrientation",
            "VerticalRight",
        )
    }

    #[cfg(feature = "proofs")]
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_select_wrapper(
            "ratatui::widgets::ScrollbarOrientation",
            "VerticalRight",
        )
    }
}

impl ElicitIntrospect for ScrollbarOrientation {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "ratatui::widgets::ScrollbarOrientation",
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

// ---------------------------------------------------------------------------
// Alignment (regular enum with serde — trenchcoat adds JsonSchema)
// ---------------------------------------------------------------------------

crate::select_trenchcoat!(ratatui::layout::Alignment, as AlignmentSelect, serde);
crate::select_trenchcoat_traits!(
    AlignmentSelect,
    ratatui::layout::Alignment,
    [copy, eq, hash]
);

// ---------------------------------------------------------------------------
// Direction (regular enum with serde — trenchcoat adds JsonSchema)
// ---------------------------------------------------------------------------

crate::select_trenchcoat!(ratatui::layout::Direction, as RatatuiDirectionSelect, serde);
crate::select_trenchcoat_traits!(
    RatatuiDirectionSelect,
    ratatui::layout::Direction,
    [copy, eq, hash]
);

// ---------------------------------------------------------------------------
// BorderType (regular enum with serde — trenchcoat adds JsonSchema)
// ---------------------------------------------------------------------------

crate::select_trenchcoat!(ratatui::widgets::BorderType, as BorderTypeSelect, serde);
crate::select_trenchcoat_traits!(
    BorderTypeSelect,
    ratatui::widgets::BorderType,
    [copy, eq, hash]
);

// ---------------------------------------------------------------------------
// Color (custom serde via Display/FromStr — trenchcoat adds JsonSchema)
// ---------------------------------------------------------------------------

crate::select_trenchcoat!(ratatui::style::Color, as ColorSelect, serde);
crate::select_trenchcoat_traits!(ColorSelect, ratatui::style::Color, [copy, eq, hash]);
