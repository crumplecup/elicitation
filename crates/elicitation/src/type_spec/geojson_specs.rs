//! [`ElicitSpec`](crate::ElicitSpec) implementations for GeoJSON type elicitation.
//!
//! Available with the `geojson-types` feature.

#[cfg(feature = "geojson-types")]
mod geojson_impls {
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
                                .description("geojson v0.24 — GeoJSON RFC 7946 document format".to_string())
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
                                .description("geojson v0.24 — GeoJSON RFC 7946 document format".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — structured document elicited field by field".to_string())
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

    impl_select_spec!(
        type    = geojson::GeoJson,
        name    = "geojson::GeoJson",
        summary = "A top-level GeoJSON document — either a Geometry, Feature, or FeatureCollection.",
        variants = [
            ("Geometry", "A bare geometry document"),
            ("Feature", "A feature document containing optional geometry and properties"),
            ("FeatureCollection", "A collection of GeoJSON features"),
        ]
    );

    impl_select_spec!(
        type    = geojson::GeometryValue,
        name    = "geojson::Value",
        summary = "A GeoJSON geometry value — the payload inside a Geometry object.",
        variants = [
            ("Point", "A single coordinate position [longitude, latitude, (altitude)]"),
            ("MultiPoint", "A collection of point positions"),
            ("LineString", "An ordered sequence of coordinate positions"),
            ("MultiLineString", "A collection of LineString coordinate arrays"),
            ("Polygon", "A surface bounded by a linear ring plus optional holes"),
            ("MultiPolygon", "A collection of Polygon coordinate arrays"),
            ("GeometryCollection", "An ordered collection of Geometry objects"),
        ]
    );

    impl_select_spec!(
        type    = geojson::feature::Id,
        name    = "geojson::feature::Id",
        summary = "A GeoJSON feature identifier represented as either a string or a JSON number.",
        variants = [
            ("String", "A string-valued feature identifier"),
            ("Number", "A numeric feature identifier"),
        ]
    );

    impl_builder_spec!(
        type    = geojson::Geometry,
        name    = "geojson::Geometry",
        summary = "A GeoJSON geometry document with a geometry value plus optional bbox and foreign members.",
        fields  = [
            ("value", "The core geometry value (Point, Polygon, GeometryCollection, etc.)"),
            ("bbox", "Optional bounding box coordinates"),
            ("foreign_members", "Optional non-standard JSON object members"),
        ]
    );

    impl_builder_spec!(
        type    = geojson::Feature,
        name    = "geojson::Feature",
        summary = "A GeoJSON feature with optional geometry, id, properties, bbox, and foreign members.",
        fields  = [
            ("geometry", "Optional geometry payload"),
            ("id", "Optional string or numeric feature identifier"),
            ("properties", "Optional JSON object of feature properties"),
            ("bbox", "Optional bounding box coordinates"),
            ("foreign_members", "Optional non-standard JSON object members"),
        ]
    );

    impl_builder_spec!(
        type    = geojson::FeatureCollection,
        name    = "geojson::FeatureCollection",
        summary = "A collection of GeoJSON features with optional bbox and foreign members.",
        fields  = [
            ("features", "The ordered list of GeoJSON features"),
            ("bbox", "Optional bounding box coordinates"),
            ("foreign_members", "Optional non-standard JSON object members"),
        ]
    );
}
