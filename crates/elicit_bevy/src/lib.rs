//! `elicit_bevy` — Bevy 0.18 shadow crate for MCP-based game development.
//!
//! Each Bevy type is wrapped in an `Arc<T>` newtype.  The wrappers expose
//! every public instance method as an rmcp `#[tool]` via `#[reflect_methods]`,
//! so agents can manipulate Bevy values directly over the Model Context
//! Protocol without an ECS runtime.
//!
//! # Modules
//!
//! | Module | Coverage |
//! |--------|---------|
//! | [`math`] | Vec2/3/4, Quat, Mat*, Dir*, Ray*, Rect, Rot2, shapes |
//! | [`color`] | Color and all color spaces |
//! | [`transform`] | Transform, GlobalTransform |
//! | [`input`] | KeyCode, MouseButton, Gamepad* |
//! | [`input_focus`] | AutoFocus, TabIndex, TabGroup, InputFocusVisible, AutoNavigationConfig |
//! | [`time`] | Timer, TimerMode, Stopwatch |
//! | [`window`] | Window, WindowMode, PresentMode, … |
//! | [`camera`] | Camera, Projection, Visibility |
//! | [`pbr`] | StandardMaterial, AlphaMode, Tonemapping, FogFalloff |
//! | [`light`] | AmbientLight, DirectionalLight, PointLight, SpotLight |
//! | [`ui`] | Val, UiRect, BorderRadius, layout enums, FocusPolicy, BoxSizing, GridAutoFlow, ZIndex, GlobalZIndex, BackgroundColor, BorderColor, Outline, AutoDirectionalNavigation |
//! | [`sprite`] | Sprite, Anchor, SpriteImageMode, SpriteScalingMode, SpritePickingCamera, SpritePickingMode, SpritePickingSettings |
//! | [`text`] | TextFont, JustifyText, LineBreak |
//! | [`audio`] | PlaybackSettings, PlaybackMode, Volume |
//! | [`animation`] | RepeatAnimation, AnimationTargetId |
//! | [`anti_alias`] | Sensitivity, Fxaa, SmaaPreset, Smaa, ContrastAdaptiveSharpening |
//! | [`picking`] | Pickable, PickingInteraction |
//! | [`mesh`] | PrimitiveTopology, Indices |
//! | [`gizmos`] | GizmoLineStyle, GizmoLineJoint, GizmoLineConfig, GizmoConfig |
//! | [`post_process`] | AutoExposure, Bloom, BloomCompositeMode, BloomPrefilter, ChromaticAberration, DepthOfFieldMode, DepthOfField, MotionBlur |
//! | [`ecs`] | Entity |
//! | [`app`] | AppExit |
//! | [`asset`] | LoadState, RecursiveDependencyLoadState |
//! | [`render`] | Msaa, Hdr, ColorGrading, ColorGradingGlobal, ColorGradingSection, NoAutomaticBatching, MipBias, OcclusionCulling |
//! | [`scene`] | Scene / DynamicScene documentation |
//! | [`state`] | States factory documentation |
//! | [`derive_plugin`] | `bevy_derive__*` fragment tools for derive codegen |
//! | [`ecs_plugin`] | `bevy_ecs__*` fragment tools for ECS/app wiring |
//! | [`app_plugin`] | `bevy_app__*` descriptor-registry tools for app assembly |
//! | [`scene_plugin`] | `bevy_scene__*` descriptor-registry tools for scene manifests |
//! | [`query_plugin`] | `bevy_query__*` factory/codegen tools for generic ECS params |
//! | [`render_plugin`] | `bevy_render__*` core render/material descriptor tools |
//! | [`render_atmosphere_workflow_plugin`] | `bevy_render_atmosphere_workflow__*` stateful atmosphere tools |
//! | [`render_mesh_workflow_plugin`] | `bevy_render_mesh_workflow__*` stateful mesh-authoring tools |
//! | [`render_workflow_plugin`] | `bevy_render_workflow__*` stateful camera-authoring tools |
//! | [`ui_plugin`] | `bevy_ui__*` layout and widget descriptor tools |
//! | [`trait_factories`] | Component, Resource, Asset, Bundle, Event, States factories |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod animation;
pub mod anti_alias;
pub mod app;
mod app_plugin;
pub mod asset;
pub mod audio;
pub mod camera;
pub mod color;
mod derive_plugin;
pub mod ecs;
mod ecs_plugin;
pub mod gizmos;
pub mod image;
pub mod input;
pub mod input_focus;
pub mod light;
pub mod math;
pub mod mesh;
pub mod pbr;
pub mod picking;
pub mod post_process;
mod query_plugin;
pub mod render;
mod render_atmosphere_workflow_plugin;
mod render_mesh_workflow_plugin;
mod render_plugin;
mod render_workflow_plugin;
pub mod scene;
mod scene_plugin;
pub mod sprite;
pub mod state;
pub mod text;
pub mod time;
pub mod trait_factories;
pub mod transform;
pub mod ui;
mod ui_plugin;
pub mod window;

// ── derive plugin re-exports ───────────────────────────────────────────────────
pub use derive_plugin::{
    AssetDeriveParams, BevyDerivePlugin, BundleDeriveParams, ComponentDeriveParams,
    EnumVariantSpec, EventDeriveParams, ItemShape, ItemTemplate, NamedFieldSpec,
    ReflectDeriveParams, ResourceDeriveParams, ScheduleLabelDeriveParams, StatesDeriveParams,
    SystemSetDeriveParams, VariantShape,
};

// ── app plugin re-exports ──────────────────────────────────────────────────────
pub use app_plugin::{
    BevyAppAddDefaultPluginsParams, BevyAppAddPluginParams, BevyAppAddScheduleParams,
    BevyAppDescribeParams, BevyAppDescriptor, BevyAppEmitParams, BevyAppNewParams,
    BevyAppNewResult, BevyAppPlugin, BevyAppSetRunnerParams, BevyDefaultPluginsDescriptor,
    BevyPluginGroupParams, BevyPluginStructParams, BevyScheduleDescriptor, BevyStateHook,
    BevyStateMachineParams,
};

// ── scene plugin re-exports ────────────────────────────────────────────────────
pub use scene_plugin::{
    BevySceneAddEntityParams, BevySceneAddResourceParams, BevySceneDescriptor,
    BevySceneEmitRonParams, BevySceneEmitSpawnCodeParams, BevySceneEntityDescriptor,
    BevySceneNewParams, BevySceneNewResult, BevyScenePlugin, BevySceneValueDescriptor,
};

// ── query plugin re-exports ────────────────────────────────────────────────────
pub use query_plugin::{
    BevyQueryFilterKind, BevyQueryItemAccess, BevyQueryItemSpec, BevyQueryPlugin,
    DefineComponentQueryParams, DefineEventReaderParams, DefineEventWriterParams,
    DefineHandleParams, DefineLocalParams, DefineResourceParams, DefineTimeParams, FilterParams,
    SystemSignatureParams,
};

// ── render plugin re-exports ───────────────────────────────────────────────────
pub use render_plugin::{
    BevyAlphaMode2dParams, BevyAlphaMode2dVariant, BevyAmbientLightParams,
    BevyAtmosphereEnvironmentMapLightParams, BevyAtmosphereParams, BevyBloomCompositeModeParams,
    BevyBloomCompositeModeVariant, BevyBloomPreset, BevyBloomSettingsParams, BevyCamera2dParams,
    BevyCamera3dDepthLoadOpParams, BevyCamera3dDepthLoadOpVariant, BevyCamera3dParams,
    BevyCascadeShadowConfigBuilderParams, BevyCascadeShadowConfigParams,
    BevyClearColorConfigParams, BevyClearColorConfigVariant, BevyClearColorParams,
    BevyClusterConfigParams, BevyClusterFarZModeKind, BevyClusterFarZModeParams,
    BevyClusterZConfigParams, BevyClusteredDecalParams, BevyColorMaterialParams, BevyColorParams,
    BevyDebandDitherParams, BevyDebandDitherVariant, BevyDefaultOpaqueRendererMethodParams,
    BevyDefaultOpaqueRendererMethodVariant, BevyDeferredPrepassDoubleBufferParams,
    BevyDeferredPrepassParams, BevyDepthPrepassDoubleBufferParams, BevyDepthPrepassParams,
    BevyDirectionalLightParams, BevyDirectionalLightShadowMapParams, BevyEnvironmentMapLightParams,
    BevyExposureParams, BevyExposurePreset, BevyFogSettingsParams, BevyFogVolumeParams,
    BevyFullscreenGraphKind, BevyFullscreenMaterialParams, BevyGeneratedEnvironmentMapLightParams,
    BevyGlobalAmbientLightParams, BevyIrradianceVolumeParams, BevyLightProbeParams,
    BevyLightmapParams, BevyMainPassResolutionOverrideParams, BevyMesh2dParams,
    BevyMesh2dWireframeParams, BevyMesh3dParams, BevyMesh3dWireframeParams,
    BevyMeshMaterial2dParams, BevyMeshMaterial3dParams, BevyMotionVectorPrepassParams,
    BevyMsaaWritebackParams, BevyMsaaWritebackVariant, BevyNoCpuCullingParams,
    BevyNoFrustumCullingParams, BevyNoWireframe2dParams, BevyNoWireframeParams,
    BevyNormalPrepassParams, BevyNotShadowCasterParams, BevyNotShadowReceiverParams,
    BevyOpaqueRendererMethodParams, BevyOpaqueRendererMethodVariant,
    BevyOrderIndependentTransparencySettingsParams, BevyOrthographicProjectionParams,
    BevyParallaxMappingMethodParams, BevyParallaxMappingMethodVariant,
    BevyPerspectiveProjectionParams, BevyPointLightParams, BevyPointLightShadowMapParams,
    BevyRenderAlphaModeParams, BevyRenderAlphaModeVariant, BevyRenderPlugin, BevyRenderTargetKind,
    BevyRenderTargetParams, BevyScalingModeParams, BevyScalingModeVariant,
    BevyScreenSpaceReflectionsParams, BevyScreenSpaceTransmissionQualityParams,
    BevyScreenSpaceTransmissionQualityVariant, BevyShadowFilteringMethodParams,
    BevyShadowFilteringMethodVariant, BevySkyboxParams, BevySpotLightParams, BevySpriteParams,
    BevySsaoParams, BevySsaoQualityParams, BevySsaoQualityVariant, BevyStandardMaterialParams,
    BevySubCameraViewParams, BevySunDiskParams, BevyTemporalAntiAliasingParams,
    BevyTextStyleParams, BevyTonemappingParams, BevyTonemappingVariant,
    BevyTransmittedShadowReceiverParams, BevyUvChannelParams, BevyUvChannelVariant,
    BevyViewportParams, BevyVisibilityRangeParams, BevyVolumetricFogParams,
    BevyVolumetricLightParams, BevyWireframe2dColorParams, BevyWireframe2dConfigParams,
    BevyWireframe2dParams, BevyWireframeColorParams, BevyWireframeConfigParams,
    BevyWireframeParams,
};

// ── render atmosphere workflow plugin re-exports ───────────────────────────────
pub use render_atmosphere_workflow_plugin::{
    BevyRenderAtmosphereWorkflowAddTermParams, BevyRenderAtmosphereWorkflowClearTermsParams,
    BevyRenderAtmosphereWorkflowDescribeParams, BevyRenderAtmosphereWorkflowEmitCodeParams,
    BevyRenderAtmosphereWorkflowNewParams, BevyRenderAtmosphereWorkflowNewResult,
    BevyRenderAtmosphereWorkflowPlugin, BevyRenderAtmosphereWorkflowRemoveTermParams,
    BevyRenderAtmosphereWorkflowReplaceTermParams, BevyRenderAtmosphereWorkflowSetAtmosphereParams,
    BevyRenderAtmosphereWorkflowSetDensityMultiplierParams,
    BevyRenderAtmosphereWorkflowSetMediumLabelParams,
    BevyRenderAtmosphereWorkflowSetResolutionsParams,
    BevyRenderAtmosphereWorkflowSetScatteringMediaVarParams,
};

// ── render mesh workflow plugin re-exports ─────────────────────────────────────
pub use render_mesh_workflow_plugin::{
    BevyRenderMeshWorkflowDescribeParams, BevyRenderMeshWorkflowDescriptor,
    BevyRenderMeshWorkflowEmitSpawnCodeParams, BevyRenderMeshWorkflowKind,
    BevyRenderMeshWorkflowNewParams, BevyRenderMeshWorkflowNewResult, BevyRenderMeshWorkflowPlugin,
    BevyRenderMeshWorkflowSetMaterialParams, BevyRenderMeshWorkflowSetMeshParams,
    BevyRenderMeshWorkflowSetTransformParams, BevyRenderMeshWorkflowSetWireframeMaterialParams,
};

// ── render workflow plugin re-exports ──────────────────────────────────────────
pub use render_workflow_plugin::{
    BevyRenderWorkflowCameraDescriptor, BevyRenderWorkflowCameraKind,
    BevyRenderWorkflowDescribeParams, BevyRenderWorkflowEmitSpawnCodeParams,
    BevyRenderWorkflowNewCameraParams, BevyRenderWorkflowNewResult, BevyRenderWorkflowPlugin,
    BevyRenderWorkflowSetHdrParams, BevyRenderWorkflowSetOrthographicProjectionParams,
    BevyRenderWorkflowSetPerspectiveProjectionParams, BevyRenderWorkflowSetRenderTargetParams,
    BevyRenderWorkflowSetTonemappingParams, BevyRenderWorkflowSetTransformParams,
};

// ── ui plugin re-exports ───────────────────────────────────────────────────────
pub use ui_plugin::{
    BevyGridPlacementKind, BevyGridPlacementParams, BevyUiButtonBundleParams,
    BevyUiFlexContainerParams, BevyUiGridContainerParams, BevyUiImageParams,
    BevyUiNodeLiteralParams, BevyUiNodeParams, BevyUiPlugin, BevyUiRectParams, BevyUiTextParams,
};

// ── ecs plugin re-exports ──────────────────────────────────────────────────────
pub use ecs_plugin::{
    AddEventParams, AddPluginsParams, AddSystemsParams, BevyEcsPlugin, ChainParams, DespawnParams,
    InSetParams, InitResourceParams, InsertComponentParams, InsertResourceParams, ObserverParams,
    PipeParams, QueryForParams, QueryItemSpec, RegisterTypeParams, RemoveComponentParams,
    RunCriteriaParams, SpawnBundleParams, SpawnEntityParams, TriggerParams, WithChildrenParams,
};

// ── ecs re-exports ───────────────────────────────────────────────────────────
pub use ecs::Entity;

// ── scene re-exports ─────────────────────────────────────────────────────────
pub use scene::{Name, SceneInstanceReady};

// ── app re-exports ───────────────────────────────────────────────────────────
pub use app::AppExit;

// ── asset re-exports ─────────────────────────────────────────────────────────
pub use asset::{LoadState, RecursiveDependencyLoadState};

// ── image re-exports ──────────────────────────────────────────────────────────
pub use image::{
    ImageAddressMode, ImageCompareFunction, ImageFilterMode, ImageSampler, ImageSamplerBorderColor,
    ImageSamplerDescriptor,
};

// ── animation re-exports ─────────────────────────────────────────────────────
pub use animation::{AnimationPlayer, AnimationTargetId, AnimationTransitions, RepeatAnimation};

// ── anti_alias re-exports ─────────────────────────────────────────────────────
pub use anti_alias::{ContrastAdaptiveSharpening, Fxaa, Sensitivity, Smaa, SmaaPreset};

// ── audio re-exports ─────────────────────────────────────────────────────────
pub use audio::{
    DefaultSpatialScale, GlobalVolume, PlaybackMode, PlaybackSettings, SpatialListener,
    SpatialScale, Volume,
};

// ── color re-exports ─────────────────────────────────────────────────────────
pub use color::{Color, Hsla, Hsva, Hwba, Laba, Lcha, LinearRgba, Oklaba, Oklcha, Srgba, Xyza};

// ── light re-exports ─────────────────────────────────────────────────────────
pub use light::{
    AmbientLight, DirectionalLight, EnvironmentMapLight, GeneratedEnvironmentMapLight,
    GlobalAmbientLight, LightProbe, PointLight, ShadowFilteringMethod, SpotLight,
};

// ── math re-exports ──────────────────────────────────────────────────────────
pub use math::{
    Affine2, Affine3A, Annulus, Arc2d, Capsule2d, Capsule3d, Circle, Cone, ConicalFrustum, Cuboid,
    Cylinder, DAffine2, DAffine3, DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, Dir2, Dir3,
    Dir3A, IRect, IVec2, IVec3, IVec4, Isometry2d, Isometry3d, Mat2, Mat3, Mat3A, Mat4, Plane2d,
    Plane3d, Quat, Ray2d, Ray3d, Rect, Rectangle, RegularPolygon, Rhombus, Rot2, Segment2d, Sphere,
    Tetrahedron, Torus, Triangle2d, Triangle3d, URect, UVec2, UVec3, UVec4, Vec2, Vec3, Vec3A,
    Vec4,
};

// ── render re-exports ─────────────────────────────────────────────────────────
pub use render::{
    ColorGrading, ColorGradingGlobal, ColorGradingSection, Hdr, MipBias, Msaa, NoAutomaticBatching,
    OcclusionCulling,
};

// ── mesh re-exports ───────────────────────────────────────────────────────────
pub use mesh::{Indices, PrimitiveTopology};

// ── pbr re-exports ────────────────────────────────────────────────────────────
pub use pbr::{
    AlphaMode, AtmosphereMode, AtmosphereSettings, DistanceFog, FogFalloff, ForwardDecal,
    ForwardDecalMaterialExt, OpaqueRendererMethod, ParallaxMappingMethod, StandardMaterial,
    Tonemapping, UvChannel,
};

// ── gizmos re-exports ─────────────────────────────────────────────────────────
pub use gizmos::{
    AabbGizmoConfigGroup, GizmoConfig, GizmoLineConfig, GizmoLineJoint, GizmoLineStyle,
    LightGizmoColor, LightGizmoConfigGroup, ShowAabbGizmo, ShowLightGizmo,
};

// ── post_process re-exports ───────────────────────────────────────────────────
pub use post_process::{
    AutoExposure, Bloom, BloomCompositeMode, BloomPrefilter, ChromaticAberration, DepthOfField,
    DepthOfFieldMode, MotionBlur,
};

// ── camera re-exports ─────────────────────────────────────────────────────────
pub use camera::{
    Camera, Camera2d, Camera3d, Camera3dDepthLoadOp, ClearColor, ClearColorConfig,
    InheritedVisibility, MsaaWriteback, OrthographicProjection, PerspectiveProjection,
    PhysicalCameraParameters, Projection, ScalingMode, ScreenSpaceTransmissionQuality,
    ViewVisibility, Visibility,
};

// ── input re-exports ──────────────────────────────────────────────────────────
pub use input::{
    ButtonState, GamepadAxis, GamepadButton, GamepadConnection, GamepadInput,
    GamepadRumbleIntensity, KeyCode, MouseButton, MouseScrollUnit, TouchPhase,
};

// ── input_focus re-exports ────────────────────────────────────────────────────
pub use input_focus::{AutoFocus, AutoNavigationConfig, InputFocusVisible, TabGroup, TabIndex};

// ── transform re-exports ──────────────────────────────────────────────────────
pub use transform::{GlobalTransform, Transform};

// ── time re-exports ───────────────────────────────────────────────────────────
pub use time::{Stopwatch, Timer, TimerMode};

// ── window re-exports ─────────────────────────────────────────────────────────
pub use window::{
    CompositeAlphaMode, CursorGrabMode, CursorIcon, CursorOptions, EnabledButtons,
    MonitorSelection, PresentMode, PrimaryWindow, ScreenEdge, SystemCursorIcon, Window,
    WindowLevel, WindowMode, WindowPosition, WindowResizeConstraints, WindowResolution,
    WindowTheme,
};

// ── state re-exports ──────────────────────────────────────────────────────────
pub use picking::{
    DirectlyHovered, Hovered, Pickable, PickingInteraction, PickingSettings, PointerButton,
    PointerId, PressDirection,
};
pub use state::StateTransitionDoc;

// ── sprite re-exports ────────────────────────────────────────────────────────
pub use sprite::{
    Anchor, BorderRect, SliceScaleMode, Sprite, SpriteImageMode, SpritePickingCamera,
    SpritePickingMode, SpritePickingSettings, SpriteScalingMode, Text2d, Text2dShadow,
    TextureSlicer,
};

// ── text re-exports ───────────────────────────────────────────────────────────
pub use text::{
    FontHinting, FontSmoothing, FontWeight, JustifyText, LineBreak, Strikethrough,
    StrikethroughColor, TextBackgroundColor, TextBounds, TextColor, TextFont, TextLayout, TextSpan,
    Underline, UnderlineColor,
};

// ── ui re-exports ─────────────────────────────────────────────────────────────
pub use ui::{
    AlignContent, AlignItems, AlignSelf, AngularColorStop, AutoDirectionalNavigation,
    BackgroundColor, BackgroundGradient, BorderColor, BorderGradient, BorderRadius, BoxShadow,
    BoxSizing, Button, Checkable, Checked, ColorStop, ConicGradient, Display, FlexDirection,
    FlexWrap, FocusPolicy, GlobalZIndex, Gradient, GridAutoFlow, GridTrack, GridTrackRepetition,
    IgnoreScroll, ImageNode, Interaction, InteractionDisabled, InterpolationColorSpace,
    IsDefaultUiCamera, JustifyContent, JustifyItems, JustifySelf, Label, LinearGradient,
    MaxTrackSizingFunction, MinTrackSizingFunction, NodeImageMode, Outline, Overflow, OverflowAxis,
    OverflowClipBox, OverflowClipMargin, OverrideClip, PositionType, Pressed, RadialGradient,
    RadialGradientShape, RepeatedGridTrack, ScrollPosition, ShadowStyle, TextShadow, UiPosition,
    UiRect, UiScale, UiText, Val, ZIndex,
};
