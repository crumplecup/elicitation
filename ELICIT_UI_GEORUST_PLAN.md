# ELICIT_UI_GEORUST_PLAN.md

## Goal
Integrate GeoRust as the spatial calculation engine for elicit_ui layout verification using a **composable, spec-backed constraint architecture**.

## Architectural Insight

UI layout is fundamentally a **2D spatial problem**:
- **AccessKit** provides the tree structure (parent/child, roles, semantics)
- **GeoRust** provides the spatial calculations (bounds, containment, intersection, distance)
- **Constraint architecture** provides composable, externally-grounded verification

All WCAG spatial constraints become geometric predicates:
- Label inside button bounds? → `button_rect.contains(label_point)`
- Element fits in viewport? → `viewport_rect.contains(element_rect)`
- Minimum touch target? → `rect.unsigned_area() >= 1936.0`
- Elements don't overlap? → `!rect1.intersects(&rect2)`
- Proper spacing? → `rect1.euclidean_distance(&rect2) >= min_spacing`

## Constraint Architecture

Following a **spec-backed constraint stack** to eliminate arbitrary decisions and enable proof-driven UI validation:

| Layer | Authority | Role | Implementation |
|-------|-----------|------|----------------|
| **L1** | WCAG 2.2 | Hard accessibility constraints | `geo::Contains`, `geo::Area`, `geo::Intersects` |
| **L2** | CSS/W3C | Unit semantics, scaling invariants | Type-safe CSS units |
| **L3** | Design Systems | Ergonomic heuristics (optional) | Material/HIG profiles |
| **L4** | ISO 9241 | Perceptual grounding (future) | Visual angle calculations |

### Design Principles

1. **External grounding**: Every constraint traceable to recognized standard
2. **Composability**: Constraints combine additively
3. **Enforcement tiers**: Hard (must pass), structural (compile-time), advisory (warnings)
4. **No bikeshedding**: All decisions anchored in specs

### Layer 1: WCAG Constraints (Hard)

Map WCAG Success Criteria to geometric predicates - these are **non-negotiable invariants**:

```rust
/// Trait for spec-backed constraints.
pub trait Constraint {
    /// Check constraint against layout.
    fn check(&self, layout: &Layout) -> Result<(), Violation>;

    /// External specification reference.
    fn spec_ref(&self) -> SpecReference;
}

/// WCAG 1.4.10 Reflow (Level AA)
/// Content must reflow without horizontal scrolling at 320 CSS pixels.
pub struct Reflow320;

impl Constraint for Reflow320 {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        use geo::Contains;

        let viewport = Rect::new(
            coord! { x: 0.0, y: 0.0 },
            coord! { x: 320.0, y: f64::MAX },
        );

        // All content must fit in 320px width
        for element in &layout.elements {
            if !viewport.contains(&element.bounds) {
                return Err(Violation::Reflow {
                    wcag: "1.4.10",
                    level: "AA",
                    element: element.id,
                    width: element.bounds.width(),
                    max_width: 320.0,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.10",
            level: Level::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/reflow",
        }
    }
}

/// WCAG 2.5.5 Target Size (Level AAA)
/// Interactive elements must be at least 44×44 CSS pixels.
pub struct MinTouchTarget {
    pub min_area: f64,  // Default: 1936.0 (44×44)
}

impl Constraint for MinTouchTarget {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        use geo::Area;

        for element in layout.interactive_elements() {
            let area = element.bounds.unsigned_area();
            if area < self.min_area {
                return Err(Violation::TouchTarget {
                    wcag: "2.5.5",
                    level: "AAA",
                    element: element.id,
                    actual_area: area,
                    min_area: self.min_area,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "2.5.5",
            level: Level::AAA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/target-size-enhanced",
        }
    }
}

/// WCAG 1.4.12 Text Spacing (Level AA)
/// No content loss when spacing is increased.
pub struct TextSpacing;

impl Constraint for TextSpacing {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        use geo::Intersects;

        // Apply WCAG spacing overrides:
        // - Line height: 1.5× font size
        // - Paragraph spacing: 2× font size
        // - Letter spacing: 0.12× font size
        // - Word spacing: 0.16× font size
        let adjusted = layout.apply_wcag_spacing();

        // Check for overlaps after spacing increase
        for (i, el1) in adjusted.elements.iter().enumerate() {
            for el2 in &adjusted.elements[i+1..] {
                if el1.bounds.intersects(&el2.bounds) {
                    return Err(Violation::TextSpacing {
                        wcag: "1.4.12",
                        level: "AA",
                        element1: el1.id,
                        element2: el2.id,
                    });
                }
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.12",
            level: Level::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/text-spacing",
        }
    }
}

/// WCAG 1.4.3 Contrast (Minimum) (Level AA)
/// Text contrast ratio must be at least 4.5:1 (or 3:1 for large text).
pub struct ContrastMinimum;

impl Constraint for ContrastMinimum {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        for element in layout.text_elements() {
            let ratio = calculate_contrast_ratio(
                element.foreground_color,
                element.background_color,
            );

            let min_ratio = if element.is_large_text() { 3.0 } else { 4.5 };

            if ratio < min_ratio {
                return Err(Violation::Contrast {
                    wcag: "1.4.3",
                    level: "AA",
                    element: element.id,
                    actual_ratio: ratio,
                    min_ratio,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.3",
            level: Level::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/contrast-minimum",
        }
    }
}
```

### Layer 2: CSS Unit Semantics (Structural)

Use CSS specs to define **legal measurement and transformation model**:

```rust
/// CSS units (perceptual, not physical).
///
/// Reference: CSS Values and Units Module Level 3
/// https://www.w3.org/TR/css-values-3/
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum CssUnit {
    /// CSS pixel (reference pixel, ~0.26mm at arm's length)
    Px(f64),
    /// Root em (scalable relative to root font size)
    Rem(f64),
    /// Viewport width percentage
    Vw(f64),
    /// Viewport height percentage
    Vh(f64),
    /// Em (relative to current font size)
    Em(f64),
}

impl CssUnit {
    /// Scale by zoom factor (200% = 2.0).
    pub fn scale(&self, factor: f64) -> Self {
        match self {
            Self::Px(v) => Self::Px(v * factor),
            Self::Rem(v) => Self::Rem(v * factor),
            Self::Em(v) => Self::Em(v * factor),
            // Viewport units don't scale (they're already relative)
            Self::Vw(v) => Self::Vw(*v),
            Self::Vh(v) => Self::Vh(*v),
        }
    }

    /// Convert to absolute pixels given layout context.
    pub fn to_px(&self, context: &LayoutContext) -> f64 {
        match self {
            Self::Px(v) => *v,
            Self::Rem(v) => v * context.root_font_size,
            Self::Em(v) => v * context.current_font_size,
            Self::Vw(v) => v * context.viewport_width / 100.0,
            Self::Vh(v) => v * context.viewport_height / 100.0,
        }
    }

    /// Convert to geo_types Coord for geometric operations.
    pub fn to_coord(&self, context: &LayoutContext) -> Coord<f64> {
        let px = self.to_px(context);
        coord! { x: px, y: px }
    }
}

/// Layout context for CSS unit resolution.
#[derive(Debug, Clone)]
pub struct LayoutContext {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub root_font_size: f64,
    pub current_font_size: f64,
    pub zoom_level: f64,
}

impl LayoutContext {
    /// Create standard context (16px root font, 100% zoom).
    pub fn standard(viewport_width: f64, viewport_height: f64) -> Self {
        Self {
            viewport_width,
            viewport_height,
            root_font_size: 16.0,
            current_font_size: 16.0,
            zoom_level: 1.0,
        }
    }

    /// Apply zoom (WCAG 1.4.4: Resize Text up to 200%).
    pub fn with_zoom(&self, zoom: f64) -> Self {
        Self {
            zoom_level: zoom,
            ..self.clone()
        }
    }
}

/// Invariant: Layout must remain valid under zoom transformations.
pub struct ZoomInvariant;

impl Constraint for ZoomInvariant {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        // Test at 200% zoom (WCAG requirement)
        let zoomed_context = layout.context.with_zoom(2.0);
        let zoomed_layout = layout.apply_context(zoomed_context);

        // Re-check all hard constraints at 200% zoom
        for constraint in layout.hard_constraints() {
            constraint.check(&zoomed_layout)?;
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.4",
            level: Level::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/resize-text",
        }
    }
}
```

### Layer 3: Design System Profiles (Advisory)

Incorporate design systems as **constraint profiles**, not hard rules:

```rust
/// Design system constraint profiles.
#[derive(Debug, Clone)]
pub enum ConstraintProfile {
    /// Apple Human Interface Guidelines
    AppleHig,
    /// Material Design 3
    MaterialDesign,
    /// WCAG 2.2 Level AA (minimal)
    Wcag22Aa,
    /// WCAG 2.2 Level AAA (enhanced)
    Wcag22Aaa,
    /// Custom profile
    Custom(Vec<Box<dyn Constraint>>),
}

impl ConstraintProfile {
    /// Get constraints for this profile.
    pub fn constraints(&self) -> Vec<Box<dyn Constraint>> {
        match self {
            Self::AppleHig => vec![
                Box::new(MinTouchTarget { min_area: 44.0 * 44.0 }), // 44pt
                Box::new(GridAlignment { step: 8.0 }),
                Box::new(Reflow320),
                Box::new(ContrastMinimum),
                // HIG-specific heuristics...
            ],

            Self::MaterialDesign => vec![
                Box::new(MinTouchTarget { min_area: 48.0 * 48.0 }), // 48dp
                Box::new(GridAlignment { step: 8.0 }),
                Box::new(Reflow320),
                Box::new(ContrastMinimum),
                // Material-specific heuristics...
            ],

            Self::Wcag22Aa => vec![
                Box::new(Reflow320),
                Box::new(ResizeText200),
                Box::new(TextSpacing),
                Box::new(ContrastMinimum),
                // All Level AA criteria...
            ],

            Self::Wcag22Aaa => vec![
                Box::new(Reflow320),
                Box::new(ResizeText200),
                Box::new(TextSpacing),
                Box::new(ContrastEnhanced),
                Box::new(MinTouchTarget { min_area: 44.0 * 44.0 }),
                // All Level AAA criteria...
            ],

            Self::Custom(constraints) => constraints.clone(),
        }
    }

    /// Get breakpoint definitions for this profile.
    pub fn breakpoints(&self) -> BreakpointSet {
        match self {
            Self::AppleHig => BreakpointSet::hig(),
            Self::MaterialDesign => BreakpointSet::material(),
            _ => BreakpointSet::default(),
        }
    }
}

/// Example design system constraint: Grid alignment.
pub struct GridAlignment {
    pub step: f64,  // e.g., 8px grid
}

impl Constraint for GridAlignment {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        for element in &layout.elements {
            let (min_x, min_y) = element.bounds.min().x_y();

            if min_x % self.step != 0.0 || min_y % self.step != 0.0 {
                return Err(Violation::GridAlignment {
                    element: element.id,
                    position: (min_x, min_y),
                    grid_step: self.step,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::DesignSystem {
            name: "Material Design / HIG",
            section: "Layout Grid",
            url: "https://m3.material.io/foundations/layout/understanding-layout/spacing",
        }
    }
}
```

### Layer 4: ISO 9241 (Future Extension)

Perceptual grounding from ISO standards:

```rust
/// ISO 9241 visual angle constraint (future).
///
/// Reference: ISO 9241-3 Visual Display Requirements
pub struct MinVisualAngle {
    pub arc_minutes: f64,
}

impl Constraint for MinVisualAngle {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        // Visual angle = 2 × arctan(size / (2 × viewing_distance))
        // Typical viewing distance: 50-70cm for desktop, 30-40cm for mobile
        // Minimum legible angle: ~16 arc minutes

        // Derived: ~16px minimum for body text at typical viewing distance
        const MIN_BODY_TEXT_PX: f64 = 16.0;

        for element in layout.text_elements() {
            if element.font_size < MIN_BODY_TEXT_PX {
                return Err(Violation::VisualAngle {
                    element: element.id,
                    actual_size: element.font_size,
                    min_size: MIN_BODY_TEXT_PX,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Iso {
            standard: "ISO 9241-3",
            section: "Visual Display Requirements",
        }
    }
}
```

### Breakpoint Profiles (Not Invariants)

Breakpoints are **named profiles**, not hardcoded thresholds:

```rust
/// Breakpoint set (device size categories).
///
/// NOT a spec - multiple competing definitions exist.
/// Represented as named profiles, not universal truth.
#[derive(Debug, Clone)]
pub struct BreakpointSet {
    pub name: String,
    pub widths: Vec<f64>,
}

impl BreakpointSet {
    /// Material Design breakpoints.
    pub fn material() -> Self {
        Self {
            name: "Material Design 3".to_string(),
            widths: vec![600.0, 905.0, 1240.0, 1440.0],
        }
    }

    /// Apple HIG breakpoints.
    pub fn hig() -> Self {
        Self {
            name: "Apple HIG".to_string(),
            widths: vec![320.0, 375.0, 414.0, 768.0, 1024.0],
        }
    }

    /// Tailwind CSS breakpoints.
    pub fn tailwind() -> Self {
        Self {
            name: "Tailwind CSS".to_string(),
            widths: vec![640.0, 768.0, 1024.0, 1280.0, 1536.0],
        }
    }

    /// Bootstrap breakpoints.
    pub fn bootstrap() -> Self {
        Self {
            name: "Bootstrap 5".to_string(),
            widths: vec![576.0, 768.0, 992.0, 1200.0, 1400.0],
        }
    }

    /// Custom breakpoints.
    pub fn custom(name: impl Into<String>, widths: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            widths,
        }
    }
}

/// Test layout at multiple breakpoints.
pub fn verify_responsive(
    layout: &Layout,
    breakpoints: &BreakpointSet,
    constraints: &ConstraintSet,
) -> Result<(), Vec<Violation>> {
    let mut violations = Vec::new();

    for width in &breakpoints.widths {
        let viewport = Rect::new(
            coord! { x: 0.0, y: 0.0 },
            coord! { x: *width, y: f64::MAX },
        );

        let adjusted = layout.with_viewport(viewport);

        if let Err(v) = constraints.verify(&adjusted) {
            violations.push(v);
        }
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
```

### Constraint Composition

Constraints are **additive and composable**:

```rust
/// Composable constraint set with enforcement tiers.
pub struct ConstraintSet {
    /// Tier 1: Hard constraints (must pass for validity)
    pub hard: Vec<Box<dyn Constraint>>,

    /// Tier 2: Structural constraints (compile-time guarantees)
    pub structural: Vec<Box<dyn Constraint>>,

    /// Tier 3: Advisory constraints (warnings, not errors)
    pub advisory: Vec<Box<dyn Constraint>>,
}

impl ConstraintSet {
    pub fn builder() -> ConstraintSetBuilder {
        ConstraintSetBuilder::default()
    }

    /// Verify layout against all constraints.
    pub fn verify(&self, layout: &Layout<Pending>) -> Result<Layout<Verified>, VerificationError> {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // Tier 1: Hard constraints (any failure = invalid)
        for constraint in &self.hard {
            if let Err(v) = constraint.check(layout) {
                violations.push(v);
            }
        }

        if !violations.is_empty() {
            return Err(VerificationError::HardConstraints(violations));
        }

        // Tier 2: Structural constraints (compile-time when possible)
        for constraint in &self.structural {
            if let Err(v) = constraint.check(layout) {
                violations.push(v);
            }
        }

        if !violations.is_empty() {
            return Err(VerificationError::StructuralConstraints(violations));
        }

        // Tier 3: Advisory constraints (collect warnings)
        for constraint in &self.advisory {
            if let Err(v) = constraint.check(layout) {
                warnings.push(v);
            }
        }

        Ok(Layout {
            state: Verified { warnings },
            elements: layout.elements.clone(),
            context: layout.context.clone(),
        })
    }
}

/// Builder for constraint sets.
pub struct ConstraintSetBuilder {
    hard: Vec<Box<dyn Constraint>>,
    structural: Vec<Box<dyn Constraint>>,
    advisory: Vec<Box<dyn Constraint>>,
}

impl ConstraintSetBuilder {
    pub fn hard(mut self, constraint: impl Constraint + 'static) -> Self {
        self.hard.push(Box::new(constraint));
        self
    }

    pub fn structural(mut self, constraint: impl Constraint + 'static) -> Self {
        self.structural.push(Box::new(constraint));
        self
    }

    pub fn advisory(mut self, constraint: impl Constraint + 'static) -> Self {
        self.advisory.push(Box::new(constraint));
        self
    }

    pub fn profile(mut self, profile: ConstraintProfile) -> Self {
        for constraint in profile.constraints() {
            self.hard.push(constraint);
        }
        self
    }

    pub fn build(self) -> ConstraintSet {
        ConstraintSet {
            hard: self.hard,
            structural: self.structural,
            advisory: self.advisory,
        }
    }
}

/// Example: Composable constraint types
pub type AccessibleMobileLayout =
    Reflow320
    + ResizeText200
    + TextSpacing
    + MinTouchTarget;

/// Usage:
fn verify_accessible_mobile(layout: &Layout<Pending>) -> Result<Layout<Verified>, VerificationError> {
    let constraints = ConstraintSet::builder()
        .hard(Reflow320)
        .hard(ResizeText200)
        .hard(TextSpacing)
        .hard(MinTouchTarget { min_area: 1936.0 })
        .hard(ContrastMinimum)
        .structural(ZoomInvariant)
        .advisory(GridAlignment { step: 8.0 })
        .build();

    constraints.verify(layout)
}
```

### Specification References

Every constraint traceable to external authority:

```rust
/// External specification reference.
#[derive(Debug, Clone)]
pub enum SpecReference {
    Wcag {
        criterion: &'static str,
        level: Level,
        url: &'static str,
    },
    Css {
        module: &'static str,
        section: &'static str,
        url: &'static str,
    },
    DesignSystem {
        name: &'static str,
        section: &'static str,
        url: &'static str,
    },
    Iso {
        standard: &'static str,
        section: &'static str,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Level {
    A,
    AA,
    AAA,
}

/// Violation with spec traceability.
#[derive(Debug, Clone)]
pub enum Violation {
    Reflow {
        wcag: &'static str,
        level: &'static str,
        element: NodeId,
        width: f64,
        max_width: f64,
    },
    TouchTarget {
        wcag: &'static str,
        level: &'static str,
        element: NodeId,
        actual_area: f64,
        min_area: f64,
    },
    TextSpacing {
        wcag: &'static str,
        level: &'static str,
        element1: NodeId,
        element2: NodeId,
    },
    Contrast {
        wcag: &'static str,
        level: &'static str,
        element: NodeId,
        actual_ratio: f64,
        min_ratio: f64,
    },
    GridAlignment {
        element: NodeId,
        position: (f64, f64),
        grid_step: f64,
    },
    VisualAngle {
        element: NodeId,
        actual_size: f64,
        min_size: f64,
    },
}

impl Violation {
    /// Get spec reference for this violation.
    pub fn spec_ref(&self) -> Option<&'static str> {
        match self {
            Self::Reflow { wcag, .. } => Some(wcag),
            Self::TouchTarget { wcag, .. } => Some(wcag),
            Self::TextSpacing { wcag, .. } => Some(wcag),
            Self::Contrast { wcag, .. } => Some(wcag),
            _ => None,
        }
    }
}
```

## GeoRust Ecosystem

**Core crates**:
- **geo-types** (0.7): Core primitives (`Point`, `Rect`, `Polygon`, `Coord`, `LineString`)
- **geo** (0.28): Geometric algorithms (`Contains`, `Intersects`, `Area`, `EuclideanDistance`, `ConvexHull`)

**Mapping to UI + Constraints**:
```rust
// UI element bounds
type ElementBounds = geo::Rect<f64>;
type ElementPosition = geo::Point<f64>;

// Viewport as bounding rectangle
type Viewport = geo::Rect<f64>;

// L1: WCAG constraints via geometric predicates
viewport.contains(&element_bounds)        // Reflow
element_bounds.unsigned_area() >= 1936.0  // Touch target
!element1.intersects(&element2)           // Text spacing
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

## Phase 2: Constraint System Design

### Files to create:
- `crates/elicit_ui/src/constraints/mod.rs`
- `crates/elicit_ui/src/constraints/trait.rs`
- `crates/elicit_ui/src/constraints/wcag.rs`
- `crates/elicit_ui/src/constraints/css.rs`
- `crates/elicit_ui/src/constraints/profiles.rs`
- `crates/elicit_ui/src/constraints/breakpoints.rs`

### Constraint trait and infrastructure:

```rust
// constraints/trait.rs
pub trait Constraint: Send + Sync {
    fn check(&self, layout: &Layout) -> Result<(), Violation>;
    fn spec_ref(&self) -> SpecReference;
}

// constraints/wcag.rs
// All WCAG constraint implementations (Reflow320, MinTouchTarget, etc.)

// constraints/css.rs
// CSS unit system and zoom invariants

// constraints/profiles.rs
// ConstraintProfile, ConstraintSet, builder

// constraints/breakpoints.rs
// BreakpointSet, responsive testing
```

## Phase 3: Replace Custom Types with GeoTypes

### File to modify:
- `crates/elicit_ui/src/types.rs`

**Before**:
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

**After**:
```rust
use geo::{Point, Rect};
use geo_types::Coord;

/// UI element bounds in pixel space (CSS pixels).
pub type ElementBounds = Rect<f64>;

/// UI element position in pixel space.
pub type ElementPosition = Point<f64>;

/// Viewport as bounding rectangle.
pub type Viewport = Rect<f64>;

/// Helper to create viewport from dimensions.
pub fn viewport_from_dimensions(width: f64, height: f64) -> Viewport {
    Rect::new(
        coord! { x: 0.0, y: 0.0 },
        coord! { x: width, y: height }
    )
}

/// Layout context for CSS unit resolution and constraint checking.
#[derive(Debug, Clone)]
pub struct LayoutContext {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub root_font_size: f64,
    pub current_font_size: f64,
    pub zoom_level: f64,
}
```

## Phase 4: Update AccessKit Bridge

### File to create:
- `crates/elicit_ui/src/accesskit_bridge.rs`

```rust
//! Bridge between AccessKit and GeoRust spatial types.

use accesskit::{Node, Rect as AccessKitRect};
use geo::{Point, Rect};
use geo_types::Coord;

/// Convert AccessKit Rect to geo::Rect.
pub fn accesskit_to_georect(ak_rect: &AccessKitRect) -> Rect<f64> {
    Rect::new(
        coord! { x: ak_rect.x0, y: ak_rect.y0 },
        coord! { x: ak_rect.x1, y: ak_rect.y1 }
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

## Phase 5: Implement WCAG Constraints

### File to create:
- `crates/elicit_ui/src/constraints/wcag.rs`

Implement all WCAG constraints from Layer 1 using GeoRust predicates:

```rust
use geo::{Area, Contains, EuclideanDistance, Intersects};

// Reflow320 - already shown above
// MinTouchTarget - already shown above
// TextSpacing - already shown above
// ContrastMinimum - already shown above
// ZoomInvariant - already shown above

/// WCAG 1.4.4 Resize Text (Level AA)
/// Text must scale up to 200% without loss of functionality.
pub struct ResizeText200;

impl Constraint for ResizeText200 {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        // Create zoomed context
        let zoomed = layout.context.with_zoom(2.0);
        let zoomed_layout = layout.apply_context(zoomed);

        // Verify all elements still fit
        for element in &zoomed_layout.elements {
            if !zoomed_layout.viewport.contains(&element.bounds) {
                return Err(Violation::ResizeText {
                    wcag: "1.4.4",
                    level: "AA",
                    element: element.id,
                });
            }
        }

        Ok(())
    }

    fn spec_ref(&self) -> SpecReference {
        SpecReference::Wcag {
            criterion: "1.4.4",
            level: Level::AA,
            url: "https://www.w3.org/WAI/WCAG22/Understanding/resize-text",
        }
    }
}
```

## Phase 6: Rewrite Validators with Constraints

### File to modify:
- `crates/elicit_ui/src/validators.rs`

Replace manual validation with constraint checking:

```rust
use crate::constraints::{Constraint, ConstraintSet, ConstraintProfile};

/// Verify layout using constraint set.
pub fn verify_with_constraints(
    layout: &Layout<Pending>,
    profile: ConstraintProfile,
) -> Result<Layout<Verified>, VerificationError> {
    let constraints = ConstraintSet::builder()
        .profile(profile)
        .build();

    constraints.verify(layout)
}

/// Verify layout is accessible (WCAG 2.2 Level AA).
pub fn verify_accessible(
    layout: &Layout<Pending>,
) -> Result<Layout<Verified>, VerificationError> {
    verify_with_constraints(layout, ConstraintProfile::Wcag22Aa)
}

/// Verify layout follows design system.
pub fn verify_design_system(
    layout: &Layout<Pending>,
    system: ConstraintProfile,
) -> Result<Layout<Verified>, VerificationError> {
    verify_with_constraints(layout, system)
}
```

## Phase 7: Update Typestate Verification

### File to modify:
- `crates/elicit_ui/src/typestate.rs`

Integrate constraint system into typestate transitions:

```rust
impl Layout<Pending> {
    /// Verify layout with constraint profile.
    pub fn verify(self, profile: ConstraintProfile) -> Result<Layout<Verified>, VerificationError> {
        let constraints = ConstraintSet::builder()
            .profile(profile)
            .build();

        constraints.verify(&self)
    }

    /// Verify layout is accessible (WCAG 2.2 AA).
    pub fn verify_accessible(self) -> Result<Layout<Verified>, VerificationError> {
        self.verify(ConstraintProfile::Wcag22Aa)
    }

    /// Verify layout with custom constraints.
    pub fn verify_with(
        self,
        constraints: ConstraintSet,
    ) -> Result<Layout<Verified>, VerificationError> {
        constraints.verify(&self)
    }
}
```

## Phase 8: Update Tests

### Files to create/modify:
- `crates/elicit_ui/tests/constraints_test.rs` (new)
- `crates/elicit_ui/tests/button_verification_test.rs` (modify)

### Constraint tests:

```rust
//! Test spec-backed constraint system.

use elicit_ui::constraints::*;
use elicit_ui::{Layout, LayoutContext, viewport_from_dimensions};

#[test]
fn test_reflow_320_pass() {
    let mut layout = Layout::pending();
    layout.context = LayoutContext::standard(1920.0, 1080.0);

    // Add element that fits in 320px width
    layout.add_element(ElementBounds::new(
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 300.0, y: 100.0 },
    ));

    let constraint = Reflow320;
    assert!(constraint.check(&layout).is_ok());
}

#[test]
fn test_reflow_320_fail() {
    let mut layout = Layout::pending();
    layout.context = LayoutContext::standard(1920.0, 1080.0);

    // Add element that exceeds 320px width
    layout.add_element(ElementBounds::new(
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 400.0, y: 100.0 },
    ));

    let constraint = Reflow320;
    assert!(constraint.check(&layout).is_err());
}

#[test]
fn test_min_touch_target_pass() {
    let mut layout = Layout::pending();

    // Add button with 44×44px area
    let button = layout.add_interactive_element(ElementBounds::new(
        coord! { x: 0.0, y: 0.0 },
        coord! { x: 44.0, y: 44.0 },
    ));

    let constraint = MinTouchTarget { min_area: 1936.0 };
    assert!(constraint.check(&layout).is_ok());
}

#[test]
fn test_constraint_composition() {
    let layout = create_test_layout();

    let constraints = ConstraintSet::builder()
        .hard(Reflow320)
        .hard(MinTouchTarget { min_area: 1936.0 })
        .hard(ContrastMinimum)
        .advisory(GridAlignment { step: 8.0 })
        .build();

    let verified = constraints.verify(&layout).expect("Should pass");
    assert!(verified.state.warnings.is_empty());
}

#[test]
fn test_profile_verification() {
    let layout = create_test_layout();

    // Verify with WCAG AA profile
    let verified = layout.verify(ConstraintProfile::Wcag22Aa)
        .expect("Should meet WCAG AA");
}

#[test]
fn test_responsive_breakpoints() {
    let layout = create_test_layout();

    let breakpoints = BreakpointSet::material();
    let constraints = ConstraintSet::builder()
        .hard(Reflow320)
        .build();

    let result = verify_responsive(&layout, &breakpoints, &constraints);
    assert!(result.is_ok(), "Should work at all Material breakpoints");
}
```

## Phase 9: Documentation

### File to modify:
- `crates/elicit_ui/README.md`

Add comprehensive constraint architecture documentation:

```markdown
## Constraint Architecture

elicit_ui uses a **spec-backed constraint system** to eliminate arbitrary design decisions:

### Four-Layer Stack

| Layer | Authority | Role | Enforcement |
|-------|-----------|------|------------|
| **L1** | WCAG 2.2 | Hard accessibility constraints | Must pass |
| **L2** | CSS/W3C | Unit semantics, scaling | Compile-time |
| **L3** | Design Systems | Ergonomic heuristics | Advisory |
| **L4** | ISO 9241 | Perceptual grounding | Future |

### WCAG Constraints as Geometric Predicates

Every WCAG spatial constraint maps to GeoRust operations:

```rust
use geo::{Area, Contains, Intersects};

// WCAG 1.4.10 Reflow (AA)
viewport_320px.contains(&all_content)

// WCAG 2.5.5 Target Size (AAA)
button.unsigned_area() >= 1936.0  // 44×44px

// WCAG 1.4.12 Text Spacing (AA)
!element1.intersects(&element2)  // After spacing increase
```

### Composable Constraints

```rust
let constraints = ConstraintSet::builder()
    .hard(Reflow320)
    .hard(MinTouchTarget { min_area: 1936.0 })
    .hard(ContrastMinimum)
    .structural(ZoomInvariant)
    .advisory(GridAlignment { step: 8.0 })
    .build();

let verified = constraints.verify(&layout)?;
```

### Design System Profiles

```rust
// Verify with Material Design constraints
layout.verify(ConstraintProfile::MaterialDesign)?;

// Verify with Apple HIG constraints
layout.verify(ConstraintProfile::AppleHig)?;

// Verify WCAG 2.2 Level AA (minimal accessibility)
layout.verify(ConstraintProfile::Wcag22Aa)?;

// Verify WCAG 2.2 Level AAA (enhanced accessibility)
layout.verify(ConstraintProfile::Wcag22Aaa)?;
```

### Breakpoint Testing

Breakpoints are **profiles**, not invariants:

```rust
let breakpoints = BreakpointSet::material();
// [600px, 905px, 1240px, 1440px]

verify_responsive(&layout, &breakpoints, &constraints)?;
```

### Spec Traceability

Every constraint references external specification:

```rust
let violation = constraint.check(&layout).unwrap_err();
println!("Violation: {:?}", violation);
println!("Spec: {:?}", violation.spec_ref());
// "WCAG 1.4.10 Level AA: https://www.w3.org/WAI/WCAG22/Understanding/reflow"
```
```

## Verification Steps

### After implementation:

1. `cargo check -p elicit_ui`
2. `cargo test -p elicit_ui`
3. `cargo test -p elicit_ui --test constraints_test`
4. `cargo test -p elicit_ui --all-features`

### Manual verification:

1. Create layout violating Reflow320 → verify detection
2. Create layout with small touch targets → verify detection
3. Create layout with proper WCAG compliance → verify acceptance
4. Test constraint composition → verify additive behavior
5. Test design system profiles → verify profile-specific constraints

## Critical Files

### To create:
- `crates/elicit_ui/src/constraints/mod.rs`
- `crates/elicit_ui/src/constraints/trait.rs`
- `crates/elicit_ui/src/constraints/wcag.rs`
- `crates/elicit_ui/src/constraints/css.rs`
- `crates/elicit_ui/src/constraints/profiles.rs`
- `crates/elicit_ui/src/constraints/breakpoints.rs`
- `crates/elicit_ui/src/accesskit_bridge.rs`
- `crates/elicit_ui/tests/constraints_test.rs`

### To modify:
- `Cargo.toml` — Add geo, geo-types workspace dependencies
- `crates/elicit_ui/Cargo.toml` — Add geo dependencies
- `crates/elicit_ui/src/types.rs` — Replace custom types with geo types, add LayoutContext
- `crates/elicit_ui/src/validators.rs` — Replace with constraint system
- `crates/elicit_ui/src/typestate.rs` — Integrate constraint verification
- `crates/elicit_ui/src/lib.rs` — Export constraints module, accesskit_bridge
- `crates/elicit_ui/tests/button_verification_test.rs` — Update for constraints
- `crates/elicit_ui/README.md` — Document constraint architecture

## Implementation Order

1. **Phase 1**: Add GeoRust dependencies (10 min)
2. **Phase 2**: Design constraint system (1-2 hours)
3. **Phase 3**: Replace custom types with geo types (30 min)
4. **Phase 4**: Create AccessKit bridge (30 min)
5. **Phase 5**: Implement WCAG constraints (3-4 hours)
6. **Phase 6**: Rewrite validators with constraints (1 hour)
7. **Phase 7**: Update typestate verification (30 min)
8. **Phase 8**: Update tests (1-2 hours)
9. **Phase 9**: Documentation (1 hour)

**Total estimated time**: 9-12 hours

## Notes

### Why This Architecture?

1. **No bikeshedding**: All constraints traceable to external specs
2. **Composability**: Constraints combine additively
3. **Enforcement tiers**: Hard (must pass), structural (compile-time), advisory (warnings)
4. **Proof-carrying**: Violations reference exact WCAG criteria
5. **Design system flexibility**: Optional profiles for Material/HIG/etc.

### WCAG Constraints → GeoRust Predicates

| WCAG Criterion | Geometric Encoding |
|----------------|-------------------|
| 1.4.10 Reflow | `viewport_320px.contains(&all_content)` |
| 1.4.12 Text Spacing | `!elements.any(\|(a, b)\| a.intersects(b))` after spacing |
| 2.5.5 Target Size | `target.unsigned_area() >= 1936.0` |
| 1.4.3 Contrast | Color distance in CIELAB space |
| 1.4.4 Resize Text | Layout valid at 200% zoom |

### GeoRust Predicates Used

- `Contains<T>`: Element inside viewport, label inside element, reflow
- `Intersects<T>`: Elements overlap detection, text spacing
- `Area`: Minimum touch target size (44×44px = 1936.0)
- `EuclideanDistance`: Element spacing, minimum gaps
- `ConvexHull`: Bounding box calculations (future)
- `Centroid`: Element center positioning (future)

### Benefits

1. **Correctness**: Proven geometric algorithms from GeoRust
2. **Expressiveness**: Natural vocabulary for spatial relationships
3. **Performance**: Optimized spatial calculations
4. **Verifiability**: Geometric proofs map to formal verification
5. **Traceability**: Every constraint references external spec
6. **Composability**: Mix WCAG + design system + custom constraints

### Future Extensions

With GeoRust + constraint foundation:
- **Adaptive layouts**: Constraint-based automatic repositioning
- **Collision detection**: Drag-and-drop validation
- **Responsive layouts**: Viewport-aware element flow
- **Accessibility paths**: Shortest path for keyboard navigation
- **Visual density**: Area coverage analysis
- **L4 constraints**: ISO 9241 visual angle calculations
