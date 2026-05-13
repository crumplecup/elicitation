//! Integration tests for `elicit_wgpu` workflow plugins.

use elicit_wgpu::{
    WgpuComputeDispatched, WgpuComputePlugin, WgpuInitPlugin, WgpuInitialized, WgpuPipelineBuilt,
    WgpuPipelinePlugin, WgpuResourceDescribed, WgpuResourcePlugin, WgpuShaderPlugin,
    WgpuShaderStaged,
};
use elicitation::{ElicitPlugin, VerifiedWorkflow};

#[track_caller]
fn assert_verified<T: VerifiedWorkflow>(label: &str) {
    assert!(T::validate_proofs_non_empty(), "{label}: proofs are empty");
}

// ── WgpuInitPlugin ────────────────────────────────────────────────────────────

#[test]
fn init_plugin_creates_successfully() {
    assert_eq!(WgpuInitPlugin::new().name(), "wgpu_init");
}

#[test]
fn init_plugin_lists_expected_tools() {
    let names: Vec<String> = WgpuInitPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    for name in &[
        "wgpu_init__instance",
        "wgpu_init__adapter_request",
        "wgpu_init__device_request",
        "wgpu_init__surface_config",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn init_proposition_proofs_non_empty() {
    assert_verified::<WgpuInitialized>("WgpuInitialized");
}

// ── WgpuResourcePlugin ───────────────────────────────────────────────────────

#[test]
fn resource_plugin_creates_successfully() {
    assert_eq!(WgpuResourcePlugin::new().name(), "wgpu_resource");
}

#[test]
fn resource_plugin_lists_expected_tools() {
    let names: Vec<String> = WgpuResourcePlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    for name in &[
        "wgpu_resource__buffer_desc",
        "wgpu_resource__texture_desc",
        "wgpu_resource__sampler_desc",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn resource_proposition_proofs_non_empty() {
    assert_verified::<WgpuResourceDescribed>("WgpuResourceDescribed");
}

// ── WgpuPipelinePlugin ───────────────────────────────────────────────────────

#[test]
fn pipeline_plugin_creates_successfully() {
    assert_eq!(WgpuPipelinePlugin::new().name(), "wgpu_pipeline");
}

#[test]
fn pipeline_plugin_lists_expected_tools() {
    let names: Vec<String> = WgpuPipelinePlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    for name in &[
        "wgpu_pipeline__primitive_state",
        "wgpu_pipeline__blend_state",
        "wgpu_pipeline__color_target_state",
        "wgpu_pipeline__depth_stencil_state",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn pipeline_proposition_proofs_non_empty() {
    assert_verified::<WgpuPipelineBuilt>("WgpuPipelineBuilt");
}

// ── WgpuShaderPlugin ─────────────────────────────────────────────────────────

#[test]
fn shader_plugin_creates_successfully() {
    assert_eq!(WgpuShaderPlugin::new().name(), "wgpu_shader");
}

#[test]
fn shader_plugin_lists_expected_tools() {
    let names: Vec<String> = WgpuShaderPlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    for name in &[
        "wgpu_shader__module_inline",
        "wgpu_shader__vertex_state",
        "wgpu_shader__fragment_state",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn shader_proposition_proofs_non_empty() {
    assert_verified::<WgpuShaderStaged>("WgpuShaderStaged");
}

// ── WgpuComputePlugin ────────────────────────────────────────────────────────

#[test]
fn compute_plugin_creates_successfully() {
    assert_eq!(WgpuComputePlugin::new().name(), "wgpu_compute");
}

#[test]
fn compute_plugin_lists_expected_tools() {
    let names: Vec<String> = WgpuComputePlugin::new()
        .list_tools()
        .iter()
        .map(|t| t.name.to_string())
        .collect();
    for name in &[
        "wgpu_compute__pipeline_desc",
        "wgpu_compute__dispatch",
        "wgpu_compute__bind_group_layout_entry",
    ] {
        assert!(names.iter().any(|n| n == name), "missing tool: {name}");
    }
}

#[test]
fn compute_proposition_proofs_non_empty() {
    assert_verified::<WgpuComputeDispatched>("WgpuComputeDispatched");
}

// ── invoke_tool smoke tests ───────────────────────────────────────────────────

#[tokio::test]
async fn init_instance_default_backend() {
    let plugin = WgpuInitPlugin::new();
    let result = plugin
        .invoke_tool(
            "wgpu_init__instance",
            serde_json::json!({ "backend": null }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("text content");

    assert!(
        text.contains("Instance::new"),
        "expected Instance::new; got: {text}"
    );
    assert!(
        text.contains("Backends::all()"),
        "expected Backends::all(); got: {text}"
    );
}

#[tokio::test]
async fn resource_buffer_desc_emits_code() {
    let plugin = WgpuResourcePlugin::new();
    let result = plugin
        .invoke_tool(
            "wgpu_resource__buffer_desc",
            serde_json::json!({
                "label": "my_buffer",
                "size": 256,
                "usage": "UNIFORM",
                "mapped_at_creation": false
            }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("text content");

    assert!(
        text.contains("BufferDescriptor"),
        "expected BufferDescriptor; got: {text}"
    );
    assert!(text.contains("256"), "expected size 256; got: {text}");
    assert!(
        text.contains("UNIFORM"),
        "expected UNIFORM usage; got: {text}"
    );
}

#[tokio::test]
async fn pipeline_primitive_state_emits_code() {
    let plugin = WgpuPipelinePlugin::new();
    let result = plugin
        .invoke_tool(
            "wgpu_pipeline__primitive_state",
            serde_json::json!({
                "topology": "triangle-list",
                "strip_index_format": null,
                "front_face": "ccw",
                "cull_mode": "back",
                "polygon_mode": "fill"
            }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("text content");

    assert!(
        text.contains("PrimitiveState"),
        "expected PrimitiveState; got: {text}"
    );
    assert!(
        text.contains("TriangleList"),
        "expected TriangleList; got: {text}"
    );
    assert!(
        text.contains("Back"),
        "expected Back cull mode; got: {text}"
    );
}

#[tokio::test]
async fn compute_dispatch_emits_code() {
    let plugin = WgpuComputePlugin::new();
    let result = plugin
        .invoke_tool(
            "wgpu_compute__dispatch",
            serde_json::json!({ "x": 64, "y": 1, "z": 1 }),
        )
        .await
        .expect("tool should succeed");

    let text = result
        .content
        .iter()
        .find_map(|c| {
            if let rmcp::model::RawContent::Text(t) = &c.raw {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .expect("text content");

    assert!(
        text.contains("dispatch_workgroups"),
        "expected dispatch_workgroups; got: {text}"
    );
    assert!(text.contains("64"), "expected 64 workgroups; got: {text}");
}
