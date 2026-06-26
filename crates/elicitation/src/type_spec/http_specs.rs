//! [`ElicitSpec`](crate::ElicitSpec) implementations for HTTP contract types.
//!
//! Available with the `reqwest` feature.

#[cfg(feature = "reqwest")]
use crate::{
    ElicitSpec, SpecCategory, SpecCategoryBuilder, SpecEntry, SpecEntryBuilder, TypeSpec,
    TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── reqwest ───────────────────────────────────────────────────────────────────

#[cfg(feature = "reqwest")]
mod reqwest_specs {
    use super::*;
    use bytes::Bytes;
    use crate::verification::types::StatusCodeValid;
    use http::{HeaderMap, HeaderName, HeaderValue};
    use reqwest::tls::Version as TlsVersion;
    use reqwest::{Body, Method, Request, StatusCode, Version};

    impl ElicitSpec for Bytes {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "from_vec",
                    "Direct elicitation and code recovery rebuild bytes::Bytes from an exact owned byte vector.",
                )
                .with_expression(Some("bytes::Bytes::from(Vec<u8>)".to_string()))],
            );
            let related = SpecCategory::new(
                "related",
                vec![SpecEntry::new(
                    "visible_slice",
                    "Core support preserves the exact visible byte slice of the current Bytes view.",
                )
                .with_expression(Some("value.as_ref()".to_string()))],
            );

            TypeSpec::new(
                "bytes::Bytes",
                "Cheaply cloneable contiguous byte buffer used by reqwest and other networking APIs.",
                vec![construction, related],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "bytes::Bytes",
        <Bytes as ElicitSpec>::type_spec,
        std::any::TypeId::of::<Bytes>
    ));

    impl ElicitSpec for StatusCodeValid {
        fn type_spec() -> TypeSpec {
            let requires =
                SpecCategoryBuilder::default()
                    .name("requires".to_string())
                    .entries(vec![SpecEntryBuilder::default()
                    .label("100..=999".to_string())
                    .description(
                        "Status code must be in the range 100–999 (valid HTTP status codes)."
                            .to_string(),
                    )
                    .expression(Some("(100..=999).contains(&value)".to_string()))
                    .build()
                    .expect("valid SpecEntry")])
                    .build()
                    .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("base_type".to_string())
                        .description("Wraps a reqwest::StatusCode".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("StatusCodeValid".to_string())
                .summary(
                    "A valid HTTP status code (100–999), validated via reqwest::StatusCode::from_u16()."
                        .to_string(),
                )
                .categories(vec![requires, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "StatusCodeValid",
        <StatusCodeValid as ElicitSpec>::type_spec,
        std::any::TypeId::of::<StatusCodeValid>
    ));

    impl ElicitSpec for reqwest::Client {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategoryBuilder::default()
                .name("construction".to_string())
                .entries(vec![SpecEntryBuilder::default()
                    .label("default_client".to_string())
                    .description(
                        "The direct elicitation path constructs reqwest::Client with Client::new()."
                            .to_string(),
                    )
                    .expression(Some("reqwest::Client::new()".to_string()))
                    .build()
                    .expect("valid SpecEntry")])
                .build()
                .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![SpecEntryBuilder::default()
                    .label("role".to_string())
                    .description(
                        "HTTP client handle used to build and execute requests.".to_string(),
                    )
                    .expression(None)
                    .build()
                    .expect("valid SpecEntry")])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("reqwest::Client".to_string())
                .summary("HTTP client handle with connection pooling, cookies, redirects, and request execution.".to_string())
                .categories(vec![construction, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Client",
        <reqwest::Client as ElicitSpec>::type_spec,
        std::any::TypeId::of::<reqwest::Client>
    ));

    impl ElicitSpec for Body {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "from_bytes",
                    "Direct elicitation and code recovery rebuild reqwest::Body from an exact owned byte vector.",
                )
                .with_expression(Some("reqwest::Body::from(Vec<u8>)".to_string()))],
            );
            let related = SpecCategory::new(
                "related",
                vec![SpecEntry::new(
                    "in_memory_subset",
                    "Core support models the in-memory body subset; streamed bodies are not recoverable from an immutable borrow.",
                )],
            );

            TypeSpec::new(
                "reqwest::Body",
                "HTTP request body payload carried as owned in-memory bytes or as a streaming body.",
                vec![construction, related],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Body",
        <Body as ElicitSpec>::type_spec,
        std::any::TypeId::of::<Body>
    ));

    impl ElicitSpec for HeaderName {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "from_bytes",
                    "Direct elicitation parses the header name with http::HeaderName::from_bytes().",
                )
                .with_expression(Some(
                    "http::HeaderName::from_bytes(value.as_bytes())".to_string(),
                ))],
            );
            let related = SpecCategory::new(
                "related",
                vec![SpecEntry::new(
                    "normalized",
                    "HTTP header names are validated tokens and are stored in normalized lowercase form.",
                )
                .with_expression(Some("value.as_str()".to_string()))],
            );

            TypeSpec::new(
                "http::HeaderName",
                "HTTP header name token suitable for request or response headers.",
                vec![construction, related],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "http::HeaderName",
        <HeaderName as ElicitSpec>::type_spec,
        std::any::TypeId::of::<HeaderName>
    ));

    impl ElicitSpec for HeaderValue {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategoryBuilder::default()
                .name("construction".to_string())
                .entries(vec![SpecEntryBuilder::default()
                    .label("from_str".to_string())
                    .description(
                        "Direct elicitation parses the value with http::HeaderValue::from_str()."
                            .to_string(),
                    )
                    .expression(Some("http::HeaderValue::from_str(value)".to_string()))
                    .build()
                    .expect("valid SpecEntry")])
                .build()
                .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("bytes".to_string())
                        .description(
                            "Code recovery rebuilds the header value from its exact byte representation."
                                .to_string(),
                        )
                        .expression(Some("value.as_bytes()".to_string()))
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("http::HeaderValue".to_string())
                .summary("HTTP header value with RFC-valid byte content suitable for request or response headers.".to_string())
                .categories(vec![construction, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "http::HeaderValue",
        <HeaderValue as ElicitSpec>::type_spec,
        std::any::TypeId::of::<HeaderValue>
    ));

    impl ElicitSpec for http::Error {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategory::new(
                "variants",
                vec![
                    SpecEntry::new(
                        "invalid_status_code",
                        "Wraps http::status::InvalidStatusCode from an out-of-range status code.",
                    )
                    .with_expression(Some("http::StatusCode::from_u16(value)".to_string())),
                    SpecEntry::new(
                        "invalid_method",
                        "Wraps http::method::InvalidMethod from an invalid HTTP method token.",
                    )
                    .with_expression(Some("http::Method::from_bytes(value.as_bytes())".to_string())),
                    SpecEntry::new(
                        "invalid_uri",
                        "Wraps http::uri::InvalidUri from a malformed URI string.",
                    )
                    .with_expression(Some("http::Uri::from_str(value)".to_string())),
                    SpecEntry::new(
                        "invalid_uri_parts",
                        "Wraps http::uri::InvalidUriParts from an inconsistent URI parts builder state.",
                    )
                    .with_expression(Some("http::Uri::from_parts(parts)".to_string())),
                    SpecEntry::new(
                        "invalid_header_name",
                        "Wraps http::header::InvalidHeaderName from an invalid header token.",
                    )
                    .with_expression(Some("http::HeaderName::from_bytes(value.as_bytes())".to_string())),
                    SpecEntry::new(
                        "invalid_header_value",
                        "Wraps http::header::InvalidHeaderValue from invalid header bytes.",
                    )
                    .with_expression(Some("http::HeaderValue::from_bytes(value)".to_string())),
                    SpecEntry::new(
                        "max_size_reached",
                        "Wraps http::header::MaxSizeReached when a HeaderMap allocation exceeds library limits.",
                    )
                    .with_expression(Some("http::HeaderMap::<()>::try_with_capacity(usize::MAX)".to_string())),
                ],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "closed_public_error_family",
                    "Core support reconstructs http::Error through one of its closed public source-error constructors rather than through ad hoc string parsing.",
                )],
            );

            TypeSpec::new(
                "http::Error",
                "Closed public error family used by the http crate for builder, parser, and header-map capacity failures.",
                vec![variants, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "http::Error",
        <http::Error as ElicitSpec>::type_spec,
        std::any::TypeId::of::<http::Error>
    ));

    impl ElicitSpec for Method {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategory::new(
                "variants",
                vec![
                    SpecEntry::new("GET", "Fetch a representation of the target resource."),
                    SpecEntry::new("POST", "Submit data to the target resource."),
                    SpecEntry::new("PUT", "Replace the target resource with the supplied payload."),
                    SpecEntry::new("DELETE", "Remove the target resource."),
                    SpecEntry::new("PATCH", "Apply a partial update to the target resource."),
                    SpecEntry::new("HEAD", "Fetch response metadata without a response body."),
                    SpecEntry::new("OPTIONS", "Query the communication options for the target resource."),
                    SpecEntry::new("CONNECT", "Establish a tunnel to the target server."),
                    SpecEntry::new("TRACE", "Echo the received request for diagnostics."),
                ],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "from_bytes",
                    "Code recovery rebuilds the method token from its exact byte representation.",
                )
                .with_expression(Some("reqwest::Method::from_bytes(value.as_str().as_bytes())".to_string()))],
            );

            TypeSpec::new(
                "reqwest::Method",
                "HTTP request method token used to route request semantics.",
                vec![variants, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Method",
        <Method as ElicitSpec>::type_spec,
        std::any::TypeId::of::<Method>
    ));

    impl ElicitSpec for HeaderMap {
        fn type_spec() -> TypeSpec {
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "line_format",
                    "Interactive elicitation accepts newline-separated `Name: Value` pairs and parses each pair into a header multimap entry.",
                )
                .with_expression(Some("http::HeaderMap::new() + append(HeaderName, HeaderValue)".to_string()))],
            );
            let related = SpecCategory::new(
                "related",
                vec![SpecEntry::new(
                    "multimap",
                    "HeaderMap preserves repeated header names by storing multiple values under the same name.",
                )],
            );

            TypeSpec::new(
                "http::HeaderMap",
                "HTTP header multimap mapping validated header names to one or more header values.",
                vec![construction, related],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "http::HeaderMap",
        <HeaderMap as ElicitSpec>::type_spec,
        std::any::TypeId::of::<HeaderMap>
    ));

    impl ElicitSpec for StatusCode {
        fn type_spec() -> TypeSpec {
            let requires = SpecCategory::new(
                "requires",
                vec![SpecEntry::new(
                    "100..=999",
                    "Status code must remain within the valid HTTP status-code range accepted by reqwest::StatusCode::from_u16().",
                )
                .with_expression(Some("(100..=999).contains(&value.as_u16())".to_string()))],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "from_u16",
                    "Code recovery rebuilds the status code from its exact numeric value.",
                )
                .with_expression(Some("reqwest::StatusCode::from_u16(value.as_u16())".to_string()))],
            );

            TypeSpec::new(
                "reqwest::StatusCode",
                "Validated HTTP status code covering informational, success, redirection, client-error, and server-error responses.",
                vec![requires, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::StatusCode",
        <StatusCode as ElicitSpec>::type_spec,
        std::any::TypeId::of::<StatusCode>
    ));

    impl ElicitSpec for Version {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategory::new(
                "variants",
                vec![
                    SpecEntry::new("HTTP/1.0", "HTTP/1.0 request or response framing."),
                    SpecEntry::new("HTTP/1.1", "HTTP/1.1 request or response framing."),
                    SpecEntry::new("HTTP/2.0", "HTTP/2 request or response framing."),
                    SpecEntry::new("HTTP/3.0", "HTTP/3 request or response framing."),
                ],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "constant_variants",
                    "Code recovery rebuilds the version using the corresponding reqwest::Version constant.",
                )],
            );

            TypeSpec::new(
                "reqwest::Version",
                "HTTP protocol version tag carried by requests and responses.",
                vec![variants, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Version",
        <Version as ElicitSpec>::type_spec,
        std::any::TypeId::of::<Version>
    ));

    impl ElicitSpec for TlsVersion {
        fn type_spec() -> TypeSpec {
            let variants = SpecCategory::new(
                "variants",
                vec![
                    SpecEntry::new("TLS 1.0", "TLS protocol version 1.0."),
                    SpecEntry::new("TLS 1.1", "TLS protocol version 1.1."),
                    SpecEntry::new("TLS 1.2", "TLS protocol version 1.2."),
                    SpecEntry::new("TLS 1.3", "TLS protocol version 1.3."),
                ],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![SpecEntry::new(
                    "constant_variants",
                    "Code recovery rebuilds the version using the corresponding reqwest::tls::Version constant.",
                )],
            );

            TypeSpec::new(
                "reqwest::tls::Version",
                "TLS protocol version selector used by reqwest client configuration.",
                vec![variants, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::tls::Version",
        <TlsVersion as ElicitSpec>::type_spec,
        std::any::TypeId::of::<TlsVersion>
    ));

    impl ElicitSpec for Request {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategory::new(
                "fields",
                vec![
                    SpecEntry::new("method", "HTTP method token controlling request semantics."),
                    SpecEntry::new("url", "Target request URL."),
                    SpecEntry::new(
                        "headers",
                        "HTTP header multimap attached to the request.",
                    ),
                    SpecEntry::new(
                        "body",
                        "Optional request body, represented in core support as optional raw bytes.",
                    ),
                    SpecEntry::new(
                        "version",
                        "HTTP protocol version carried on the request.",
                    ),
                    SpecEntry::new("timeout", "Optional per-request timeout override."),
                ],
            );
            let construction = SpecCategory::new(
                "construction",
                vec![
                    SpecEntry::new(
                        "request_new",
                        "Direct elicitation starts from reqwest::Request::new(method, url).",
                    )
                    .with_expression(Some("reqwest::Request::new(method, url)".to_string())),
                    SpecEntry::new(
                        "mutators",
                        "Headers, version, timeout, and optional body are then applied through reqwest's public mutable accessors.",
                    ),
                ],
            );

            TypeSpec::new(
                "reqwest::Request",
                "Owned HTTP request carrying method, URL, headers, optional body, protocol version, and optional timeout.",
                vec![fields, construction],
            )
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Request",
        <Request as ElicitSpec>::type_spec,
        std::any::TypeId::of::<Request>
    ));

    impl ElicitSpec for reqwest::RequestBuilder {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategoryBuilder::default()
                .name("fields".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("method".to_string())
                        .description("HTTP method used for the request.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("url".to_string())
                        .description("Target URL for the request.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![SpecEntryBuilder::default()
                    .label("builder_state".to_string())
                    .description(
                        "Additional state such as headers, body, timeout, or version may be layered on after construction."
                            .to_string(),
                    )
                    .expression(None)
                    .build()
                    .expect("valid SpecEntry")])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("reqwest::RequestBuilder".to_string())
                .summary("Builder for an HTTP request bound to a reqwest client.".to_string())
                .categories(vec![fields, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::RequestBuilder",
        <reqwest::RequestBuilder as ElicitSpec>::type_spec,
        std::any::TypeId::of::<reqwest::RequestBuilder>
    ));

    impl ElicitSpec for reqwest::Response {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategoryBuilder::default()
                .name("fields".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("status".to_string())
                        .description("HTTP status code carried by the response.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("body".to_string())
                        .description("Response body bytes or text payload.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("url".to_string())
                        .description("Final response URL.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![SpecEntryBuilder::default()
                    .label("mockable".to_string())
                    .description(
                        "Direct elicitation constructs a mock reqwest::Response from status, body, and URL."
                            .to_string(),
                    )
                    .expression(None)
                    .build()
                    .expect("valid SpecEntry")])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("reqwest::Response".to_string())
                .summary("HTTP response carrying status, headers, metadata, and a body payload.".to_string())
                .categories(vec![fields, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Response",
        <reqwest::Response as ElicitSpec>::type_spec,
        std::any::TypeId::of::<reqwest::Response>
    ));

    impl ElicitSpec for reqwest::Error {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategoryBuilder::default()
                .name("fields".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("status".to_string())
                        .description(
                            "HTTP status code when the error came from Response::error_for_status."
                                .to_string(),
                        )
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("url".to_string())
                        .description("URL associated with the failed request when known.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let classification = SpecCategoryBuilder::default()
                .name("classification".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("is_builder".to_string())
                        .description("True for request-builder and setup failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_redirect".to_string())
                        .description("True for redirect-policy failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_status".to_string())
                        .description("True for errors produced by Response::error_for_status.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_timeout".to_string())
                        .description("True when the error or one of its sources indicates a timeout.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_request".to_string())
                        .description("True for request execution failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_connect".to_string())
                        .description("True for connection-establishment failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_body".to_string())
                        .description("True for request or response body transport failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_decode".to_string())
                        .description("True for response-body decoding failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("is_upgrade".to_string())
                        .description("True for HTTP upgrade failures.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let url_management = SpecCategoryBuilder::default()
                .name("url_management".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("url_mut".to_string())
                        .description("Mutable access to the associated URL when one is present.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("with_url".to_string())
                        .description("Attach or replace the URL associated with the error.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("without_url".to_string())
                        .description("Strip any associated URL from the error.".to_string())
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let related = SpecCategoryBuilder::default()
                .name("related".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("direct_subset".to_string())
                        .description(
                            "Direct elicitation and code recovery model the public HTTP status-error subclass because reqwest does not expose public constructors for arbitrary runtime error kinds."
                                .to_string(),
                        )
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("shadow_recommended".to_string())
                        .description(
                            "Use the shadow crate when you need full-fidelity, serializable error snapshots across all reqwest error classifications."
                                .to_string(),
                        )
                        .expression(None)
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("reqwest::Error".to_string())
                .summary("Reqwest runtime error with optional status and URL context.".to_string())
                .categories(vec![fields, classification, url_management, related])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "reqwest::Error",
        <reqwest::Error as ElicitSpec>::type_spec,
        std::any::TypeId::of::<reqwest::Error>
    ));
}
