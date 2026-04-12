//! Workflow plugins for `elicit_wgpu`.

mod compute_plugin;
mod init_plugin;
mod pipeline_plugin;
mod resource_plugin;
mod shader_plugin;

pub use compute_plugin::{WgpuComputeDispatched, WgpuComputePlugin};
pub use init_plugin::{WgpuInitPlugin, WgpuInitialized};
pub use pipeline_plugin::{WgpuPipelineBuilt, WgpuPipelinePlugin};
pub use resource_plugin::{WgpuResourceDescribed, WgpuResourcePlugin};
pub use shader_plugin::{WgpuShaderPlugin, WgpuShaderStaged};
