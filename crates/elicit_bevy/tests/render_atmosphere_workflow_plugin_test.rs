//! Integration tests for `BevyRenderAtmosphereWorkflowPlugin`.

use elicit_bevy::BevyRenderAtmosphereWorkflowPlugin;
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
fn render_atmosphere_workflow_plugin_lists_expected_tools() {
    let plugin = BevyRenderAtmosphereWorkflowPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "add_term",
            "clear_terms",
            "describe",
            "emit_code",
            "new_atmosphere",
            "remove_term",
            "replace_term",
            "set_atmosphere",
            "set_density_multiplier",
            "set_medium_label",
            "set_resolutions",
            "set_scattering_media_var",
        ]
    );
}

#[tokio::test]
async fn render_atmosphere_workflow_emits_custom_medium_block() {
    let plugin = BevyRenderAtmosphereWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_atmosphere", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let atmosphere_id = created_json["atmosphere_id"].as_str().unwrap().to_string();

    plugin
        .invoke_tool(
            "set_scattering_media_var",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "scattering_media_var": "scattering_media"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_medium_label",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "medium_label": "dusty_sky"
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_resolutions",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "falloff_resolution": 128,
                "phase_resolution": 64
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_density_multiplier",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "density_multiplier": 0.5
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "add_term",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "term": {
                    "absorption": { "x": 0.0, "y": 0.0, "z": 0.0 },
                    "scattering": { "x": 0.001, "y": 0.002, "z": 0.003 },
                    "falloff": { "variant": "exponential", "scale": 0.3 },
                    "phase": { "variant": "mie", "asymmetry": 0.7 }
                }
            }),
        )
        .await
        .unwrap();
    plugin
        .invoke_tool(
            "set_atmosphere",
            serde_json::json!({
                "atmosphere_id": atmosphere_id.clone(),
                "atmosphere": {
                    "bottom_radius": 100.0,
                    "top_radius": 120.0,
                    "ground_albedo": { "x": 0.2, "y": 0.25, "z": 0.3 }
                }
            }),
        )
        .await
        .unwrap();

    let described = plugin
        .invoke_tool(
            "describe",
            serde_json::json!({ "atmosphere_id": atmosphere_id.clone() }),
        )
        .await
        .unwrap();
    let described_json: serde_json::Value = serde_json::from_str(&result_text(&described)).unwrap();
    assert_eq!(described_json["medium_label"], "dusty_sky");
    assert_eq!(described_json["terms"].as_array().unwrap().len(), 1);

    let emitted = plugin
        .invoke_tool(
            "emit_code",
            serde_json::json!({ "atmosphere_id": atmosphere_id }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("(scattering_media).add("));
    assert!(source.contains("::bevy::pbr::ScatteringMedium::new(128"));
    assert!(source.contains("with_label(\"dusty_sky\")"));
    assert!(source.contains("with_density_multiplier(0.5"));
    assert!(source.contains("PhaseFunction::Mie{asymmetry:0.7"));
    assert!(source.contains("bottom_radius:100"));
    assert!(source.contains("ground_albedo:"));
    assert!(source.contains("Vec3::new(0.2"));
}

#[tokio::test]
async fn render_atmosphere_workflow_emits_earthlike_defaults() {
    let plugin = BevyRenderAtmosphereWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_atmosphere", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let atmosphere_id = created_json["atmosphere_id"].as_str().unwrap().to_string();

    let emitted = plugin
        .invoke_tool(
            "emit_code",
            serde_json::json!({ "atmosphere_id": atmosphere_id }),
        )
        .await
        .unwrap();
    let source = normalize(&result_text(&emitted));

    assert!(source.contains("::bevy::pbr::ScatteringMedium::earthlike("));
    assert!(source.contains("::bevy::pbr::Atmosphere::earthlike(medium)"));
}

#[tokio::test]
async fn render_atmosphere_workflow_rejects_zero_resolutions() {
    let plugin = BevyRenderAtmosphereWorkflowPlugin::new();

    let created = plugin
        .invoke_tool("new_atmosphere", serde_json::json!({}))
        .await
        .unwrap();
    let created_json: serde_json::Value = serde_json::from_str(&result_text(&created)).unwrap();
    let atmosphere_id = created_json["atmosphere_id"].as_str().unwrap().to_string();

    let error = plugin
        .invoke_tool(
            "set_resolutions",
            serde_json::json!({
                "atmosphere_id": atmosphere_id,
                "falloff_resolution": 0,
                "phase_resolution": 64
            }),
        )
        .await
        .unwrap_err();

    assert!(error.message.contains("falloff_resolution"));
}
