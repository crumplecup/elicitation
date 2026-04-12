//! [`ElicitSpec`](crate::ElicitSpec) implementations for winit windowing types.
//!
//! Available with the `winit-types` feature.
//!
//! `ElicitSpec` is implemented on both the raw winit types and their
//! [`select_trenchcoat!`](crate::select_trenchcoat) wrappers.
//! [`ElicitComplete`](crate::ElicitComplete) is implemented on all exported
//! winit types — both trenchcoat wrappers and the plain struct types.

#[cfg(feature = "winit-types")]
mod winit_impls {
    use crate::{
        ElicitComplete, ElicitSpec, Select, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey, WinitCursorIconSelect, WinitElementStateSelect,
        WinitKeyCodeSelect, WinitLogicalPosition, WinitLogicalSize, WinitMouseButtonSelect,
        WinitPhysicalSize, WinitThemeSelect, WinitTouchPhaseSelect, WinitWindowAttributes,
        WinitWindowLevelSelect,
    };

    // -------------------------------------------------------------------------
    // Macro: impl_winit_select_spec!
    //
    // Derives ElicitSpec for a winit Select enum using Select::labels()
    // at runtime. Also impls ElicitSpec + ElicitComplete on the trenchcoat
    // wrapper, which satisfies all remaining bounds via select_trenchcoat!.
    // -------------------------------------------------------------------------

    macro_rules! impl_winit_select_spec {
        (
            type      = $ty:ty,
            wrapper   = $wrapper:ty,
            name      = $name:literal,
            summary   = $summary:literal
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _winit_select_type_spec::<$ty>($name, $summary)
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));

            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    <$ty as ElicitSpec>::type_spec()
                }
            }

            impl ElicitComplete for $wrapper {}
        };
    }

    fn _winit_select_type_spec<T: Select>(name: &str, summary: &str) -> TypeSpec {
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
                    .description(
                        "winit v0.30 — cross-platform window creation and event loop".to_string(),
                    )
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

    // ── Select enum specs ─────────────────────────────────────────────────────

    impl_winit_select_spec!(
        type    = winit::window::WindowLevel,
        wrapper = WinitWindowLevelSelect,
        name    = "winit::window::WindowLevel",
        summary = "Window stacking level: AlwaysOnBottom, Normal, or AlwaysOnTop."
    );

    impl_winit_select_spec!(
        type    = winit::window::Theme,
        wrapper = WinitThemeSelect,
        name    = "winit::window::Theme",
        summary = "OS color theme preference: Light or Dark."
    );

    impl_winit_select_spec!(
        type    = winit::window::CursorIcon,
        wrapper = WinitCursorIconSelect,
        name    = "winit::window::CursorIcon",
        summary = "Mouse cursor shape — 36 CSS standard cursors (from cursor-icon crate)."
    );

    impl_winit_select_spec!(
        type    = winit::event::ElementState,
        wrapper = WinitElementStateSelect,
        name    = "winit::event::ElementState",
        summary = "Key or mouse-button state: Pressed or Released."
    );

    impl_winit_select_spec!(
        type    = winit::event::MouseButton,
        wrapper = WinitMouseButtonSelect,
        name    = "winit::event::MouseButton",
        summary = "Named mouse button: Left, Right, Middle, Back, or Forward \
                   (Other(u16) is excluded from the select surface)."
    );

    impl_winit_select_spec!(
        type    = winit::event::TouchPhase,
        wrapper = WinitTouchPhaseSelect,
        name    = "winit::event::TouchPhase",
        summary = "Touch event lifecycle phase: Started, Moved, Ended, or Cancelled."
    );

    impl_winit_select_spec!(
        type    = winit::keyboard::KeyCode,
        wrapper = WinitKeyCodeSelect,
        name    = "winit::keyboard::KeyCode",
        summary = "Physical keyboard scan-code — 161 unit variants covering writing keys, \
                   function keys, numpad, media, and browser keys."
    );

    // ── Struct specs ──────────────────────────────────────────────────────────

    impl ElicitSpec for WinitPhysicalSize {
        fn type_spec() -> TypeSpec {
            TypeSpecBuilder::default()
                .type_name("WinitPhysicalSize".to_string())
                .summary("Physical-pixel window size (integer width × height).".to_string())
                .categories(vec![_winit_size_fields("physical pixels")])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WinitPhysicalSize",
        <WinitPhysicalSize as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WinitPhysicalSize>
    ));

    impl ElicitComplete for WinitPhysicalSize {}

    impl ElicitSpec for WinitLogicalSize {
        fn type_spec() -> TypeSpec {
            TypeSpecBuilder::default()
                .type_name("WinitLogicalSize".to_string())
                .summary("Logical (DPI-aware) window size (f64 width × height).".to_string())
                .categories(vec![_winit_size_fields("logical pixels")])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WinitLogicalSize",
        <WinitLogicalSize as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WinitLogicalSize>
    ));

    impl ElicitComplete for WinitLogicalSize {}

    impl ElicitSpec for WinitLogicalPosition {
        fn type_spec() -> TypeSpec {
            TypeSpecBuilder::default()
                .type_name("WinitLogicalPosition".to_string())
                .summary("Logical (DPI-aware) window position (f64 x, y).".to_string())
                .categories(vec![_winit_position_fields()])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WinitLogicalPosition",
        <WinitLogicalPosition as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WinitLogicalPosition>
    ));

    impl ElicitComplete for WinitLogicalPosition {}

    impl ElicitSpec for WinitWindowAttributes {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategoryBuilder::default()
                .name("fields".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("title".to_string())
                        .description("Window title bar text.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("inner_size".to_string())
                        .description("Initial content-area size in logical pixels.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("resizable".to_string())
                        .description("Whether the user can resize the window.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("decorations".to_string())
                        .description("Whether the window has OS title bar and borders.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("transparent".to_string())
                        .description("Whether the window background is transparent.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("fullscreen".to_string())
                        .description("Start in borderless-fullscreen mode.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("window_level".to_string())
                        .description(
                            "Window stacking level (AlwaysOnBottom / Normal / AlwaysOnTop)."
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("theme".to_string())
                        .description("Preferred OS color theme (Light / Dark).".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");

            TypeSpecBuilder::default()
                .type_name("WinitWindowAttributes".to_string())
                .summary(
                    "Flat serializable window-creation config. \
                     Maps to `winit::window::WindowAttributes`."
                        .to_string(),
                )
                .categories(vec![fields])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "WinitWindowAttributes",
        <WinitWindowAttributes as ElicitSpec>::type_spec,
        std::any::TypeId::of::<WinitWindowAttributes>
    ));

    impl ElicitComplete for WinitWindowAttributes {}

    fn _winit_size_fields(unit: &str) -> crate::SpecCategory {
        SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("width".to_string())
                    .description(format!("Horizontal extent in {unit}."))
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("height".to_string())
                    .description(format!("Vertical extent in {unit}."))
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory")
    }

    fn _winit_position_fields() -> crate::SpecCategory {
        SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("x".to_string())
                    .description("Horizontal offset in logical pixels.".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("y".to_string())
                    .description("Vertical offset in logical pixels.".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory")
    }
}
