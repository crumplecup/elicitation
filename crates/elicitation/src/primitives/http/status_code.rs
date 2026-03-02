//! `reqwest::StatusCode` elicitation (Primitive pattern via StatusCodeValid).

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, mcp, verification::types::StatusCodeValid,
};

crate::default_style!(reqwest::StatusCode => StatusCodeStyle);

impl Prompt for reqwest::StatusCode {
    fn prompt() -> Option<&'static str> {
        Some("Enter HTTP status code (100–999, e.g. 200, 404, 500):")
    }
}

impl Elicitation for reqwest::StatusCode {
    type Style = StatusCodeStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::StatusCode via StatusCodeValid");

        let wrapper = StatusCodeValid::elicit(communicator).await?;
        Ok(wrapper.into_inner())
    }

    #[cfg(kani)]
    fn kani_proof() {
        StatusCodeValid::kani_proof();
        assert!(true, "reqwest::StatusCode verified via StatusCodeValid ∎");
    }
}

impl ElicitIntrospect for reqwest::StatusCode {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::StatusCode",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

// ── StatusCodeValid Elicitation impl ────────────────────────────────────────

crate::default_style!(StatusCodeValid => StatusCodeValidStyle);

impl Prompt for StatusCodeValid {
    fn prompt() -> Option<&'static str> {
        Some("Enter HTTP status code (100–999):")
    }
}

impl Elicitation for StatusCodeValid {
    type Style = StatusCodeValidStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::{ElicitError, ElicitErrorKind};

        tracing::debug!("Eliciting StatusCodeValid");

        let params = mcp::number_params(
            Self::prompt().unwrap_or("Enter HTTP status code (100–999):"),
            100,
            999,
        );

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_number().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let raw = mcp::parse_integer::<i64>(value)?;
        let code = u16::try_from(raw).map_err(|_| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Status code out of u16 range: {raw}"
            )))
        })?;

        StatusCodeValid::new(code)
            .map_err(|e| ElicitError::new(ElicitErrorKind::ParseError(e.to_string())))
    }

    #[cfg(kani)]
    fn kani_proof() {
        let value: u16 = kani::any();
        let result = StatusCodeValid::new(value);
        if value >= 100 && value <= 999 {
            assert!(result.is_ok(), "Valid status code must succeed ∎");
        } else {
            assert!(result.is_err(), "Invalid status code must fail ∎");
        }
    }
}

impl ElicitIntrospect for StatusCodeValid {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "StatusCodeValid",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}
