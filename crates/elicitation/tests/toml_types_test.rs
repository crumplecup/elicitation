//! Tests for TOML third-party support in `elicitation`.

#![cfg(feature = "toml-types")]

use elicitation::{
    ElicitIntrospect, ElicitPromptTree, Elicitation, ElicitationPattern, PatternDetails,
    PromptTree, TomlDate, TomlDatetime, TomlDeError, TomlOffset, TomlSerError, TomlTime, TomlValue,
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

mod introspect {
    use super::*;

    #[test]
    fn toml_offset_is_select() {
        assert_eq!(TomlOffset::pattern(), ElicitationPattern::Select);
        let meta = TomlOffset::metadata();
        assert_eq!(meta.type_name, "toml_datetime::Offset");
        match &meta.details {
            PatternDetails::Select { variants } => {
                assert_eq!(variants.len(), 2);
                let labels: Vec<_> = variants.iter().map(|v| v.label.as_str()).collect();
                assert!(labels.contains(&"Z"));
                assert!(labels.contains(&"Custom"));
            }
            _ => panic!("expected Select metadata for TomlOffset"),
        }
    }

    #[test]
    fn toml_date_is_survey() {
        assert_eq!(TomlDate::pattern(), ElicitationPattern::Survey);
        let meta = TomlDate::metadata();
        assert_eq!(meta.type_name, "toml_datetime::Date");
        match &meta.details {
            PatternDetails::Survey { fields } => {
                let names: Vec<_> = fields.iter().map(|f| f.name).collect();
                assert!(names.contains(&"year"));
                assert!(names.contains(&"month"));
                assert!(names.contains(&"day"));
            }
            _ => panic!("expected Survey metadata for TomlDate"),
        }
    }

    #[test]
    fn toml_time_is_survey() {
        assert_eq!(TomlTime::pattern(), ElicitationPattern::Survey);
        let meta = TomlTime::metadata();
        assert_eq!(meta.type_name, "toml_datetime::Time");
        match &meta.details {
            PatternDetails::Survey { fields } => {
                let names: Vec<_> = fields.iter().map(|f| f.name).collect();
                assert!(names.contains(&"hour"));
                assert!(names.contains(&"minute"));
                assert!(names.contains(&"second"));
                assert!(names.contains(&"nanosecond"));
            }
            _ => panic!("expected Survey metadata for TomlTime"),
        }
    }

    #[test]
    fn toml_datetime_is_select() {
        assert_eq!(TomlDatetime::pattern(), ElicitationPattern::Select);
        let meta = TomlDatetime::metadata();
        assert_eq!(meta.type_name, "toml_datetime::Datetime");
    }

    #[test]
    fn toml_value_is_select() {
        assert_eq!(TomlValue::pattern(), ElicitationPattern::Select);
        let meta = TomlValue::metadata();
        assert_eq!(meta.type_name, "toml::Value");
        match &meta.details {
            PatternDetails::Select { variants } => {
                let labels: Vec<_> = variants.iter().map(|v| v.label.as_str()).collect();
                assert!(labels.contains(&"String"));
                assert!(labels.contains(&"Integer"));
                assert!(labels.contains(&"Float"));
                assert!(labels.contains(&"Boolean"));
                assert!(labels.contains(&"Datetime"));
                assert!(labels.contains(&"Array"));
                assert!(labels.contains(&"Table"));
            }
            _ => panic!("expected Select metadata for TomlValue"),
        }
    }

    #[test]
    fn toml_de_error_is_survey() {
        assert_eq!(TomlDeError::pattern(), ElicitationPattern::Survey);
        let meta = TomlDeError::metadata();
        assert_eq!(meta.type_name, "toml::de::Error");
    }

    #[test]
    fn toml_ser_error_is_survey() {
        assert_eq!(TomlSerError::pattern(), ElicitationPattern::Survey);
        let meta = TomlSerError::metadata();
        assert_eq!(meta.type_name, "toml::ser::Error");
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn toml_offset_prompt_tree_is_complete() {
        let tree = TomlOffset::prompt_tree();
        assert_prompts_complete(&tree, "TomlOffset");
    }

    #[test]
    fn toml_date_prompt_tree_is_complete() {
        let tree = TomlDate::prompt_tree();
        assert_prompts_complete(&tree, "TomlDate");
    }

    #[test]
    fn toml_time_prompt_tree_is_complete() {
        let tree = TomlTime::prompt_tree();
        assert_prompts_complete(&tree, "TomlTime");
    }

    #[test]
    fn toml_datetime_prompt_tree_is_complete() {
        let tree = TomlDatetime::prompt_tree();
        assert_prompts_complete(&tree, "TomlDatetime");
    }

    #[test]
    fn toml_value_prompt_tree_is_complete() {
        let tree = TomlValue::prompt_tree();
        assert_prompts_complete(&tree, "TomlValue");
    }
}

mod type_spec {
    use super::*;

    #[test]
    fn toml_offset_registered() {
        let spec = lookup_type_spec("toml_datetime::Offset");
        assert!(spec.is_some(), "TomlOffset not registered in type spec");
    }

    #[test]
    fn toml_date_registered() {
        let spec = lookup_type_spec("toml_datetime::Date");
        assert!(spec.is_some(), "TomlDate not registered in type spec");
    }

    #[test]
    fn toml_time_registered() {
        let spec = lookup_type_spec("toml_datetime::Time");
        assert!(spec.is_some(), "TomlTime not registered in type spec");
    }

    #[test]
    fn toml_datetime_registered() {
        let spec = lookup_type_spec("toml_datetime::Datetime");
        assert!(spec.is_some(), "TomlDatetime not registered in type spec");
    }

    #[test]
    fn toml_value_registered() {
        let spec = lookup_type_spec("toml::Value");
        assert!(spec.is_some(), "TomlValue not registered in type spec");
    }

    #[test]
    fn toml_de_error_registered() {
        let spec = lookup_type_spec("toml::de::Error");
        assert!(spec.is_some(), "TomlDeError not registered in type spec");
    }

    #[test]
    fn toml_ser_error_registered() {
        let spec = lookup_type_spec("toml::ser::Error");
        assert!(spec.is_some(), "TomlSerError not registered in type spec");
    }

    #[test]
    fn toml_offset_spec_describes_select() {
        assert_eq!(TomlOffset::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn toml_value_spec_describes_select() {
        assert_eq!(TomlValue::pattern(), ElicitationPattern::Select);
    }

    #[test]
    fn toml_date_spec_describes_survey() {
        assert_eq!(TomlDate::pattern(), ElicitationPattern::Survey);
    }
}

mod proofs {
    use super::*;

    #[test]
    fn toml_offset_proofs_non_empty() {
        assert_proofs_non_empty::<TomlOffset>("TomlOffset");
    }

    #[test]
    fn toml_date_proofs_non_empty() {
        assert_proofs_non_empty::<TomlDate>("TomlDate");
    }

    #[test]
    fn toml_time_proofs_non_empty() {
        assert_proofs_non_empty::<TomlTime>("TomlTime");
    }

    #[test]
    fn toml_datetime_proofs_non_empty() {
        assert_proofs_non_empty::<TomlDatetime>("TomlDatetime");
    }

    #[test]
    fn toml_value_proofs_non_empty() {
        assert_proofs_non_empty::<TomlValue>("TomlValue");
    }

    #[test]
    fn toml_de_error_proofs_non_empty() {
        assert_proofs_non_empty::<TomlDeError>("TomlDeError");
    }

    #[test]
    fn toml_ser_error_proofs_non_empty() {
        assert_proofs_non_empty::<TomlSerError>("TomlSerError");
    }
}

mod serde_roundtrip {
    use super::*;

    #[test]
    fn toml_offset_z_roundtrip() {
        let original = TomlOffset::Z;
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlOffset = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_offset_custom_roundtrip() {
        let original = TomlOffset::Custom {
            hours: -5,
            minutes: 30,
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlOffset = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_date_roundtrip() {
        let original = TomlDate {
            year: 2024,
            month: 6,
            day: 15,
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlDate = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_time_roundtrip() {
        let original = TomlTime {
            hour: 12,
            minute: 30,
            second: 45,
            nanosecond: 0,
        };
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlTime = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_value_string_roundtrip() {
        let original = TomlValue::String("hello world".to_string());
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlValue = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_value_integer_roundtrip() {
        let original = TomlValue::Integer(42);
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlValue = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }

    #[test]
    fn toml_value_boolean_roundtrip() {
        let original = TomlValue::Boolean(true);
        let json = serde_json::to_string(&original).unwrap();
        let decoded: TomlValue = serde_json::from_str(&json).unwrap();
        assert_eq!(original, decoded);
    }
}

mod upstream_conversion {
    use super::*;

    #[test]
    fn toml_offset_z_to_upstream() {
        let wrapper = TomlOffset::Z;
        let upstream: toml_datetime::Offset = wrapper.into();
        assert_eq!(upstream, toml_datetime::Offset::Z);
    }

    #[test]
    fn toml_offset_custom_to_upstream_roundtrip() {
        let wrapper = TomlOffset::Custom {
            hours: 5,
            minutes: 30,
        };
        let upstream: toml_datetime::Offset = wrapper.into();
        let back: TomlOffset = upstream.into();
        assert_eq!(wrapper, back);
    }

    #[test]
    fn toml_offset_negative_to_upstream_roundtrip() {
        let wrapper = TomlOffset::Custom {
            hours: -8,
            minutes: 0,
        };
        let upstream: toml_datetime::Offset = wrapper.into();
        let back: TomlOffset = upstream.into();
        assert_eq!(wrapper, back);
    }

    #[test]
    fn toml_date_to_upstream_roundtrip() {
        let wrapper = TomlDate {
            year: 2024,
            month: 1,
            day: 15,
        };
        let upstream: toml_datetime::Date = wrapper.into();
        let back: TomlDate = upstream.into();
        assert_eq!(wrapper, back);
    }

    #[test]
    fn toml_time_to_upstream_roundtrip() {
        let wrapper = TomlTime {
            hour: 9,
            minute: 15,
            second: 30,
            nanosecond: 500_000_000,
        };
        let upstream: toml_datetime::Time = wrapper.into();
        let back: TomlTime = upstream.into();
        assert_eq!(wrapper, back);
    }

    #[test]
    fn toml_value_string_to_upstream_roundtrip() {
        let wrapper = TomlValue::String("test".to_string());
        let upstream: toml::Value = wrapper.clone().into();
        let back: TomlValue = upstream.into();
        assert_eq!(wrapper, back);
    }

    #[test]
    fn toml_value_integer_to_upstream_roundtrip() {
        let wrapper = TomlValue::Integer(99);
        let upstream: toml::Value = wrapper.clone().into();
        let back: TomlValue = upstream.into();
        assert_eq!(wrapper, back);
    }
}
