//! `reqwest::Response` elicitation (Survey pattern).
//!
//! Elicits a status code, body, and URL, then constructs a mock `reqwest::Response`
//! via `http::Response::builder().into()`. Invaluable for agents constructing
//! mock responses in tests without a live HTTP server.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult,
    Elicitation, ElicitationPattern, PatternDetails, Prompt, TypeMetadata,
    emit_code::ToCodeLiteral,
};
use bytes::Bytes;
use reqwest::Response;
use reqwest::ResponseBuilderExt;
use url::Url;

crate::default_style!(Response => ResponseStyle);

#[derive(Clone, Debug)]
struct CapturedResponseSnapshot {
    status: reqwest::StatusCode,
    version: reqwest::Version,
    headers: reqwest::header::HeaderMap,
    url: Url,
    body: Bytes,
}

/// Capture a live `reqwest::Response` into a rebuildable form for code recovery.
///
/// This drains the one-shot body into owned bytes, rebuilds an equivalent
/// `reqwest::Response`, and stores the captured snapshot in the response's
/// extensions so `ToCodeLiteral(&self)` can later recover it from an immutable
/// borrow.
pub async fn capture_reqwest_response(response: Response) -> ElicitResult<Response> {
    let snapshot = CapturedResponseSnapshot {
        status: response.status(),
        version: response.version(),
        headers: response.headers().clone(),
        url: response.url().clone(),
        body: response.bytes().await?,
    };

    let mut builder = http::Response::builder()
        .status(snapshot.status)
        .version(snapshot.version)
        .url(snapshot.url.clone());
    let headers = builder.headers_mut().ok_or_else(|| {
        ElicitError::new(ElicitErrorKind::ParseError(
            "http::Response::builder() did not expose headers for captured reqwest::Response"
                .to_string(),
        ))
    })?;
    *headers = snapshot.headers.clone();

    let rebuilt = builder.body(snapshot.body.clone())?;

    let mut response = Response::from(rebuilt);
    response.extensions_mut().insert(snapshot);
    Ok(response)
}

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
            ?;

        Ok(Response::from(response))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("response")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("response")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("response")
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

impl crate::ElicitPromptTree for Response {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "reqwest::Response".to_string(),
            fields: vec![
                (
                    "status".to_string(),
                    Box::new(
                        <reqwest::StatusCode as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some(
                                "HTTP status code (e.g. 200, 404, 500)".to_string(),
                            )),
                    ),
                ),
                (
                    "body".to_string(),
                    Box::new(
                        <String as crate::ElicitPromptTree>::prompt_tree().with_prompt(Some(
                            "Response body text (empty for no body)".to_string(),
                        )),
                    ),
                ),
                (
                    "url".to_string(),
                    Box::new(
                        <url::Url as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("Response URL".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for Response {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let Some(snapshot) = self.extensions().get::<CapturedResponseSnapshot>() else {
            let message =
                "reqwest::Response code recovery requires capture_reqwest_response(response).await before storing or emitting the value";
            return quote::quote! {{
                compile_error!(#message);
            }};
        };
        let status = snapshot.status.to_code_literal();
        let version = snapshot.version.to_code_literal();
        let headers = snapshot.headers.to_code_literal();
        let url = snapshot.url.to_code_literal();
        let body = snapshot.body.iter();

        quote::quote! {{
            use reqwest::ResponseBuilderExt as _;

            let mut builder = http::Response::builder()
                .status(#status)
                .version(#version)
                .url(#url);
            let headers = builder.headers_mut().ok_or_else(|| {
                elicitation::ElicitError::new(elicitation::ElicitErrorKind::ParseError(
                    "http::Response::builder() did not expose headers during reqwest::Response code recovery"
                        .to_string(),
                ))
            })?;
            *headers = #headers;

            reqwest::Response::from(
                builder
                    .body(::bytes::Bytes::from(::std::vec![#(#body),*]))
                    ?,
            )
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::capture_reqwest_response;
    use crate::emit_code::ToCodeLiteral;
    use bytes::Bytes;
    use reqwest::ResponseBuilderExt;
    use url::Url;

    #[tokio::test]
    async fn captured_response_preserves_visible_state_and_body() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .status(reqwest::StatusCode::CREATED)
                .version(reqwest::Version::HTTP_2)
                .url(Url::parse("https://example.com/data").expect("valid URL"))
                .header("content-type", "application/json")
                .body(Bytes::from_static(br#"{"ok":true}"#))
                .expect("valid response"),
        );

        let response = capture_reqwest_response(response)
            .await
            .expect("capture should succeed");

        assert_eq!(response.status(), reqwest::StatusCode::CREATED);
        assert_eq!(response.version(), reqwest::Version::HTTP_2);
        assert_eq!(response.url().as_str(), "https://example.com/data");
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("header should be present"),
            "application/json"
        );
        assert_eq!(
            response.bytes().await.expect("body should still be readable"),
            Bytes::from_static(br#"{"ok":true}"#),
        );
    }

    #[tokio::test]
    async fn captured_response_to_code_literal_includes_body_bytes() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .status(reqwest::StatusCode::OK)
                .version(reqwest::Version::HTTP_11)
                .url(Url::parse("https://example.com/body").expect("valid URL"))
                .body(Bytes::from_static(b"abc"))
                .expect("valid response"),
        );

        let response = capture_reqwest_response(response)
            .await
            .expect("capture should succeed");
        let tokens = response.to_code_literal().to_string();

        assert!(tokens.contains("97"));
        assert!(tokens.contains("98"));
        assert!(tokens.contains("99"));
        assert!(tokens.contains("https://example.com/body"));
    }
}
