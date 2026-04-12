//! [`ElicitSpec`](crate::ElicitSpec) implementations for concrete `proj` types.
//!
//! Available with the `proj-types` feature.

#[cfg(feature = "proj-types")]
mod proj_impls {
    use crate::{
        ElicitComplete, ElicitSpec, ProjArea, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec,
        TypeSpecBuilder, TypeSpecInventoryKey,
    };

    impl ElicitSpec for ProjArea {
        fn type_spec() -> TypeSpec {
            let fields = SpecCategoryBuilder::default()
                .name("fields".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("west".to_string())
                        .description(
                            "Western boundary of the area of use. For antimeridian-crossing boxes, west may be greater than east."
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("south".to_string())
                        .description("Southern boundary of the area of use.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("east".to_string())
                        .description("Eastern boundary of the area of use.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("north".to_string())
                        .description("Northern boundary of the area of use.".to_string())
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            let source = SpecCategoryBuilder::default()
                .name("source".to_string())
                .entries(vec![
                    SpecEntryBuilder::default()
                        .label("crate".to_string())
                        .description(
                            "proj 0.31.x — area-of-use bounding box for coordinate transforms"
                                .to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                    SpecEntryBuilder::default()
                        .label("pattern".to_string())
                        .description(
                            "Survey — structured value elicited field by field".to_string(),
                        )
                        .build()
                        .expect("valid SpecEntry"),
                ])
                .build()
                .expect("valid SpecCategory");
            TypeSpecBuilder::default()
                .type_name("proj::Area".to_string())
                .summary(
                    "The bounding box of a PROJ area of use, expressed as west/south/east/north limits."
                        .to_string(),
                )
                .categories(vec![fields, source])
                .build()
                .expect("valid TypeSpec")
        }
    }

    inventory::submit!(TypeSpecInventoryKey::new(
        "proj::Area",
        <ProjArea as ElicitSpec>::type_spec,
        std::any::TypeId::of::<ProjArea>
    ));

    impl ElicitComplete for ProjArea {}
}
