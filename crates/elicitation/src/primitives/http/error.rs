//! `reqwest::Error` elicitation support.
//!
//! Direct elicitation models the public status-error subclass because reqwest
//! does not expose constructors for arbitrary builder/request/decode errors.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult, Elicitation,
    ElicitationPattern, PatternDetails, Prompt, TypeMetadata,
};
use bytes::Bytes;
use reqwest::ResponseBuilderExt;
use url::Url;

crate::default_style!(reqwest::Error => ErrorStyle);

fn build_status_error(status: reqwest::StatusCode, url: Url) -> ElicitResult<reqwest::Error> {
    let response = http::Response::builder()
        .status(status)
        .url(url)
        .body(Bytes::new())?;

    match reqwest::Response::from(response).error_for_status() {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
            "synthetic status response {status} did not yield reqwest::Error",
        )))),
        Err(error) => Ok(error),
    }
}

impl Prompt for reqwest::Error {
    fn prompt() -> Option<&'static str> {
        Some("Construct a reqwest status error — provide an HTTP error status (4xx/5xx) and URL.")
    }
}

impl Elicitation for reqwest::Error {
    type Style = ErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::Error (status subclass)");

        let status = reqwest::StatusCode::elicit(communicator).await?;
        if !(status.is_client_error() || status.is_server_error()) {
            return Err(ElicitError::new(ElicitErrorKind::ParseError(format!(
                "reqwest::Error direct elicitation only models HTTP status errors; got {status}",
            ))));
        }

        let url = Url::elicit(communicator).await?;
        build_status_error(status, url)
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("error")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("error")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("error")
    }
}

impl ElicitIntrospect for reqwest::Error {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Error",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "status",
                        type_name: "reqwest::StatusCode",
                        prompt: Some("HTTP error status code (4xx or 5xx)"),
                    },
                    crate::FieldInfo {
                        name: "url",
                        type_name: "url::Url",
                        prompt: Some("URL associated with the error"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for reqwest::Error {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "reqwest::Error".to_string(),
            fields: vec![
                (
                    "status".to_string(),
                    Box::new(
                        <reqwest::StatusCode as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("HTTP error status code (4xx or 5xx)".to_string())),
                    ),
                ),
                (
                    "url".to_string(),
                    Box::new(
                        <url::Url as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("URL associated with the error".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for reqwest::Error {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let Some(status) = self.status() else {
            let message = "reqwest::Error code recovery currently supports the public status-error subset only";
            return quote::quote! {{
                compile_error!(#message);
            }};
        };
        let Some(url) = self.url() else {
            let message = "reqwest::Error status-error code recovery requires an associated URL";
            return quote::quote! {{
                compile_error!(#message);
            }};
        };
        let status = status.to_code_literal();
        let url = url.to_code_literal();

        quote::quote! {{
            use reqwest::ResponseBuilderExt as _;

            let response = http::Response::builder()
                .status(#status)
                .url(#url)
                .body(::bytes::Bytes::new())
                ?;

            match reqwest::Response::from(response).error_for_status() {
                Ok(_) => {
                    return Err(elicitation::ElicitError::new(
                        elicitation::ElicitErrorKind::ParseError(
                            "synthetic error status unexpectedly produced a successful reqwest::Response"
                                .to_string(),
                        ),
                    )
                    .into());
                }
                Err(error) => error,
            }
        }}
    }
}

#[cfg(test)]
mod tests {
    use super::build_status_error;
    use crate::emit_code::ToCodeLiteral;
    use url::Url;

    #[test]
    fn status_error_to_code_literal_mentions_status_and_url() {
        let error = build_status_error(
            reqwest::StatusCode::BAD_REQUEST,
            Url::parse("https://example.com/fail").expect("valid URL"),
        )
        .expect("status error should build");

        let tokens = error.to_code_literal().to_string();

        assert!(tokens.contains("BAD_REQUEST") || tokens.contains("400"));
        assert!(tokens.contains("https://example.com/fail"));
    }

    #[test]
    fn non_status_error_code_literal_emits_explicit_compile_failure() {
        let error = reqwest::Client::new()
            .get("blob:https://example.com/not-http")
            .build()
            .expect_err("non-http schemes should fail during request building");

        let rendered = error.to_code_literal().to_string();

        assert!(rendered.contains("status errors only"));
        assert!(rendered.contains("compile_error"));
    }
}
