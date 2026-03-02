//! [`ElicitSpec`](crate::ElicitSpec) implementations for UUID contract types.
//!
//! Available with the `uuid` feature.

#[cfg(feature = "uuid")]
mod uuid_impls {
    use crate::verification::types::{UuidNonNil, UuidV4};
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_uuid_spec {
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
                                .description("Wraps a uuid::Uuid".to_string())
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

    impl_uuid_spec!(
        type     = UuidV4,
        name     = "UuidV4",
        summary  = "A uuid::Uuid guaranteed to be version 4 (random).",
        requires = [("version_4", "UUID must be version 4 (randomly generated).", "uuid.get_version_num() == 4")],
    );

    impl_uuid_spec!(
        type     = UuidNonNil,
        name     = "UuidNonNil",
        summary  = "A uuid::Uuid guaranteed to not be the nil UUID (all zeros).",
        requires = [("non_nil", "UUID must not be the nil UUID (00000000-0000-0000-0000-000000000000).", "uuid != Uuid::nil()")],
    );
}
