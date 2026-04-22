//! Integration tests for `BevyRenderPlugin`.

use elicit_bevy::{
    BevyAlphaMode2dParams, BevyAmbientLightParams, BevyAtmosphereEnvironmentMapLightParams,
    BevyBloomCompositeModeParams, BevyBloomCompositeModeVariant, BevyBloomPreset,
    BevyBloomSettingsParams, BevyCamera3dDepthLoadOpParams, BevyCamera3dParams,
    BevyCascadeShadowConfigBuilderParams, BevyCascadeShadowConfigParams,
    BevyClearColorConfigParams, BevyClearColorParams, BevyClusterConfigParams,
    BevyClusterFarZModeParams, BevyClusterZConfigParams, BevyClusteredDecalParams,
    BevyColorMaterialParams, BevyColorParams, BevyDebandDitherParams, BevyDebandDitherVariant,
    BevyDefaultOpaqueRendererMethodParams, BevyDefaultOpaqueRendererMethodVariant,
    BevyDeferredPrepassDoubleBufferParams, BevyDeferredPrepassParams,
    BevyDepthPrepassDoubleBufferParams, BevyDepthPrepassParams,
    BevyDirectionalLightShadowMapParams, BevyEnvironmentMapLightParams, BevyExposureParams,
    BevyFogSettingsParams, BevyFogVolumeParams, BevyFullscreenGraphKind,
    BevyFullscreenMaterialParams, BevyGeneratedEnvironmentMapLightParams,
    BevyGlobalAmbientLightParams, BevyIrradianceVolumeParams, BevyLightProbeParams,
    BevyLightmapParams, BevyMainPassResolutionOverrideParams, BevyMesh2dParams,
    BevyMesh2dWireframeParams, BevyMesh3dParams, BevyMesh3dWireframeParams,
    BevyMeshMaterial2dParams, BevyMeshMaterial3dParams, BevyMotionVectorPrepassParams,
    BevyMsaaWritebackParams, BevyMsaaWritebackVariant, BevyNoCpuCullingParams,
    BevyNoFrustumCullingParams, BevyNoWireframe2dParams, BevyNoWireframeParams,
    BevyNormalPrepassParams, BevyNotShadowCasterParams, BevyNotShadowReceiverParams,
    BevyOpaqueRendererMethodParams, BevyOpaqueRendererMethodVariant,
    BevyOrderIndependentTransparencySettingsParams, BevyOrthographicProjectionParams,
    BevyParallaxMappingMethodParams, BevyPerspectiveProjectionParams,
    BevyPointLightShadowMapParams, BevyRenderAlphaModeParams, BevyRenderPlugin,
    BevyRenderTargetParams, BevyScalingModeParams, BevyScreenSpaceReflectionsParams,
    BevyScreenSpaceTransmissionQualityParams, BevyScreenSpaceTransmissionQualityVariant,
    BevyShadowFilteringMethodParams, BevyShadowFilteringMethodVariant, BevySkyboxParams,
    BevySpriteParams, BevySsaoParams, BevySsaoQualityParams, BevyStandardMaterialParams,
    BevySubCameraViewParams, BevySunDiskParams, BevyTemporalAntiAliasingParams,
    BevyTextStyleParams, BevyTonemappingParams, BevyTonemappingVariant,
    BevyTransmittedShadowReceiverParams, BevyUvChannelParams, BevyUvChannelVariant,
    BevyViewportParams, BevyVisibilityRangeParams, BevyVolumetricFogParams,
    BevyVolumetricLightParams, BevyWireframe2dColorParams, BevyWireframe2dConfigParams,
    BevyWireframe2dParams, BevyWireframeColorParams, BevyWireframeConfigParams,
    BevyWireframeParams,
};
use elicitation::ElicitPlugin;
use elicitation::emit_code::{EmitCode, dispatch_emit_from};

fn normalize(source: &str) -> String {
    source.chars().filter(|c| !c.is_whitespace()).collect()
}

fn from_json<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> T {
    serde_json::from_value(value).expect("valid test params")
}

#[test]
fn render_plugin_lists_expected_tools() {
    let plugin = BevyRenderPlugin::new();
    let mut names = plugin
        .list_tools()
        .into_iter()
        .map(|tool| tool.name.to_string())
        .collect::<Vec<_>>();
    names.sort();

    assert_eq!(
        names,
        vec![
            "alpha_mode",
            "alpha_mode_2d",
            "ambient_light",
            "atmosphere",
            "atmosphere_environment_map_light",
            "bloom_composite_mode",
            "bloom_settings",
            "camera_2d",
            "camera_3d",
            "camera_3d_depth_load_op",
            "cascade_shadow_config",
            "cascade_shadow_config_builder",
            "clear_color",
            "clear_color_config",
            "cluster_config",
            "cluster_far_z_mode",
            "cluster_z_config",
            "clustered_decal",
            "color",
            "color_material",
            "deband_dither",
            "default_opaque_renderer_method",
            "deferred_prepass",
            "deferred_prepass_double_buffer",
            "depth_prepass",
            "depth_prepass_double_buffer",
            "directional_light",
            "directional_light_shadow_map",
            "environment_map_light",
            "exposure",
            "fog_settings",
            "fog_volume",
            "fullscreen_material",
            "generated_environment_map_light",
            "global_ambient_light",
            "irradiance_volume",
            "light_probe",
            "lightmap",
            "main_pass_resolution_override",
            "mesh_2d",
            "mesh_2d_wireframe",
            "mesh_3d",
            "mesh_3d_wireframe",
            "mesh_material_2d",
            "mesh_material_3d",
            "motion_vector_prepass",
            "msaa_writeback",
            "no_cpu_culling",
            "no_frustum_culling",
            "no_wireframe",
            "no_wireframe_2d",
            "normal_prepass",
            "not_shadow_caster",
            "not_shadow_receiver",
            "opaque_renderer_method",
            "order_independent_transparency_settings",
            "orthographic_projection",
            "parallax_mapping_method",
            "perspective_projection",
            "point_light",
            "point_light_shadow_map",
            "render_target",
            "scaling_mode",
            "screen_space_reflections",
            "screen_space_transmission_quality",
            "shadow_filtering_method",
            "skybox",
            "spot_light",
            "sprite",
            "ssao",
            "ssao_quality",
            "standard_material",
            "sub_camera_view",
            "sun_disk",
            "temporal_anti_aliasing",
            "text_style",
            "tonemapping",
            "transmitted_shadow_receiver",
            "uv_channel",
            "viewport",
            "visibility_range",
            "volumetric_fog",
            "volumetric_light",
            "wireframe",
            "wireframe_2d",
            "wireframe_2d_color",
            "wireframe_2d_config",
            "wireframe_color",
            "wireframe_config",
        ]
    );
}

#[test]
fn standard_material_params_emit_extended_surface_fields() {
    let params = BevyStandardMaterialParams {
        base_color_expr: Some("Color::srgb(0.9, 0.7, 0.6)".into()),
        base_color_channel: Some(BevyUvChannelVariant::Uv1),
        base_color_texture_expr: Some("asset_server.load(\"materials/base.png\")".into()),
        emissive_expr: Some("LinearRgba::rgb(0.1, 0.0, 0.0)".into()),
        emissive_channel: Some(BevyUvChannelVariant::Uv1),
        emissive_texture_expr: Some("asset_server.load(\"materials/emissive.png\")".into()),
        emissive_exposure_weight: Some(0.25),
        metallic: Some(0.65),
        perceptual_roughness: Some(0.2),
        metallic_roughness_channel: Some(BevyUvChannelVariant::Uv1),
        metallic_roughness_texture_expr: Some(
            "asset_server.load(\"materials/metal_rough.png\")".into(),
        ),
        reflectance: Some(0.55),
        specular_tint_expr: Some("Color::srgb(0.95, 0.9, 0.85)".into()),
        diffuse_transmission: Some(0.35),
        diffuse_transmission_channel: Some(BevyUvChannelVariant::Uv1),
        diffuse_transmission_texture_expr: Some(
            "asset_server.load(\"materials/diffuse_transmission.png\")".into(),
        ),
        specular_transmission: Some(0.6),
        specular_transmission_channel: Some(BevyUvChannelVariant::Uv1),
        specular_transmission_texture_expr: Some(
            "asset_server.load(\"materials/specular_transmission.png\")".into(),
        ),
        thickness: Some(0.15),
        thickness_channel: Some(BevyUvChannelVariant::Uv1),
        thickness_texture_expr: Some("asset_server.load(\"materials/thickness.png\")".into()),
        ior: Some(1.45),
        attenuation_distance: Some(12.0),
        attenuation_color_expr: Some("Color::srgb(0.85, 0.95, 1.0)".into()),
        clearcoat: Some(0.4),
        clearcoat_channel: Some(BevyUvChannelVariant::Uv1),
        clearcoat_texture_expr: Some("asset_server.load(\"materials/clearcoat.png\")".into()),
        clearcoat_perceptual_roughness: Some(0.3),
        clearcoat_roughness_channel: Some(BevyUvChannelVariant::Uv1),
        clearcoat_roughness_texture_expr: Some(
            "asset_server.load(\"materials/clearcoat_roughness.png\")".into(),
        ),
        clearcoat_normal_channel: Some(BevyUvChannelVariant::Uv1),
        clearcoat_normal_texture_expr: Some(
            "asset_server.load(\"materials/clearcoat_normal.png\")".into(),
        ),
        anisotropy_strength: Some(0.7),
        anisotropy_rotation: Some(1.57),
        anisotropy_channel: Some(BevyUvChannelVariant::Uv1),
        anisotropy_texture_expr: Some("asset_server.load(\"materials/anisotropy.png\")".into()),
        normal_map_channel: Some(BevyUvChannelVariant::Uv1),
        normal_map_texture_expr: Some("asset_server.load(\"materials/normal.png\")".into()),
        flip_normal_map_y: Some(true),
        occlusion_channel: Some(BevyUvChannelVariant::Uv1),
        occlusion_texture_expr: Some("asset_server.load(\"materials/ao.png\")".into()),
        specular_channel: Some(BevyUvChannelVariant::Uv1),
        specular_texture_expr: Some("asset_server.load(\"materials/specular.png\")".into()),
        specular_tint_channel: Some(BevyUvChannelVariant::Uv1),
        specular_tint_texture_expr: Some(
            "asset_server.load(\"materials/specular_tint.png\")".into(),
        ),
        alpha_mode_expr: Some("AlphaMode::Blend".into()),
        double_sided: Some(true),
        unlit: Some(false),
        fog_enabled: Some(true),
        cull_mode_expr: Some("Face::Back".into()),
        depth_bias: Some(1.5),
        depth_map_expr: Some("asset_server.load(\"materials/depth.png\")".into()),
        parallax_depth_scale: Some(0.08),
        parallax_mapping_method_expr: Some("ParallaxMappingMethod::Occlusion".into()),
        max_parallax_layer_count: Some(24.0),
        lightmap_exposure: Some(1.25),
        opaque_render_method_expr: Some("OpaqueRendererMethod::Deferred".into()),
        deferred_lighting_pass_id: Some(2),
        uv_transform_expr: Some("Affine2::IDENTITY".into()),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::pbr::StandardMaterial{"));
    assert!(source.contains("base_color:Color::srgb(0.9,0.7,0.6),"));
    assert!(source.contains("base_color_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains("base_color_texture:Some(asset_server.load(\"materials/base.png\")),"));
    assert!(source.contains("emissive:LinearRgba::rgb(0.1,0.0,0.0),"));
    assert!(source.contains("emissive_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source.contains("emissive_texture:Some(asset_server.load(\"materials/emissive.png\")),")
    );
    assert!(source.contains("emissive_exposure_weight:0.25"));
    assert!(source.contains("metallic:0.65"));
    assert!(source.contains("perceptual_roughness:0.2"));
    assert!(source.contains("metallic_roughness_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "metallic_roughness_texture:Some(asset_server.load(\"materials/metal_rough.png\")),"
    ));
    assert!(source.contains("reflectance:0.55"));
    assert!(source.contains("specular_tint:Color::srgb(0.95,0.9,0.85),"));
    assert!(source.contains("diffuse_transmission:0.35"));
    assert!(source.contains("diffuse_transmission_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "diffuse_transmission_texture:Some(asset_server.load(\"materials/diffuse_transmission.png\")),"
    ));
    assert!(source.contains("specular_transmission:0.6"));
    assert!(source.contains("specular_transmission_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "specular_transmission_texture:Some(asset_server.load(\"materials/specular_transmission.png\")),"
    ));
    assert!(source.contains("thickness:0.15"));
    assert!(source.contains("thickness_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source.contains("thickness_texture:Some(asset_server.load(\"materials/thickness.png\")),")
    );
    assert!(source.contains("ior:1.45"));
    assert!(source.contains("attenuation_distance:12"));
    assert!(source.contains("attenuation_color:Color::srgb(0.85,0.95,1.0),"));
    assert!(source.contains("clearcoat:0.4"));
    assert!(source.contains("clearcoat_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source.contains("clearcoat_texture:Some(asset_server.load(\"materials/clearcoat.png\")),")
    );
    assert!(source.contains("clearcoat_perceptual_roughness:0.3"));
    assert!(source.contains("clearcoat_roughness_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "clearcoat_roughness_texture:Some(asset_server.load(\"materials/clearcoat_roughness.png\")),"
    ));
    assert!(source.contains("clearcoat_normal_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "clearcoat_normal_texture:Some(asset_server.load(\"materials/clearcoat_normal.png\")),"
    ));
    assert!(source.contains("anisotropy_strength:0.7"));
    assert!(source.contains("anisotropy_rotation:1.57"));
    assert!(source.contains("anisotropy_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source
            .contains("anisotropy_texture:Some(asset_server.load(\"materials/anisotropy.png\")),")
    );
    assert!(source.contains("normal_map_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source.contains("normal_map_texture:Some(asset_server.load(\"materials/normal.png\")),")
    );
    assert!(source.contains("flip_normal_map_y:true"));
    assert!(source.contains("occlusion_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains("occlusion_texture:Some(asset_server.load(\"materials/ao.png\")),"));
    assert!(source.contains("specular_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(
        source.contains("specular_texture:Some(asset_server.load(\"materials/specular.png\")),")
    );
    assert!(source.contains("specular_tint_channel:::bevy::pbr::UvChannel::Uv1,"));
    assert!(source.contains(
        "specular_tint_texture:Some(asset_server.load(\"materials/specular_tint.png\")),"
    ));
    assert!(source.contains("alpha_mode:AlphaMode::Blend,"));
    assert!(source.contains("double_sided:true"));
    assert!(source.contains("fog_enabled:true"));
    assert!(source.contains("cull_mode:Some(Face::Back),"));
    assert!(source.contains("depth_bias:1.5"));
    assert!(source.contains("depth_map:Some(asset_server.load(\"materials/depth.png\")),"));
    assert!(source.contains("parallax_depth_scale:0.08"));
    assert!(source.contains("parallax_mapping_method:ParallaxMappingMethod::Occlusion,"));
    assert!(source.contains("max_parallax_layer_count:24"));
    assert!(source.contains("lightmap_exposure:1.25"));
    assert!(source.contains("opaque_render_method:OpaqueRendererMethod::Deferred,"));
    assert!(source.contains("deferred_lighting_pass_id:2u8"));
    assert!(source.contains("uv_transform:Affine2::IDENTITY,"));
}

#[test]
fn render_target_params_emit_image_target() {
    let params: BevyRenderTargetParams = from_json(serde_json::json!({
        "kind": "image",
        "target_expr": "post_process_image"
    }));
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::camera::RenderTarget::Image((post_process_image).into())"));
}

#[test]
fn alpha_mode_and_tonemapping_emit_expected_variants() {
    let alpha = BevyRenderAlphaModeParams::Mask { threshold: 0.33 };
    let tonemapping = BevyTonemappingParams {
        variant: BevyTonemappingVariant::AgX,
    };

    let alpha_source = normalize(&alpha.emit_code().to_string());
    let tonemapping_source = normalize(&tonemapping.emit_code().to_string());

    assert!(alpha_source.contains("::bevy::render::alpha::AlphaMode::Mask(0.33"));
    assert!(tonemapping_source.contains("::bevy::core_pipeline::tonemapping::Tonemapping::AgX"));
}

#[test]
fn deband_dither_and_oit_helpers_emit_current_bevy_paths() {
    let deband = BevyDebandDitherParams {
        variant: BevyDebandDitherVariant::Enabled,
    };
    let oit = BevyOrderIndependentTransparencySettingsParams {
        layer_count: Some(16),
        alpha_threshold: Some(0.2),
    };

    let deband_source = normalize(&deband.emit_code().to_string());
    let oit_source = normalize(&oit.emit_code().to_string());

    assert!(deband_source.contains("::bevy::core_pipeline::tonemapping::DebandDither::Enabled"));
    assert!(
        oit_source.contains("::bevy::core_pipeline::oit::OrderIndependentTransparencySettings{")
    );
    assert!(oit_source.contains("layer_count:16i32,"));
    assert!(oit_source.contains("alpha_threshold:0.2f32,"));
    assert!(oit_source.contains("..::std::default::Default::default()"));
}

#[test]
fn projection_helpers_emit_current_bevy_paths() {
    let scaling_mode = BevyScalingModeParams::AutoMin {
        min_width: 320.0,
        min_height: 180.0,
    };
    let perspective = BevyPerspectiveProjectionParams {
        fov: Some(1.0),
        aspect_ratio: Some(1.7777778),
        near: Some(0.2),
        far: Some(1500.0),
        near_clip_plane_expr: Some("Vec4::new(0.0, 0.0, -1.0, -0.2)".into()),
    };
    let orthographic: BevyOrthographicProjectionParams = from_json(serde_json::json!({
        "use_2d_defaults": true,
        "near": -500.0,
        "far": 500.0,
        "viewport_origin_expr": "Vec2::new(0.0, 1.0)",
        "scaling_mode_expr": "::bevy::camera::ScalingMode::FixedVertical{viewport_height:240.0}",
        "scale": 0.5,
        "area_expr": "Rect::new(-100.0, -50.0, 100.0, 50.0)"
    }));

    let scaling_source = normalize(&scaling_mode.emit_code().to_string());
    let perspective_source = normalize(&perspective.emit_code().to_string());
    let orthographic_source = normalize(&orthographic.emit_code().to_string());

    assert!(scaling_source.contains("::bevy::camera::ScalingMode::AutoMin{"));
    assert!(scaling_source.contains("min_width:320"));
    assert!(scaling_source.contains("min_height:180"));
    assert!(perspective_source.contains("::bevy::camera::PerspectiveProjection{"));
    assert!(perspective_source.contains("fov:1"));
    assert!(perspective_source.contains("aspect_ratio:1.7777778f32,"));
    assert!(perspective_source.contains("near_clip_plane:Vec4::new(0.0,0.0,-1.0,-0.2),"));
    assert!(perspective_source.contains("..::bevy::camera::PerspectiveProjection::default()"));
    assert!(orthographic_source.contains("::bevy::camera::OrthographicProjection{"));
    assert!(orthographic_source.contains("near:-500"));
    assert!(orthographic_source.contains("viewport_origin:Vec2::new(0.0,1.0),"));
    assert!(orthographic_source.contains(
        "scaling_mode:::bevy::camera::ScalingMode::FixedVertical{viewport_height:240.0},"
    ));
    assert!(orthographic_source.contains("area:Rect::new(-100.0,-50.0,100.0,50.0),"));
    assert!(orthographic_source.contains("..::bevy::camera::OrthographicProjection::default_2d()"));
}

#[test]
fn camera_setting_helpers_emit_current_bevy_paths() {
    let clear_color_config =
        BevyClearColorConfigParams::Custom("Color::srgb(0.02, 0.03, 0.05)".into());
    let msaa_writeback = BevyMsaaWritebackParams {
        variant: BevyMsaaWritebackVariant::Always,
    };
    let exposure = from_json::<BevyExposureParams>(serde_json::json!({
        "preset": "sunlight"
    }));
    let clear_color = BevyClearColorParams {
        color_expr: Some("Color::srgb(0.1, 0.15, 0.2)".into()),
    };

    let clear_color_config_source = normalize(&clear_color_config.emit_code().to_string());
    let msaa_writeback_source = normalize(&msaa_writeback.emit_code().to_string());
    let exposure_source = normalize(&exposure.emit_code().to_string());
    let clear_color_source = normalize(&clear_color.emit_code().to_string());

    assert!(
        clear_color_config_source
            .contains("::bevy::camera::ClearColorConfig::Custom(Color::srgb(0.02,0.03,0.05))")
    );
    assert!(msaa_writeback_source.contains("::bevy::camera::MsaaWriteback::Always"));
    assert!(exposure_source.contains("::bevy::camera::Exposure::SUNLIGHT"));
    assert!(clear_color_source.contains("::bevy::camera::ClearColor(Color::srgb(0.1,0.15,0.2))"));
}

#[test]
fn camera_view_helpers_emit_current_bevy_paths() {
    let depth_load = BevyCamera3dDepthLoadOpParams::Clear { depth: 0.25 };
    let transmission_quality = BevyScreenSpaceTransmissionQualityParams {
        variant: BevyScreenSpaceTransmissionQualityVariant::Ultra,
    };
    let main_pass_resolution =
        from_json::<BevyMainPassResolutionOverrideParams>(serde_json::json!({
            "width": 1600,
            "height": 900
        }));
    let sub_camera_view = from_json::<BevySubCameraViewParams>(serde_json::json!({
        "full_width": 3840,
        "full_height": 2160,
        "offset_x": 1920.0,
        "offset_y": 1080.0,
        "width": 1920,
        "height": 1080
    }));

    let depth_load_source = normalize(&depth_load.emit_code().to_string());
    let transmission_quality_source = normalize(&transmission_quality.emit_code().to_string());
    let main_pass_resolution_source = normalize(&main_pass_resolution.emit_code().to_string());
    let sub_camera_view_source = normalize(&sub_camera_view.emit_code().to_string());

    assert!(depth_load_source.contains("::bevy::camera::Camera3dDepthLoadOp::Clear(0.25"));
    assert!(
        transmission_quality_source
            .contains("::bevy::camera::ScreenSpaceTransmissionQuality::Ultra")
    );
    assert!(main_pass_resolution_source.contains(
        "::bevy::camera::MainPassResolutionOverride(::bevy::math::UVec2::new(1600u32,900u32))"
    ));
    assert!(sub_camera_view_source.contains("full_size:::bevy::math::UVec2::new(3840u32,2160u32)"));
    assert!(sub_camera_view_source.contains("offset:::bevy::math::Vec2::new(1920"));
    assert!(sub_camera_view_source.contains("size:::bevy::math::UVec2::new(1920u32,1080u32)"));
}

#[test]
fn visibility_helpers_emit_current_bevy_paths() {
    let no_cpu_culling = BevyNoCpuCullingParams;
    let no_frustum_culling = BevyNoFrustumCullingParams;
    let visibility_range = from_json::<BevyVisibilityRangeParams>(serde_json::json!({
        "start_margin_start": 5.0,
        "start_margin_end": 10.0,
        "end_margin_start": 100.0,
        "end_margin_end": 120.0,
        "use_aabb": true
    }));

    let no_cpu_culling_source = normalize(&no_cpu_culling.emit_code().to_string());
    let no_frustum_culling_source = normalize(&no_frustum_culling.emit_code().to_string());
    let visibility_range_source = normalize(&visibility_range.emit_code().to_string());

    assert_eq!(
        no_cpu_culling_source,
        "::bevy::camera::visibility::NoCpuCulling"
    );
    assert_eq!(
        no_frustum_culling_source,
        "::bevy::camera::visibility::NoFrustumCulling"
    );
    assert!(visibility_range_source.contains("::bevy::camera::visibility::VisibilityRange{"));
    assert!(visibility_range_source.contains("start_margin:5"));
    assert!(visibility_range_source.contains("..10"));
    assert!(visibility_range_source.contains("end_margin:100"));
    assert!(visibility_range_source.contains("..120"));
    assert!(visibility_range_source.contains("use_aabb:true"));
}

#[test]
fn material_enum_and_lightmap_helpers_emit_current_bevy_paths() {
    let uv_channel = BevyUvChannelParams {
        variant: BevyUvChannelVariant::Uv1,
    };
    let parallax = BevyParallaxMappingMethodParams::Relief { max_steps: 24 };
    let opaque = BevyOpaqueRendererMethodParams {
        variant: BevyOpaqueRendererMethodVariant::Deferred,
    };
    let default_opaque = BevyDefaultOpaqueRendererMethodParams {
        variant: BevyDefaultOpaqueRendererMethodVariant::Forward,
    };
    let lightmap = BevyLightmapParams {
        image_expr: "asset_server.load(\"lightmaps/hall.ktx2\")".into(),
        uv_rect_expr: Some("Rect::from_corners(Vec2::ZERO, Vec2::ONE)".into()),
        bicubic_sampling: Some(true),
    };
    let alpha_mode_2d = BevyAlphaMode2dParams::Mask(0.4);
    let color_material = BevyColorMaterialParams {
        color_expr: Some("Color::srgb(0.2, 0.6, 0.9)".into()),
        alpha_mode_expr: Some("AlphaMode2d::Mask(0.4)".into()),
        uv_transform_expr: Some("Affine2::IDENTITY".into()),
        texture_expr: Some("asset_server.load(\"sprites/tile.png\")".into()),
    };

    assert_eq!(
        normalize(&uv_channel.emit_code().to_string()),
        "::bevy::pbr::UvChannel::Uv1"
    );
    assert_eq!(
        normalize(&parallax.emit_code().to_string()),
        normalize("::bevy::pbr::ParallaxMappingMethod::Relief { max_steps: 24u32 }")
    );
    assert_eq!(
        normalize(&opaque.emit_code().to_string()),
        "::bevy::pbr::OpaqueRendererMethod::Deferred"
    );
    assert_eq!(
        normalize(&default_opaque.emit_code().to_string()),
        "::bevy::pbr::DefaultOpaqueRendererMethod::forward()"
    );
    assert_eq!(
        normalize(&alpha_mode_2d.emit_code().to_string()),
        "::bevy::sprite_render::AlphaMode2d::Mask(0.4f32)"
    );

    let lightmap_source = normalize(&lightmap.emit_code().to_string());
    assert!(lightmap_source.contains("::bevy::pbr::Lightmap{"));
    assert!(lightmap_source.contains("image:asset_server.load(\"lightmaps/hall.ktx2\"),"));
    assert!(lightmap_source.contains("uv_rect:Rect::from_corners(Vec2::ZERO,Vec2::ONE),"));
    assert!(lightmap_source.contains("bicubic_sampling:true,"));

    let color_material_source = normalize(&color_material.emit_code().to_string());
    assert!(color_material_source.contains("::bevy::sprite_render::ColorMaterial{"));
    assert!(color_material_source.contains("color:Color::srgb(0.2,0.6,0.9),"));
    assert!(color_material_source.contains("alpha_mode:AlphaMode2d::Mask(0.4),"));
    assert!(color_material_source.contains("uv_transform:Affine2::IDENTITY,"));
    assert!(
        color_material_source.contains("texture:Some(asset_server.load(\"sprites/tile.png\")),")
    );
}

#[test]
fn skybox_and_prepass_helpers_emit_current_bevy_paths() {
    let skybox = BevySkyboxParams {
        image_expr: "asset_server.load(\"skyboxes/studio.ktx2\")".into(),
        brightness: Some(1200.0),
        rotation_expr: Some("Quat::from_rotation_y(1.2)".into()),
    };
    let depth = BevyDepthPrepassParams;
    let normal = BevyNormalPrepassParams;
    let motion = BevyMotionVectorPrepassParams;
    let deferred = BevyDeferredPrepassParams;
    let depth_double = BevyDepthPrepassDoubleBufferParams;
    let deferred_double = BevyDeferredPrepassDoubleBufferParams;

    let skybox_source = normalize(&skybox.emit_code().to_string());
    assert!(skybox_source.contains("::bevy::core_pipeline::Skybox{"));
    assert!(skybox_source.contains("image:asset_server.load(\"skyboxes/studio.ktx2\"),"));
    assert!(skybox_source.contains("brightness:1200"));
    assert!(skybox_source.contains("rotation:Quat::from_rotation_y(1.2),"));

    assert_eq!(
        normalize(&depth.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::DepthPrepass"
    );
    assert_eq!(
        normalize(&normal.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::NormalPrepass"
    );
    assert_eq!(
        normalize(&motion.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::MotionVectorPrepass"
    );
    assert_eq!(
        normalize(&deferred.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::DeferredPrepass"
    );
    assert_eq!(
        normalize(&depth_double.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::DepthPrepassDoubleBuffer"
    );
    assert_eq!(
        normalize(&deferred_double.emit_code().to_string()),
        "::bevy::core_pipeline::prepass::DeferredPrepassDoubleBuffer"
    );
}

#[test]
fn camera_3d_params_emit_spawn_tuple_with_render_target_and_projection() {
    let params = BevyCamera3dParams {
        commands_var: "commands".into(),
        transform_expr: Some("Transform::from_xyz(0.0, 4.0, 12.0)".into()),
        fov: Some(1.0),
        near: Some(0.1),
        far: Some(500.0),
        hdr: Some(true),
        render_target: Some(from_json(serde_json::json!({
            "kind": "primary_window"
        }))),
        tonemapping_expr: Some("Tonemapping::AgX".into()),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("commands.spawn(("));
    assert!(source.contains("::bevy::camera::Camera3d::default()"));
    assert!(source.contains("::bevy::camera::Projection::Perspective"));
    assert!(source.contains("::bevy::camera::PerspectiveProjection{"));
    assert!(source.contains("fov:1"));
    assert!(source.contains("near:0.1"));
    assert!(source.contains("far:500"));
    assert!(
        source.contains("::bevy::camera::RenderTarget::Window(::bevy::window::WindowRef::Primary)")
    );
    assert!(source.contains("Tonemapping::AgX"));
    assert!(source.contains("::bevy::camera::Camera{hdr:true"));
}

#[test]
fn mesh_wrapper_helpers_emit_current_bevy_component_paths() {
    let mesh_3d = BevyMesh3dParams {
        mesh_expr: "meshes.add(Cuboid::new(1.0, 2.0, 3.0))".into(),
    };
    let mesh_2d = BevyMesh2dParams {
        mesh_expr: "meshes.add(Circle::new(24.0))".into(),
    };
    let material_3d = BevyMeshMaterial3dParams {
        material_expr: "materials.add(StandardMaterial::default())".into(),
    };
    let material_2d = BevyMeshMaterial2dParams {
        material_expr: "materials.add(ColorMaterial::from_color(RED))".into(),
    };

    assert_eq!(
        normalize(&mesh_3d.emit_code().to_string()),
        normalize("::bevy::mesh::Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 3.0)))")
    );
    assert_eq!(
        normalize(&mesh_2d.emit_code().to_string()),
        normalize("::bevy::mesh::Mesh2d(meshes.add(Circle::new(24.0)))")
    );
    assert_eq!(
        normalize(&material_3d.emit_code().to_string()),
        normalize("::bevy::pbr::MeshMaterial3d(materials.add(StandardMaterial::default()))")
    );
    assert_eq!(
        normalize(&material_2d.emit_code().to_string()),
        normalize(
            "::bevy::sprite_render::MeshMaterial2d(materials.add(ColorMaterial::from_color(RED)))"
        )
    );
}

#[test]
fn dispatch_emit_mesh_material_3d_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "mesh_material_3d",
        "elicit_bevy",
        serde_json::json!({
            "material_expr": "materials.add(StandardMaterial::from_color(Color::WHITE))"
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains(
        "::bevy::pbr::MeshMaterial3d(materials.add(StandardMaterial::from_color(Color::WHITE)))"
    ));
}

#[test]
fn dispatch_emit_lightmap_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "lightmap",
        "elicit_bevy",
        serde_json::json!({
            "image_expr": "asset_server.load(\"baked/room.ktx2\")",
            "bicubic_sampling": true
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::pbr::Lightmap{"));
    assert!(source.contains("image:asset_server.load(\"baked/room.ktx2\"),"));
    assert!(source.contains("bicubic_sampling:true,"));
}

#[test]
fn dispatch_emit_color_material_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "color_material",
        "elicit_bevy",
        serde_json::json!({
            "color_expr": "Color::WHITE",
            "texture_expr": "asset_server.load(\"sprites/player.png\")"
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::sprite_render::ColorMaterial{"));
    assert!(source.contains("color:Color::WHITE,"));
    assert!(source.contains("texture:Some(asset_server.load(\"sprites/player.png\")),"));
}

#[test]
fn wireframe_helpers_emit_current_2d_and_3d_components() {
    let wireframe = BevyWireframeParams;
    let wireframe_color = BevyWireframeColorParams {
        color_expr: "Color::srgb(0.8, 0.2, 0.1)".into(),
    };
    let no_wireframe = BevyNoWireframeParams;
    let wireframe_config = BevyWireframeConfigParams {
        global: Some(true),
        default_color_expr: Some("Color::WHITE".into()),
    };
    let mesh_3d_wireframe = BevyMesh3dWireframeParams {
        material_expr: "wireframe_materials.add(WireframeMaterial::default())".into(),
    };
    let wireframe_2d = BevyWireframe2dParams;
    let wireframe_2d_color = BevyWireframe2dColorParams {
        color_expr: "Color::srgba(0.2, 0.7, 1.0, 0.9)".into(),
    };
    let no_wireframe_2d = BevyNoWireframe2dParams;
    let wireframe_2d_config = BevyWireframe2dConfigParams {
        global: Some(false),
        default_color_expr: Some("Color::BLACK".into()),
    };
    let mesh_2d_wireframe = BevyMesh2dWireframeParams {
        material_expr: "wireframe_materials.add(Wireframe2dMaterial::default())".into(),
    };

    assert_eq!(
        normalize(&wireframe.emit_code().to_string()),
        "::bevy::pbr::Wireframe"
    );
    assert_eq!(
        normalize(&wireframe_color.emit_code().to_string()),
        normalize("::bevy::pbr::WireframeColor { color: Color::srgb(0.8, 0.2, 0.1), }")
    );
    assert_eq!(
        normalize(&no_wireframe.emit_code().to_string()),
        "::bevy::pbr::NoWireframe"
    );
    assert!(
        normalize(&wireframe_config.emit_code().to_string())
            .contains("::bevy::pbr::WireframeConfig{")
    );
    assert!(normalize(&wireframe_config.emit_code().to_string()).contains("global:true,"));
    assert!(
        normalize(&wireframe_config.emit_code().to_string())
            .contains("default_color:Color::WHITE,")
    );
    assert_eq!(
        normalize(&mesh_3d_wireframe.emit_code().to_string()),
        normalize(
            "::bevy::pbr::Mesh3dWireframe(wireframe_materials.add(WireframeMaterial::default()))"
        )
    );
    assert_eq!(
        normalize(&wireframe_2d.emit_code().to_string()),
        "::bevy::sprite_render::Wireframe2d"
    );
    assert_eq!(
        normalize(&wireframe_2d_color.emit_code().to_string()),
        normalize(
            "::bevy::sprite_render::Wireframe2dColor { color: Color::srgba(0.2, 0.7, 1.0, 0.9), }"
        )
    );
    assert_eq!(
        normalize(&no_wireframe_2d.emit_code().to_string()),
        "::bevy::sprite_render::NoWireframe2d"
    );
    assert!(
        normalize(&wireframe_2d_config.emit_code().to_string())
            .contains("::bevy::sprite_render::Wireframe2dConfig{")
    );
    assert!(
        normalize(&wireframe_2d_config.emit_code().to_string())
            .contains("default_color:Color::BLACK,")
    );
    assert_eq!(
        normalize(&mesh_2d_wireframe.emit_code().to_string()),
        normalize(
            "::bevy::sprite_render::Mesh2dWireframe(wireframe_materials.add(Wireframe2dMaterial::default()))"
        )
    );
}

#[test]
fn dispatch_emit_wireframe_config_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "wireframe_config",
        "elicit_bevy",
        serde_json::json!({
            "global": true,
            "default_color_expr": "Color::srgb(0.4, 0.9, 0.4)"
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::pbr::WireframeConfig{"));
    assert!(source.contains("global:true,"));
    assert!(source.contains("default_color:Color::srgb(0.4,0.9,0.4),"));
}

#[test]
fn dispatch_emit_order_independent_transparency_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "order_independent_transparency_settings",
        "elicit_bevy",
        serde_json::json!({
            "layer_count": 12,
            "alpha_threshold": 0.05
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::core_pipeline::oit::OrderIndependentTransparencySettings{"));
    assert!(source.contains("layer_count:12i32,"));
    assert!(source.contains("alpha_threshold:0.05f32,"));
    assert!(source.contains("..::std::default::Default::default()"));
}

#[test]
fn dispatch_emit_scaling_mode_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "scaling_mode",
        "elicit_bevy",
        serde_json::json!({
            "variant": "fixed_horizontal",
            "viewport_width": 640.0
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::camera::ScalingMode::FixedHorizontal{"));
    assert!(source.contains("viewport_width:640"));
}

#[test]
fn dispatch_emit_clear_color_config_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "clear_color_config",
        "elicit_bevy",
        serde_json::json!({
            "variant": "custom",
            "color_expr": "Color::BLACK"
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::camera::ClearColorConfig::Custom(Color::BLACK)"));
}

#[test]
fn dispatch_emit_sub_camera_view_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "sub_camera_view",
        "elicit_bevy",
        serde_json::json!({
            "full_width": 32,
            "full_height": 18,
            "offset_x": 16.0,
            "offset_y": 9.0,
            "width": 16,
            "height": 9
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("full_size:::bevy::math::UVec2::new(32u32,18u32)"));
    assert!(source.contains("offset:::bevy::math::Vec2::new(16"));
    assert!(source.contains("size:::bevy::math::UVec2::new(16u32,9u32)"));
}

#[test]
fn dispatch_emit_visibility_range_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "visibility_range",
        "elicit_bevy",
        serde_json::json!({
            "start_margin_start": 1.0,
            "start_margin_end": 2.0,
            "end_margin_start": 50.0,
            "end_margin_end": 60.0,
            "use_aabb": false
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("start_margin:1"));
    assert!(source.contains("..2"));
    assert!(source.contains("end_margin:50"));
    assert!(source.contains("..60"));
    assert!(source.contains("use_aabb:false"));
}

#[test]
fn dispatch_emit_camera_2d_uses_registered_emit_entry() {
    let emitter = dispatch_emit_from(
        "camera_2d",
        "elicit_bevy",
        serde_json::json!({
            "commands_var": "commands",
            "scale": 2.0,
            "render_target": {
                "kind": "window_entity",
                "target_expr": "hud_window"
            }
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("::bevy::camera::Camera2d::default()"));
    assert!(source.contains("::bevy::camera::OrthographicProjection{"));
    assert!(source.contains("scale:2"));
    assert!(source.contains("WindowRef::Entity(hud_window)"));
}

#[test]
fn viewport_and_fog_params_emit_current_bevy_types() {
    let viewport = BevyViewportParams {
        physical_position: [32, 64],
        physical_size: [512, 288],
        depth: Some([0.2, 0.9]),
    };
    let fog = BevyFogSettingsParams {
        color_expr: Some("Color::srgba(0.5, 0.6, 0.8, 0.7)".into()),
        directional_light_color_expr: Some("Color::srgb(1.0, 0.9, 0.7)".into()),
        directional_light_exponent: Some(12.0),
        falloff_expr: Some("FogFalloff::from_visibility(80.0)".into()),
    };

    let viewport_source = normalize(&viewport.emit_code().to_string());
    let fog_source = normalize(&fog.emit_code().to_string());

    assert!(viewport_source.contains("::bevy::camera::Viewport{"));
    assert!(viewport_source.contains("UVec2::new(32u32,64u32)"));
    assert!(viewport_source.contains("UVec2::new(512u32,288u32)"));
    assert!(viewport_source.contains("0.2"));
    assert!(viewport_source.contains("0.9"));
    assert!(fog_source.contains("::bevy::pbr::DistanceFog{"));
    assert!(fog_source.contains("Color::srgba(0.5,0.6,0.8,0.7)"));
    assert!(fog_source.contains("directional_light_exponent:12"));
    assert!(fog_source.contains("FogFalloff::from_visibility(80.0)"));
}

#[test]
fn bloom_sprite_and_text_style_emit_helper_fragments() {
    let bloom = BevyBloomSettingsParams {
        preset: Some(BevyBloomPreset::Anamorphic),
        intensity: Some(0.25),
        low_frequency_boost: None,
        low_frequency_boost_curvature: None,
        high_pass_frequency: Some(0.8),
        prefilter_threshold: Some(0.6),
        prefilter_threshold_softness: Some(0.2),
        composite_mode: Some(BevyBloomCompositeModeVariant::Additive),
        max_mip_dimension: None,
        scale_expr: Some("Vec2::new(2.0, 1.0)".into()),
    };
    let sprite = BevySpriteParams {
        image_expr: Some("asset_server.load(\"branding/icon.png\")".into()),
        color_expr: Some("Color::WHITE".into()),
        flip_x: Some(true),
        flip_y: None,
        custom_size_expr: Some("Vec2::new(96.0, 96.0)".into()),
    };
    let text_style: BevyTextStyleParams = from_json(serde_json::json!({
        "font_handle_expr": "asset_server.load(\"fonts/FiraSans-Bold.ttf\")",
        "font_size": 32.0,
        "color_expr": "Color::WHITE",
        "justify_expr": "Justify::Center",
        "linebreak_expr": "LineBreak::NoWrap"
    }));

    let bloom_source = normalize(&bloom.emit_code().to_string());
    let sprite_source = normalize(&sprite.emit_code().to_string());
    let text_style_source = normalize(&text_style.emit_code().to_string());

    assert!(bloom_source.contains("::bevy::post_process::bloom::Bloom{"));
    assert!(bloom_source.contains("intensity:0.25"));
    assert!(bloom_source.contains("high_pass_frequency:0.8"));
    assert!(bloom_source.contains("prefilter:::bevy::post_process::bloom::BloomPrefilter{"));
    assert!(bloom_source.contains("threshold:0.6"));
    assert!(bloom_source.contains("threshold_softness:0.2"));
    assert!(bloom_source.contains("BloomCompositeMode::Additive"));
    assert!(bloom_source.contains("scale:Vec2::new(2.0,1.0)"));
    assert!(bloom_source.contains("..::bevy::post_process::bloom::Bloom::ANAMORPHIC"));
    assert!(sprite_source.contains("::bevy::sprite::Sprite{"));
    assert!(sprite_source.contains("image:asset_server.load(\"branding/icon.png\")"));
    assert!(sprite_source.contains("color:(Color::WHITE).into()"));
    assert!(sprite_source.contains("flip_x:true"));
    assert!(sprite_source.contains("custom_size:Some(Vec2::new(96.0,96.0))"));
    assert!(text_style_source.contains("::bevy::text::TextFont{"));
    assert!(text_style_source.contains("font:asset_server.load(\"fonts/FiraSans-Bold.ttf\"),"));
    assert!(text_style_source.contains("font_size:32"));
    assert!(text_style_source.contains("::bevy::text::TextColor((Color::WHITE).into())"));
    assert!(text_style_source.contains("::bevy::text::TextLayout{"));
    assert!(text_style_source.contains("justify:Justify::Center,"));
    assert!(text_style_source.contains("linebreak:LineBreak::NoWrap,"));
}

#[test]
fn color_and_bloom_composite_mode_emit_current_bevy_paths() {
    let color = BevyColorParams {
        color: serde_json::from_value(serde_json::json!({
            "space": "Srgba",
            "red": 0.9,
            "green": 0.7,
            "blue": 0.6,
            "alpha": 1.0
        }))
        .unwrap(),
    };
    let composite = BevyBloomCompositeModeParams {
        variant: BevyBloomCompositeModeVariant::EnergyConserving,
    };

    let color_source = normalize(&color.emit_code().to_string());
    let composite_source = normalize(&composite.emit_code().to_string());

    assert!(color_source.contains("bevy::color::Color::Srgba"));
    assert!(color_source.contains("red:0.9"));
    assert!(
        composite_source
            .contains("::bevy::post_process::bloom::BloomCompositeMode::EnergyConserving")
    );
}

#[test]
fn ssao_and_temporal_anti_aliasing_emit_current_components() {
    let quality = BevySsaoQualityParams::Custom {
        slice_count: 6,
        samples_per_slice_side: 3,
    };
    let ssao = BevySsaoParams {
        quality_level: Some(quality.clone()),
        constant_object_thickness: Some(0.5),
    };
    let taa = BevyTemporalAntiAliasingParams { reset: Some(false) };

    let quality_source = normalize(&quality.emit_code().to_string());
    let ssao_source = normalize(&ssao.emit_code().to_string());
    let taa_source = normalize(&taa.emit_code().to_string());

    assert!(quality_source.contains("ScreenSpaceAmbientOcclusionQualityLevel::Custom"));
    assert!(quality_source.contains("slice_count:6u32"));
    assert!(quality_source.contains("samples_per_slice_side:3u32"));
    assert!(ssao_source.contains("::bevy::pbr::ScreenSpaceAmbientOcclusion{"));
    assert!(ssao_source.contains("constant_object_thickness:0.5"));
    assert!(ssao_source.contains("ScreenSpaceAmbientOcclusionQualityLevel::Custom"));
    assert!(taa_source.contains("::bevy::anti_alias::taa::TemporalAntiAliasing{"));
    assert!(taa_source.contains("reset:false"));
}

#[test]
fn cascade_shadow_helpers_emit_current_bevy_light_types() {
    let config: BevyCascadeShadowConfigParams = from_json(serde_json::json!({
        "bounds": [6.0, 18.0, 48.0],
        "overlap_proportion": 0.15,
        "minimum_distance": 0.4
    }));
    let builder = BevyCascadeShadowConfigBuilderParams {
        num_cascades: Some(3),
        minimum_distance: Some(0.2),
        maximum_distance: Some(80.0),
        first_cascade_far_bound: Some(12.0),
        overlap_proportion: Some(0.1),
    };

    let config_source = normalize(&config.emit_code().to_string());
    let builder_source = normalize(&builder.emit_code().to_string());

    assert!(config_source.contains("::bevy::light::CascadeShadowConfig{"));
    assert!(config_source.contains("bounds:vec![6f32,18f32,48f32],"));
    assert!(config_source.contains("overlap_proportion:0.15f32,"));
    assert!(config_source.contains("minimum_distance:0.4f32,"));
    assert!(config_source.contains("..::std::default::Default::default()"));
    assert!(builder_source.contains("::bevy::light::CascadeShadowConfigBuilder{"));
    assert!(builder_source.contains("num_cascades:3usize"));
    assert!(builder_source.contains("minimum_distance:0.2f32"));
    assert!(builder_source.contains("maximum_distance:80f32"));
    assert!(builder_source.contains("first_cascade_far_bound:12f32"));
    assert!(builder_source.contains("overlap_proportion:0.1f32"));
    assert!(builder_source.contains("..::std::default::Default::default()"));
}

#[test]
fn shadow_map_and_environment_map_helpers_emit_current_light_resources() {
    let directional_shadow_map = BevyDirectionalLightShadowMapParams { size: Some(4096) };
    let point_shadow_map = BevyPointLightShadowMapParams { size: Some(2048) };
    let environment_map = BevyEnvironmentMapLightParams {
        diffuse_map_expr: "asset_server.load(\"environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2\")"
            .into(),
        specular_map_expr: "asset_server.load(\"environment_maps/pisa_specular_rgb9e5_zstd.ktx2\")"
            .into(),
        intensity: Some(650.0),
        rotation_expr: Some("Quat::from_rotation_y(0.25)".into()),
        affects_lightmapped_mesh_diffuse: Some(false),
    };

    let directional_source = normalize(&directional_shadow_map.emit_code().to_string());
    let point_source = normalize(&point_shadow_map.emit_code().to_string());
    let environment_source = normalize(&environment_map.emit_code().to_string());

    assert!(
        directional_source.contains("::bevy::light::DirectionalLightShadowMap{size:4096usize,}")
    );
    assert!(point_source.contains("::bevy::light::PointLightShadowMap{size:2048usize,}"));
    assert!(environment_source.contains("::bevy::light::EnvironmentMapLight{"));
    assert!(environment_source.contains(
        "diffuse_map:asset_server.load(\"environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2\")"
    ));
    assert!(environment_source.contains(
        "specular_map:asset_server.load(\"environment_maps/pisa_specular_rgb9e5_zstd.ktx2\")"
    ));
    assert!(environment_source.contains("intensity:650"));
    assert!(environment_source.contains("rotation:Quat::from_rotation_y(0.25)"));
    assert!(environment_source.contains("affects_lightmapped_mesh_diffuse:false"));
}

#[test]
fn generated_probe_and_volumetric_helpers_emit_current_bevy_light_components() {
    let generated_environment_map = BevyGeneratedEnvironmentMapLightParams {
        environment_map_expr: "asset_server.load(\"environment_maps/pisa_rgb9e5_zstd.ktx2\")"
            .into(),
        intensity: Some(320.0),
        rotation_expr: Some("Quat::from_rotation_z(0.5)".into()),
        affects_lightmapped_mesh_diffuse: Some(false),
    };
    let atmosphere_environment_map = BevyAtmosphereEnvironmentMapLightParams {
        intensity: Some(1.5),
        affects_lightmapped_mesh_diffuse: Some(false),
        size: Some([1024, 1024]),
    };
    let volumetric_light = BevyVolumetricLightParams;
    let volumetric_fog = BevyVolumetricFogParams {
        ambient_color_expr: Some("Color::srgb(0.7, 0.8, 1.0)".into()),
        ambient_intensity: Some(0.2),
        jitter: Some(0.05),
        step_count: Some(96),
    };
    let fog_volume = BevyFogVolumeParams {
        fog_color_expr: Some("Color::WHITE".into()),
        density_factor: Some(0.25),
        density_texture_expr: Some("asset_server.load(\"textures/fog_density.ktx2\")".into()),
        density_texture_offset_expr: Some("Vec3::new(0.0, 1.0, 2.0)".into()),
        absorption: Some(0.4),
        scattering: Some(0.6),
        scattering_asymmetry: Some(0.3),
        light_tint_expr: Some("Color::srgb(1.0, 0.9, 0.8)".into()),
        light_intensity: Some(1.25),
    };

    let generated_source = normalize(&generated_environment_map.emit_code().to_string());
    let atmosphere_source = normalize(&atmosphere_environment_map.emit_code().to_string());
    let volumetric_light_source = normalize(&volumetric_light.emit_code().to_string());
    let volumetric_fog_source = normalize(&volumetric_fog.emit_code().to_string());
    let fog_volume_source = normalize(&fog_volume.emit_code().to_string());

    assert!(generated_source.contains("::bevy::light::GeneratedEnvironmentMapLight{"));
    assert!(
        generated_source.contains(
            "environment_map:asset_server.load(\"environment_maps/pisa_rgb9e5_zstd.ktx2\")"
        )
    );
    assert!(generated_source.contains("intensity:320"));
    assert!(generated_source.contains("rotation:Quat::from_rotation_z(0.5)"));
    assert!(generated_source.contains("affects_lightmapped_mesh_diffuse:false"));
    assert!(atmosphere_source.contains("::bevy::light::AtmosphereEnvironmentMapLight{"));
    assert!(atmosphere_source.contains("intensity:1.5f32,"));
    assert!(atmosphere_source.contains("affects_lightmapped_mesh_diffuse:false,"));
    assert!(atmosphere_source.contains("size:::bevy::math::UVec2::new(1024u32,1024u32),"));
    assert!(atmosphere_source.contains("..::std::default::Default::default()"));
    assert_eq!(volumetric_light_source, "::bevy::light::VolumetricLight");
    assert!(volumetric_fog_source.contains("::bevy::light::VolumetricFog{"));
    assert!(volumetric_fog_source.contains("ambient_color:Color::srgb(0.7,0.8,1.0)"));
    assert!(volumetric_fog_source.contains("ambient_intensity:0.2"));
    assert!(volumetric_fog_source.contains("jitter:0.05"));
    assert!(volumetric_fog_source.contains("step_count:96u32"));
    assert!(fog_volume_source.contains("::bevy::light::FogVolume{"));
    assert!(
        fog_volume_source
            .contains("density_texture:Some(asset_server.load(\"textures/fog_density.ktx2\"))")
    );
    assert!(fog_volume_source.contains("density_texture_offset:Vec3::new(0.0,1.0,2.0)"));
    assert!(fog_volume_source.contains("absorption:0.4"));
    assert!(fog_volume_source.contains("scattering:0.6"));
    assert!(fog_volume_source.contains("scattering_asymmetry:0.3"));
    assert!(fog_volume_source.contains("light_tint:Color::srgb(1.0,0.9,0.8)"));
    assert!(fog_volume_source.contains("light_intensity:1.25"));
}

#[test]
fn light_probe_and_irradiance_volume_emit_current_probe_components() {
    let light_probe = BevyLightProbeParams;
    let irradiance_volume = BevyIrradianceVolumeParams {
        voxels_expr: "asset_server.load(\"irradiance/room.ktx2\")".into(),
        intensity: Some(180.0),
        affects_lightmapped_meshes: Some(false),
    };

    let light_probe_source = normalize(&light_probe.emit_code().to_string());
    let irradiance_source = normalize(&irradiance_volume.emit_code().to_string());

    assert_eq!(light_probe_source, "::bevy::light::LightProbe");
    assert!(irradiance_source.contains("::bevy::light::IrradianceVolume{"));
    assert!(irradiance_source.contains("voxels:asset_server.load(\"irradiance/room.ktx2\")"));
    assert!(irradiance_source.contains("intensity:180"));
    assert!(irradiance_source.contains("affects_lightmapped_meshes:false"));
}

#[test]
fn light_marker_and_small_light_helpers_emit_current_bevy_components() {
    let ambient = BevyAmbientLightParams {
        color_expr: Some("Color::srgb(0.8, 0.7, 0.6)".into()),
        brightness: Some(120.0),
        affects_lightmapped_meshes: Some(false),
    };
    let global_ambient = BevyGlobalAmbientLightParams {
        color_expr: Some("Color::srgb(0.1, 0.2, 0.3)".into()),
        brightness: Some(32.0),
        affects_lightmapped_meshes: Some(true),
    };
    let sun_disk = BevySunDiskParams {
        angular_size: Some(0.02),
        intensity: Some(1.5),
    };
    let not_shadow_caster = BevyNotShadowCasterParams;
    let not_shadow_receiver = BevyNotShadowReceiverParams;
    let transmitted_shadow_receiver = BevyTransmittedShadowReceiverParams;

    let ambient_source = normalize(&ambient.emit_code().to_string());
    let global_source = normalize(&global_ambient.emit_code().to_string());
    let sun_disk_source = normalize(&sun_disk.emit_code().to_string());
    let not_shadow_caster_source = normalize(&not_shadow_caster.emit_code().to_string());
    let not_shadow_receiver_source = normalize(&not_shadow_receiver.emit_code().to_string());
    let transmitted_shadow_receiver_source =
        normalize(&transmitted_shadow_receiver.emit_code().to_string());

    assert!(ambient_source.contains("::bevy::light::AmbientLight{"));
    assert!(ambient_source.contains("brightness:120"));
    assert!(ambient_source.contains("affects_lightmapped_meshes:false"));
    assert!(global_source.contains("::bevy::light::GlobalAmbientLight{"));
    assert!(global_source.contains("brightness:32"));
    assert!(global_source.contains("affects_lightmapped_meshes:true"));
    assert!(sun_disk_source.contains("::bevy::light::SunDisk{"));
    assert!(sun_disk_source.contains("angular_size:0.02"));
    assert!(sun_disk_source.contains("intensity:1.5"));
    assert_eq!(not_shadow_caster_source, "::bevy::light::NotShadowCaster");
    assert_eq!(
        not_shadow_receiver_source,
        "::bevy::light::NotShadowReceiver"
    );
    assert_eq!(
        transmitted_shadow_receiver_source,
        "::bevy::light::TransmittedShadowReceiver"
    );
}

#[test]
fn render_config_helpers_emit_current_cluster_and_ssr_components() {
    let shadow_filtering = BevyShadowFilteringMethodParams {
        variant: BevyShadowFilteringMethodVariant::Temporal,
    };
    let cluster_far_z_mode = BevyClusterFarZModeParams::Constant(250.0);
    let cluster_z_config = BevyClusterZConfigParams {
        first_slice_depth: Some(8.0),
        far_z_mode: Some(cluster_far_z_mode),
    };
    let cluster_config = BevyClusterConfigParams::FixedZ {
        total: 2048,
        z_slices: 16,
        z_config: cluster_z_config.clone(),
        dynamic_resizing: false,
    };
    let ssr = BevyScreenSpaceReflectionsParams {
        perceptual_roughness_threshold: Some(0.2),
        thickness: Some(0.35),
        linear_steps: Some(24),
        linear_march_exponent: Some(2.0),
        bisection_steps: Some(5),
        use_secant: Some(false),
    };

    let shadow_filtering_source = normalize(&shadow_filtering.emit_code().to_string());
    let cluster_far_z_source = normalize(&cluster_far_z_mode.emit_code().to_string());
    let cluster_z_source = normalize(&cluster_z_config.emit_code().to_string());
    let cluster_config_source = normalize(&cluster_config.emit_code().to_string());
    let ssr_source = normalize(&ssr.emit_code().to_string());

    assert_eq!(
        shadow_filtering_source,
        "::bevy::light::ShadowFilteringMethod::Temporal"
    );
    assert_eq!(
        cluster_far_z_source,
        "::bevy::light::ClusterFarZMode::Constant(250f32)"
    );
    assert!(cluster_z_source.contains("::bevy::light::ClusterZConfig{"));
    assert!(cluster_z_source.contains("first_slice_depth:8"));
    assert!(cluster_z_source.contains("ClusterFarZMode::Constant(250f32)"));
    assert!(cluster_config_source.contains("::bevy::light::ClusterConfig::FixedZ{"));
    assert!(cluster_config_source.contains("total:2048u32"));
    assert!(cluster_config_source.contains("z_slices:16u32"));
    assert!(cluster_config_source.contains("dynamic_resizing:false"));
    assert!(ssr_source.contains("::bevy::pbr::ScreenSpaceReflections{"));
    assert!(ssr_source.contains("perceptual_roughness_threshold:0.2"));
    assert!(ssr_source.contains("thickness:0.35"));
    assert!(ssr_source.contains("linear_steps:24u32"));
    assert!(ssr_source.contains("linear_march_exponent:2"));
    assert!(ssr_source.contains("bisection_steps:5u32"));
    assert!(ssr_source.contains("use_secant:false"));
}

#[test]
fn clustered_decal_helper_emits_current_decal_component() {
    let params = BevyClusteredDecalParams {
        base_color_texture_expr: Some("asset_server.load(\"decals/base_color.png\")".into()),
        normal_map_texture_expr: Some("asset_server.load(\"decals/normal.png\")".into()),
        metallic_roughness_texture_expr: Some(
            "asset_server.load(\"decals/metallic_roughness.png\")".into(),
        ),
        emissive_texture_expr: Some("asset_server.load(\"decals/emissive.png\")".into()),
        tag: Some(7),
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains("::bevy::light::ClusteredDecal{"));
    assert!(
        source.contains("base_color_texture:Some(asset_server.load(\"decals/base_color.png\"))")
    );
    assert!(source.contains("normal_map_texture:Some(asset_server.load(\"decals/normal.png\"))"));
    assert!(source.contains(
        "metallic_roughness_texture:Some(asset_server.load(\"decals/metallic_roughness.png\"))"
    ));
    assert!(source.contains("emissive_texture:Some(asset_server.load(\"decals/emissive.png\"))"));
    assert!(source.contains("tag:7u32"));
}

#[test]
fn atmosphere_tool_emits_medium_asset_and_component_block() {
    let emitter = dispatch_emit_from(
        "atmosphere",
        "elicit_bevy",
        serde_json::json!({
            "scattering_media_var": "scattering_media",
            "medium_label": "dusty_sky",
            "falloff_resolution": 128,
            "phase_resolution": 64,
            "density_multiplier": 0.5,
            "terms": [
                {
                    "absorption": { "x": 0.0, "y": 0.0, "z": 0.0 },
                    "scattering": { "x": 0.001, "y": 0.002, "z": 0.003 },
                    "falloff": { "variant": "exponential", "scale": 0.3 },
                    "phase": { "variant": "mie", "asymmetry": 0.7 }
                }
            ],
            "atmosphere": {
                "bottom_radius": 100.0,
                "top_radius": 120.0,
                "ground_albedo": { "x": 0.2, "y": 0.25, "z": 0.3 }
            }
        }),
    )
    .unwrap();
    let source = normalize(&emitter.emit_code().to_string());

    assert!(source.contains("(scattering_media).add("));
    assert!(source.contains("::bevy::pbr::ScatteringMedium::new(128"));
    assert!(source.contains("with_label(\"dusty_sky\")"));
    assert!(source.contains("with_density_multiplier(0.5"));
    assert!(source.contains("PhaseFunction::Mie{asymmetry:0.7"));
    assert!(source.contains("bottom_radius:100"));
    assert!(source.contains("ground_albedo:"));
    assert!(source.contains("Vec3::new(0.2"));
}

#[test]
fn fullscreen_material_params_emit_trait_impl_with_default_3d_edges() {
    let params = BevyFullscreenMaterialParams {
        material_type: "FullscreenEffect".into(),
        shader_path: "shaders/fullscreen_effect.wgsl".into(),
        graph: BevyFullscreenGraphKind::Core3d,
        start_node_expr: None,
        end_node_expr: None,
    };
    let source = normalize(&params.emit_code().to_string());

    assert!(source.contains(
        "impl::bevy::core_pipeline::fullscreen_material::FullscreenMaterialforFullscreenEffect{"
    ));
    assert!(source.contains("\"shaders/fullscreen_effect.wgsl\".into()"));
    assert!(source.contains("Node3d::Tonemapping"));
    assert!(source.contains("Self::node_label().intern()"));
    assert!(source.contains("Node3d::EndMainPassPostProcessing"));
    assert!(source.contains("use::bevy::render::render_graph::RenderLabelas_;"));
}
