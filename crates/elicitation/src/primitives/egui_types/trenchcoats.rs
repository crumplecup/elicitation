//! Select-trenchcoat wrappers for egui types.
//!
//! Each wrapper adds `Serialize`, `Deserialize`, and `JsonSchema` to
//! the corresponding egui enum, enabling [`ElicitComplete`](crate::ElicitComplete).
//!
//! All types except [`UiKindSelect`] use the `serde` variant (transparent
//! delegation to egui's own serde impls). `UiKind` lacks serde in egui,
//! so its wrapper uses manual label-based serialization.

// ── Types with serde (transparent delegation) ───────────────────────────
crate::select_trenchcoat!(egui::Align, as AlignSelect, serde);
crate::select_trenchcoat_traits!(AlignSelect, egui::Align, [copy, eq, hash]);

crate::select_trenchcoat!(egui::CursorIcon, as CursorIconSelect, serde);
crate::select_trenchcoat_traits!(CursorIconSelect, egui::CursorIcon, [copy, eq]);

crate::select_trenchcoat!(egui::Direction, as DirectionSelect, serde);
crate::select_trenchcoat_traits!(DirectionSelect, egui::Direction, [copy, eq]);

crate::select_trenchcoat!(egui::FontFamily, as FontFamilySelect, serde);
crate::select_trenchcoat_traits!(FontFamilySelect, egui::FontFamily, [eq, hash]);

crate::select_trenchcoat!(egui::Key, as KeySelect, serde);
crate::select_trenchcoat_traits!(KeySelect, egui::Key, [copy, eq, hash]);

crate::select_trenchcoat!(egui::Order, as OrderSelect, serde);
crate::select_trenchcoat_traits!(OrderSelect, egui::Order, [copy, eq, hash]);

crate::select_trenchcoat!(egui::PointerButton, as PointerButtonSelect, serde);
crate::select_trenchcoat_traits!(PointerButtonSelect, egui::PointerButton, [copy, eq]);

crate::select_trenchcoat!(egui::TextStyle, as TextStyleSelect, serde);
crate::select_trenchcoat_traits!(TextStyleSelect, egui::TextStyle, [eq, hash]);

crate::select_trenchcoat!(egui::TextWrapMode, as TextWrapModeSelect, serde);
crate::select_trenchcoat_traits!(TextWrapModeSelect, egui::TextWrapMode, [copy, eq]);

crate::select_trenchcoat!(egui::epaint::textures::TextureFilter, as TextureFilterSelect, serde);
crate::select_trenchcoat_traits!(
    TextureFilterSelect,
    egui::epaint::textures::TextureFilter,
    [copy, eq, hash]
);

crate::select_trenchcoat!(egui::epaint::textures::TextureWrapMode, as TextureWrapModeSelect, serde);
crate::select_trenchcoat_traits!(
    TextureWrapModeSelect,
    egui::epaint::textures::TextureWrapMode,
    [copy, eq, hash]
);

crate::select_trenchcoat!(egui::Theme, as ThemeSelect, serde);
crate::select_trenchcoat_traits!(ThemeSelect, egui::Theme, [copy, eq, hash]);

crate::select_trenchcoat!(egui::ThemePreference, as ThemePreferenceSelect, serde);
crate::select_trenchcoat_traits!(
    ThemePreferenceSelect,
    egui::ThemePreference,
    [copy, eq, hash]
);

crate::select_trenchcoat!(egui::TouchPhase, as TouchPhaseSelect, serde);
crate::select_trenchcoat_traits!(TouchPhaseSelect, egui::TouchPhase, [copy, eq]);

crate::select_trenchcoat!(egui::WidgetType, as WidgetTypeSelect, serde);
crate::select_trenchcoat_traits!(WidgetTypeSelect, egui::WidgetType, [copy, eq]);

// ── Types without serde (manual label-based serialization) ──────────────
crate::select_trenchcoat!(egui::UiKind, as UiKindSelect);
crate::select_trenchcoat_traits!(UiKindSelect, egui::UiKind, [copy, eq]);
