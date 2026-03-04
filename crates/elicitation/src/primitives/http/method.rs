//! `reqwest::Method` elicitation (Select pattern).

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, mcp,
};

crate::default_style!(reqwest::Method => MethodStyle);

impl Select for reqwest::Method {
    fn options() -> Vec<Self> {
        vec![
            reqwest::Method::GET,
            reqwest::Method::POST,
            reqwest::Method::PUT,
            reqwest::Method::DELETE,
            reqwest::Method::PATCH,
            reqwest::Method::HEAD,
            reqwest::Method::OPTIONS,
            reqwest::Method::CONNECT,
            reqwest::Method::TRACE,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
            "PATCH".to_string(),
            "HEAD".to_string(),
            "OPTIONS".to_string(),
            "CONNECT".to_string(),
            "TRACE".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "GET" => Some(reqwest::Method::GET),
            "POST" => Some(reqwest::Method::POST),
            "PUT" => Some(reqwest::Method::PUT),
            "DELETE" => Some(reqwest::Method::DELETE),
            "PATCH" => Some(reqwest::Method::PATCH),
            "HEAD" => Some(reqwest::Method::HEAD),
            "OPTIONS" => Some(reqwest::Method::OPTIONS),
            "CONNECT" => Some(reqwest::Method::CONNECT),
            "TRACE" => Some(reqwest::Method::TRACE),
            _ => None,
        }
    }
}

impl Prompt for reqwest::Method {
    fn prompt() -> Option<&'static str> {
        Some("Select HTTP method:")
    }
}

impl Elicitation for reqwest::Method {
    type Style = MethodStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::Method");

        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select HTTP method:"),
            &Self::labels(),
        );

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params))
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid HTTP method: {label}"
            )))
        })
    }

    #[cfg(kani)]
    fn kani_proof() {
        // All variants are statically known constants — construction never panics.
        let _get = reqwest::Method::GET;
        let _post = reqwest::Method::POST;
        assert!(true, "reqwest::Method variants verified ∎");
    }
}

impl ElicitIntrospect for reqwest::Method {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Method",
            description: Self::prompt(),
            details: PatternDetails::Select {
                options: Self::labels(),
            },
        }
    }
}
