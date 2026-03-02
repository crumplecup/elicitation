//! [`ElicitSpec`](crate::ElicitSpec) implementations for `serde_json::Value` contract types.
//!
//! Available with the `serde_json` feature.

#[cfg(feature = "serde_json")]
mod value_impls {
    use crate::verification::types::{ValueArray, ValueNonNull, ValueObject};
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_value_spec {
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
                                .description("Wraps a serde_json::Value".to_string())
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

    impl_value_spec!(
        type     = ValueObject,
        name     = "ValueObject",
        summary  = "A serde_json::Value guaranteed to be a JSON object ({...}).",
        requires = [("is_object", "Value must be a JSON object variant.", "value.is_object()")],
    );

    impl_value_spec!(
        type     = ValueArray,
        name     = "ValueArray",
        summary  = "A serde_json::Value guaranteed to be a JSON array ([...]).",
        requires = [("is_array", "Value must be a JSON array variant.", "value.is_array()")],
    );

    impl_value_spec!(
        type     = ValueNonNull,
        name     = "ValueNonNull",
        summary  = "A serde_json::Value guaranteed to not be JSON null.",
        requires = [("non_null", "Value must not be the JSON null variant.", "!value.is_null()")],
    );
}
