//! [`ElicitSpec`](crate::ElicitSpec) implementations for bool contract types.

use crate::verification::types::{BoolFalse, BoolTrue};
use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

impl ElicitSpec for BoolTrue {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("must_be_true".to_string())
                    .description("Value must be true.".to_string())
                    .expression(Some("value == true".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("BoolTrue".to_string())
            .summary("A boolean that must be true.".to_string())
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "BoolTrue",
    BoolTrue::type_spec,
    std::any::TypeId::of::<BoolTrue>
));

impl ElicitSpec for BoolFalse {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("must_be_false".to_string())
                    .description("Value must be false.".to_string())
                    .expression(Some("value == false".to_string()))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("BoolFalse".to_string())
            .summary("A boolean that must be false.".to_string())
            .categories(vec![requires])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "BoolFalse",
    BoolFalse::type_spec,
    std::any::TypeId::of::<BoolFalse>
));
