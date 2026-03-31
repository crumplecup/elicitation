//! [`ElicitSpec`](crate::ElicitSpec) implementations for geo-types elicitation.
//!
//! Available with the `geo-types` feature.
//!
//! Complements the [`ElicitIntrospect`](crate::ElicitIntrospect) impls in
//! `primitives/geo_types/` — those describe *structure* (pattern, fields),
//! these describe *contracts and usage* browsable by agents via `describe_type`.

#[cfg(feature = "geo-types")]
mod geo_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    // ── Composite spec for survey wrappers ───────────────────────────────

    macro_rules! impl_geo_composite_spec {
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
                                    "geo-types v0.7 — GeoRust spatial primitives".to_string(),
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

    // ── Composite struct wrappers (Survey pattern) ───────────────────────

    impl_geo_composite_spec!(
        wrapper = crate::GeoCoord,
        name    = "geo_types::Coord<f64>",
        summary = "A 2D coordinate with x and y components, used for spatial positioning.",
        fields  = [
            ("x", "X coordinate (longitude or horizontal position)"),
            ("y", "Y coordinate (latitude or vertical position)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoRect,
        name    = "geo_types::Rect<f64>",
        summary = "An axis-aligned rectangle defined by min and max corners. \
                   Corners are normalized so min ≤ max on each axis.",
        fields  = [
            ("min", "Minimum corner (GeoCoord — lower-left)"),
            ("max", "Maximum corner (GeoCoord — upper-right)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoLine,
        name    = "geo_types::Line<f64>",
        summary = "A line segment defined by start and end coordinates.",
        fields  = [
            ("start", "Start coordinate (GeoCoord)"),
            ("end", "End coordinate (GeoCoord)"),
        ]
    );
}
