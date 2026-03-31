# Rust Crate Research: UI Constraint Implementation

Research findings for implementing the spec-backed constraint system defined in ELICIT_UI_GEORUST_PLAN.md.

## Executive Summary

### High-Value Additions

| Crate | Layer | Purpose | Priority | Integration Effort |
|-------|-------|---------|----------|-------------------|
| **palette** | L1 | WCAG contrast + perceptual color | ⭐⭐⭐ | Low (stable API) |
| **oak-css** | L2 | CSS unit parsing/validation | ⭐⭐⭐ | Medium (new, Jan 2026) |
| **taffy** | L2 | CSS layout algorithms | ⭐⭐ | High (complex integration) |
| **rustybuzz** | L2 | Typography metrics | ⭐ | Medium (font shaping) |

### Quick Wins

1. **Add `palette`** → Implement WCAG 1.4.3 (Contrast Minimum) constraint
2. **Add `oak-css`** → Parse CSS units (px, rem, em, vw, vh) for validation

---

## Layer 1: WCAG Constraints (Hard Accessibility)

### Color Contrast

**Problem**: WCAG 1.4.3 requires 4.5:1 contrast for normal text, 3:1 for large text.

**Solution**: Multiple options, ranked by capability:

#### 1. **palette** (Recommended ⭐)
```toml
palette = { version = "0.7", features = ["std"] }
```

**Why**: Most comprehensive color science library in Rust ecosystem.

**Features**:
- CIELAB, Oklab, LCH color spaces (perceptually uniform)
- DeltaE distance algorithms (CIEDE2000, CIEDE1994)
- Color conversion: sRGB ↔ linear RGB ↔ XYZ ↔ Lab
- Type-safe color operations

**Usage**:
```rust
use palette::{Srgb, Lab, IntoColor, color_difference::ImprovedCiede2000};

// WCAG contrast calculation
pub fn wcag_contrast_ratio(fg: Srgb, bg: Srgb) -> f64 {
    let l1 = relative_luminance(fg);
    let l2 = relative_luminance(bg);

    let (lighter, darker) = if l1 > l2 { (l1, l2) } else { (l2, l1) };
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Srgb) -> f64 {
    // sRGB → linear RGB → luminance
    let linear = color.into_linear();
    0.2126 * linear.red + 0.7152 * linear.green + 0.0722 * linear.blue
}

// Perceptual color distance (for advanced constraints)
pub fn color_distance(c1: Srgb, c2: Srgb) -> f64 {
    let lab1: Lab = c1.into_color();
    let lab2: Lab = c2.into_color();
    lab1.difference(lab2)  // DeltaE
}
```

**Integration**:
```rust
// constraints/wcag.rs
pub struct ContrastMinimum;

impl Constraint for ContrastMinimum {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        for element in layout.text_elements() {
            let ratio = wcag_contrast_ratio(
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

**Docs**: https://docs.rs/palette/latest/palette/

---

#### 2. **contrast** (Alternative)
```toml
contrast = "0.1"
```

**Why**: Focused solely on WCAG contrast (simpler API).

**Usage**:
```rust
use contrast::contrast_ratio;

let ratio = contrast_ratio([255, 255, 255], [0, 0, 0]);
// Returns 21.0 (maximum contrast)
```

**Limitation**: No perceptual color spaces (Lab/Oklab), only RGB.

**Docs**: https://docs.rs/contrast/latest/contrast/

---

#### 3. **deltae** (Perceptual distance only)
```toml
deltae = "0.2"
```

**Why**: Pure DeltaE implementation (CIEDE2000).

**Usage**:
```rust
use deltae::{LabValue, DEMethod, DeltaE};

let lab1 = LabValue { l: 50.0, a: 10.0, b: -20.0 };
let lab2 = LabValue { l: 55.0, a: 8.0, b: -18.0 };

let delta = DeltaE::new(lab1, lab2, DEMethod::DE2000);
// Returns perceptual distance
```

**Use case**: Advanced color harmony validation (not WCAG contrast).

**Docs**: https://docs.rs/deltae/latest/deltae/

---

### Recommendation: Use **palette**
- Most comprehensive
- Handles both WCAG contrast AND perceptual color science
- Active maintenance
- Type-safe color operations

---

## Layer 2: CSS Standards (Structural)

### CSS Unit Parsing

**Problem**: Need to parse and convert CSS units (px, rem, em, vw, vh) for constraint validation.

**Solution**: **oak-css** (new, actively developed)

```toml
oak-css = "0.1"  # Check latest version
```

**Why**: Modern CSS parser with detailed unit handling.

**Features**:
- Parse CSS units: px, rem, em, vw, vh, %, pt, pc, etc.
- Parse CSS functions: calc(), clamp(), min(), max()
- Incremental parsing (efficient updates)

**Usage**:
```rust
use oak_css::{Parser, Token};

pub enum CssUnit {
    Px(f64),
    Rem(f64),
    Em(f64),
    Vw(f64),
    Vh(f64),
}

impl CssUnit {
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        let parser = Parser::new(input);
        // Parse unit tokens
        match parser.next_token() {
            Token::Dimension { value, unit } => {
                match unit.as_str() {
                    "px" => Ok(CssUnit::Px(value)),
                    "rem" => Ok(CssUnit::Rem(value)),
                    "em" => Ok(CssUnit::Em(value)),
                    "vw" => Ok(CssUnit::Vw(value)),
                    "vh" => Ok(CssUnit::Vh(value)),
                    _ => Err(ParseError::UnknownUnit(unit)),
                }
            }
            _ => Err(ParseError::NotAUnit),
        }
    }

    pub fn to_px(&self, context: &LayoutContext) -> f64 {
        match self {
            Self::Px(v) => *v,
            Self::Rem(v) => v * context.root_font_size,
            Self::Em(v) => v * context.current_font_size,
            Self::Vw(v) => v * context.viewport_width / 100.0,
            Self::Vh(v) => v * context.viewport_height / 100.0,
        }
    }
}
```

**Integration**:
```rust
// constraints/css.rs
pub struct ZoomInvariant;

impl Constraint for ZoomInvariant {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        // Test at 200% zoom (WCAG 1.4.4)
        let zoomed_context = layout.context.with_zoom(2.0);
        let zoomed_layout = layout.apply_context(zoomed_context);

        // Verify all hard constraints still pass
        for constraint in layout.hard_constraints() {
            constraint.check(&zoomed_layout)?;
        }

        Ok(())
    }
}
```

**Docs**: https://crates.io/crates/oak-css

**Alternative**: **cssparser** (Mozilla/Servo)
- Lower-level (more control)
- Used in Firefox (battle-tested)
- Requires more manual parsing

---

### CSS Layout Algorithms

**Problem**: Need to validate layouts against CSS Flexbox/Grid specifications.

**Solution**: **taffy** (collaborative project)

```toml
taffy = "0.5"  # Check latest version
```

**Why**: Reference implementation of CSS layout algorithms.

**Features**:
- CSS Block layout
- CSS Flexbox (complete spec)
- CSS Grid (complete spec)
- Used by Dioxus, egui_taffy

**Usage**:
```rust
use taffy::prelude::*;

pub struct LayoutEngine {
    taffy: Taffy,
}

impl LayoutEngine {
    pub fn compute_layout(
        &mut self,
        tree: &AccessKitTree,
        viewport: Size<f32>,
    ) -> Result<ComputedLayout, LayoutError> {
        // Build taffy tree from AccessKit tree
        let root = self.build_taffy_tree(tree)?;

        // Compute layout
        self.taffy.compute_layout(
            root,
            taffy::prelude::Size {
                width: AvailableSpace::Definite(viewport.width),
                height: AvailableSpace::Definite(viewport.height),
            },
        )?;

        // Extract computed bounds
        Ok(self.extract_bounds(root))
    }
}
```

**Integration**:
```rust
// constraints/wcag.rs
pub struct Reflow320;

impl Constraint for Reflow320 {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        let mut engine = LayoutEngine::new();

        // Compute layout at 320px width
        let computed = engine.compute_layout(
            &layout.tree,
            Size { width: 320.0, height: f32::MAX },
        )?;

        // Check for horizontal overflow
        if computed.requires_horizontal_scroll() {
            return Err(Violation::Reflow {
                wcag: "1.4.10",
                level: "AA",
                // ...
            });
        }

        Ok(())
    }
}
```

**Complexity**: High (requires building parallel tree structure).

**Benefit**: Accurate layout-based overflow detection (not just bounds-based).

**Docs**: https://github.com/DioxusLabs/taffy

---

### Typography Metrics

**Problem**: Need font metrics for line height, kerning, minimum sizes.

**Solution**: **rustybuzz** (HarfBuzz in Rust)

```toml
rustybuzz = "0.18"
ttf-parser = "0.24"
```

**Why**: Industry-standard text shaping algorithm.

**Features**:
- Text shaping (converts text → positioned glyphs)
- Font metrics (ascent, descent, line height)
- Passes 98.6% of HarfBuzz test suite

**Usage**:
```rust
use rustybuzz::{Face, UnicodeBuffer};
use ttf-parser::Face as TtfFace;

pub fn get_text_metrics(
    font_data: &[u8],
    text: &str,
    font_size: f32,
) -> TextMetrics {
    let face = Face::from_slice(font_data, 0).unwrap();
    let mut buffer = UnicodeBuffer::new();
    buffer.push_str(text);

    let output = rustybuzz::shape(&face, &[], buffer);

    let upem = face.units_per_em() as f32;
    let scale = font_size / upem;

    TextMetrics {
        width: output.glyph_positions().iter()
            .map(|p| p.x_advance)
            .sum::<i32>() as f32 * scale,
        ascent: face.ascender() as f32 * scale,
        descent: face.descender() as f32 * scale,
        line_height: (face.ascender() - face.descender() + face.line_gap()) as f32 * scale,
    }
}
```

**Integration**:
```rust
// constraints/wcag.rs
pub struct MinTextSize;

impl Constraint for MinTextSize {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        for element in layout.text_elements() {
            let metrics = get_text_metrics(
                element.font_data,
                element.text,
                element.font_size,
            );

            // WCAG 1.4.4: Minimum 16px for body text
            if metrics.line_height < 16.0 {
                return Err(Violation::TextSize {
                    wcag: "1.4.4",
                    level: "AA",
                    element: element.id,
                    actual_size: metrics.line_height,
                    min_size: 16.0,
                });
            }
        }
        Ok(())
    }
}
```

**Docs**: https://github.com/harfbuzz/rustybuzz

---

## Layer 3: Design Systems (Advisory)

### Material Design

**Problem**: Need Material Design 3 color system for design token validation.

**Solution**: **material-color-utilities**

```toml
material-color-utilities = "0.2"
```

**Features**:
- Generate M3 tonal palettes from seed color
- Extract colors from images (HCT color space)
- Ensure accessible color combinations

**Usage**:
```rust
use material_color_utilities::{Hct, TonalPalette};

pub struct MaterialDesignProfile;

impl ConstraintProfile for MaterialDesignProfile {
    fn constraints(&self) -> Vec<Box<dyn Constraint>> {
        vec![
            Box::new(MinTouchTarget { min_area: 48.0 * 48.0 }), // 48dp
            Box::new(GridAlignment { step: 8.0 }),
            Box::new(MaterialColorScheme),
        ]
    }
}

pub struct MaterialColorScheme;

impl Constraint for MaterialColorScheme {
    fn check(&self, layout: &Layout) -> Result<(), Violation> {
        // Validate colors follow M3 tonal palette
        for element in layout.elements() {
            if !is_m3_color(element.background_color) {
                return Err(Violation::DesignSystem {
                    element: element.id,
                    rule: "M3 tonal palette",
                });
            }
        }
        Ok(())
    }
}
```

**Docs**: https://crates.io/crates/material-color-utilities

---

### Design Tokens

**Problem**: Parse and validate design token files.

**Solution**: **design_token_parser**

```toml
design_token_parser = "0.1"
```

**Features**:
- Parse Design Tokens Format (W3C Community Group)
- Load colors, typography, spacing from JSON/YAML

**Usage**:
```rust
use design_token_parser::DesignTokens;

pub fn load_design_tokens(path: &str) -> Result<ConstraintProfile, ParseError> {
    let tokens = DesignTokens::from_file(path)?;

    let constraints = vec![
        Box::new(MinTouchTarget {
            min_area: tokens.spacing.touch_target.pow(2),
        }),
        Box::new(GridAlignment {
            step: tokens.spacing.grid_base,
        }),
    ];

    Ok(ConstraintProfile::Custom(constraints))
}
```

**Docs**: https://crates.io/crates/design_token_parser

---

## Layer 4: Formal Verification (ISO/Proofs)

### Formal Verification Stack

**Current**: We already have comprehensive formal verification tooling:

- **Kani** - Rust Model Checker (bounded verification)
- **Creusot** - Deductive verification for Rust
- **Verus** - Verified Rust with linear types

**No additional SMT solvers needed**. The existing verification stack provides:
- Proof-carrying code generation
- Invariant checking
- Precondition/postcondition verification
- Refinement types

**Integration with constraints**:
```rust
#[kani::proof]
fn prove_reflow_320() {
    let layout = kani::any::<Layout>();
    kani::assume(layout.is_valid());

    let constraint = Reflow320;
    let result = constraint.check(&layout);

    // Kani proves this holds for all valid layouts
    assert!(result.is_ok());
}
```

---

### Property-Based Testing

**Current**: Already using **proptest** ✅

```toml
proptest = "1.5"
```

**Usage**:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reflow_invariant(
        width in 320.0f64..=1920.0,
        height in 480.0f64..=1080.0,
    ) {
        let layout = generate_layout(width, height);
        let constraint = Reflow320;

        // Property: All layouts must satisfy reflow at 320px
        prop_assert!(constraint.check(&layout).is_ok());
    }

    #[test]
    fn test_zoom_invariant(
        zoom in 1.0f64..=2.0,
    ) {
        let layout = generate_standard_layout();
        let zoomed = layout.apply_zoom(zoom);

        // Property: All constraints valid at 100% → valid at any zoom ≤ 200%
        for constraint in layout.hard_constraints() {
            prop_assert!(constraint.check(&zoomed).is_ok());
        }
    }
}
```

**Keep using proptest** - it's working well for constraint validation.

---

## Implementation Roadmap

### Phase 1: Color Contrast (Low Effort, High Value)

**Goal**: Add WCAG 1.4.3 (Contrast Minimum) constraint.

**Tasks**:
1. Add `palette` dependency
2. Implement `wcag_contrast_ratio()` function
3. Create `ContrastMinimum` constraint
4. Add to `Wcag22Aa` profile
5. Write proptest properties

**Time**: 2-4 hours

**Files**:
- `Cargo.toml` - Add palette
- `crates/elicit_ui/src/constraints/wcag.rs` - Add ContrastMinimum
- `crates/elicit_ui/tests/constraints_test.rs` - Add tests

---

### Phase 2: CSS Units (Medium Effort, High Value)

**Goal**: Parse and validate CSS units for layout calculations.

**Tasks**:
1. Add `oak-css` dependency
2. Extend `CssUnit` enum with parser
3. Update `LayoutContext` to resolve units
4. Modify `Reflow320` to handle CSS units
5. Add unit conversion tests

**Time**: 4-6 hours

**Files**:
- `Cargo.toml` - Add oak-css
- `crates/elicit_ui/src/constraints/css.rs` - Implement CssUnit parsing
- `crates/elicit_ui/src/types.rs` - Update LayoutContext
- `crates/elicit_ui/tests/css_units_test.rs` - Add tests

---

### Phase 3: Layout Engine Integration (High Effort, High Value)

**Goal**: Compute layouts with `taffy` for accurate overflow detection.

**Tasks**:
1. Add `taffy` dependency
2. Create `LayoutEngine` wrapper
3. Build AccessKit → Taffy tree converter
4. Update `Reflow320` to use computed layout
5. Add layout computation tests

**Time**: 8-12 hours

**Files**:
- `Cargo.toml` - Add taffy
- `crates/elicit_ui/src/layout_engine.rs` - New file
- `crates/elicit_ui/src/constraints/wcag.rs` - Update Reflow320
- `crates/elicit_ui/tests/layout_test.rs` - Add tests

---

### Phase 4: Typography Metrics (Medium Effort, Medium Value)

**Goal**: Validate font sizes and line heights with `rustybuzz`.

**Tasks**:
1. Add `rustybuzz` and `ttf-parser` dependencies
2. Implement `get_text_metrics()` function
3. Create `MinTextSize` constraint
4. Add to `Wcag22Aa` profile
5. Write tests with sample fonts

**Time**: 4-6 hours

**Files**:
- `Cargo.toml` - Add rustybuzz, ttf-parser
- `crates/elicit_ui/src/typography.rs` - New file
- `crates/elicit_ui/src/constraints/wcag.rs` - Add MinTextSize
- `crates/elicit_ui/tests/typography_test.rs` - Add tests

---

## Crate Summary Table

| Crate | Version | Layer | Status | Priority | Docs |
|-------|---------|-------|--------|----------|------|
| **palette** | 0.7 | L1 | Stable | ⭐⭐⭐ | [docs.rs](https://docs.rs/palette) |
| **oak-css** | 0.1 | L2 | New (2026) | ⭐⭐⭐ | [crates.io](https://crates.io/crates/oak-css) |
| **taffy** | 0.5 | L2 | Active | ⭐⭐ | [GitHub](https://github.com/DioxusLabs/taffy) |
| **rustybuzz** | 0.18 | L2 | Stable | ⭐ | [GitHub](https://github.com/harfbuzz/rustybuzz) |
| **material-color-utilities** | 0.2 | L3 | Active | ⭐ | [crates.io](https://crates.io/crates/material-color-utilities) |
| **design_token_parser** | 0.1 | L3 | Maintenance | ⭐ | [crates.io](https://crates.io/crates/design_token_parser) |
| **proptest** ✅ | 1.5 | L4 | Active | (current) | [GitHub](https://github.com/proptest-rs/proptest) |

---

## Decision Matrix

### Add Immediately
- ✅ **palette** - Color contrast is critical, stable crate
- ✅ **oak-css** - CSS units needed for proper constraint validation

### Evaluate Next
- 🔍 **taffy** - High value but complex integration

### Future Extensions
- 📅 **rustybuzz** - Typography metrics (WCAG 1.4.4 extended)
- 📅 **material-color-utilities** - Design system layer
- 📅 **design_token_parser** - Token validation

---

## References

All crate links verified as of March 2026.

### Documentation
- [WCAG 2.2 Guidelines](https://www.w3.org/WAI/WCAG22/)
- [CSS Values and Units Module](https://www.w3.org/TR/css-values-3/)
- [Material Design 3](https://m3.material.io/)
- [Design Tokens Format](https://design-tokens.github.io/community-group/)

### Related Plans
- `ELICIT_UI_GEORUST_PLAN.md` - Main constraint architecture
- `TYPESTATE_UI_DESIGN.md` - Typestate verification pattern
