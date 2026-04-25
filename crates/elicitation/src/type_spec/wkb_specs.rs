//! [`ElicitSpec`](crate::ElicitSpec) implementations for WKB type elicitation.
//!
//! Available with the `wkb-types` feature.

#[cfg(feature = "wkb-types")]
mod wkb_impls {
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
                                .description("wkb 0.9.x — Well-Known Binary reader/writer support".to_string())
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
                                .description("wkb 0.9.x — Well-Known Binary reader/writer support".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — structured builder type elicited field by field".to_string())
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

    use crate::{ElicitComplete, WkbBytes, WkbDimension, WkbEndianness, WkbGeometryType, WkbWriteOptions};

    impl_select_spec!(
        type    = WkbEndianness,
        name    = "WkbEndianness",
        summary = "Byte order used by WKB payloads and write options.",
        variants = [
            ("BigEndian", "Big-endian byte order"),
            ("LittleEndian", "Little-endian byte order"),
        ]
    );

    impl_select_spec!(
        type    = WkbDimension,
        name    = "WkbDimension",
        summary = "Coordinate dimensionality reported by the WKB reader.",
        variants = [
            ("Xy", "2D coordinates: X and Y"),
            ("Xyz", "3D coordinates: X, Y, and Z"),
            ("Xym", "Measured coordinates: X, Y, and M"),
            ("Xyzm", "4D coordinates: X, Y, Z, and M"),
        ]
    );

    impl_select_spec!(
        type    = WkbGeometryType,
        name    = "WkbGeometryType",
        summary = "Geometry kind reported by the WKB reader.",
        variants = [
            ("Point", "A WKB point geometry"),
            ("LineString", "A WKB line string geometry"),
            ("Polygon", "A WKB polygon geometry"),
            ("MultiPoint", "A WKB multi-point geometry"),
            ("MultiLineString", "A WKB multi-line string geometry"),
            ("MultiPolygon", "A WKB multi-polygon geometry"),
            ("GeometryCollection", "A WKB geometry collection"),
        ]
    );

    impl_builder_spec!(
        type    = WkbBytes,
        name    = "WkbBytes",
        summary = "A validated owned WKB payload stored as raw bytes.",
        fields  = [
            ("bytes", "Validated WKB bytes, typically supplied as a hex string during elicitation"),
        ]
    );

    impl_builder_spec!(
        type    = WkbWriteOptions,
        name    = "WkbWriteOptions",
        summary = "Write configuration for WKB output.",
        fields  = [
            ("endianness", "Byte order to use when writing WKB bytes"),
        ]
    );

    impl ElicitComplete for WkbEndianness {}
    impl ElicitComplete for WkbBytes {}
    impl ElicitComplete for WkbWriteOptions {}
}
