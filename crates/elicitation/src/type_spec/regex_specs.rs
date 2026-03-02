//! [`ElicitSpec`](crate::ElicitSpec) implementations for regex contract types.
//!
//! Available with the `regex` feature.

#[cfg(feature = "regex")]
mod regex_impls {
    use crate::verification::types::{RegexCaseInsensitive, RegexMultiline, RegexValid};
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_regex_spec {
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
                                .description(
                                    "Wraps a regex::Regex compiled from a pattern &str".to_string(),
                                )
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

    impl_regex_spec!(
        type     = RegexValid,
        name     = "RegexValid",
        summary  = "A regex::Regex guaranteed to be a valid, compilable pattern.",
        requires = [("valid_pattern", "Pattern must be non-empty and compilable by the regex crate.", "!pattern.is_empty()")],
    );

    impl_regex_spec!(
        type     = RegexCaseInsensitive,
        name     = "RegexCaseInsensitive",
        summary  = "A regex::Regex compiled with case-insensitive matching enabled.",
        requires = [("valid_pattern", "Pattern must be non-empty and compilable with case-insensitive flag.", "!pattern.is_empty()")],
    );

    impl_regex_spec!(
        type     = RegexMultiline,
        name     = "RegexMultiline",
        summary  = "A regex::Regex compiled with multi-line mode enabled (^ and $ match line boundaries).",
        requires = [("valid_pattern", "Pattern must be non-empty and compilable with multi-line flag.", "!pattern.is_empty()")],
    );
}
