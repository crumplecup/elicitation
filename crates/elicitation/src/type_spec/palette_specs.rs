//! [`ElicitSpec`](crate::ElicitSpec) implementations for palette elicitation.
//!
//! Available with the `palette` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/palette_types/` — those describe *structure* (pattern, fields),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "palette")]
mod palette_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // ── Composite spec for survey wrappers ───────────────────────────────

    macro_rules! impl_palette_composite_spec {
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
                                    "palette v0.7 — Color science and conversion".to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description(
                                    "Survey — elicit each channel in sequence".to_string(),
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

    // ── Composite struct wrappers (Survey pattern) ───────────────────────

    impl_palette_composite_spec!(
        wrapper = crate::PaletteSrgb,
        name    = "palette::Srgb<f32>",
        summary = "An RGB color in sRGB color space with floating-point channels (0.0–1.0).",
        fields  = [
            ("r", "Red channel intensity (0.0 = dark, 1.0 = bright)"),
            ("g", "Green channel intensity (0.0 = dark, 1.0 = bright)"),
            ("b", "Blue channel intensity (0.0 = dark, 1.0 = bright)"),
        ]
    );
}
