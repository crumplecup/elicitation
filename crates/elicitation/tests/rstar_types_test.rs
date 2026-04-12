//! Tests for `rstar` third-party support in `elicitation`.

#![cfg(feature = "rstar-types")]

use elicitation::{
    ElicitComplete, ElicitIntrospect, ElicitPromptTree, ElicitSpec, Elicitation,
    ElicitationPattern, PatternDetails, PromptTree, RstarAabb, RstarLine, RstarRectangle,
    lookup_type_spec,
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

fn assert_elicit_complete<T: ElicitComplete>() {}

mod introspect {
    use super::*;

    #[test]
    fn rstar_primitives_are_surveys() {
        assert_eq!(RstarAabb::pattern(), ElicitationPattern::Survey);
        assert_eq!(RstarRectangle::pattern(), ElicitationPattern::Survey);
        assert_eq!(RstarLine::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn line_metadata_lists_both_endpoints() {
        let meta = RstarLine::metadata();
        assert_eq!(meta.type_name, "rstar::primitives::Line<[f64; 2]>");
        match &meta.details {
            PatternDetails::Survey { fields } => assert_eq!(fields.len(), 2),
            _ => panic!("expected Survey metadata"),
        }
    }
}

mod specs {
    use super::*;

    #[test]
    fn rstar_specs_are_registered() {
        for name in [
            "rstar::AABB<[f64; 2]>",
            "rstar::primitives::Rectangle<[f64; 2]>",
            "rstar::primitives::Line<[f64; 2]>",
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
    fn rectangle_spec_describes_fields() {
        let spec = <RstarRectangle as ElicitSpec>::type_spec();
        let fields = spec
            .categories()
            .iter()
            .find(|category| category.name() == "fields")
            .expect("Rectangle should have fields category");
        assert_eq!(fields.entries().len(), 2);
    }

    #[test]
    fn rstar_wrappers_are_elicit_complete() {
        assert_elicit_complete::<RstarAabb>();
        assert_elicit_complete::<RstarRectangle>();
        assert_elicit_complete::<RstarLine>();
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn aabb_prompt_tree_is_complete() {
        let tree = RstarAabb::prompt_tree();
        assert_eq!(tree.type_name(), "RstarAabb");
        assert_prompts_complete(&tree, "RstarAabb");
    }

    #[test]
    fn line_prompt_tree_is_complete() {
        let tree = RstarLine::prompt_tree();
        assert_eq!(tree.type_name(), "RstarLine");
        assert_prompts_complete(&tree, "RstarLine");
    }
}

mod proofs {
    use super::*;

    #[test]
    fn proofs_are_non_empty() {
        assert_proofs_non_empty::<RstarAabb>("rstar::AABB<[f64; 2]>");
        assert_proofs_non_empty::<RstarRectangle>("rstar::primitives::Rectangle<[f64; 2]>");
        assert_proofs_non_empty::<RstarLine>("rstar::primitives::Line<[f64; 2]>");
    }
}
