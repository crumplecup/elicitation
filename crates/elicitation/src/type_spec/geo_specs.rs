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

    // ── Spec for Vec-delegation newtypes ─────────────────────────────────

    macro_rules! impl_geo_vec_delegation_spec {
        (
            wrapper  = $wrapper:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            inner    = $inner:literal,
        ) => {
            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("items".to_string())
                                .description(format!(
                                    "Sequence of {} values (collected iteratively)",
                                    $inner
                                ))
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
                                    "geo-types v0.7 — GeoRust spatial primitives".to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description(format!(
                                    "Vec delegation — repeated {} elicitation",
                                    $inner
                                ))
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

    // ── Spec for Select enum (GeoGeometry) ───────────────────────────────

    macro_rules! impl_geo_select_spec {
        (
            wrapper  = $wrapper:ty,
            name     = $name:literal,
            summary  = $summary:literal,
            variants = [ $( $variant:literal ),+ $(,)? ]
        ) => {
            impl ElicitSpec for $wrapper {
                fn type_spec() -> TypeSpec {
                    let variant_entries: Vec<_> = vec![
                        $(
                            SpecEntryBuilder::default()
                                .label($variant.to_string())
                                .description(format!(
                                    "Select {} then elicit the inner geometry data",
                                    $variant
                                ))
                                .build()
                                .expect("valid SpecEntry"),
                        )+
                    ];

                    let variants_cat = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(variant_entries)
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
                                    "Select — choose variant, then elicit inner data".to_string(),
                                )
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");

                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants_cat, source])
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
        name = "geo_types::Coord<f64>",
        summary = "A 2D coordinate with x and y components, used for spatial positioning.",
        fields = [
            ("x", "X coordinate (longitude or horizontal position)"),
            ("y", "Y coordinate (latitude or vertical position)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoRect,
        name = "geo_types::Rect<f64>",
        summary = "An axis-aligned rectangle defined by min and max corners. \
                   Corners are normalized so min ≤ max on each axis.",
        fields = [
            ("min", "Minimum corner (GeoCoord — lower-left)"),
            ("max", "Maximum corner (GeoCoord — upper-right)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoLine,
        name = "geo_types::Line<f64>",
        summary = "A line segment defined by start and end coordinates.",
        fields = [
            ("start", "Start coordinate (GeoCoord)"),
            ("end", "End coordinate (GeoCoord)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoPoint,
        name = "geo_types::Point<f64>",
        summary = "A geographic point defined by a single 2D coordinate.",
        fields = [("coord", "The underlying coordinate (GeoCoord)"),]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoTriangle,
        name = "geo_types::Triangle<f64>",
        summary = "A triangle defined by three 2D vertices.",
        fields = [
            ("v1", "First vertex (GeoCoord)"),
            ("v2", "Second vertex (GeoCoord)"),
            ("v3", "Third vertex (GeoCoord)"),
        ]
    );

    impl_geo_composite_spec!(
        wrapper = crate::GeoPolygon,
        name = "geo_types::Polygon<f64>",
        summary = "A polygon with an exterior ring and zero or more interior rings (holes).",
        fields = [
            ("exterior", "Exterior ring (GeoLineString)"),
            ("interiors", "Interior rings / holes (Vec<GeoLineString>)"),
        ]
    );

    // ── Vec-delegation newtypes ───────────────────────────────────────────

    impl_geo_vec_delegation_spec!(
        wrapper = crate::GeoLineString,
        name = "geo_types::LineString<f64>",
        summary = "An ordered sequence of 2D coordinates forming an open or closed path.",
        inner = "GeoCoord",
    );

    impl_geo_vec_delegation_spec!(
        wrapper = crate::GeoMultiPoint,
        name = "geo_types::MultiPoint<f64>",
        summary = "An ordered collection of geographic points.",
        inner = "GeoPoint",
    );

    impl_geo_vec_delegation_spec!(
        wrapper = crate::GeoMultiLineString,
        name = "geo_types::MultiLineString<f64>",
        summary = "An ordered collection of line strings.",
        inner = "GeoLineString",
    );

    impl_geo_vec_delegation_spec!(
        wrapper = crate::GeoMultiPolygon,
        name = "geo_types::MultiPolygon<f64>",
        summary = "An ordered collection of polygons.",
        inner = "GeoPolygon",
    );

    impl_geo_vec_delegation_spec!(
        wrapper = crate::GeoGeometryCollection,
        name = "geo_types::GeometryCollection<f64>",
        summary = "A heterogeneous collection of geometry values of any concrete type.",
        inner = "GeoGeometry",
    );

    // ── Select enum (GeoGeometry) ─────────────────────────────────────────

    impl_geo_select_spec!(
        wrapper = crate::GeoGeometry,
        name = "geo_types::Geometry<f64>",
        summary = "A tagged-union enum covering every concrete geometry variant: \
                    Point, Line, LineString, Polygon, MultiPoint, MultiLineString, \
                    MultiPolygon, Rect, Triangle, GeometryCollection.",
        variants = [
            "Point",
            "Line",
            "LineString",
            "Polygon",
            "MultiPoint",
            "MultiLineString",
            "MultiPolygon",
            "Rect",
            "Triangle",
            "GeometryCollection",
        ]
    );
}
