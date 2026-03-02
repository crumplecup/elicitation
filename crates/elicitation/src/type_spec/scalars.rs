//! [`ElicitSpec`](crate::ElicitSpec) implementations for float, bool, and char primitives.

use crate::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey,
};

// ── f32 ──────────────────────────────────────────────────────────────────────

impl ElicitSpec for f32 {
    fn type_spec() -> TypeSpec {
        let bounds = SpecCategoryBuilder::default()
            .name("bounds".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("min_positive".to_string())
                    .description("Smallest positive value: 1.1754944e-38".to_string())
                    .expression(Some("f32::MIN_POSITIVE == 1.1754944e-38".to_string()))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("max".to_string())
                    .description("Maximum finite value: 3.4028235e38".to_string())
                    .expression(Some("f32::MAX == 3.4028235e38".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid bounds");

        let special = SpecCategoryBuilder::default()
            .name("special_values".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("nan".to_string())
                    .description("Not-a-Number: f32::NAN (not equal to itself)".to_string())
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("infinity".to_string())
                    .description("Positive infinity: f32::INFINITY".to_string())
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("neg_infinity".to_string())
                    .description("Negative infinity: f32::NEG_INFINITY".to_string())
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid special_values");

        TypeSpecBuilder::default()
            .type_name("f32".to_string())
            .summary(
                "A 32-bit IEEE 754 floating-point number. Supports NaN, ±∞, and subnormal values."
                    .to_string(),
            )
            .categories(vec![bounds, special])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "f32",
    <f32 as ElicitSpec>::type_spec,
    std::any::TypeId::of::<f32>
));

// ── f64 ──────────────────────────────────────────────────────────────────────

impl ElicitSpec for f64 {
    fn type_spec() -> TypeSpec {
        let bounds = SpecCategoryBuilder::default()
            .name("bounds".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("min_positive".to_string())
                    .description("Smallest positive value: 2.2250738585072014e-308".to_string())
                    .expression(Some(
                        "f64::MIN_POSITIVE == 2.2250738585072014e-308".to_string(),
                    ))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("max".to_string())
                    .description("Maximum finite value: 1.7976931348623157e308".to_string())
                    .expression(Some("f64::MAX == 1.7976931348623157e308".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid bounds");

        let special = SpecCategoryBuilder::default()
            .name("special_values".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("nan".to_string())
                    .description("Not-a-Number: f64::NAN (not equal to itself)".to_string())
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("infinity".to_string())
                    .description("Positive infinity: f64::INFINITY".to_string())
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("neg_infinity".to_string())
                    .description("Negative infinity: f64::NEG_INFINITY".to_string())
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid special_values");

        TypeSpecBuilder::default()
            .type_name("f64".to_string())
            .summary(
                "A 64-bit IEEE 754 floating-point number. Supports NaN, ±∞, and subnormal values."
                    .to_string(),
            )
            .categories(vec![bounds, special])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "f64",
    <f64 as ElicitSpec>::type_spec,
    std::any::TypeId::of::<f64>
));

// ── bool ─────────────────────────────────────────────────────────────────────

impl ElicitSpec for bool {
    fn type_spec() -> TypeSpec {
        let values = SpecCategoryBuilder::default()
            .name("values".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("true".to_string())
                    .description(
                        "Logical true. Accepted inputs: \"true\", \"yes\", \"1\", \"y\"."
                            .to_string(),
                    )
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("false".to_string())
                    .description(
                        "Logical false. Accepted inputs: \"false\", \"no\", \"0\", \"n\"."
                            .to_string(),
                    )
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid values");

        TypeSpecBuilder::default()
            .type_name("bool".to_string())
            .summary("A boolean value: true or false.".to_string())
            .categories(vec![values])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "bool",
    <bool as ElicitSpec>::type_spec,
    std::any::TypeId::of::<bool>
));

// ── char ─────────────────────────────────────────────────────────────────────

impl ElicitSpec for char {
    fn type_spec() -> TypeSpec {
        let requires = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("unicode_scalar".to_string())
                    .description("Must be a valid Unicode scalar value (U+0000 to U+D7FF or U+E000 to U+10FFFF). Lone surrogates (U+D800–U+DFFF) are not valid chars.".to_string())
                    .expression(Some("value != surrogate_range (U+D800..=U+DFFF)".to_string()))
                    .build().expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("single_codepoint".to_string())
                    .description("Must be exactly one Unicode codepoint, not a multi-character string.".to_string())
                    .build().expect("valid entry"),
            ])
            .build().expect("valid requires");

        let bounds = SpecCategoryBuilder::default()
            .name("bounds".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("min".to_string())
                    .description("Minimum: U+0000 (null character)".to_string())
                    .expression(Some("char::MIN == '\\0'".to_string()))
                    .build()
                    .expect("valid entry"),
                SpecEntryBuilder::default()
                    .label("max".to_string())
                    .description("Maximum: U+10FFFF".to_string())
                    .expression(Some("char::MAX == '\\u{10FFFF}'".to_string()))
                    .build()
                    .expect("valid entry"),
            ])
            .build()
            .expect("valid bounds");

        TypeSpecBuilder::default()
            .type_name("char".to_string())
            .summary("A Unicode scalar value (a single character codepoint, 4 bytes).".to_string())
            .categories(vec![requires, bounds])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "char",
    <char as ElicitSpec>::type_spec,
    std::any::TypeId::of::<char>
));
