//! [`ElicitSpec`](crate::ElicitSpec) implementations for concrete `rstar` types.
//!
//! Available with the `rstar-types` feature.

#[cfg(feature = "rstar-types")]
mod rstar_impls {
    use crate::{
        ElicitComplete, ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    macro_rules! impl_builder_spec {
        (
            wrapper = $wrapper:ty,
            name    = $name:literal,
            summary = $summary:literal,
            source  = $source:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description($source.to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — structured value elicited field by field".to_string())
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

    impl_builder_spec!(
        wrapper = crate::RstarAabb,
        name = "rstar::AABB<[f64; 2]>",
        summary =
            "A 2D axis-aligned bounding box used as the envelope type for rstar spatial objects.",
        source = "rstar 0.12.x — envelope primitive for RTreeObject implementations",
        fields = [
            ("lower", "Lower corner of the bounding box as a 2D point"),
            ("upper", "Upper corner of the bounding box as a 2D point"),
        ]
    );

    impl_builder_spec!(
        wrapper = crate::RstarRectangle,
        name = "rstar::primitives::Rectangle<[f64; 2]>",
        summary = "A 2D rectangle primitive that can be inserted directly into an rstar R-tree.",
        source = "rstar 0.12.x — insertable rectangle primitive",
        fields = [
            ("lower", "Lower corner of the rectangle as a 2D point"),
            ("upper", "Upper corner of the rectangle as a 2D point"),
        ]
    );

    impl_builder_spec!(
        wrapper = crate::RstarLine,
        name = "rstar::primitives::Line<[f64; 2]>",
        summary = "A 2D line segment primitive that supports envelope and point-distance queries in rstar.",
        source = "rstar 0.12.x — insertable line primitive with PointDistance support",
        fields = [
            ("from", "Start point of the line as a 2D point"),
            ("to", "End point of the line as a 2D point"),
        ]
    );
}
