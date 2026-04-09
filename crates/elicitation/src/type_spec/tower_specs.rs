//! [`ElicitSpec`](crate::ElicitSpec) + [`ElicitComplete`](crate::ElicitComplete) impls
//! for tower and tower-http type trenchcoats.
//!
//! Available with the `tower-types` feature.

#[cfg(feature = "tower-types")]
mod tower_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    macro_rules! impl_tower_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [ $( ($field:literal, $desc:literal) ),+ $(,)? ]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let field_entries = vec![
                        $(
                            SpecEntryBuilder::default()
                                .label($field.to_string())
                                .description($desc.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        )+
                    ];
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(field_entries)
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("tower 0.5 / tower-http 0.6 — middleware for async services".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — elicit each field in sequence".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    macro_rules! impl_tower_unit_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal $(,)?
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("tower 0.5 / tower-http 0.6 — middleware for async services".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Unit — zero-configuration layer".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitComplete for $ty {}
        };
    }

    use crate::{
        TowerAndThenHandle, TowerBoxServiceHandle, TowerBufferHandle, TowerBufferLayer,
        TowerCatchPanicLayer, TowerClosed, TowerCompressionLayer, TowerConcurrencyLimitHandle,
        TowerConcurrencyLimitLayer, TowerCorsLayer, TowerDecompressionLayer, TowerElapsed,
        TowerExponentialBackoffMaker, TowerFilterHandle, TowerFilterLayer, TowerHttpServiceHandle,
        TowerHttpTimeoutLayer, TowerLoadShedHandle, TowerLoadShedLayer, TowerMapErrHandle,
        TowerMapRequestHandle, TowerMapResponseHandle, TowerNormalizePathLayer, TowerOverloaded,
        TowerPropagateHeaderLayer, TowerRate, TowerRateLimitHandle, TowerRateLimitLayer,
        TowerRetryHandle, TowerRetryLayer, TowerServiceBuilderHandle, TowerServiceError,
        TowerSetRequestHeaderLayer, TowerSetResponseHeaderLayer,
        TowerSetSensitiveRequestHeadersLayer, TowerSetSensitiveResponseHeadersLayer,
        TowerSetStatusLayer, TowerSpawnReadyLayer, TowerThenHandle, TowerTimeoutHandle,
        TowerTimeoutLayer, TowerTpsBudget, TowerTraceLayer, TowerValidateRequestHeaderLayer,
    };

    // ── Core rate / limit ────────────────────────────────────────────────────

    impl_tower_survey_spec!(
        type    = TowerRate,
        name    = "TowerRate",
        summary = "Rate limit config: N requests per T-millisecond window.",
        fields  = [
            ("num",        "Number of requests allowed in the time window"),
            ("per_millis", "Duration of the time window in milliseconds"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerTimeoutLayer,
        name    = "TowerTimeoutLayer",
        summary = "Applies a per-request timeout; returns an error if exceeded.",
        fields  = [("timeout_millis", "Request timeout in milliseconds")]
    );

    impl_tower_survey_spec!(
        type    = TowerConcurrencyLimitLayer,
        name    = "TowerConcurrencyLimitLayer",
        summary = "Limits the number of concurrent in-flight requests.",
        fields  = [("max", "Maximum concurrent requests")]
    );

    impl_tower_survey_spec!(
        type    = TowerRateLimitLayer,
        name    = "TowerRateLimitLayer",
        summary = "Limits requests to N per T-millisecond window.",
        fields  = [
            ("num",        "Requests allowed per window"),
            ("per_millis", "Window duration in milliseconds"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerBufferLayer,
        name    = "TowerBufferLayer",
        summary = "Adds an async mpsc buffer in front of a service.",
        fields  = [("bound", "Channel capacity before backpressure applies")]
    );

    impl_tower_unit_spec!(
        type    = TowerLoadShedLayer,
        name    = "TowerLoadShedLayer",
        summary = "Drops requests when the inner service is not ready.",
    );

    impl_tower_unit_spec!(
        type    = TowerSpawnReadyLayer,
        name    = "TowerSpawnReadyLayer",
        summary = "Drives services to readiness on a background Tokio task.",
    );

    impl_tower_survey_spec!(
        type    = TowerFilterLayer,
        name    = "TowerFilterLayer",
        summary = "Filters requests using a registered predicate type.",
        fields  = [("predicate_name", "Name of the registered predicate type")]
    );

    impl_tower_survey_spec!(
        type    = TowerRetryLayer,
        name    = "TowerRetryLayer",
        summary = "Retries failed requests using a registered policy type.",
        fields  = [("policy_name", "Name of the registered retry policy type")]
    );

    // ── Backoff / budget ─────────────────────────────────────────────────────

    impl_tower_survey_spec!(
        type    = TowerExponentialBackoffMaker,
        name    = "TowerExponentialBackoffMaker",
        summary = "Exponential backoff config with jitter for retry strategies.",
        fields  = [
            ("min_millis", "Minimum backoff duration in milliseconds"),
            ("max_millis", "Maximum backoff duration in milliseconds"),
            ("jitter",     "Jitter factor in [0.0, 100.0]"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerTpsBudget,
        name    = "TowerTpsBudget",
        summary = "Token-bucket retry budget based on transactions per second.",
        fields  = [
            ("ttl_millis",     "Budget TTL window in milliseconds"),
            ("min_per_sec",    "Minimum retries allowed per second"),
            ("retry_percent",  "Ratio of retries to original requests [0.0..1.0]"),
        ]
    );

    // ── Service handles ──────────────────────────────────────────────────────

    impl_tower_survey_spec!(
        type    = TowerServiceBuilderHandle,
        name    = "TowerServiceBuilderHandle",
        summary = "UUID handle for a live ServiceBuilder stored in the plugin registry.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerBufferHandle,
        name    = "TowerBufferHandle",
        summary = "UUID handle for a live Buffer service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerRateLimitHandle,
        name    = "TowerRateLimitHandle",
        summary = "UUID handle for a live RateLimit service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerConcurrencyLimitHandle,
        name    = "TowerConcurrencyLimitHandle",
        summary = "UUID handle for a live ConcurrencyLimit service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerTimeoutHandle,
        name    = "TowerTimeoutHandle",
        summary = "UUID handle for a live Timeout service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerLoadShedHandle,
        name    = "TowerLoadShedHandle",
        summary = "UUID handle for a live LoadShed service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerRetryHandle,
        name    = "TowerRetryHandle",
        summary = "UUID handle for a live Retry service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerFilterHandle,
        name    = "TowerFilterHandle",
        summary = "UUID handle for a live Filter service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerBoxServiceHandle,
        name    = "TowerBoxServiceHandle",
        summary = "UUID handle for a live BoxService.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerMapRequestHandle,
        name    = "TowerMapRequestHandle",
        summary = "UUID handle for a live MapRequest service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerMapResponseHandle,
        name    = "TowerMapResponseHandle",
        summary = "UUID handle for a live MapResponse service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerMapErrHandle,
        name    = "TowerMapErrHandle",
        summary = "UUID handle for a live MapErr service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerThenHandle,
        name    = "TowerThenHandle",
        summary = "UUID handle for a live Then service.",
        fields  = [("id", "Registry UUID")]
    );

    impl_tower_survey_spec!(
        type    = TowerAndThenHandle,
        name    = "TowerAndThenHandle",
        summary = "UUID handle for a live AndThen service.",
        fields  = [("id", "Registry UUID")]
    );

    // ── Errors ───────────────────────────────────────────────────────────────

    impl_tower_unit_spec!(
        type    = TowerElapsed,
        name    = "TowerElapsed",
        summary = "Error indicating a request timed out.",
    );

    impl_tower_unit_spec!(
        type    = TowerOverloaded,
        name    = "TowerOverloaded",
        summary = "Error indicating the service was not ready (load-shed).",
    );

    impl_tower_unit_spec!(
        type    = TowerClosed,
        name    = "TowerClosed",
        summary = "Error indicating the buffer worker closed unexpectedly.",
    );

    impl_tower_survey_spec!(
        type    = TowerServiceError,
        name    = "TowerServiceError",
        summary = "Error produced by a buffered service failure.",
        fields  = [("message", "Error message from the underlying service")]
    );

    // ── tower-http layers ────────────────────────────────────────────────────

    impl_tower_survey_spec!(
        type    = TowerNormalizePathLayer,
        name    = "TowerNormalizePathLayer",
        summary = "Normalizes request paths by trimming or appending trailing slashes.",
        fields  = [("trim", "true = trim trailing slash; false = append trailing slash")]
    );

    impl_tower_survey_spec!(
        type    = TowerPropagateHeaderLayer,
        name    = "TowerPropagateHeaderLayer",
        summary = "Copies a named request header to the response.",
        fields  = [("header", "HTTP header name to propagate")]
    );

    impl_tower_survey_spec!(
        type    = TowerSetStatusLayer,
        name    = "TowerSetStatusLayer",
        summary = "Overrides the response status code for every request.",
        fields  = [("status_code", "HTTP status code (e.g. 200, 404)")]
    );

    impl_tower_survey_spec!(
        type    = TowerSetSensitiveRequestHeadersLayer,
        name    = "TowerSetSensitiveRequestHeadersLayer",
        summary = "Marks named request headers as sensitive so they are redacted from traces.",
        fields  = [("headers", "Header names to mark sensitive")]
    );

    impl_tower_survey_spec!(
        type    = TowerSetSensitiveResponseHeadersLayer,
        name    = "TowerSetSensitiveResponseHeadersLayer",
        summary = "Marks named response headers as sensitive so they are redacted from traces.",
        fields  = [("headers", "Header names to mark sensitive")]
    );

    impl_tower_unit_spec!(
        type    = TowerCatchPanicLayer,
        name    = "TowerCatchPanicLayer",
        summary = "Catches handler panics and converts them to 500 responses.",
    );

    impl_tower_survey_spec!(
        type    = TowerCompressionLayer,
        name    = "TowerCompressionLayer",
        summary = "Compresses response bodies using the negotiated algorithm.",
        fields  = [
            ("gzip",    "Enable gzip compression"),
            ("deflate", "Enable deflate compression"),
            ("br",      "Enable Brotli compression"),
            ("zstd",    "Enable Zstandard compression"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerDecompressionLayer,
        name    = "TowerDecompressionLayer",
        summary = "Decompresses response bodies using the negotiated algorithm.",
        fields  = [
            ("gzip",    "Enable gzip decompression"),
            ("deflate", "Enable deflate decompression"),
            ("br",      "Enable Brotli decompression"),
            ("zstd",    "Enable Zstandard decompression"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerHttpTimeoutLayer,
        name    = "TowerHttpTimeoutLayer",
        summary = "Applies a request timeout that returns HTTP 408 on expiry.",
        fields  = [("timeout_millis", "Timeout in milliseconds")]
    );

    impl_tower_unit_spec!(
        type    = TowerTraceLayer,
        name    = "TowerTraceLayer",
        summary = "Adds HTTP tracing spans using the tower-http default classifier.",
    );

    impl_tower_survey_spec!(
        type    = TowerCorsLayer,
        name    = "TowerCorsLayer",
        summary = "Handles CORS preflight and injects access-control response headers.",
        fields  = [
            ("allow_origins",      "Allowed origin values; use [\"*\"] for permissive"),
            ("allow_methods",      "Allowed HTTP methods"),
            ("allow_headers",      "Allowed request headers"),
            ("allow_credentials",  "Whether credentials are permitted"),
            ("max_age_secs",       "Preflight cache lifetime in seconds"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerValidateRequestHeaderLayer,
        name    = "TowerValidateRequestHeaderLayer",
        summary = "Rejects requests that do not carry the expected header value.",
        fields  = [
            ("header",         "Header name to validate"),
            ("expected_value", "Expected header value"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerSetRequestHeaderLayer,
        name    = "TowerSetRequestHeaderLayer",
        summary = "Inserts or overrides a static header on every request.",
        fields  = [
            ("header", "Header name"),
            ("value",  "Static header value"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerSetResponseHeaderLayer,
        name    = "TowerSetResponseHeaderLayer",
        summary = "Inserts or overrides a static header on every response.",
        fields  = [
            ("header", "Header name"),
            ("value",  "Static header value"),
        ]
    );

    impl_tower_survey_spec!(
        type    = TowerHttpServiceHandle,
        name    = "TowerHttpServiceHandle",
        summary = "UUID handle for a live tower-http layered service.",
        fields  = [("id", "Registry UUID")]
    );
}
