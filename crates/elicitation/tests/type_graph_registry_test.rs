//! Tests for the TypeGraphKey structural registry.

use elicitation::{
    Elicit, ElicitIntrospect, PatternDetails, Prompt, Select, TypeGraphKey,
    all_graphable_types, lookup_type_graph,
};

// --- Test types ---

#[derive(Debug, Clone, Elicit)]
pub struct SimpleStruct {
    pub name: String,
    pub count: u32,
}

#[derive(Debug, Clone, Elicit)]
pub enum SimpleEnum {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Debug, Clone, Elicit)]
pub enum MixedEnum {
    Unit,
    WithData(String),
}

// --- Registry lookup ---

#[test]
fn lookup_registered_struct() {
    let meta = lookup_type_graph("SimpleStruct");
    assert!(meta.is_some(), "SimpleStruct should be registered");
    let meta = meta.unwrap();
    assert_eq!(meta.type_name, "SimpleStruct");
    matches!(meta.details, PatternDetails::Survey { .. });
}

#[test]
fn lookup_registered_enum() {
    let meta = lookup_type_graph("SimpleEnum");
    assert!(meta.is_some(), "SimpleEnum should be registered");
    let meta = meta.unwrap();
    assert_eq!(meta.type_name, "SimpleEnum");
    matches!(meta.details, PatternDetails::Select { .. });
}

#[test]
fn lookup_unknown_returns_none() {
    assert!(lookup_type_graph("NonExistentType").is_none());
}

#[test]
fn all_graphable_types_includes_registered() {
    let names = all_graphable_types();
    assert!(
        names.contains(&"SimpleStruct"),
        "SimpleStruct should appear in all_graphable_types"
    );
    assert!(
        names.contains(&"SimpleEnum"),
        "SimpleEnum should appear in all_graphable_types"
    );
}

#[test]
fn all_graphable_types_sorted() {
    let names = all_graphable_types();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    assert_eq!(
        names, sorted,
        "all_graphable_types() should return sorted names"
    );
}

// --- Struct field metadata ---

#[test]
fn struct_metadata_has_correct_fields() {
    let meta = lookup_type_graph("SimpleStruct").unwrap();
    match meta.details {
        PatternDetails::Survey { fields } => {
            assert_eq!(fields.len(), 2);
            assert!(
                fields.iter().any(|f| f.name == "name"),
                "should have 'name' field"
            );
            assert!(
                fields.iter().any(|f| f.name == "count"),
                "should have 'count' field"
            );
        }
        other => panic!("Expected Survey, got {:?}", other),
    }
}

// --- Enum variant metadata ---

#[test]
fn enum_metadata_has_correct_variants() {
    let meta = lookup_type_graph("SimpleEnum").unwrap();
    match meta.details {
        PatternDetails::Select { variants } => {
            assert_eq!(variants.len(), 3);
            let labels: Vec<&str> = variants.iter().map(|v| v.label.as_str()).collect();
            assert!(labels.contains(&"Alpha"));
            assert!(labels.contains(&"Beta"));
            assert!(labels.contains(&"Gamma"));
            // Unit variants have no fields
            for v in &variants {
                assert!(v.fields.is_empty(), "unit variants should have no fields");
            }
        }
        other => panic!("Expected Select, got {:?}", other),
    }
}

#[test]
fn mixed_enum_data_variant_has_fields() {
    let meta = lookup_type_graph("MixedEnum").unwrap();
    match meta.details {
        PatternDetails::Select { variants } => {
            let with_data = variants.iter().find(|v| v.label == "WithData");
            assert!(
                with_data.is_some(),
                "MixedEnum should have WithData variant"
            );
            let with_data = with_data.unwrap();
            assert_eq!(with_data.fields.len(), 1, "WithData should have 1 field");
        }
        other => panic!("Expected Select, got {:?}", other),
    }
}

// --- TypeGraphKey direct API ---

#[test]
fn type_graph_key_build_returns_metadata() {
    let key = TypeGraphKey::new("TestKey", <SimpleStruct as ElicitIntrospect>::metadata);
    assert_eq!(key.type_name(), "TestKey");
    let meta = key.build();
    assert_eq!(meta.type_name, "SimpleStruct");
}

#[test]
fn mixed_enum_with_data_variant_is_constructible() {
    let variant = MixedEnum::WithData("hello".to_string());
    if let MixedEnum::WithData(inner) = variant {
        assert!(!inner.is_empty());
    }
}
