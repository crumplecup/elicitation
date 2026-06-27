//! Structured reqwest error shadow.

use elicitation::{
    Elicit, ElicitCommunicator, ElicitError, ElicitIntrospect, ElicitPromptTree, ElicitResult,
    ElicitSpec, Elicitation, Prompt, PromptTree, TypeMetadata, TypeSpec, emit_code::ToCodeLiteral,
};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{StatusCode, Url};

/// Classification flags for a reqwest error snapshot.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe reqwest error classification flags:")]
pub struct ErrorFlags {
    #[prompt("Whether this is a builder/setup error:")]
    is_builder: bool,
    #[prompt("Whether this is a redirect error:")]
    is_redirect: bool,
    #[prompt("Whether this is a status error:")]
    is_status: bool,
    #[prompt("Whether this is a timeout error:")]
    is_timeout: bool,
    #[prompt("Whether this is a request execution error:")]
    is_request: bool,
    #[prompt("Whether this is a connect error:")]
    is_connect: bool,
    #[prompt("Whether this is a body transport error:")]
    is_body: bool,
    #[prompt("Whether this is a decode/parsing error:")]
    is_decode: bool,
    #[prompt("Whether this is an upgrade error:")]
    is_upgrade: bool,
}

/// Elicitation-aware reqwest error snapshot.
#[derive(Debug, Clone)]
pub struct Error {
    snapshot: ErrorSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Describe an HTTP error snapshot:")]
struct ErrorSnapshot {
    #[prompt("Human-readable error message:")]
    message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("Error URL when known:")]
    url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[prompt("HTTP status associated with the error when present:")]
    status: Option<StatusCode>,
    #[serde(default)]
    #[prompt("Reqwest error classification flags:")]
    flags: ErrorFlags,
}

impl Serialize for Error {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Error {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            snapshot: ErrorSnapshot::deserialize(deserializer)?,
        })
    }
}

impl JsonSchema for Error {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("Error")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <ErrorSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl Prompt for Error {
    fn prompt() -> Option<&'static str> {
        Some("Describe an HTTP error snapshot:")
    }
}

impl Elicitation for Error {
    type Style = ();

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let snapshot = ErrorSnapshot::elicit(communicator).await?;
        Ok(Self { snapshot })
    }

    fn kani_proof() -> elicitation::proc_macro2::TokenStream {
        ErrorSnapshot::kani_proof()
    }

    fn verus_proof() -> elicitation::proc_macro2::TokenStream {
        ErrorSnapshot::verus_proof()
    }

    fn creusot_proof() -> elicitation::proc_macro2::TokenStream {
        ErrorSnapshot::creusot_proof()
    }
}

impl ElicitIntrospect for Error {
    fn pattern() -> elicitation::ElicitationPattern {
        ErrorSnapshot::pattern()
    }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "Error",
            description: Self::prompt(),
            details: ErrorSnapshot::metadata().details,
        }
    }
}

impl ElicitPromptTree for Error {
    fn prompt_tree() -> PromptTree {
        match ErrorSnapshot::prompt_tree() {
            PromptTree::Survey { fields, .. } => PromptTree::Survey {
                prompt: Self::prompt().map(str::to_string),
                type_name: "Error".to_string(),
                fields,
            },
            tree => tree.with_prompt(Self::prompt().map(str::to_string)),
        }
    }
}

impl ElicitSpec for Error {
    fn type_spec() -> TypeSpec {
        let base = ErrorSnapshot::type_spec();
        TypeSpec::new(
            "Error",
            "Structured reqwest error snapshot with classification flags and optional URL/status context.",
            base.categories().clone(),
        )
    }
}

impl Error {
    /// Construct a structured reqwest error snapshot from explicit parts.
    pub fn from_parts(
        message: String,
        url: Option<Url>,
        status: Option<StatusCode>,
        flags: ErrorFlags,
    ) -> Self {
        Self {
            snapshot: ErrorSnapshot {
                message,
                url,
                status,
                flags,
            },
        }
    }

    #[track_caller]
    pub(crate) fn builder(message: impl Into<String>) -> Self {
        Self::from_parts(
            message.into(),
            None,
            None,
            ErrorFlags {
                is_builder: true,
                ..ErrorFlags::default()
            },
        )
    }

    #[track_caller]
    pub(crate) fn status(status: crate::StatusCode, url: Option<Url>) -> Self {
        Self::from_parts(
            format!("HTTP status error: {}", status.as_u16()),
            url,
            Some(status),
            ErrorFlags {
                is_status: true,
                ..ErrorFlags::default()
            },
        )
    }

    #[track_caller]
    pub(crate) fn decode(message: impl Into<String>, url: Option<Url>) -> Self {
        Self::from_parts(
            message.into(),
            url,
            None,
            ErrorFlags {
                is_decode: true,
                ..ErrorFlags::default()
            },
        )
    }
}

impl From<ElicitError> for Error {
    #[track_caller]
    fn from(e: ElicitError) -> Self {
        Self::from_parts(
            e.to_string(),
            None,
            None,
            ErrorFlags {
                is_builder: true,
                ..ErrorFlags::default()
            },
        )
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::from_parts(
            value.to_string(),
            value.url().cloned().map(Url::from),
            value.status().map(StatusCode::from),
            ErrorFlags {
                is_builder: value.is_builder(),
                is_redirect: value.is_redirect(),
                is_status: value.is_status(),
                is_timeout: value.is_timeout(),
                is_request: value.is_request(),
                is_connect: value.is_connect(),
                is_body: value.is_body(),
                is_decode: value.is_decode(),
                is_upgrade: value.is_upgrade(),
            },
        )
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.snapshot.message)
    }
}

impl std::error::Error for Error {}

impl elicitation::ElicitComplete for Error {}

impl ToCodeLiteral for Error {
    fn to_code_literal(&self) -> elicitation::proc_macro2::TokenStream {
        let message = &self.snapshot.message;
        let url = self.snapshot.url.to_code_literal();
        let status = self.snapshot.status.to_code_literal();
        let f = &self.snapshot.flags;
        let is_builder = f.is_builder;
        let is_redirect = f.is_redirect;
        let is_status = f.is_status;
        let is_timeout = f.is_timeout;
        let is_request = f.is_request;
        let is_connect = f.is_connect;
        let is_body = f.is_body;
        let is_decode = f.is_decode;
        let is_upgrade = f.is_upgrade;
        quote::quote! {
            ::elicit_reqwest::Error::from_parts(
                #message.to_string(),
                #url,
                #status,
                ::elicit_reqwest::ErrorFlags {
                    is_builder: #is_builder,
                    is_redirect: #is_redirect,
                    is_status: #is_status,
                    is_timeout: #is_timeout,
                    is_request: #is_request,
                    is_connect: #is_connect,
                    is_body: #is_body,
                    is_decode: #is_decode,
                    is_upgrade: #is_upgrade,
                },
            )
        }
    }
}
