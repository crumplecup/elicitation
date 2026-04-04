//! [`ElicitSpec`](crate::ElicitSpec) implementations for ratatui type elicitation.
//!
//! Available with the `ratatui` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/ratatui_types/` — those describe *structure* (pattern, variants),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "ratatui")]
mod ratatui_impls {
    use crate::{
        ElicitComplete, ElicitSpec, Select, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // ── Select spec for trenchcoat wrappers ──────────────────────────────

    macro_rules! impl_ratatui_wrapper_spec {
        (
            type    = $ty:ty,
            wrapper = $wrapper:ty,
            name    = $name:literal,
            summary = $summary:literal
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    _ratatui_select_spec::<$ty>($name, $summary)
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

    // ── Composite spec for survey wrappers ───────────────────────────────

    macro_rules! impl_ratatui_composite_spec {
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
                                    "ratatui v0.30 — terminal UI library for Rust".to_string(),
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

    fn _ratatui_select_spec<T: Select>(name: &str, summary: &str) -> TypeSpec {
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
                    .description("ratatui v0.30 — terminal UI library for Rust".to_string())
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

    // ── Direct select enums (impl Select on ratatui type, wrapper for ElicitComplete) ─

    impl_ratatui_wrapper_spec!(
        type    = ratatui::layout::Alignment,
        wrapper = crate::AlignmentSelect,
        name    = "ratatui::layout::Alignment",
        summary = "Text alignment: Left, Center, or Right."
    );

    impl_ratatui_wrapper_spec!(
        type    = ratatui::layout::Direction,
        wrapper = crate::RatatuiDirectionSelect,
        name    = "ratatui::layout::Direction",
        summary = "Layout direction: Horizontal or Vertical."
    );

    impl_ratatui_wrapper_spec!(
        type    = ratatui::widgets::BorderType,
        wrapper = crate::BorderTypeSelect,
        name    = "ratatui::widgets::BorderType",
        summary = "Border drawing style: Plain, Rounded, Double, or Thick."
    );

    impl_ratatui_wrapper_spec!(
        type    = ratatui::style::Color,
        wrapper = crate::ColorSelect,
        name    = "ratatui::style::Color",
        summary = "Terminal colour — 17 named presets, 256-colour palette, or 24-bit RGB."
    );

    // ── Trenchcoat wrappers (Select on wrapper, ElicitComplete on wrapper) ─

    impl_ratatui_wrapper_spec!(
        type    = ratatui::widgets::Borders,
        wrapper = crate::BordersSelect,
        name    = "ratatui::widgets::Borders",
        summary = "Which borders to draw on a Block: None, All, Top, Bottom, Left, Right, \
                   or common combinations."
    );

    impl_ratatui_wrapper_spec!(
        type    = ratatui::widgets::ScrollbarOrientation,
        wrapper = crate::ScrollbarOrientationSelect,
        name    = "ratatui::widgets::ScrollbarOrientation",
        summary = "Scrollbar placement: VerticalRight, VerticalLeft, \
                   HorizontalBottom, or HorizontalTop."
    );

    // ── Composite struct wrappers (Survey pattern) ───────────────────────

    impl_ratatui_composite_spec!(
        wrapper = crate::RatatuiStyle,
        name = "ratatui::style::Style",
        summary = "Terminal text style — foreground/background colour and modifiers.",
        fields = [
            ("fg", "Foreground colour (e.g. Red, #FF00AA)"),
            ("bg", "Background colour"),
            ("bold", "Apply bold modifier"),
            ("italic", "Apply italic modifier"),
            ("underlined", "Apply underline modifier"),
        ]
    );

    impl_ratatui_composite_spec!(
        wrapper = crate::RatatuiPadding,
        name = "ratatui::widgets::Padding",
        summary = "Inner padding for a Block widget (left, right, top, bottom).",
        fields = [
            ("left", "Left padding in cells"),
            ("right", "Right padding in cells"),
            ("top", "Top padding in cells"),
            ("bottom", "Bottom padding in cells"),
        ]
    );

    impl_ratatui_composite_spec!(
        wrapper = crate::RatatuiMargin,
        name = "ratatui::layout::Margin",
        summary = "Outer margin for layout — horizontal and vertical spacing.",
        fields = [
            ("horizontal", "Left + right margin in cells"),
            ("vertical", "Top + bottom margin in cells"),
        ]
    );
}
