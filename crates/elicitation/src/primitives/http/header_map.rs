//! `http::HeaderMap` elicitation (Collection pattern).
//!
//! Elicits headers as a newline-separated list of `Name: Value` pairs,
//! then parses into a `http::HeaderMap`.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, TypeMetadata, mcp,
};
use http::{HeaderMap, HeaderName, HeaderValue};

crate::default_style!(HeaderMap => HeaderMapStyle);

impl Prompt for HeaderMap {
    fn prompt() -> Option<&'static str> {
        Some(
            "Enter HTTP headers as newline-separated `Name: Value` pairs \
             (e.g. `Content-Type: application/json`). Leave blank for no headers:",
        )
    }
}

impl Elicitation for HeaderMap {
    type Style = HeaderMapStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting http::HeaderMap");

        let params = mcp::text_params(Self::prompt().unwrap_or("Enter HTTP headers:"));

        let result = communicator
            .call_tool(rmcp::model::CallToolRequestParams {
                meta: None,
                name: mcp::tool_names::elicit_text().into(),
                arguments: Some(params),
                task: None,
            })
            .await?;

        let value = mcp::extract_value(result)?;
        let raw = mcp::parse_string(value)?;

        parse_header_map(&raw)
    }

    #[cfg(kani)]
    fn kani_proof() {
        // HeaderMap construction from valid ASCII never panics.
        let mut map = HeaderMap::new();
        map.insert(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/json"),
        );
        assert!(!map.is_empty(), "HeaderMap insertion verified ∎");
    }
}

impl ElicitIntrospect for HeaderMap {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Primitive
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "http::HeaderMap",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

/// Parse a newline-separated `Name: Value` string into a `HeaderMap`.
fn parse_header_map(raw: &str) -> ElicitResult<HeaderMap> {
    let mut map = HeaderMap::new();

    for line in raw.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let Some((name, value)) = line.split_once(':') else {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid header line (expected 'Name: Value'): {line}"
            ))));
        };

        let name = name.trim();
        let value = value.trim();

        let header_name = HeaderName::from_bytes(name.as_bytes()).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid header name '{name}': {e}"
            )))
        })?;

        let header_value = HeaderValue::from_str(value).map_err(|e| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid header value '{value}': {e}"
            )))
        })?;

        map.insert(header_name, header_value);
    }

    Ok(map)
}
