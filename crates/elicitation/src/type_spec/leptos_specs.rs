//! [`ElicitSpec`](crate::ElicitSpec) impls for leptos descriptor types.
//!
//! Available with the `leptos-types` feature.

#[cfg(feature = "leptos-types")]
mod leptos_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_leptos_enum_spec {
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
                                .description("leptos 0.7 — full-stack reactive web framework".to_string())
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

    macro_rules! impl_leptos_survey_spec {
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
                                .description("leptos 0.7 — full-stack reactive web framework".to_string())
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
        LeptosAppDescriptor, LeptosAxumDescriptor, LeptosAxumMode, LeptosClientMode,
        LeptosComponentDescriptor, LeptosCustomRouteDescriptor, LeptosDisplayMode, LeptosHtmlTag,
        LeptosMode, LeptosPropDescriptor, LeptosResponseHeaderDescriptor, LeptosRouteDescriptor,
        LeptosViewNode,
    };

    impl_leptos_enum_spec!(
        type    = LeptosMode,
        name    = "LeptosMode",
        summary = "Top-level Leptos rendering mode",
        variants = [
            ("Csr",     "Client-Side Rendering — full app runs in the browser"),
            ("Ssr",     "Server-Side Rendering — HTML rendered on the server"),
            ("Hydrate", "SSR + client hydration — server HTML, then client takeover"),
            ("Islands", "Islands architecture — selective hydration of interactive components"),
        ]
    );

    impl_leptos_enum_spec!(
        type    = LeptosHtmlTag,
        name    = "LeptosHtmlTag",
        summary = "HTML element tag for a Leptos view node",
        variants = [
            ("div",      "Block-level container element"),
            ("span",     "Inline container element"),
            ("p",        "Paragraph element"),
            ("a",        "Anchor / hyperlink element"),
            ("button",   "Clickable button element"),
            ("input",    "Form input element"),
            ("form",     "Form element"),
            ("h1",       "Heading level 1"),
            ("h2",       "Heading level 2"),
            ("h3",       "Heading level 3"),
            ("ul",       "Unordered (bulleted) list"),
            ("ol",       "Ordered (numbered) list"),
            ("li",       "List item"),
            ("img",      "Image element"),
            ("nav",      "Navigation landmark"),
            ("main",     "Main content landmark"),
            ("section",  "Thematic section element"),
            ("article",  "Self-contained article element"),
            ("header",   "Page or section header"),
            ("footer",   "Page or section footer"),
            ("aside",    "Sidebar / complementary content"),
            ("table",    "Table element"),
            ("tr",       "Table row"),
            ("td",       "Table data cell"),
            ("th",       "Table header cell"),
            ("select",   "Drop-down select element"),
            ("option",   "Option within a select element"),
            ("textarea", "Multi-line text input"),
            ("label",    "Label for a form control"),
            ("strong",   "Strong importance (bold)"),
            ("em",       "Emphasis (italic)"),
            ("code",     "Inline code element"),
            ("pre",      "Preformatted text block"),
        ]
    );

    impl_leptos_enum_spec!(
        type    = LeptosClientMode,
        name    = "LeptosClientMode",
        summary = "Client-side rendering mode for leptos-axum",
        variants = [
            ("csr",    "Client-Side Rendering — JS bundle drives the entire app"),
            ("hydrate","Hydration — reuse server HTML and attach event handlers"),
        ]
    );

    impl_leptos_enum_spec!(
        type    = LeptosAxumMode,
        name    = "LeptosAxumMode",
        summary = "SSR strategy used by the leptos-axum integration",
        variants = [
            ("static_html", "Pre-rendered static HTML with no server function support"),
            ("full_ssr",    "Full SSR with server functions via axum handlers"),
            ("wasm_shell",  "WASM shell — serve a minimal HTML + hydrate on the client"),
        ]
    );

    impl_leptos_enum_spec!(
        type    = LeptosDisplayMode,
        name    = "LeptosDisplayMode",
        summary = "Display / layout mode for a Leptos application",
        variants = [
            ("bare",      "Bare — no outer shell, component renders directly"),
            ("standard",  "Standard — wrap with a default app shell"),
            ("dashboard", "Dashboard — wrap with a dashboard layout shell"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosCustomRouteDescriptor,
        name    = "LeptosCustomRouteDescriptor",
        summary = "A custom axum route added to a leptos-axum router",
        fields  = [
            ("method",  "HTTP method, e.g. \"GET\" or \"POST\""),
            ("path",    "URL path pattern, e.g. \"/api/health\""),
            ("handler", "Handler function name or expression"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosResponseHeaderDescriptor,
        name    = "LeptosResponseHeaderDescriptor",
        summary = "A response header to inject into leptos SSR responses",
        fields  = [
            ("name",  "Header name, e.g. \"Cache-Control\""),
            ("value", "Header value, e.g. \"no-store\""),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosAxumDescriptor,
        name    = "LeptosAxumDescriptor",
        summary = "Top-level descriptor for a leptos-axum application",
        fields  = [
            ("app_component", "Name of the root Leptos app component, e.g. \"App\""),
            ("mode",          "SSR strategy (LeptosAxumMode)"),
            ("site_addr",     "Bind address for the axum server, e.g. \"0.0.0.0:3000\""),
            ("client_mode",   "Client-side rendering mode (LeptosClientMode)"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosPropDescriptor,
        name    = "LeptosPropDescriptor",
        summary = "A single prop on a Leptos component",
        fields  = [
            ("name",          "Prop name, e.g. \"label\""),
            ("ty",            "Rust type expression, e.g. \"String\""),
            ("optional",      "Whether the prop is optional (#[prop(optional)])"),
            ("default_value", "Default value expression; None means required"),
            ("into",          "Whether #[prop(into)] coercion is applied"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosComponentDescriptor,
        name    = "LeptosComponentDescriptor",
        summary = "Descriptor for a Leptos component function",
        fields  = [
            ("name",         "Component name in PascalCase, e.g. \"MyButton\""),
            ("props",        "List of props (LeptosPropDescriptor)"),
            ("has_children", "Whether the component accepts children"),
            ("island",       "Whether this is a #[island] component"),
            ("body",         "Component body — view! macro body or raw code"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosViewNode,
        name    = "LeptosViewNode",
        summary = "A node in a Leptos view tree (element, text, or reactive expression)",
        fields  = [
            ("tag",           "Element tag name, e.g. \"div\", or \"text\" for text nodes"),
            ("text",          "Static text content (for text nodes)"),
            ("reactive_expr", "Reactive expression, e.g. `{move || count.get()}`"),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosRouteDescriptor,
        name    = "LeptosRouteDescriptor",
        summary = "A route registered in a Leptos <Router>",
        fields  = [
            ("path", "URL path pattern, e.g. \"/users/:id\""),
            ("view", "Component name rendered at this route, e.g. \"UserPage\""),
        ]
    );

    impl_leptos_survey_spec!(
        type    = LeptosAppDescriptor,
        name    = "LeptosAppDescriptor",
        summary = "Top-level descriptor for a Leptos application",
        fields  = [
            ("package_name", "Cargo package name, e.g. \"my-leptos-app\""),
            ("mode",         "Top-level rendering mode (LeptosMode)"),
            ("components",   "Component descriptors included in the app"),
            ("routes",       "Route descriptors registered in the Router"),
        ]
    );

    // ElicitComplete for all 13 leptos ReadyNow types
    impl crate::ElicitComplete for crate::LeptosMode {}
    impl crate::ElicitComplete for crate::LeptosHtmlTag {}
    impl crate::ElicitComplete for crate::LeptosClientMode {}
    impl crate::ElicitComplete for crate::LeptosAxumMode {}
    impl crate::ElicitComplete for crate::LeptosDisplayMode {}
    impl crate::ElicitComplete for crate::LeptosCustomRouteDescriptor {}
    impl crate::ElicitComplete for crate::LeptosResponseHeaderDescriptor {}
    impl crate::ElicitComplete for crate::LeptosAxumDescriptor {}
    impl crate::ElicitComplete for crate::LeptosPropDescriptor {}
    impl crate::ElicitComplete for crate::LeptosComponentDescriptor {}
    impl crate::ElicitComplete for crate::LeptosViewNode {}
    impl crate::ElicitComplete for crate::LeptosRouteDescriptor {}
    impl crate::ElicitComplete for crate::LeptosAppDescriptor {}
}
