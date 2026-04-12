//! Smoke test for `elicit_wgpu` — verifies the crate compiles and plugins register tools.

use elicit_wgpu::{
    WgpuComputePlugin, WgpuInitPlugin, WgpuPipelinePlugin, WgpuResourcePlugin, WgpuShaderPlugin,
};
use elicitation::ElicitPlugin;

#[test]
fn all_plugins_have_tools() {
    assert!(
        !WgpuInitPlugin::new().list_tools().is_empty(),
        "init plugin empty"
    );
    assert!(
        !WgpuResourcePlugin::new().list_tools().is_empty(),
        "resource plugin empty"
    );
    assert!(
        !WgpuPipelinePlugin::new().list_tools().is_empty(),
        "pipeline plugin empty"
    );
    assert!(
        !WgpuShaderPlugin::new().list_tools().is_empty(),
        "shader plugin empty"
    );
    assert!(
        !WgpuComputePlugin::new().list_tools().is_empty(),
        "compute plugin empty"
    );
}

#[test]
fn total_tool_count_matches_plan() {
    let total: usize = [
        WgpuInitPlugin::new().list_tools().len(),
        WgpuResourcePlugin::new().list_tools().len(),
        WgpuPipelinePlugin::new().list_tools().len(),
        WgpuShaderPlugin::new().list_tools().len(),
        WgpuComputePlugin::new().list_tools().len(),
    ]
    .iter()
    .sum();
    // 4 + 3 + 4 + 3 + 3 = 17 tools
    assert_eq!(total, 17, "unexpected tool count: {total}");
}
