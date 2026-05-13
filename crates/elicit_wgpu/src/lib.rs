//! `elicit_wgpu` — code-generation MCP tools for wgpu GPU descriptor construction.
//!
//! All tools are **emit-only**: they generate Rust code snippets for GPU
//! applications. No GPU device is created or accessed at runtime.
//!
//! # Plugins
//!
//! | Plugin | Prefix | Tools | Coverage |
//! |--------|--------|-------|---------|
//! | [`WgpuInitPlugin`] | `wgpu_init__` | 4 | Instance/adapter/device/surface init |
//! | [`WgpuResourcePlugin`] | `wgpu_resource__` | 3 | Buffer/texture/sampler descriptors |
//! | [`WgpuPipelinePlugin`] | `wgpu_pipeline__` | 4 | Render pipeline state descriptors |
//! | [`WgpuShaderPlugin`] | `wgpu_shader__` | 3 | Shader module and stage descriptors |
//! | [`WgpuComputePlugin`] | `wgpu_compute__` | 3 | Compute pipeline and dispatch |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod workflow;

pub use workflow::{
    WgpuComputeDispatched, WgpuComputePlugin, WgpuInitPlugin, WgpuInitialized, WgpuPipelineBuilt,
    WgpuPipelinePlugin, WgpuResourceDescribed, WgpuResourcePlugin, WgpuShaderPlugin,
    WgpuShaderStaged,
};
