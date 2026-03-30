# ELICIT_RSTAR_PLAN.md

## Goal
Add complete rstar support to elicitation as spatial indexing alphabet:
1. **Core type integration** — rstar in `elicitation` with feature gating
2. **Shadow crate** — `elicit_rstar` with MCP tools for efficient spatial queries

## Architecture Overview

Following established patterns from elicit_hashbrown, elicit_indexmap:
- **Core**: Feature-gated rstar with R*-tree spatial index
- **Shadow crate**: ~4-6 workflow plugins covering spatial indexing operations
- **Indexing alphabet**: Efficient O(log n) spatial queries for large UI layouts

## Why R*-tree for UI?

R*-tree provides efficient spatial indexing:
- **Point queries**: "Find element at (x, y)" - O(log n) instead of O(n)
- **Range queries**: "Find all elements in viewport" - efficient viewport culling
- **Nearest neighbor**: "Find closest element to cursor" - O(log n)
- **Bulk operations**: Insert/remove hundreds of elements efficiently
- **Dynamic**: Handles layout changes (drag-and-drop, resizing)

## Performance Impact

Without spatial index (linear scan):
```rust
// O(n) - scan every element
for (id, node) in layout.nodes() {
    if node.bounds().contains(&cursor_pos) {
        return Some(id);
    }
}
```

With R*-tree:
```rust
// O(log n) - tree traversal
rtree.locate_at_point(&cursor_pos)
```

For 1000 UI elements:
- Linear: ~1000 comparisons
- R*-tree: ~10 comparisons

## API Coverage

rstar provides R*-tree spatial indexing:
- **Index Operations**: `insert`, `remove`, `bulk_load`
- **Point Queries**: `locate_at_point`, `locate_all_at_point`
- **Range Queries**: `locate_in_envelope`, `locate_within_distance`
- **Nearest Neighbor**: `nearest_neighbor`, `nearest_neighbor_iter`
- **Iteration**: `iter`, `drain`, `remove_at_point`
- **Bulk Updates**: Efficient batch operations

**Total API surface**: ~20 core methods → ~30-40 MCP tools across 4-6 plugins

## Phase 1: Workspace Configuration

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicitation/Cargo.toml`

### Changes:

**1.1 Add rstar to workspace dependencies**:
```toml
# Spatial indexing
rstar = "0.12"
```

**1.2 Add elicit_rstar member**:
```toml
  "crates/elicit_rstar",
```

**1.3 Add elicit_rstar workspace dependency**:
```toml
elicit_rstar = { path = "crates/elicit_rstar", version = "0.9.1" }
```

**1.4 Add rstar feature to elicitation**:
- Add optional dependency: `rstar = { workspace = true, optional = true }`
- Add feature: `rstar = ["dep:rstar", "geo_types"]`
- Update `full` feature to include `"rstar"`

## Phase 2: Core Type Integration

### Files to create/modify:
- `crates/elicitation/src/rstar_support.rs` (new)
- `crates/elicitation/src/lib.rs` (modify)

### Integration Strategy:

**2.1 RTree Type**:

rstar provides generic R*-tree:
```rust
pub struct RTree<T> where T: RTreeObject {
    // Internal node structure
}

// Elements must implement RTreeObject
pub trait RTreeObject: Sized {
    type Envelope: Envelope;
    fn envelope(&self) -> Self::Envelope;
}

// Envelope defines bounding box
pub trait Envelope: PartialEq {
    type Point: Point;
    fn new_empty() -> Self;
    fn contains_point(&self, point: &Self::Point) -> bool;
    fn contains_envelope(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;
}
```

**2.2 Integration with geo-types**:

rstar has built-in geo-types support:
```rust
use rstar::{RTree, AABB};
use geo_types::{Point, Rect};

// geo-types already implements RTreeObject!
let points = vec![
    Point::new(0.0, 0.0),
    Point::new(10.0, 20.0),
];
let tree = RTree::bulk_load(points);

// Query
let search_point = Point::new(5.0, 10.0);
let nearest = tree.nearest_neighbor(&search_point);
```

**2.3 Custom UI Element Wrapper**:

For elicit_ui integration:
```rust
use accesskit::NodeId;
use geo_types::Rect;
use rstar::RTreeObject;

/// UI element in spatial index
#[derive(Debug, Clone)]
pub struct IndexedElement {
    pub id: NodeId,
    pub bounds: Rect<f64>,
}

impl RTreeObject for IndexedElement {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners(
            [self.bounds.min().x, self.bounds.min().y],
            [self.bounds.max().x, self.bounds.max().y]
        )
    }
}
```

### Implementation Pattern:

```rust
// crates/elicitation/src/rstar_support.rs
#![cfg(feature = "rstar")]

use rstar::{RTree, AABB, RTreeObject};

// Re-export rstar types
pub use rstar::{RTree, AABB, PointDistance, SelectionFunction};

// Elicitation impl for RTree (if needed)
// Most operations are direct method calls, no custom impl needed
```

**2.4 Export from lib.rs**:
```rust
#[cfg(feature = "rstar")]
pub mod rstar_support;

#[cfg(feature = "rstar")]
pub use rstar_support::*;
```

## Phase 3: Create elicit_rstar Shadow Crate

### Directory Structure:

```
crates/elicit_rstar/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs
│   ├── indexed_element.rs  (UI element wrapper)
│   └── workflow/
│       ├── mod.rs
│       ├── construction_plugin.rs  (~6 tools: Create, bulk_load trees)
│       ├── query_plugin.rs         (~10 tools: Point, range, nearest queries)
│       ├── update_plugin.rs        (~6 tools: Insert, remove, drain)
│       ├── iteration_plugin.rs     (~4 tools: Iter, filter)
│       └── workflow_plugin.rs      (~8 tools: Common UI patterns)
└── tests/
    └── rstar_test.rs
```

### Cargo.toml:

```toml
[package]
name = "elicit_rstar"
version.workspace = true
edition.workspace = true
license.workspace = true
repository.workspace = true
authors.workspace = true
homepage.workspace = true
documentation.workspace = true
readme = "README.md"
description = "Elicitation-enabled rstar wrappers with MCP tools for spatial indexing"
keywords = ["mcp", "rstar", "rtree", "spatial", "elicitation"]
categories = ["data-structures", "algorithms", "development-tools"]

[dependencies]
elicitation = { workspace = true, features = ["rstar", "geo_types"] }
elicitation_derive.workspace = true
elicitation_macros.workspace = true
rstar = { workspace = true }
geo-types = { workspace = true }
elicit_geo_types = { workspace = true }
inventory.workspace = true
rmcp.workspace = true
schemars.workspace = true
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
tracing.workspace = true
accesskit = { workspace = true }  # For NodeId in IndexedElement

# Code emission
proc-macro2 = { workspace = true, optional = true }
quote = { workspace = true, optional = true }

[features]
emit = ["dep:proc-macro2", "dep:quote", "elicitation/emit"]

[lints.rust]
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(kani)', 'cfg(creusot)', 'cfg(prusti)', 'cfg(verus)'] }
```

### lib.rs structure:

```rust
//! `elicit_rstar` — R*-tree spatial indexing via MCP tools.
//!
//! Provides efficient spatial queries for large UI layouts:
//! - Point queries (find element at cursor)
//! - Range queries (find elements in viewport)
//! - Nearest neighbor (find closest element)
//! - Bulk operations (insert/remove many elements)
//!
//! # Performance
//!
//! Spatial indexing provides O(log n) queries instead of O(n):
//! - 1000 elements: ~10 comparisons vs ~1000
//! - 10000 elements: ~13 comparisons vs ~10000
//!
//! # Plugin Organization (5 plugins, ~34 total tools)
//!
//! | Plugin | Tools | Coverage |
//! |--------|-------|----------|
//! | `RStarConstructionPlugin` | 6 | Create, bulk_load trees |
//! | `RStarQueryPlugin` | 10 | Point, range, nearest queries |
//! | `RStarUpdatePlugin` | 6 | Insert, remove, drain |
//! | `RStarIterationPlugin` | 4 | Iter, filter elements |
//! | `RStarWorkflowPlugin` | 8 | Common UI patterns |

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod indexed_element;
pub mod workflow;

pub use indexed_element::IndexedElement;
pub use workflow::{
    RStarConstructionPlugin, RStarIterationPlugin, RStarQueryPlugin,
    RStarUpdatePlugin, RStarWorkflowPlugin,
};
```

## Phase 4: Implement Core Type Wrappers

### 4.1 IndexedElement (indexed_element.rs):

```rust
use accesskit::NodeId;
use geo_types::Rect;
use rstar::{RTreeObject, AABB};

/// UI element stored in spatial index.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexedElement {
    /// AccessKit node ID
    pub id: NodeId,
    /// Element bounding box
    pub bounds: Rect<f64>,
}

impl IndexedElement {
    /// Create a new indexed element.
    pub fn new(id: NodeId, bounds: Rect<f64>) -> Self {
        Self { id, bounds }
    }
}

impl RTreeObject for IndexedElement {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let min = self.bounds.min();
        let max = self.bounds.max();
        AABB::from_corners(
            [min.x, min.y],
            [max.x, max.y]
        )
    }
}

// Implement PointDistance for nearest neighbor queries
impl rstar::PointDistance for IndexedElement {
    fn distance_2(&self, point: &[f64; 2]) -> f64 {
        let rect_point = geo_types::Point::new(point[0], point[1]);
        let center = self.bounds.center();

        let dx = (center.x() - rect_point.x()).abs();
        let dy = (center.y() - rect_point.y()).abs();
        dx * dx + dy * dy
    }
}
```

## Phase 5: Implement MCP Tools

### 5.1 Construction Plugin (workflow/construction_plugin.rs):

```rust
use elicitation_derive::elicit_tool;
use rmcp::{CallToolResult, Content, ErrorData};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTreeParams {
    pub initial_capacity: Option<usize>,
}

#[elicit_tool(
    plugin = "rstar_construction",
    name = "rstar_construction__create_empty",
    description = "Create an empty R*-tree with optional initial capacity.",
    emit = Auto
)]
async fn construction_create_empty(p: CreateTreeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Created empty R*-tree with capacity {:?}",
            p.initial_capacity))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BulkLoadParams {
    pub elements: Vec<serde_json::Value>,  // Array of {id, bounds}
}

#[elicit_tool(
    plugin = "rstar_construction",
    name = "rstar_construction__bulk_load",
    description = "Bulk load elements into R*-tree (more efficient than sequential inserts).",
    emit = Auto
)]
async fn construction_bulk_load(p: BulkLoadParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Bulk loaded {} elements into R*-tree",
            p.elements.len()))
    ]))
}

// ... 4 more tools: from_layout, from_nodes, etc.
```

### 5.2 Query Plugin (workflow/query_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocateAtPointParams {
    pub x: f64,
    pub y: f64,
}

#[elicit_tool(
    plugin = "rstar_query",
    name = "rstar_query__locate_at_point",
    description = "Find the first element containing the point (x, y). O(log n) query.",
    emit = Auto
)]
async fn query_locate_at_point(p: LocateAtPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Located element at ({}, {})", p.x, p.y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocateAllAtPointParams {
    pub x: f64,
    pub y: f64,
}

#[elicit_tool(
    plugin = "rstar_query",
    name = "rstar_query__locate_all_at_point",
    description = "Find ALL elements containing the point (handles overlapping elements).",
    emit = Auto
)]
async fn query_locate_all_at_point(p: LocateAllAtPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Located all elements at ({}, {})", p.x, p.y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LocateInEnvelopeParams {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

#[elicit_tool(
    plugin = "rstar_query",
    name = "rstar_query__locate_in_envelope",
    description = "Find all elements intersecting the bounding box (range query). \
                   Efficient viewport culling.",
    emit = Auto
)]
async fn query_locate_in_envelope(p: LocateInEnvelopeParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Located elements in envelope ({}, {}) to ({}, {})",
            p.min_x, p.min_y, p.max_x, p.max_y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NearestNeighborParams {
    pub x: f64,
    pub y: f64,
}

#[elicit_tool(
    plugin = "rstar_query",
    name = "rstar_query__nearest_neighbor",
    description = "Find the nearest element to point (x, y). O(log n) query.",
    emit = Auto
)]
async fn query_nearest_neighbor(p: NearestNeighborParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Found nearest neighbor to ({}, {})", p.x, p.y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct WithinDistanceParams {
    pub x: f64,
    pub y: f64,
    pub max_distance: f64,
}

#[elicit_tool(
    plugin = "rstar_query",
    name = "rstar_query__locate_within_distance",
    description = "Find all elements within max_distance of point (x, y).",
    emit = Auto
)]
async fn query_within_distance(p: WithinDistanceParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Located elements within {} of ({}, {})",
            p.max_distance, p.x, p.y))
    ]))
}

// ... 5 more tools: k_nearest_neighbors, locate_within_rect, etc.
```

### 5.3 Update Plugin (workflow/update_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct InsertParams {
    pub element: serde_json::Value,  // {id, bounds}
}

#[elicit_tool(
    plugin = "rstar_update",
    name = "rstar_update__insert",
    description = "Insert an element into the R*-tree.",
    emit = Auto
)]
async fn update_insert(p: InsertParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("Inserted element into R*-tree")
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveParams {
    pub element: serde_json::Value,
}

#[elicit_tool(
    plugin = "rstar_update",
    name = "rstar_update__remove",
    description = "Remove an element from the R*-tree.",
    emit = Auto
)]
async fn update_remove(p: RemoveParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text("Removed element from R*-tree")
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RemoveAtPointParams {
    pub x: f64,
    pub y: f64,
}

#[elicit_tool(
    plugin = "rstar_update",
    name = "rstar_update__remove_at_point",
    description = "Remove the first element at point (x, y).",
    emit = Auto
)]
async fn update_remove_at_point(p: RemoveAtPointParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Removed element at ({}, {})", p.x, p.y))
    ]))
}

// ... 3 more tools: drain, clear, rebuild, etc.
```

### 5.4 Workflow Plugin (workflow/workflow_plugin.rs):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct FindElementAtCursorParams {
    pub cursor_x: f64,
    pub cursor_y: f64,
    pub layout_nodes: serde_json::Value,
}

#[elicit_tool(
    plugin = "rstar_workflow",
    name = "rstar_workflow__find_element_at_cursor",
    description = "Complete workflow: Build R*-tree from layout, find element at cursor position.",
    emit = Auto
)]
async fn workflow_find_element_at_cursor(p: FindElementAtCursorParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Found element at cursor ({}, {})", p.cursor_x, p.cursor_y))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ViewportCullingParams {
    pub viewport_x0: f64,
    pub viewport_y0: f64,
    pub viewport_x1: f64,
    pub viewport_y1: f64,
}

#[elicit_tool(
    plugin = "rstar_workflow",
    name = "rstar_workflow__viewport_culling",
    description = "Find all elements visible in viewport (efficient rendering).",
    emit = Auto
)]
async fn workflow_viewport_culling(p: ViewportCullingParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Found elements in viewport ({}, {}) to ({}, {})",
            p.viewport_x0, p.viewport_y0, p.viewport_x1, p.viewport_y1))
    ]))
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HoverDetectionParams {
    pub mouse_x: f64,
    pub mouse_y: f64,
}

#[elicit_tool(
    plugin = "rstar_workflow",
    name = "rstar_workflow__hover_detection",
    description = "Detect which element is being hovered (for tooltips, highlighting).",
    emit = Auto
)]
async fn workflow_hover_detection(p: HoverDetectionParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(vec![
        Content::text(format!("Detected hover at ({}, {})", p.mouse_x, p.mouse_y))
    ]))
}

// ... 5 more tools: drag_drop_target, snap_to_grid, collision_detection, etc.
```

## Phase 6: Testing

### File to create:
- `crates/elicit_rstar/tests/rstar_test.rs`

### Test Coverage:

```rust
use rstar::{RTree, AABB};
use geo_types::{Point, Rect, Coord};

#[test]
fn test_point_query() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(10.0, 10.0),
        Point::new(20.0, 20.0),
    ];

    let tree = RTree::bulk_load(points);

    let nearest = tree.nearest_neighbor(&[5.0, 5.0]).unwrap();
    assert_eq!(*nearest, Point::new(10.0, 10.0));
}

#[test]
fn test_range_query() {
    let rects = vec![
        Rect::new(Coord { x: 0.0, y: 0.0 }, Coord { x: 10.0, y: 10.0 }),
        Rect::new(Coord { x: 20.0, y: 20.0 }, Coord { x: 30.0, y: 30.0 }),
        Rect::new(Coord { x: 5.0, y: 5.0 }, Coord { x: 15.0, y: 15.0 }),
    ];

    let tree = RTree::bulk_load(rects);

    let envelope = AABB::from_corners([0.0, 0.0], [12.0, 12.0]);
    let results: Vec<_> = tree.locate_in_envelope(&envelope).collect();

    assert_eq!(results.len(), 2);  // First and third rects
}

#[test]
fn test_indexed_element() {
    use crate::IndexedElement;
    use accesskit::NodeId;

    let elements = vec![
        IndexedElement::new(
            NodeId(1),
            Rect::new(Coord { x: 0.0, y: 0.0 }, Coord { x: 100.0, y: 50.0 })
        ),
        IndexedElement::new(
            NodeId(2),
            Rect::new(Coord { x: 120.0, y: 10.0 }, Coord { x: 220.0, y: 60.0 })
        ),
    ];

    let tree = RTree::bulk_load(elements);

    let point = [50.0, 25.0];
    let result = tree.locate_at_point(&point).unwrap();
    assert_eq!(result.id, NodeId(1));
}
```

## Phase 7: Documentation

### File to create:
- `crates/elicit_rstar/README.md`

### Content:

```markdown
# elicit_rstar

Elicitation-enabled wrappers around [`rstar`](https://docs.rs/rstar) for R*-tree spatial indexing.

## Purpose

Provides **efficient spatial indexing** for UI layouts:
- O(log n) point queries instead of O(n) linear scan
- Efficient viewport culling
- Nearest neighbor search
- Range queries

## Performance

| Elements | Linear Scan | R*-tree |
|----------|-------------|---------|
| 100 | ~100 checks | ~7 checks |
| 1,000 | ~1,000 checks | ~10 checks |
| 10,000 | ~10,000 checks | ~13 checks |

## API Coverage

| Plugin | Tools | Coverage |
|--------|-------|----------|
| `rstar_construction` | 6 | Create, bulk_load trees |
| `rstar_query` | 10 | Point, range, nearest queries |
| `rstar_update` | 6 | Insert, remove, drain |
| `rstar_iteration` | 4 | Iter, filter elements |
| `rstar_workflow` | 8 | Common UI patterns |

**Total: ~34 MCP tools**

## Usage

```rust
use rstar::RTree;
use elicit_rstar::IndexedElement;
use accesskit::NodeId;
use geo_types::Rect;

// Create indexed elements
let elements = vec![
    IndexedElement::new(NodeId(1), button_bounds),
    IndexedElement::new(NodeId(2), label_bounds),
    // ... thousands more
];

// Bulk load into R*-tree
let tree = RTree::bulk_load(elements);

// O(log n) point query
let cursor_pos = [150.0, 75.0];
if let Some(element) = tree.locate_at_point(&cursor_pos) {
    println!("Found element: {:?}", element.id);
}

// Range query (viewport culling)
let viewport = AABB::from_corners([0.0, 0.0], [1920.0, 1080.0]);
let visible: Vec<_> = tree.locate_in_envelope(&viewport).collect();
```

## Integration with elicit_ui

Add spatial indexing to verified layouts:

```rust
use elicit_ui::Layout;
use elicit_rstar::IndexedElement;
use rstar::RTree;

let layout = Layout::from_update(update);
let verified = layout.verify_aa(viewport)?;

// Build spatial index from verified layout
let elements: Vec<IndexedElement> = verified.nodes()
    .filter_map(|(id, node)| {
        node.bounds().map(|bounds| IndexedElement::new(id, bounds))
    })
    .collect();

let tree = RTree::bulk_load(elements);

// Now queries are O(log n) instead of O(n)
let element_at_cursor = tree.locate_at_point(&cursor_pos);
```

## Use Cases

**Click Detection**:
```rust
// O(log n) - find element under mouse
let clicked = tree.locate_at_point(&mouse_pos);
```

**Viewport Culling**:
```rust
// Only render visible elements
let visible = tree.locate_in_envelope(&viewport_aabb);
for element in visible {
    render(element);
}
```

**Hover Effects**:
```rust
// Find element under cursor for highlighting
let hovered = tree.locate_at_point(&cursor_pos);
```

**Drag and Drop**:
```rust
// Find drop target
let drop_target = tree.locate_at_point(&drop_pos);
```
```

## Verification Steps

**After implementation**:
1. `cargo check -p elicit_rstar`
2. `cargo test -p elicit_rstar`
3. `cargo check -p elicitation --no-default-features --features rstar`
4. `cargo check --all-features`

**Performance verification**:
1. Create layout with 1000 elements
2. Benchmark linear scan vs R*-tree
3. Verify O(log n) scaling

## Critical Files

### To create:
- `crates/elicit_rstar/Cargo.toml`
- `crates/elicit_rstar/README.md`
- `crates/elicit_rstar/src/lib.rs`
- `crates/elicit_rstar/src/indexed_element.rs`
- `crates/elicit_rstar/src/workflow/mod.rs`
- `crates/elicit_rstar/src/workflow/construction_plugin.rs`
- `crates/elicit_rstar/src/workflow/query_plugin.rs`
- `crates/elicit_rstar/src/workflow/update_plugin.rs`
- `crates/elicit_rstar/src/workflow/iteration_plugin.rs`
- `crates/elicit_rstar/src/workflow/workflow_plugin.rs`
- `crates/elicit_rstar/tests/rstar_test.rs`
- `crates/elicitation/src/rstar_support.rs`

### To modify:
- `Cargo.toml`
- `crates/elicitation/Cargo.toml`
- `crates/elicitation/src/lib.rs`

## Implementation Order

1. **Phase 1**: Workspace configuration (15 min)
2. **Phase 2**: Core type integration (30 min)
3. **Phase 3**: Create elicit_rstar structure (30 min)
4. **Phase 4**: Implement IndexedElement wrapper (1 hour)
5. **Phase 5**: Implement MCP tools (~34 tools) (5-7 hours)
6. **Phase 6**: Testing + benchmarks (1-2 hours)
7. **Phase 7**: Documentation (30 min)

**Total estimated time**: 9-12 hours

## Notes

### R*-tree Algorithm

The R*-tree is an optimized variant of R-tree:
- Better split heuristics than basic R-tree
- Forced reinsertion improves tree quality
- Generally better query performance

### When to Use Spatial Indexing

**Use R*-tree when**:
- Layout has >100 elements
- Frequent point/range queries
- Interactive applications (mouse hover, clicks)
- Real-time rendering with viewport culling

**Linear scan is fine when**:
- Small layouts (<50 elements)
- Rare queries
- Static layouts

### Memory Overhead

R*-tree has ~2-3x memory overhead vs flat array:
- 1000 elements: ~100KB tree structure
- Worth it for query performance gain
