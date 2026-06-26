//! `reqwest::Request` elicitation support.
//!
//! Direct elicitation models the visible request parts exposed by reqwest's
//! public API: method, URL, headers, optional in-memory body, version, and
//! optional timeout.

use std::time::Duration;

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};
use url::Url;

crate::default_style!(Request => RequestStyle);

#[tracing::instrument(skip(headers, body))]
fn build_request(
    method: reqwest::Method,
    url: Url,
    headers: http::HeaderMap,
    body: Option<Vec<u8>>,
    version: reqwest::Version,
    timeout: Option<Duration>,
) -> reqwest::Request {
    let mut request = reqwest::Request::new(method, url);
    *request.headers_mut() = headers;
    *request.version_mut() = version;
    *request.timeout_mut() = timeout;
    *request.body_mut() = body.map(reqwest::Body::from);
    request
}

#[tracing::instrument(skip(request))]
fn request_body_bytes(request: &reqwest::Request) -> Result<Option<&[u8]>, &'static str> {
    match request.body() {
        Some(body) => body.as_bytes().map(Some).ok_or(
            "reqwest::Request code recovery requires an in-memory body; streamed bodies cannot be reconstructed from &self",
        ),
        None => Ok(None),
    }
}

impl Prompt for reqwest::Request {
    fn prompt() -> Option<&'static str> {
        Some(
            "Construct an HTTP request snapshot from its method, URL, headers, \
             optional body bytes, version, and optional timeout.",
        )
    }
}

impl Elicitation for reqwest::Request {
    type Style = RequestStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::Request");

        let method = reqwest::Method::elicit(communicator).await?;
        let url = Url::elicit(communicator).await?;
        let headers = http::HeaderMap::elicit(communicator).await?;
        let body = Option::<Vec<u8>>::elicit(communicator).await?;
        let version = reqwest::Version::elicit(communicator).await?;
        let timeout = Option::<Duration>::elicit(communicator).await?;

        tracing::debug!(
            ?method,
            %url,
            header_count = headers.len(),
            has_body = body.is_some(),
            ?version,
            has_timeout = timeout.is_some(),
            "Request parts selected",
        );

        Ok(build_request(method, url, headers, body, version, timeout))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("request")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("request")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("request")
    }
}

impl ElicitIntrospect for reqwest::Request {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::Request",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "method",
                        type_name: "reqwest::Method",
                        prompt: Some("HTTP method"),
                    },
                    crate::FieldInfo {
                        name: "url",
                        type_name: "url::Url",
                        prompt: Some("Target URL"),
                    },
                    crate::FieldInfo {
                        name: "headers",
                        type_name: "http::HeaderMap",
                        prompt: Some("HTTP headers"),
                    },
                    crate::FieldInfo {
                        name: "body",
                        type_name: "Option<Vec<u8>>",
                        prompt: Some("Optional raw request body bytes"),
                    },
                    crate::FieldInfo {
                        name: "version",
                        type_name: "reqwest::Version",
                        prompt: Some("HTTP protocol version"),
                    },
                    crate::FieldInfo {
                        name: "timeout",
                        type_name: "Option<std::time::Duration>",
                        prompt: Some("Optional request timeout"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for reqwest::Request {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "reqwest::Request".to_string(),
            fields: vec![
                (
                    "method".to_string(),
                    Box::new(
                        <reqwest::Method as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("HTTP method".to_string())),
                    ),
                ),
                (
                    "url".to_string(),
                    Box::new(
                        <url::Url as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("Target URL".to_string())),
                    ),
                ),
                (
                    "headers".to_string(),
                    Box::new(
                        <http::HeaderMap as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("HTTP headers".to_string())),
                    ),
                ),
                (
                    "body".to_string(),
                    Box::new(
                        <Option<Vec<u8>> as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("Optional raw request body bytes".to_string())),
                    ),
                ),
                (
                    "version".to_string(),
                    Box::new(
                        <reqwest::Version as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("HTTP protocol version".to_string())),
                    ),
                ),
                (
                    "timeout".to_string(),
                    Box::new(
                        <Option<Duration> as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("Optional request timeout".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for reqwest::Request {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let method_bytes = self.method().as_str().as_bytes().iter();
        let url = self.url().to_code_literal();
        let headers = self.headers().to_code_literal();
        let version = self.version().to_code_literal();
        let timeout = self
            .timeout()
            .copied()
            .map(|timeout| timeout.to_code_literal());
        let body = match request_body_bytes(self) {
            Ok(body) => body.map(|bytes| bytes.iter()),
            Err(message) => {
                return quote::quote! {{
                    compile_error!(#message);
                }};
            }
        };

        let body_assignment = match body {
            Some(bytes) => quote::quote! {
                *request.body_mut() = ::std::option::Option::Some(
                    reqwest::Body::from(::std::vec![#(#bytes),*]),
                );
            },
            None => quote::quote! {},
        };

        let timeout_assignment = match timeout {
            Some(timeout) => quote::quote! {
                *request.timeout_mut() = ::std::option::Option::Some(#timeout);
            },
            None => quote::quote! {},
        };

        quote::quote! {{
            let __method = match reqwest::Method::from_bytes(&[#(#method_bytes),*]) {
                ::std::result::Result::Ok(method) => method,
                ::std::result::Result::Err(error) => {
                    return ::std::result::Result::Err(::http::Error::from(error).into());
                }
            };
            let mut request = reqwest::Request::new(__method, #url);
            *request.headers_mut() = #headers;
            *request.version_mut() = #version;
            #timeout_assignment
            #body_assignment
            request
        }}
    }
}
