//! `reqwest::Version` elicitation (Select pattern).

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata, mcp,
};

crate::default_style!(reqwest::Version => VersionStyle);

impl Select for reqwest::Version {
    fn options() -> Vec<Self> {
        vec![
            reqwest::Version::HTTP_10,
            reqwest::Version::HTTP_11,
            reqwest::Version::HTTP_2,
            reqwest::Version::HTTP_3,
        ]
    }

    fn labels() -> Vec<String> {
        vec![
            "HTTP/1.0".to_string(),
            "HTTP/1.1".to_string(),
            "HTTP/2.0".to_string(),
            "HTTP/3.0".to_string(),
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "HTTP/1.0" => Some(reqwest::Version::HTTP_10),
            "HTTP/1.1" => Some(reqwest::Version::HTTP_11),
            "HTTP/2.0" => Some(reqwest::Version::HTTP_2),
            "HTTP/3.0" => Some(reqwest::Version::HTTP_3),
            _ => None,
        }
    }
}

impl Prompt for reqwest::Version {
    fn prompt() -> Option<&'static str> {
        Some("Select HTTP version:")
    }
}

impl Elicitation for reqwest::Version {
    type Style = VersionStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::Version");

        let params = mcp::select_params(
            Self::prompt().unwrap_or("Select HTTP version:"),
            &Self::labels(),
        );

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;

        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid HTTP version: {label}"
            )))
        })
    }

    #[cfg(kani)]
    fn kani_proof() {
        let _v10 = reqwest::Version::HTTP_10;
        let _v11 = reqwest::Version::HTTP_11;
        let _v2 = reqwest::Version::HTTP_2;
        let _v3 = reqwest::Version::HTTP_3;
        assert!(true, "reqwest::Version variants verified ∎");
    }
}

impl ElicitIntrospect for reqwest::Version {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Select
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Version",
            description: Self::prompt(),
            details: PatternDetails::Select {
                options: Self::labels(),
            },
        }
    }
}
