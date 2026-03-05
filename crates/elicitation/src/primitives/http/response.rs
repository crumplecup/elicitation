//! `reqwest::Response` elicitation (Survey pattern).
//!
//! Elicits a status code, body, and URL, then constructs a mock `reqwest::Response`
//! via `http::Response::builder().into()`. Invaluable for agents constructing
//! mock responses in tests without a live HTTP server.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
};
use bytes::Bytes;
use reqwest::Response;
use reqwest::ResponseBuilderExt;
use url::Url;

crate::default_style!(Response => ResponseStyle);

impl Prompt for Response {
    fn prompt() -> Option<&'static str> {
        Some(
            "Construct a mock HTTP response — provide a status code, optional body, \
             and optional URL. Useful for testing and agent simulations.",
        )
    }
}

impl Elicitation for Response {
    type Style = ResponseStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        use crate::{ElicitError, ElicitErrorKind};

        tracing::debug!("Eliciting reqwest::Response (mock)");

        let status = reqwest::StatusCode::elicit(communicator).await?;
        tracing::debug!(%status, "Status code selected");

        let body_text = String::elicit(communicator).await?;
        tracing::debug!(body_len = body_text.len(), "Body elicited");

        let url = Url::elicit(communicator).await?;
        tracing::debug!(%url, "URL selected");

        let response = http::Response::builder()
            .status(status)
            .url(url)
            .body(Bytes::from(body_text))
            .map_err(|e| {
                ElicitError::new(ElicitErrorKind::ParseError(format!(
                    "Failed to build mock response: {e}"
                )))
            })?;

        Ok(Response::from(response))
    }
}

impl ElicitIntrospect for Response {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Response",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "status",
                        type_name: "reqwest::StatusCode",
                        prompt: Some("HTTP status code (e.g. 200, 404, 500)"),
                    },
                    crate::FieldInfo {
                        name: "body",
                        type_name: "String",
                        prompt: Some("Response body text (empty for no body)"),
                    },
                    crate::FieldInfo {
                        name: "url",
                        type_name: "url::Url",
                        prompt: Some("Response URL"),
                    },
                ],
            },
        }
    }
}
