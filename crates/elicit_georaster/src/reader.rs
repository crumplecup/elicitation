//! `GeoTiffReader` — owned wrapper around the upstream generic GeoTIFF reader.

use crate::{Coordinate, ImageInfo, Pixels, RasterValue};
use schemars::{JsonSchema, SchemaGenerator};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    fmt,
    io::{Cursor, Read, Seek},
    path::Path,
    sync::{Arc, Mutex},
};

/// Result type for fallible GeoTIFF reader operations.
pub type GeoRasterResult<T> = Result<T, tiff::TiffError>;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    elicitation_derive::ToCodeLiteral,
)]
struct GeoTiffReaderSnapshot {
    bytes: Vec<u8>,
    current_image_idx: usize,
    selected_band: u8,
}

/// Owned wrapper around [`georaster::geotiff::GeoTiffReader`].
#[derive(Clone)]
pub struct GeoTiffReader {
    snapshot: Arc<Mutex<GeoTiffReaderSnapshot>>,
}

impl fmt::Debug for GeoTiffReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GeoTiffReader")
            .field("snapshot", &self.snapshot())
            .finish()
    }
}

impl Serialize for GeoTiffReader {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.snapshot().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for GeoTiffReader {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let snapshot = GeoTiffReaderSnapshot::deserialize(deserializer)?;
        Self::from_snapshot(snapshot).map_err(serde::de::Error::custom)
    }
}

impl JsonSchema for GeoTiffReader {
    fn schema_name() -> Cow<'static, str> {
        Cow::Borrowed("GeoTiffReader")
    }

    fn json_schema(generator: &mut SchemaGenerator) -> schemars::Schema {
        <GeoTiffReaderSnapshot as JsonSchema>::json_schema(generator)
    }
}

impl GeoTiffReader {
    /// Open a GeoTIFF reader from any readable+seekable byte source.
    pub fn open<R>(mut src: R) -> GeoRasterResult<Self>
    where
        R: Read + Seek,
    {
        let mut bytes = Vec::new();
        src.read_to_end(&mut bytes)
            .map_err(tiff::TiffError::IoError)?;
        Self::from_bytes(bytes)
    }

    /// Open a GeoTIFF reader from owned bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> GeoRasterResult<Self> {
        let reader = georaster::geotiff::GeoTiffReader::open(Cursor::new(bytes.clone()))?;
        let current_image_idx = reader.images().len().saturating_sub(1);
        Self::from_snapshot(GeoTiffReaderSnapshot {
            bytes,
            current_image_idx,
            selected_band: 1,
        })
    }

    /// Open a GeoTIFF reader from a filesystem path.
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(bytes).map_err(|error| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string())
        })
    }

    /// Returns metadata for all images/IFDs in the file.
    pub fn images(&self) -> Vec<ImageInfo> {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        reader.images().iter().map(ImageInfo::from).collect()
    }

    /// Returns metadata for the current image/IFD.
    pub fn image_info(&self) -> ImageInfo {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        ImageInfo::from(reader.image_info())
    }

    /// Change the current image/IFD.
    pub fn seek_to_image(&mut self, index: usize) -> GeoRasterResult<()> {
        let snapshot = self.snapshot();
        let mut reader = Self::build_reader_from_snapshot(&snapshot)?;
        reader.seek_to_image(index)?;
        self.replace_snapshot(GeoTiffReaderSnapshot {
            current_image_idx: index,
            ..snapshot
        });
        Ok(())
    }

    /// Returns the raster origin if geoinformation is available.
    pub fn origin(&self) -> Option<[f64; 2]> {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        reader.origin()
    }

    /// Returns pixel size if geoinformation is available.
    pub fn pixel_size(&self) -> Option<[f64; 2]> {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        reader.pixel_size()
    }

    /// Select the raster band to sample (1-based, matching upstream semantics).
    pub fn select_raster_band(&mut self, band: u8) -> GeoRasterResult<()> {
        let snapshot = self.snapshot();
        let mut reader = Self::build_reader_from_snapshot(&snapshot)?;
        reader.select_raster_band(band)?;
        self.replace_snapshot(GeoTiffReaderSnapshot {
            selected_band: band,
            ..snapshot
        });
        Ok(())
    }

    /// Read a pixel value at integer x/y offsets.
    pub fn read_pixel(&self, x: u32, y: u32) -> RasterValue {
        let mut reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        RasterValue::from(reader.read_pixel(x, y))
    }

    /// Read a pixel value at a geographic location.
    pub fn read_pixel_at_location(&self, coord: impl Into<Coordinate>) -> RasterValue {
        let mut reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        let coord: georaster::Coordinate = coord.into().into();
        RasterValue::from(reader.read_pixel_at_location(coord))
    }

    /// Collect the upstream pixel iterator into an owned wrapper.
    pub fn pixels(&self, x: u32, y: u32, width: u32, height: u32) -> Pixels {
        let mut reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        Pixels {
            items: reader
                .pixels(x, y, width, height)
                .map(|(x, y, value)| (x, y, RasterValue::from(value)))
                .collect(),
        }
    }

    /// Convert a geographic coordinate into a pixel coordinate, if possible.
    pub fn coord_to_pixel(&self, coord: impl Into<Coordinate>) -> Option<(u32, u32)> {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        let coord: georaster::Coordinate = coord.into().into();
        reader.coord_to_pixel(coord)
    }

    /// Convert a pixel coordinate into a geographic coordinate, if possible.
    pub fn pixel_to_coord(&self, x: u32, y: u32) -> Option<Coordinate> {
        let reader = self
            .build_reader()
            .expect("GeoTiffReader snapshot should always reconstruct");
        reader.pixel_to_coord(x, y).map(Coordinate::from)
    }

    fn from_snapshot(snapshot: GeoTiffReaderSnapshot) -> GeoRasterResult<Self> {
        Self::build_reader_from_snapshot(&snapshot)?;
        Ok(Self {
            snapshot: Arc::new(Mutex::new(snapshot)),
        })
    }

    fn snapshot(&self) -> GeoTiffReaderSnapshot {
        self.snapshot
            .lock()
            .expect("GeoTiffReader snapshot lock poisoned")
            .clone()
    }

    fn replace_snapshot(&self, snapshot: GeoTiffReaderSnapshot) {
        *self
            .snapshot
            .lock()
            .expect("GeoTiffReader snapshot lock poisoned") = snapshot;
    }

    fn build_reader(&self) -> GeoRasterResult<georaster::geotiff::GeoTiffReader<Cursor<Vec<u8>>>> {
        Self::build_reader_from_snapshot(&self.snapshot())
    }

    fn build_reader_from_snapshot(
        snapshot: &GeoTiffReaderSnapshot,
    ) -> GeoRasterResult<georaster::geotiff::GeoTiffReader<Cursor<Vec<u8>>>> {
        let mut reader =
            georaster::geotiff::GeoTiffReader::open(Cursor::new(snapshot.bytes.clone()))?;
        if snapshot.current_image_idx != reader.images().len().saturating_sub(1) {
            reader.seek_to_image(snapshot.current_image_idx)?;
        }
        if snapshot.selected_band != 1 {
            reader.select_raster_band(snapshot.selected_band)?;
        }
        Ok(reader)
    }
}

impl elicitation::emit_code::ToCodeLiteral for GeoTiffReader {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let snapshot = self.snapshot();
        let bytes = snapshot.bytes;
        let image_index = snapshot.current_image_idx;
        let band = snapshot.selected_band;
        quote::quote! {{
            let mut reader = ::elicit_georaster::GeoTiffReader::from_bytes(vec![#(#bytes),*])
                .expect("valid GeoTIFF bytes");
            reader
                .seek_to_image(#image_index)
                .expect("valid GeoTIFF image index");
            reader
                .select_raster_band(#band)
                .expect("valid GeoTIFF band");
            reader
        }}
    }
}
