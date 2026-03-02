//! [`ElicitSpec`](crate::ElicitSpec) implementations for URL contract types.
//!
//! Available with the `url` feature.

#[cfg(feature = "url")]
mod url_impls {
    use crate::verification::types::{UrlCanBeBase, UrlHttp, UrlHttps, UrlValid, UrlWithHost};
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_url_spec {
        (
            type     = $ty:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            requires = [($req_label:literal, $req_desc:literal, $req_expr:literal)] $(,)?
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let requires = SpecCategoryBuilder::default()
                        .name("requires".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label($req_label.to_string())
                                .description($req_desc.to_string())
                                .expression(Some($req_expr.to_string()))
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let related = SpecCategoryBuilder::default()
                        .name("related".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("base_type".to_string())
                                .description("Wraps a url::Url parsed from a &str".to_string())
                                .expression(None)
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![requires, related])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec
            ));
        };
    }

    impl_url_spec!(
        type     = UrlValid,
        name     = "UrlValid",
        summary  = "A url::Url guaranteed to be syntactically valid (parseable by the url crate).",
        requires = [("valid", "Input must be a non-empty string parseable by url::Url::parse.", "!value.is_empty()")],
    );

    impl_url_spec!(
        type     = UrlHttps,
        name     = "UrlHttps",
        summary  = "A url::Url guaranteed to use the https:// scheme.",
        requires = [("https_scheme", "URL scheme must be 'https'.", "!value.is_empty()")],
    );

    impl_url_spec!(
        type     = UrlHttp,
        name     = "UrlHttp",
        summary  = "A url::Url guaranteed to use the http:// scheme.",
        requires = [("http_scheme", "URL scheme must be 'http'.", "!value.is_empty()")],
    );

    impl_url_spec!(
        type     = UrlWithHost,
        name     = "UrlWithHost",
        summary  = "A url::Url guaranteed to have a non-empty host component.",
        requires = [("has_host", "URL must have a host component (domain or IP).", "!value.is_empty()")],
    );

    impl_url_spec!(
        type     = UrlCanBeBase,
        name     = "UrlCanBeBase",
        summary  = "A url::Url guaranteed to be capable of being a base URL (not opaque).",
        requires = [("can_be_base", "URL must be capable of being a base URL (scheme://authority/path form).", "!value.is_empty()")],
    );
}
