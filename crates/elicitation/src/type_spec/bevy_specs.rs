//! [`ElicitSpec`](crate::ElicitSpec) implementations for Bevy 0.18 types.
//!
//! Available with the `bevy-types` feature.

#[cfg(feature = "bevy-types")]
mod bevy_impls {
    use crate::{
        BevyAlignContent, BevyAlignItems, BevyAlignSelf, BevyAlphaMode, BevyAmbientLight,
        BevyAnchor, BevyAtmosphere, BevyBorderRadius, BevyBoxSizing, BevyButtonState, BevyColor,
        BevyDirectionalLight, BevyDisplay, BevyFalloff, BevyFlexDirection, BevyFlexWrap,
        BevyFontSmoothing, BevyGamepadAxis, BevyGamepadButton, BevyGlobalTransform, BevyJustify,
        BevyJustifyContent, BevyJustifyItems, BevyJustifySelf, BevyKeyCode, BevyLineBreak,
        BevyMonitorSelection, BevyMouseButton, BevyOrthographicProjection, BevyOverflowAxis,
        BevyOverflowClipBox, BevyPerspectiveProjection, BevyPhaseFunction, BevyPickable,
        BevyPickingInteraction, BevyPlaybackMode, BevyPlaybackSettings, BevyPointLight,
        BevyPositionType, BevyPresentMode, BevyRepeatAnimation, BevyScalingMode,
        BevyScatteringTerm, BevySpotLight, BevySpriteConfig, BevySpriteScalingMode,
        BevyStandardMaterial, BevyTextFont, BevyTimer, BevyTonemapping, BevyTouchPhase,
        BevyTransform, BevyUiRect, BevyVal, BevyVolume, BevyWindowLevel, BevyWindowMode,
        BevyWindowResolution, BevyWindowTheme, ElicitSpec, Select, SpecCategoryBuilder,
        SpecEntryBuilder, TypeSpec, TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Helper: Select-pattern TypeSpec from Select::labels()
    // -------------------------------------------------------------------------

    fn _bevy_select_type_spec<T: Select>(name: &str, summary: &str) -> TypeSpec {
        let variants = SpecCategoryBuilder::default()
            .name("variants".to_string())
            .entries(
                T::labels()
                    .into_iter()
                    .map(|label| {
                        SpecEntryBuilder::default()
                            .label(label.clone())
                            .description(label)
                            .build()
                            .expect("valid SpecEntry")
                    })
                    .collect(),
            )
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("crate".to_string())
                    .description("bevy v0.18.1 — data-driven game engine".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description("Select — choose one variant from the list".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name(name.to_string())
            .summary(summary.to_string())
            .categories(vec![variants, source])
            .build()
            .expect("valid TypeSpec")
    }

    // -------------------------------------------------------------------------
    // Helper: Survey-pattern TypeSpec from field list
    // -------------------------------------------------------------------------

    fn _bevy_survey_type_spec(name: &str, summary: &str, fields: Vec<(&str, &str)>) -> TypeSpec {
        let fields_cat = SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(
                fields
                    .into_iter()
                    .map(|(fname, fdesc)| {
                        SpecEntryBuilder::default()
                            .label(fname.to_string())
                            .description(fdesc.to_string())
                            .build()
                            .expect("valid SpecEntry")
                    })
                    .collect(),
            )
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name(name.to_string())
            .summary(summary.to_string())
            .categories(vec![fields_cat])
            .build()
            .expect("valid TypeSpec")
    }

    // -------------------------------------------------------------------------
    // Macros
    // -------------------------------------------------------------------------

    macro_rules! impl_bevy_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _bevy_select_type_spec::<$ty>($name, $summary)
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    macro_rules! impl_bevy_survey_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [ $( ($fname:literal, $fdesc:literal) ),* $(,)? ]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _bevy_survey_type_spec(
                        $name,
                        $summary,
                        vec![ $( ($fname, $fdesc) ),* ],
                    )
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    macro_rules! impl_bevy_variants_spec {
        (
            type     = $ty:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            variants = [ $($variant:literal),* $(,)? ]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _bevy_select_type_spec_from_labels(
                        $name,
                        $summary,
                        &[$($variant),*],
                    )
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    fn _bevy_select_type_spec_from_labels(
        name: &str,
        summary: &str,
        variant_labels: &[&str],
    ) -> TypeSpec {
        let variants = SpecCategoryBuilder::default()
            .name("variants".to_string())
            .entries(
                variant_labels
                    .iter()
                    .map(|label| {
                        SpecEntryBuilder::default()
                            .label(label.to_string())
                            .description(label.to_string())
                            .build()
                            .expect("valid SpecEntry")
                    })
                    .collect(),
            )
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("crate".to_string())
                    .description("bevy v0.18.1 — data-driven game engine".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description("Select — choose one variant from the list".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name(name.to_string())
            .summary(summary.to_string())
            .categories(vec![variants, source])
            .build()
            .expect("valid TypeSpec")
    }

    // ── Render / material enums ───────────────────────────────────────────────

    impl_bevy_variants_spec!(
        type     = BevyAlphaMode,
        name     = "BevyAlphaMode",
        summary  = "Material alpha blending mode: Opaque, Mask, Blend, Premultiplied, Add, Multiply.",
        variants = ["Opaque", "Mask", "Blend", "Premultiplied", "AlphaToCoverage", "Add", "Multiply"]
    );

    impl_bevy_select_spec!(
        type    = BevyTonemapping,
        name    = "BevyTonemapping",
        summary = "HDR tonemapping algorithm: None, Reinhard, AcesFit, AgX, etc."
    );

    // ── Window / monitor enums ────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyPresentMode,
        name    = "BevyPresentMode",
        summary = "Swap-chain presentation mode: AutoVsync, Fifo, Mailbox, Immediate, etc."
    );

    impl_bevy_select_spec!(
        type    = BevyWindowLevel,
        name    = "BevyWindowLevel",
        summary = "OS window z-level: AlwaysOnBottom, Normal, or AlwaysOnTop."
    );

    impl_bevy_select_spec!(
        type    = BevyWindowTheme,
        name    = "BevyWindowTheme",
        summary = "Requested OS window decoration theme: Light or Dark."
    );

    impl_bevy_select_spec!(
        type    = BevyMonitorSelection,
        name    = "BevyMonitorSelection",
        summary = "Monitor selection for fullscreen: Current, Primary, or Index(n)."
    );

    impl_bevy_variants_spec!(
        type     = BevyWindowMode,
        name     = "BevyWindowMode",
        summary  = "Window display mode: Windowed, BorderlessFullscreen, SizedFullscreen, Fullscreen.",
        variants = ["Windowed", "BorderlessFullscreen", "Fullscreen"]
    );

    // ── Input enums ───────────────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyButtonState,
        name    = "BevyButtonState",
        summary = "Button pressed/released state: Pressed or Released."
    );

    impl_bevy_select_spec!(
        type    = BevyTouchPhase,
        name    = "BevyTouchPhase",
        summary = "Touch event phase: Started, Moved, Ended, or Cancelled."
    );

    impl_bevy_select_spec!(
        type    = BevyMouseButton,
        name    = "BevyMouseButton",
        summary = "Mouse button: Left, Right, Middle, Back, Forward, or Other(u16)."
    );

    impl_bevy_select_spec!(
        type    = BevyKeyCode,
        name    = "BevyKeyCode",
        summary = "Physical keyboard key code (186 variants covering full USB HID keyboard layout)."
    );

    impl_bevy_select_spec!(
        type    = BevyGamepadButton,
        name    = "BevyGamepadButton",
        summary = "Gamepad button: South/East/North/West face buttons, shoulders, triggers, sticks, D-pad, etc."
    );

    impl_bevy_select_spec!(
        type    = BevyGamepadAxis,
        name    = "BevyGamepadAxis",
        summary = "Gamepad axis: LeftStickX/Y, RightStickX/Y, LeftZ, RightZ."
    );

    // ── UI layout enums ───────────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyAlignItems,
        name    = "BevyAlignItems",
        summary = "Cross-axis child alignment: Default, Start, End, Center, Baseline, Stretch."
    );

    impl_bevy_select_spec!(
        type    = BevyJustifyItems,
        name    = "BevyJustifyItems",
        summary = "Inline-axis child justification: Default, Start, End, Center, Baseline, Stretch."
    );

    impl_bevy_select_spec!(
        type    = BevyAlignSelf,
        name    = "BevyAlignSelf",
        summary = "Self cross-axis alignment override: Auto, Start, End, Center, Baseline, Stretch."
    );

    impl_bevy_select_spec!(
        type    = BevyJustifySelf,
        name    = "BevyJustifySelf",
        summary = "Self inline-axis justification override: Auto, Start, End, Center, Baseline, Stretch."
    );

    impl_bevy_select_spec!(
        type    = BevyAlignContent,
        name    = "BevyAlignContent",
        summary = "Multi-line cross-axis packing: Default, Start, End, Center, Stretch, SpaceBetween, SpaceAround, SpaceEvenly."
    );

    impl_bevy_select_spec!(
        type    = BevyJustifyContent,
        name    = "BevyJustifyContent",
        summary = "Main-axis packing: Default, Start, End, Center, SpaceBetween, SpaceAround, SpaceEvenly, Stretch."
    );

    impl_bevy_select_spec!(
        type    = BevyDisplay,
        name    = "BevyDisplay",
        summary = "Node display mode: Flex, Grid, Block, or None."
    );

    impl_bevy_select_spec!(
        type    = BevyBoxSizing,
        name    = "BevyBoxSizing",
        summary = "CSS box-sizing mode: BorderBox (includes border/padding) or ContentBox."
    );

    impl_bevy_select_spec!(
        type    = BevyFlexDirection,
        name    = "BevyFlexDirection",
        summary = "Flex layout main axis: Row, Column, RowReverse, or ColumnReverse."
    );

    impl_bevy_select_spec!(
        type    = BevyFlexWrap,
        name    = "BevyFlexWrap",
        summary = "Flex line wrapping: NoWrap, Wrap, or WrapReverse."
    );

    impl_bevy_select_spec!(
        type    = BevyPositionType,
        name    = "BevyPositionType",
        summary = "Node positioning: Relative (normal flow) or Absolute (removed from flow)."
    );

    impl_bevy_select_spec!(
        type    = BevyOverflowAxis,
        name    = "BevyOverflowAxis",
        summary = "Single-axis overflow handling: Visible, Clip, Hidden, or Scroll."
    );

    impl_bevy_select_spec!(
        type    = BevyOverflowClipBox,
        name    = "BevyOverflowClipBox",
        summary = "Overflow clip reference box: ContentBox or PaddingBox."
    );

    impl_bevy_variants_spec!(
        type     = BevyVal,
        name     = "BevyVal",
        summary  = "UI measurement: Auto, Px (logical pixels), Percent, Vw, Vh, VMin, or VMax.",
        variants = ["Auto", "Px", "Percent", "Vw", "Vh", "VMin", "VMax"]
    );

    // ── Text enums ────────────────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyFontSmoothing,
        name    = "BevyFontSmoothing",
        summary = "Font rendering anti-aliasing: None or AntiAliased."
    );

    impl_bevy_select_spec!(
        type    = BevyJustify,
        name    = "BevyJustify",
        summary = "Text horizontal alignment: Left, Center, Right, or Justified."
    );

    impl_bevy_select_spec!(
        type    = BevyLineBreak,
        name    = "BevyLineBreak",
        summary = "Text line-breaking policy: WordBoundary, AnyCharacter, WordOrCharacter, or NoWrap."
    );

    // ── Audio enums ───────────────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyPlaybackMode,
        name    = "BevyPlaybackMode",
        summary = "Audio playback mode: Once, Loop, Despawn, or Remove on completion."
    );

    // ── Animation enums ───────────────────────────────────────────────────────

    impl_bevy_variants_spec!(
        type     = BevyRepeatAnimation,
        name     = "BevyRepeatAnimation",
        summary  = "Animation repeat policy: Never, Forever, or Count(n).",
        variants = ["Never", "Count", "Forever"]
    );

    // ── Sprite enums ──────────────────────────────────────────────────────────

    impl_bevy_survey_spec!(
        type    = BevyAnchor,
        name    = "BevyAnchor",
        summary = "Sprite/image origin anchor position in normalized sprite space (Vec2, range [−0.5, 0.5]).",
        fields  = [
            ("position", "Anchor position (Vec2): x/y in [−0.5, 0.5], (0,0) = center, (−0.5,−0.5) = bottom-left"),
        ]
    );

    impl_bevy_select_spec!(
        type    = BevySpriteScalingMode,
        name    = "BevySpriteScalingMode",
        summary = "9-slice sprite stretch mode: SliceByPercent (relative), SliceByPixels (absolute)."
    );

    // ── Atmosphere enums ──────────────────────────────────────────────────────

    impl_bevy_variants_spec!(
        type     = BevyFalloff,
        name     = "BevyFalloff",
        summary  = "Atmosphere density falloff: Linear, Exponential { scale }, or Tent { center, width }.",
        variants = ["Linear", "Exponential", "Tent"]
    );

    impl_bevy_variants_spec!(
        type     = BevyPhaseFunction,
        name     = "BevyPhaseFunction",
        summary  = "Light scattering phase function: Isotropic, Rayleigh, or Mie { asymmetry }.",
        variants = ["Isotropic", "Rayleigh", "Mie"]
    );

    // ── Picking enum ──────────────────────────────────────────────────────────

    impl_bevy_select_spec!(
        type    = BevyPickingInteraction,
        name    = "BevyPickingInteraction",
        summary = "Pointer interaction state: None, Hovered, or Pressed."
    );

    // ── Camera enum ───────────────────────────────────────────────────────────

    impl_bevy_variants_spec!(
        type     = BevyScalingMode,
        name     = "BevyScalingMode",
        summary  = "Orthographic projection scaling: WindowSize, AutoMin/AutoMax, FixedVertical, FixedHorizontal, Fixed.",
        variants = ["WindowSize", "Fixed", "AutoMin", "AutoMax", "FixedVertical", "FixedHorizontal"]
    );

    // ── Survey types ──────────────────────────────────────────────────────────

    impl_bevy_survey_spec!(
        type    = BevyTransform,
        name    = "BevyTransform",
        summary = "3D entity transform (translation Vec3, rotation Quat, scale Vec3).",
        fields  = [
            ("translation", "World-space position (Vec3)"),
            ("rotation", "Rotation quaternion (Quat)"),
            ("scale",       "Scale factor (Vec3, default 1,1,1)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyGlobalTransform,
        name    = "BevyGlobalTransform",
        summary = "Computed world-space transform (Affine3A — do not set directly).",
        fields  = [
            ("matrix", "Column-major 4×3 affine matrix (Affine3A)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyWindowResolution,
        name    = "BevyWindowResolution",
        summary = "Window physical/logical resolution (width and height in pixels).",
        fields  = [
            ("width",  "Physical width in pixels (u32)"),
            ("height", "Physical height in pixels (u32)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyTimer,
        name    = "BevyTimer",
        summary = "Countdown or stopwatch timer with optional repeat.",
        fields  = [
            ("duration_secs", "Timer duration in seconds (f32)"),
            ("mode",          "Once or Repeating"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyVolume,
        name    = "BevyVolume",
        summary = "Audio volume level [0.0, 1.0] (linear amplitude).",
        fields  = [
            ("volume", "Volume level [0.0, 1.0] (f32)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyPlaybackSettings,
        name    = "BevyPlaybackSettings",
        summary = "Audio playback configuration (mode, volume, speed, spatial, paused).",
        fields  = [
            ("mode",    "BevyPlaybackMode — Once/Loop/Despawn/Remove"),
            ("volume",  "Playback volume (BevyVolume)"),
            ("speed",   "Playback speed multiplier (f32, 1.0 = normal)"),
            ("paused",  "Start in paused state (bool)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyColor,
        name    = "BevyColor",
        summary = "Bevy color in one of four colour spaces: Srgba, LinearRgba, Hsla, or Oklcha.",
        fields  = [
            ("space",  "Color space (Srgba / LinearRgba / Hsla / Oklcha)"),
            ("ch0",    "First channel (R / R / H / L) — f32"),
            ("ch1",    "Second channel (G / G / S / C) — f32"),
            ("ch2",    "Third channel (B / B / L / H) — f32"),
            ("alpha",  "Alpha channel [0.0, 1.0] — f32"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyStandardMaterial,
        name    = "BevyStandardMaterial",
        summary = "PBR standard material (numeric/boolean parameters only; texture handles excluded).",
        fields  = [
            ("base_color_r",               "Base color red [0, 1]"),
            ("base_color_g",               "Base color green [0, 1]"),
            ("base_color_b",               "Base color blue [0, 1]"),
            ("base_color_a",               "Base color alpha [0, 1]"),
            ("emissive_r",                 "Emissive red (HDR, may exceed 1.0)"),
            ("emissive_g",                 "Emissive green (HDR)"),
            ("emissive_b",                 "Emissive blue (HDR)"),
            ("emissive_exposure_weight",   "Emissive exposure weight [0, 1]"),
            ("perceptual_roughness",       "Perceptual roughness [0, 1]"),
            ("metallic",                   "Metallic factor [0, 1]"),
            ("reflectance",                "Dielectric specular reflectance [0, 1]"),
            ("ior",                        "Index of refraction (≥ 1.0)"),
            ("double_sided",               "Render both faces (bool)"),
            ("unlit",                      "Disable all lighting (bool)"),
            ("fog_enabled",                "Affected by fog (bool)"),
            ("depth_bias",                 "Constant depth offset (f32)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyTextFont,
        name    = "BevyTextFont",
        summary = "Text font configuration (font asset path, size, smoothing).",
        fields  = [
            ("font",          "Font asset path (String)"),
            ("font_size",     "Font size in logical pixels (f32)"),
            ("font_smoothing","Anti-aliasing mode (BevyFontSmoothing)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevySpriteConfig,
        name    = "BevySpriteConfig",
        summary = "Sprite configuration (color tint, flip axes, custom size, anchor).",
        fields  = [
            ("color",         "Tint color (BevyColor)"),
            ("flip_x",        "Flip horizontally (bool)"),
            ("flip_y",        "Flip vertically (bool)"),
            ("custom_size",   "Optional custom render size (Option<BevyVec2>)"),
            ("anchor",        "Sprite origin anchor (BevyAnchor)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyPickable,
        name    = "BevyPickable",
        summary = "Picking interaction settings (block lower entities, enable hover).",
        fields  = [
            ("should_block_lower", "Block pointer events to entities below (bool)"),
            ("is_hoverable",       "Allow hover detection (bool)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyOrthographicProjection,
        name    = "BevyOrthographicProjection",
        summary = "Orthographic camera projection (scaling mode, near/far planes, scale).",
        fields  = [
            ("near",         "Near clipping plane distance (f32)"),
            ("far",          "Far clipping plane distance (f32)"),
            ("scale",        "Zoom scale factor (f32, 1.0 = default)"),
            ("scaling_mode", "How viewport adapts to window (BevyScalingMode)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyPerspectiveProjection,
        name    = "BevyPerspectiveProjection",
        summary = "Perspective camera projection (FOV, near/far planes, aspect ratio).",
        fields  = [
            ("fov",          "Vertical field of view in radians (f32)"),
            ("near",         "Near clipping plane distance (f32)"),
            ("far",          "Far clipping plane distance (f32)"),
            ("aspect_ratio", "Viewport aspect ratio width/height (f32)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyScatteringTerm,
        name    = "BevyScatteringTerm",
        summary = "Single optical element of an atmosphere medium (absorption, scattering, falloff, phase).",
        fields  = [
            ("absorption", "Optical absorption density per metre (BevyVec3)"),
            ("scattering", "Optical scattering density per metre (BevyVec3)"),
            ("falloff",    "Density falloff distribution (BevyFalloff)"),
            ("phase",      "Light scattering phase function (BevyPhaseFunction)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyAtmosphere,
        name    = "BevyAtmosphere",
        summary = "Atmospheric scattering parameters for an HDR camera (planet radii, ground albedo).",
        fields  = [
            ("bottom_radius", "Planet surface radius in metres (f32, Earth ≈ 6 360 000)"),
            ("top_radius",    "Outer atmosphere radius in metres (f32, Earth ≈ 6 460 000)"),
            ("ground_albedo", "Average planet surface albedo (BevyVec3)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyUiRect,
        name    = "BevyUiRect",
        summary = "UI edge insets/margins (left, right, top, bottom in BevyVal).",
        fields  = [
            ("left",   "Left edge (BevyVal)"),
            ("right",  "Right edge (BevyVal)"),
            ("top",    "Top edge (BevyVal)"),
            ("bottom", "Bottom edge (BevyVal)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyBorderRadius,
        name    = "BevyBorderRadius",
        summary = "UI node corner radii (top-left, top-right, bottom-right, bottom-left in BevyVal).",
        fields  = [
            ("top_left",     "Top-left corner radius (BevyVal)"),
            ("top_right",    "Top-right corner radius (BevyVal)"),
            ("bottom_right", "Bottom-right corner radius (BevyVal)"),
            ("bottom_left",  "Bottom-left corner radius (BevyVal)"),
        ]
    );

    // Light types

    impl_bevy_survey_spec!(
        type    = BevyDirectionalLight,
        name    = "BevyDirectionalLight",
        summary = "Directional (sun-like) light parameters (illuminance, color, shadows).",
        fields  = [
            ("illuminance",       "Light illuminance in lux (f32)"),
            ("color_r",           "Light color red channel [0, 1]"),
            ("color_g",           "Light color green channel [0, 1]"),
            ("color_b",           "Light color blue channel [0, 1]"),
            ("shadows_enabled",   "Cast shadows (bool)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyPointLight,
        name    = "BevyPointLight",
        summary = "Omnidirectional point light parameters (intensity, range, radius, shadows).",
        fields  = [
            ("intensity",       "Luminous power in lumens (f32)"),
            ("range",           "Effective light range in metres (f32)"),
            ("radius",          "Physical light radius for soft shadows (f32)"),
            ("color_r",         "Light color red channel [0, 1]"),
            ("color_g",         "Light color green channel [0, 1]"),
            ("color_b",         "Light color blue channel [0, 1]"),
            ("shadows_enabled", "Cast shadows (bool)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevySpotLight,
        name    = "BevySpotLight",
        summary = "Cone spot light parameters (intensity, range, inner/outer angle, shadows).",
        fields  = [
            ("intensity",         "Luminous power in lumens (f32)"),
            ("range",             "Effective light range in metres (f32)"),
            ("radius",            "Physical light radius for soft shadows (f32)"),
            ("color_r",           "Light color red channel [0, 1]"),
            ("color_g",           "Light color green channel [0, 1]"),
            ("color_b",           "Light color blue channel [0, 1]"),
            ("inner_angle",       "Fully-lit cone half-angle in radians (f32)"),
            ("outer_angle",       "Fade-out cone half-angle in radians (f32)"),
            ("shadows_enabled",   "Cast shadows (bool)"),
        ]
    );

    impl_bevy_survey_spec!(
        type    = BevyAmbientLight,
        name    = "BevyAmbientLight",
        summary = "Scene-wide ambient (fill) light (color and brightness).",
        fields  = [
            ("color_r",     "Ambient color red channel [0, 1]"),
            ("color_g",     "Ambient color green channel [0, 1]"),
            ("color_b",     "Ambient color blue channel [0, 1]"),
            ("brightness",  "Lux value of ambient illuminance (f32)"),
        ]
    );
}
