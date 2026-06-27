//! `http::Error` elicitation support.

use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitPromptTree,
    ElicitResult, Elicitation, ElicitationPattern, FieldInfo, PatternDetails, Prompt, PromptTree,
    TypeMetadata, mcp,
};
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode, Uri};
use std::str::FromStr;

crate::default_style!(http::Error => HttpErrorStyle);

const INVALID_STATUS_CODE_LABEL: &str = "InvalidStatusCode";
const INVALID_METHOD_LABEL: &str = "InvalidMethod";
const INVALID_URI_LABEL: &str = "InvalidUri";
const INVALID_URI_PARTS_LABEL: &str = "InvalidUriParts";
const INVALID_HEADER_NAME_LABEL: &str = "InvalidHeaderName";
const INVALID_HEADER_VALUE_LABEL: &str = "InvalidHeaderValue";
const MAX_SIZE_REACHED_LABEL: &str = "MaxSizeReached";

impl Prompt for http::Error {
    fn prompt() -> Option<&'static str> {
        Some(
            "Construct an http::Error by choosing the public variant family and supplying invalid source data when needed.",
        )
    }
}

impl Elicitation for http::Error {
    type Style = HttpErrorStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting http::Error");

        let variant = elicit_error_variant(communicator).await?;

        match variant.as_str() {
            INVALID_STATUS_CODE_LABEL => {
                let invalid = u16::elicit(communicator).await?;
                build_invalid_status_code_error(invalid)
            }
            INVALID_METHOD_LABEL => {
                let invalid = String::elicit(communicator).await?;
                build_invalid_method_error(&invalid)
            }
            INVALID_URI_LABEL => {
                let invalid = String::elicit(communicator).await?;
                build_invalid_uri_error(&invalid)
            }
            INVALID_URI_PARTS_LABEL => build_invalid_uri_parts_error(),
            INVALID_HEADER_NAME_LABEL => {
                let invalid = String::elicit(communicator).await?;
                build_invalid_header_name_error(&invalid)
            }
            INVALID_HEADER_VALUE_LABEL => {
                let invalid = String::elicit(communicator).await?;
                build_invalid_header_value_error(&invalid)
            }
            MAX_SIZE_REACHED_LABEL => build_max_size_reached_error(),
            label => Err(ElicitError::new(ElicitErrorKind::InvalidSelection(
                label.to_string(),
            ))),
        }
    }

    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("http_error")
    }

    fn verus_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("http_error")
    }

    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("http_error")
    }
}

impl ElicitIntrospect for http::Error {
    fn pattern() -> ElicitationPattern {
        ElicitationPattern::Survey
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "http::Error",
            description: Self::prompt(),
            details: PatternDetails::Survey {
                fields: vec![
                    FieldInfo {
                        name: "variant",
                        type_name: "String",
                        prompt: Some(
                            "One of: InvalidStatusCode, InvalidMethod, InvalidUri, InvalidUriParts, InvalidHeaderName, InvalidHeaderValue, MaxSizeReached",
                        ),
                    },
                    FieldInfo {
                        name: "invalid_input",
                        type_name: "String",
                        prompt: Some(
                            "Variant-specific invalid source data for the constructor path.",
                        ),
                    },
                ],
            },
        }
    }
}

impl ElicitPromptTree for http::Error {
    fn prompt_tree() -> PromptTree {
        PromptTree::Survey {
            prompt: Self::prompt().map(str::to_string),
            type_name: "http::Error".to_string(),
            fields: vec![
                (
                    "variant".to_string(),
                    Box::new(PromptTree::Leaf {
                        prompt: "Choose one of: InvalidStatusCode, InvalidMethod, InvalidUri, InvalidUriParts, InvalidHeaderName, InvalidHeaderValue, MaxSizeReached".to_string(),
                        type_name: "String".to_string(),
                    }),
                ),
                (
                    "invalid_input".to_string(),
                    Box::new(PromptTree::Leaf {
                        prompt: "Provide invalid source data when the selected variant needs it; InvalidUriParts and MaxSizeReached ignore this field.".to_string(),
                        type_name: "String".to_string(),
                    }),
                ),
            ],
        }
    }
}

impl crate::emit_code::ToCodeLiteral for http::Error {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        if self.is::<http::status::InvalidStatusCode>() {
            return quote::quote! {{
                match http::StatusCode::from_u16(0) {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid HTTP status unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::method::InvalidMethod>() {
            return quote::quote! {{
                match http::Method::from_bytes(b"bad method") {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid HTTP method unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::uri::InvalidUri>() {
            return quote::quote! {{
                match <http::Uri as ::std::str::FromStr>::from_str("http://[::1") {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid URI unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::uri::InvalidUriParts>() {
            return quote::quote! {{
                let mut parts = http::uri::Parts::default();
                parts.scheme = Some(http::uri::Scheme::HTTP);

                match http::Uri::from_parts(parts) {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid URI parts unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::header::InvalidHeaderName>() {
            return quote::quote! {{
                match http::header::HeaderName::from_bytes(b"bad header") {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid header name unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::header::InvalidHeaderValue>() {
            return quote::quote! {{
                match http::header::HeaderValue::from_bytes(&[b'\n']) {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic invalid header value unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        if self.is::<http::header::MaxSizeReached>() {
            return quote::quote! {{
                match http::HeaderMap::<()>::try_with_capacity(::std::usize::MAX) {
                    Ok(_) => {
                        return Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::Validation(
                                "synthetic header map max-size error unexpectedly succeeded".to_string(),
                            ),
                        )
                        .into());
                    }
                    Err(error) => http::Error::from(error),
                }
            }};
        }

        let message = "http::Error contained an unsupported public error variant";
        quote::quote! {{
            compile_error!(#message);
        }}
    }
}

#[tracing::instrument(skip(communicator))]
async fn elicit_error_variant<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<String> {
    let labels = [
        INVALID_STATUS_CODE_LABEL,
        INVALID_METHOD_LABEL,
        INVALID_URI_LABEL,
        INVALID_URI_PARTS_LABEL,
        INVALID_HEADER_NAME_LABEL,
        INVALID_HEADER_VALUE_LABEL,
        MAX_SIZE_REACHED_LABEL,
    ];
    let options = labels
        .iter()
        .map(|label| (*label).to_string())
        .collect::<Vec<_>>();
    let params = mcp::select_params(
        "Select the public http::Error variant family to reconstruct:",
        &options,
    );
    let result = communicator
        .call_tool(
            rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                .with_arguments(params),
        )
        .await?;

    let value = mcp::extract_value(result)?;
    mcp::parse_string(value)
}

#[tracing::instrument]
fn build_invalid_status_code_error(value: u16) -> ElicitResult<http::Error> {
    match StatusCode::from_u16(value) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "http::Error::InvalidStatusCode requires a u16 outside 100..=999".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_invalid_method_error(raw: &str) -> ElicitResult<http::Error> {
    match Method::from_bytes(raw.as_bytes()) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "http::Error::InvalidMethod requires an invalid HTTP method token".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_invalid_uri_error(raw: &str) -> ElicitResult<http::Error> {
    match Uri::from_str(raw) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "http::Error::InvalidUri requires an invalid URI string".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_invalid_uri_parts_error() -> ElicitResult<http::Error> {
    let mut parts = http::uri::Parts::default();
    parts.scheme = Some(http::uri::Scheme::HTTP);

    match Uri::from_parts(parts) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "synthetic invalid URI parts unexpectedly succeeded".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_invalid_header_name_error(raw: &str) -> ElicitResult<http::Error> {
    match HeaderName::from_bytes(raw.as_bytes()) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "http::Error::InvalidHeaderName requires an invalid header name token".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_invalid_header_value_error(raw: &str) -> ElicitResult<http::Error> {
    match HeaderValue::from_str(raw) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "http::Error::InvalidHeaderValue requires invalid header bytes".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}

#[tracing::instrument]
fn build_max_size_reached_error() -> ElicitResult<http::Error> {
    match HeaderMap::<()>::try_with_capacity(usize::MAX) {
        Ok(_) => Err(ElicitError::new(ElicitErrorKind::Validation(
            "synthetic header map max-size error unexpectedly succeeded".to_string(),
        ))),
        Err(error) => Ok(error.into()),
    }
}
