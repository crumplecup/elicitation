//! [`ElicitSpec`](crate::ElicitSpec) implementations for string contract types.

use crate::verification::types::StringNonEmpty;
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

impl ElicitSpec for StringNonEmpty {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("non_empty".to_string())
                    .description("String must not be empty.".to_string())
                    .expression(Some("!value.is_empty()".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("max_len".to_string())
                    .description(
                        "String byte length must not exceed MAX_LEN (default 4096).".to_string(),
                    )
                    .expression(Some("value.len() <= MAX_LEN".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        let bounds = SpecCategoryBuilder::default()
            .name("bounds".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("utf8".to_string())
                    .description(
                        "Content must be valid UTF-8 (guaranteed for stdlib String).".to_string(),
                    )
                    .expression(None)
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("StringNonEmpty".to_string())
            .summary(
                "A non-empty UTF-8 string with a configurable maximum byte length (default 4096)."
                    .to_string(),
            )
            .categories(vec![requires, bounds])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "StringNonEmpty",
    StringNonEmpty::type_spec,
    std::any::TypeId::of::<StringNonEmpty>
));
