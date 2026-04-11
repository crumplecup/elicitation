//! `elicit_georaster` — elicitation-enabled wrappers around the `georaster` reader API.
//!
//! This crate mirrors the current upstream `georaster` surface:
//! `Coordinate`, `geotiff::GeoTiffReader`, `geotiff::ImageInfo`,
//! `geotiff::Pixels`, and `geotiff::RasterValue`, plus the TIFF support enums
//! referenced by `ImageInfo`.
//!
//! # Workflow plugins
//!
//! | Plugin | Namespace | Description |
//! |---|---|---|
//! | [`GeoTiffReaderPlugin`] | `geotiff_reader__*` | Open/configure readers and inspect image metadata |
//! | [`GeoTiffSamplingPlugin`] | `geotiff_sampling__*` | Read pixels, convert coordinates, collect windows |
//!
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod color_type;
mod coordinate;
mod image_info;
mod photometric_interpretation;
mod pixels;
mod planar_configuration;
mod raster_value;
mod reader;
mod workflow;

pub use color_type::ColorType;
pub use coordinate::Coordinate;
pub use image_info::ImageInfo;
pub use photometric_interpretation::PhotometricInterpretation;
pub use pixels::Pixels;
pub use planar_configuration::PlanarConfiguration;
pub use raster_value::RasterValue;
pub use reader::{GeoRasterResult, GeoTiffReader};
pub use workflow::{
    GeoTiffReaderConfigured, GeoTiffReaderOpened, GeoTiffReaderPlugin, GeoTiffSampled,
    GeoTiffSamplingPlugin, ReaderCoordToPixelParams, ReaderImageInfoParams, ReaderImagesParams,
    ReaderOpenBytesParams, ReaderOpenPathParams, ReaderOriginParams, ReaderPixelSizeParams,
    ReaderPixelToCoordParams, ReaderPixelsParams, ReaderReadPixelAtLocationParams,
    ReaderReadPixelParams, ReaderSeekToImageParams, ReaderSelectRasterBandParams,
};
