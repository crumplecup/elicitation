//! Tests for the type_spec layer.

use elicitation::{
    ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
    TypeSpecInventoryKey, lookup_type_spec,
};

// ── Builder round-trips ───────────────────────────────────────────────────────

#[test]
fn spec_entry_builder_minimal() {
    let entry = SpecEntryBuilder::default()
        .label("positive".to_string())
        .description("value must be greater than zero".to_string())
        .build()
        .expect("valid entry");

    assert_eq!(entry.label(), "positive");
    assert_eq!(entry.description(), "value must be greater than zero");
    assert_eq!(entry.expression(), &None);
}

#[test]
fn spec_entry_builder_with_expression() {
    let entry = SpecEntryBuilder::default()
        .label("non_empty".to_string())
        .description("string must not be empty".to_string())
        .expression(Some("!value.is_empty()".to_string()))
        .build()
        .expect("valid entry");

    assert_eq!(entry.expression(), &Some("!value.is_empty()".to_string()));
}

#[test]
fn spec_category_builder_with_entries() {
    let entry = SpecEntryBuilder::default()
        .label("positive".to_string())
        .description("value must be > 0".to_string())
        .build()
        .expect("valid entry");

    let category = SpecCategoryBuilder::default()
        .name("requires".to_string())
        .entries(vec![entry])
        .build()
        .expect("valid category");

    assert_eq!(category.name(), "requires");
    assert_eq!(category.entries().len(), 1);
    assert_eq!(category.entries()[0].label(), "positive");
}

#[test]
fn spec_category_builder_default_entries() {
    let category = SpecCategoryBuilder::default()
        .name("ensures".to_string())
        .build()
        .expect("valid category");

    assert_eq!(category.entries().len(), 0);
}

#[test]
fn type_spec_builder_full() {
    let entry = SpecEntryBuilder::default()
        .label("positive".to_string())
        .description("value must be > 0".to_string())
        .expression(Some("value > 0".to_string()))
        .build()
        .expect("valid entry");

    let requires = SpecCategoryBuilder::default()
        .name("requires".to_string())
        .entries(vec![entry])
        .build()
        .expect("valid category");

    let spec = TypeSpecBuilder::default()
        .type_name("I32Positive".to_string())
        .summary("A positive 32-bit integer (value > 0)".to_string())
        .categories(vec![requires])
        .build()
        .expect("valid spec");

    assert_eq!(spec.type_name(), "I32Positive");
    assert_eq!(spec.summary(), "A positive 32-bit integer (value > 0)");
    assert_eq!(spec.categories().len(), 1);
}

#[test]
fn type_spec_builder_default_categories() {
    let spec = TypeSpecBuilder::default()
        .type_name("Opaque".to_string())
        .summary("An opaque handle".to_string())
        .build()
        .expect("valid spec");

    assert_eq!(spec.categories().len(), 0);
}

// ── ElicitSpec trait and inventory registry ──────────────────────────────────

struct TestType;

impl ElicitSpec for TestType {
    fn type_spec() -> TypeSpec {
        let entry = SpecEntryBuilder::default()
            .label("always".to_string())
            .description("always valid".to_string())
            .build()
            .expect("valid entry");

        let cat = SpecCategoryBuilder::default()
            .name("requires".to_string())
            .entries(vec![entry])
            .build()
            .expect("valid category");

        TypeSpecBuilder::default()
            .type_name("TestType".to_string())
            .summary("A test-only type".to_string())
            .categories(vec![cat])
            .build()
            .expect("valid spec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "TestType",
    TestType::type_spec,
    std::any::TypeId::of::<TestType>
));

#[test]
fn lookup_registered_type() {
    let spec = lookup_type_spec("TestType").expect("TestType should be registered");
    assert_eq!(spec.type_name(), "TestType");
    assert_eq!(spec.summary(), "A test-only type");
    assert_eq!(spec.categories().len(), 1);
    assert_eq!(spec.categories()[0].name(), "requires");
}

#[test]
fn lookup_unknown_type_returns_none() {
    assert!(lookup_type_spec("NoSuchType").is_none());
}

#[test]
fn inventory_key_type_name_and_build() {
    let key = TypeSpecInventoryKey::new(
        "Direct",
        || {
            TypeSpecBuilder::default()
                .type_name("Direct".to_string())
                .summary("direct build".to_string())
                .build()
                .expect("valid spec")
        },
        std::any::TypeId::of::<u8>,
    ); // u8 as a placeholder type_id for this test-only key

    assert_eq!(key.type_name(), "Direct");
    let spec = key.build();
    assert_eq!(spec.type_name(), "Direct");
}

// ── Integer specs ────────────────────────────────────────────────────────────

#[test]
fn integer_specs_registered_in_inventory() {
    for name in [
        "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ] {
        let spec = lookup_type_spec(name).unwrap_or_else(|| panic!("{name} should be registered"));
        assert_eq!(spec.type_name(), name);
        assert!(
            !spec.summary().is_empty(),
            "{name} summary should not be empty"
        );
        assert_eq!(
            spec.categories().len(),
            1,
            "{name} should have exactly one category"
        );
        assert_eq!(spec.categories()[0].name(), "bounds");
        assert_eq!(
            spec.categories()[0].entries().len(),
            2,
            "{name} bounds should have min+max"
        );
    }
}

#[test]
fn i32_spec_bounds_values() {
    let spec = lookup_type_spec("i32").expect("i32 registered");
    let bounds = &spec.categories()[0];
    let min = bounds
        .entries()
        .iter()
        .find(|e| e.label() == "min")
        .expect("min entry");
    let max = bounds
        .entries()
        .iter()
        .find(|e| e.label() == "max")
        .expect("max entry");
    assert!(min.description().contains("-2147483648"));
    assert!(max.description().contains("2147483647"));
}

#[test]
fn u8_spec_bounds_values() {
    let spec = lookup_type_spec("u8").expect("u8 registered");
    let bounds = &spec.categories()[0];
    let min = bounds
        .entries()
        .iter()
        .find(|e| e.label() == "min")
        .expect("min");
    let max = bounds
        .entries()
        .iter()
        .find(|e| e.label() == "max")
        .expect("max");
    assert!(min.description().contains('0'));
    assert!(max.description().contains("255"));
}

#[test]
fn integer_specs_have_expressions() {
    let spec = lookup_type_spec("i64").expect("i64 registered");
    for entry in spec.categories()[0].entries() {
        assert!(
            entry.expression().is_some(),
            "bounds entries should have expressions"
        );
    }
}

// ── Scalar specs (f32, f64, bool, char) ──────────────────────────────────────

#[test]
fn scalar_specs_registered() {
    for name in ["f32", "f64", "bool", "char"] {
        let spec = lookup_type_spec(name).unwrap_or_else(|| panic!("{name} should be registered"));
        assert_eq!(spec.type_name(), name);
        assert!(!spec.summary().is_empty());
        assert!(!spec.categories().is_empty());
    }
}

#[test]
fn f32_has_bounds_and_special_values() {
    let spec = lookup_type_spec("f32").expect("f32");
    let names: Vec<&str> = spec
        .categories()
        .iter()
        .map(|c| c.name().as_str())
        .collect();
    assert!(names.contains(&"bounds"), "f32 should have bounds category");
    assert!(
        names.contains(&"special_values"),
        "f32 should have special_values category"
    );
}

#[test]
fn bool_has_values_category() {
    let spec = lookup_type_spec("bool").expect("bool");
    assert_eq!(spec.categories()[0].name(), "values");
    assert_eq!(spec.categories()[0].entries().len(), 2);
}

#[test]
fn char_has_requires_and_bounds() {
    let spec = lookup_type_spec("char").expect("char");
    let names: Vec<&str> = spec
        .categories()
        .iter()
        .map(|c| c.name().as_str())
        .collect();
    assert!(names.contains(&"requires"));
    assert!(names.contains(&"bounds"));
}

// ── String, collections, Option, Result specs ────────────────────────────────

#[test]
fn string_spec_registered() {
    let spec = lookup_type_spec("String").expect("String registered");
    assert!(
        spec.categories().iter().any(|c| c.name() == "ensures"),
        "String should have ensures category"
    );
    assert!(
        spec.categories().iter().any(|c| c.name() == "fields"),
        "String should have fields category"
    );
}

#[test]
fn collection_specs_registered() {
    for name in ["Vec<T>", "HashMap<K, V>", "HashSet<T>"] {
        let spec = lookup_type_spec(name).unwrap_or_else(|| panic!("{name} should be registered"));
        assert!(
            !spec.categories().is_empty(),
            "{name} should have at least one category"
        );
    }
}

#[test]
fn option_result_specs_registered() {
    let opt = lookup_type_spec("Option<T>").expect("Option<T>");
    assert_eq!(opt.categories()[0].name(), "values");
    assert_eq!(opt.categories()[0].entries().len(), 2);

    let res = lookup_type_spec("Result<T, E>").expect("Result<T, E>");
    assert_eq!(res.categories()[0].name(), "values");
    let labels: Vec<&str> = res.categories()[0]
        .entries()
        .iter()
        .map(|e| e.label().as_str())
        .collect();
    assert!(labels.contains(&"ok") && labels.contains(&"err"));
}

// ── Integer contract type specs ──────────────────────────────────────────────

#[test]
fn integer_contract_specs_registered() {
    let expected = [
        ("I8Positive", "i8"),
        ("I8NonNegative", "i8"),
        ("I8NonZero", "i8"),
        ("I16Positive", "i16"),
        ("I16NonNegative", "i16"),
        ("I16NonZero", "i16"),
        ("U8NonZero", "u8"),
        ("U8Positive", "u8"),
        ("U16NonZero", "u16"),
        ("U16Positive", "u16"),
    ];

    for (name, base) in expected {
        let spec = lookup_type_spec(name).unwrap_or_else(|| panic!("{name} should be registered"));
        assert_eq!(spec.type_name(), name, "{name} type_name mismatch");
        assert!(!spec.summary().is_empty(), "{name} summary empty");

        let cat_names: Vec<&str> = spec
            .categories()
            .iter()
            .map(|c| c.name().as_str())
            .collect();
        assert!(
            cat_names.contains(&"requires"),
            "{name} missing requires category"
        );
        assert!(
            cat_names.contains(&"related"),
            "{name} missing related category"
        );

        let related = spec
            .categories()
            .iter()
            .find(|c| c.name() == "related")
            .unwrap();
        let base_entry = related
            .entries()
            .iter()
            .find(|e| e.label() == "base_type")
            .unwrap_or_else(|| panic!("{name} missing base_type entry"));
        assert!(
            base_entry.description().contains(base),
            "{name} base_type should mention {base}"
        );
    }
}

#[test]
fn i8_positive_requires_expression() {
    let spec = lookup_type_spec("I8Positive").expect("I8Positive");
    let requires = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    let entry = &requires.entries()[0];
    assert_eq!(entry.label(), "positive");
    assert_eq!(entry.expression().as_deref(), Some("value > 0"));
}

// ── Integer contract specs (extended families) ────────────────────────────────

#[test]
fn i32_positive_registered() {
    let spec = lookup_type_spec("I32Positive").expect("I32Positive");
    assert_eq!(spec.type_name(), "I32Positive");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries()[0].expression().as_deref(), Some("value > 0"));
}

#[test]
fn i64_non_zero_registered() {
    let spec = lookup_type_spec("I64NonZero").expect("I64NonZero");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries()[0].expression().as_deref(), Some("value != 0"));
}

#[test]
fn u64_non_zero_registered() {
    let spec = lookup_type_spec("U64NonZero").expect("U64NonZero");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries()[0].expression().as_deref(), Some("value != 0"));
}

#[test]
fn usize_positive_registered() {
    let spec = lookup_type_spec("UsizePositive").expect("UsizePositive");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries()[0].expression().as_deref(), Some("value > 0"));
}

// ── Float contract specs ──────────────────────────────────────────────────────

#[test]
fn f32_positive_registered() {
    let spec = lookup_type_spec("F32Positive").expect("F32Positive");
    assert_eq!(spec.type_name(), "F32Positive");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries().len(), 2);
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value.is_finite()")
    );
    assert_eq!(
        req.entries()[1].expression().as_deref(),
        Some("value > 0.0")
    );
}

#[test]
fn f64_finite_registered() {
    let spec = lookup_type_spec("F64Finite").expect("F64Finite");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries().len(), 1);
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value.is_finite()")
    );
}

// ── Bool contract specs ───────────────────────────────────────────────────────

#[test]
fn bool_true_registered() {
    let spec = lookup_type_spec("BoolTrue").expect("BoolTrue");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value == true")
    );
}

#[test]
fn bool_false_registered() {
    let spec = lookup_type_spec("BoolFalse").expect("BoolFalse");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value == false")
    );
}

// ── Char contract specs ───────────────────────────────────────────────────────

#[test]
fn char_alphabetic_registered() {
    let spec = lookup_type_spec("CharAlphabetic").expect("CharAlphabetic");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value.is_alphabetic()")
    );
}

#[test]
fn char_alphanumeric_registered() {
    let spec = lookup_type_spec("CharAlphanumeric").expect("CharAlphanumeric");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value.is_alphanumeric()")
    );
}

// ── String contract specs ─────────────────────────────────────────────────────

#[test]
fn string_non_empty_registered() {
    let spec = lookup_type_spec("StringNonEmpty").expect("StringNonEmpty");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(req.entries().len(), 2);
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("!value.is_empty()")
    );
}

// ── Collection contract specs ─────────────────────────────────────────────────

#[test]
fn vec_non_empty_registered() {
    let spec = lookup_type_spec("VecNonEmpty").expect("VecNonEmpty");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("!vec.is_empty()")
    );
}

#[test]
fn hashmap_non_empty_registered() {
    let spec = lookup_type_spec("HashMapNonEmpty").expect("HashMapNonEmpty");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("!map.is_empty()")
    );
}

#[test]
fn option_some_contract_registered() {
    let spec = lookup_type_spec("OptionSome").expect("OptionSome");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("opt.is_some()")
    );
}

#[test]
fn result_ok_contract_registered() {
    let spec = lookup_type_spec("ResultOk").expect("ResultOk");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("result.is_ok()")
    );
}

// ── Std extras (Duration, PathBuf) ────────────────────────────────────────────

#[test]
fn duration_positive_registered() {
    let spec = lookup_type_spec("DurationPositive").expect("DurationPositive");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("duration.as_nanos() > 0")
    );
}

#[test]
fn pathbuf_exists_registered() {
    let spec = lookup_type_spec("PathBufExists").expect("PathBufExists");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("path.exists()")
    );
}

#[test]
fn pathbuf_is_dir_registered() {
    let spec = lookup_type_spec("PathBufIsDir").expect("PathBufIsDir");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("path.is_dir()")
    );
}

// ── Network contract specs ────────────────────────────────────────────────────

#[test]
fn ip_v4_registered() {
    let spec = lookup_type_spec("IpV4").expect("IpV4");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("ip.is_ipv4()")
    );
}

#[test]
fn ip_v6_registered() {
    let spec = lookup_type_spec("IpV6").expect("IpV6");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("ip.is_ipv6()")
    );
}

#[test]
fn ipv4_loopback_registered() {
    let spec = lookup_type_spec("Ipv4Loopback").expect("Ipv4Loopback");
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("ip.is_loopback()")
    );
}

#[test]
fn ip_private_registered() {
    let spec = lookup_type_spec("IpPrivate").expect("IpPrivate");
    assert!(!spec.summary().is_empty());
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert!(!req.entries().is_empty());
}

// ── time ──────────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "time")]
fn offset_datetime_after_registered() {
    let spec = lookup_type_spec("OffsetDateTimeAfter").expect("OffsetDateTimeAfter");
    assert!(spec.summary().contains("after"));
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value > threshold")
    );
}

#[test]
#[cfg(feature = "time")]
fn offset_datetime_before_registered() {
    let spec = lookup_type_spec("OffsetDateTimeBefore").expect("OffsetDateTimeBefore");
    assert!(spec.summary().contains("before"));
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value < threshold")
    );
}

// ── reqwest ───────────────────────────────────────────────────────────────────

#[test]
#[cfg(feature = "reqwest")]
fn status_code_valid_registered() {
    let spec = lookup_type_spec("StatusCodeValid").expect("StatusCodeValid");
    assert!(spec.summary().contains("100"));
    let req = spec
        .categories()
        .iter()
        .find(|c| c.name() == "requires")
        .unwrap();
    assert_eq!(
        req.entries()[0].expression().as_deref(),
        Some("value >= 100 && value <= 999")
    );
}
