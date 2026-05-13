//! `Pixels` — owned pixel-window wrapper.

use crate::RasterValue;

/// Owned wrapper for the collected output of `GeoTiffReader::pixels(...)`.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct Pixels {
    /// Materialized `(x, y, value)` entries from the upstream iterator.
    pub items: Vec<(u32, u32, RasterValue)>,
}

impl Pixels {
    /// Returns the number of collected pixel entries.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true when no pixels were collected.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl IntoIterator for Pixels {
    type Item = (u32, u32, RasterValue);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
