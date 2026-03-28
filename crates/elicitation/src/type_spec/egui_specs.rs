//! [`ElicitSpec`](crate::ElicitSpec) implementations for egui type elicitation.
//!
//! Available with the `egui-types` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/egui_types/` — those describe *structure* (pattern, variants),
//! these describe *contracts and usage* browsable by agents via `describe_type`.
//!
//! `ElicitSpec` is implemented on both the raw egui types and their
//! [`select_trenchcoat!`](crate::select_trenchcoat) wrappers.
//! [`ElicitComplete`](crate::ElicitComplete) is only implemented on the
//! trenchcoat wrappers (which add the missing `JsonSchema` + `serde`).

#[cfg(feature = "egui-types")]
mod egui_impls {
    use crate::{
        ElicitComplete, ElicitSpec, Select, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_egui_select_spec!
    //
    // Derives ElicitSpec for an egui Select enum using Select::labels()
    // at runtime. Also impls ElicitSpec + ElicitComplete on the trenchcoat
    // wrapper, which satisfies all remaining bounds via select_trenchcoat!.
    // -------------------------------------------------------------------------

    macro_rules! impl_egui_select_spec {
        (
            type      = $ty:ty,
            wrapper   = $wrapper:ty,
            name      = $name:literal,
            summary   = $summary:literal
        ) => {
            // ElicitSpec on the raw egui type
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _egui_type_spec::<$ty>($name, $summary)
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            // ElicitSpec on the trenchcoat wrapper (delegates to raw type)
            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    <$ty as ElicitSpec>::type_spec()
                }
            }

            // ElicitComplete on the trenchcoat wrapper — it has all bounds:
            // Elicitation + ElicitIntrospect + ElicitSpec + Serialize +
            // Deserialize + JsonSchema (the last three from select_trenchcoat!)
            impl ElicitComplete for $wrapper {}
        };
    }

    fn _egui_type_spec<T: Select>(name: &str, summary: &str) -> TypeSpec {
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
                    .description("egui v0.33 — immediate-mode GUI library for Rust".to_string())
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

    impl_egui_select_spec!(
        type      = egui::Align,
        wrapper   = crate::AlignSelect,
        name      = "egui::Align",
        summary   = "Layout alignment: Min (left/top), Center, or Max (right/bottom)."
    );

    impl_egui_select_spec!(
        type      = egui::CursorIcon,
        wrapper   = crate::CursorIconSelect,
        name      = "egui::CursorIcon",
        summary   = "The visual cursor icon displayed when hovering over a UI element — \
                     35 standard CSS cursor variants."
    );

    impl_egui_select_spec!(
        type      = egui::Direction,
        wrapper   = crate::DirectionSelect,
        name      = "egui::Direction",
        summary   = "Layout flow direction: LeftToRight, RightToLeft, TopDown, or BottomUp."
    );

    impl_egui_select_spec!(
        type      = egui::FontFamily,
        wrapper   = crate::FontFamilySelect,
        name      = "egui::FontFamily",
        summary   = "A font family selector: Monospace or Proportional \
                     (custom Name(String) variants are elicited separately)."
    );

    impl_egui_select_spec!(
        type      = egui::Key,
        wrapper   = crate::KeySelect,
        name      = "egui::Key",
        summary   = "A logical keyboard key — arrows, letters, digits, function keys, \
                     and special keys (97 variants)."
    );

    impl_egui_select_spec!(
        type      = egui::Order,
        wrapper   = crate::OrderSelect,
        name      = "egui::Order",
        summary   = "Paint layer ordering: Background, Middle, Foreground, Tooltip, or Debug."
    );

    impl_egui_select_spec!(
        type      = egui::PointerButton,
        wrapper   = crate::PointerButtonSelect,
        name      = "egui::PointerButton",
        summary   = "Mouse/pointer button: Primary, Secondary, Middle, Extra1, or Extra2."
    );

    impl_egui_select_spec!(
        type      = egui::TextStyle,
        wrapper   = crate::TextStyleSelect,
        name      = "egui::TextStyle",
        summary   = "A named text style preset: Small, Body, Monospace, Button, or Heading \
                     (custom Name(String) variants are elicited separately)."
    );

    impl_egui_select_spec!(
        type      = egui::TextWrapMode,
        wrapper   = crate::TextWrapModeSelect,
        name      = "egui::TextWrapMode",
        summary   = "How to handle text that exceeds the available width: \
                     Extend (no wrap), Wrap, or Truncate."
    );

    impl_egui_select_spec!(
        type      = egui::epaint::textures::TextureFilter,
        wrapper   = crate::TextureFilterSelect,
        name      = "egui::TextureFilter",
        summary   = "Texture sampling filter for scaling: Nearest (pixelated) or Linear (smooth)."
    );

    impl_egui_select_spec!(
        type      = egui::epaint::textures::TextureWrapMode,
        wrapper   = crate::TextureWrapModeSelect,
        name      = "egui::TextureWrapMode",
        summary   = "How texture coordinates outside [0,1] are handled: \
                     ClampToEdge, Repeat, or MirroredRepeat."
    );

    impl_egui_select_spec!(
        type      = egui::Theme,
        wrapper   = crate::ThemeSelect,
        name      = "egui::Theme",
        summary   = "The visual theme for egui: Dark or Light."
    );

    impl_egui_select_spec!(
        type      = egui::ThemePreference,
        wrapper   = crate::ThemePreferenceSelect,
        name      = "egui::ThemePreference",
        summary   = "User preference for visual theme: Dark, Light, or System (follow OS)."
    );

    impl_egui_select_spec!(
        type      = egui::TouchPhase,
        wrapper   = crate::TouchPhaseSelect,
        name      = "egui::TouchPhase",
        summary   = "The phase of a touch gesture: Start, Move, End, or Cancel."
    );

    impl_egui_select_spec!(
        type      = egui::UiKind,
        wrapper   = crate::UiKindSelect,
        name      = "egui::UiKind",
        summary   = "The kind of UI region: Window, CentralPanel, SidePanel, Menu, \
                     Popup, Tooltip, and other container types (17 variants)."
    );

    impl_egui_select_spec!(
        type      = egui::WidgetType,
        wrapper   = crate::WidgetTypeSelect,
        name      = "egui::WidgetType",
        summary   = "The type of a built-in egui widget: Label, Button, TextEdit, \
                     Slider, ComboBox, and others (18 variants)."
    );

    // -------------------------------------------------------------------------
    // Macro: impl_egui_composite_spec!
    //
    // Derives ElicitSpec + ElicitComplete for composite struct wrappers
    // (Survey pattern). Each wrapper already has serde + JsonSchema via
    // derive, and Elicitation + ElicitIntrospect + ElicitSpec from this macro.
    // -------------------------------------------------------------------------

    macro_rules! impl_egui_composite_spec {
        (
            wrapper = $wrapper:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [ $( ($field_name:literal, $field_desc:literal) ),+ $(,)? ]
        ) => {
            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    let field_entries: Vec<_> = vec![
                        $(
                            SpecEntryBuilder::default()
                                .label($field_name.to_string())
                                .description($field_desc.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        )+
                    ];

                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(field_entries)
                        .build()
                        .expect("valid SpecCategory");

                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description(
                                    "egui v0.33 — immediate-mode GUI library for Rust"
                                        .to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description(
                                    "Survey — elicit each field in sequence".to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");

                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$wrapper as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$wrapper>
            ));

            impl ElicitComplete for $wrapper {}
        };
    }

    // ── Composite struct wrappers ────────────────────────────────────────

    impl_egui_composite_spec!(
        wrapper = crate::EguiColor32,
        name = "egui::Color32",
        summary = "A 32-bit sRGBA color with unmultiplied alpha (r, g, b, a).",
        fields = [
            ("r", "Red channel (0–255)"),
            ("g", "Green channel (0–255)"),
            ("b", "Blue channel (0–255)"),
            ("a", "Alpha channel (0–255, 255 = opaque)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiPos2,
        name = "egui::Pos2",
        summary = "A 2D position in screen coordinates (x, y).",
        fields = [
            ("x", "Horizontal position in points"),
            ("y", "Vertical position in points"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiVec2,
        name = "egui::Vec2",
        summary = "A 2D vector representing size or offset (x, y).",
        fields = [
            ("x", "Horizontal component in points"),
            ("y", "Vertical component in points"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiRect,
        name = "egui::Rect",
        summary = "An axis-aligned rectangle defined by min/max corners.",
        fields = [
            ("min_x", "Left edge (minimum x)"),
            ("min_y", "Top edge (minimum y)"),
            ("max_x", "Right edge (maximum x)"),
            ("max_y", "Bottom edge (maximum y)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiStroke,
        name = "egui::Stroke",
        summary = "A stroke defined by width and color, used for outlines and borders.",
        fields = [
            ("width", "Stroke width in points"),
            ("color", "Stroke color (EguiColor32)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiCornerRadius,
        name = "egui::CornerRadius",
        summary = "Corner rounding radii for rectangles (nw, ne, sw, se).",
        fields = [
            ("nw", "North-west corner radius (0–255)"),
            ("ne", "North-east corner radius (0–255)"),
            ("sw", "South-west corner radius (0–255)"),
            ("se", "South-east corner radius (0–255)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiShadow,
        name = "egui::Shadow",
        summary = "A box shadow with offset, blur, spread, and color.",
        fields = [
            ("offset_x", "Horizontal offset (-128 to 127)"),
            ("offset_y", "Vertical offset (-128 to 127)"),
            ("blur", "Blur radius (0–255)"),
            ("spread", "Spread radius (0–255)"),
            ("color", "Shadow color (EguiColor32)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiMargin,
        name = "egui::Margin",
        summary = "Margin around a rectangle (left, right, top, bottom).",
        fields = [
            ("left", "Left margin (-128 to 127)"),
            ("right", "Right margin (-128 to 127)"),
            ("top", "Top margin (-128 to 127)"),
            ("bottom", "Bottom margin (-128 to 127)"),
        ]
    );

    impl_egui_composite_spec!(
        wrapper = crate::EguiFontId,
        name = "egui::FontId",
        summary = "A font identifier combining size and family.",
        fields = [
            ("size", "Font size in points"),
            ("family", "Font family (FontFamilySelect)"),
        ]
    );
}
