//! Integration tests for `BevyRenderMeshWorkflowPlugin`.

use elicit_bevy::BevyRenderMeshWorkflowPlugin;
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
fn render_mesh_workflow_plugin_lists_expected_tools() {
    let plugin = BevyRenderMeshWorkflowPlugin::new();
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
            "new_mesh_2d",
            "new_mesh_3d",
            "set_material",
            "set_mesh",
            "set_transform",
            "set_wireframe_material",
        ]
    );
}

#[tokio::test]
async fn render_mesh_workflow_3d_emits_spawn_code() {
    let plugin = BevyRenderMeshWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_mesh_3d", serde_json::json!({ "name": "Crate" }))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let mesh_id = created_json["mesh_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "set_mesh",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "mesh_expr": "meshes.add(Cuboid::new(1.0, 2.0, 3.0))"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_material",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "material_expr": "materials.add(StandardMaterial::default())"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_wireframe_material",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "wireframe_material_expr": "wireframe_materials.add(WireframeMaterial::default())"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_transform",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "transform_expr": "Transform::from_xyz(1.0, 0.5, -2.0)"
            }),
        )
        .await
        .unwrap();

    let described = plugin
        .invoke_tool(
            "describe",
            serde_json::json!({ "mesh_id": mesh_id.clone() }),
        )
        .await
        .unwrap();
    let described_json: serde_json::Value = serde_json::from_str(&result_text(&described)).unwrap();
    assert_eq!(described_json["kind"], "mesh_3d");
    assert_eq!(described_json["name"], "Crate");

    let emitted = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "mesh_id": mesh_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("//Rendermesh:Crate"));
    assert!(source.contains("commands.spawn(("));
    assert!(source.contains("::bevy::mesh::Mesh3d(meshes.add(Cuboid::new(1.0,2.0,3.0)))"));
    assert!(
        source.contains("::bevy::pbr::MeshMaterial3d(materials.add(StandardMaterial::default()))")
    );
    assert!(source.contains(
        "::bevy::pbr::Mesh3dWireframe(wireframe_materials.add(WireframeMaterial::default()))"
    ));
    assert!(source.contains("Transform::from_xyz(1.0,0.5,-2.0)"));
}

#[tokio::test]
async fn render_mesh_workflow_2d_emits_spawn_code() {
    let plugin = BevyRenderMeshWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_mesh_2d", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let mesh_id = created_json["mesh_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "set_mesh",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "mesh_expr": "meshes.add(Circle::new(24.0))"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_material",
            serde_json::json!({
                "mesh_id": mesh_id.clone(),
                "material_expr": "materials.add(ColorMaterial::from_color(RED))"
            }),
        )
        .await
        .unwrap();

    let emitted = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "mesh_id": mesh_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("::bevy::mesh::Mesh2d(meshes.add(Circle::new(24.0)))"));
    assert!(source.contains(
        "::bevy::sprite_render::MeshMaterial2d(materials.add(ColorMaterial::from_color(RED)))"
    ));
}

#[tokio::test]
async fn render_mesh_workflow_requires_mesh_before_emit() {
    let plugin = BevyRenderMeshWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_mesh_3d", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let mesh_id = created_json["mesh_id"].as_str().unwrap().to_string();

    let error = plugin
        .invoke_tool(
            "emit_spawn_code",
            serde_json::json!({
                "mesh_id": mesh_id,
                "commands_var": "commands"
            }),
        )
        .await
        .unwrap_err();

    assert!(error.message.contains("mesh_expr"));
}
