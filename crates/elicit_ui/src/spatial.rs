//! Spatial bridge between AccessKit nodes and GeoRust geometry.
//!
//! Provides conversions from AccessKit bounding boxes to geo-types
//! primitives, enabling geometric constraint checking (containment,
//! overlap, spacing).

#[cfg(feature = "geo")]
use geo_types::{Coord, Rect as GeoRect};

use crate::{Size, Viewport};

/// Axis-aligned bounding box for a UI element.
///
/// Stored as origin + size to match AccessKit's coordinate model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// X coordinate of the top-left corner.
    pub x: f64,
    /// Y coordinate of the top-left corner.
    pub y: f64,
    /// Width in pixels.
    pub width: f64,
    /// Height in pixels.
    pub height: f64,
}

impl BoundingBox {
    /// Create a bounding box from origin and dimensions.
    #[tracing::instrument(level = "trace")]
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create from AccessKit node bounds (if present).
    #[tracing::instrument(level = "trace", skip(node))]
    pub fn from_node(node: &accesskit::Node) -> Option<Self> {
        let bounds = node.bounds()?;
        Some(Self {
            x: bounds.x0,
            y: bounds.y0,
            width: bounds.x1 - bounds.x0,
            height: bounds.y1 - bounds.y0,
        })
    }

    /// Right edge x coordinate.
    pub fn right(&self) -> f64 {
        self.x + self.width
    }

    /// Bottom edge y coordinate.
    pub fn bottom(&self) -> f64 {
        self.y + self.height
    }

    /// Check if this box is fully contained within a viewport.
    #[tracing::instrument(level = "trace")]
    pub fn within_viewport(&self, viewport: &Viewport) -> bool {
        self.x >= 0.0
            && self.y >= 0.0
            && self.right() <= f64::from(viewport.width)
            && self.bottom() <= f64::from(viewport.height)
    }

    /// Check if this box meets minimum touch target size (44x44).
    pub fn meets_touch_target(&self) -> bool {
        self.width >= 44.0 && self.height >= 44.0
    }

    /// Convert to [`Size`] (truncating to integer pixels).
    pub fn to_size(&self) -> Size {
        Size::new(self.width as u32, self.height as u32)
    }

    /// Convert to a geo-types [`Rect`] for spatial queries.
    #[cfg(feature = "geo")]
    #[tracing::instrument(level = "trace")]
    pub fn to_geo_rect(&self) -> GeoRect {
        GeoRect::new(
            Coord {
                x: self.x,
                y: self.y,
            },
            Coord {
                x: self.right(),
                y: self.bottom(),
            },
        )
    }
}

/// Layout context providing viewport and spatial index data.
///
/// Used by constraints that need geometric reasoning (containment,
/// spacing, reflow).
#[derive(Debug, Clone)]
pub struct LayoutContext {
    /// Viewport dimensions.
    pub viewport: Viewport,
    /// All element bounding boxes indexed by NodeId.
    pub bounds: std::collections::HashMap<accesskit::NodeId, BoundingBox>,
}

impl LayoutContext {
    /// Create a new layout context.
    #[tracing::instrument(level = "debug", skip(bounds), fields(num_bounds = bounds.len()))]
    pub fn new(
        viewport: Viewport,
        bounds: std::collections::HashMap<accesskit::NodeId, BoundingBox>,
    ) -> Self {
        Self { viewport, bounds }
    }

    /// Get the bounding box for a node.
    pub fn get_bounds(&self, node_id: &accesskit::NodeId) -> Option<&BoundingBox> {
        self.bounds.get(node_id)
    }

    /// Check if a node's bounding box is within the viewport.
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn is_within_viewport(&self, node_id: &accesskit::NodeId) -> Option<bool> {
        self.bounds
            .get(node_id)
            .map(|bb| bb.within_viewport(&self.viewport))
    }

    /// Check containment: is `inner` fully inside `outer`?
    #[cfg(feature = "geo")]
    #[tracing::instrument(level = "trace", skip(self))]
    pub fn contains(
        &self,
        outer: &accesskit::NodeId,
        inner: &accesskit::NodeId,
    ) -> Option<bool> {
        use geo::Contains;
        let outer_rect = self.bounds.get(outer)?.to_geo_rect();
        let inner_rect = self.bounds.get(inner)?.to_geo_rect();
        Some(outer_rect.contains(&inner_rect))
    }
}

#[cfg(feature = "geo")]
impl BoundingBox {
    /// Convert to a geo-types [`Polygon`] for complex spatial queries.
    pub fn to_geo_polygon(&self) -> geo_types::Polygon {
        use geo_types::{LineString, Polygon};
        let coords = vec![
            Coord {
                x: self.x,
                y: self.y,
            },
            Coord {
                x: self.right(),
                y: self.y,
            },
            Coord {
                x: self.right(),
                y: self.bottom(),
            },
            Coord {
                x: self.x,
                y: self.bottom(),
            },
            Coord {
                x: self.x,
                y: self.y,
            },
        ];
        Polygon::new(LineString::from(coords), vec![])
    }
}
