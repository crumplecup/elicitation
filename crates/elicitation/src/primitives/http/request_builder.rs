//! `reqwest::RequestBuilder` elicitation (Survey pattern).
//!
//! Elicits an HTTP method and URL, then constructs a `RequestBuilder` via
//! `RequestBuilder::from_parts(Client::new(), Request::new(method, url))`.
//! This makes mock request builders trivially available to agents for testing.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata, emit_code::ToCodeLiteral,
};
use reqwest::{Client, Request, RequestBuilder};
use url::Url;

crate::default_style!(RequestBuilder => RequestBuilderStyle);

impl Prompt for RequestBuilder {
    fn prompt() -> Option<&'static str> {
        Some(
            "Construct an HTTP request builder — provide the method and target URL. \
             Additional configuration (headers, body, timeout) can be applied via \
             subsequent builder method calls.",
        )
    }
}

impl Elicitation for RequestBuilder {
    type Style = RequestBuilderStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting reqwest::RequestBuilder");

        let method = reqwest::Method::elicit(communicator).await?;
        tracing::debug!(?method, "Method selected");

        let url = Url::elicit(communicator).await?;
        tracing::debug!(%url, "URL selected");

        let request = Request::new(method, url);
        Ok(RequestBuilder::from_parts(Client::new(), request))
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("request_builder")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("request_builder")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("request_builder")
    }
}

impl ElicitIntrospect for RequestBuilder {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "reqwest::RequestBuilder",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    crate::FieldInfo {
                        name: "method",
                        type_name: "reqwest::Method",
                        prompt: Some("HTTP method (GET, POST, …)"),
                    },
                    crate::FieldInfo {
                        name: "url",
                        type_name: "url::Url",
                        prompt: Some("Target URL"),
                    },
                ],
            },
        }
    }
}

impl crate::ElicitPromptTree for RequestBuilder {
    fn prompt_tree() -> crate::PromptTree {
        crate::PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "reqwest::RequestBuilder".to_string(),
            fields: vec![
                (
                    "method".to_string(),
                    Box::new(
                        <reqwest::Method as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("HTTP method (GET, POST, ... )".to_string())),
                    ),
                ),
                (
                    "url".to_string(),
                    Box::new(
                        <url::Url as crate::ElicitPromptTree>::prompt_tree()
                            .with_prompt(Some("Target URL".to_string())),
                    ),
                ),
            ],
        }
    }
}

impl ToCodeLiteral for RequestBuilder {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let Some(cloned) = self.try_clone() else {
            let message = "reqwest::RequestBuilder code recovery requires a cloneable builder with a non-stream body and no retained builder error";
            return quote::quote! {{
                compile_error!(#message);
            }};
        };

        let (client, request_result) = cloned.build_split();
        let request = match request_result {
            Ok(request) => request,
            Err(_) => {
                let message = "reqwest::RequestBuilder code recovery requires a builder that still holds a recoverable reqwest::Request";
                return quote::quote! {{
                    compile_error!(#message);
                }};
            }
        };

        let method_bytes = request.method().as_str().as_bytes().iter();
        let url = request.url().to_code_literal();
        let headers = request.headers().to_code_literal();
        let version = request.version().to_code_literal();
        let client = client.to_code_literal();
        let timeout = request
            .timeout()
            .copied()
            .map(|timeout| timeout.to_code_literal());
        let body = match request.body() {
            Some(body) => match body.as_bytes() {
                Some(bytes) => Some(bytes.iter()),
                None => {
                    let message = "reqwest::RequestBuilder code recovery requires an in-memory body; streamed bodies cannot be reconstructed from &self";
                    return quote::quote! {{
                        compile_error!(#message);
                    }};
                }
            },
            None => None,
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
            reqwest::RequestBuilder::from_parts(#client, request)
        }}
    }
}
