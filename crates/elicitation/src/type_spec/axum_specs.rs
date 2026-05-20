//! [`ElicitSpec`](crate::ElicitSpec) impls for axum descriptor types.
//!
//! Available with the `axum-types` feature.

#[cfg(feature = "axum-types")]
mod axum_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_axum_enum_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [ $( ($label:literal, $desc:literal) ),+ $(,)? ]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("axum 0.8 — ergonomic async web framework".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Select — choose one variant from the list".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    macro_rules! impl_axum_survey_spec {
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
                                .description("axum 0.8 — ergonomic async web framework".to_string())
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
        };
    }

    use crate::{
        AxumDbSlot, AxumExtractorEntry, AxumExtractorKind, AxumHandlerDescriptor, AxumHttpMethod,
        AxumResponseDescriptor, AxumResponseKind, AxumRouteEntry, AxumRouterDescriptor,
        AxumServeDescriptor,
    };

    impl_axum_enum_spec!(
        type    = AxumHttpMethod,
        name    = "AxumHttpMethod",
        summary = "HTTP method for an axum route",
        variants = [
            ("Get",     "HTTP GET — retrieve a resource"),
            ("Post",    "HTTP POST — submit data, create a resource"),
            ("Put",     "HTTP PUT — replace a resource"),
            ("Delete",  "HTTP DELETE — remove a resource"),
            ("Patch",   "HTTP PATCH — partially update a resource"),
            ("Head",    "HTTP HEAD — like GET but response body is omitted"),
            ("Options", "HTTP OPTIONS — describe communication options"),
            ("Trace",   "HTTP TRACE — message loop-back test"),
            ("Any",     "Match any HTTP method (axum::routing::any)"),
        ]
    );

    impl_axum_enum_spec!(
        type    = AxumExtractorKind,
        name    = "AxumExtractorKind",
        summary = "Axum extractor kind for a handler argument",
        variants = [
            ("Path",        "Path<T> — extract typed path parameters"),
            ("Query",       "Query<T> — extract typed query string parameters"),
            ("Json",        "Json<T> — extract and deserialize a JSON body"),
            ("State",       "State<T> — access shared application state"),
            ("Extension",   "Extension<T> — access request extensions"),
            ("Form",        "Form<T> — extract URL-encoded form data"),
            ("Headers",     "HeaderMap — access all request headers"),
            ("RawBody",     "Bytes — access the raw request body"),
            ("RawQuery",    "RawQuery — access the raw query string"),
            ("OriginalUri", "OriginalUri — original request URI before any rewriting"),
            ("MatchedPath", "MatchedPath — the matched route pattern"),
            ("ConnectInfo", "ConnectInfo<T> — remote connection address info"),
        ]
    );

    impl_axum_enum_spec!(
        type    = AxumResponseKind,
        name    = "AxumResponseKind",
        summary = "Axum response kind",
        variants = [
            ("Json",      "Json<T> — serialize value as JSON response"),
            ("Html",      "Html<T> — return HTML content"),
            ("Redirect",  "Redirect — redirect to a URI"),
            ("NoContent", "StatusCode::NO_CONTENT — empty 204 response"),
            ("Status",    "Status-only response with optional body"),
            ("Custom",    "Custom / arbitrary response expression"),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumRouteEntry,
        name    = "AxumRouteEntry",
        summary = "A single route registration in an axum Router",
        fields  = [
            ("method",  "HTTP method for this route"),
            ("path",    "URL path pattern, e.g. \"/users/:id\""),
            ("handler", "Handler function name or expression, e.g. \"get_user\""),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumRouterDescriptor,
        name    = "AxumRouterDescriptor",
        summary = "Descriptor for a Router<S> configuration",
        fields  = [
            ("state_type", "Rust type name of the router state, e.g. \"AppState\""),
            ("routes",     "Routes registered on this router"),
            ("layers",     "Layer expressions applied in order"),
            ("fallback",   "Optional fallback handler expression"),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumExtractorEntry,
        name    = "AxumExtractorEntry",
        summary = "A single extractor argument in a handler signature",
        fields  = [
            ("var_name",  "Rust variable name, e.g. \"payload\""),
            ("kind",      "Extractor kind"),
            ("type_name", "Inner Rust type name, e.g. \"CreateUserRequest\""),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumHandlerDescriptor,
        name    = "AxumHandlerDescriptor",
        summary = "Descriptor for an async axum handler function",
        fields  = [
            ("name",        "Handler function name, e.g. \"create_user\""),
            ("extractors",  "Extractor parameters in order"),
            ("return_type", "Return type expression, e.g. \"impl IntoResponse\""),
            ("body",        "Optional body expression; None emits a todo!() stub"),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumResponseDescriptor,
        name    = "AxumResponseDescriptor",
        summary = "Descriptor for an axum response",
        fields  = [
            ("kind",        "Response kind"),
            ("status_code", "HTTP status code"),
            ("body_expr",   "Body expression; optional"),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumServeDescriptor,
        name    = "AxumServeDescriptor",
        summary = "Descriptor for an axum serve configuration",
        fields  = [
            ("addr",               "Bind address, e.g. \"0.0.0.0:3000\""),
            ("router_id",          "UUID of the router descriptor this server wraps"),
            ("graceful_shutdown",  "Optional graceful shutdown signal expression"),
        ]
    );

    impl_axum_survey_spec!(
        type    = AxumDbSlot,
        name    = "elicitation::AxumDbSlot",
        summary = "Db pool / state slot injected into an axum router via .with_state().",
        fields  = [
            ("pool_type",              "String — Rust type expression for the pool or state struct"),
            ("var_name",               "String — variable name used in generated code"),
            ("provide_leptos_context", "bool — emit leptos_routes_with_context and provide_context"),
        ]
    );

    // ElicitComplete for all 10 axum ReadyNow types
    impl crate::ElicitComplete for crate::AxumHttpMethod {}
    impl crate::ElicitComplete for crate::AxumExtractorKind {}
    impl crate::ElicitComplete for crate::AxumResponseKind {}
    impl crate::ElicitComplete for crate::AxumRouteEntry {}
    impl crate::ElicitComplete for crate::AxumDbSlot {}
    impl crate::ElicitComplete for crate::AxumRouterDescriptor {}
    impl crate::ElicitComplete for crate::AxumExtractorEntry {}
    impl crate::ElicitComplete for crate::AxumHandlerDescriptor {}
    impl crate::ElicitComplete for crate::AxumResponseDescriptor {}
    impl crate::ElicitComplete for crate::AxumServeDescriptor {}
}
