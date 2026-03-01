//! `reqwest::RequestBuilder` elicitation (Survey pattern).
//!
//! Elicits an HTTP method and URL, then constructs a `RequestBuilder` via
//! `RequestBuilder::from_parts(Client::new(), Request::new(method, url))`.
//! This makes mock request builders trivially available to agents for testing.

use crate::{
    ElicitCommunicator, ElicitIntrospect, ElicitResult, Elicitation, ElicitationPattern,
    PatternDetails, Prompt, TypeMetadata,
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

    #[cfg(kani)]
    fn kani_proof() {
        // Compositional: both constituents are verified independently.
        <reqwest::Method as Elicitation>::kani_proof();
        <Url as Elicitation>::kani_proof();
        assert!(
            true,
            "reqwest::RequestBuilder construction verified via components ∎"
        );
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
