//! Leaf validator trait impls for `BevyGisBackend`.
//!
//! Each trait validates one renderer-agnostic descriptor and mints the
//! corresponding `Established<*Valid>` proof token.  The proof is asserted
//! here; the validation logic IS the audit trail.

use elicit_gis::{
    GisError, GisErrorKind, GisRenderAmbientLightFactory, GisRenderAtmosphereFactory,
    GisRenderAutoExposureFactory, GisRenderBloomFactory, GisRenderCascadeShadowFactory,
    GisRenderChromaticAberrationFactory, GisRenderClearPolicyFactory, GisRenderColorGradingFactory,
    GisRenderDepthOfFieldFactory, GisRenderDirectionalLightFactory, GisRenderExposureFactory,
    GisRenderFeatureStyleFactory, GisRenderFogFactory, GisRenderFogVolumeFactory,
    GisRenderImageBasedLightingFactory, GisRenderLightProbeFactory, GisRenderMaterialIntentFactory,
    GisRenderMotionBlurFactory, GisRenderOutputTargetFactory, GisRenderRasterStyleFactory,
    GisRenderResolutionOverrideFactory, GisRenderScreenSpaceAmbientOcclusionFactory,
    GisRenderScreenSpaceReflectionsFactory, GisRenderShadowParticipationFactory,
    GisRenderSkyboxFactory, GisRenderSubViewLayoutFactory, GisRenderTerrainStyleFactory,
    GisRenderTileStyleFactory, GisRenderViewOutputFactory, GisRenderViewParticipationFactory,
    GisRenderViewportFactory, GisRenderVolumetricFogFactory, GisRenderWorldSpaceFactory, GisResult,
    RenderAmbientLightDescriptor, RenderAmbientLightValid, RenderAtmosphereDescriptor,
    RenderAtmosphereValid, RenderAutoExposureDescriptor, RenderAutoExposureValid,
    RenderBloomDescriptor, RenderBloomValid, RenderCascadeShadowConfigDescriptor,
    RenderCascadeShadowConfigValid, RenderChromaticAberrationDescriptor,
    RenderChromaticAberrationValid, RenderClearPolicyDescriptor, RenderClearPolicyValid,
    RenderColorGradingDescriptor, RenderColorGradingValid, RenderDepthOfFieldDescriptor,
    RenderDepthOfFieldValid, RenderDirectionalLightDescriptor, RenderDirectionalLightValid,
    RenderExposureDescriptor, RenderExposureValid, RenderFeatureStyleDescriptor,
    RenderFeatureStyleValid, RenderFogDescriptor, RenderFogValid, RenderFogVolumeDescriptor,
    RenderFogVolumeValid, RenderImageBasedLightingDescriptor, RenderImageBasedLightingValid,
    RenderLightProbeDescriptor, RenderLightProbeValid, RenderMaterialIntentDescriptor,
    RenderMaterialIntentValid, RenderMotionBlurDescriptor, RenderMotionBlurValid,
    RenderOutputTargetDescriptor, RenderOutputTargetValid, RenderRasterStyleDescriptor,
    RenderRasterStyleValid, RenderResolutionOverrideDescriptor, RenderResolutionOverrideValid,
    RenderScreenSpaceAmbientOcclusionDescriptor, RenderScreenSpaceAmbientOcclusionValid,
    RenderScreenSpaceReflectionsDescriptor, RenderScreenSpaceReflectionsValid,
    RenderShadowParticipationDescriptor, RenderShadowParticipationValid, RenderSkyboxDescriptor,
    RenderSkyboxValid, RenderSubViewLayoutDescriptor, RenderSubViewLayoutValid,
    RenderTerrainStyleDescriptor, RenderTerrainStyleValid, RenderTileStyleDescriptor,
    RenderTileStyleValid, RenderViewOutputDescriptor, RenderViewOutputValid,
    RenderViewParticipationDescriptor, RenderViewParticipationValid, RenderViewportDescriptor,
    RenderViewportValid, RenderVolumetricFogDescriptor, RenderVolumetricFogValid,
    RenderWorldSpaceDescriptor, RenderWorldSpaceValid,
};
use elicitation::Established;
use tracing::instrument;

use super::BevyGisBackend;

#[inline]
fn invalid(msg: impl Into<String>) -> GisError {
    GisError::new(GisErrorKind::InvalidDescriptor(msg.into()))
}

#[inline]
fn check_finite_f32(value: f32, name: &str) -> GisResult<()> {
    if !value.is_finite() {
        Err(invalid(format!("{name} must be finite")))
    } else {
        Ok(())
    }
}

#[inline]
fn check_positive_f32(value: f32, name: &str) -> GisResult<()> {
    if !value.is_finite() || value <= 0.0 {
        Err(invalid(format!("{name} must be finite and positive")))
    } else {
        Ok(())
    }
}

#[inline]
fn check_non_negative_f32(value: f32, name: &str) -> GisResult<()> {
    if !value.is_finite() || value < 0.0 {
        Err(invalid(format!("{name} must be finite and non-negative")))
    } else {
        Ok(())
    }
}

#[inline]
fn check_positive_u32(value: u32, name: &str) -> GisResult<()> {
    if value == 0 {
        Err(invalid(format!("{name} must be positive (> 0)")))
    } else {
        Ok(())
    }
}

// ── GisRenderFeatureStyleFactory ──────────────────────────────────────────────

impl GisRenderFeatureStyleFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_feature_style(
        &self,
        input: RenderFeatureStyleDescriptor,
    ) -> GisResult<Established<RenderFeatureStyleValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderRasterStyleFactory ───────────────────────────────────────────────

impl GisRenderRasterStyleFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_raster_style(
        &self,
        input: RenderRasterStyleDescriptor,
    ) -> GisResult<Established<RenderRasterStyleValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderTileStyleFactory ─────────────────────────────────────────────────

impl GisRenderTileStyleFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_tile_style(
        &self,
        input: RenderTileStyleDescriptor,
    ) -> GisResult<Established<RenderTileStyleValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderTerrainStyleFactory ──────────────────────────────────────────────

impl GisRenderTerrainStyleFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_terrain_style(
        &self,
        input: RenderTerrainStyleDescriptor,
    ) -> GisResult<Established<RenderTerrainStyleValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderMaterialIntentFactory ────────────────────────────────────────────

impl GisRenderMaterialIntentFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_material_intent(
        &self,
        input: RenderMaterialIntentDescriptor,
    ) -> GisResult<Established<RenderMaterialIntentValid>> {
        if let Some(bias) = input.depth_bias {
            check_finite_f32(bias, "material depth_bias")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderShadowParticipationFactory ───────────────────────────────────────

impl GisRenderShadowParticipationFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_shadow_participation(
        &self,
        input: RenderShadowParticipationDescriptor,
    ) -> GisResult<Established<RenderShadowParticipationValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderOutputTargetFactory ──────────────────────────────────────────────

impl GisRenderOutputTargetFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_output_target(
        &self,
        input: RenderOutputTargetDescriptor,
    ) -> GisResult<Established<RenderOutputTargetValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderClearPolicyFactory ───────────────────────────────────────────────

impl GisRenderClearPolicyFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_clear_policy(
        &self,
        input: RenderClearPolicyDescriptor,
    ) -> GisResult<Established<RenderClearPolicyValid>> {
        if let Some(depth) = input.depth {
            check_finite_f32(depth, "clear policy depth")?;
            if !(0.0..=1.0).contains(&depth) {
                return Err(invalid("clear policy depth must be in [0.0, 1.0]"));
            }
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderViewportFactory ──────────────────────────────────────────────────

impl GisRenderViewportFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_viewport(
        &self,
        input: RenderViewportDescriptor,
    ) -> GisResult<Established<RenderViewportValid>> {
        check_positive_u32(input.physical_size[0], "viewport width")?;
        check_positive_u32(input.physical_size[1], "viewport height")?;
        if let Some([near, far]) = input.depth {
            check_finite_f32(near, "viewport depth near")?;
            check_finite_f32(far, "viewport depth far")?;
            if near >= far {
                return Err(invalid("viewport depth near must be less than far"));
            }
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderResolutionOverrideFactory ────────────────────────────────────────

impl GisRenderResolutionOverrideFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_resolution_override(
        &self,
        input: RenderResolutionOverrideDescriptor,
    ) -> GisResult<Established<RenderResolutionOverrideValid>> {
        check_positive_u32(input.width, "resolution override width")?;
        check_positive_u32(input.height, "resolution override height")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderSubViewLayoutFactory ─────────────────────────────────────────────

impl GisRenderSubViewLayoutFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_subview_layout(
        &self,
        input: RenderSubViewLayoutDescriptor,
    ) -> GisResult<Established<RenderSubViewLayoutValid>> {
        check_positive_u32(input.full_width, "subview full_width")?;
        check_positive_u32(input.full_height, "subview full_height")?;
        check_positive_u32(input.width, "subview width")?;
        check_positive_u32(input.height, "subview height")?;
        check_finite_f32(input.offset_x, "subview offset_x")?;
        check_finite_f32(input.offset_y, "subview offset_y")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderViewParticipationFactory ─────────────────────────────────────────

impl GisRenderViewParticipationFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_view_participation(
        &self,
        input: RenderViewParticipationDescriptor,
    ) -> GisResult<Established<RenderViewParticipationValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderViewOutputFactory ────────────────────────────────────────────────

impl GisRenderViewOutputFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_view_output(
        &self,
        input: RenderViewOutputDescriptor,
    ) -> GisResult<Established<RenderViewOutputValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderExposureFactory ──────────────────────────────────────────────────

impl GisRenderExposureFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_exposure(
        &self,
        input: RenderExposureDescriptor,
    ) -> GisResult<Established<RenderExposureValid>> {
        if let RenderExposureDescriptor::Ev100(ev) = &input {
            check_finite_f32(*ev, "exposure ev100")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderAutoExposureFactory ──────────────────────────────────────────────

impl GisRenderAutoExposureFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_auto_exposure(
        &self,
        input: RenderAutoExposureDescriptor,
    ) -> GisResult<Established<RenderAutoExposureValid>> {
        check_finite_f32(input.range_start, "auto_exposure range_start")?;
        check_finite_f32(input.range_end, "auto_exposure range_end")?;
        if input.range_start >= input.range_end {
            return Err(invalid(
                "auto_exposure range_start must be less than range_end",
            ));
        }
        check_finite_f32(input.filter_start, "auto_exposure filter_start")?;
        check_finite_f32(input.filter_end, "auto_exposure filter_end")?;
        if input.filter_start >= input.filter_end {
            return Err(invalid(
                "auto_exposure filter_start must be less than filter_end",
            ));
        }
        check_positive_f32(input.speed_brighten, "auto_exposure speed_brighten")?;
        check_positive_f32(input.speed_darken, "auto_exposure speed_darken")?;
        check_finite_f32(
            input.exponential_transition_distance,
            "auto_exposure exponential_transition_distance",
        )?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderColorGradingFactory ──────────────────────────────────────────────

impl GisRenderColorGradingFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_color_grading(
        &self,
        input: RenderColorGradingDescriptor,
    ) -> GisResult<Established<RenderColorGradingValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderBloomFactory ─────────────────────────────────────────────────────

impl GisRenderBloomFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_bloom(
        &self,
        input: RenderBloomDescriptor,
    ) -> GisResult<Established<RenderBloomValid>> {
        check_non_negative_f32(input.intensity, "bloom intensity")?;
        check_finite_f32(input.low_frequency_boost, "bloom low_frequency_boost")?;
        check_finite_f32(
            input.low_frequency_boost_curvature,
            "bloom low_frequency_boost_curvature",
        )?;
        check_finite_f32(input.high_pass_frequency, "bloom high_pass_frequency")?;
        check_finite_f32(input.prefilter.threshold, "bloom prefilter threshold")?;
        check_finite_f32(
            input.prefilter.threshold_softness,
            "bloom prefilter threshold_softness",
        )?;
        check_positive_u32(input.max_mip_dimension, "bloom max_mip_dimension")?;
        check_positive_f32(input.scale_x, "bloom scale_x")?;
        check_positive_f32(input.scale_y, "bloom scale_y")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderDepthOfFieldFactory ──────────────────────────────────────────────

impl GisRenderDepthOfFieldFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_depth_of_field(
        &self,
        input: RenderDepthOfFieldDescriptor,
    ) -> GisResult<Established<RenderDepthOfFieldValid>> {
        check_positive_f32(input.focal_distance, "depth_of_field focal_distance")?;
        check_positive_f32(input.sensor_height, "depth_of_field sensor_height")?;
        check_positive_f32(input.aperture_f_stops, "depth_of_field aperture_f_stops")?;
        check_positive_f32(
            input.max_circle_of_confusion_diameter,
            "depth_of_field max_circle_of_confusion_diameter",
        )?;
        check_positive_f32(input.max_depth, "depth_of_field max_depth")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderMotionBlurFactory ────────────────────────────────────────────────

impl GisRenderMotionBlurFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_motion_blur(
        &self,
        input: RenderMotionBlurDescriptor,
    ) -> GisResult<Established<RenderMotionBlurValid>> {
        check_positive_f32(input.shutter_angle, "motion_blur shutter_angle")?;
        check_positive_u32(input.samples, "motion_blur samples")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderChromaticAberrationFactory ───────────────────────────────────────

impl GisRenderChromaticAberrationFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_chromatic_aberration(
        &self,
        input: RenderChromaticAberrationDescriptor,
    ) -> GisResult<Established<RenderChromaticAberrationValid>> {
        check_non_negative_f32(input.intensity, "chromatic_aberration intensity")?;
        check_positive_u32(input.max_samples, "chromatic_aberration max_samples")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderScreenSpaceReflectionsFactory ────────────────────────────────────

impl GisRenderScreenSpaceReflectionsFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_screen_space_reflections(
        &self,
        input: RenderScreenSpaceReflectionsDescriptor,
    ) -> GisResult<Established<RenderScreenSpaceReflectionsValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderScreenSpaceAmbientOcclusionFactory ───────────────────────────────

impl GisRenderScreenSpaceAmbientOcclusionFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_screen_space_ambient_occlusion(
        &self,
        input: RenderScreenSpaceAmbientOcclusionDescriptor,
    ) -> GisResult<Established<RenderScreenSpaceAmbientOcclusionValid>> {
        Ok(Established::prove(&input))
    }
}

// ── GisRenderWorldSpaceFactory ────────────────────────────────────────────────

impl GisRenderWorldSpaceFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_world_space(
        &self,
        input: RenderWorldSpaceDescriptor,
    ) -> GisResult<Established<RenderWorldSpaceValid>> {
        if !input.units_per_meter.is_finite() || input.units_per_meter <= 0.0 {
            return Err(invalid(
                "world_space units_per_meter must be finite and positive",
            ));
        }
        if !input.vertical_exaggeration.is_finite() || input.vertical_exaggeration <= 0.0 {
            return Err(invalid(
                "world_space vertical_exaggeration must be finite and positive",
            ));
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderLightProbeFactory ────────────────────────────────────────────────

impl GisRenderLightProbeFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_light_probe(
        &self,
        input: RenderLightProbeDescriptor,
    ) -> GisResult<Established<RenderLightProbeValid>> {
        check_positive_f32(input.region.size_x, "light_probe region size_x")?;
        check_positive_f32(input.region.size_y, "light_probe region size_y")?;
        check_positive_f32(input.region.size_z, "light_probe region size_z")?;
        if let Some(vol) = &input.irradiance_volume
            && let Some(intensity) = vol.intensity
        {
            check_non_negative_f32(intensity, "irradiance_volume intensity")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderImageBasedLightingFactory ────────────────────────────────────────

impl GisRenderImageBasedLightingFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_image_based_lighting(
        &self,
        input: RenderImageBasedLightingDescriptor,
    ) -> GisResult<Established<RenderImageBasedLightingValid>> {
        if let Some(intensity) = input.intensity {
            check_non_negative_f32(intensity, "ibl intensity")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderSkyboxFactory ────────────────────────────────────────────────────

impl GisRenderSkyboxFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_skybox(
        &self,
        input: RenderSkyboxDescriptor,
    ) -> GisResult<Established<RenderSkyboxValid>> {
        if input.asset.uri.as_str().is_empty() {
            return Err(invalid("skybox asset URI must not be empty"));
        }
        check_non_negative_f32(input.brightness, "skybox brightness")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderAmbientLightFactory ──────────────────────────────────────────────

impl GisRenderAmbientLightFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_ambient_light(
        &self,
        input: RenderAmbientLightDescriptor,
    ) -> GisResult<Established<RenderAmbientLightValid>> {
        check_non_negative_f32(input.brightness, "ambient_light brightness")?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderDirectionalLightFactory ──────────────────────────────────────────

impl GisRenderDirectionalLightFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_directional_light(
        &self,
        input: RenderDirectionalLightDescriptor,
    ) -> GisResult<Established<RenderDirectionalLightValid>> {
        check_non_negative_f32(input.illuminance, "directional_light illuminance")?;
        check_finite_f32(input.azimuth_degrees, "directional_light azimuth_degrees")?;
        check_finite_f32(
            input.elevation_degrees,
            "directional_light elevation_degrees",
        )?;
        Ok(Established::prove(&input))
    }
}

// ── GisRenderAtmosphereFactory ────────────────────────────────────────────────

impl GisRenderAtmosphereFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_atmosphere(
        &self,
        input: RenderAtmosphereDescriptor,
    ) -> GisResult<Established<RenderAtmosphereValid>> {
        check_positive_f32(input.bottom_radius, "atmosphere bottom_radius")?;
        check_positive_f32(input.top_radius, "atmosphere top_radius")?;
        if input.bottom_radius >= input.top_radius {
            return Err(invalid(
                "atmosphere bottom_radius must be less than top_radius",
            ));
        }
        check_positive_u32(input.falloff_resolution, "atmosphere falloff_resolution")?;
        check_positive_u32(input.phase_resolution, "atmosphere phase_resolution")?;
        if input.terms.is_empty() {
            return Err(invalid("atmosphere must have at least one scattering term"));
        }
        if let Some(density) = input.density_multiplier {
            check_non_negative_f32(density, "atmosphere density_multiplier")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderFogFactory ───────────────────────────────────────────────────────

impl GisRenderFogFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_fog(
        &self,
        input: RenderFogDescriptor,
    ) -> GisResult<Established<RenderFogValid>> {
        use elicit_gis::RenderFogFalloffDescriptor as Falloff;
        match &input.falloff {
            Falloff::Linear { start, end } => {
                check_finite_f32(*start, "fog linear start")?;
                check_finite_f32(*end, "fog linear end")?;
                if start >= end {
                    return Err(invalid("fog linear start must be less than end"));
                }
            }
            Falloff::Exponential { density } | Falloff::ExponentialSquared { density } => {
                check_positive_f32(*density, "fog density")?;
            }
            Falloff::Atmospheric {
                extinction_r,
                extinction_g,
                extinction_b,
                inscattering_r,
                inscattering_g,
                inscattering_b,
            } => {
                for (v, n) in [
                    (*extinction_r, "extinction_r"),
                    (*extinction_g, "extinction_g"),
                    (*extinction_b, "extinction_b"),
                    (*inscattering_r, "inscattering_r"),
                    (*inscattering_g, "inscattering_g"),
                    (*inscattering_b, "inscattering_b"),
                ] {
                    check_non_negative_f32(v, &format!("fog atmospheric {n}"))?;
                }
            }
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderVolumetricFogFactory ─────────────────────────────────────────────

impl GisRenderVolumetricFogFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_volumetric_fog(
        &self,
        input: RenderVolumetricFogDescriptor,
    ) -> GisResult<Established<RenderVolumetricFogValid>> {
        if let Some(intensity) = input.ambient_intensity {
            check_non_negative_f32(intensity, "volumetric_fog ambient_intensity")?;
        }
        if let Some(jitter) = input.jitter {
            check_non_negative_f32(jitter, "volumetric_fog jitter")?;
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderFogVolumeFactory ─────────────────────────────────────────────────

impl GisRenderFogVolumeFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_fog_volume(
        &self,
        input: RenderFogVolumeDescriptor,
    ) -> GisResult<Established<RenderFogVolumeValid>> {
        if let Some(density) = input.density_factor {
            check_non_negative_f32(density, "fog_volume density_factor")?;
        }
        if let Some(absorption) = input.absorption {
            check_non_negative_f32(absorption, "fog_volume absorption")?;
        }
        if let Some(scattering) = input.scattering {
            check_non_negative_f32(scattering, "fog_volume scattering")?;
        }
        if let Some(asymmetry) = input.scattering_asymmetry {
            check_finite_f32(asymmetry, "fog_volume scattering_asymmetry")?;
            if !(-1.0..=1.0).contains(&asymmetry) {
                return Err(invalid(
                    "fog_volume scattering_asymmetry must be in [-1.0, 1.0]",
                ));
            }
        }
        Ok(Established::prove(&input))
    }
}

// ── GisRenderCascadeShadowFactory ─────────────────────────────────────────────

impl GisRenderCascadeShadowFactory for BevyGisBackend {
    #[instrument(skip_all)]
    fn build_render_cascade_shadows(
        &self,
        input: RenderCascadeShadowConfigDescriptor,
    ) -> GisResult<Established<RenderCascadeShadowConfigValid>> {
        if let Some(proportion) = input.overlap_proportion {
            check_finite_f32(proportion, "cascade_shadow overlap_proportion")?;
            if !(0.0..=1.0).contains(&proportion) {
                return Err(invalid(
                    "cascade_shadow overlap_proportion must be in [0.0, 1.0]",
                ));
            }
        }
        if let Some(min_dist) = input.minimum_distance {
            check_non_negative_f32(min_dist, "cascade_shadow minimum_distance")?;
        }
        if let Some(max_dist) = input.maximum_distance {
            check_positive_f32(max_dist, "cascade_shadow maximum_distance")?;
        }
        if let (Some(min_dist), Some(max_dist)) = (input.minimum_distance, input.maximum_distance)
            && min_dist >= max_dist
        {
            return Err(invalid(
                "cascade_shadow minimum_distance must be less than maximum_distance",
            ));
        }
        if let Some(num) = input.num_cascades {
            check_positive_u32(num, "cascade_shadow num_cascades")?;
        }
        Ok(Established::prove(&input))
    }
}
