//! Tests for csv third-party support in `elicitation`.

#![cfg(feature = "csv-types")]

use elicitation::{
    CsvByteRecord, CsvErrorKind, CsvPosition, CsvQuoteStyle, CsvStringRecord, CsvTerminator,
    CsvTrim, ElicitIntrospect, ElicitPromptTree, Elicitation, ElicitationPattern, PatternDetails,
    PromptTree, lookup_type_spec,
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
    fn csv_quote_style_is_select() {
        assert_eq!(CsvQuoteStyle::pattern(), ElicitationPattern::Select);
        let meta = CsvQuoteStyle::metadata();
        assert_eq!(meta.type_name, "csv::QuoteStyle");
        match &meta.details {
            PatternDetails::Select { variants } => {
                assert_eq!(variants.len(), 4);
                let labels: Vec<_> = variants.iter().map(|v| v.label.as_str()).collect();
                assert!(labels.contains(&"Always"));
                assert!(labels.contains(&"Necessary"));
                assert!(labels.contains(&"NonNumeric"));
                assert!(labels.contains(&"Never"));
            }
            _ => panic!("expected Select metadata for CsvQuoteStyle"),
        }
    }

    #[test]
    fn csv_trim_is_select() {
        assert_eq!(CsvTrim::pattern(), ElicitationPattern::Select);
        let meta = CsvTrim::metadata();
        assert_eq!(meta.type_name, "csv::Trim");
        match &meta.details {
            PatternDetails::Select { variants } => {
                assert_eq!(variants.len(), 4);
            }
            _ => panic!("expected Select metadata for CsvTrim"),
        }
    }

    #[test]
    fn csv_terminator_is_select_with_data_variant() {
        assert_eq!(CsvTerminator::pattern(), ElicitationPattern::Select);
        let meta = CsvTerminator::metadata();
        assert_eq!(meta.type_name, "csv::Terminator");
        match &meta.details {
            PatternDetails::Select { variants } => {
                assert_eq!(variants.len(), 3);
                let any_byte = variants.iter().find(|v| v.label == "AnyByte").unwrap();
                assert_eq!(any_byte.fields.len(), 1);
            }
            _ => panic!("expected Select metadata for CsvTerminator"),
        }
    }

    #[test]
    fn csv_position_is_survey() {
        assert_eq!(CsvPosition::pattern(), ElicitationPattern::Survey);
        let meta = CsvPosition::metadata();
        assert_eq!(meta.type_name, "csv::Position");
        match &meta.details {
            PatternDetails::Survey { fields } => {
                assert_eq!(fields.len(), 3);
                let names: Vec<_> = fields.iter().map(|f| f.name).collect();
                assert!(names.contains(&"byte"));
                assert!(names.contains(&"line"));
                assert!(names.contains(&"record"));
            }
            _ => panic!("expected Survey metadata for CsvPosition"),
        }
    }

    #[test]
    fn csv_string_record_is_survey() {
        assert_eq!(CsvStringRecord::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn csv_byte_record_is_survey() {
        assert_eq!(CsvByteRecord::pattern(), ElicitationPattern::Survey);
    }

    #[test]
    fn csv_error_kind_is_select() {
        assert_eq!(CsvErrorKind::pattern(), ElicitationPattern::Select);
    }
}

mod prompt_tree {
    use super::*;

    #[test]
    fn csv_quote_style_prompt_tree_complete() {
        let tree = CsvQuoteStyle::prompt_tree();
        assert_prompts_complete(&tree, "CsvQuoteStyle");
    }

    #[test]
    fn csv_trim_prompt_tree_complete() {
        let tree = CsvTrim::prompt_tree();
        assert_prompts_complete(&tree, "CsvTrim");
    }

    #[test]
    fn csv_terminator_prompt_tree_complete() {
        let tree = CsvTerminator::prompt_tree();
        assert_prompts_complete(&tree, "CsvTerminator");
    }

    #[test]
    fn csv_position_prompt_tree_complete() {
        let tree = CsvPosition::prompt_tree();
        assert_prompts_complete(&tree, "CsvPosition");
    }

    #[test]
    fn csv_string_record_prompt_tree_complete() {
        let tree = CsvStringRecord::prompt_tree();
        assert_prompts_complete(&tree, "CsvStringRecord");
    }

    #[test]
    fn csv_byte_record_prompt_tree_complete() {
        let tree = CsvByteRecord::prompt_tree();
        assert_prompts_complete(&tree, "CsvByteRecord");
    }

    #[test]
    fn csv_error_kind_prompt_tree_complete() {
        let tree = CsvErrorKind::prompt_tree();
        assert_prompts_complete(&tree, "CsvErrorKind");
    }
}

mod proofs {
    use super::*;

    #[test]
    fn csv_quote_style_proofs_non_empty() {
        assert_proofs_non_empty::<CsvQuoteStyle>("CsvQuoteStyle");
    }

    #[test]
    fn csv_trim_proofs_non_empty() {
        assert_proofs_non_empty::<CsvTrim>("CsvTrim");
    }

    #[test]
    fn csv_terminator_proofs_non_empty() {
        assert_proofs_non_empty::<CsvTerminator>("CsvTerminator");
    }

    #[test]
    fn csv_position_proofs_non_empty() {
        assert_proofs_non_empty::<CsvPosition>("CsvPosition");
    }

    #[test]
    fn csv_string_record_proofs_non_empty() {
        assert_proofs_non_empty::<CsvStringRecord>("CsvStringRecord");
    }

    #[test]
    fn csv_byte_record_proofs_non_empty() {
        assert_proofs_non_empty::<CsvByteRecord>("CsvByteRecord");
    }

    #[test]
    fn csv_error_kind_proofs_non_empty() {
        assert_proofs_non_empty::<CsvErrorKind>("CsvErrorKind");
    }
}

mod type_spec {
    use super::*;

    #[test]
    fn csv_quote_style_spec_registered() {
        assert!(lookup_type_spec("csv::QuoteStyle").is_some());
    }

    #[test]
    fn csv_trim_spec_registered() {
        assert!(lookup_type_spec("csv::Trim").is_some());
    }

    #[test]
    fn csv_terminator_spec_registered() {
        assert!(lookup_type_spec("csv::Terminator").is_some());
    }

    #[test]
    fn csv_position_spec_registered() {
        assert!(lookup_type_spec("csv::Position").is_some());
    }

    #[test]
    fn csv_string_record_spec_registered() {
        assert!(lookup_type_spec("csv::StringRecord").is_some());
    }

    #[test]
    fn csv_byte_record_spec_registered() {
        assert!(lookup_type_spec("csv::ByteRecord").is_some());
    }

    #[test]
    fn csv_error_kind_spec_registered() {
        assert!(lookup_type_spec("csv::ErrorKind").is_some());
    }
}
