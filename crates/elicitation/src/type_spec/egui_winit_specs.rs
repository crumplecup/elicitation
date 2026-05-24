//! [`ElicitSpec`](crate::ElicitSpec) and [`ElicitComplete`](crate::ElicitComplete)
//! implementations for egui + winit descriptor types.

use crate::{
    EguiWinitDescriptor, EguiWinitRenderer, EguiWinitTheme, ElicitComplete, ElicitSpec,
    SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder, TypeSpecInventoryKey,
};

// ============================================================================
// EguiWinitRenderer
// ============================================================================

impl ElicitSpec for EguiWinitRenderer {
    fn type_spec() -> TypeSpec {
        let variants = SpecCategoryBuilder::default()
            .name("variants".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("wgpu".to_string())
                    .description(
                        "Cross-platform WebGPU backend (egui-wgpu). Recommended choice."
                            .to_string(),
                    )
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("glow".to_string())
                    .description(
                        "OpenGL via glow (egui-glow). Simpler setup, fewer dependencies."
                            .to_string(),
                    )
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description("Select — choose one GPU rendering backend".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("elicitation::EguiWinitRenderer".to_string())
            .summary(
                "GPU rendering backend for a native egui application. \
                 Wgpu is the modern cross-platform choice; Glow uses OpenGL."
                    .to_string(),
            )
            .categories(vec![variants, source])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::EguiWinitRenderer",
    <EguiWinitRenderer as ElicitSpec>::type_spec,
    std::any::TypeId::of::<EguiWinitRenderer>
));

impl ElicitComplete for EguiWinitRenderer {}

// ============================================================================
// EguiWinitTheme
// ============================================================================

impl ElicitSpec for EguiWinitTheme {
    fn type_spec() -> TypeSpec {
        let variants = SpecCategoryBuilder::default()
            .name("variants".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("dark".to_string())
                    .description("Force dark mode regardless of OS setting.".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("light".to_string())
                    .description("Force light mode regardless of OS setting.".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("system".to_string())
                    .description("Follow the OS / system preference.".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description("Select — choose one colour theme".to_string())
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("elicitation::EguiWinitTheme".to_string())
            .summary(
                "Colour theme preference for the egui context. \
                 Dark and Light force a specific theme; System follows the OS."
                    .to_string(),
            )
            .categories(vec![variants, source])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::EguiWinitTheme",
    <EguiWinitTheme as ElicitSpec>::type_spec,
    std::any::TypeId::of::<EguiWinitTheme>
));

impl ElicitComplete for EguiWinitTheme {}

// ============================================================================
// EguiWinitDescriptor
// ============================================================================

impl ElicitSpec for EguiWinitDescriptor {
    fn type_spec() -> TypeSpec {
        let fields = SpecCategoryBuilder::default()
            .name("fields".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("app_struct".to_string())
                    .description(
                        "String — application struct name (PascalCase, e.g. \"MyApp\")".to_string(),
                    )
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("title".to_string())
                    .description("String — window title shown in the OS title bar".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("width".to_string())
                    .description("u32 — initial window width in logical pixels".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("height".to_string())
                    .description("u32 — initial window height in logical pixels".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("renderer".to_string())
                    .description("EguiWinitRenderer — GPU backend (wgpu or glow)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("theme".to_string())
                    .description("EguiWinitTheme — colour theme (dark/light/system)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("vsync".to_string())
                    .description("bool — enable vertical sync (default: true)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("decorations".to_string())
                    .description("bool — show OS window decorations (default: true)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("resizable".to_string())
                    .description("bool — allow window resizing (default: true)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("maximized".to_string())
                    .description("bool — start maximised (default: false)".to_string())
                    .build()
                    .expect("valid SpecEntry"),
                SpecEntryBuilder::default()
                    .label("transparent".to_string())
                    .description(
                        "bool — transparent window background (default: false)".to_string(),
                    )
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        let source = SpecCategoryBuilder::default()
            .name("source".to_string())
            .entries(vec![
                SpecEntryBuilder::default()
                    .label("pattern".to_string())
                    .description(
                        "Survey — fill in each field to describe the application".to_string(),
                    )
                    .build()
                    .expect("valid SpecEntry"),
            ])
            .build()
            .expect("valid SpecCategory");
        TypeSpecBuilder::default()
            .type_name("elicitation::EguiWinitDescriptor".to_string())
            .summary(
                "Descriptor for a native egui + winit application. \
                 Agents build this incrementally, then call egui_winit__emit \
                 to produce a complete main.rs scaffold."
                    .to_string(),
            )
            .categories(vec![fields, source])
            .build()
            .expect("valid TypeSpec")
    }
}

inventory::submit!(TypeSpecInventoryKey::new(
    "elicitation::EguiWinitDescriptor",
    <EguiWinitDescriptor as ElicitSpec>::type_spec,
    std::any::TypeId::of::<EguiWinitDescriptor>
));

impl ElicitComplete for EguiWinitDescriptor {}
