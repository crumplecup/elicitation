//! Integration tests for `elicit_accesskit`.
//!
//! Covers:
//! - Enum wrapper round-trip serialization (JSON → wrapper → JSON)
//! - `NodeJson` construction and JSON schema generation
//! - `TreeUpdateJson` construction and serialization
//! - `reflect_methods` invocations on `Role` and `Action` wrappers
//! - Geometry and struct wrapper conversions

use accesskit::{
    Node as AkNode, NodeId as AkNodeId, Role as AkRole, TreeId as AkTreeId, TreeUpdate,
};
use elicit_accesskit::{
    Action, Invalid, NodeEntry, NodeId, NodeJson, Rect, Role, Tree, TreeUpdateJson,
};
use schemars::schema_for;
use serde_json::{from_value, json, to_value};

// ── Enum wrappers ─────────────────────────────────────────────────────────────

#[test]
fn role_serializes_as_inner() {
    let role = Role(AkRole::Button);
    let v = to_value(&role).unwrap();
    let expected = to_value(AkRole::Button).unwrap();
    assert_eq!(v, expected);
}

#[test]
fn role_round_trips_json() {
    let json_val = json!("button");
    let role: Role = from_value(json_val.clone()).unwrap();
    assert_eq!(role.0, AkRole::Button);
    assert_eq!(to_value(&role).unwrap(), json_val);
}

#[test]
fn action_round_trips_json() {
    let json_val = json!("click");
    let action: Action = from_value(json_val.clone()).unwrap();
    assert_eq!(to_value(&action).unwrap(), json_val);
}

#[test]
fn invalid_round_trips_json() {
    let json_val = json!("grammar");
    let inv: Invalid = from_value(json_val.clone()).unwrap();
    assert_eq!(to_value(&inv).unwrap(), json_val);
}

// ── reflect_methods on Role wrapper ──────────────────────────────────────────

#[test]
fn role_name_button() {
    assert_eq!(Role(AkRole::Button).name(), "Button");
}

#[test]
fn role_is_form_control_button() {
    assert!(Role(AkRole::Button).is_form_control());
    assert!(!Role(AkRole::Region).is_form_control());
}

#[test]
fn role_is_text_input_text_input() {
    assert!(Role(AkRole::TextInput).is_text_input());
    assert!(!Role(AkRole::Button).is_text_input());
}

#[test]
fn role_is_container_group() {
    assert!(Role(AkRole::Group).is_container());
    assert!(!Role(AkRole::Button).is_container());
}

#[test]
fn role_is_landmark_main() {
    assert!(Role(AkRole::Main).is_landmark());
    assert!(!Role(AkRole::Button).is_landmark());
}

// ── reflect_methods on Action wrapper ────────────────────────────────────────

#[test]
fn action_name_click() {
    assert_eq!(Action(accesskit::Action::Click).name(), "Click");
}

#[test]
fn action_is_value_action_set_value() {
    assert!(Action(accesskit::Action::SetValue).is_value_action());
    assert!(!Action(accesskit::Action::Click).is_value_action());
}

#[test]
fn action_is_focus_action_focus() {
    assert!(Action(accesskit::Action::Focus).is_focus_action());
    assert!(!Action(accesskit::Action::Click).is_focus_action());
}

// ── NodeId ────────────────────────────────────────────────────────────────────

#[test]
fn node_id_round_trips() {
    let id = AkNodeId::from(42u64);
    let wrapped: NodeId = id.into();
    let back: AkNodeId = wrapped.into();
    assert_eq!(id, back);
}

#[test]
fn node_id_serializes_as_u64() {
    let id = AkNodeId::from(1u64);
    let wrapped: NodeId = id.into();
    let v = to_value(&wrapped).unwrap();
    assert_eq!(v, json!(1u64));
}

// ── Geometry ──────────────────────────────────────────────────────────────────

#[test]
fn rect_round_trips() {
    let ak = accesskit::Rect {
        x0: 0.0,
        y0: 10.0,
        x1: 100.0,
        y1: 200.0,
    };
    let wrapped: Rect = ak.into();
    let back: accesskit::Rect = wrapped.into();
    assert_eq!(ak.x0, back.x0);
    assert_eq!(ak.y0, back.y0);
    assert_eq!(ak.x1, back.x1);
    assert_eq!(ak.y1, back.y1);
}

// ── Structs ───────────────────────────────────────────────────────────────────

#[test]
fn color_round_trips() {
    use elicit_accesskit::Color;
    let ak = accesskit::Color {
        red: 255,
        green: 128,
        blue: 0,
        alpha: 200,
    };
    let wrapped: Color = ak.into();
    assert_eq!(wrapped.red, 255);
    assert_eq!(wrapped.green, 128);
    let back: accesskit::Color = wrapped.into();
    assert_eq!(ak.red, back.red);
    assert_eq!(ak.green, back.green);
}

#[test]
fn tree_round_trips() {
    let root = AkNodeId::from(1u64);
    let ak = accesskit::Tree::new(root);
    let wrapped: Tree = ak.into();
    let back: accesskit::Tree = wrapped.into();
    assert_eq!(root, back.root);
}

// ── NodeJson ──────────────────────────────────────────────────────────────────

#[test]
fn node_json_from_button_node() {
    let mut node = AkNode::new(AkRole::Button);
    node.set_label("Submit");

    let nj = NodeJson::from(&node);
    assert_eq!(nj.role.0, AkRole::Button);
    assert_eq!(nj.label.as_deref(), Some("Submit"));
}

#[test]
fn node_json_schema_generates() {
    let schema = schema_for!(NodeJson);
    let v = to_value(&schema).unwrap();
    let props = v["properties"].as_object().expect("should have properties");
    assert!(
        props.contains_key("role"),
        "schema should contain role field"
    );
}

#[test]
fn node_json_serializes_to_json() {
    let mut node = AkNode::new(AkRole::TextInput);
    node.set_label("Username");

    let nj = NodeJson::from(&node);
    let v = to_value(&nj).unwrap();
    assert_eq!(v["role"], json!("textInput"));
    assert_eq!(v["label"], json!("Username"));
}

// ── TreeUpdateJson ────────────────────────────────────────────────────────────

#[test]
fn tree_update_json_round_trip() {
    let root_id = AkNodeId::from(1u64);

    let mut root_node = AkNode::new(AkRole::RootWebArea);
    root_node.set_label("App");

    let update = TreeUpdate {
        nodes: vec![(root_id, root_node)],
        tree: Some(accesskit::Tree::new(root_id)),
        tree_id: AkTreeId::ROOT,
        focus: root_id,
    };

    let json_update = TreeUpdateJson::from(update);
    assert_eq!(json_update.nodes.len(), 1);
    assert_eq!(json_update.nodes[0].id, NodeId::from(root_id));

    let v = to_value(&json_update).unwrap();
    assert!(v["nodes"].is_array());
    assert_eq!(v["nodes"][0]["node"]["role"], json!("rootWebArea"));
}
