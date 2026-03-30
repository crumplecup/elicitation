//! Tests for `PromptTree::to_accesskit_tree()` behind the
//! `prompt-tree-accesskit` feature.

#![cfg(feature = "prompt-tree-accesskit")]

use accesskit::{NodeId, Role};
use elicitation::{Elicit, ElicitPromptTree, Prompt, PromptTree, Select};

// ============================================================================
// Helper types
// ============================================================================

#[derive(Debug, Clone, Elicit)]
#[prompt("Pick a colour:")]
enum Colour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Elicit)]
#[prompt("Configure server:")]
struct ServerCfg {
    #[prompt("Host name:")]
    host: String,
    #[prompt("Port number:")]
    port: u16,
}

// ============================================================================
// Primitives
// ============================================================================

#[test]
fn bool_to_accesskit_has_checkbox_root() {
    let update = bool::prompt_tree().to_accesskit_tree();
    assert_eq!(update.nodes.len(), 1);
    let (root_id, root_node) = &update.nodes[0];
    assert_eq!(*root_id, NodeId(0));
    assert_eq!(root_node.role(), Role::CheckBox);
}

#[test]
fn string_to_accesskit_has_textinput_root() {
    let update = String::prompt_tree().to_accesskit_tree();
    assert_eq!(update.nodes.len(), 1);
    let (_, node) = &update.nodes[0];
    assert_eq!(node.role(), Role::TextInput);
}

#[test]
fn u16_to_accesskit_has_textinput_root() {
    let update = u16::prompt_tree().to_accesskit_tree();
    let (_, node) = &update.nodes[0];
    assert_eq!(node.role(), Role::TextInput);
}

// ============================================================================
// Enums (Select → ComboBox)
// ============================================================================

#[test]
fn enum_to_accesskit_root_is_combobox() {
    let update = Colour::prompt_tree().to_accesskit_tree();
    // root node is last (DFS post-order), but let's find it by tree root id
    let root_id = update.tree.as_ref().unwrap().root;
    let root_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .expect("root node present");
    assert_eq!(root_node.role(), Role::ComboBox);
}

#[test]
fn enum_to_accesskit_has_three_options() {
    let update = Colour::prompt_tree().to_accesskit_tree();
    let option_count = update
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == Role::ListBoxOption)
        .count();
    assert_eq!(option_count, 3);
}

#[test]
fn enum_option_labels_match_variants() {
    let update = Colour::prompt_tree().to_accesskit_tree();
    let labels: Vec<_> = update
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == Role::ListBoxOption)
        .filter_map(|(_, n)| n.label())
        .collect();
    assert!(labels.contains(&"Red"));
    assert!(labels.contains(&"Green"));
    assert!(labels.contains(&"Blue"));
}

#[test]
fn enum_root_label_is_prompt() {
    let update = Colour::prompt_tree().to_accesskit_tree();
    let root_id = update.tree.as_ref().unwrap().root;
    let root_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .unwrap();
    assert_eq!(root_node.label(), Some("Pick a colour:"));
}

// ============================================================================
// Structs (Survey → Form)
// ============================================================================

#[test]
fn struct_to_accesskit_root_is_form() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let root_id = update.tree.as_ref().unwrap().root;
    let root_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .unwrap();
    assert_eq!(root_node.role(), Role::Form);
}

#[test]
fn struct_to_accesskit_root_label_is_prompt() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let root_id = update.tree.as_ref().unwrap().root;
    let root_node = update
        .nodes
        .iter()
        .find(|(id, _)| *id == root_id)
        .map(|(_, n)| n)
        .unwrap();
    assert_eq!(root_node.label(), Some("Configure server:"));
}

#[test]
fn struct_to_accesskit_has_two_group_wrappers() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let groups = update
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == Role::Group)
        .count();
    assert_eq!(groups, 2);
}

#[test]
fn struct_group_labels_are_field_names() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let labels: Vec<_> = update
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == Role::Group)
        .filter_map(|(_, n)| n.label())
        .collect();
    assert!(labels.contains(&"host"));
    assert!(labels.contains(&"port"));
}

#[test]
fn struct_field_prompts_override_defaults() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let leaf_labels: Vec<_> = update
        .nodes
        .iter()
        .filter(|(_, n)| n.role() == Role::TextInput)
        .filter_map(|(_, n)| n.label())
        .collect();
    assert!(leaf_labels.contains(&"Host name:"));
    assert!(leaf_labels.contains(&"Port number:"));
}

// ============================================================================
// TreeUpdate metadata
// ============================================================================

#[test]
fn tree_update_tree_is_some() {
    let update = bool::prompt_tree().to_accesskit_tree();
    assert!(update.tree.is_some());
}

#[test]
fn tree_update_root_id_matches_tree() {
    let update = ServerCfg::prompt_tree().to_accesskit_tree();
    let tree_root = update.tree.as_ref().unwrap().root;
    assert_eq!(update.focus, tree_root);
}
