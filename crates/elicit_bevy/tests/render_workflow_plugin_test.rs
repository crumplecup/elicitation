//! Integration tests for `BevyRenderWorkflowPlugin`.

use elicit_bevy::BevyRenderWorkflowPlugin;
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
fn render_workflow_plugin_lists_expected_tools() {
    let plugin = BevyRenderWorkflowPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "describe",
            "emit_spawn_code",
            "new_camera_2d",
            "new_camera_3d",
            "set_hdr",
            "set_orthographic_projection",
            "set_perspective_projection",
            "set_render_target",
            "set_tonemapping",
            "set_transform",
        ]
    );
}

#[tokio::test]
async fn render_workflow_camera_3d_emits_spawn_code() {
    let plugin = BevyRenderWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_camera_3d", serde_json::json!({ "name": "MainCamera" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let camera_id = created_json["camera_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "set_transform",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "transform_expr": "Transform::from_xyz(0.0, 4.0, 12.0)"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_hdr",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "hdr": true
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_render_target",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "render_target": {
                    "kind": "primary_window"
                }
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_tonemapping",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "tonemapping_expr": "Tonemapping::AgX"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_perspective_projection",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "projection": {
                    "fov": 1.0,
                    "near": 0.1,
                    "far": 500.0
                }
            }),
        )
        .await
        .unwrap();

    let described = plugin
        .invoke_tool(
            "describe",
            serde_json::json!({ "camera_id": camera_id.clone() }),
        )
        .await
        .unwrap();
    let described_json: serde_json::Value = serde_json::from_str(&result_text(&described)).unwrap();
    assert_eq!(described_json["kind"], "camera_3d");
    assert_eq!(described_json["name"], "MainCamera");

    let emitted = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "camera_id": camera_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("//Rendercamera:MainCamera"));
    assert!(source.contains("commands.spawn(("));
    assert!(source.contains("::bevy::camera::Camera3d::default()"));
    assert!(source.contains("::bevy::camera::Projection::Perspective("));
    assert!(source.contains("::bevy::camera::PerspectiveProjection{"));
    assert!(source.contains("fov:1"));
    assert!(source.contains("near:0.1"));
    assert!(source.contains("far:500"));
    assert!(
        source.contains("::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary)")
    );
    assert!(source.contains("Tonemapping::AgX"));
    assert!(source.contains("::bevy::camera::Camera{hdr:true"));
    assert!(source.contains("Transform::from_xyz(0.0,4.0,12.0)"));
}

#[tokio::test]
async fn render_workflow_camera_2d_emits_spawn_code() {
    let plugin = BevyRenderWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_camera_2d", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let camera_id = created_json["camera_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "set_render_target",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "render_target": {
                    "kind": "window_entity",
                    "target_expr": "hud_window"
                }
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_orthographic_projection",
            serde_json::json!({
                "camera_id": camera_id.clone(),
                "projection": {
                    "scale": 2.0
                }
            }),
        )
        .await
        .unwrap();

    let emitted = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "camera_id": camera_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("::bevy::camera::Camera2d::default()"));
    assert!(source.contains("::bevy::camera::Projection::Orthographic("));
    assert!(source.contains("::bevy::camera::OrthographicProjection{"));
    assert!(source.contains("scale:2"));
    assert!(source.contains("..::bevy::camera::OrthographicProjection::default_2d()"));
    assert!(source.contains("WindowRef::Entity(hud_window)"));
}

#[tokio::test]
async fn render_workflow_rejects_perspective_on_2d_camera() {
    let plugin = BevyRenderWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_camera_2d", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let camera_id = created_json["camera_id"].as_str().unwrap().to_string();

    let error = plugin
        .invoke_tool(
            "set_perspective_projection",
            serde_json::json!({
                "camera_id": camera_id,
                "projection": {
                    "fov": 1.0
                }
            }),
        )
        .await
        .unwrap_err();

    assert!(error.message.contains("set_perspective_projection"));
    assert!(error.message.contains("Camera3d"));
}
