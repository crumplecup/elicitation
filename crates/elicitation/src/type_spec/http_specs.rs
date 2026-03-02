//! [`ElicitSpec`](crate::ElicitSpec) implementations for HTTP contract types.
//!
//! Available with the `reqwest` feature.

use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── reqwest ───────────────────────────────────────────────────────────────────

#[cfg(feature = "reqwest")]
mod reqwest_specs {
    use super::*;
    use crate::verification::types::StatusCodeValid;

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
                    .expression(Some("value >= 100 && value <= 999".to_string()))
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
        <StatusCodeValid as ElicitSpec>::type_spec
    ));
}
