//! Integration tests for `BevyScenePlugin`.

use elicit_bevy::BevyScenePlugin;
use elicitation::ElicitPlugin;

fn result_text(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text().map(|text| text.text.clone()))
        .collect()
}

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn scene_plugin_lists_expected_tools() {
    let plugin = BevyScenePlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "add_entity",
            "add_resource",
            "emit_ron",
            "emit_spawn_code",
            "new",
        ]
    );
}

#[tokio::test]
async fn scene_descriptor_roundtrip_emits_ron_manifest() {
    let plugin = BevyScenePlugin::new();

    let created = plugin
        .invoke_tool("new", serde_json::json!({ "name": "LevelOne" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let scene_id = created_json["scene_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "add_resource",
            serde_json::json!({
                "scene_id": scene_id,
                "resource": {
                    "type_path": "my_game::GameConfig",
                    "value_expr": "GameConfig::default()"
                }
            }),
        )
        .await
        .unwrap();

    plugin
        .invoke_tool(
            "add_entity",
            serde_json::json!({
                "scene_id": scene_id,
                "name": "CameraRig",
                "components": [
                    {
                        "type_path": "bevy::transform::components::Transform",
                        "value_expr": "Transform::from_xyz(0.0, 5.0, 12.0)"
                    },
                    {
                        "type_path": "bevy::camera::components::Camera3d",
                        "value_expr": "Camera3d::default()"
                    }
                ]
            }),
        )
        .await
        .unwrap();

    let emitted = plugin
        .invoke_tool("emit_ron", serde_json::json!({ "scene_id": scene_id }))
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("name:\"LevelOne\""));
    assert!(source.contains("type_path:\"my_game::GameConfig\""));
    assert!(source.contains("value_expr:\"GameConfig::default()\""));
    assert!(source.contains("name:Some(\"CameraRig\")"));
    assert!(source.contains("type_path:\"bevy::transform::components::Transform\""));
    assert!(source.contains("value_expr:\"Transform::from_xyz(0.0,5.0,12.0)\""));
}

#[tokio::test]
async fn scene_spawn_code_emits_commands_for_resources_and_entities() {
    let plugin = BevyScenePlugin::new();

    let created = plugin
        .invoke_tool("new", serde_json::json!({ "name": "Gameplay" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let scene_id = created_json["scene_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "add_resource",
            serde_json::json!({
                "scene_id": scene_id,
                "resource": {
                    "type_path": "my_game::Settings",
                    "value_expr": "Settings::default()"
                }
            }),
        )
        .await
        .unwrap();

    plugin
        .invoke_tool(
            "add_entity",
            serde_json::json!({
                "scene_id": scene_id,
                "name": "Player",
                "components": [
                    {
                        "type_path": "bevy::sprite::Sprite",
                        "value_expr": "Sprite::default()"
                    }
                ]
            }),
        )
        .await
        .unwrap();

    let emitted = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "scene_id": scene_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("//Scene:Gameplay"));
    assert!(source.contains("commands.insert_resource(Settings::default());"));
    assert!(source.contains("commands.spawn(("));
    assert!(source.contains("::bevy::prelude::Name::new(\"Player\")"));
    assert!(source.contains("Sprite::default()"));
}

#[tokio::test]
async fn scene_add_entity_rejects_invalid_component_paths() {
    let plugin = BevyScenePlugin::new();

    let created = plugin
        .invoke_tool("new", serde_json::json!({ "name": "Broken" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let scene_id = created_json["scene_id"].as_str().unwrap().to_string();

    let error = plugin
        .invoke_tool(
            "add_entity",
            serde_json::json!({
                "scene_id": scene_id,
                "components": [
                    {
                        "type_path": "not a path",
                        "value_expr": "Sprite::default()"
                    }
                ]
            }),
        )
        .await
        .unwrap_err();

    assert!(error.message.contains("invalid component type path"));
}
