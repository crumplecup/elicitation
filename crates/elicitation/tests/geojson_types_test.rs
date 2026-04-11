//! Tests for GeoJSON third-party support in `elicitation`.

#![cfg(feature = "geojson-types")]

use elicitation::{
    ElicitIntrospect, ElicitPromptTree, ElicitSpec, Elicitation, ElicitationPattern,
    PatternDetails, PromptTree, lookup_type_spec,
};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

fn assert_prompts_complete(tree: &PromptTree, path: &str) {
    match tree {
        PromptTree::Leaf { prompt, .. } | PromptTree::Affirm { prompt, .. } => {
            assert!(!prompt.is_empty(), "{path}: empty prompt");
        }
        PromptTree::Select {
            prompt,
            options,
            branches,
            ..
        } => {
            assert!(!prompt.is_empty(), "{path}: empty select prompt");
            assert!(!options.is_empty(), "{path}: no options");
            for (label, branch) in options.iter().zip(branches.iter()) {
                if let Some(sub) = branch {
                    assert_prompts_complete(sub, &format!("{path}/{label}"));
                }
            }
        }
        PromptTree::Survey { fields, .. } => {
            for (field_name, sub) in fields {
                assert_prompts_complete(sub, &format!("{path}.{field_name}"));
            }
        }
    }
}

#[track_caller]
fn assert_proofs_non_empty<T: Elicitation>(label: &str) {
    assert!(!T::kani_proof().is_empty(), "{label}: empty kani proof");
    assert!(!T::verus_proof().is_empty(), "{label}: empty verus proof");
    assert!(
        !T::creusot_proof().is_empty(),
        "{label}: empty creusot proof"
    );
}

mod introspect {
    use super::*;

    #[test]
    fn geojson_document_is_select() {
        assert_eq!(GeoJson::pattern(), ElicitationPattern::Select);
        let meta = GeoJson::metadata();
        assert_eq!(meta.type_name, "geojson::GeoJson");
        match &meta.details {
            PatternDetails::Select { variants } => {
                assert_eq!(variants.len(), 3);
            }
            _ => panic!("expected Select metadata"),
        }
    }

    #[test]
    fn geojson_value_uses_actual_upstream_name() {
        assert_eq!(Value::pattern(), ElicitationPattern::Select);
        let meta = Value::metadata();
        assert_eq!(meta.type_name, "geojson::Value");
        match &meta.details {
            PatternDetails::Select { variants } => {
                let labels: Vec<_> = variants
                    .iter()
                    .map(|variant| variant.label.as_str())
                    .collect();
                assert_eq!(
                    labels,
                    vec![
                        "Point",
                        "MultiPoint",
                        "LineString",
                        "MultiLineString",
                        "Polygon",
                        "MultiPolygon",
                        "GeometryCollection",
                    ]
                );
            }
            _ => panic!("expected Select metadata"),
        }
    }

    #[test]
    fn feature_and_collection_are_surveys() {
        assert_eq!(Feature::pattern(), ElicitationPattern::Survey);
        assert_eq!(FeatureCollection::pattern(), ElicitationPattern::Survey);
    }
}

mod specs {
    use super::*;

    #[test]
    fn geojson_specs_are_registered() {
        for name in [
            "geojson::GeoJson",
            "geojson::Value",
            "geojson::Geometry",
            "geojson::Feature",
            "geojson::FeatureCollection",
            "geojson::feature::Id",
        ] {
            let spec =
                lookup_type_spec(name).unwrap_or_else(|| panic!("{name} should be registered"));
            assert_eq!(spec.type_name(), name);
            assert!(
                !spec.summary().is_empty(),
                "{name} summary should not be empty"
            );
        }
    }

    #[test]
    fn geometry_spec_describes_fields() {
        let spec = <Geometry as ElicitSpec>::type_spec();
        let fields = spec
            .categories()
            .iter()
            .find(|category| category.name() == "fields")
            .expect("geometry should have fields category");
        assert_eq!(fields.entries().len(), 3);
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn geometry_prompt_tree_is_complete() {
        let tree = Geometry::prompt_tree();
        assert_eq!(tree.type_name(), "geojson::Geometry");
        assert_prompts_complete(&tree, "geojson::Geometry");
    }

    #[test]
    fn feature_prompt_tree_is_complete() {
        let tree = Feature::prompt_tree();
        assert_eq!(tree.type_name(), "geojson::Feature");
        assert_prompts_complete(&tree, "geojson::Feature");
    }
}

mod proofs {
    use super::*;

    #[test]
    fn proofs_are_non_empty() {
        assert_proofs_non_empty::<GeoJson>("geojson::GeoJson");
        assert_proofs_non_empty::<Value>("geojson::Value");
        assert_proofs_non_empty::<Geometry>("geojson::Geometry");
        assert_proofs_non_empty::<Feature>("geojson::Feature");
        assert_proofs_non_empty::<FeatureCollection>("geojson::FeatureCollection");
        assert_proofs_non_empty::<geojson::feature::Id>("geojson::feature::Id");
    }
}
