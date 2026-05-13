//! Integration tests for `BevyDerivePlugin`.

use elicit_bevy::{
    AssetDeriveParams, BevyDerivePlugin, ComponentDeriveParams, EnumVariantSpec, ItemShape,
    ItemTemplate, NamedFieldSpec, ReflectDeriveParams, StatesDeriveParams, VariantShape,
};
use elicitation::ElicitPlugin;
use elicitation::emit_code::EmitCode;
use elicitation::emit_code::dispatch_emit_from;

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn derive_plugin_lists_expected_tools() {
    let plugin = BevyDerivePlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "asset",
            "bundle",
            "component",
            "event",
            "reflect",
            "resource",
            "schedule_label",
            "states",
            "system_set",
        ]
    );
}

#[test]
fn component_params_emit_named_struct() {
    let params = ComponentDeriveParams {
        item: ItemTemplate {
            name: "Velocity".into(),
            visibility: Some("pub".into()),
            docs: vec![],
            attrs: vec![],
            extra_derives: vec![],
            shape: ItemShape::NamedStruct {
                fields: vec![
                    NamedFieldSpec {
                        name: "x".into(),
                        ty: "f32".into(),
                        docs: vec![],
                        attrs: vec![],
                    },
                    NamedFieldSpec {
                        name: "y".into(),
                        ty: "f32".into(),
                        docs: vec![],
                        attrs: vec![],
                    },
                ],
            },
        },
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("#[derive(bevy::prelude::Component)]"));
    assert!(source.contains("pubstructVelocity{x:f32,y:f32}"));
}

#[test]
fn states_params_emit_enum_with_standard_traits() {
    let params = StatesDeriveParams {
        item: ItemTemplate {
            name: "AppState".into(),
            visibility: Some("pub".into()),
            docs: vec![],
            attrs: vec![],
            extra_derives: vec![],
            shape: ItemShape::Enum {
                variants: vec![
                    EnumVariantSpec {
                        name: "Loading".into(),
                        docs: vec![],
                        attrs: vec![],
                        shape: VariantShape::Unit,
                    },
                    EnumVariantSpec {
                        name: "InGame".into(),
                        docs: vec![],
                        attrs: vec![],
                        shape: VariantShape::Unit,
                    },
                    EnumVariantSpec {
                        name: "Paused".into(),
                        docs: vec![],
                        attrs: vec![],
                        shape: VariantShape::Unit,
                    },
                ],
            },
        },
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("bevy::prelude::States"));
    assert!(source.contains("Debug"));
    assert!(source.contains("Clone"));
    assert!(source.contains("PartialEq"));
    assert!(source.contains("Eq"));
    assert!(source.contains("Hash"));
    assert!(source.contains("pubenumAppState{Loading,InGame,Paused}"));
}

#[test]
fn reflect_params_emit_parenthesized_reflect_attr() {
    let params = ReflectDeriveParams {
        item: ItemTemplate {
            name: "PlayerTag".into(),
            visibility: None,
            docs: vec![],
            attrs: vec![],
            extra_derives: vec![],
            shape: ItemShape::UnitStruct,
        },
        reflect_traits: vec!["Clone".into(), "Default".into()],
        type_path: Some("my_game::components".into()),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("#[derive(bevy::reflect::Reflect)]"));
    assert!(source.contains("#[reflect(Clone,Default)]"));
    assert!(source.contains("#[type_path=\"my_game::components\"]"));
    assert!(source.contains("structPlayerTag;"));
}

#[test]
fn dispatch_emit_component_uses_registered_emit_entry() {
    let params = serde_json::json!({
        "item": {
            "name": "Marker",
            "shape": { "kind": "unit_struct" }
        }
    });
    let emitter = dispatch_emit_from("component", "elicit_bevy", params).unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("#[derive(bevy::prelude::Component)]"));
    assert!(source.contains("structMarker;"));
}

#[test]
fn params_emit_code_supports_asset_and_reflect_skeletons() {
    let asset = AssetDeriveParams {
        item: ItemTemplate {
            name: "TextureManifest".into(),
            visibility: Some("pub".into()),
            docs: vec![],
            attrs: vec![],
            extra_derives: vec!["Clone".into()],
            shape: ItemShape::NamedStruct {
                fields: vec![NamedFieldSpec {
                    name: "path".into(),
                    ty: "String".into(),
                    docs: vec![],
                    attrs: vec![],
                }],
            },
        },
    };
    let reflect = ReflectDeriveParams {
        item: ItemTemplate {
            name: "EditorState".into(),
            visibility: Some("pub".into()),
            docs: vec![],
            attrs: vec![],
            extra_derives: vec![],
            shape: ItemShape::Enum {
                variants: vec![
                    EnumVariantSpec {
                        name: "Idle".into(),
                        docs: vec![],
                        attrs: vec![],
                        shape: VariantShape::Unit,
                    },
                    EnumVariantSpec {
                        name: "Dragging".into(),
                        docs: vec![],
                        attrs: vec![],
                        shape: VariantShape::Named {
                            fields: vec![NamedFieldSpec {
                                name: "entity".into(),
                                ty: "u64".into(),
                                docs: vec![],
                                attrs: vec![],
                            }],
                        },
                    },
                ],
            },
        },
        reflect_traits: vec!["Clone".into()],
        type_path: None,
    };

    let asset_source = normalize(&asset.emit_code().to_string());
    let reflect_source = normalize(&reflect.emit_code().to_string());

    assert!(asset_source.contains("bevy::asset::Asset"));
    assert!(asset_source.contains("bevy::reflect::TypePath"));
    assert!(asset_source.contains("Clone"));
    assert!(reflect_source.contains("#[reflect(Clone)]"));
    assert!(reflect_source.contains("pubenumEditorState{Idle,Dragging{entity:u64}}"));
}

#[test]
fn params_serialize_roundtrip() {
    let params = StatesDeriveParams {
        item: ItemTemplate {
            name: "FlowState".into(),
            visibility: Some("pub".into()),
            docs: vec!["Top-level app state.".into()],
            attrs: vec![],
            extra_derives: vec![],
            shape: ItemShape::Enum {
                variants: vec![EnumVariantSpec {
                    name: "Start".into(),
                    docs: vec![],
                    attrs: vec![],
                    shape: VariantShape::Unit,
                }],
            },
        },
    };

    let json = serde_json::to_string(&params).unwrap();
    let roundtrip: StatesDeriveParams = serde_json::from_str(&json).unwrap();
    assert_eq!(roundtrip.item.name, "FlowState");
}
