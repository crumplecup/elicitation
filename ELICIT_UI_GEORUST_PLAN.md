# ELICIT_UI_GEORUST_PLAN.md

## Goal
Integrate GeoRust as the spatial calculation engine for elicit_ui layout verification.

## Architectural Insight

UI layout is fundamentally a **2D spatial problem**:
- **AccessKit** provides the tree structure (parent/child, roles, semantics)
- **GeoRust** provides the spatial calculations (bounds, containment, intersection, distance)

All WCAG spatial constraints become geometric predicates:
- Label inside button bounds? → `button_rect.contains(label_point)`
- Element fits in viewport? → `viewport_rect.contains(element_rect)`
- Minimum touch target? → `rect.area() >= 44.0 * 44.0`
- Elements don't overlap? → `!rect1.intersects(&rect2)`
- Proper spacing? → `rect1.euclidean_distance(&rect2) >= min_spacing`

## GeoRust Ecosystem

**Core crates**:
- **geo-types** (0.7): Core primitives (`Point`, `Rect`, `Polygon`, `Coord`, `LineString`)
- **geo** (0.28): Geometric algorithms (`Contains`, `Intersects`, `Area`, `EuclideanDistance`, `ConvexHull`)

**Mapping to UI**:
```rust
// UI element bounds
type ElementBounds = geo::Rect<f64>;
type ElementPosition = geo::Point<f64>;

// Viewport as bounding rectangle
type Viewport = geo::Rect<f64>;

// Spatial queries
button_bounds.contains(&label_position)  // Label inside button?
viewport.contains(&element_bounds)        // Fits in viewport?
element_bounds.unsigned_area() >= 1936.0  // Meets 44x44 target?
```

## Phase 1: Add GeoRust Dependencies

### Files to modify:
- `Cargo.toml` (workspace root)
- `crates/elicit_ui/Cargo.toml`

### Changes:

**1.1 Add geo crates to workspace dependencies**:
```toml
# Spatial calculations
geo = "0.28"
geo-types = "0.7"
```

**1.2 Add to elicit_ui dependencies**:
```toml
geo = { workspace = true }
geo-types = { workspace = true }
```

## Phase 2: Replace Custom Types with GeoTypes

### Current elicit_ui types to replace:

**Before** (`types.rs`):
```rust
#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}
```

**After** (`types.rs`):
```rust
use geo::{Point, Rect};
use geo_types::Coord;

/// UI element bounds in pixel space.
pub type ElementBounds = Rect<f64>;

/// UI element position in pixel space.
pub type ElementPosition = Point<f64>;

/// Viewport as bounding rectangle.
pub type Viewport = Rect<f64>;

/// Helper to create viewport from dimensions.
impl Viewport {
    pub fn from_dimensions(width: u32, height: u32) -> Self {
        Rect::new(
            Coord { x: 0.0, y: 0.0 },
            Coord { x: width as f64, y: height as f64 }
        )
    }
}
```

## Phase 3: Update AccessKit Bridge

### File to modify:
- `crates/elicit_ui/src/accesskit_bridge.rs` (new file)

### Purpose:
Convert AccessKit bounds to GeoRust primitives.

```rust
//! Bridge between AccessKit and GeoRust spatial types.

use accesskit::{Node, Rect as AccessKitRect};
use geo::{Point, Rect};
use geo_types::Coord;

/// Convert AccessKit Rect to geo::Rect.
pub fn accesskit_to_georect(ak_rect: &AccessKitRect) -> Rect<f64> {
    Rect::new(
        Coord { x: ak_rect.x0, y: ak_rect.y0 },
        Coord { x: ak_rect.x1, y: ak_rect.y1 }
    )
}

/// Convert AccessKit Rect to geo::Point (center).
pub fn accesskit_to_geopoint(ak_rect: &AccessKitRect) -> Point<f64> {
    Point::new(
        (ak_rect.x0 + ak_rect.x1) / 2.0,
        (ak_rect.y0 + ak_rect.y1) / 2.0
    )
}

/// Get element bounds from AccessKit Node.
pub fn element_bounds(node: &Node) -> Option<Rect<f64>> {
    node.bounds().map(accesskit_to_georect)
}

/// Get element center position from AccessKit Node.
pub fn element_position(node: &Node) -> Option<Point<f64>> {
    node.bounds().map(accesskit_to_geopoint)
}
```

## Phase 4: Rewrite Validators with GeoRust

### File to modify:
- `crates/elicit_ui/src/validators.rs`

### Before (custom implementation):
```rust
pub fn validate_no_overflow(
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
    viewport: Viewport,
) -> Result<(), VerificationError> {
    let node = nodes.get(&node_id)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if let Some(bounds) = node.bounds() {
        // Manual bounds checking
        if bounds.x1 > viewport.width as f64 || bounds.y1 > viewport.height as f64 {
            return Err(VerificationError::new(/* ... */));
        }
    }
    Ok(())
}
```

### After (GeoRust predicates):
```rust
use geo::Contains;
use crate::accesskit_bridge::element_bounds;

pub fn validate_no_overflow(
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
    viewport: Rect<f64>,
) -> Result<(), VerificationError> {
    let node = nodes.get(&node_id)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if let Some(bounds) = element_bounds(node) {
        // Geometric predicate: viewport contains element bounds
        if !viewport.contains(&bounds) {
            return Err(VerificationError::new(
                VerificationErrorKind::OverflowsViewport(node_id, bounds, viewport)
            ));
        }
    }
    Ok(())
}
```

### Complete validator rewrites:

**MinTargetSize validator**:
```rust
use geo::Area;

pub fn validate_min_target_size(
    nodes: &HashMap<NodeId, Node>,
    node_id: NodeId,
) -> Result<(), VerificationError> {
    let node = nodes.get(&node_id)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if !node.role().is_interactive() {
        return Ok(());
    }

    if let Some(bounds) = element_bounds(node) {
        const MIN_AREA: f64 = 44.0 * 44.0;  // WCAG 2.5.5 Level AAA

        if bounds.unsigned_area() < MIN_AREA {
            return Err(VerificationError::new(
                VerificationErrorKind::BelowMinTargetSize(node_id, bounds)
            ));
        }
    }
    Ok(())
}
```

**HasLabel validator (label position inside element)**:
```rust
use geo::Contains;

pub fn validate_label_position(
    nodes: &HashMap<NodeId, Node>,
    element_id: NodeId,
    label_id: NodeId,
) -> Result<(), VerificationError> {
    let element = nodes.get(&element_id)
        .ok_or_else(|| VerificationError::new(/* ... */))?;
    let label = nodes.get(&label_id)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if let (Some(element_bounds), Some(label_pos)) =
        (element_bounds(element), element_position(label)) {

        // Geometric predicate: element contains label position
        if !element_bounds.contains(&label_pos) {
            return Err(VerificationError::new(
                VerificationErrorKind::LabelOutsideElement(element_id, label_id)
            ));
        }
    }
    Ok(())
}
```

**NoOverlap validator** (new - elements don't intersect):
```rust
use geo::Intersects;

pub fn validate_no_overlap(
    nodes: &HashMap<NodeId, Node>,
    node_id_1: NodeId,
    node_id_2: NodeId,
) -> Result<(), VerificationError> {
    let node1 = nodes.get(&node_id_1)
        .ok_or_else(|| VerificationError::new(/* ... */))?;
    let node2 = nodes.get(&node_id_2)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if let (Some(bounds1), Some(bounds2)) =
        (element_bounds(node1), element_bounds(node2)) {

        // Geometric predicate: bounds don't intersect
        if bounds1.intersects(&bounds2) {
            return Err(VerificationError::new(
                VerificationErrorKind::ElementsOverlap(node_id_1, node_id_2, bounds1, bounds2)
            ));
        }
    }
    Ok(())
}
```

**MinSpacing validator** (new - proper element spacing):
```rust
use geo::EuclideanDistance;

pub fn validate_min_spacing(
    nodes: &HashMap<NodeId, Node>,
    node_id_1: NodeId,
    node_id_2: NodeId,
    min_spacing: f64,
) -> Result<(), VerificationError> {
    let node1 = nodes.get(&node_id_1)
        .ok_or_else(|| VerificationError::new(/* ... */))?;
    let node2 = nodes.get(&node_id_2)
        .ok_or_else(|| VerificationError::new(/* ... */))?;

    if let (Some(bounds1), Some(bounds2)) =
        (element_bounds(node1), element_bounds(node2)) {

        // Geometric predicate: distance between bounds >= min spacing
        let distance = bounds1.euclidean_distance(&bounds2);
        if distance < min_spacing {
            return Err(VerificationError::new(
                VerificationErrorKind::InsufficientSpacing(
                    node_id_1, node_id_2, distance, min_spacing
                )
            ));
        }
    }
    Ok(())
}
```

## Phase 5: Add Spatial Contracts

### File to modify:
- `crates/elicit_ui/src/contracts.rs`

### New spatial contracts using GeoRust:

```rust
use geo::{Area, Contains, EuclideanDistance, Intersects};

/// Element fits within viewport bounds.
///
/// WCAG 1.4.10 Level AA: Reflow
#[derive(Debug, Clone)]
pub struct FitsViewport<T> {
    element_bounds: Rect<f64>,
    viewport: Rect<f64>,
    _phantom: PhantomData<T>,
}

impl<T> FitsViewport<T> {
    #[cfg(feature = "proofs")]
    pub fn prove(element: Rect<f64>, viewport: Rect<f64>) -> Result<Self, String> {
        if viewport.contains(&element) {
            Ok(Self {
                element_bounds: element,
                viewport,
                _phantom: PhantomData,
            })
        } else {
            Err(format!("Element {:?} overflows viewport {:?}", element, viewport))
        }
    }
}

/// Elements don't overlap (no visual occlusion).
#[derive(Debug, Clone)]
pub struct NoOverlap<T1, T2> {
    bounds1: Rect<f64>,
    bounds2: Rect<f64>,
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2> NoOverlap<T1, T2> {
    #[cfg(feature = "proofs")]
    pub fn prove(el1: Rect<f64>, el2: Rect<f64>) -> Result<Self, String> {
        if !el1.intersects(&el2) {
            Ok(Self {
                bounds1: el1,
                bounds2: el2,
                _phantom: PhantomData,
            })
        } else {
            Err(format!("Elements overlap: {:?} ∩ {:?}", el1, el2))
        }
    }
}

/// Element meets minimum area requirements.
///
/// WCAG 2.5.5 Level AAA: Target Size (Enhanced)
#[derive(Debug, Clone)]
pub struct MinArea<T> {
    bounds: Rect<f64>,
    min_area: f64,
    _phantom: PhantomData<T>,
}

impl<T> MinArea<T> {
    #[cfg(feature = "proofs")]
    pub fn prove(element: Rect<f64>, min_area: f64) -> Result<Self, String> {
        let area = element.unsigned_area();
        if area >= min_area {
            Ok(Self {
                bounds: element,
                min_area,
                _phantom: PhantomData,
            })
        } else {
            Err(format!("Element area {} < required {}", area, min_area))
        }
    }
}

/// Elements properly spaced (accessibility, readability).
///
/// WCAG 1.4.12 Level AA: Text Spacing
#[derive(Debug, Clone)]
pub struct ProperSpacing<T1, T2> {
    bounds1: Rect<f64>,
    bounds2: Rect<f64>,
    min_spacing: f64,
    _phantom: PhantomData<(T1, T2)>,
}

impl<T1, T2> ProperSpacing<T1, T2> {
    #[cfg(feature = "proofs")]
    pub fn prove(el1: Rect<f64>, el2: Rect<f64>, min: f64) -> Result<Self, String> {
        let distance = el1.euclidean_distance(&el2);
        if distance >= min {
            Ok(Self {
                bounds1: el1,
                bounds2: el2,
                min_spacing: min,
                _phantom: PhantomData,
            })
        } else {
            Err(format!("Spacing {} < required {}", distance, min))
        }
    }
}
```

## Phase 6: Update Tests

### Files to modify:
- `crates/elicit_ui/tests/button_verification_test.rs`
- `crates/elicit_ui/tests/spatial_test.rs` (new)

### Viewport creation:
```rust
// Before
let viewport = Viewport::new(1920, 1080);

// After
let viewport = Viewport::from_dimensions(1920, 1080);
```

### New spatial tests:
```rust
//! Test spatial validation with GeoRust.

use elicit_ui::{Layout, Viewport, validators};
use accesskit::{Node, NodeId, Role, Tree, TreeId, TreeUpdate};
use geo::Rect;

#[test]
fn test_elements_dont_overlap() {
    // Create two buttons side-by-side
    let button1 = create_button_at(10.0, 10.0, 100.0, 50.0);
    let button2 = create_button_at(120.0, 10.0, 100.0, 50.0);

    let result = validators::validate_no_overlap(&nodes, button1_id, button2_id);
    assert!(result.is_ok(), "Side-by-side buttons should not overlap");
}

#[test]
fn test_elements_overlap() {
    // Create two buttons that overlap
    let button1 = create_button_at(10.0, 10.0, 100.0, 50.0);
    let button2 = create_button_at(50.0, 20.0, 100.0, 50.0);

    let result = validators::validate_no_overlap(&nodes, button1_id, button2_id);
    assert!(result.is_err(), "Overlapping buttons should fail validation");
}

#[test]
fn test_proper_spacing() {
    // Create buttons with proper spacing
    let button1 = create_button_at(10.0, 10.0, 100.0, 50.0);
    let button2 = create_button_at(120.0, 10.0, 100.0, 50.0);

    let min_spacing = 8.0;  // 8px minimum spacing
    let result = validators::validate_min_spacing(&nodes, button1_id, button2_id, min_spacing);
    assert!(result.is_ok(), "Properly spaced buttons should pass");
}

#[test]
fn test_label_inside_button() {
    // Create button with label inside bounds
    let button = create_button_with_label_at(10.0, 10.0, 100.0, 50.0);

    let result = validators::validate_label_position(&nodes, button_id, label_id);
    assert!(result.is_ok(), "Label inside button bounds should pass");
}
```

## Phase 7: Documentation

### File to modify:
- `crates/elicit_ui/README.md`

### Add section:

```markdown
## Spatial Calculations with GeoRust

elicit_ui uses the GeoRust ecosystem to handle all spatial calculations:

### Architecture

- **AccessKit**: Provides UI tree structure (parent/child, roles, semantics)
- **GeoRust**: Provides spatial calculations (bounds, containment, distance)

All WCAG spatial constraints map to geometric predicates:

```rust
use geo::{Contains, Intersects, Area, EuclideanDistance};

// Element fits in viewport?
viewport.contains(&element_bounds)

// Elements don't overlap?
!element1.intersects(&element2)

// Meets minimum touch target?
element_bounds.unsigned_area() >= 44.0 * 44.0

// Proper spacing?
element1.euclidean_distance(&element2) >= min_spacing
```

### Benefits

1. **Proven algorithms**: GeoRust is extensively tested for spatial operations
2. **Natural mapping**: UI layout is 2D geometry
3. **Expressive**: Contains, intersects, distance, area cover all UI spatial needs
4. **Verifiable**: Geometric predicates can be formally proven
```

## Verification Steps

### After implementation:

1. `cargo check -p elicit_ui`
2. `cargo test -p elicit_ui`
3. `cargo test -p elicit_ui --all-features`

### Manual verification:

1. Create layout with overlapping elements → verify detection
2. Create layout with elements outside viewport → verify detection
3. Create layout with proper spacing → verify acceptance
4. Check that all validators use GeoRust predicates

## Critical Files

### To create:
- `crates/elicit_ui/src/accesskit_bridge.rs`
- `crates/elicit_ui/tests/spatial_test.rs`

### To modify:
- `Cargo.toml` — Add geo, geo-types workspace dependencies
- `crates/elicit_ui/Cargo.toml` — Add geo dependencies
- `crates/elicit_ui/src/types.rs` — Replace custom types with geo types
- `crates/elicit_ui/src/validators.rs` — Rewrite with GeoRust predicates
- `crates/elicit_ui/src/contracts.rs` — Add spatial contracts
- `crates/elicit_ui/src/lib.rs` — Export accesskit_bridge module
- `crates/elicit_ui/tests/button_verification_test.rs` — Update viewport creation
- `crates/elicit_ui/README.md` — Document GeoRust integration

## Implementation Order

1. **Phase 1**: Add GeoRust dependencies (10 min)
2. **Phase 2**: Replace custom types with geo types (30 min)
3. **Phase 3**: Create AccessKit bridge (30 min)
4. **Phase 4**: Rewrite validators with GeoRust (1-2 hours)
5. **Phase 5**: Add spatial contracts (1 hour)
6. **Phase 6**: Update tests (30 min)
7. **Phase 7**: Documentation (20 min)

**Total estimated time**: 4-5 hours

## Notes

### Why GeoRust?

UI layout is fundamentally spatial geometry in 2D coordinate space:
- Every element has bounds (Rectangle)
- Labels have positions (Point)
- Viewport is a bounding rectangle
- All WCAG spatial constraints are geometric predicates

### Benefits

1. **Correctness**: Proven geometric algorithms
2. **Expressiveness**: Natural vocabulary for spatial relationships
3. **Performance**: Optimized spatial calculations
4. **Verifiability**: Geometric proofs map to formal verification
5. **Consistency**: Single source of truth for all spatial logic

### GeoRust Predicates Used

- `Contains<T>`: Element inside viewport, label inside element
- `Intersects<T>`: Elements overlap detection
- `Area`: Minimum touch target size
- `EuclideanDistance`: Element spacing
- `ConvexHull`: Bounding box calculations (future)
- `Centroid`: Element center positioning (future)

### Future Spatial Features

With GeoRust foundation:
- **Layout algorithms**: Auto-spacing, grid layouts, flex layouts
- **Collision detection**: Drag-and-drop validation
- **Responsive layouts**: Viewport-aware element repositioning
- **Accessibility paths**: Shortest path for keyboard navigation
- **Visual density**: Area coverage analysis
