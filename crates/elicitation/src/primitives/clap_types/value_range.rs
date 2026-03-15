//! [`clap::builder::ValueRange`] elicitation.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp,
};
use clap::builder::ValueRange;

crate::default_style!(ValueRange => ValueRangeStyle);

impl Prompt for ValueRange {
    fn prompt() -> Option<&'static str> {
        Some("Enter the number of values this argument accepts:")
    }
}

impl Elicitation for ValueRange {
    type Style = ValueRangeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting ValueRange");

        let params = mcp::text_params(
            "Enter value count as a single number (e.g. '1'), \
             a range 'min..max' (e.g. '1..3'), or 'min..' for unbounded (e.g. '1..'):",
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?;
        let s = s.trim();

        tracing::debug!(input = %s, "Parsing ValueRange");
        let range = if let Some(pos) = s.find("..") {
            let min_str = &s[..pos];
            let max_str = &s[pos + 2..];
            let min: usize = min_str.trim().parse().map_err(|_| {
                crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                    "Invalid range minimum: {}",
                    min_str
                )))
            })?;
            if max_str.trim().is_empty() {
                ValueRange::from(min..)
            } else {
                let max: usize = max_str.trim().parse().map_err(|_| {
                    crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                        "Invalid range maximum: {}",
                        max_str
                    )))
                })?;
                ValueRange::from(min..=max)
            }
        } else {
            let n: usize = s.parse().map_err(|_| {
                crate::ElicitError::new(crate::ElicitErrorKind::ParseError(format!(
                    "Invalid value count: {}",
                    s
                )))
            })?;
            ValueRange::from(n)
        };

        tracing::debug!("Elicited ValueRange");
        Ok(range)
    }
}

impl ElicitIntrospect for ValueRange {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "clap::builder::ValueRange",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
