//! Tests for the static prompt tree.
//!
//! Covers [`PromptTree`] construction and traversal, [`ElicitPromptTree`]
//! blanket impls for primitives, derive-generated impls for structs and enums,
//! `assembled_prompts()` output format, and a completeness helper.

#![cfg(feature = "prompt-tree")]

use elicitation::{Elicit, ElicitPromptTree, Prompt, PromptKind, PromptTree, Select};

// ============================================================================
// Helpers
// ============================================================================

/// Assert every Leaf/Affirm/Select node in the tree has a non-empty prompt.
#[track_caller]
fn assert_prompts_complete(tree: &PromptTree, path: &str) {
    match tree {
        PromptTree::Leaf { prompt, .. } | PromptTree::Affirm { prompt, .. } => {
            assert!(!prompt.is_empty(), "{path}: empty prompt on leaf/affirm");
        }
        PromptTree::Select {
            prompt,
            options,
            branches,
            ..
        } => {
            assert!(!prompt.is_empty(), "{path}: empty prompt on select");
            assert!(
                options.len() >= 2,
                "{path}: select has fewer than 2 options"
            );
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

// ============================================================================
// Primitives
// ============================================================================

#[test]
fn bool_is_affirm() {
    let tree = bool::prompt_tree();
    assert!(matches!(tree, PromptTree::Affirm { .. }));
    assert!(!tree.prompt().unwrap().is_empty());
    assert_eq!(tree.type_name(), "bool");
    assert_eq!(tree.depth(), 1);
}

#[test]
fn integer_is_leaf() {
    for tree in [
        i8::prompt_tree(),
        i32::prompt_tree(),
        u64::prompt_tree(),
        f64::prompt_tree(),
    ] {
        assert!(matches!(tree, PromptTree::Leaf { .. }));
        assert_eq!(tree.depth(), 1);
    }
}

#[test]
fn string_is_leaf() {
    let tree = String::prompt_tree();
    assert!(matches!(tree, PromptTree::Leaf { .. }));
    assert_eq!(tree.type_name(), "String");
}

#[test]
fn unit_is_leaf() {
    let tree = <()>::prompt_tree();
    assert!(matches!(tree, PromptTree::Leaf { .. }));
    assert_eq!(tree.type_name(), "()");
}

// ============================================================================
// Generic containers
// ============================================================================

#[test]
fn vec_delegates_to_inner() {
    let inner = bool::prompt_tree();
    let vec_tree = Vec::<bool>::prompt_tree();
    assert_eq!(vec_tree, inner);
}

#[test]
fn option_delegates_to_inner() {
    let inner = i32::prompt_tree();
    let opt_tree = Option::<i32>::prompt_tree();
    assert_eq!(opt_tree, inner);
}

// ============================================================================
// assembled_prompts() for primitives
// ============================================================================

#[test]
fn bool_assembled_is_one_prompt() {
    let prompts = bool::assembled_prompts();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].kind, PromptKind::Affirm);
    assert!(prompts[0].path.is_empty());
}

#[test]
fn string_assembled_is_one_prompt() {
    let prompts = String::assembled_prompts();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].kind, PromptKind::Leaf);
}

// ============================================================================
// Derived enums
// ============================================================================

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Elicit,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[prompt("Pick a color:")]
enum Color {
    Red,
    Green,
    Blue,
}

#[test]
fn derived_enum_is_select() {
    let tree = Color::prompt_tree();
    let PromptTree::Select {
        prompt,
        type_name,
        options,
        branches,
    } = &tree
    else {
        panic!("expected Select, got {tree:?}");
    };
    assert_eq!(prompt, "Pick a color:");
    assert_eq!(type_name, "Color");
    assert_eq!(options, &["Red", "Green", "Blue"]);
    assert_eq!(branches.len(), 3);
    assert!(
        branches.iter().all(|b| b.is_none()),
        "unit variants → None branches"
    );
    assert_eq!(tree.depth(), 1);
}

#[test]
fn derived_enum_assembled_contains_options_list() {
    let prompts = Color::assembled_prompts();
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0].kind, PromptKind::Select);
    let text = &prompts[0].text;
    assert!(text.contains("Pick a color:"), "missing base prompt");
    assert!(text.contains("1. Red"), "missing option 1");
    assert!(text.contains("2. Green"), "missing option 2");
    assert!(text.contains("3. Blue"), "missing option 3");
    assert!(
        text.contains("Respond with the number (1-3)"),
        "missing footer"
    );
}

/// Enum with a tuple variant — exercises branch sub-tree generation.
#[derive(
    Debug, Clone, PartialEq, Eq, Elicit, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
enum Shape {
    Circle,
    Rect(u32, u32),
}

#[test]
fn enum_with_tuple_variant_has_branch() {
    let tree = Shape::prompt_tree();
    let PromptTree::Select {
        options, branches, ..
    } = &tree
    else {
        panic!("expected Select");
    };
    assert_eq!(options, &["Circle", "Rect"]);
    assert!(branches[0].is_none(), "Circle is unit → None");
    assert!(branches[1].is_some(), "Rect has fields → Some");

    let branch = branches[1].as_deref().unwrap();
    assert!(matches!(branch, PromptTree::Survey { .. }));
    let PromptTree::Survey { fields, .. } = branch else {
        unreachable!()
    };
    assert_eq!(fields.len(), 2);
    assert_eq!(fields[0].0, "_0");
    assert_eq!(fields[1].0, "_1");
}

#[test]
fn enum_with_branch_assembled_includes_subprompts() {
    // Shape::Rect has two u32 fields; assembled_prompts walks the first branch
    let prompts = Shape::assembled_prompts();
    // 1 Select prompt + 2 field prompts from the Rect branch
    assert_eq!(prompts.len(), 3);
    assert_eq!(prompts[0].kind, PromptKind::Select);
    assert_eq!(prompts[1].kind, PromptKind::Leaf);
    assert_eq!(prompts[2].kind, PromptKind::Leaf);
    assert_eq!(prompts[1].path, vec!["Rect", "_0"]);
    assert_eq!(prompts[2].path, vec!["Rect", "_1"]);
}

// ============================================================================
// Derived structs
// ============================================================================

#[derive(Debug, Clone, Elicit, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[prompt("Configure server:")]
struct ServerConfig {
    #[prompt("Host name or IP:")]
    host: String,
    #[prompt("Port number:")]
    port: u16,
    #[prompt("Enable TLS?")]
    tls: bool,
}

#[test]
fn server_config_fields_accessible() {
    let cfg = ServerConfig {
        host: "localhost".to_string(),
        port: 8080,
        tls: true,
    };
    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
    assert!(cfg.tls);
}

#[test]
fn derived_struct_is_survey() {
    let tree = ServerConfig::prompt_tree();
    let PromptTree::Survey {
        prompt,
        type_name,
        fields,
    } = &tree
    else {
        panic!("expected Survey, got {tree:?}");
    };
    assert_eq!(prompt.as_deref(), Some("Configure server:"));
    assert_eq!(type_name, "ServerConfig");
    assert_eq!(fields.len(), 3);
    assert_eq!(fields[0].0, "host");
    assert_eq!(fields[1].0, "port");
    assert_eq!(fields[2].0, "tls");
}

#[test]
fn derived_struct_field_types_correct() {
    let tree = ServerConfig::prompt_tree();
    let PromptTree::Survey { fields, .. } = &tree else {
        unreachable!()
    };

    assert!(
        matches!(fields[0].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "String")
    );
    assert!(
        matches!(fields[1].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "u16")
    );
    assert!(
        matches!(fields[2].1.as_ref(), PromptTree::Affirm { type_name, .. } if type_name == "bool")
    );
}

#[test]
fn derived_struct_assembled_one_per_field() {
    let prompts = ServerConfig::assembled_prompts();
    assert_eq!(prompts.len(), 3);
    assert_eq!(prompts[0].kind, PromptKind::Leaf);
    assert_eq!(prompts[1].kind, PromptKind::Leaf);
    assert_eq!(prompts[2].kind, PromptKind::Affirm);
    assert_eq!(prompts[0].path, vec!["host"]);
    assert_eq!(prompts[1].path, vec!["port"]);
    assert_eq!(prompts[2].path, vec!["tls"]);
    assert_eq!(prompts[0].text, "Host name or IP:");
    assert_eq!(prompts[1].text, "Port number:");
    assert_eq!(prompts[2].text, "Enable TLS?");
}

#[test]
fn derived_struct_depth() {
    assert_eq!(ServerConfig::prompt_tree().depth(), 2);
}

// ============================================================================
// Nesting — struct containing an enum
// ============================================================================

#[derive(Debug, Clone, Elicit, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
struct Deployment {
    #[prompt("Target environment:")]
    env: Color,
    #[prompt("Replica count:")]
    replicas: u8,
}

#[test]
fn deployment_fields_accessible() {
    let d = Deployment {
        env: Color::Red,
        replicas: 3,
    };
    assert!(matches!(d.env, Color::Red));
    assert_eq!(d.replicas, 3);
}

#[test]
fn nested_struct_depth() {
    // Deployment(Survey) → Color(Select) = depth 2
    assert_eq!(Deployment::prompt_tree().depth(), 2);
}

#[test]
fn nested_struct_assembled() {
    let prompts = Deployment::assembled_prompts();
    // env → 1 Select prompt (Color has no field branches)
    // replicas → 1 Leaf prompt
    assert_eq!(prompts.len(), 2);
    assert_eq!(prompts[0].kind, PromptKind::Select);
    assert_eq!(prompts[0].path, vec!["env"]);
    assert_eq!(prompts[1].kind, PromptKind::Leaf);
    assert_eq!(prompts[1].path, vec!["replicas"]);
}

// ============================================================================
// Completeness check
// ============================================================================

#[test]
fn all_derived_types_prompts_complete() {
    assert_prompts_complete(&bool::prompt_tree(), "bool");
    assert_prompts_complete(&String::prompt_tree(), "String");
    assert_prompts_complete(&Color::prompt_tree(), "Color");
    assert_prompts_complete(&Shape::prompt_tree(), "Shape");
    assert_prompts_complete(&ServerConfig::prompt_tree(), "ServerConfig");
    assert_prompts_complete(&Deployment::prompt_tree(), "Deployment");
}

// ============================================================================
// Regression: missing integer types usize / isize / u128 / i128
// (previously had no ElicitPromptTree impl; any struct using these as fields
// would fail to compile with `prompt-tree` enabled)
// ============================================================================

#[test]
fn all_integer_widths_are_leaf() {
    for (tree, name) in [
        (i8::prompt_tree(), "i8"),
        (i16::prompt_tree(), "i16"),
        (i32::prompt_tree(), "i32"),
        (i64::prompt_tree(), "i64"),
        (i128::prompt_tree(), "i128"),
        (isize::prompt_tree(), "isize"),
        (u8::prompt_tree(), "u8"),
        (u16::prompt_tree(), "u16"),
        (u32::prompt_tree(), "u32"),
        (u64::prompt_tree(), "u64"),
        (u128::prompt_tree(), "u128"),
        (usize::prompt_tree(), "usize"),
        (f32::prompt_tree(), "f32"),
        (f64::prompt_tree(), "f64"),
    ] {
        assert!(
            matches!(tree, PromptTree::Leaf { .. }),
            "{name}: expected Leaf, got {tree:?}"
        );
        assert_eq!(tree.type_name(), name, "{name}: wrong type_name");
        assert_eq!(tree.depth(), 1, "{name}: expected depth 1");
    }
}

/// A struct using the previously-missing integer widths should compile and
/// produce a Survey tree with the right number of fields.
#[derive(Debug, Clone, Elicit, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[prompt("Enter index info:")]
struct IndexInfo {
    #[prompt("Array index:")]
    index: usize,
    #[prompt("Signed offset:")]
    offset: isize,
    #[prompt("Large count:")]
    count: u128,
    #[prompt("Large signed:")]
    signed: i128,
}

#[test]
fn index_info_fields_accessible() {
    let info = IndexInfo {
        index: 42,
        offset: -7,
        count: 999,
        signed: -123,
    };
    assert_eq!(info.index, 42);
    assert_eq!(info.offset, -7);
    assert_eq!(info.count, 999);
    assert_eq!(info.signed, -123);
}

#[test]
fn struct_with_usize_isize_fields_is_survey() {
    let tree = IndexInfo::prompt_tree();
    let PromptTree::Survey { fields, .. } = &tree else {
        panic!("expected Survey, got {tree:?}");
    };
    assert_eq!(fields.len(), 4);
    assert!(
        matches!(fields[0].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "usize")
    );
    assert!(
        matches!(fields[1].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "isize")
    );
    assert!(
        matches!(fields[2].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "u128")
    );
    assert!(
        matches!(fields[3].1.as_ref(), PromptTree::Leaf { type_name, .. } if type_name == "i128")
    );
}

// ============================================================================
// Regression: Established<P> missing ElicitPromptTree impl
// (any struct with a proof-token field would fail to compile with prompt-tree)
// ============================================================================

mod established_tests {
    use super::*;
    use elicitation::contracts::Established;

    #[derive(elicitation::Prop)]
    struct BetPlaced;

    #[test]
    fn established_is_leaf() {
        // Established<P> itself should satisfy ElicitPromptTree (empty leaf).
        let tree = Established::<BetPlaced>::prompt_tree();
        assert!(
            matches!(tree, PromptTree::Leaf { .. }),
            "Established<P> should produce a Leaf, got {tree:?}"
        );
        assert_eq!(tree.type_name(), "Established");
    }

    /// A struct where the proof token is skipped (the common real-world pattern).
    /// Before the fix, the `Established<P>: ElicitPromptTree` bound was missing,
    /// so any code path that mentioned the type in a where clause would fail.
    /// We verify the trait is callable directly (compilation = test passing).
    #[test]
    fn established_impl_exists_for_multiple_props() {
        // Two different Prop types — both should satisfy ElicitPromptTree.
        use elicitation::contracts::Is;
        let t1 = Established::<BetPlaced>::prompt_tree();
        let t2 = Established::<Is<String>>::prompt_tree();
        for tree in [&t1, &t2] {
            assert!(
                matches!(tree, PromptTree::Leaf { .. }),
                "Established should be a Leaf, got {tree:?}"
            );
            assert_eq!(tree.type_name(), "Established");
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Elicit,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
enum TwoState {
    A,
    B,
}

#[test]
fn unit_variant_enum_has_non_empty_select_tree() {
    let tree = TwoState::prompt_tree();
    let PromptTree::Select {
        options, branches, ..
    } = &tree
    else {
        panic!("expected Select for unit-variant enum");
    };
    assert_eq!(options, &["A", "B"]);
    assert!(branches.iter().all(|b| b.is_none()));
    assert!(!tree.prompt().unwrap_or("").is_empty());
}
