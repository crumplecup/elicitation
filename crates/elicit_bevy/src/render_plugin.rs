//! `BevyRenderPlugin` — core render/material descriptor codegen tools.
//!
//! This first render slice focuses on the high-value Bevy 0.18 render building
//! blocks that agents commonly need to assemble scenes and apps:
//!
//! - `StandardMaterial`
//! - ambient/directional/point/spot lights
//! - `RenderTarget`
//! - camera spawn tuples for 2D and 3D
//! - `Tonemapping` and `AlphaMode` selectors

use elicitation::emit_code::{CrateDep, EmitCode, ToCodeLiteral};
use elicitation::{
    BevyAtmosphere, BevyColor, BevyFalloff, BevyPhaseFunction, BevyScatteringTerm, ElicitPlugin,
    elicit_tool,
};
use proc_macro2::TokenStream;
use quote::quote;
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use tracing::instrument;

/// Supported render-target output kinds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyRenderTargetKind {
    /// Emit `RenderTarget::Window(WindowRef::Primary)`.
    PrimaryWindow,
    /// Emit `RenderTarget::Window(WindowRef::Entity(expr))`.
    WindowEntity,
    /// Emit `RenderTarget::Image(expr.into())`.
    Image,
    /// Emit `RenderTarget::TextureView(expr)`.
    TextureView,
}

/// Supported tonemapping algorithms.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::core_pipeline::tonemapping::Tonemapping")]
pub enum BevyTonemappingVariant {
    /// `Tonemapping::None`
    None,
    /// `Tonemapping::Reinhard`
    Reinhard,
    /// `Tonemapping::ReinhardLuminance`
    ReinhardLuminance,
    /// `Tonemapping::AcesFitted`
    AcesFitted,
    /// `Tonemapping::AgX`
    AgX,
    /// `Tonemapping::SomewhatBoringDisplayTransform`
    SomewhatBoringDisplayTransform,
    /// `Tonemapping::TonyMcMapface`
    TonyMcMapface,
    /// `Tonemapping::BlenderFilmic`
    BlenderFilmic,
}

/// Supported alpha-mode variants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyRenderAlphaModeVariant {
    /// `AlphaMode::Opaque`
    Opaque,
    /// `AlphaMode::Mask(threshold)`
    Mask,
    /// `AlphaMode::Blend`
    Blend,
    /// `AlphaMode::Premultiplied`
    Premultiplied,
    /// `AlphaMode::AlphaToCoverage`
    AlphaToCoverage,
    /// `AlphaMode::Add`
    Add,
    /// `AlphaMode::Multiply`
    Multiply,
}

/// Parameters for `bevy_render__render_target`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct BevyRenderTargetFields {
    /// Which `RenderTarget` variant to emit.
    pub kind: BevyRenderTargetKind,
    /// Expression used by non-primary variants.
    #[serde(default)]
    pub target_expr: Option<String>,
}

/// Parameters for `bevy_render__render_target`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent, path = "::bevy::camera::RenderTarget")]
pub struct BevyRenderTargetParams {
    /// Flattened render-target fields preserving the MCP JSON shape.
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "render_target_tokens")]
    fields: BevyRenderTargetFields,
}

/// Parameters for `bevy_render__tonemapping`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyTonemappingParams {
    /// Tonemapping algorithm to emit.
    pub variant: BevyTonemappingVariant,
}

/// Supported deband-dither variants.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::core_pipeline::tonemapping::DebandDither")]
pub enum BevyDebandDitherVariant {
    /// Disable debanding dithering.
    Disabled,
    /// Enable debanding dithering.
    Enabled,
}

/// Parameters for `bevy_render__deband_dither`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyDebandDitherParams {
    /// Deband-dither mode to emit.
    pub variant: BevyDebandDitherVariant,
}

/// Parameters for `bevy_render__order_independent_transparency_settings`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::core_pipeline::oit::OrderIndependentTransparencySettings",
    update = "::std::default::Default::default()",
    default_expr = "::bevy::core_pipeline::oit::OrderIndependentTransparencySettings::default()"
)]
pub struct BevyOrderIndependentTransparencySettingsParams {
    /// Optional number of transparency layers to keep.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub layer_count: Option<i32>,
    /// Optional alpha threshold for placing fragments into OIT layers.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub alpha_threshold: Option<f32>,
}

/// Supported orthographic scaling modes.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyScalingModeVariant {
    /// Match the viewport size.
    WindowSize,
    /// Use a fixed width and height in world units.
    Fixed,
    /// Constrain the minimum width and height.
    AutoMin,
    /// Constrain the maximum width and height.
    AutoMax,
    /// Keep a fixed viewport height.
    FixedVertical,
    /// Keep a fixed viewport width.
    FixedHorizontal,
}

/// Parameters for `bevy_render__scaling_mode`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::camera::ScalingMode")]
pub enum BevyScalingModeParams {
    /// Match the viewport size.
    WindowSize,
    /// Use a fixed width and height in world units.
    Fixed {
        /// Fixed-mode width.
        width: f32,
        /// Fixed-mode height.
        height: f32,
    },
    /// Constrain the minimum width and height.
    AutoMin {
        /// Auto-min minimum width.
        min_width: f32,
        /// Auto-min minimum height.
        min_height: f32,
    },
    /// Constrain the maximum width and height.
    AutoMax {
        /// Auto-max maximum width.
        max_width: f32,
        /// Auto-max maximum height.
        max_height: f32,
    },
    /// Keep a fixed viewport height.
    FixedVertical {
        /// Fixed-vertical viewport height.
        viewport_height: f32,
    },
    /// Keep a fixed viewport width.
    FixedHorizontal {
        /// Fixed-horizontal viewport width.
        viewport_width: f32,
    },
}

/// Parameters for `bevy_render__perspective_projection`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::camera::PerspectiveProjection",
    update = "::bevy::camera::PerspectiveProjection::default()",
    default_expr = "::bevy::camera::PerspectiveProjection::default()"
)]
pub struct BevyPerspectiveProjectionParams {
    /// Optional vertical field of view in radians.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub fov: Option<f32>,
    /// Optional aspect ratio.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub aspect_ratio: Option<f32>,
    /// Optional near clip distance.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub near: Option<f32>,
    /// Optional far clip distance.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub far: Option<f32>,
    /// Optional clip-plane expression.
    #[serde(default)]
    #[to_code_literal(rename = "near_clip_plane", expr, optional)]
    pub near_clip_plane_expr: Option<String>,
}

/// Parameters for `bevy_render__orthographic_projection`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
pub(crate) struct BevyOrthographicProjectionFields {
    /// Whether to start from Bevy's 2D defaults instead of 3D defaults.
    #[serde(default)]
    pub use_2d_defaults: Option<bool>,
    /// Optional near clip distance.
    #[serde(default)]
    pub near: Option<f32>,
    /// Optional far clip distance.
    #[serde(default)]
    pub far: Option<f32>,
    /// Optional viewport-origin expression.
    #[serde(default)]
    pub viewport_origin_expr: Option<String>,
    /// Optional scaling-mode expression.
    #[serde(default)]
    pub scaling_mode_expr: Option<String>,
    /// Optional scale multiplier.
    #[serde(default)]
    pub scale: Option<f32>,
    /// Optional orthographic area expression.
    #[serde(default)]
    pub area_expr: Option<String>,
}

/// Parameters for `bevy_render__orthographic_projection`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent, path = "::bevy::camera::OrthographicProjection")]
pub struct BevyOrthographicProjectionParams {
    /// Flattened orthographic projection fields preserving the MCP JSON shape.
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "orthographic_projection_tokens")]
    pub(crate) fields: BevyOrthographicProjectionFields,
}

/// Supported clear-color configuration variants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyClearColorConfigVariant {
    /// Use the world's `ClearColor` resource.
    Default,
    /// Use a camera-specific clear color.
    Custom,
    /// Do not clear before rendering.
    None,
}

/// Parameters for `bevy_render__clear_color_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", content = "color_expr", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::camera::ClearColorConfig")]
pub enum BevyClearColorConfigParams {
    /// Use the world's `ClearColor` resource.
    Default,
    /// Use a camera-specific clear color expression.
    Custom(#[to_code_literal(expr)] String),
    /// Do not clear before rendering.
    None,
}

/// Supported MSAA writeback variants.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::camera::MsaaWriteback")]
pub enum BevyMsaaWritebackVariant {
    /// Never write back MSAA contents.
    Off,
    /// Automatically write back when needed.
    Auto,
    /// Always perform writeback.
    Always,
}

/// Parameters for `bevy_render__msaa_writeback`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyMsaaWritebackParams {
    /// Which MSAA writeback mode to emit.
    pub variant: BevyMsaaWritebackVariant,
}

/// Named Bevy exposure presets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyExposurePreset {
    /// Match bright outdoor sunlight.
    Sunlight,
    /// Match overcast outdoor lighting.
    Overcast,
    /// Match indoor lighting.
    Indoor,
    /// Match Blender's default exposure.
    Blender,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default)]
struct BevyExposureFields {
    /// Optional named exposure preset.
    #[serde(default)]
    preset: Option<BevyExposurePreset>,
    /// Optional explicit EV100 value.
    #[serde(default)]
    ev100: Option<f32>,
}

/// Parameters for `bevy_render__exposure`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::Exposure", transparent)]
pub struct BevyExposureParams {
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "exposure_tokens")]
    fields: BevyExposureFields,
}

/// Parameters for `bevy_render__clear_color`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::ClearColor", transparent)]
pub struct BevyClearColorParams {
    /// Optional color expression overriding Bevy's default clear color.
    #[serde(default)]
    #[to_code_literal(to_tokens = "clear_color_tokens")]
    pub color_expr: Option<String>,
}

/// Supported camera depth-load operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
pub enum BevyCamera3dDepthLoadOpVariant {
    /// Clear the depth buffer, optionally with a custom depth.
    #[default]
    Clear,
    /// Load the existing depth buffer contents.
    Load,
}

/// Parameters for `bevy_render__camera_3d_depth_load_op`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::camera::Camera3dDepthLoadOp")]
pub enum BevyCamera3dDepthLoadOpParams {
    /// Clear the depth buffer, optionally with a custom depth.
    #[to_code_literal(tuple)]
    Clear {
        /// Clear value when `variant = clear`.
        #[serde(default = "default_camera_3d_depth_clear")]
        depth: f32,
    },
    /// Load the existing depth buffer contents.
    Load,
}

impl Default for BevyCamera3dDepthLoadOpParams {
    fn default() -> Self {
        Self::Clear {
            depth: default_camera_3d_depth_clear(),
        }
    }
}

/// Supported screen-space transmission quality levels.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    Default,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::camera::ScreenSpaceTransmissionQuality")]
pub enum BevyScreenSpaceTransmissionQualityVariant {
    /// Lowest quality.
    Low,
    /// Medium quality.
    #[default]
    Medium,
    /// High quality.
    High,
    /// Ultra quality.
    Ultra,
}

/// Parameters for `bevy_render__screen_space_transmission_quality`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyScreenSpaceTransmissionQualityParams {
    /// Which transmission quality to emit.
    pub variant: BevyScreenSpaceTransmissionQualityVariant,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct BevyMainPassResolutionOverrideFields {
    /// Override width in physical pixels.
    width: u32,
    /// Override height in physical pixels.
    height: u32,
}

/// Parameters for `bevy_render__main_pass_resolution_override`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::MainPassResolutionOverride", transparent)]
pub struct BevyMainPassResolutionOverrideParams {
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "main_pass_resolution_override_tokens")]
    fields: BevyMainPassResolutionOverrideFields,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct BevySubCameraViewFields {
    /// Total logical width of the full multi-camera image.
    full_width: u32,
    /// Total logical height of the full multi-camera image.
    full_height: u32,
    /// Horizontal offset of this sub view.
    offset_x: f32,
    /// Vertical offset of this sub view.
    offset_y: f32,
    /// Width of the sub view.
    width: u32,
    /// Height of the sub view.
    height: u32,
}

/// Parameters for `bevy_render__sub_camera_view`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::SubCameraView", transparent)]
pub struct BevySubCameraViewParams {
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "sub_camera_view_tokens")]
    fields: BevySubCameraViewFields,
}

/// Parameters for `bevy_render__no_cpu_culling`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::visibility::NoCpuCulling")]
pub struct BevyNoCpuCullingParams;

/// Parameters for `bevy_render__no_frustum_culling`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::visibility::NoFrustumCulling")]
pub struct BevyNoFrustumCullingParams;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct BevyVisibilityRangeFields {
    /// Start distance where the near fade begins.
    start_margin_start: f32,
    /// Distance where the near fade completes.
    start_margin_end: f32,
    /// Distance where the far fade begins.
    end_margin_start: f32,
    /// Distance where the far fade completes.
    end_margin_end: f32,
    /// Whether to use the mesh AABB center instead of the origin.
    #[serde(default)]
    use_aabb: Option<bool>,
}

/// Parameters for `bevy_render__visibility_range`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::visibility::VisibilityRange", transparent)]
pub struct BevyVisibilityRangeParams {
    #[serde(flatten)]
    #[to_code_literal(to_tokens = "visibility_range_tokens")]
    fields: BevyVisibilityRangeFields,
}

/// Parameters for `bevy_render__color`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyColorParams {
    /// Structured Bevy color-space payload.
    pub color: BevyColor,
}

/// Parameters for `bevy_render__alpha_mode`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::render::alpha::AlphaMode")]
pub enum BevyRenderAlphaModeParams {
    /// `AlphaMode::Opaque`
    Opaque,
    /// `AlphaMode::Mask(threshold)`
    #[to_code_literal(tuple)]
    Mask {
        /// Mask threshold used when `variant = mask`.
        #[serde(default = "default_render_alpha_mode_threshold")]
        threshold: f32,
    },
    /// `AlphaMode::Blend`
    Blend,
    /// `AlphaMode::Premultiplied`
    Premultiplied,
    /// `AlphaMode::AlphaToCoverage`
    AlphaToCoverage,
    /// `AlphaMode::Add`
    Add,
    /// `AlphaMode::Multiply`
    Multiply,
}

/// Supported Bevy UV channels.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::pbr::UvChannel")]
pub enum BevyUvChannelVariant {
    /// Use the first UV set.
    Uv0,
    /// Use the second UV set.
    Uv1,
}

/// Parameters for `bevy_render__uv_channel`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyUvChannelParams {
    /// Which Bevy UV channel to emit.
    pub variant: BevyUvChannelVariant,
}

/// Supported Bevy parallax mapping methods.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BevyParallaxMappingMethodVariant {
    /// Use fast occlusion mapping.
    Occlusion,
    /// Use relief mapping with a configurable maximum step count.
    Relief,
}

/// Parameters for `bevy_render__parallax_mapping_method`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::pbr::ParallaxMappingMethod")]
pub enum BevyParallaxMappingMethodParams {
    /// Use fast occlusion mapping.
    Occlusion,
    /// Use relief mapping with a configurable maximum step count.
    Relief {
        /// Maximum step count for the relief-mapping solver.
        #[serde(default = "default_parallax_relief_max_steps")]
        max_steps: u32,
    },
}

/// Supported Bevy opaque renderer methods.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::pbr::OpaqueRendererMethod")]
pub enum BevyOpaqueRendererMethodVariant {
    /// Let Bevy pick based on the default resource.
    Auto,
    /// Force forward rendering.
    Forward,
    /// Force deferred rendering.
    Deferred,
}

/// Parameters for `bevy_render__opaque_renderer_method`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyOpaqueRendererMethodParams {
    /// Which opaque renderer method to emit.
    pub variant: BevyOpaqueRendererMethodVariant,
}

/// Supported constructors for `DefaultOpaqueRendererMethod`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BevyDefaultOpaqueRendererMethodVariant {
    /// Use Bevy's default constructor.
    Default,
    /// Force forward rendering as the resource default.
    Forward,
    /// Force deferred rendering as the resource default.
    Deferred,
}

/// Parameters for `bevy_render__default_opaque_renderer_method`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::DefaultOpaqueRendererMethod", transparent)]
pub struct BevyDefaultOpaqueRendererMethodParams {
    /// Which resource constructor to emit.
    #[to_code_literal(to_tokens = "default_opaque_renderer_method_tokens")]
    pub variant: BevyDefaultOpaqueRendererMethodVariant,
}

/// Supported alpha modes for 2D materials.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BevyAlphaMode2dVariant {
    /// Force fully opaque output.
    Opaque,
    /// Use alpha cutoff masking.
    Mask,
    /// Use standard alpha blending.
    Blend,
}

/// Parameters for `bevy_render__alpha_mode_2d`.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, elicitation::ToCodeLiteral,
)]
#[serde(tag = "variant", content = "threshold", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::sprite_render::AlphaMode2d")]
pub enum BevyAlphaMode2dParams {
    /// Force fully opaque output.
    Opaque,
    /// Use alpha cutoff masking.
    Mask(f32),
    /// Use standard alpha blending.
    Blend,
}

/// Parameters for `bevy_render__standard_material`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::StandardMaterial", default_update)]
pub struct BevyStandardMaterialParams {
    /// Optional base-color expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "base_color", expr)]
    pub base_color_expr: Option<String>,
    /// Optional UV channel for the base-color texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub base_color_channel: Option<BevyUvChannelVariant>,
    /// Optional base-color texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "base_color_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub base_color_texture_expr: Option<String>,
    /// Optional emissive color expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "emissive", expr)]
    pub emissive_expr: Option<String>,
    /// Optional UV channel for the emissive texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub emissive_channel: Option<BevyUvChannelVariant>,
    /// Optional emissive texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "emissive_texture", to_tokens = "some_expr_tokens")]
    pub emissive_texture_expr: Option<String>,
    /// Optional emissive exposure multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub emissive_exposure_weight: Option<f32>,
    /// Optional metallic factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub metallic: Option<f32>,
    /// Optional perceptual roughness factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub perceptual_roughness: Option<f32>,
    /// Optional UV channel for the metallic/roughness texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub metallic_roughness_channel: Option<BevyUvChannelVariant>,
    /// Optional metallic/roughness texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "metallic_roughness_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub metallic_roughness_texture_expr: Option<String>,
    /// Optional reflectance factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub reflectance: Option<f32>,
    /// Optional specular tint expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "specular_tint", expr)]
    pub specular_tint_expr: Option<String>,
    /// Optional diffuse transmission factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub diffuse_transmission: Option<f32>,
    /// Optional UV channel for the diffuse transmission texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub diffuse_transmission_channel: Option<BevyUvChannelVariant>,
    /// Optional diffuse transmission texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "diffuse_transmission_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub diffuse_transmission_texture_expr: Option<String>,
    /// Optional specular transmission factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub specular_transmission: Option<f32>,
    /// Optional UV channel for the specular transmission texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub specular_transmission_channel: Option<BevyUvChannelVariant>,
    /// Optional specular transmission texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "specular_transmission_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub specular_transmission_texture_expr: Option<String>,
    /// Optional volume thickness.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub thickness: Option<f32>,
    /// Optional UV channel for the thickness texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub thickness_channel: Option<BevyUvChannelVariant>,
    /// Optional thickness texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "thickness_texture", to_tokens = "some_expr_tokens")]
    pub thickness_texture_expr: Option<String>,
    /// Optional index of refraction.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub ior: Option<f32>,
    /// Optional attenuation distance.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub attenuation_distance: Option<f32>,
    /// Optional attenuation color expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "attenuation_color", expr)]
    pub attenuation_color_expr: Option<String>,
    /// Optional clearcoat strength.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub clearcoat: Option<f32>,
    /// Optional UV channel for the clearcoat texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub clearcoat_channel: Option<BevyUvChannelVariant>,
    /// Optional clearcoat texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "clearcoat_texture", to_tokens = "some_expr_tokens")]
    pub clearcoat_texture_expr: Option<String>,
    /// Optional clearcoat roughness.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub clearcoat_perceptual_roughness: Option<f32>,
    /// Optional UV channel for the clearcoat roughness texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub clearcoat_roughness_channel: Option<BevyUvChannelVariant>,
    /// Optional clearcoat roughness texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "clearcoat_roughness_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub clearcoat_roughness_texture_expr: Option<String>,
    /// Optional UV channel for the clearcoat normal texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub clearcoat_normal_channel: Option<BevyUvChannelVariant>,
    /// Optional clearcoat normal texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "clearcoat_normal_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub clearcoat_normal_texture_expr: Option<String>,
    /// Optional anisotropy strength.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub anisotropy_strength: Option<f32>,
    /// Optional anisotropy rotation in radians.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub anisotropy_rotation: Option<f32>,
    /// Optional UV channel for the anisotropy texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub anisotropy_channel: Option<BevyUvChannelVariant>,
    /// Optional anisotropy texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "anisotropy_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub anisotropy_texture_expr: Option<String>,
    /// Optional UV channel for the normal map texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub normal_map_channel: Option<BevyUvChannelVariant>,
    /// Optional normal-map texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "normal_map_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub normal_map_texture_expr: Option<String>,
    /// Optional flag for DirectX-authored normal maps.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub flip_normal_map_y: Option<bool>,
    /// Optional UV channel for the occlusion texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub occlusion_channel: Option<BevyUvChannelVariant>,
    /// Optional occlusion texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "occlusion_texture", to_tokens = "some_expr_tokens")]
    pub occlusion_texture_expr: Option<String>,
    /// Optional UV channel for the specular texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub specular_channel: Option<BevyUvChannelVariant>,
    /// Optional specular texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "specular_texture", to_tokens = "some_expr_tokens")]
    pub specular_texture_expr: Option<String>,
    /// Optional UV channel for the specular tint texture.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_uv_channel_variant_tokens")]
    pub specular_tint_channel: Option<BevyUvChannelVariant>,
    /// Optional specular tint texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        optional,
        rename = "specular_tint_texture",
        to_tokens = "some_expr_tokens"
    )]
    pub specular_tint_texture_expr: Option<String>,
    /// Optional alpha-mode expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "alpha_mode", expr)]
    pub alpha_mode_expr: Option<String>,
    /// Whether the material is double-sided.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub double_sided: Option<bool>,
    /// Whether the material is unlit.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub unlit: Option<bool>,
    /// Whether the material participates in fog.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub fog_enabled: Option<bool>,
    /// Optional cull mode expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "cull_mode", to_tokens = "some_expr_tokens")]
    pub cull_mode_expr: Option<String>,
    /// Optional constant depth bias.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub depth_bias: Option<f32>,
    /// Optional depth-map texture handle expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "depth_map", to_tokens = "some_expr_tokens")]
    pub depth_map_expr: Option<String>,
    /// Optional parallax depth scale.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub parallax_depth_scale: Option<f32>,
    /// Optional parallax mapping method expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "parallax_mapping_method", expr)]
    pub parallax_mapping_method_expr: Option<String>,
    /// Optional parallax layer count ceiling.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub max_parallax_layer_count: Option<f32>,
    /// Optional lightmap exposure scale.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub lightmap_exposure: Option<f32>,
    /// Optional opaque renderer method expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "opaque_render_method", expr)]
    pub opaque_render_method_expr: Option<String>,
    /// Optional deferred lighting pass identifier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub deferred_lighting_pass_id: Option<u8>,
    /// Optional UV transform expression.
    #[serde(default)]
    #[to_code_literal(optional, rename = "uv_transform", expr)]
    pub uv_transform_expr: Option<String>,
}

/// Parameters for `bevy_render__lightmap`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::Lightmap", default_update)]
pub struct BevyLightmapParams {
    /// Expression that yields the `Handle<Image>` for the baked lightmap.
    #[to_code_literal(rename = "image", expr)]
    pub image_expr: String,
    /// Optional `Rect` expression describing the atlas region.
    #[serde(default)]
    #[to_code_literal(rename = "uv_rect", expr, optional)]
    pub uv_rect_expr: Option<String>,
    /// Whether bicubic sampling should be enabled.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub bicubic_sampling: Option<bool>,
}

/// Parameters for `bevy_render__color_material`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::ColorMaterial", default_update)]
pub struct BevyColorMaterialParams {
    /// Optional tint color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional `AlphaMode2d` expression.
    #[serde(default)]
    #[to_code_literal(rename = "alpha_mode", expr, optional)]
    pub alpha_mode_expr: Option<String>,
    /// Optional UV transform expression.
    #[serde(default)]
    #[to_code_literal(rename = "uv_transform", expr, optional)]
    pub uv_transform_expr: Option<String>,
    /// Optional texture handle expression.
    #[serde(default)]
    #[to_code_literal(rename = "texture", optional, to_tokens = "some_expr_tokens")]
    pub texture_expr: Option<String>,
}

/// Parameters for `bevy_render__skybox`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::Skybox", default_update)]
pub struct BevySkyboxParams {
    /// Cubemap image handle expression.
    #[to_code_literal(rename = "image", expr)]
    pub image_expr: String,
    /// Optional brightness multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub brightness: Option<f32>,
    /// Optional rotation expression.
    #[serde(default)]
    #[to_code_literal(rename = "rotation", expr, optional)]
    pub rotation_expr: Option<String>,
}

/// Parameters for `bevy_render__depth_prepass`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::DepthPrepass")]
pub struct BevyDepthPrepassParams;

/// Parameters for `bevy_render__normal_prepass`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::NormalPrepass")]
pub struct BevyNormalPrepassParams;

/// Parameters for `bevy_render__motion_vector_prepass`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::MotionVectorPrepass")]
pub struct BevyMotionVectorPrepassParams;

/// Parameters for `bevy_render__deferred_prepass`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::DeferredPrepass")]
pub struct BevyDeferredPrepassParams;

/// Parameters for `bevy_render__depth_prepass_double_buffer`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer")]
pub struct BevyDepthPrepassDoubleBufferParams;

/// Parameters for `bevy_render__deferred_prepass_double_buffer`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer")]
pub struct BevyDeferredPrepassDoubleBufferParams;

/// Parameters for `bevy_render__ambient_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::AmbientLight", default_update)]
pub struct BevyAmbientLightParams {
    /// Optional color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional brightness multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub brightness: Option<f32>,
    /// Optional flag controlling whether the ambient light affects meshes with lightmaps.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_meshes: Option<bool>,
}

/// Parameters for `bevy_render__directional_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::DirectionalLight", default_update)]
pub struct BevyDirectionalLightParams {
    /// Optional color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional illuminance in lux.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub illuminance: Option<f32>,
    /// Optional shadows-enabled flag.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub shadows_enabled: Option<bool>,
}

/// Parameters for `bevy_render__global_ambient_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::GlobalAmbientLight", default_update)]
pub struct BevyGlobalAmbientLightParams {
    /// Optional color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional brightness multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub brightness: Option<f32>,
    /// Optional flag controlling whether the ambient light affects meshes with lightmaps.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_meshes: Option<bool>,
}

/// Parameters for `bevy_render__point_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::PointLight", default_update)]
pub struct BevyPointLightParams {
    /// Optional color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional light intensity.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional light range.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub range: Option<f32>,
    /// Optional radius.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub radius: Option<f32>,
    /// Optional shadows-enabled flag.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub shadows_enabled: Option<bool>,
}

/// Parameters for `bevy_render__spot_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::SpotLight", default_update)]
pub struct BevySpotLightParams {
    /// Optional color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", expr, optional)]
    pub color_expr: Option<String>,
    /// Optional light intensity.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional light range.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub range: Option<f32>,
    /// Optional radius.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub radius: Option<f32>,
    /// Optional inner-cone angle in radians.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub inner_angle: Option<f32>,
    /// Optional outer-cone angle in radians.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub outer_angle: Option<f32>,
    /// Optional shadows-enabled flag.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub shadows_enabled: Option<bool>,
}

/// Parameters for `bevy_render__directional_light_shadow_map`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::light::DirectionalLightShadowMap",
    default_expr = "::bevy::light::DirectionalLightShadowMap::default()"
)]
pub struct BevyDirectionalLightShadowMapParams {
    /// Optional width/height for each directional-light cascade shadow map.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub size: Option<usize>,
}

/// Parameters for `bevy_render__point_light_shadow_map`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::light::PointLightShadowMap",
    default_expr = "::bevy::light::PointLightShadowMap::default()"
)]
pub struct BevyPointLightShadowMapParams {
    /// Optional width/height for each point-light cubemap face shadow map.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub size: Option<usize>,
}

/// Parameters for `bevy_render__environment_map_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::EnvironmentMapLight", default_update)]
pub struct BevyEnvironmentMapLightParams {
    /// Expression that yields the diffuse cubemap handle.
    #[to_code_literal(rename = "diffuse_map", expr)]
    pub diffuse_map_expr: String,
    /// Expression that yields the specular cubemap handle.
    #[to_code_literal(rename = "specular_map", expr)]
    pub specular_map_expr: String,
    /// Optional radiance scale factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional world-space rotation expression.
    #[serde(default)]
    #[to_code_literal(rename = "rotation", expr, optional)]
    pub rotation_expr: Option<String>,
    /// Optional flag controlling diffuse contribution on lightmapped meshes.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_mesh_diffuse: Option<bool>,
}

/// Parameters for `bevy_render__generated_environment_map_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::GeneratedEnvironmentMapLight", default_update)]
pub struct BevyGeneratedEnvironmentMapLightParams {
    /// Expression that yields the source cubemap handle.
    #[to_code_literal(rename = "environment_map", expr)]
    pub environment_map_expr: String,
    /// Optional radiance scale factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional world-space rotation expression.
    #[serde(default)]
    #[to_code_literal(rename = "rotation", expr, optional)]
    pub rotation_expr: Option<String>,
    /// Optional flag controlling diffuse contribution on lightmapped meshes.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_mesh_diffuse: Option<bool>,
}

/// Parameters for `bevy_render__atmosphere_environment_map_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::light::AtmosphereEnvironmentMapLight",
    default_update,
    default_expr = "::bevy::light::AtmosphereEnvironmentMapLight::default()"
)]
pub struct BevyAtmosphereEnvironmentMapLightParams {
    /// Optional brightness multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional flag controlling diffuse contribution on lightmapped meshes.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_mesh_diffuse: Option<bool>,
    /// Optional cubemap resolution in pixels.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "uvec2_array_tokens")]
    pub size: Option<[u32; 2]>,
}

/// Parameters for `bevy_render__volumetric_light`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::VolumetricLight")]
pub struct BevyVolumetricLightParams;

/// Parameters for `bevy_render__volumetric_fog`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::VolumetricFog", default_update)]
pub struct BevyVolumetricFogParams {
    /// Optional ambient-light color expression.
    #[serde(default)]
    #[to_code_literal(rename = "ambient_color", expr, optional)]
    pub ambient_color_expr: Option<String>,
    /// Optional ambient intensity.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub ambient_intensity: Option<f32>,
    /// Optional ray-origin jitter distance in meters.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub jitter: Option<f32>,
    /// Optional raymarch step count.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub step_count: Option<u32>,
}

/// Parameters for `bevy_render__fog_volume`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::FogVolume", default_update)]
pub struct BevyFogVolumeParams {
    /// Optional fog color expression.
    #[serde(default)]
    #[to_code_literal(rename = "fog_color", expr, optional)]
    pub fog_color_expr: Option<String>,
    /// Optional density factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub density_factor: Option<f32>,
    /// Optional density-texture handle expression.
    #[serde(default)]
    #[to_code_literal(rename = "density_texture", optional, to_tokens = "some_expr_tokens")]
    pub density_texture_expr: Option<String>,
    /// Optional density-texture offset expression.
    #[serde(default)]
    #[to_code_literal(rename = "density_texture_offset", expr, optional)]
    pub density_texture_offset_expr: Option<String>,
    /// Optional absorption coefficient.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub absorption: Option<f32>,
    /// Optional scattering coefficient.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub scattering: Option<f32>,
    /// Optional anisotropy factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub scattering_asymmetry: Option<f32>,
    /// Optional nonphysical light tint expression.
    #[serde(default)]
    #[to_code_literal(rename = "light_tint", expr, optional)]
    pub light_tint_expr: Option<String>,
    /// Optional nonphysical light intensity scale.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub light_intensity: Option<f32>,
}

/// Parameters for `bevy_render__light_probe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::LightProbe")]
pub struct BevyLightProbeParams;

/// Parameters for `bevy_render__irradiance_volume`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::IrradianceVolume", default_update)]
pub struct BevyIrradianceVolumeParams {
    /// Expression that yields the irradiance voxel texture handle.
    #[to_code_literal(rename = "voxels", expr)]
    pub voxels_expr: String,
    /// Optional radiance scale factor.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
    /// Optional flag controlling whether the probe affects lightmapped meshes.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub affects_lightmapped_meshes: Option<bool>,
}

/// Parameters for `bevy_render__sun_disk`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::SunDisk", default_update)]
pub struct BevySunDiskParams {
    /// Optional angular size in radians.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub angular_size: Option<f32>,
    /// Optional brightness multiplier.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub intensity: Option<f32>,
}

/// Parameters for `bevy_render__not_shadow_caster`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::NotShadowCaster")]
pub struct BevyNotShadowCasterParams;

/// Parameters for `bevy_render__not_shadow_receiver`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::NotShadowReceiver")]
pub struct BevyNotShadowReceiverParams;

/// Parameters for `bevy_render__transmitted_shadow_receiver`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::TransmittedShadowReceiver")]
pub struct BevyTransmittedShadowReceiverParams;

/// Supported `ShadowFilteringMethod` variants.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::light::ShadowFilteringMethod")]
pub enum BevyShadowFilteringMethodVariant {
    /// `ShadowFilteringMethod::Hardware2x2`
    Hardware2x2,
    /// `ShadowFilteringMethod::Gaussian`
    Gaussian,
    /// `ShadowFilteringMethod::Temporal`
    Temporal,
}

/// Parameters for `bevy_render__shadow_filtering_method`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyShadowFilteringMethodParams {
    /// Filtering mode to emit.
    pub variant: BevyShadowFilteringMethodVariant,
}

/// Supported `ClusterFarZMode` variants.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyClusterFarZModeKind {
    /// `ClusterFarZMode::MaxClusterableObjectRange`
    MaxClusterableObjectRange,
    /// `ClusterFarZMode::Constant(value)`
    Constant,
}

/// Parameters for `bevy_render__cluster_far_z_mode`.
#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, elicitation::ToCodeLiteral,
)]
#[serde(tag = "kind", content = "constant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::light::ClusterFarZMode")]
pub enum BevyClusterFarZModeParams {
    /// Use Bevy's max-clusterable-object-range heuristic.
    MaxClusterableObjectRange,
    /// Use a fixed far-z value.
    Constant(f32),
}

/// Parameters for `bevy_render__cluster_z_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::light::ClusterZConfig",
    default_update,
    default_expr = "::std::default::Default::default()"
)]
pub struct BevyClusterZConfigParams {
    /// Optional far plane for the first slice.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub first_slice_depth: Option<f32>,
    /// Optional far-z strategy override.
    #[serde(default)]
    #[to_code_literal(rename = "far_z_mode", optional, to_tokens = "emit_code_tokens")]
    pub far_z_mode: Option<BevyClusterFarZModeParams>,
}

/// Parameters for `bevy_render__cluster_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::light::ClusterConfig")]
pub enum BevyClusterConfigParams {
    /// `ClusterConfig::None`
    None,
    /// `ClusterConfig::Single`
    Single,
    /// `ClusterConfig::XYZ { .. }`
    #[serde(rename = "xyz")]
    XYZ {
        /// Explicit cluster dimensions for the XYZ grid.
        #[to_code_literal(to_tokens = "uvec3_array_tokens")]
        dimensions: [u32; 3],
        /// Optional z-configuration override.
        #[serde(default)]
        z_config: BevyClusterZConfigParams,
        /// Whether the cluster grid may dynamically resize.
        #[serde(default = "default_true")]
        dynamic_resizing: bool,
    },
    /// `ClusterConfig::FixedZ { .. }`
    FixedZ {
        /// Maximum total clusters.
        total: u32,
        /// Number of z-slices.
        z_slices: u32,
        /// Optional z-configuration override.
        #[serde(default)]
        z_config: BevyClusterZConfigParams,
        /// Whether the cluster grid may dynamically resize.
        #[serde(default = "default_true")]
        dynamic_resizing: bool,
    },
}

/// Parameters for `bevy_render__screen_space_reflections`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::ScreenSpaceReflections", default_update)]
pub struct BevyScreenSpaceReflectionsParams {
    /// Optional roughness threshold.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub perceptual_roughness_threshold: Option<f32>,
    /// Optional assumed surface thickness.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub thickness: Option<f32>,
    /// Optional number of linear marching steps.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub linear_steps: Option<u32>,
    /// Optional exponent for the linear march distribution.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub linear_march_exponent: Option<f32>,
    /// Optional number of bisection steps.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub bisection_steps: Option<u32>,
    /// Optional secant-method toggle.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub use_secant: Option<bool>,
}

/// Parameters for `bevy_render__clustered_decal`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::ClusteredDecal", default_update)]
pub struct BevyClusteredDecalParams {
    /// Optional base-color decal texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        rename = "base_color_texture",
        optional,
        to_tokens = "some_expr_tokens"
    )]
    pub base_color_texture_expr: Option<String>,
    /// Optional normal-map decal texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        rename = "normal_map_texture",
        optional,
        to_tokens = "some_expr_tokens"
    )]
    pub normal_map_texture_expr: Option<String>,
    /// Optional metallic-roughness texture handle expression.
    #[serde(default)]
    #[to_code_literal(
        rename = "metallic_roughness_texture",
        optional,
        to_tokens = "some_expr_tokens"
    )]
    pub metallic_roughness_texture_expr: Option<String>,
    /// Optional emissive texture handle expression.
    #[serde(default)]
    #[to_code_literal(rename = "emissive_texture", optional, to_tokens = "some_expr_tokens")]
    pub emissive_texture_expr: Option<String>,
    /// Optional application-specific tag.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub tag: Option<u32>,
}

/// Parameters for `bevy_render__mesh_3d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::mesh::Mesh3d", tuple)]
pub struct BevyMesh3dParams {
    /// Expression that yields a `Handle<Mesh>`.
    #[to_code_literal(expr)]
    pub mesh_expr: String,
}

/// Parameters for `bevy_render__mesh_2d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::mesh::Mesh2d", tuple)]
pub struct BevyMesh2dParams {
    /// Expression that yields a `Handle<Mesh>`.
    #[to_code_literal(expr)]
    pub mesh_expr: String,
}

/// Parameters for `bevy_render__mesh_material_3d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::MeshMaterial3d", tuple)]
pub struct BevyMeshMaterial3dParams {
    /// Expression that yields a typed material handle.
    #[to_code_literal(expr)]
    pub material_expr: String,
}

/// Parameters for `bevy_render__mesh_material_2d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::MeshMaterial2d", tuple)]
pub struct BevyMeshMaterial2dParams {
    /// Expression that yields a typed 2D material handle.
    #[to_code_literal(expr)]
    pub material_expr: String,
}

/// Parameters for `bevy_render__wireframe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::Wireframe")]
pub struct BevyWireframeParams;

/// Parameters for `bevy_render__wireframe_color`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::WireframeColor")]
pub struct BevyWireframeColorParams {
    /// Expression that yields a `Color`.
    #[to_code_literal(rename = "color", expr)]
    pub color_expr: String,
}

/// Parameters for `bevy_render__no_wireframe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::NoWireframe")]
pub struct BevyNoWireframeParams;

/// Parameters for `bevy_render__wireframe_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::pbr::WireframeConfig",
    default_update,
    default_expr = "::bevy::pbr::WireframeConfig::default()"
)]
pub struct BevyWireframeConfigParams {
    /// Whether all eligible meshes should render in wireframe by default.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub global: Option<bool>,
    /// Override for the default wireframe color.
    #[serde(default)]
    #[to_code_literal(rename = "default_color", expr, optional)]
    pub default_color_expr: Option<String>,
}

/// Parameters for `bevy_render__mesh_3d_wireframe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::Mesh3dWireframe", tuple)]
pub struct BevyMesh3dWireframeParams {
    /// Expression that yields a `Handle<WireframeMaterial>`.
    #[to_code_literal(expr)]
    pub material_expr: String,
}

/// Parameters for `bevy_render__wireframe_2d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::Wireframe2d")]
pub struct BevyWireframe2dParams;

/// Parameters for `bevy_render__wireframe_2d_color`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::Wireframe2dColor")]
pub struct BevyWireframe2dColorParams {
    /// Expression that yields a `Color`.
    #[to_code_literal(rename = "color", expr)]
    pub color_expr: String,
}

/// Parameters for `bevy_render__no_wireframe_2d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::NoWireframe2d")]
pub struct BevyNoWireframe2dParams;

/// Parameters for `bevy_render__wireframe_2d_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::sprite_render::Wireframe2dConfig",
    default_update,
    default_expr = "::bevy::sprite_render::Wireframe2dConfig::default()"
)]
pub struct BevyWireframe2dConfigParams {
    /// Whether all eligible 2D meshes should render in wireframe by default.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub global: Option<bool>,
    /// Override for the default 2D wireframe color.
    #[serde(default)]
    #[to_code_literal(rename = "default_color", expr, optional)]
    pub default_color_expr: Option<String>,
}

/// Parameters for `bevy_render__mesh_2d_wireframe`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::sprite_render::Mesh2dWireframe", tuple)]
pub struct BevyMesh2dWireframeParams {
    /// Expression that yields a `Handle<Wireframe2dMaterial>`.
    #[to_code_literal(expr)]
    pub material_expr: String,
}

/// Parameters for `bevy_render__camera_3d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyCamera3dParams {
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Optional transform expression.
    #[serde(default)]
    pub transform_expr: Option<String>,
    /// Optional perspective FOV in radians.
    #[serde(default)]
    pub fov: Option<f32>,
    /// Optional near clip plane.
    #[serde(default)]
    pub near: Option<f32>,
    /// Optional far clip plane.
    #[serde(default)]
    pub far: Option<f32>,
    /// Optional explicit HDR override on `Camera`.
    #[serde(default)]
    pub hdr: Option<bool>,
    /// Optional render-target descriptor.
    #[serde(default)]
    pub render_target: Option<BevyRenderTargetParams>,
    /// Optional tonemapping expression.
    #[serde(default)]
    pub tonemapping_expr: Option<String>,
}

/// Parameters for `bevy_render__camera_2d`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyCamera2dParams {
    /// Commands-like receiver expression.
    #[serde(default = "default_commands_var")]
    pub commands_var: String,
    /// Optional transform expression.
    #[serde(default)]
    pub transform_expr: Option<String>,
    /// Optional orthographic near clip plane.
    #[serde(default)]
    pub near: Option<f32>,
    /// Optional orthographic far clip plane.
    #[serde(default)]
    pub far: Option<f32>,
    /// Optional orthographic scale.
    #[serde(default)]
    pub scale: Option<f32>,
    /// Optional explicit HDR override on `Camera`.
    #[serde(default)]
    pub hdr: Option<bool>,
    /// Optional render-target descriptor.
    #[serde(default)]
    pub render_target: Option<BevyRenderTargetParams>,
    /// Optional tonemapping expression.
    #[serde(default)]
    pub tonemapping_expr: Option<String>,
}

/// Parameters for `bevy_render__viewport`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::camera::Viewport")]
pub struct BevyViewportParams {
    /// Physical top-left position in pixels.
    #[to_code_literal(to_tokens = "uvec2_array_tokens")]
    pub physical_position: [u32; 2],
    /// Physical size in pixels.
    #[to_code_literal(to_tokens = "uvec2_array_tokens")]
    pub physical_size: [u32; 2],
    /// Optional depth range `[min, max]`.
    #[serde(default)]
    #[to_code_literal(to_tokens = "viewport_depth_tokens")]
    pub depth: Option<[f32; 2]>,
}

/// Parameters for `bevy_render__fog_settings`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::DistanceFog", default_update)]
pub struct BevyFogSettingsParams {
    /// Optional fog color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", optional, to_tokens = "into_expr_tokens")]
    pub color_expr: Option<String>,
    /// Optional directional-light contribution color expression.
    #[serde(default)]
    #[to_code_literal(
        rename = "directional_light_color",
        optional,
        to_tokens = "into_expr_tokens"
    )]
    pub directional_light_color_expr: Option<String>,
    /// Optional directional-light exponent.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub directional_light_exponent: Option<f32>,
    /// Optional `FogFalloff` expression.
    #[serde(default)]
    #[to_code_literal(rename = "falloff", expr, optional)]
    pub falloff_expr: Option<String>,
}

/// Preset bloom configurations available in Bevy 0.18.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyBloomPreset {
    /// `Bloom::NATURAL`
    Natural,
    /// `Bloom::ANAMORPHIC`
    Anamorphic,
    /// `Bloom::OLD_SCHOOL`
    OldSchool,
    /// `Bloom::SCREEN_BLUR`
    ScreenBlur,
}

/// Supported bloom composite modes.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    JsonSchema,
    PartialEq,
    Eq,
    elicitation::ToCodeLiteral,
)]
#[serde(rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::post_process::bloom::BloomCompositeMode")]
pub enum BevyBloomCompositeModeVariant {
    /// `BloomCompositeMode::EnergyConserving`
    EnergyConserving,
    /// `BloomCompositeMode::Additive`
    Additive,
}

/// Parameters for `bevy_render__bloom_composite_mode`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
pub struct BevyBloomCompositeModeParams {
    /// Bloom composite mode to emit.
    pub variant: BevyBloomCompositeModeVariant,
}

/// Parameters for `bevy_render__bloom_settings`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyBloomSettingsParams {
    /// Optional starting preset.
    #[serde(default)]
    pub preset: Option<BevyBloomPreset>,
    /// Optional bloom intensity override.
    #[serde(default)]
    pub intensity: Option<f32>,
    /// Optional low-frequency boost override.
    #[serde(default)]
    pub low_frequency_boost: Option<f32>,
    /// Optional low-frequency boost curvature override.
    #[serde(default)]
    pub low_frequency_boost_curvature: Option<f32>,
    /// Optional high-pass frequency override.
    #[serde(default)]
    pub high_pass_frequency: Option<f32>,
    /// Optional bloom prefilter threshold override.
    #[serde(default)]
    pub prefilter_threshold: Option<f32>,
    /// Optional bloom prefilter threshold softness override.
    #[serde(default)]
    pub prefilter_threshold_softness: Option<f32>,
    /// Optional bloom composite mode override.
    #[serde(default)]
    pub composite_mode: Option<BevyBloomCompositeModeVariant>,
    /// Optional max-mip-dimension override.
    #[serde(default)]
    pub max_mip_dimension: Option<u32>,
    /// Optional `Vec2` scale expression.
    #[serde(default)]
    pub scale_expr: Option<String>,
}

/// Parameters for `bevy_render__sprite`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::sprite::Sprite",
    default_update,
    default_expr = "::bevy::sprite::Sprite { ..::std::default::Default::default() }"
)]
pub struct BevySpriteParams {
    /// Optional image handle expression.
    #[serde(default)]
    #[to_code_literal(rename = "image", expr, optional)]
    pub image_expr: Option<String>,
    /// Optional tint color expression.
    #[serde(default)]
    #[to_code_literal(rename = "color", optional, to_tokens = "into_expr_tokens")]
    pub color_expr: Option<String>,
    /// Whether the sprite should be flipped on X.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub flip_x: Option<bool>,
    /// Whether the sprite should be flipped on Y.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub flip_y: Option<bool>,
    /// Optional custom-size expression.
    #[serde(default)]
    #[to_code_literal(rename = "custom_size", optional, to_tokens = "some_expr_tokens")]
    pub custom_size_expr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::text::TextFont",
    default_update,
    default_expr = "::bevy::text::TextFont::default()"
)]
struct BevyTextFontParams {
    /// Optional font handle expression.
    #[serde(default)]
    #[to_code_literal(rename = "font", expr, optional)]
    font_handle_expr: Option<String>,
    /// Optional font size in logical pixels.
    #[serde(default)]
    #[to_code_literal(optional)]
    font_size: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
struct BevyTextColorParams {
    /// Optional text color expression.
    #[serde(default)]
    #[to_code_literal(to_tokens = "text_color_tokens")]
    color_expr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Default, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::text::TextLayout",
    default_update,
    default_expr = "::bevy::text::TextLayout::default()"
)]
struct BevyTextLayoutParams {
    /// Optional `Justify` expression.
    #[serde(default)]
    #[to_code_literal(rename = "justify", expr, optional)]
    justify_expr: Option<String>,
    /// Optional `LineBreak` expression.
    #[serde(default)]
    #[to_code_literal(rename = "linebreak", expr, optional)]
    linebreak_expr: Option<String>,
}

/// Parameters for `bevy_render__text_style`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(raw_tuple)]
pub struct BevyTextStyleParams {
    /// Font tuple fragment fields.
    #[serde(flatten)]
    font: BevyTextFontParams,
    /// Color tuple fragment fields.
    #[serde(flatten)]
    color: BevyTextColorParams,
    /// Layout tuple fragment fields.
    #[serde(flatten)]
    layout: BevyTextLayoutParams,
}

/// SSAO quality variants in Bevy 0.18.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevySsaoQualityVariant {
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Low`
    Low,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Medium`
    Medium,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::High`
    High,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Ultra`
    Ultra,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Custom { .. }`
    Custom,
}

/// Parameters for `bevy_render__ssao_quality`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[serde(tag = "variant", rename_all = "snake_case")]
#[to_code_literal(path = "::bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel")]
pub enum BevySsaoQualityParams {
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Low`
    Low,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Medium`
    Medium,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::High`
    High,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Ultra`
    Ultra,
    /// `ScreenSpaceAmbientOcclusionQualityLevel::Custom { .. }`
    Custom {
        /// Custom slice count.
        slice_count: u32,
        /// Custom samples-per-slice-side.
        samples_per_slice_side: u32,
    },
}

/// Parameters for `bevy_render__ssao`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::pbr::ScreenSpaceAmbientOcclusion", default_update)]
pub struct BevySsaoParams {
    /// Optional SSAO quality-level payload.
    #[serde(default)]
    #[to_code_literal(optional, to_tokens = "emit_code_tokens")]
    pub quality_level: Option<BevySsaoQualityParams>,
    /// Optional constant estimated object thickness.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub constant_object_thickness: Option<f32>,
}

/// Parameters for `bevy_render__temporal_anti_aliasing`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::anti_alias::taa::TemporalAntiAliasing", default_update)]
pub struct BevyTemporalAntiAliasingParams {
    /// Optional reset flag; defaults to Bevy's `TemporalAntiAliasing::default()`.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub reset: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(transparent)]
struct BevyCascadeShadowBoundsParams(
    #[to_code_literal(to_tokens = "cascade_bounds_tokens")] Vec<f32>,
);

/// Parameters for `bevy_render__cascade_shadow_config`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(
    path = "::bevy::light::CascadeShadowConfig",
    default_update,
    default_expr = "::bevy::light::CascadeShadowConfig::default()"
)]
pub struct BevyCascadeShadowConfigParams {
    /// Optional explicit cascade far bounds.
    #[serde(
        default,
        deserialize_with = "deserialize_optional_cascade_bounds",
        skip_serializing_if = "Option::is_none"
    )]
    #[to_code_literal(optional)]
    bounds: Option<BevyCascadeShadowBoundsParams>,
    /// Optional overlap proportion between cascades.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub overlap_proportion: Option<f32>,
    /// Optional near boundary of the first cascade.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub minimum_distance: Option<f32>,
}

/// Parameters for `bevy_render__cascade_shadow_config_builder`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, elicitation::ToCodeLiteral)]
#[to_code_literal(path = "::bevy::light::CascadeShadowConfigBuilder", default_update)]
pub struct BevyCascadeShadowConfigBuilderParams {
    /// Optional number of cascades.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub num_cascades: Option<usize>,
    /// Optional minimum shadow distance.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub minimum_distance: Option<f32>,
    /// Optional maximum shadow distance.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub maximum_distance: Option<f32>,
    /// Optional far bound of the first cascade.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub first_cascade_far_bound: Option<f32>,
    /// Optional overlap proportion between cascades.
    #[serde(default)]
    #[to_code_literal(optional)]
    pub overlap_proportion: Option<f32>,
}

/// Render-graph family used by `bevy_render__fullscreen_material`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BevyFullscreenGraphKind {
    /// Generate `Node2d` post-processing edges.
    Core2d,
    /// Generate `Node3d` post-processing edges.
    Core3d,
}

impl Default for BevyFullscreenGraphKind {
    fn default() -> Self {
        Self::Core3d
    }
}

fn default_parallax_relief_max_steps() -> u32 {
    32
}

fn default_render_alpha_mode_threshold() -> f32 {
    0.5
}

fn default_camera_3d_depth_clear() -> f32 {
    0.0
}

/// Parameters for `bevy_render__atmosphere`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyAtmosphereParams {
    /// `Assets<ScatteringMedium>` resource expression used to add the generated medium.
    #[serde(default = "default_scattering_media_var")]
    pub scattering_media_var: String,
    /// Optional explicit atmosphere component fields. When omitted, emit `Atmosphere::earthlike`.
    #[serde(default)]
    pub atmosphere: Option<BevyAtmosphere>,
    /// Optional label applied to the generated scattering medium.
    #[serde(default)]
    pub medium_label: Option<String>,
    /// Resolution used to sample each term's falloff distribution.
    #[serde(default = "default_scattering_resolution")]
    pub falloff_resolution: u32,
    /// Resolution used to sample each term's phase function.
    #[serde(default = "default_scattering_resolution")]
    pub phase_resolution: u32,
    /// Optional custom scattering terms. When omitted, emit `ScatteringMedium::earthlike`.
    #[serde(default)]
    pub terms: Vec<BevyScatteringTerm>,
    /// Optional density multiplier applied after building the medium.
    #[serde(default)]
    pub density_multiplier: Option<f32>,
}

/// Parameters for `bevy_render__fullscreen_material`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BevyFullscreenMaterialParams {
    /// Type implementing `FullscreenMaterial`, e.g. `FullscreenEffect`.
    pub material_type: String,
    /// Asset path for the fragment shader.
    pub shader_path: String,
    /// Which core render graph to target.
    #[serde(default)]
    pub graph: BevyFullscreenGraphKind,
    /// Optional starting edge label expression.
    #[serde(default)]
    pub start_node_expr: Option<String>,
    /// Optional ending edge label expression.
    #[serde(default)]
    pub end_node_expr: Option<String>,
}

fn default_commands_var() -> String {
    "commands".to_string()
}

fn default_true() -> bool {
    true
}

fn default_scattering_media_var() -> String {
    "scattering_media".to_string()
}

fn default_scattering_resolution() -> u32 {
    256
}

fn tool_err(msg: impl std::fmt::Display) -> ErrorData {
    ErrorData::invalid_params(msg.to_string(), None)
}

fn ok_source(source: String) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![Content::text(source)]))
}

fn parse_expr(src: &str, context: &str) -> Result<syn::Expr, ErrorData> {
    syn::parse_str::<syn::Expr>(src)
        .map_err(|error| tool_err(format!("invalid {context} expression `{src}`: {error}")))
}

fn parse_type(src: &str, context: &str) -> Result<syn::Type, ErrorData> {
    syn::parse_str::<syn::Type>(src)
        .map_err(|error| tool_err(format!("invalid {context} type `{src}`: {error}")))
}

fn expr_tokens(src: &str, context: &str) -> syn::Expr {
    parse_expr(src, context).expect("validated expressions must parse")
}

fn type_tokens(src: &str, context: &str) -> syn::Type {
    parse_type(src, context).expect("validated types must parse")
}

fn uvec2_array_tokens(value: &[u32; 2]) -> TokenStream {
    let [x, y] = *value;
    quote! { ::bevy::math::UVec2::new(#x, #y) }
}

fn viewport_depth_tokens(depth: &Option<[f32; 2]>) -> TokenStream {
    let [min, max] = depth.unwrap_or([0.0, 1.0]);
    quote! { #min..#max }
}

fn uvec3_array_tokens(value: &[u32; 3]) -> TokenStream {
    let [x, y, z] = *value;
    quote! { ::bevy::math::UVec3::new(#x, #y, #z) }
}

fn some_expr_tokens(value: &String) -> TokenStream {
    let expr = expr_tokens(value, "optional expression field");
    quote! { Some(#expr) }
}

fn into_expr_tokens(value: &String) -> TokenStream {
    let expr = expr_tokens(value, "into expression field");
    quote! { (#expr).into() }
}

fn text_color_tokens(value: &Option<String>) -> TokenStream {
    match value.as_deref() {
        Some(expr) => {
            let expr = expr_tokens(expr, "text color");
            quote! { ::bevy::text::TextColor((#expr).into()) }
        }
        None => quote! { ::bevy::text::TextColor::default() },
    }
}

fn cascade_bounds_tokens(value: &Vec<f32>) -> TokenStream {
    let bounds = value.iter().map(|bound| quote! { #bound });
    quote! { vec![#(#bounds),*] }
}

fn deserialize_optional_cascade_bounds<'de, D>(
    deserializer: D,
) -> Result<Option<BevyCascadeShadowBoundsParams>, D::Error>
where
    D: Deserializer<'de>,
{
    let bounds = Option::<Vec<f32>>::deserialize(deserializer)?;
    Ok(bounds
        .filter(|values| !values.is_empty())
        .map(BevyCascadeShadowBoundsParams))
}

fn emit_code_tokens<T: EmitCode>(value: &T) -> TokenStream {
    value.emit_code()
}

fn bevy_dep() -> Vec<CrateDep> {
    vec![CrateDep::new("bevy", "0.18")]
}

macro_rules! impl_bevy_literal_emit {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl EmitCode for $ty {
                fn emit_code(&self) -> TokenStream {
                    self.to_code_literal()
                }

                fn crate_deps(&self) -> Vec<CrateDep> {
                    bevy_dep()
                }
            }
        )+
    };
}

fn validate_optional_expr(src: &Option<String>, context: &str) -> Result<(), ErrorData> {
    if let Some(src) = src {
        let _ = parse_expr(src, context)?;
    }
    Ok(())
}

pub(crate) fn validate_render_target(params: &BevyRenderTargetParams) -> Result<(), ErrorData> {
    match params.fields.kind {
        BevyRenderTargetKind::PrimaryWindow => {
            if params.fields.target_expr.is_some() {
                return Err(tool_err(
                    "primary_window render target does not accept target_expr",
                ));
            }
        }
        BevyRenderTargetKind::WindowEntity
        | BevyRenderTargetKind::Image
        | BevyRenderTargetKind::TextureView => {
            let target = params
                .fields
                .target_expr
                .as_deref()
                .ok_or_else(|| tool_err("render target kind requires target_expr"))?;
            let _ = parse_expr(target, "render target")?;
        }
    }
    Ok(())
}

fn validate_order_independent_transparency_settings(
    params: &BevyOrderIndependentTransparencySettingsParams,
) -> Result<(), ErrorData> {
    if let Some(layer_count) = params.layer_count
        && layer_count <= 0
    {
        return Err(tool_err(
            "order independent transparency layer_count must be positive",
        ));
    }
    if let Some(alpha_threshold) = params.alpha_threshold {
        if !alpha_threshold.is_finite() {
            return Err(tool_err(
                "order independent transparency alpha_threshold must be finite",
            ));
        }
        if alpha_threshold < 0.0 {
            return Err(tool_err(
                "order independent transparency alpha_threshold must be non-negative",
            ));
        }
    }
    Ok(())
}

fn validate_scaling_mode(params: &BevyScalingModeParams) -> Result<(), ErrorData> {
    match params {
        BevyScalingModeParams::WindowSize => {}
        BevyScalingModeParams::Fixed { width, height } => {
            validate_optional_non_negative_f32(Some(*width), "scaling mode width")?;
            validate_optional_non_negative_f32(Some(*height), "scaling mode height")?;
        }
        BevyScalingModeParams::AutoMin {
            min_width,
            min_height,
        } => {
            validate_optional_non_negative_f32(Some(*min_width), "scaling mode min_width")?;
            validate_optional_non_negative_f32(Some(*min_height), "scaling mode min_height")?;
        }
        BevyScalingModeParams::AutoMax {
            max_width,
            max_height,
        } => {
            validate_optional_non_negative_f32(Some(*max_width), "scaling mode max_width")?;
            validate_optional_non_negative_f32(Some(*max_height), "scaling mode max_height")?;
        }
        BevyScalingModeParams::FixedVertical { viewport_height } => {
            validate_optional_non_negative_f32(
                Some(*viewport_height),
                "scaling mode viewport_height",
            )?;
        }
        BevyScalingModeParams::FixedHorizontal { viewport_width } => {
            validate_optional_non_negative_f32(
                Some(*viewport_width),
                "scaling mode viewport_width",
            )?;
        }
    }
    Ok(())
}

pub(crate) fn validate_perspective_projection(
    params: &BevyPerspectiveProjectionParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.near_clip_plane_expr, "near clip plane")?;
    validate_optional_non_negative_f32(params.fov, "perspective fov")?;
    validate_optional_non_negative_f32(params.aspect_ratio, "perspective aspect_ratio")?;
    validate_optional_non_negative_f32(params.near, "perspective near")?;
    validate_optional_non_negative_f32(params.far, "perspective far")?;
    Ok(())
}

pub(crate) fn validate_orthographic_projection(
    params: &BevyOrthographicProjectionParams,
) -> Result<(), ErrorData> {
    validate_optional_expr(&params.fields.viewport_origin_expr, "viewport origin")?;
    validate_optional_expr(&params.fields.scaling_mode_expr, "scaling mode")?;
    validate_optional_expr(&params.fields.area_expr, "orthographic area")?;
    validate_optional_finite_f32(params.fields.near, "orthographic near")?;
    validate_optional_finite_f32(params.fields.far, "orthographic far")?;
    validate_optional_non_negative_f32(params.fields.scale, "orthographic scale")?;
    Ok(())
}

fn validate_clear_color_config(params: &BevyClearColorConfigParams) -> Result<(), ErrorData> {
    match params {
        BevyClearColorConfigParams::Default | BevyClearColorConfigParams::None => {}
        BevyClearColorConfigParams::Custom(color_expr) => {
            let _ = parse_expr(color_expr, "clear color")?;
        }
    }
    Ok(())
}

fn validate_exposure(params: &BevyExposureParams) -> Result<(), ErrorData> {
    if params.fields.preset.is_some() && params.fields.ev100.is_some() {
        return Err(tool_err(
            "exposure accepts either preset or ev100, but not both",
        ));
    }
    validate_optional_finite_f32(params.fields.ev100, "exposure ev100")
}

fn validate_clear_color(params: &BevyClearColorParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "clear color")
}

fn validate_camera_3d_depth_load_op(
    params: &BevyCamera3dDepthLoadOpParams,
) -> Result<(), ErrorData> {
    match params {
        BevyCamera3dDepthLoadOpParams::Clear { depth } => {
            validate_optional_finite_f32(Some(*depth), "camera 3d depth clear value")
        }
        BevyCamera3dDepthLoadOpParams::Load => Ok(()),
    }
}

fn validate_screen_space_transmission_quality(
    _params: &BevyScreenSpaceTransmissionQualityParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_main_pass_resolution_override(
    params: &BevyMainPassResolutionOverrideParams,
) -> Result<(), ErrorData> {
    if params.fields.width == 0 || params.fields.height == 0 {
        return Err(tool_err(
            "main_pass_resolution_override width and height must be positive",
        ));
    }
    Ok(())
}

fn validate_sub_camera_view(params: &BevySubCameraViewParams) -> Result<(), ErrorData> {
    if params.fields.full_width == 0 || params.fields.full_height == 0 {
        return Err(tool_err("sub_camera_view full size must be positive"));
    }
    if params.fields.width == 0 || params.fields.height == 0 {
        return Err(tool_err("sub_camera_view size must be positive"));
    }
    if !params.fields.offset_x.is_finite() || !params.fields.offset_y.is_finite() {
        return Err(tool_err("sub_camera_view offsets must be finite"));
    }
    Ok(())
}

fn validate_visibility_range(params: &BevyVisibilityRangeParams) -> Result<(), ErrorData> {
    for (value, context) in [
        (
            params.fields.start_margin_start,
            "visibility_range start_margin_start",
        ),
        (
            params.fields.start_margin_end,
            "visibility_range start_margin_end",
        ),
        (
            params.fields.end_margin_start,
            "visibility_range end_margin_start",
        ),
        (
            params.fields.end_margin_end,
            "visibility_range end_margin_end",
        ),
    ] {
        if !value.is_finite() {
            return Err(tool_err(format!("{context} must be finite")));
        }
        if value < 0.0 {
            return Err(tool_err(format!("{context} must be non-negative")));
        }
    }
    if params.fields.start_margin_start > params.fields.start_margin_end {
        return Err(tool_err(
            "visibility_range start_margin_start must be <= start_margin_end",
        ));
    }
    if params.fields.end_margin_start > params.fields.end_margin_end {
        return Err(tool_err(
            "visibility_range end_margin_start must be <= end_margin_end",
        ));
    }
    if params.fields.start_margin_end > params.fields.end_margin_start {
        return Err(tool_err(
            "visibility_range start_margin_end must be <= end_margin_start",
        ));
    }
    Ok(())
}

fn render_target_tokens(params: &BevyRenderTargetFields) -> TokenStream {
    match params.kind {
        BevyRenderTargetKind::PrimaryWindow => {
            quote! {
                ::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary)
            }
        }
        BevyRenderTargetKind::WindowEntity => {
            let target = expr_tokens(
                params
                    .target_expr
                    .as_deref()
                    .expect("validated render target must have target expr"),
                "window entity",
            );
            quote! {
                ::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Entity(#target))
            }
        }
        BevyRenderTargetKind::Image => {
            let target = expr_tokens(
                params
                    .target_expr
                    .as_deref()
                    .expect("validated render target must have target expr"),
                "image target",
            );
            quote! { ::bevy::camera::RenderTarget::Image((#target).into()) }
        }
        BevyRenderTargetKind::TextureView => {
            let target = expr_tokens(
                params
                    .target_expr
                    .as_deref()
                    .expect("validated render target must have target expr"),
                "texture-view target",
            );
            quote! { ::bevy::camera::RenderTarget::TextureView(#target) }
        }
    }
}

fn orthographic_projection_tokens(params: &BevyOrthographicProjectionFields) -> TokenStream {
    let constructor = if params.use_2d_defaults.unwrap_or(false) {
        quote! { ::bevy::camera::OrthographicProjection::default_2d() }
    } else {
        quote! { ::bevy::camera::OrthographicProjection::default_3d() }
    };
    if params.near.is_none()
        && params.far.is_none()
        && params.viewport_origin_expr.is_none()
        && params.scaling_mode_expr.is_none()
        && params.scale.is_none()
        && params.area_expr.is_none()
    {
        constructor
    } else {
        let near = params
            .near
            .map(|value| quote! { near: #value, })
            .unwrap_or_default();
        let far = params
            .far
            .map(|value| quote! { far: #value, })
            .unwrap_or_default();
        let viewport_origin = params
            .viewport_origin_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "viewport origin");
                quote! { viewport_origin: #expr, }
            })
            .unwrap_or_default();
        let scaling_mode = params
            .scaling_mode_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "scaling mode");
                quote! { scaling_mode: #expr, }
            })
            .unwrap_or_default();
        let scale = params
            .scale
            .map(|value| quote! { scale: #value, })
            .unwrap_or_default();
        let area = params
            .area_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "orthographic area");
                quote! { area: #expr, }
            })
            .unwrap_or_default();
        quote! {
            ::bevy::camera::OrthographicProjection {
                #near
                #far
                #viewport_origin
                #scaling_mode
                #scale
                #area
                ..#constructor
            }
        }
    }
}

impl_bevy_literal_emit!(
    BevyRenderTargetParams,
    BevyTonemappingParams,
    BevyDebandDitherParams,
    BevyOrderIndependentTransparencySettingsParams,
    BevyScalingModeParams,
    BevyPerspectiveProjectionParams,
    BevyClearColorConfigParams,
    BevyExposureParams,
    BevyClearColorParams,
    BevyMsaaWritebackParams,
    BevyCamera3dDepthLoadOpParams,
    BevyScreenSpaceTransmissionQualityParams,
    BevyMainPassResolutionOverrideParams,
    BevySubCameraViewParams,
    BevyNoCpuCullingParams,
    BevyNoFrustumCullingParams,
    BevyVisibilityRangeParams,
    BevyColorParams,
    BevyRenderAlphaModeParams,
    BevyUvChannelParams,
    BevyOpaqueRendererMethodParams,
    BevyDefaultOpaqueRendererMethodParams,
    BevyDepthPrepassParams,
    BevyNormalPrepassParams,
    BevyMotionVectorPrepassParams,
    BevyDeferredPrepassParams,
    BevyDepthPrepassDoubleBufferParams,
    BevyDeferredPrepassDoubleBufferParams,
    BevyAmbientLightParams,
    BevyDirectionalLightParams,
    BevyGlobalAmbientLightParams,
    BevyPointLightParams,
    BevySpotLightParams,
    BevyDirectionalLightShadowMapParams,
    BevyPointLightShadowMapParams,
    BevyEnvironmentMapLightParams,
    BevyGeneratedEnvironmentMapLightParams,
    BevyAtmosphereEnvironmentMapLightParams,
    BevyVolumetricLightParams,
    BevyVolumetricFogParams,
    BevyLightProbeParams,
    BevyIrradianceVolumeParams,
    BevySunDiskParams,
    BevyWireframeColorParams,
    BevyWireframeConfigParams,
    BevyWireframe2dColorParams,
    BevyWireframe2dConfigParams,
    BevyNotShadowCasterParams,
    BevyNotShadowReceiverParams,
    BevyTransmittedShadowReceiverParams,
    BevyShadowFilteringMethodParams,
    BevyClusterFarZModeParams,
    BevyScreenSpaceReflectionsParams,
    BevyViewportParams,
    BevyBloomCompositeModeParams,
    BevyTemporalAntiAliasingParams,
    BevyCascadeShadowConfigBuilderParams,
    BevyParallaxMappingMethodParams,
    BevyLightmapParams,
    BevyColorMaterialParams,
    BevySkyboxParams,
    BevyFogVolumeParams,
    BevyFogSettingsParams,
    BevySsaoQualityParams,
    BevySsaoParams,
    BevyClusterZConfigParams,
    BevyClusterConfigParams,
    BevyAlphaMode2dParams,
    BevyMesh3dParams,
    BevyMesh2dParams,
    BevyMeshMaterial3dParams,
    BevyMeshMaterial2dParams,
    BevyClusteredDecalParams,
    BevyMesh3dWireframeParams,
    BevyMesh2dWireframeParams,
    BevyWireframeParams,
    BevyNoWireframeParams,
    BevyWireframe2dParams,
    BevyNoWireframe2dParams,
    BevySpriteParams,
    BevyTextStyleParams,
    BevyCascadeShadowConfigParams,
    BevyOrthographicProjectionParams,
    BevyStandardMaterialParams,
);

fn emit_uv_channel_variant_tokens(variant: &BevyUvChannelVariant) -> TokenStream {
    let variant = match variant {
        BevyUvChannelVariant::Uv0 => quote! { Uv0 },
        BevyUvChannelVariant::Uv1 => quote! { Uv1 },
    };
    quote! { ::bevy::pbr::UvChannel::#variant }
}

fn default_opaque_renderer_method_tokens(
    variant: &BevyDefaultOpaqueRendererMethodVariant,
) -> TokenStream {
    match variant {
        BevyDefaultOpaqueRendererMethodVariant::Default => {
            quote! { ::bevy::pbr::DefaultOpaqueRendererMethod::default() }
        }
        BevyDefaultOpaqueRendererMethodVariant::Forward => {
            quote! { ::bevy::pbr::DefaultOpaqueRendererMethod::forward() }
        }
        BevyDefaultOpaqueRendererMethodVariant::Deferred => {
            quote! { ::bevy::pbr::DefaultOpaqueRendererMethod::deferred() }
        }
    }
}

fn exposure_tokens(fields: &BevyExposureFields) -> TokenStream {
    match (fields.preset, fields.ev100) {
        (Some(BevyExposurePreset::Sunlight), None) => {
            quote! { ::bevy::camera::Exposure::SUNLIGHT }
        }
        (Some(BevyExposurePreset::Overcast), None) => {
            quote! { ::bevy::camera::Exposure::OVERCAST }
        }
        (Some(BevyExposurePreset::Indoor), None) => {
            quote! { ::bevy::camera::Exposure::INDOOR }
        }
        (Some(BevyExposurePreset::Blender), None) => {
            quote! { ::bevy::camera::Exposure::BLENDER }
        }
        (None, Some(ev100)) => quote! { ::bevy::camera::Exposure { ev100: #ev100 } },
        (None, None) => quote! { ::bevy::camera::Exposure::default() },
        (Some(_), Some(_)) => unreachable!("validated exposure params"),
    }
}

fn clear_color_tokens(color_expr: &Option<String>) -> TokenStream {
    match color_expr.as_deref() {
        Some(expr) => {
            let expr = expr_tokens(expr, "clear color");
            quote! { ::bevy::camera::ClearColor(#expr) }
        }
        None => quote! { ::bevy::camera::ClearColor::default() },
    }
}

fn main_pass_resolution_override_tokens(
    fields: &BevyMainPassResolutionOverrideFields,
) -> TokenStream {
    let width = fields.width;
    let height = fields.height;
    quote! {
        ::bevy::camera::MainPassResolutionOverride(::bevy::math::UVec2::new(#width, #height))
    }
}

fn sub_camera_view_tokens(fields: &BevySubCameraViewFields) -> TokenStream {
    let full_width = fields.full_width;
    let full_height = fields.full_height;
    let offset_x = fields.offset_x;
    let offset_y = fields.offset_y;
    let width = fields.width;
    let height = fields.height;
    quote! {
        ::bevy::camera::SubCameraView {
            full_size: ::bevy::math::UVec2::new(#full_width, #full_height),
            offset: ::bevy::math::Vec2::new(#offset_x, #offset_y),
            size: ::bevy::math::UVec2::new(#width, #height),
        }
    }
}

fn visibility_range_tokens(fields: &BevyVisibilityRangeFields) -> TokenStream {
    let start_margin_start = fields.start_margin_start;
    let start_margin_end = fields.start_margin_end;
    let end_margin_start = fields.end_margin_start;
    let end_margin_end = fields.end_margin_end;
    let use_aabb = fields.use_aabb.unwrap_or(false);
    quote! {
        ::bevy::camera::visibility::VisibilityRange {
            start_margin: #start_margin_start..#start_margin_end,
            end_margin: #end_margin_start..#end_margin_end,
            use_aabb: #use_aabb,
        }
    }
}

impl EmitCode for BevyCamera3dParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let transform = self
            .transform_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "transform");
                quote! { #expr, }
            })
            .unwrap_or_default();
        let hdr = self
            .hdr
            .map(|value| quote! { ::bevy::camera::Camera { hdr: #value, ..::std::default::Default::default() }, })
            .unwrap_or_default();
        let render_target = self
            .render_target
            .as_ref()
            .map(|target| {
                let target = target.emit_code();
                quote! { #target, }
            })
            .unwrap_or_else(|| {
                quote! {
                    ::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary),
                }
            });
        let tonemapping = self
            .tonemapping_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "tonemapping");
                quote! { #expr, }
            })
            .unwrap_or_default();
        let projection = if self.fov.is_none() && self.near.is_none() && self.far.is_none() {
            quote! { ::bevy::camera::PerspectiveProjection::default() }
        } else {
            let fov = self
                .fov
                .map(|value| quote! { fov: #value, })
                .unwrap_or_default();
            let near = self
                .near
                .map(|value| quote! { near: #value, })
                .unwrap_or_default();
            let far = self
                .far
                .map(|value| quote! { far: #value, })
                .unwrap_or_default();
            quote! {
                ::bevy::camera::PerspectiveProjection {
                    #fov
                    #near
                    #far
                    ..::bevy::camera::PerspectiveProjection::default()
                }
            }
        };
        quote! {
            #commands.spawn((
                ::bevy::camera::Camera3d::default(),
                ::bevy::camera::Projection::Perspective(#projection),
                #render_target
                #tonemapping
                #hdr
                #transform
            ))
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyCamera2dParams {
    fn emit_code(&self) -> TokenStream {
        let commands = expr_tokens(&self.commands_var, "commands receiver");
        let transform = self
            .transform_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "transform");
                quote! { #expr, }
            })
            .unwrap_or_default();
        let hdr = self
            .hdr
            .map(|value| quote! { ::bevy::camera::Camera { hdr: #value, ..::std::default::Default::default() }, })
            .unwrap_or_default();
        let render_target = self
            .render_target
            .as_ref()
            .map(|target| {
                let target = target.emit_code();
                quote! { #target, }
            })
            .unwrap_or_else(|| {
                quote! {
                    ::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary),
                }
            });
        let tonemapping = self
            .tonemapping_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "tonemapping");
                quote! { #expr, }
            })
            .unwrap_or_default();
        let projection = if self.near.is_none() && self.far.is_none() && self.scale.is_none() {
            quote! { ::bevy::camera::OrthographicProjection::default_2d() }
        } else {
            let near = self
                .near
                .map(|value| quote! { near: #value, })
                .unwrap_or_default();
            let far = self
                .far
                .map(|value| quote! { far: #value, })
                .unwrap_or_default();
            let scale = self
                .scale
                .map(|value| quote! { scale: #value, })
                .unwrap_or_default();
            quote! {
                ::bevy::camera::OrthographicProjection {
                    #near
                    #far
                    #scale
                    ..::bevy::camera::OrthographicProjection::default_2d()
                }
            }
        };
        quote! {
            #commands.spawn((
                ::bevy::camera::Camera2d::default(),
                ::bevy::camera::Projection::Orthographic(#projection),
                #render_target
                #tonemapping
                #hdr
                #transform
            ))
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyBloomSettingsParams {
    fn emit_code(&self) -> TokenStream {
        let preset = match self.preset {
            Some(BevyBloomPreset::Natural) => {
                quote! { ::bevy::post_process::bloom::Bloom::NATURAL }
            }
            Some(BevyBloomPreset::Anamorphic) => {
                quote! { ::bevy::post_process::bloom::Bloom::ANAMORPHIC }
            }
            Some(BevyBloomPreset::OldSchool) => {
                quote! { ::bevy::post_process::bloom::Bloom::OLD_SCHOOL }
            }
            Some(BevyBloomPreset::ScreenBlur) => {
                quote! { ::bevy::post_process::bloom::Bloom::SCREEN_BLUR }
            }
            None => quote! { ::bevy::post_process::bloom::Bloom::default() },
        };
        if self.intensity.is_none()
            && self.low_frequency_boost.is_none()
            && self.low_frequency_boost_curvature.is_none()
            && self.high_pass_frequency.is_none()
            && self.prefilter_threshold.is_none()
            && self.prefilter_threshold_softness.is_none()
            && self.composite_mode.is_none()
            && self.max_mip_dimension.is_none()
            && self.scale_expr.is_none()
        {
            preset
        } else {
            let intensity = self
                .intensity
                .map(|value| quote! { intensity: #value, })
                .unwrap_or_default();
            let low_frequency_boost = self
                .low_frequency_boost
                .map(|value| quote! { low_frequency_boost: #value, })
                .unwrap_or_default();
            let low_frequency_boost_curvature = self
                .low_frequency_boost_curvature
                .map(|value| quote! { low_frequency_boost_curvature: #value, })
                .unwrap_or_default();
            let high_pass_frequency = self
                .high_pass_frequency
                .map(|value| quote! { high_pass_frequency: #value, })
                .unwrap_or_default();
            let prefilter = if self.prefilter_threshold.is_none()
                && self.prefilter_threshold_softness.is_none()
            {
                TokenStream::new()
            } else {
                let threshold = self
                    .prefilter_threshold
                    .map(|value| quote! { threshold: #value, })
                    .unwrap_or_default();
                let threshold_softness = self
                    .prefilter_threshold_softness
                    .map(|value| quote! { threshold_softness: #value, })
                    .unwrap_or_default();
                quote! {
                    prefilter: ::bevy::post_process::bloom::BloomPrefilter {
                        #threshold
                        #threshold_softness
                        ..(#preset).prefilter
                    },
                }
            };
            let composite_mode = self
                .composite_mode
                .map(|variant| {
                    let variant = emit_bloom_composite_mode_tokens(variant);
                    quote! { composite_mode: #variant, }
                })
                .unwrap_or_default();
            let max_mip_dimension = self
                .max_mip_dimension
                .map(|value| quote! { max_mip_dimension: #value, })
                .unwrap_or_default();
            let scale = self
                .scale_expr
                .as_deref()
                .map(|expr| {
                    let expr = expr_tokens(expr, "bloom scale");
                    quote! { scale: #expr, }
                })
                .unwrap_or_default();
            quote! {
                ::bevy::post_process::bloom::Bloom {
                    #intensity
                    #low_frequency_boost
                    #low_frequency_boost_curvature
                    #high_pass_frequency
                    #prefilter
                    #composite_mode
                    #max_mip_dimension
                    #scale
                    ..#preset
                }
            }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

fn emit_bloom_composite_mode_tokens(variant: BevyBloomCompositeModeVariant) -> TokenStream {
    match variant {
        BevyBloomCompositeModeVariant::EnergyConserving => {
            quote! { ::bevy::post_process::bloom::BloomCompositeMode::EnergyConserving }
        }
        BevyBloomCompositeModeVariant::Additive => {
            quote! { ::bevy::post_process::bloom::BloomCompositeMode::Additive }
        }
    }
}

fn emit_falloff_tokens(falloff: &BevyFalloff) -> TokenStream {
    match falloff {
        BevyFalloff::Linear => quote! { ::bevy::pbr::Falloff::Linear },
        BevyFalloff::Exponential { scale } => {
            quote! { ::bevy::pbr::Falloff::Exponential { scale: #scale } }
        }
        BevyFalloff::Tent { center, width } => {
            quote! { ::bevy::pbr::Falloff::Tent { center: #center, width: #width } }
        }
    }
}

fn emit_phase_tokens(phase: &BevyPhaseFunction) -> TokenStream {
    match phase {
        BevyPhaseFunction::Isotropic => quote! { ::bevy::pbr::PhaseFunction::Isotropic },
        BevyPhaseFunction::Rayleigh => quote! { ::bevy::pbr::PhaseFunction::Rayleigh },
        BevyPhaseFunction::Mie { asymmetry } => {
            quote! { ::bevy::pbr::PhaseFunction::Mie { asymmetry: #asymmetry } }
        }
    }
}

fn emit_scattering_term_tokens(term: &BevyScatteringTerm) -> TokenStream {
    let absorption = term.absorption.to_code_literal();
    let scattering = term.scattering.to_code_literal();
    let falloff = emit_falloff_tokens(&term.falloff);
    let phase = emit_phase_tokens(&term.phase);
    quote! {
        ::bevy::pbr::ScatteringTerm {
            absorption: #absorption,
            scattering: #scattering,
            falloff: #falloff,
            phase: #phase,
        }
    }
}

fn emit_atmosphere_component_tokens(atmosphere: &BevyAtmosphere) -> TokenStream {
    let bottom_radius = atmosphere.bottom_radius;
    let top_radius = atmosphere.top_radius;
    let ground_albedo = atmosphere.ground_albedo.to_code_literal();
    quote! {
        ::bevy::pbr::Atmosphere {
            bottom_radius: #bottom_radius,
            top_radius: #top_radius,
            ground_albedo: #ground_albedo,
            medium,
        }
    }
}

fn default_fullscreen_edge_tokens(graph: BevyFullscreenGraphKind) -> (TokenStream, TokenStream) {
    match graph {
        BevyFullscreenGraphKind::Core2d => (
            quote! { ::bevy::core_pipeline::core_2d::graph::Node2d::Tonemapping },
            quote! { ::bevy::core_pipeline::core_2d::graph::Node2d::EndMainPassPostProcessing },
        ),
        BevyFullscreenGraphKind::Core3d => (
            quote! { ::bevy::core_pipeline::core_3d::graph::Node3d::Tonemapping },
            quote! { ::bevy::core_pipeline::core_3d::graph::Node3d::EndMainPassPostProcessing },
        ),
    }
}

impl EmitCode for BevyAtmosphereParams {
    fn emit_code(&self) -> TokenStream {
        let scattering_media = expr_tokens(&self.scattering_media_var, "scattering media");
        let falloff_resolution = self.falloff_resolution;
        let phase_resolution = self.phase_resolution;
        let terms = self
            .terms
            .iter()
            .map(emit_scattering_term_tokens)
            .collect::<Vec<_>>();
        let medium_base = if terms.is_empty() {
            quote! {
                ::bevy::pbr::ScatteringMedium::earthlike(#falloff_resolution, #phase_resolution)
            }
        } else {
            quote! {
                ::bevy::pbr::ScatteringMedium::new(
                    #falloff_resolution,
                    #phase_resolution,
                    [#(#terms),*],
                )
            }
        };
        let medium_base = if let Some(label) = &self.medium_label {
            let label = syn::LitStr::new(label, proc_macro2::Span::call_site());
            quote! { (#medium_base).with_label(#label) }
        } else {
            medium_base
        };
        let medium_base = if let Some(multiplier) = self.density_multiplier {
            quote! { (#medium_base).with_density_multiplier(#multiplier) }
        } else {
            medium_base
        };
        let atmosphere = self
            .atmosphere
            .as_ref()
            .map(emit_atmosphere_component_tokens)
            .unwrap_or_else(|| quote! { ::bevy::pbr::Atmosphere::earthlike(medium) });
        quote! {{
            let medium = (#scattering_media).add(#medium_base);
            #atmosphere
        }}
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

impl EmitCode for BevyFullscreenMaterialParams {
    fn emit_code(&self) -> TokenStream {
        let material_type = type_tokens(&self.material_type, "fullscreen material");
        let shader_path = syn::LitStr::new(&self.shader_path, proc_macro2::Span::call_site());
        let (default_start, default_end) = default_fullscreen_edge_tokens(self.graph);
        let start = self
            .start_node_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "fullscreen start node");
                quote! { #expr }
            })
            .unwrap_or(default_start);
        let end = self
            .end_node_expr
            .as_deref()
            .map(|expr| {
                let expr = expr_tokens(expr, "fullscreen end node");
                quote! { #expr }
            })
            .unwrap_or(default_end);
        quote! {
            impl ::bevy::core_pipeline::fullscreen_material::FullscreenMaterial for #material_type {
                fn fragment_shader() -> ::bevy::shader::ShaderRef {
                    #shader_path.into()
                }

                fn node_edges() -> ::std::vec::Vec<::bevy::render::render_graph::InternedRenderLabel> {
                    use ::bevy::render::render_graph::RenderLabel as _;

                    vec![
                        (#start).intern(),
                        Self::node_label().intern(),
                        (#end).intern(),
                    ]
                }
            }
        }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        bevy_dep()
    }
}

elicitation::register_emit!("render_target", BevyRenderTargetParams);
elicitation::register_emit!("tonemapping", BevyTonemappingParams);
elicitation::register_emit!("deband_dither", BevyDebandDitherParams);
elicitation::register_emit!(
    "order_independent_transparency_settings",
    BevyOrderIndependentTransparencySettingsParams
);
elicitation::register_emit!("scaling_mode", BevyScalingModeParams);
elicitation::register_emit!("perspective_projection", BevyPerspectiveProjectionParams);
elicitation::register_emit!("orthographic_projection", BevyOrthographicProjectionParams);
elicitation::register_emit!("clear_color_config", BevyClearColorConfigParams);
elicitation::register_emit!("msaa_writeback", BevyMsaaWritebackParams);
elicitation::register_emit!("exposure", BevyExposureParams);
elicitation::register_emit!("clear_color", BevyClearColorParams);
elicitation::register_emit!("camera_3d_depth_load_op", BevyCamera3dDepthLoadOpParams);
elicitation::register_emit!(
    "screen_space_transmission_quality",
    BevyScreenSpaceTransmissionQualityParams
);
elicitation::register_emit!(
    "main_pass_resolution_override",
    BevyMainPassResolutionOverrideParams
);
elicitation::register_emit!("sub_camera_view", BevySubCameraViewParams);
elicitation::register_emit!("no_cpu_culling", BevyNoCpuCullingParams);
elicitation::register_emit!("no_frustum_culling", BevyNoFrustumCullingParams);
elicitation::register_emit!("visibility_range", BevyVisibilityRangeParams);
elicitation::register_emit!("color", BevyColorParams);
elicitation::register_emit!("alpha_mode", BevyRenderAlphaModeParams);
elicitation::register_emit!("uv_channel", BevyUvChannelParams);
elicitation::register_emit!("parallax_mapping_method", BevyParallaxMappingMethodParams);
elicitation::register_emit!("opaque_renderer_method", BevyOpaqueRendererMethodParams);
elicitation::register_emit!(
    "default_opaque_renderer_method",
    BevyDefaultOpaqueRendererMethodParams
);
elicitation::register_emit!("alpha_mode_2d", BevyAlphaMode2dParams);
elicitation::register_emit!("standard_material", BevyStandardMaterialParams);
elicitation::register_emit!("lightmap", BevyLightmapParams);
elicitation::register_emit!("color_material", BevyColorMaterialParams);
elicitation::register_emit!("skybox", BevySkyboxParams);
elicitation::register_emit!("depth_prepass", BevyDepthPrepassParams);
elicitation::register_emit!("normal_prepass", BevyNormalPrepassParams);
elicitation::register_emit!("motion_vector_prepass", BevyMotionVectorPrepassParams);
elicitation::register_emit!("deferred_prepass", BevyDeferredPrepassParams);
elicitation::register_emit!(
    "depth_prepass_double_buffer",
    BevyDepthPrepassDoubleBufferParams
);
elicitation::register_emit!(
    "deferred_prepass_double_buffer",
    BevyDeferredPrepassDoubleBufferParams
);
elicitation::register_emit!("ambient_light", BevyAmbientLightParams);
elicitation::register_emit!("global_ambient_light", BevyGlobalAmbientLightParams);
elicitation::register_emit!("directional_light", BevyDirectionalLightParams);
elicitation::register_emit!("point_light", BevyPointLightParams);
elicitation::register_emit!("spot_light", BevySpotLightParams);
elicitation::register_emit!(
    "directional_light_shadow_map",
    BevyDirectionalLightShadowMapParams
);
elicitation::register_emit!("point_light_shadow_map", BevyPointLightShadowMapParams);
elicitation::register_emit!("environment_map_light", BevyEnvironmentMapLightParams);
elicitation::register_emit!(
    "generated_environment_map_light",
    BevyGeneratedEnvironmentMapLightParams
);
elicitation::register_emit!(
    "atmosphere_environment_map_light",
    BevyAtmosphereEnvironmentMapLightParams
);
elicitation::register_emit!("volumetric_light", BevyVolumetricLightParams);
elicitation::register_emit!("volumetric_fog", BevyVolumetricFogParams);
elicitation::register_emit!("fog_volume", BevyFogVolumeParams);
elicitation::register_emit!("light_probe", BevyLightProbeParams);
elicitation::register_emit!("irradiance_volume", BevyIrradianceVolumeParams);
elicitation::register_emit!("sun_disk", BevySunDiskParams);
elicitation::register_emit!("not_shadow_caster", BevyNotShadowCasterParams);
elicitation::register_emit!("not_shadow_receiver", BevyNotShadowReceiverParams);
elicitation::register_emit!(
    "transmitted_shadow_receiver",
    BevyTransmittedShadowReceiverParams
);
elicitation::register_emit!("shadow_filtering_method", BevyShadowFilteringMethodParams);
elicitation::register_emit!("cluster_far_z_mode", BevyClusterFarZModeParams);
elicitation::register_emit!("cluster_z_config", BevyClusterZConfigParams);
elicitation::register_emit!("cluster_config", BevyClusterConfigParams);
elicitation::register_emit!("screen_space_reflections", BevyScreenSpaceReflectionsParams);
elicitation::register_emit!("clustered_decal", BevyClusteredDecalParams);
elicitation::register_emit!("mesh_3d", BevyMesh3dParams);
elicitation::register_emit!("mesh_2d", BevyMesh2dParams);
elicitation::register_emit!("mesh_material_3d", BevyMeshMaterial3dParams);
elicitation::register_emit!("mesh_material_2d", BevyMeshMaterial2dParams);
elicitation::register_emit!("wireframe", BevyWireframeParams);
elicitation::register_emit!("wireframe_color", BevyWireframeColorParams);
elicitation::register_emit!("no_wireframe", BevyNoWireframeParams);
elicitation::register_emit!("wireframe_config", BevyWireframeConfigParams);
elicitation::register_emit!("mesh_3d_wireframe", BevyMesh3dWireframeParams);
elicitation::register_emit!("wireframe_2d", BevyWireframe2dParams);
elicitation::register_emit!("wireframe_2d_color", BevyWireframe2dColorParams);
elicitation::register_emit!("no_wireframe_2d", BevyNoWireframe2dParams);
elicitation::register_emit!("wireframe_2d_config", BevyWireframe2dConfigParams);
elicitation::register_emit!("mesh_2d_wireframe", BevyMesh2dWireframeParams);
elicitation::register_emit!("camera_3d", BevyCamera3dParams);
elicitation::register_emit!("camera_2d", BevyCamera2dParams);
elicitation::register_emit!("viewport", BevyViewportParams);
elicitation::register_emit!("fog_settings", BevyFogSettingsParams);
elicitation::register_emit!("bloom_composite_mode", BevyBloomCompositeModeParams);
elicitation::register_emit!("bloom_settings", BevyBloomSettingsParams);
elicitation::register_emit!("sprite", BevySpriteParams);
elicitation::register_emit!("text_style", BevyTextStyleParams);
elicitation::register_emit!("ssao_quality", BevySsaoQualityParams);
elicitation::register_emit!("ssao", BevySsaoParams);
elicitation::register_emit!("temporal_anti_aliasing", BevyTemporalAntiAliasingParams);
elicitation::register_emit!("cascade_shadow_config", BevyCascadeShadowConfigParams);
elicitation::register_emit!(
    "cascade_shadow_config_builder",
    BevyCascadeShadowConfigBuilderParams
);
elicitation::register_emit!("atmosphere", BevyAtmosphereParams);
elicitation::register_emit!("fullscreen_material", BevyFullscreenMaterialParams);

/// MCP plugin exposing core Bevy render/material fragment tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "bevy_render")]
pub struct BevyRenderPlugin;

impl BevyRenderPlugin {
    /// Creates a new Bevy render fragment plugin.
    #[instrument]
    pub fn new() -> Self {
        Self
    }
}

impl Default for BevyRenderPlugin {
    fn default() -> Self {
        Self::new()
    }
}

fn validate_standard_material(params: &BevyStandardMaterialParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.base_color_expr, "base color")?;
    validate_optional_expr(&params.base_color_texture_expr, "base color texture")?;
    validate_optional_expr(&params.emissive_expr, "emissive")?;
    validate_optional_expr(&params.emissive_texture_expr, "emissive texture")?;
    validate_optional_expr(
        &params.metallic_roughness_texture_expr,
        "metallic roughness texture",
    )?;
    validate_optional_expr(&params.specular_tint_expr, "specular tint")?;
    validate_optional_expr(
        &params.diffuse_transmission_texture_expr,
        "diffuse transmission texture",
    )?;
    validate_optional_expr(
        &params.specular_transmission_texture_expr,
        "specular transmission texture",
    )?;
    validate_optional_expr(&params.thickness_texture_expr, "thickness texture")?;
    validate_optional_expr(&params.attenuation_color_expr, "attenuation color")?;
    validate_optional_expr(&params.clearcoat_texture_expr, "clearcoat texture")?;
    validate_optional_expr(
        &params.clearcoat_roughness_texture_expr,
        "clearcoat roughness texture",
    )?;
    validate_optional_expr(
        &params.clearcoat_normal_texture_expr,
        "clearcoat normal texture",
    )?;
    validate_optional_expr(&params.anisotropy_texture_expr, "anisotropy texture")?;
    validate_optional_expr(&params.normal_map_texture_expr, "normal map texture")?;
    validate_optional_expr(&params.occlusion_texture_expr, "occlusion texture")?;
    validate_optional_expr(&params.specular_texture_expr, "specular texture")?;
    validate_optional_expr(&params.specular_tint_texture_expr, "specular tint texture")?;
    validate_optional_expr(&params.alpha_mode_expr, "alpha mode")?;
    validate_optional_expr(&params.cull_mode_expr, "cull mode")?;
    validate_optional_expr(&params.depth_map_expr, "depth map")?;
    validate_optional_expr(
        &params.parallax_mapping_method_expr,
        "parallax mapping method",
    )?;
    validate_optional_expr(&params.opaque_render_method_expr, "opaque render method")?;
    validate_optional_expr(&params.uv_transform_expr, "uv transform")?;
    validate_optional_finite_f32(params.emissive_exposure_weight, "emissive exposure weight")?;
    validate_optional_finite_f32(params.depth_bias, "depth bias")?;
    validate_optional_finite_f32(params.parallax_depth_scale, "parallax depth scale")?;
    validate_optional_non_negative_f32(
        params.max_parallax_layer_count,
        "max parallax layer count",
    )?;
    validate_optional_non_negative_f32(params.lightmap_exposure, "lightmap exposure")?;
    validate_optional_non_negative_f32(params.diffuse_transmission, "diffuse transmission")?;
    validate_optional_non_negative_f32(params.specular_transmission, "specular transmission")?;
    validate_optional_non_negative_f32(params.thickness, "thickness")?;
    validate_optional_non_negative_f32(params.ior, "index of refraction")?;
    validate_optional_non_negative_f32(params.attenuation_distance, "attenuation distance")?;
    validate_optional_non_negative_f32(params.clearcoat, "clearcoat")?;
    validate_optional_non_negative_f32(
        params.clearcoat_perceptual_roughness,
        "clearcoat perceptual roughness",
    )?;
    validate_optional_non_negative_f32(params.anisotropy_strength, "anisotropy strength")?;
    validate_optional_finite_f32(params.anisotropy_rotation, "anisotropy rotation")?;
    Ok(())
}

fn validate_uv_channel(_params: &BevyUvChannelParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_parallax_mapping_method(
    params: &BevyParallaxMappingMethodParams,
) -> Result<(), ErrorData> {
    match params {
        BevyParallaxMappingMethodParams::Occlusion => {}
        BevyParallaxMappingMethodParams::Relief { max_steps } => {
            if *max_steps == 0 {
                return Err(tool_err(
                    "relief parallax mapping max_steps must be greater than zero",
                ));
            }
        }
    }
    Ok(())
}

fn validate_opaque_renderer_method(
    _params: &BevyOpaqueRendererMethodParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_default_opaque_renderer_method(
    _params: &BevyDefaultOpaqueRendererMethodParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_alpha_mode_2d(params: &BevyAlphaMode2dParams) -> Result<(), ErrorData> {
    match params {
        BevyAlphaMode2dParams::Mask(threshold) => {
            if !threshold.is_finite() {
                return Err(tool_err("alpha_mode_2d threshold must be finite"));
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_lightmap(params: &BevyLightmapParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.image_expr, "lightmap image")?;
    validate_optional_expr(&params.uv_rect_expr, "lightmap uv_rect")
}

fn validate_color_material(params: &BevyColorMaterialParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "color material color")?;
    validate_optional_expr(&params.alpha_mode_expr, "color material alpha mode")?;
    validate_optional_expr(&params.uv_transform_expr, "color material uv transform")?;
    validate_optional_expr(&params.texture_expr, "color material texture")
}

fn validate_skybox(params: &BevySkyboxParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.image_expr, "skybox image")?;
    validate_optional_expr(&params.rotation_expr, "skybox rotation")?;
    validate_optional_non_negative_f32(params.brightness, "skybox brightness")
}

fn validate_depth_prepass(_params: &BevyDepthPrepassParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_normal_prepass(_params: &BevyNormalPrepassParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_motion_vector_prepass(
    _params: &BevyMotionVectorPrepassParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_deferred_prepass(_params: &BevyDeferredPrepassParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_depth_prepass_double_buffer(
    _params: &BevyDepthPrepassDoubleBufferParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_deferred_prepass_double_buffer(
    _params: &BevyDeferredPrepassDoubleBufferParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_ambient_light(params: &BevyAmbientLightParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "ambient light color")?;
    validate_optional_non_negative_f32(params.brightness, "ambient light brightness")
}

fn validate_global_ambient_light(params: &BevyGlobalAmbientLightParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "global ambient light color")?;
    validate_optional_non_negative_f32(params.brightness, "global ambient light brightness")
}

fn validate_directional_light(params: &BevyDirectionalLightParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "directional light color")
}

fn validate_point_light(params: &BevyPointLightParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "point light color")
}

fn validate_spot_light(params: &BevySpotLightParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "spot light color")
}

fn validate_directional_light_shadow_map(
    params: &BevyDirectionalLightShadowMapParams,
) -> Result<(), ErrorData> {
    validate_shadow_map_size(params.size, "directional light shadow map")
}

fn validate_point_light_shadow_map(
    params: &BevyPointLightShadowMapParams,
) -> Result<(), ErrorData> {
    validate_shadow_map_size(params.size, "point light shadow map")
}

fn validate_environment_map_light(params: &BevyEnvironmentMapLightParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.diffuse_map_expr, "environment diffuse map")?;
    let _ = parse_expr(&params.specular_map_expr, "environment specular map")?;
    validate_optional_expr(&params.rotation_expr, "environment rotation")?;
    validate_optional_non_negative_f32(params.intensity, "environment map light intensity")?;
    Ok(())
}

fn validate_generated_environment_map_light(
    params: &BevyGeneratedEnvironmentMapLightParams,
) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.environment_map_expr, "generated environment map")?;
    validate_optional_expr(&params.rotation_expr, "generated environment rotation")?;
    validate_optional_non_negative_f32(
        params.intensity,
        "generated environment map light intensity",
    )?;
    Ok(())
}

fn validate_atmosphere_environment_map_light(
    params: &BevyAtmosphereEnvironmentMapLightParams,
) -> Result<(), ErrorData> {
    validate_optional_non_negative_f32(
        params.intensity,
        "atmosphere environment map light intensity",
    )?;
    if let Some([width, height]) = params.size {
        validate_shadow_map_u32_size(width, "atmosphere environment map width")?;
        validate_shadow_map_u32_size(height, "atmosphere environment map height")?;
    }
    Ok(())
}

fn validate_volumetric_light(_params: &BevyVolumetricLightParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_volumetric_fog(params: &BevyVolumetricFogParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.ambient_color_expr, "volumetric fog ambient color")?;
    validate_optional_non_negative_f32(
        params.ambient_intensity,
        "volumetric fog ambient_intensity",
    )?;
    validate_optional_non_negative_f32(params.jitter, "volumetric fog jitter")?;
    if let Some(step_count) = params.step_count
        && step_count == 0
    {
        return Err(tool_err(
            "volumetric fog step_count must be greater than zero",
        ));
    }
    Ok(())
}

fn validate_fog_volume(params: &BevyFogVolumeParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.fog_color_expr, "fog volume color")?;
    validate_optional_expr(&params.density_texture_expr, "fog volume density texture")?;
    validate_optional_expr(
        &params.density_texture_offset_expr,
        "fog volume density texture offset",
    )?;
    validate_optional_expr(&params.light_tint_expr, "fog volume light tint")?;
    validate_optional_non_negative_f32(params.density_factor, "fog volume density_factor")?;
    validate_optional_non_negative_f32(params.absorption, "fog volume absorption")?;
    validate_optional_non_negative_f32(params.scattering, "fog volume scattering")?;
    validate_optional_finite_f32(
        params.scattering_asymmetry,
        "fog volume scattering_asymmetry",
    )?;
    validate_optional_non_negative_f32(params.light_intensity, "fog volume light_intensity")?;
    Ok(())
}

fn validate_light_probe(_params: &BevyLightProbeParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_irradiance_volume(params: &BevyIrradianceVolumeParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.voxels_expr, "irradiance volume voxels")?;
    validate_optional_non_negative_f32(params.intensity, "irradiance volume intensity")?;
    Ok(())
}

fn validate_sun_disk(params: &BevySunDiskParams) -> Result<(), ErrorData> {
    validate_optional_non_negative_f32(params.angular_size, "sun disk angular_size")?;
    validate_optional_non_negative_f32(params.intensity, "sun disk intensity")
}

fn validate_not_shadow_caster(_params: &BevyNotShadowCasterParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_not_shadow_receiver(_params: &BevyNotShadowReceiverParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_transmitted_shadow_receiver(
    _params: &BevyTransmittedShadowReceiverParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_shadow_filtering_method(
    _params: &BevyShadowFilteringMethodParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_cluster_far_z_mode(params: &BevyClusterFarZModeParams) -> Result<(), ErrorData> {
    match params {
        BevyClusterFarZModeParams::MaxClusterableObjectRange => {}
        BevyClusterFarZModeParams::Constant(constant) => {
            validate_optional_non_negative_f32(Some(*constant), "cluster far-z constant")?;
        }
    }
    Ok(())
}

fn validate_cluster_z_config(params: &BevyClusterZConfigParams) -> Result<(), ErrorData> {
    validate_optional_non_negative_f32(
        params.first_slice_depth,
        "cluster z config first_slice_depth",
    )?;
    if let Some(mode) = &params.far_z_mode {
        validate_cluster_far_z_mode(mode)?;
    }
    Ok(())
}

fn validate_cluster_config(params: &BevyClusterConfigParams) -> Result<(), ErrorData> {
    match params {
        BevyClusterConfigParams::None | BevyClusterConfigParams::Single => {}
        BevyClusterConfigParams::XYZ {
            dimensions,
            z_config,
            ..
        } => {
            let [x, y, z] = *dimensions;
            if x == 0 || y == 0 || z == 0 {
                return Err(tool_err(
                    "cluster config xyz dimensions must all be greater than zero",
                ));
            }
            validate_cluster_z_config(z_config)?;
        }
        BevyClusterConfigParams::FixedZ {
            total,
            z_slices,
            z_config,
            ..
        } => {
            if *total == 0 || *z_slices == 0 {
                return Err(tool_err(
                    "cluster config fixed_z total and z_slices must be greater than zero",
                ));
            }
            validate_cluster_z_config(z_config)?;
        }
    }
    Ok(())
}

fn validate_screen_space_reflections(
    params: &BevyScreenSpaceReflectionsParams,
) -> Result<(), ErrorData> {
    if let Some(value) = params.perceptual_roughness_threshold {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(tool_err(
                "screen-space reflections perceptual_roughness_threshold must be in [0.0, 1.0]",
            ));
        }
    }
    validate_optional_non_negative_f32(params.thickness, "screen-space reflections thickness")?;
    if let Some(linear_steps) = params.linear_steps
        && linear_steps == 0
    {
        return Err(tool_err(
            "screen-space reflections linear_steps must be greater than zero",
        ));
    }
    validate_optional_non_negative_f32(
        params.linear_march_exponent,
        "screen-space reflections linear_march_exponent",
    )?;
    Ok(())
}

fn validate_clustered_decal(params: &BevyClusteredDecalParams) -> Result<(), ErrorData> {
    validate_optional_expr(
        &params.base_color_texture_expr,
        "clustered decal base_color_texture",
    )?;
    validate_optional_expr(
        &params.normal_map_texture_expr,
        "clustered decal normal_map_texture",
    )?;
    validate_optional_expr(
        &params.metallic_roughness_texture_expr,
        "clustered decal metallic_roughness_texture",
    )?;
    validate_optional_expr(
        &params.emissive_texture_expr,
        "clustered decal emissive_texture",
    )?;
    Ok(())
}

fn validate_shadow_map_size(size: Option<usize>, context: &str) -> Result<(), ErrorData> {
    if let Some(size) = size {
        validate_shadow_map_usize_size(size, context)?;
    }
    Ok(())
}

fn validate_mesh_3d(params: &BevyMesh3dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.mesh_expr, "mesh_3d mesh")?;
    Ok(())
}

fn validate_mesh_2d(params: &BevyMesh2dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.mesh_expr, "mesh_2d mesh")?;
    Ok(())
}

fn validate_mesh_material_3d(params: &BevyMeshMaterial3dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.material_expr, "mesh_material_3d material")?;
    Ok(())
}

fn validate_mesh_material_2d(params: &BevyMeshMaterial2dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.material_expr, "mesh_material_2d material")?;
    Ok(())
}

fn validate_wireframe(_params: &BevyWireframeParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_wireframe_color(params: &BevyWireframeColorParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.color_expr, "wireframe_color color")?;
    Ok(())
}

fn validate_no_wireframe(_params: &BevyNoWireframeParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_wireframe_config(params: &BevyWireframeConfigParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.default_color_expr, "wireframe_config default color")
}

fn validate_mesh_3d_wireframe(params: &BevyMesh3dWireframeParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.material_expr, "mesh_3d_wireframe material")?;
    Ok(())
}

fn validate_wireframe_2d(_params: &BevyWireframe2dParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_wireframe_2d_color(params: &BevyWireframe2dColorParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.color_expr, "wireframe_2d_color color")?;
    Ok(())
}

fn validate_no_wireframe_2d(_params: &BevyNoWireframe2dParams) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_wireframe_2d_config(params: &BevyWireframe2dConfigParams) -> Result<(), ErrorData> {
    validate_optional_expr(
        &params.default_color_expr,
        "wireframe_2d_config default color",
    )
}

fn validate_mesh_2d_wireframe(params: &BevyMesh2dWireframeParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.material_expr, "mesh_2d_wireframe material")?;
    Ok(())
}

fn validate_shadow_map_usize_size(size: usize, context: &str) -> Result<(), ErrorData> {
    if size == 0 {
        return Err(tool_err(format!(
            "{context} size must be greater than zero"
        )));
    }
    if !size.is_power_of_two() {
        return Err(tool_err(format!("{context} size must be a power of two")));
    }
    Ok(())
}

fn validate_shadow_map_u32_size(size: u32, context: &str) -> Result<(), ErrorData> {
    if size == 0 {
        return Err(tool_err(format!("{context} must be greater than zero")));
    }
    if !size.is_power_of_two() {
        return Err(tool_err(format!("{context} must be a power of two")));
    }
    Ok(())
}

fn validate_optional_non_negative_f32(value: Option<f32>, context: &str) -> Result<(), ErrorData> {
    if let Some(value) = value
        && (!value.is_finite() || value < 0.0)
    {
        return Err(tool_err(format!(
            "{context} must be a finite non-negative number"
        )));
    }
    Ok(())
}

fn validate_optional_finite_f32(value: Option<f32>, context: &str) -> Result<(), ErrorData> {
    if let Some(value) = value
        && !value.is_finite()
    {
        return Err(tool_err(format!("{context} must be finite")));
    }
    Ok(())
}

fn validate_camera_3d(params: &BevyCamera3dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_optional_expr(&params.transform_expr, "transform")?;
    validate_optional_expr(&params.tonemapping_expr, "tonemapping")?;
    if let Some(target) = &params.render_target {
        validate_render_target(target)?;
    }
    Ok(())
}

fn validate_camera_2d(params: &BevyCamera2dParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.commands_var, "commands receiver")?;
    validate_optional_expr(&params.transform_expr, "transform")?;
    validate_optional_expr(&params.tonemapping_expr, "tonemapping")?;
    if let Some(target) = &params.render_target {
        validate_render_target(target)?;
    }
    Ok(())
}

fn validate_viewport(params: &BevyViewportParams) -> Result<(), ErrorData> {
    if let Some([min, max]) = params.depth {
        if min > max {
            return Err(tool_err(
                "viewport depth min must be less than or equal to max",
            ));
        }
    }
    Ok(())
}

fn validate_fog_settings(params: &BevyFogSettingsParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.color_expr, "fog color")?;
    validate_optional_expr(
        &params.directional_light_color_expr,
        "fog directional light color",
    )?;
    validate_optional_expr(&params.falloff_expr, "fog falloff")?;
    Ok(())
}

fn validate_bloom_settings(params: &BevyBloomSettingsParams) -> Result<(), ErrorData> {
    if let Some(softness) = params.prefilter_threshold_softness {
        if !softness.is_finite() {
            return Err(tool_err(
                "bloom prefilter threshold softness must be finite",
            ));
        }
    }
    validate_optional_expr(&params.scale_expr, "bloom scale")
}

fn validate_sprite(params: &BevySpriteParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.image_expr, "sprite image")?;
    validate_optional_expr(&params.color_expr, "sprite color")?;
    validate_optional_expr(&params.custom_size_expr, "sprite custom size")?;
    Ok(())
}

fn validate_text_style(params: &BevyTextStyleParams) -> Result<(), ErrorData> {
    validate_optional_expr(&params.font.font_handle_expr, "font handle")?;
    validate_optional_expr(&params.color.color_expr, "text color")?;
    validate_optional_expr(&params.layout.justify_expr, "text justify")?;
    validate_optional_expr(&params.layout.linebreak_expr, "text linebreak")?;
    Ok(())
}

fn validate_ssao_quality(params: &BevySsaoQualityParams) -> Result<(), ErrorData> {
    if let BevySsaoQualityParams::Custom {
        slice_count,
        samples_per_slice_side,
    } = params
    {
        if *slice_count == 0 {
            return Err(tool_err(
                "custom ssao slice_count must be greater than zero",
            ));
        }
        if *samples_per_slice_side == 0 {
            return Err(tool_err(
                "custom ssao samples_per_slice_side must be greater than zero",
            ));
        }
    }
    Ok(())
}

fn validate_ssao(params: &BevySsaoParams) -> Result<(), ErrorData> {
    if let Some(quality) = &params.quality_level {
        validate_ssao_quality(quality)?;
    }
    if let Some(value) = params.constant_object_thickness {
        if !value.is_finite() || value < 0.0 {
            return Err(tool_err(
                "ssao constant_object_thickness must be a finite non-negative number",
            ));
        }
    }
    Ok(())
}

fn validate_temporal_anti_aliasing(
    _params: &BevyTemporalAntiAliasingParams,
) -> Result<(), ErrorData> {
    Ok(())
}

fn validate_cascade_shadow_config(params: &BevyCascadeShadowConfigParams) -> Result<(), ErrorData> {
    if let Some(value) = params.minimum_distance {
        if !value.is_finite() || value < 0.0 {
            return Err(tool_err(
                "cascade minimum_distance must be a finite non-negative number",
            ));
        }
    }
    if let Some(value) = params.overlap_proportion {
        if !value.is_finite() || !(0.0..1.0).contains(&value) {
            return Err(tool_err(
                "cascade overlap_proportion must be in the range [0.0, 1.0)",
            ));
        }
    }
    if let Some(bounds) = &params.bounds {
        let mut previous = None;
        for bound in &bounds.0 {
            if !bound.is_finite() || *bound <= 0.0 {
                return Err(tool_err(
                    "cascade bounds must be finite positive far-bound distances",
                ));
            }
            if let Some(prev) = previous
                && *bound <= prev
            {
                return Err(tool_err(
                    "cascade bounds must be strictly increasing far-bound distances",
                ));
            }
            previous = Some(*bound);
        }
    }
    Ok(())
}

fn validate_cascade_shadow_config_builder(
    params: &BevyCascadeShadowConfigBuilderParams,
) -> Result<(), ErrorData> {
    const DEFAULT_MINIMUM_DISTANCE: f32 = 0.1;
    const DEFAULT_FIRST_CASCADE_FAR_BOUND: f32 = 10.0;

    if let Some(num_cascades) = params.num_cascades
        && num_cascades == 0
    {
        return Err(tool_err(
            "cascade builder num_cascades must be greater than zero",
        ));
    }

    let minimum_distance = params.minimum_distance.unwrap_or(DEFAULT_MINIMUM_DISTANCE);
    if !minimum_distance.is_finite() || minimum_distance < 0.0 {
        return Err(tool_err(
            "cascade builder minimum_distance must be a finite non-negative number",
        ));
    }

    if let Some(maximum_distance) = params.maximum_distance {
        if !maximum_distance.is_finite() || maximum_distance <= minimum_distance {
            return Err(tool_err(
                "cascade builder maximum_distance must be finite and greater than minimum_distance",
            ));
        }
    }

    if let Some(first_cascade_far_bound) = params.first_cascade_far_bound
        && (!first_cascade_far_bound.is_finite()
            || first_cascade_far_bound <= 0.0
            || (params.num_cascades.unwrap_or(4) != 1
                && minimum_distance >= first_cascade_far_bound))
    {
        return Err(tool_err(
            "cascade builder first_cascade_far_bound must be finite, positive, and greater than minimum_distance when using multiple cascades",
        ));
    }

    if params.num_cascades.unwrap_or(4) != 1
        && params.first_cascade_far_bound.is_none()
        && minimum_distance >= DEFAULT_FIRST_CASCADE_FAR_BOUND
    {
        return Err(tool_err(
            "cascade builder minimum_distance must be less than the default first_cascade_far_bound for multi-cascade builds",
        ));
    }

    if let Some(overlap_proportion) = params.overlap_proportion
        && (!overlap_proportion.is_finite() || !(0.0..1.0).contains(&overlap_proportion))
    {
        return Err(tool_err(
            "cascade builder overlap_proportion must be in the range [0.0, 1.0)",
        ));
    }

    Ok(())
}

pub(crate) fn validate_atmosphere(params: &BevyAtmosphereParams) -> Result<(), ErrorData> {
    let _ = parse_expr(&params.scattering_media_var, "scattering media")?;
    if params.falloff_resolution == 0 {
        return Err(tool_err("falloff_resolution must be greater than zero"));
    }
    if params.phase_resolution == 0 {
        return Err(tool_err("phase_resolution must be greater than zero"));
    }
    if let Some(multiplier) = params.density_multiplier {
        if !multiplier.is_finite() {
            return Err(tool_err("density_multiplier must be finite"));
        }
    }
    Ok(())
}

fn validate_fullscreen_material(params: &BevyFullscreenMaterialParams) -> Result<(), ErrorData> {
    let _ = parse_type(&params.material_type, "fullscreen material")?;
    if params.shader_path.trim().is_empty() {
        return Err(tool_err("shader_path must not be empty"));
    }
    validate_optional_expr(&params.start_node_expr, "fullscreen start node")?;
    validate_optional_expr(&params.end_node_expr, "fullscreen end node")?;
    Ok(())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "render_target",
    description = "Emit a `RenderTarget` component for primary-window, entity-window, image, or texture-view outputs.",
    emit = None
)]
#[instrument(skip_all)]
async fn render_target(p: BevyRenderTargetParams) -> Result<CallToolResult, ErrorData> {
    validate_render_target(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "tonemapping",
    description = "Emit a `Tonemapping` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn tonemapping(p: BevyTonemappingParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "deband_dither",
    description = "Emit a `DebandDither` camera component for final-image debanding control.",
    emit = None
)]
#[instrument(skip_all)]
async fn deband_dither(p: BevyDebandDitherParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "order_independent_transparency_settings",
    description = "Emit an `OrderIndependentTransparencySettings` camera component with optional layer-count and alpha-threshold overrides.",
    emit = None
)]
#[instrument(skip_all)]
async fn order_independent_transparency_settings(
    p: BevyOrderIndependentTransparencySettingsParams,
) -> Result<CallToolResult, ErrorData> {
    validate_order_independent_transparency_settings(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "scaling_mode",
    description = "Emit a `ScalingMode` expression for orthographic camera projections.",
    emit = None
)]
#[instrument(skip_all)]
async fn scaling_mode(p: BevyScalingModeParams) -> Result<CallToolResult, ErrorData> {
    validate_scaling_mode(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "perspective_projection",
    description = "Emit a `PerspectiveProjection` literal using Bevy defaults plus optional overrides.",
    emit = None
)]
#[instrument(skip_all)]
async fn perspective_projection(
    p: BevyPerspectiveProjectionParams,
) -> Result<CallToolResult, ErrorData> {
    validate_perspective_projection(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "orthographic_projection",
    description = "Emit an `OrthographicProjection` literal using Bevy 2D or 3D defaults plus optional overrides.",
    emit = None
)]
#[instrument(skip_all)]
async fn orthographic_projection(
    p: BevyOrthographicProjectionParams,
) -> Result<CallToolResult, ErrorData> {
    validate_orthographic_projection(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "clear_color_config",
    description = "Emit a `ClearColorConfig` expression for camera clear behavior.",
    emit = None
)]
#[instrument(skip_all)]
async fn clear_color_config(p: BevyClearColorConfigParams) -> Result<CallToolResult, ErrorData> {
    validate_clear_color_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "msaa_writeback",
    description = "Emit an `MsaaWriteback` expression for camera layering behavior.",
    emit = None
)]
#[instrument(skip_all)]
async fn msaa_writeback(p: BevyMsaaWritebackParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "exposure",
    description = "Emit an `Exposure` component using a preset or explicit EV100 value.",
    emit = None
)]
#[instrument(skip_all)]
async fn exposure(p: BevyExposureParams) -> Result<CallToolResult, ErrorData> {
    validate_exposure(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "clear_color",
    description = "Emit a `ClearColor` resource literal using Bevy's default or an explicit color.",
    emit = None
)]
#[instrument(skip_all)]
async fn clear_color(p: BevyClearColorParams) -> Result<CallToolResult, ErrorData> {
    validate_clear_color(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "camera_3d_depth_load_op",
    description = "Emit a `Camera3dDepthLoadOp` expression for main-pass depth behavior.",
    emit = None
)]
#[instrument(skip_all)]
async fn camera_3d_depth_load_op(
    p: BevyCamera3dDepthLoadOpParams,
) -> Result<CallToolResult, ErrorData> {
    validate_camera_3d_depth_load_op(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "screen_space_transmission_quality",
    description = "Emit a `ScreenSpaceTransmissionQuality` resource value.",
    emit = None
)]
#[instrument(skip_all)]
async fn screen_space_transmission_quality(
    p: BevyScreenSpaceTransmissionQualityParams,
) -> Result<CallToolResult, ErrorData> {
    validate_screen_space_transmission_quality(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "main_pass_resolution_override",
    description = "Emit a `MainPassResolutionOverride` component for camera render scaling.",
    emit = None
)]
#[instrument(skip_all)]
async fn main_pass_resolution_override(
    p: BevyMainPassResolutionOverrideParams,
) -> Result<CallToolResult, ErrorData> {
    validate_main_pass_resolution_override(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "sub_camera_view",
    description = "Emit a `SubCameraView` value for tiled or multi-monitor camera layouts.",
    emit = None
)]
#[instrument(skip_all)]
async fn sub_camera_view(p: BevySubCameraViewParams) -> Result<CallToolResult, ErrorData> {
    validate_sub_camera_view(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "no_cpu_culling",
    description = "Emit a `NoCpuCulling` marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn no_cpu_culling(p: BevyNoCpuCullingParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "no_frustum_culling",
    description = "Emit a `NoFrustumCulling` marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn no_frustum_culling(p: BevyNoFrustumCullingParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "visibility_range",
    description = "Emit a `VisibilityRange` component with explicit near and far fade bands.",
    emit = None
)]
#[instrument(skip_all)]
async fn visibility_range(p: BevyVisibilityRangeParams) -> Result<CallToolResult, ErrorData> {
    validate_visibility_range(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "color",
    description = "Emit a structured `bevy::color::Color` expression using the selected Bevy color space.",
    emit = None
)]
#[instrument(skip_all)]
async fn color(p: BevyColorParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "alpha_mode",
    description = "Emit an `AlphaMode` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn alpha_mode(p: BevyRenderAlphaModeParams) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "uv_channel",
    description = "Emit a `UvChannel` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn uv_channel(p: BevyUvChannelParams) -> Result<CallToolResult, ErrorData> {
    validate_uv_channel(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "parallax_mapping_method",
    description = "Emit a `ParallaxMappingMethod` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn parallax_mapping_method(
    p: BevyParallaxMappingMethodParams,
) -> Result<CallToolResult, ErrorData> {
    validate_parallax_mapping_method(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "opaque_renderer_method",
    description = "Emit an `OpaqueRendererMethod` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn opaque_renderer_method(
    p: BevyOpaqueRendererMethodParams,
) -> Result<CallToolResult, ErrorData> {
    validate_opaque_renderer_method(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "default_opaque_renderer_method",
    description = "Emit a `DefaultOpaqueRendererMethod` resource constructor expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn default_opaque_renderer_method(
    p: BevyDefaultOpaqueRendererMethodParams,
) -> Result<CallToolResult, ErrorData> {
    validate_default_opaque_renderer_method(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "alpha_mode_2d",
    description = "Emit an `AlphaMode2d` enum expression for 2D mesh materials.",
    emit = None
)]
#[instrument(skip_all)]
async fn alpha_mode_2d(p: BevyAlphaMode2dParams) -> Result<CallToolResult, ErrorData> {
    validate_alpha_mode_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "standard_material",
    description = "Emit a `StandardMaterial` struct literal with common surface fields.",
    emit = None
)]
#[instrument(skip_all)]
async fn standard_material(p: BevyStandardMaterialParams) -> Result<CallToolResult, ErrorData> {
    validate_standard_material(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "lightmap",
    description = "Emit a `Lightmap` component for baked 3D lighting with optional atlas rect and bicubic sampling.",
    emit = None
)]
#[instrument(skip_all)]
async fn lightmap(p: BevyLightmapParams) -> Result<CallToolResult, ErrorData> {
    validate_lightmap(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "color_material",
    description = "Emit a `ColorMaterial` asset literal with optional tint, alpha mode, UV transform, and texture handle.",
    emit = None
)]
#[instrument(skip_all)]
async fn color_material(p: BevyColorMaterialParams) -> Result<CallToolResult, ErrorData> {
    validate_color_material(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "skybox",
    description = "Emit a `Skybox` component with cubemap handle, optional brightness, and optional rotation.",
    emit = None
)]
#[instrument(skip_all)]
async fn skybox(p: BevySkyboxParams) -> Result<CallToolResult, ErrorData> {
    validate_skybox(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "depth_prepass",
    description = "Emit the `DepthPrepass` camera marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn depth_prepass(p: BevyDepthPrepassParams) -> Result<CallToolResult, ErrorData> {
    validate_depth_prepass(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "normal_prepass",
    description = "Emit the `NormalPrepass` camera marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn normal_prepass(p: BevyNormalPrepassParams) -> Result<CallToolResult, ErrorData> {
    validate_normal_prepass(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "motion_vector_prepass",
    description = "Emit the `MotionVectorPrepass` camera marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn motion_vector_prepass(
    p: BevyMotionVectorPrepassParams,
) -> Result<CallToolResult, ErrorData> {
    validate_motion_vector_prepass(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "deferred_prepass",
    description = "Emit the `DeferredPrepass` camera marker component.",
    emit = None
)]
#[instrument(skip_all)]
async fn deferred_prepass(p: BevyDeferredPrepassParams) -> Result<CallToolResult, ErrorData> {
    validate_deferred_prepass(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "depth_prepass_double_buffer",
    description = "Emit the `DepthPrepassDoubleBuffer` marker component for previous-frame depth queries.",
    emit = None
)]
#[instrument(skip_all)]
async fn depth_prepass_double_buffer(
    p: BevyDepthPrepassDoubleBufferParams,
) -> Result<CallToolResult, ErrorData> {
    validate_depth_prepass_double_buffer(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "deferred_prepass_double_buffer",
    description = "Emit the `DeferredPrepassDoubleBuffer` marker component for previous-frame deferred g-buffer queries.",
    emit = None
)]
#[instrument(skip_all)]
async fn deferred_prepass_double_buffer(
    p: BevyDeferredPrepassDoubleBufferParams,
) -> Result<CallToolResult, ErrorData> {
    validate_deferred_prepass_double_buffer(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "ambient_light",
    description = "Emit an `AmbientLight` struct literal.",
    emit = None
)]
#[instrument(skip_all)]
async fn ambient_light(p: BevyAmbientLightParams) -> Result<CallToolResult, ErrorData> {
    validate_ambient_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "global_ambient_light",
    description = "Emit a `GlobalAmbientLight` resource literal for scene-wide ambient lighting defaults.",
    emit = None
)]
#[instrument(skip_all)]
async fn global_ambient_light(
    p: BevyGlobalAmbientLightParams,
) -> Result<CallToolResult, ErrorData> {
    validate_global_ambient_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "directional_light",
    description = "Emit a `DirectionalLight` struct literal.",
    emit = None
)]
#[instrument(skip_all)]
async fn directional_light(p: BevyDirectionalLightParams) -> Result<CallToolResult, ErrorData> {
    validate_directional_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "point_light",
    description = "Emit a `PointLight` struct literal.",
    emit = None
)]
#[instrument(skip_all)]
async fn point_light(p: BevyPointLightParams) -> Result<CallToolResult, ErrorData> {
    validate_point_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "spot_light",
    description = "Emit a `SpotLight` struct literal.",
    emit = None
)]
#[instrument(skip_all)]
async fn spot_light(p: BevySpotLightParams) -> Result<CallToolResult, ErrorData> {
    validate_spot_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "directional_light_shadow_map",
    description = "Emit a `DirectionalLightShadowMap` resource expression with an optional power-of-two cascade size.",
    emit = None
)]
#[instrument(skip_all)]
async fn directional_light_shadow_map(
    p: BevyDirectionalLightShadowMapParams,
) -> Result<CallToolResult, ErrorData> {
    validate_directional_light_shadow_map(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "point_light_shadow_map",
    description = "Emit a `PointLightShadowMap` resource expression with an optional power-of-two cubemap-face size.",
    emit = None
)]
#[instrument(skip_all)]
async fn point_light_shadow_map(
    p: BevyPointLightShadowMapParams,
) -> Result<CallToolResult, ErrorData> {
    validate_point_light_shadow_map(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "environment_map_light",
    description = "Emit an `EnvironmentMapLight` component with diffuse/specular cubemap handles and optional tuning fields.",
    emit = None
)]
#[instrument(skip_all)]
async fn environment_map_light(
    p: BevyEnvironmentMapLightParams,
) -> Result<CallToolResult, ErrorData> {
    validate_environment_map_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "generated_environment_map_light",
    description = "Emit a `GeneratedEnvironmentMapLight` component for runtime-filtered cubemap lighting.",
    emit = None
)]
#[instrument(skip_all)]
async fn generated_environment_map_light(
    p: BevyGeneratedEnvironmentMapLightParams,
) -> Result<CallToolResult, ErrorData> {
    validate_generated_environment_map_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "atmosphere_environment_map_light",
    description = "Emit an `AtmosphereEnvironmentMapLight` component for atmosphere-driven image-based lighting.",
    emit = None
)]
#[instrument(skip_all)]
async fn atmosphere_environment_map_light(
    p: BevyAtmosphereEnvironmentMapLightParams,
) -> Result<CallToolResult, ErrorData> {
    validate_atmosphere_environment_map_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "volumetric_light",
    description = "Emit the `VolumetricLight` marker component used for light shafts and god rays.",
    emit = None
)]
#[instrument(skip_all)]
async fn volumetric_light(p: BevyVolumetricLightParams) -> Result<CallToolResult, ErrorData> {
    validate_volumetric_light(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "volumetric_fog",
    description = "Emit a `VolumetricFog` component for Bevy 0.18 camera-based volumetric fog.",
    emit = None
)]
#[instrument(skip_all)]
async fn volumetric_fog(p: BevyVolumetricFogParams) -> Result<CallToolResult, ErrorData> {
    validate_volumetric_fog(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "fog_volume",
    description = "Emit a `FogVolume` component with optional density texture, optical coefficients, and artistic tint controls.",
    emit = None
)]
#[instrument(skip_all)]
async fn fog_volume(p: BevyFogVolumeParams) -> Result<CallToolResult, ErrorData> {
    validate_fog_volume(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "light_probe",
    description = "Emit the `LightProbe` marker component for probe-bounded indirect lighting regions.",
    emit = None
)]
#[instrument(skip_all)]
async fn light_probe(p: BevyLightProbeParams) -> Result<CallToolResult, ErrorData> {
    validate_light_probe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "irradiance_volume",
    description = "Emit an `IrradianceVolume` component with voxel texture, intensity, and lightmap interaction controls.",
    emit = None
)]
#[instrument(skip_all)]
async fn irradiance_volume(p: BevyIrradianceVolumeParams) -> Result<CallToolResult, ErrorData> {
    validate_irradiance_volume(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "sun_disk",
    description = "Emit a `SunDisk` component for atmosphere-visible directional-light disks.",
    emit = None
)]
#[instrument(skip_all)]
async fn sun_disk(p: BevySunDiskParams) -> Result<CallToolResult, ErrorData> {
    validate_sun_disk(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "not_shadow_caster",
    description = "Emit the `NotShadowCaster` marker component to disable mesh shadow casting.",
    emit = None
)]
#[instrument(skip_all)]
async fn not_shadow_caster(p: BevyNotShadowCasterParams) -> Result<CallToolResult, ErrorData> {
    validate_not_shadow_caster(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "not_shadow_receiver",
    description = "Emit the `NotShadowReceiver` marker component to disable mesh shadow reception.",
    emit = None
)]
#[instrument(skip_all)]
async fn not_shadow_receiver(p: BevyNotShadowReceiverParams) -> Result<CallToolResult, ErrorData> {
    validate_not_shadow_receiver(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "transmitted_shadow_receiver",
    description = "Emit the `TransmittedShadowReceiver` marker component for diffuse transmission shadows.",
    emit = None
)]
#[instrument(skip_all)]
async fn transmitted_shadow_receiver(
    p: BevyTransmittedShadowReceiverParams,
) -> Result<CallToolResult, ErrorData> {
    validate_transmitted_shadow_receiver(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "shadow_filtering_method",
    description = "Emit a `ShadowFilteringMethod` camera component for shadow edge filtering mode.",
    emit = None
)]
#[instrument(skip_all)]
async fn shadow_filtering_method(
    p: BevyShadowFilteringMethodParams,
) -> Result<CallToolResult, ErrorData> {
    validate_shadow_filtering_method(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "cluster_far_z_mode",
    description = "Emit a `ClusterFarZMode` expression for clustered-lighting depth strategy.",
    emit = None
)]
#[instrument(skip_all)]
async fn cluster_far_z_mode(p: BevyClusterFarZModeParams) -> Result<CallToolResult, ErrorData> {
    validate_cluster_far_z_mode(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "cluster_z_config",
    description = "Emit a `ClusterZConfig` literal for clustered-lighting depth slicing.",
    emit = None
)]
#[instrument(skip_all)]
async fn cluster_z_config(p: BevyClusterZConfigParams) -> Result<CallToolResult, ErrorData> {
    validate_cluster_z_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "cluster_config",
    description = "Emit a `ClusterConfig` camera component for Bevy clustered-lighting strategy.",
    emit = None
)]
#[instrument(skip_all)]
async fn cluster_config(p: BevyClusterConfigParams) -> Result<CallToolResult, ErrorData> {
    validate_cluster_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "screen_space_reflections",
    description = "Emit a `ScreenSpaceReflections` camera component for deferred screen-space reflections.",
    emit = None
)]
#[instrument(skip_all)]
async fn screen_space_reflections(
    p: BevyScreenSpaceReflectionsParams,
) -> Result<CallToolResult, ErrorData> {
    validate_screen_space_reflections(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "clustered_decal",
    description = "Emit a `ClusteredDecal` component with optional texture handles and application tag.",
    emit = None
)]
#[instrument(skip_all)]
async fn clustered_decal(p: BevyClusteredDecalParams) -> Result<CallToolResult, ErrorData> {
    validate_clustered_decal(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_3d",
    description = "Emit a `Mesh3d` component from a mesh handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_3d(p: BevyMesh3dParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_3d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_2d",
    description = "Emit a `Mesh2d` component from a mesh handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_2d(p: BevyMesh2dParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_material_3d",
    description = "Emit a generic `MeshMaterial3d<M>` component from a typed material handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_material_3d(p: BevyMeshMaterial3dParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_material_3d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_material_2d",
    description = "Emit a generic `MeshMaterial2d<M>` component from a typed material handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_material_2d(p: BevyMeshMaterial2dParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_material_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe",
    description = "Emit the `Wireframe` marker component for 3D meshes.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe(p: BevyWireframeParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe_color",
    description = "Emit a `WireframeColor` component for overriding 3D wireframe tint.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe_color(p: BevyWireframeColorParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe_color(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "no_wireframe",
    description = "Emit the `NoWireframe` marker component to opt out of global 3D wireframes.",
    emit = None
)]
#[instrument(skip_all)]
async fn no_wireframe(p: BevyNoWireframeParams) -> Result<CallToolResult, ErrorData> {
    validate_no_wireframe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe_config",
    description = "Emit a `WireframeConfig` resource with optional global flag and fallback color.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe_config(p: BevyWireframeConfigParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_3d_wireframe",
    description = "Emit a `Mesh3dWireframe` component from a wireframe material handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_3d_wireframe(p: BevyMesh3dWireframeParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_3d_wireframe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe_2d",
    description = "Emit the `Wireframe2d` marker component for 2D meshes.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe_2d(p: BevyWireframe2dParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe_2d_color",
    description = "Emit a `Wireframe2dColor` component for overriding 2D wireframe tint.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe_2d_color(p: BevyWireframe2dColorParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe_2d_color(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "no_wireframe_2d",
    description = "Emit the `NoWireframe2d` marker component to opt out of global 2D wireframes.",
    emit = None
)]
#[instrument(skip_all)]
async fn no_wireframe_2d(p: BevyNoWireframe2dParams) -> Result<CallToolResult, ErrorData> {
    validate_no_wireframe_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "wireframe_2d_config",
    description = "Emit a `Wireframe2dConfig` resource with optional global flag and fallback color.",
    emit = None
)]
#[instrument(skip_all)]
async fn wireframe_2d_config(p: BevyWireframe2dConfigParams) -> Result<CallToolResult, ErrorData> {
    validate_wireframe_2d_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "mesh_2d_wireframe",
    description = "Emit a `Mesh2dWireframe` component from a 2D wireframe material handle expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn mesh_2d_wireframe(p: BevyMesh2dWireframeParams) -> Result<CallToolResult, ErrorData> {
    validate_mesh_2d_wireframe(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "camera_3d",
    description = "Emit a Bevy 0.18 camera spawn tuple with `Camera3d`, `Projection`, and `RenderTarget`.",
    emit = None
)]
#[instrument(skip_all)]
async fn camera_3d(p: BevyCamera3dParams) -> Result<CallToolResult, ErrorData> {
    validate_camera_3d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "camera_2d",
    description = "Emit a Bevy 0.18 camera spawn tuple with `Camera2d`, `Projection`, and `RenderTarget`.",
    emit = None
)]
#[instrument(skip_all)]
async fn camera_2d(p: BevyCamera2dParams) -> Result<CallToolResult, ErrorData> {
    validate_camera_2d(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "viewport",
    description = "Emit a `Viewport` literal with physical position, size, and depth range.",
    emit = None
)]
#[instrument(skip_all)]
async fn viewport(p: BevyViewportParams) -> Result<CallToolResult, ErrorData> {
    validate_viewport(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "fog_settings",
    description = "Emit a Bevy 0.18 `DistanceFog` literal; this replaces the older `FogSettings` surface.",
    emit = None
)]
#[instrument(skip_all)]
async fn fog_settings(p: BevyFogSettingsParams) -> Result<CallToolResult, ErrorData> {
    validate_fog_settings(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "bloom_composite_mode",
    description = "Emit a `BloomCompositeMode` enum expression.",
    emit = None
)]
#[instrument(skip_all)]
async fn bloom_composite_mode(
    p: BevyBloomCompositeModeParams,
) -> Result<CallToolResult, ErrorData> {
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "bloom_settings",
    description = "Emit a Bevy 0.18 `Bloom` value, starting from an optional preset and overriding common fields.",
    emit = None
)]
#[instrument(skip_all)]
async fn bloom_settings(p: BevyBloomSettingsParams) -> Result<CallToolResult, ErrorData> {
    validate_bloom_settings(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "sprite",
    description = "Emit a current Bevy `Sprite` literal with image, tint, flips, and optional custom size.",
    emit = None
)]
#[instrument(skip_all)]
async fn sprite(p: BevySpriteParams) -> Result<CallToolResult, ErrorData> {
    validate_sprite(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "text_style",
    description = "Emit the current Bevy text-style component tuple: `TextFont`, `TextColor`, and `TextLayout`.",
    emit = None
)]
#[instrument(skip_all)]
async fn text_style(p: BevyTextStyleParams) -> Result<CallToolResult, ErrorData> {
    validate_text_style(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "ssao_quality",
    description = "Emit a `ScreenSpaceAmbientOcclusionQualityLevel` enum expression, including custom slice/sample counts.",
    emit = None
)]
#[instrument(skip_all)]
async fn ssao_quality(p: BevySsaoQualityParams) -> Result<CallToolResult, ErrorData> {
    validate_ssao_quality(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "ssao",
    description = "Emit a `ScreenSpaceAmbientOcclusion` component literal for Bevy 0.18.",
    emit = None
)]
#[instrument(skip_all)]
async fn ssao(p: BevySsaoParams) -> Result<CallToolResult, ErrorData> {
    validate_ssao(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "temporal_anti_aliasing",
    description = "Emit the current Bevy 0.18 `TemporalAntiAliasing` component literal.",
    emit = None
)]
#[instrument(skip_all)]
async fn temporal_anti_aliasing(
    p: BevyTemporalAntiAliasingParams,
) -> Result<CallToolResult, ErrorData> {
    validate_temporal_anti_aliasing(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "cascade_shadow_config",
    description = "Emit a `CascadeShadowConfig` expression for Bevy 0.18 directional-light shadows.",
    emit = None
)]
#[instrument(skip_all)]
async fn cascade_shadow_config(
    p: BevyCascadeShadowConfigParams,
) -> Result<CallToolResult, ErrorData> {
    validate_cascade_shadow_config(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "cascade_shadow_config_builder",
    description = "Emit a `CascadeShadowConfigBuilder` literal with optional overrides over Bevy's defaults.",
    emit = None
)]
#[instrument(skip_all)]
async fn cascade_shadow_config_builder(
    p: BevyCascadeShadowConfigBuilderParams,
) -> Result<CallToolResult, ErrorData> {
    validate_cascade_shadow_config_builder(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "atmosphere",
    description = "Emit a block expression that adds a `ScatteringMedium` asset and returns a Bevy 0.18 `Atmosphere` component.",
    emit = None
)]
#[instrument(skip_all)]
async fn atmosphere(p: BevyAtmosphereParams) -> Result<CallToolResult, ErrorData> {
    validate_atmosphere(&p)?;
    ok_source(p.emit_code().to_string())
}

#[elicit_tool(
    plugin = "bevy_render",
    name = "fullscreen_material",
    description = "Emit `impl FullscreenMaterial` boilerplate for an existing fullscreen effect component.",
    emit = None
)]
#[instrument(skip_all)]
async fn fullscreen_material(p: BevyFullscreenMaterialParams) -> Result<CallToolResult, ErrorData> {
    validate_fullscreen_material(&p)?;
    ok_source(p.emit_code().to_string())
}
