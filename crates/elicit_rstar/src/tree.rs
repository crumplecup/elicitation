//! Snapshot wrapper around `rstar::RTree<T>`.

use elicitation::{ElicitComplete, RstarAabb};
use rstar::{AABB, PointDistance, RTree, RTreeObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Serializable snapshot of an `rstar::RTree<T>`.
///
/// The dynamic-tool registry serializes tool targets as JSON between calls, so
/// this wrapper stores the logical tree contents as an ordered `Vec<T>` and
/// reconstructs the runtime `RTree<T>` on demand for spatial queries.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    derive_new::new,
    Serialize,
    Deserialize,
    JsonSchema,
    elicitation_derive::Elicit,
)]
#[serde(bound(
    serialize = "T: Serialize",
    deserialize = "T: serde::de::DeserializeOwned"
))]
#[schemars(bound = "T: JsonSchema")]
pub struct RstarTree<T: ElicitComplete + Send + Sync + 'static> {
    /// Logical contents of the tree snapshot.
    items: Vec<T>,
}

impl<T> RstarTree<T>
where
    T: ElicitComplete + Send + Sync + 'static,
{
    /// Returns an empty tree snapshot.
    #[tracing::instrument]
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Returns the number of stored items.
    #[tracing::instrument(skip(self))]
    pub fn size(&self) -> usize {
        self.items.len()
    }
}

impl<T> RstarTree<T>
where
    T: ElicitComplete + Clone + RTreeObject<Envelope = AABB<[f64; 2]>> + Send + Sync + 'static,
{
    /// Bulk-load a tree snapshot from items.
    #[tracing::instrument(skip(items), fields(count = items.len()))]
    pub fn bulk_load(items: Vec<T>) -> Self {
        Self::new(items)
    }

    /// Returns the current items in snapshot order.
    #[tracing::instrument(skip(self))]
    pub fn items(&self) -> Vec<T> {
        self.items.clone()
    }

    /// Returns a new tree snapshot with one additional item inserted.
    #[tracing::instrument(skip(self, item))]
    pub fn insert(&self, item: T) -> Self {
        let mut items = self.items.clone();
        items.push(item);
        Self::new(items)
    }

    /// Returns all items fully contained within the given envelope.
    #[tracing::instrument(skip(self))]
    pub fn locate_in_envelope(&self, envelope: RstarAabb) -> Vec<T> {
        let tree = self.runtime_tree();
        let envelope: AABB<[f64; 2]> = envelope.into();
        tree.locate_in_envelope(&envelope).cloned().collect()
    }

    /// Returns all items whose envelopes intersect the given envelope.
    #[tracing::instrument(skip(self))]
    pub fn locate_in_envelope_intersecting(&self, envelope: RstarAabb) -> Vec<T> {
        let tree = self.runtime_tree();
        let envelope: AABB<[f64; 2]> = envelope.into();
        tree.locate_in_envelope_intersecting(&envelope)
            .cloned()
            .collect()
    }

    fn runtime_tree(&self) -> RTree<T> {
        RTree::bulk_load(self.items.clone())
    }
}

impl<T> RstarTree<T>
where
    T: ElicitComplete
        + Clone
        + PointDistance
        + RTreeObject<Envelope = AABB<[f64; 2]>>
        + Send
        + Sync
        + 'static,
{
    /// Returns the nearest neighbor to the query point, if any.
    #[tracing::instrument(skip(self))]
    pub fn nearest_neighbor(&self, point: [f64; 2]) -> Option<T> {
        let tree = self.runtime_tree();
        tree.nearest_neighbor(&point).cloned()
    }

    /// Returns all nearest neighbors tied for minimum distance to the query point.
    #[tracing::instrument(skip(self))]
    pub fn nearest_neighbors(&self, point: [f64; 2]) -> Vec<T> {
        let tree = self.runtime_tree();
        tree.nearest_neighbors(&point)
            .into_iter()
            .cloned()
            .collect()
    }

    /// Returns all items containing the query point.
    #[tracing::instrument(skip(self))]
    pub fn locate_all_at_point(&self, point: [f64; 2]) -> Vec<T> {
        let tree = self.runtime_tree();
        tree.locate_all_at_point(&point).cloned().collect()
    }

    /// Returns all items within the given squared distance of the query point.
    #[tracing::instrument(skip(self))]
    pub fn locate_within_distance(&self, point: [f64; 2], max_distance_2: f64) -> Vec<T> {
        let tree = self.runtime_tree();
        tree.locate_within_distance(point, max_distance_2)
            .cloned()
            .collect()
    }
}

/// Built-in tree alias for rectangle items.
pub type RectangleTree = RstarTree<elicitation::RstarRectangle>;

/// Built-in tree alias for line items.
pub type LineTree = RstarTree<elicitation::RstarLine>;
