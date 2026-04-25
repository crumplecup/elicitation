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

    use crate::{
        ElicitComplete, WktCoord, WktGeom, WktGeometryCollection, WktLineString,
        WktMultiLineString, WktMultiPoint, WktMultiPolygon, WktPoint, WktPolygon, WktString,
    };

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

    impl_builder_spec!(
        type    = WktCoord,
        name    = "WktCoord",
        summary = "A WKT coordinate with X, Y and optional Z/M values.",
        fields  = [
            ("x", "X (easting / longitude) as f64"),
            ("y", "Y (northing / latitude) as f64"),
            ("z", "Optional Z (altitude/elevation) as f64"),
            ("m", "Optional M (measure) as f64"),
        ]
    );

    impl_builder_spec!(
        type    = WktPoint,
        name    = "WktPoint",
        summary = "A WKT POINT geometry, optionally empty.",
        fields  = [
            ("coord", "Coordinate of the point, or None for an empty point"),
        ]
    );

    impl_builder_spec!(
        type    = WktLineString,
        name    = "WktLineString",
        summary = "A WKT LINESTRING geometry — an ordered sequence of coordinates.",
        fields  = [
            ("coords", "Ordered list of WktCoord values forming the line"),
        ]
    );

    impl_builder_spec!(
        type    = WktPolygon,
        name    = "WktPolygon",
        summary = "A WKT POLYGON geometry with an exterior ring and optional interior holes.",
        fields  = [
            ("exterior", "Outer boundary ring (WktLineString)"),
            ("interiors", "Interior hole rings (Vec<WktLineString>)"),
        ]
    );

    impl_builder_spec!(
        type    = WktMultiPoint,
        name    = "WktMultiPoint",
        summary = "A WKT MULTIPOINT geometry — a collection of points.",
        fields  = [
            ("points", "Collection of WktPoint values"),
        ]
    );

    impl_builder_spec!(
        type    = WktMultiLineString,
        name    = "WktMultiLineString",
        summary = "A WKT MULTILINESTRING geometry — a collection of line strings.",
        fields  = [
            ("lines", "Collection of WktLineString values"),
        ]
    );

    impl_builder_spec!(
        type    = WktMultiPolygon,
        name    = "WktMultiPolygon",
        summary = "A WKT MULTIPOLYGON geometry — a collection of polygons.",
        fields  = [
            ("polygons", "Collection of WktPolygon values"),
        ]
    );

    impl_builder_spec!(
        type    = WktGeometryCollection,
        name    = "WktGeometryCollection",
        summary = "A WKT GEOMETRYCOLLECTION — a heterogeneous collection of WKT geometries.",
        fields  = [
            ("geometries", "Collection of WktGeom values"),
        ]
    );

    impl ElicitComplete for WktGeom {}
    impl ElicitComplete for WktString {}
    impl ElicitComplete for WktCoord {}
    impl ElicitComplete for WktPoint {}
    impl ElicitComplete for WktLineString {}
    impl ElicitComplete for WktPolygon {}
    impl ElicitComplete for WktMultiPoint {}
    impl ElicitComplete for WktMultiLineString {}
    impl ElicitComplete for WktMultiPolygon {}
    impl ElicitComplete for WktGeometryCollection {}
}
