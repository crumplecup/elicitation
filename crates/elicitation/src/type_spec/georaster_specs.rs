//! [`ElicitSpec`](crate::ElicitSpec) implementations for georaster type elicitation.
//!
//! Available with the `georaster-types` feature.

#[cfg(feature = "georaster-types")]
mod georaster_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    macro_rules! impl_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            source  = $source:literal,
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
                                .description($source.to_string())
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
            source  = $source:literal,
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
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    impl_builder_spec!(
        type    = georaster::Coordinate,
        name    = "georaster::Coordinate",
        summary = "A 2D coordinate used by georaster for geographic sampling and pixel conversion.",
        source  = "georaster 0.2.x — GeoTIFF/COG reader coordinate type",
        fields  = [
            ("x", "Longitude / X coordinate"),
            ("y", "Latitude / Y coordinate"),
        ]
    );

    impl_select_spec!(
        type    = tiff::tags::PlanarConfiguration,
        name    = "tiff::tags::PlanarConfiguration",
        summary = "How TIFF samples are arranged across image storage.",
        source  = "tiff 0.9.x — TIFF tag vocabulary exposed through georaster::geotiff::ImageInfo",
        variants = [
            ("Chunky", "All samples for a pixel are stored together"),
            ("Planar", "Samples are stored in separate planes"),
        ]
    );

    impl_select_spec!(
        type    = tiff::tags::PhotometricInterpretation,
        name    = "tiff::tags::PhotometricInterpretation",
        summary = "The TIFF photometric interpretation describing how pixel samples map to colors.",
        source  = "tiff 0.9.x — TIFF tag vocabulary exposed through georaster::geotiff::ImageInfo",
        variants = [
            ("WhiteIsZero", "Zero means white for grayscale imagery"),
            ("BlackIsZero", "Zero means black for grayscale imagery"),
            ("RGB", "Samples represent RGB color channels"),
            ("RGBPalette", "Samples index into a palette"),
            ("TransparencyMask", "Samples represent a transparency mask"),
            ("CMYK", "Samples represent CMYK channels"),
            ("YCbCr", "Samples represent YCbCr channels"),
            ("CIELab", "Samples represent CIELab channels"),
        ]
    );

    impl_select_spec!(
        type    = tiff::ColorType,
        name    = "tiff::ColorType",
        summary = "A TIFF color model paired with bit depth per channel.",
        source  = "tiff 0.9.x — TIFF color type vocabulary exposed through georaster::geotiff::ImageInfo",
        variants = [
            ("Gray", "Grayscale pixels with a bit depth"),
            ("RGB", "RGB pixels with a bit depth"),
            ("Palette", "Palette-indexed pixels with a bit depth"),
            ("GrayA", "Grayscale plus alpha with a bit depth"),
            ("RGBA", "RGBA pixels with a bit depth"),
            ("CMYK", "CMYK pixels with a bit depth"),
            ("YCbCr", "YCbCr pixels with a bit depth"),
        ]
    );

    impl_select_spec!(
        type    = georaster::geotiff::RasterValue,
        name    = "georaster::geotiff::RasterValue",
        summary = "A pixel value returned by georaster's GeoTIFF reader, covering scalar and RGB/RGBA payloads.",
        source  = "georaster 0.2.x — GeoTIFF/COG reader pixel value enum",
        variants = [
            ("NoData", "No data available at the requested location"),
            ("U8", "Unsigned 8-bit scalar pixel"),
            ("U16", "Unsigned 16-bit scalar pixel"),
            ("U32", "Unsigned 32-bit scalar pixel"),
            ("U64", "Unsigned 64-bit scalar pixel"),
            ("F32", "32-bit floating-point scalar pixel"),
            ("F64", "64-bit floating-point scalar pixel"),
            ("I8", "Signed 8-bit scalar pixel"),
            ("I16", "Signed 16-bit scalar pixel"),
            ("I32", "Signed 32-bit scalar pixel"),
            ("I64", "Signed 64-bit scalar pixel"),
            ("Rgb8", "8-bit RGB pixel"),
            ("Rgba8", "8-bit RGBA pixel"),
            ("Rgb16", "16-bit RGB pixel"),
            ("Rgba16", "16-bit RGBA pixel"),
        ]
    );

    impl_builder_spec!(
        type    = georaster::geotiff::ImageInfo,
        name    = "georaster::geotiff::ImageInfo",
        summary = "Metadata for a TIFF image/IFD as exposed by georaster's GeoTIFF reader.",
        source  = "georaster 0.2.x — GeoTIFF/COG reader image metadata struct",
        fields  = [
            ("dimensions", "Optional image width and height"),
            ("colortype", "Optional TIFF color type and bit depth"),
            ("photometric_interpretation", "Optional TIFF photometric interpretation"),
            ("planar_config", "Optional TIFF planar configuration"),
            ("samples", "Number of samples per pixel"),
        ]
    );
}
