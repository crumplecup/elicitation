//! [`ElicitSpec`](crate::ElicitSpec) implementations for wkt type elicitation.
//!
//! Available with the `wkt-types` feature.

#[cfg(feature = "wkt-types")]
mod wkt_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
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
                                .description("wkt v0.11 — Well-Known Text spatial format".to_string())
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
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }

            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    macro_rules! impl_builder_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
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
                                .description("wkt v0.11 — Well-Known Text spatial format".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description(
                                    "Survey — structured builder type elicited field by field"
                                        .to_string(),
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
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    use crate::{WktGeom, WktString};

    impl_select_spec!(
        type    = WktGeom,
        name    = "WktGeom",
        summary = "A WKT geometry value — one of Point, LineString, Polygon, MultiPoint, MultiLineString, MultiPolygon, or GeometryCollection.",
        variants = [
            ("Point", "A single WKT point, optionally with x/y/z/m coordinates"),
            ("LineString", "An ordered sequence of WKT coordinates forming a line"),
            ("Polygon", "A WKT polygon with an exterior ring and optional holes"),
            ("MultiPoint", "A collection of WKT points"),
            ("MultiLineString", "A collection of WKT line strings"),
            ("MultiPolygon", "A collection of WKT polygons"),
            ("GeometryCollection", "A heterogeneous collection of WKT geometries"),
        ]
    );

    impl_builder_spec!(
        type    = WktString,
        name    = "WktString",
        summary = "A validated raw WKT string (e.g. POINT(1 2) or LINESTRING(0 0, 1 1)).",
        fields  = [
            ("wkt", "The raw WKT geometry string, validated by wkt::Wkt::<f64>::from_str"),
        ]
    );
}
