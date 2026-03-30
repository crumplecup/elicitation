# ELICIT_GEORASTER_PLAN.md

## Goal

Add comprehensive raster data support to elicitation for geospatial grid operations:

1. **Core type integration** — georaster types in `elicitation` with feature gating
2. **Shadow crate** — `elicit_georaster` with MCP tools for raster operations (~40 tools)
3. **GIS alphabet** — Foundation for raster data processing and analysis

## Use Cases

- **Elevation data**: Digital elevation models (DEM), terrain analysis
- **Satellite imagery**: Landsat, Sentinel, aerial photography processing
- **Climate data**: Temperature, precipitation, climate model grids
- **Land cover**: Classification rasters (forest, urban, water, etc.)
- **Raster algebra**: Mathematical operations on grids
- **Terrain analysis**: Slope, aspect, hillshade, viewshed
- **Interpolation**: Point data → continuous surfaces
- **Resampling**: Change resolution, reproject rasters
- **Multi-band**: RGB imagery, multispectral analysis
- **NoData handling**: Missing data, masking

## Raster Data Concepts

```
Raster Grid Structure:
┌─────────────────────────┐
│ [11] [12] [13] [14] ... │  ← Row 0 (northernmost)
│ [21] [22] [23] [24] ... │  ← Row 1
│ [31] [32] [33] [34] ... │  ← Row 2
│  ...  ...  ...  ... ... │
└─────────────────────────┘
  ↑
  Column 0 (westernmost)

Geotransform (6 parameters):
[0] x-coordinate of upper-left corner
[1] pixel width (west-east resolution)
[2] row rotation (usually 0)
[3] y-coordinate of upper-left corner
[4] column rotation (usually 0)
[5] pixel height (north-south resolution, usually negative)

Convert pixel (row, col) → geo (x, y):
x = geotransform[0] + col * geotransform[1] + row * geotransform[2]
y = geotransform[3] + col * geotransform[4] + row * geotransform[5]
```

## Architecture Overview

Following established patterns from elicit_geo, elicit_proj:
- **Core**: Feature-gated georaster types with Elicitation impls
- **Shadow crate**: 7 workflow plugins covering ~40 operations
- **GIS alphabet**: Raster data processing for spatial analysis

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add georaster to workspace dependencies** (`Cargo.toml`):
```toml
# Geospatial - Raster Operations
georaster = { version = "0.1", default-features = false }
ndarray = { version = "0.16", default-features = false }  # Array operations
```

**1.2 Add elicit_georaster member** (`Cargo.toml` after other elicit_* members):
```toml
  "crates/elicit_georaster",
```

**1.3 Add elicit_georaster workspace dependency** (`Cargo.toml`):
```toml
elicit_georaster = { path = "crates/elicit_georaster", version = "0.9.1" }
```

**1.4 Add georaster feature to elicitation** (`crates/elicitation/Cargo.toml`):
- Add optional dependencies:
  - `georaster = { workspace = true, optional = true }`
  - `ndarray = { workspace = true, optional = true }`
- Add feature: `georaster = ["dep:georaster", "dep:ndarray"]`
- Update `gis` meta-feature: `gis = ["proj", "geo", "geo-types", "geojson", "rstar", "wkt", "wkb", "georaster"]`

## Phase 2: Core Type Integration in elicitation

### Files to create/modify:
- `crates/elicitation/src/georaster_types.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Type Support Strategy:

**2.1 Core Raster Types**:

```rust
// crates/elicitation/src/georaster_types.rs
#![cfg(feature = "georaster")]

use ndarray::Array2;
use elicitation::{Elicitation, ElicitationContext};

/// Geotransform for converting pixel coordinates to geographic coordinates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeoTransform {
    /// X-coordinate of upper-left corner
    pub origin_x: f64,
    /// Pixel width (west-east resolution)
    pub pixel_width: f64,
    /// Row rotation (usually 0)
    pub row_rotation: f64,
    /// Y-coordinate of upper-left corner
    pub origin_y: f64,
    /// Column rotation (usually 0)
    pub col_rotation: f64,
    /// Pixel height (north-south resolution, usually negative)
    pub pixel_height: f64,
}

impl GeoTransform {
    /// Create standard north-up geotransform
    pub fn new(origin_x: f64, origin_y: f64, pixel_width: f64, pixel_height: f64) -> Self {
        Self {
            origin_x,
            pixel_width,
            row_rotation: 0.0,
            origin_y,
            col_rotation: 0.0,
            pixel_height,
        }
    }

    /// Convert pixel coordinates to geographic coordinates
    pub fn pixel_to_geo(&self, row: usize, col: usize) -> (f64, f64) {
        let x = self.origin_x + (col as f64) * self.pixel_width + (row as f64) * self.row_rotation;
        let y = self.origin_y + (col as f64) * self.col_rotation + (row as f64) * self.pixel_height;
        (x, y)
    }

    /// Convert geographic coordinates to pixel coordinates
    pub fn geo_to_pixel(&self, x: f64, y: f64) -> (isize, isize) {
        // Inverse geotransform
        let det = self.pixel_width * self.pixel_height - self.row_rotation * self.col_rotation;

        let col = ((x - self.origin_x) * self.pixel_height - (y - self.origin_y) * self.row_rotation) / det;
        let row = ((y - self.origin_y) * self.pixel_width - (x - self.origin_x) * self.col_rotation) / det;

        (row.round() as isize, col.round() as isize)
    }

    /// Get pixel width (resolution in x direction)
    pub fn resolution_x(&self) -> f64 {
        self.pixel_width.abs()
    }

    /// Get pixel height (resolution in y direction)
    pub fn resolution_y(&self) -> f64 {
        self.pixel_height.abs()
    }
}

/// Single-band raster with generic data type
#[derive(Debug, Clone)]
pub struct Raster<T> {
    /// Raster data as 2D array
    pub data: Array2<T>,
    /// Geotransform for georeferencing
    pub geotransform: GeoTransform,
    /// No-data value (if any)
    pub nodata: Option<T>,
    /// Coordinate reference system (WKT or PROJ string)
    pub crs: Option<String>,
}

impl<T> Raster<T>
where
    T: Clone + PartialEq,
{
    /// Create new raster
    pub fn new(
        data: Array2<T>,
        geotransform: GeoTransform,
        nodata: Option<T>,
        crs: Option<String>,
    ) -> Self {
        Self { data, geotransform, nodata, crs }
    }

    /// Get raster dimensions (rows, cols)
    pub fn shape(&self) -> (usize, usize) {
        (self.data.nrows(), self.data.ncols())
    }

    /// Get value at pixel location
    pub fn get_pixel(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get((row, col))
    }

    /// Get value at geographic coordinates
    pub fn get_value(&self, x: f64, y: f64) -> Option<&T> {
        let (row, col) = self.geotransform.geo_to_pixel(x, y);

        if row >= 0 && col >= 0 {
            self.get_pixel(row as usize, col as usize)
        } else {
            None
        }
    }

    /// Check if value is NoData
    pub fn is_nodata(&self, value: &T) -> bool {
        if let Some(ref nodata) = self.nodata {
            value == nodata
        } else {
            false
        }
    }

    /// Get bounding box (min_x, min_y, max_x, max_y)
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let (rows, cols) = self.shape();

        let (x_min, y_max) = self.geotransform.pixel_to_geo(0, 0);
        let (x_max, y_min) = self.geotransform.pixel_to_geo(rows, cols);

        (x_min, y_min, x_max, y_max)
    }
}

/// Common raster types
pub type RasterF32 = Raster<f32>;
pub type RasterF64 = Raster<f64>;
pub type RasterI32 = Raster<i32>;
pub type RasterU8 = Raster<u8>;

impl Elicitation for RasterF64 {
    type Error = String;

    async fn elicit(ctx: &mut ElicitationContext) -> Result<Self, Self::Error> {
        // Interactive raster creation
        // Could prompt for: dimensions, resolution, origin, initial values
        todo!("Interactive raster construction")
    }
}

/// Multi-band raster (e.g., RGB imagery)
#[derive(Debug, Clone)]
pub struct MultiBandRaster<T> {
    /// Bands (each is a single-band raster)
    pub bands: Vec<Raster<T>>,
}

impl<T: Clone + PartialEq> MultiBandRaster<T> {
    /// Create from bands (must have same dimensions and geotransform)
    pub fn new(bands: Vec<Raster<T>>) -> Result<Self, String> {
        if bands.is_empty() {
            return Err("No bands provided".to_string());
        }

        // Validate all bands have same dimensions
        let first_shape = bands[0].shape();
        for band in &bands[1..] {
            if band.shape() != first_shape {
                return Err("All bands must have same dimensions".to_string());
            }
        }

        Ok(Self { bands })
    }

    /// Get number of bands
    pub fn band_count(&self) -> usize {
        self.bands.len()
    }

    /// Get band by index
    pub fn get_band(&self, index: usize) -> Option<&Raster<T>> {
        self.bands.get(index)
    }

    /// Get dimensions (from first band)
    pub fn shape(&self) -> (usize, usize) {
        self.bands[0].shape()
    }
}

/// RGB raster (3-band with u8 values)
pub type RgbRaster = MultiBandRaster<u8>;
```

**2.2 Raster Statistics**:

```rust
/// Raster statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RasterStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
    pub count: usize,
    pub nodata_count: usize,
}

impl RasterStats {
    /// Calculate statistics from f64 raster
    pub fn from_raster(raster: &RasterF64) -> Self {
        let mut valid_values = Vec::new();

        for value in raster.data.iter() {
            if !raster.is_nodata(value) {
                valid_values.push(*value);
            }
        }

        let count = valid_values.len();
        let nodata_count = raster.data.len() - count;

        if valid_values.is_empty() {
            return Self {
                min: f64::NAN,
                max: f64::NAN,
                mean: f64::NAN,
                std_dev: f64::NAN,
                count: 0,
                nodata_count,
            };
        }

        let min = valid_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = valid_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let mean = valid_values.iter().sum::<f64>() / count as f64;

        let variance = valid_values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        Self {
            min,
            max,
            mean,
            std_dev,
            count,
            nodata_count,
        }
    }
}
```

**2.3 Export from lib.rs** (`crates/elicitation/src/lib.rs`):
```rust
#[cfg(feature = "georaster")]
pub mod georaster_types;

#[cfg(feature = "georaster")]
pub use georaster_types::{
    GeoTransform, Raster, RasterF32, RasterF64, RasterI32, RasterU8,
    MultiBandRaster, RgbRaster, RasterStats,
};
```

## Phase 3: Create elicit_georaster Shadow Crate

### Directory Structure:

```
crates/elicit_georaster/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── raster.rs          (Raster wrapper)
│   ├── transform.rs       (GeoTransform utilities)
│   ├── stats.rs           (Statistics calculations)
│   ├── algebra.rs         (Raster algebra operations)
│   └── workflow/
│       ├── mod.rs
│       ├── create_plugin.rs      (~6 tools: create rasters)
│       ├── io_plugin.rs          (~5 tools: read/write GeoTIFF)
│       ├── algebra_plugin.rs     (~8 tools: add, subtract, multiply, etc.)
│       ├── stats_plugin.rs       (~6 tools: statistics, histograms)
│       ├── terrain_plugin.rs     (~6 tools: slope, aspect, hillshade)
│       ├── resample_plugin.rs    (~4 tools: resampling, reprojection)
│       └── workflow_plugin.rs    (~5 tools: common workflows)
└── tests/
    └── georaster_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_georaster"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled raster wrappers with comprehensive MCP tools for geospatial grid operations"
keywords = ["mcp", "raster", "gis", "georaster", "elicitation"]
categories = ["science::geo", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["georaster"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
georaster = { workspace = true }
ndarray = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
uuid = { workspace = true }

# Optional GeoTIFF support
# gdal = { version = "0.16", optional = true }

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[dev-dependencies]
tokio = { workspace = true }
approx = "0.5"  # For floating-point comparisons

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]
# geotiff = ["dep:gdal"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure:

```rust
//! `elicit_georaster` — comprehensive raster data API exposure via MCP tools.
//!
//! Provides complete coverage of geospatial raster operations:
//! - Raster creation and manipulation
//! - Raster algebra (add, subtract, multiply, divide)
//! - Statistics (min, max, mean, std dev, histograms)
//! - Terrain analysis (slope, aspect, hillshade)
//! - Resampling and reprojection
//! - Multi-band operations (RGB, multispectral)
//! - NoData handling
//!
//! # Plugin Organization (7 plugins, ~40 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `RasterCreatePlugin` | 6 | Create rasters from data |
//! | `RasterIoPlugin` | 5 | Read/write GeoTIFF, formats |
//! | `RasterAlgebraPlugin` | 8 | Mathematical operations |
//! | `RasterStatsPlugin` | 6 | Statistics and histograms |
//! | `RasterTerrainPlugin` | 6 | Slope, aspect, hillshade |
//! | `RasterResamplePlugin` | 4 | Resampling, reprojection |
//! | `RasterWorkflowPlugin` | 5 | Common workflows |
//!
//! # Raster Data Model
//!
//! ```text
//! Raster = Grid + Geotransform + NoData + CRS
//!
//! Grid:         2D array of values (ndarray::Array2)
//! Geotransform: Pixel coords → Geographic coords
//! NoData:       Missing data value
//! CRS:          Coordinate reference system
//! ```
//!
//! # Integration with GeoRust
//!
//! Works with geo_types for vector-raster operations:
//!
//! ```rust
//! use elicit_georaster::Raster;
//! use geo_types::Point;
//!
//! let raster: Raster<f64> = /* ... */;
//!
//! // Sample raster at point
//! let point = Point::new(-122.4194, 37.7749);
//! let value = raster.get_value(point.x(), point.y());
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod algebra;
mod raster;
mod resample;
mod stats;
mod terrain;
mod transform;
pub mod workflow;

pub use algebra::RasterAlgebra;
pub use raster::{Raster, MultiBandRaster};
pub use stats::RasterStats;
pub use terrain::Terrain;
pub use transform::GeoTransform;
pub use workflow::{
    RasterAlgebraPlugin, RasterCreatePlugin, RasterIoPlugin,
    RasterResamplePlugin, RasterStatsPlugin, RasterTerrainPlugin,
    RasterWorkflowPlugin,
};

/// Common raster types
pub type RasterF32 = Raster<f32>;
pub type RasterF64 = Raster<f64>;
pub type RasterI32 = Raster<i32>;
pub type RasterU8 = Raster<u8>;
pub type RgbRaster = MultiBandRaster<u8>;
```

## Phase 4: Implement Core Type Wrappers

### 4.1 Raster Wrapper (raster.rs):

```rust
// raster.rs
use elicitation_derive::reflect_methods;
use ndarray::Array2;

/// Single-band raster wrapper
#[derive(Debug, Clone)]
pub struct Raster<T> {
    pub(crate) data: Array2<T>,
    pub(crate) geotransform: crate::GeoTransform,
    pub(crate) nodata: Option<T>,
    pub(crate) crs: Option<String>,
}

#[reflect_methods]
impl<T: Clone + PartialEq> Raster<T> {
    /// Create new raster
    #[instrument(skip(data))]
    pub fn new(
        data: Array2<T>,
        geotransform: crate::GeoTransform,
        nodata: Option<T>,
        crs: Option<String>,
    ) -> Self {
        Self { data, geotransform, nodata, crs }
    }

    /// Create filled raster
    #[instrument]
    pub fn filled(
        rows: usize,
        cols: usize,
        value: T,
        geotransform: crate::GeoTransform,
        crs: Option<String>,
    ) -> Self {
        let data = Array2::from_elem((rows, cols), value);
        Self::new(data, geotransform, None, crs)
    }

    /// Get dimensions (rows, cols)
    pub fn shape(&self) -> (usize, usize) {
        (self.data.nrows(), self.data.ncols())
    }

    /// Get value at pixel
    #[instrument(skip(self))]
    pub fn get_pixel(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get((row, col))
    }

    /// Set value at pixel
    #[instrument(skip(self))]
    pub fn set_pixel(&mut self, row: usize, col: usize, value: T) -> Result<(), String> {
        if row >= self.data.nrows() || col >= self.data.ncols() {
            return Err("Pixel out of bounds".to_string());
        }
        self.data[(row, col)] = value;
        Ok(())
    }

    /// Get value at geographic coordinates
    #[instrument(skip(self))]
    pub fn get_value(&self, x: f64, y: f64) -> Option<&T> {
        let (row, col) = self.geotransform.geo_to_pixel(x, y);

        if row >= 0 && col >= 0 {
            self.get_pixel(row as usize, col as usize)
        } else {
            None
        }
    }

    /// Check if value is NoData
    pub fn is_nodata(&self, value: &T) -> bool {
        if let Some(ref nodata) = self.nodata {
            value == nodata
        } else {
            false
        }
    }

    /// Get bounding box (min_x, min_y, max_x, max_y)
    #[instrument(skip(self))]
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let (rows, cols) = self.shape();

        let (x_min, y_max) = self.geotransform.pixel_to_geo(0, 0);
        let (x_max, y_min) = self.geotransform.pixel_to_geo(rows, cols);

        (x_min, y_min, x_max, y_max)
    }

    /// Get geotransform
    pub fn geotransform(&self) -> &crate::GeoTransform {
        &self.geotransform
    }

    /// Get CRS
    pub fn crs(&self) -> Option<&str> {
        self.crs.as_deref()
    }

    /// Get NoData value
    pub fn nodata(&self) -> Option<&T> {
        self.nodata.as_ref()
    }
}

/// Multi-band raster wrapper
#[derive(Debug, Clone)]
pub struct MultiBandRaster<T> {
    pub(crate) bands: Vec<Raster<T>>,
}

#[reflect_methods]
impl<T: Clone + PartialEq> MultiBandRaster<T> {
    /// Create from bands
    #[instrument(skip(bands))]
    pub fn new(bands: Vec<Raster<T>>) -> Result<Self, String> {
        if bands.is_empty() {
            return Err("No bands provided".to_string());
        }

        let first_shape = bands[0].shape();
        for band in &bands[1..] {
            if band.shape() != first_shape {
                return Err("All bands must have same dimensions".to_string());
            }
        }

        Ok(Self { bands })
    }

    /// Get band count
    pub fn band_count(&self) -> usize {
        self.bands.len()
    }

    /// Get band by index
    pub fn get_band(&self, index: usize) -> Option<&Raster<T>> {
        self.bands.get(index)
    }

    /// Get dimensions
    pub fn shape(&self) -> (usize, usize) {
        self.bands[0].shape()
    }
}
```

### 4.2 Algebra Operations (algebra.rs):

```rust
// algebra.rs
use crate::Raster;
use std::ops::{Add, Sub, Mul, Div};

/// Raster algebra operations
pub struct RasterAlgebra;

impl RasterAlgebra {
    /// Add two rasters
    #[instrument(skip(a, b))]
    pub fn add(a: &Raster<f64>, b: &Raster<f64>) -> Result<Raster<f64>, String> {
        if a.shape() != b.shape() {
            return Err("Rasters must have same dimensions".to_string());
        }

        let result_data = &a.data + &b.data;

        Ok(Raster::new(
            result_data,
            a.geotransform.clone(),
            a.nodata.clone(),
            a.crs.clone(),
        ))
    }

    /// Subtract raster b from a
    #[instrument(skip(a, b))]
    pub fn subtract(a: &Raster<f64>, b: &Raster<f64>) -> Result<Raster<f64>, String> {
        if a.shape() != b.shape() {
            return Err("Rasters must have same dimensions".to_string());
        }

        let result_data = &a.data - &b.data;

        Ok(Raster::new(
            result_data,
            a.geotransform.clone(),
            a.nodata.clone(),
            a.crs.clone(),
        ))
    }

    /// Multiply two rasters
    #[instrument(skip(a, b))]
    pub fn multiply(a: &Raster<f64>, b: &Raster<f64>) -> Result<Raster<f64>, String> {
        if a.shape() != b.shape() {
            return Err("Rasters must have same dimensions".to_string());
        }

        let result_data = &a.data * &b.data;

        Ok(Raster::new(
            result_data,
            a.geotransform.clone(),
            a.nodata.clone(),
            a.crs.clone(),
        ))
    }

    /// Divide raster a by b
    #[instrument(skip(a, b))]
    pub fn divide(a: &Raster<f64>, b: &Raster<f64>) -> Result<Raster<f64>, String> {
        if a.shape() != b.shape() {
            return Err("Rasters must have same dimensions".to_string());
        }

        let result_data = &a.data / &b.data;

        Ok(Raster::new(
            result_data,
            a.geotransform.clone(),
            a.nodata.clone(),
            a.crs.clone(),
        ))
    }

    /// Multiply raster by scalar
    #[instrument(skip(raster))]
    pub fn scale(raster: &Raster<f64>, scalar: f64) -> Raster<f64> {
        let result_data = &raster.data * scalar;

        Raster::new(
            result_data,
            raster.geotransform.clone(),
            raster.nodata.clone(),
            raster.crs.clone(),
        )
    }

    /// Apply function to each pixel
    #[instrument(skip(raster, f))]
    pub fn map<F>(raster: &Raster<f64>, f: F) -> Raster<f64>
    where
        F: Fn(f64) -> f64,
    {
        let result_data = raster.data.mapv(f);

        Raster::new(
            result_data,
            raster.geotransform.clone(),
            raster.nodata.clone(),
            raster.crs.clone(),
        )
    }
}
```

### 4.3 Terrain Analysis (terrain.rs):

```rust
// terrain.rs
use crate::Raster;
use std::f64::consts::PI;

/// Terrain analysis operations
pub struct Terrain;

impl Terrain {
    /// Calculate slope in degrees
    #[instrument(skip(dem))]
    pub fn slope(dem: &Raster<f64>) -> Result<Raster<f64>, String> {
        let (rows, cols) = dem.shape();
        let mut slope_data = ndarray::Array2::zeros((rows, cols));

        let res_x = dem.geotransform.resolution_x();
        let res_y = dem.geotransform.resolution_y();

        for row in 1..rows-1 {
            for col in 1..cols-1 {
                let z1 = dem.data[(row-1, col-1)];
                let z2 = dem.data[(row-1, col)];
                let z3 = dem.data[(row-1, col+1)];
                let z4 = dem.data[(row, col-1)];
                let z6 = dem.data[(row, col+1)];
                let z7 = dem.data[(row+1, col-1)];
                let z8 = dem.data[(row+1, col)];
                let z9 = dem.data[(row+1, col+1)];

                let dz_dx = ((z3 + 2.0 * z6 + z9) - (z1 + 2.0 * z4 + z7)) / (8.0 * res_x);
                let dz_dy = ((z7 + 2.0 * z8 + z9) - (z1 + 2.0 * z2 + z3)) / (8.0 * res_y);

                let slope_rad = (dz_dx * dz_dx + dz_dy * dz_dy).sqrt().atan();
                slope_data[(row, col)] = slope_rad * 180.0 / PI;
            }
        }

        Ok(Raster::new(
            slope_data,
            dem.geotransform.clone(),
            None,
            dem.crs.clone(),
        ))
    }

    /// Calculate aspect in degrees (0-360)
    #[instrument(skip(dem))]
    pub fn aspect(dem: &Raster<f64>) -> Result<Raster<f64>, String> {
        let (rows, cols) = dem.shape();
        let mut aspect_data = ndarray::Array2::zeros((rows, cols));

        let res_x = dem.geotransform.resolution_x();
        let res_y = dem.geotransform.resolution_y();

        for row in 1..rows-1 {
            for col in 1..cols-1 {
                let z1 = dem.data[(row-1, col-1)];
                let z2 = dem.data[(row-1, col)];
                let z3 = dem.data[(row-1, col+1)];
                let z4 = dem.data[(row, col-1)];
                let z6 = dem.data[(row, col+1)];
                let z7 = dem.data[(row+1, col-1)];
                let z8 = dem.data[(row+1, col)];
                let z9 = dem.data[(row+1, col+1)];

                let dz_dx = ((z3 + 2.0 * z6 + z9) - (z1 + 2.0 * z4 + z7)) / (8.0 * res_x);
                let dz_dy = ((z7 + 2.0 * z8 + z9) - (z1 + 2.0 * z2 + z3)) / (8.0 * res_y);

                let aspect_rad = dz_dy.atan2(dz_dx);
                let aspect_deg = aspect_rad * 180.0 / PI;

                // Convert to compass bearing (0 = North, 90 = East)
                aspect_data[(row, col)] = 90.0 - aspect_deg;
                if aspect_data[(row, col)] < 0.0 {
                    aspect_data[(row, col)] += 360.0;
                }
            }
        }

        Ok(Raster::new(
            aspect_data,
            dem.geotransform.clone(),
            None,
            dem.crs.clone(),
        ))
    }

    /// Calculate hillshade
    #[instrument(skip(dem))]
    pub fn hillshade(
        dem: &Raster<f64>,
        azimuth: f64,
        altitude: f64,
    ) -> Result<Raster<f64>, String> {
        // Azimuth: 0-360 degrees (0 = North, 90 = East)
        // Altitude: 0-90 degrees (sun angle above horizon)

        let slope = Self::slope(dem)?;
        let aspect = Self::aspect(dem)?;

        let (rows, cols) = dem.shape();
        let mut hillshade_data = ndarray::Array2::zeros((rows, cols));

        let azimuth_rad = azimuth * PI / 180.0;
        let altitude_rad = altitude * PI / 180.0;

        for row in 0..rows {
            for col in 0..cols {
                let slope_rad = slope.data[(row, col)] * PI / 180.0;
                let aspect_rad = aspect.data[(row, col)] * PI / 180.0;

                let shade = altitude_rad.sin() * slope_rad.cos() +
                           altitude_rad.cos() * slope_rad.sin() *
                           (azimuth_rad - aspect_rad).cos();

                hillshade_data[(row, col)] = shade.max(0.0) * 255.0;
            }
        }

        Ok(Raster::new(
            hillshade_data,
            dem.geotransform.clone(),
            None,
            dem.crs.clone(),
        ))
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Create Plugin (workflow/create_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateRasterParams {
    pub rows: usize,
    pub cols: usize,
    pub origin_x: f64,
    pub origin_y: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
    pub fill_value: Option<f64>,
}

#[elicit_tool(
    plugin = "raster_create",
    name = "raster_create__filled",
    description = "Create raster filled with value. Default fill is 0.0.",
    emit = Auto
)]
async fn create_filled(p: CreateRasterParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created {}x{} raster", p.rows, p.cols))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateFromArrayParams {
    pub data: Vec<Vec<f64>>,
    pub origin_x: f64,
    pub origin_y: f64,
    pub pixel_width: f64,
    pub pixel_height: f64,
}

#[elicit_tool(
    plugin = "raster_create",
    name = "raster_create__from_array",
    description = "Create raster from 2D array of values.",
    emit = Auto
)]
async fn create_from_array(p: CreateFromArrayParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created raster from array"))
    ]))
}

// ... 4 more tools: from_function, zeros, ones, random
```

### 5.2 Algebra Plugin (workflow/algebra_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BinaryOpParams {
    pub raster_a_id: String,
    pub raster_b_id: String,
}

#[elicit_tool(
    plugin = "raster_algebra",
    name = "raster_algebra__add",
    description = "Add two rasters element-wise.",
    emit = Auto
)]
async fn algebra_add(p: BinaryOpParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Added rasters"))
    ]))
}

#[elicit_tool(
    plugin = "raster_algebra",
    name = "raster_algebra__subtract",
    description = "Subtract raster B from raster A.",
    emit = Auto
)]
async fn algebra_subtract(p: BinaryOpParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Subtracted rasters"))
    ]))
}

// ... 6 more tools: multiply, divide, scale, power, log, normalize
```

### 5.3 Terrain Plugin (workflow/terrain_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SlopeParams {
    pub dem_id: String,
}

#[elicit_tool(
    plugin = "raster_terrain",
    name = "raster_terrain__slope",
    description = "Calculate slope from DEM in degrees.",
    emit = Auto
)]
async fn terrain_slope(p: SlopeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Calculated slope"))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HillshadeParams {
    pub dem_id: String,
    pub azimuth: f64,
    pub altitude: f64,
}

#[elicit_tool(
    plugin = "raster_terrain",
    name = "raster_terrain__hillshade",
    description = "Calculate hillshade. Azimuth: 0-360 (0=N), Altitude: 0-90.",
    emit = Auto
)]
async fn terrain_hillshade(p: HillshadeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Calculated hillshade with azimuth={}, altitude={}", p.azimuth, p.altitude))
    ]))
}

// ... 4 more tools: aspect, curvature, tpi, tri
```

### 5.4 Stats Plugin (workflow/stats_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatsParams {
    pub raster_id: String,
}

#[elicit_tool(
    plugin = "raster_stats",
    name = "raster_stats__summary",
    description = "Calculate raster statistics: min, max, mean, std dev.",
    emit = Auto
)]
async fn stats_summary(p: StatsParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Raster statistics"))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramParams {
    pub raster_id: String,
    pub bins: Option<usize>,
}

#[elicit_tool(
    plugin = "raster_stats",
    name = "raster_stats__histogram",
    description = "Calculate histogram. Default 10 bins.",
    emit = Auto
)]
async fn stats_histogram(p: HistogramParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Histogram with {} bins", p.bins.unwrap_or(10)))
    ]))
}

// ... 4 more tools: percentile, variance, range, count_values
```

## Phase 6: Testing

### Test Coverage:

```rust
#[test]
fn test_create_raster() {
    let gt = GeoTransform::new(0.0, 10.0, 1.0, -1.0);
    let raster = Raster::filled(10, 10, 0.0, gt, None);

    assert_eq!(raster.shape(), (10, 10));
}

#[test]
fn test_raster_algebra() {
    let gt = GeoTransform::new(0.0, 10.0, 1.0, -1.0);
    let a = Raster::filled(5, 5, 2.0, gt.clone(), None);
    let b = Raster::filled(5, 5, 3.0, gt, None);

    let result = RasterAlgebra::add(&a, &b).unwrap();
    assert_eq!(result.data[(0, 0)], 5.0);
}

#[test]
fn test_slope_calculation() {
    // Create simple DEM (plane)
    let mut data = Array2::zeros((5, 5));
    for row in 0..5 {
        for col in 0..5 {
            data[(row, col)] = (row + col) as f64 * 10.0;
        }
    }

    let gt = GeoTransform::new(0.0, 10.0, 1.0, -1.0);
    let dem = Raster::new(data, gt, None, None);

    let slope = Terrain::slope(&dem).unwrap();
    assert!(slope.data[(2, 2)] > 0.0);
}

#[test]
fn test_pixel_to_geo() {
    let gt = GeoTransform::new(100.0, 10.0, 1.0, -1.0);

    let (x, y) = gt.pixel_to_geo(0, 0);
    assert_eq!(x, 100.0);
    assert_eq!(y, 10.0);

    let (x, y) = gt.pixel_to_geo(1, 1);
    assert_eq!(x, 110.0);
    assert_eq!(y, 9.0);
}

#[test]
fn test_geo_to_pixel() {
    let gt = GeoTransform::new(100.0, 10.0, 1.0, -1.0);

    let (row, col) = gt.geo_to_pixel(100.0, 10.0);
    assert_eq!(row, 0);
    assert_eq!(col, 0);

    let (row, col) = gt.geo_to_pixel(110.0, 9.0);
    assert_eq!(row, 1);
    assert_eq!(col, 1);
}
```

## Phase 7: Documentation

### README.md content (abbreviated):

```markdown
# elicit_georaster

Comprehensive raster data operations for geospatial analysis.

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `raster_create` | 6 | Create rasters |
| `raster_io` | 5 | Read/write GeoTIFF |
| `raster_algebra` | 8 | Mathematical operations |
| `raster_stats` | 6 | Statistics |
| `raster_terrain` | 6 | Slope, aspect, hillshade |
| `raster_resample` | 4 | Resampling |
| `raster_workflow` | 5 | Common workflows |

**Total: ~40 MCP tools**

## Use Cases

- **Elevation analysis**: DEM processing, terrain metrics
- **Satellite imagery**: Landsat, Sentinel processing
- **Climate data**: Temperature, precipitation grids
- **Land cover**: Classification rasters
```

## Implementation Order

1. **Phase 1**: Workspace configuration (30 min)
2. **Phase 2**: Core type integration (2 hours)
3. **Phase 3**: Create structure (1 hour)
4. **Phase 4**: Implement wrappers (4-5 hours)
5. **Phase 5**: Implement MCP tools (8-10 hours)
6. **Phase 6**: Testing (2 hours)
7. **Phase 7**: Documentation (1 hour)

**Total: 18-22 hours**
