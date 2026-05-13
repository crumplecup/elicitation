//! Tests for georaster third-party support in `elicitation`.

#![cfg(feature = "georaster-types")]

use elicitation::{
    ElicitIntrospect, ElicitPromptTree, ElicitSpec, Elicitation, ElicitationPattern,
    PatternDetails, PromptTree, lookup_type_spec,
};
use georaster::{
    Coordinate,
    geotiff::{ImageInfo, RasterValue},
};
use tiff::{
    ColorType,
    tags::{PhotometricInterpretation, PlanarConfiguration},
};

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
    fn coordinate_and_image_info_are_surveys() {
        assert_eq!(Coordinate::pattern(), ElicitationPattern::Survey);
        assert_eq!(ImageInfo::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn raster_and_tiff_enums_are_selects() {
        for (name, pattern) in [
            ("RasterValue", RasterValue::pattern()),
            ("ColorType", ColorType::pattern()),
            (
                "PhotometricInterpretation",
                PhotometricInterpretation::pattern(),
            ),
            ("PlanarConfiguration", PlanarConfiguration::pattern()),
        ] {
            assert_eq!(
                pattern,
                ElicitationPattern::Select,
                "{name} should be Select"
            );
        }
    }

    #[test]
    fn image_info_metadata_lists_all_fields() {
        let meta = ImageInfo::metadata();
        assert_eq!(meta.type_name, "georaster::geotiff::ImageInfo");
        match &meta.details {
            PatternDetails::Survey { fields } => assert_eq!(fields.len(), 5),
            _ => panic!("expected Survey metadata"),
        }
    }
}

mod specs {
    use super::*;

    #[test]
    fn georaster_specs_are_registered() {
        for name in [
            "georaster::Coordinate",
            "georaster::geotiff::RasterValue",
            "georaster::geotiff::ImageInfo",
            "tiff::ColorType",
            "tiff::tags::PhotometricInterpretation",
            "tiff::tags::PlanarConfiguration",
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
    fn raster_value_spec_describes_variants() {
        let spec = <RasterValue as ElicitSpec>::type_spec();
        let variants = spec
            .categories()
            .iter()
            .find(|category| category.name() == "variants")
            .expect("RasterValue should have variants category");
        assert_eq!(variants.entries().len(), 15);
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn raster_value_prompt_tree_is_complete() {
        let tree = RasterValue::prompt_tree();
        assert_eq!(tree.type_name(), "georaster::geotiff::RasterValue");
        assert_prompts_complete(&tree, "georaster::geotiff::RasterValue");
    }

    #[test]
    fn image_info_prompt_tree_is_complete() {
        let tree = ImageInfo::prompt_tree();
        assert_eq!(tree.type_name(), "georaster::geotiff::ImageInfo");
        assert_prompts_complete(&tree, "georaster::geotiff::ImageInfo");
    }
}

mod proofs {
    use super::*;

    #[test]
    fn proofs_are_non_empty() {
        assert_proofs_non_empty::<Coordinate>("georaster::Coordinate");
        assert_proofs_non_empty::<RasterValue>("georaster::geotiff::RasterValue");
        assert_proofs_non_empty::<ImageInfo>("georaster::geotiff::ImageInfo");
        assert_proofs_non_empty::<ColorType>("tiff::ColorType");
        assert_proofs_non_empty::<PhotometricInterpretation>(
            "tiff::tags::PhotometricInterpretation",
        );
        assert_proofs_non_empty::<PlanarConfiguration>("tiff::tags::PlanarConfiguration");
    }
}
