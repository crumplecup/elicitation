//! Tests for `proj` third-party support in `elicitation`.

#![cfg(feature = "proj-types")]

use elicitation::{
    ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitSpec, Elicitation,
    ElicitationPattern, PatternDetails, ProjArea, PromptTree, lookup_type_spec,
};
use proj::Area;

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

fn assert_elicit_complete<T: ElicitComplete>() {}

mod conversions {
    use super::*;

    #[test]
    fn proj_area_roundtrips_to_upstream_type() {
        let wrapper = ProjArea::new(-180.0, -90.0, 180.0, 90.0);
        let upstream = wrapper.into_inner();
        assert_eq!(upstream.west, -180.0);
        assert_eq!(upstream.south, -90.0);
        assert_eq!(upstream.east, 180.0);
        assert_eq!(upstream.north, 90.0);

        let wrapped_again = ProjArea::from(Area::new(-10.0, -20.0, 30.0, 40.0));
        assert_eq!(wrapped_again.west, -10.0);
        assert_eq!(wrapped_again.south, -20.0);
        assert_eq!(wrapped_again.east, 30.0);
        assert_eq!(wrapped_again.north, 40.0);
    }
}

mod introspect {
    use super::*;

    #[test]
    fn proj_area_is_a_survey() {
        assert_eq!(ProjArea::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn proj_area_metadata_lists_all_bounds() {
        let meta = ProjArea::metadata();
        assert_eq!(meta.type_name, "proj::Area");
        match &meta.details {
            PatternDetails::Survey { fields } => assert_eq!(fields.len(), 4),
            _ => panic!("expected Survey metadata"),
        }
    }
}

mod specs {
    use super::*;

    #[test]
    fn proj_area_spec_is_registered() {
        let spec = lookup_type_spec("proj::Area").expect("proj::Area should be registered");
        assert_eq!(spec.type_name(), "proj::Area");
        assert!(
            !spec.summary().is_empty(),
            "proj::Area summary should not be empty"
        );
    }

    #[test]
    fn proj_area_spec_describes_four_fields() {
        let spec = <ProjArea as ElicitSpec>::type_spec();
        let fields = spec
            .categories()
            .iter()
            .find(|category| category.name() == "fields")
            .expect("ProjArea should have fields category");
        assert_eq!(fields.entries().len(), 4);
    }

    #[test]
    fn proj_area_is_elicit_complete() {
        assert_elicit_complete::<ProjArea>();
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn proj_area_prompt_tree_is_complete() {
        let tree = ProjArea::prompt_tree();
        assert_eq!(tree.type_name(), "ProjArea");
        assert_prompts_complete(&tree, "ProjArea");
    }
}

mod proofs {
    use super::*;

    #[test]
    fn proofs_are_non_empty() {
        assert_proofs_non_empty::<ProjArea>("proj::Area");
    }
}
