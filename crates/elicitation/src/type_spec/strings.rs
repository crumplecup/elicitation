//! [`ElicitSpec`](crate::ElicitSpec) implementations for string types.

use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── String ───────────────────────────────────────────────────────────────────

impl ElicitSpec for String {
    fn type_spec() -> TypeSpec {
        let ensures = SpecCategoryBuilder::default()
            .name("ensures".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("valid_utf8".to_string())
                    .description(
                        "Always valid UTF-8. Rust's String type guarantees valid UTF-8 encoding."
                            .to_string(),
                    )
                    .expression(Some(
                        "std::str::from_utf8(output.as_bytes()).is_ok()".to_string(),
                    ))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid ensures");

        let fields = SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("len".to_string())
                    .description("Length in bytes (not characters). Use .chars().count() for Unicode character count.".to_string())
                    .build().expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("capacity".to_string())
                    .description("Allocated buffer capacity in bytes. Grows automatically on push/append.".to_string())
                    .build().expect("valid entry"),
            ])
            .build().expect("valid fields");

        TypeSpecBuilder::default()
            .type_name("String".to_string())
            .summary("A heap-allocated, growable, UTF-8 encoded string.".to_string())
            .categories(vec![ensures, fields])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "String",
    <String as ElicitSpec>::type_spec,
    std::any::TypeId::of::<String>
));
