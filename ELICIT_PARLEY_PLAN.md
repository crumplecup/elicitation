# elicit_parley — Implementation Plan

> **Premise:** Expose parley's rich text layout and shaping API as MCP tools for both runtime computation and code generation.
> **Approach:** Completionist harvesting using runtime-only tools (context state), dual-mode tools (style/layout), and fragment tools (text layout code gen).

---

## Why Parley is Straightforward

**Shares characteristics with taffy:**
- **Natural serialization**: Style properties are enums/primitives
- **Synchronous operations**: Text shaping is deterministic
- **Stateful by nature**: FontContext/LayoutContext manage state
- **Clear API flow**: Create context → Build layout → Break lines → Position glyphs

**Similar to other "straightforward" crates:**
- No async/lifetimes in core API
- All inputs/outputs serializable
- CSS-like style properties
- Visual domain (text layout)

**Domain-specific characteristics:**
- **Text shaping complexity**: HarfBuzz integration, OpenType features
- **Bidirectional text**: RTL/LTR handling, Unicode normalization
- **Typography details**: Kerning, ligatures, variation axes
- **Line breaking**: Unicode line break algorithm

**Perfect fit for:**
- Rich text editors (AI-assisted text layout)
- Document generation (PDF, HTML rendering)
- Typography tools (font preview, style exploration)
- Accessibility analysis (text metrics, contrast checking)
- Internationalization (bidi, locales, multi-script)

---

## Core Constraint: Stateful Contexts + Lifetimes

Parley's core API uses shared resources and builders with lifetimes:

```rust
pub struct FontContext { /* font database */ }
pub struct LayoutContext { /* scratch space */ }

pub struct RangedBuilder<'a, B> { /* lifetime-bound */ }
pub struct TreeBuilder<'a, B> { /* lifetime-bound */ }

pub struct Layout<B> { /* output */ }
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Contexts | `FontContext`, `LayoutContext` | ✅ Runtime-only (UUID → context handle) |
| Style properties | `StyleProperty` (23 variants) | ✅ Dual-mode (serialize as JSON) |
| Text content | String, char ranges | ✅ Dual-mode (plain text in JSON) |
| Layout output | `Layout<B>` with lines/runs | ✅ Dual-mode (serialize positioned glyphs) |
| Builders (lifetimes) | `RangedBuilder<'a, B>` | ⚠️ Runtime-only (cannot serialize lifetimes) |
| Brush trait | `B: Brush` (generic color) | ✅ Dual-mode (RGBA color in JSON) |
| Font queries | `FontStack`, `FontWeight`, etc. | ✅ Dual-mode (serialize as strings/numbers) |
| Glyph data | Positioned glyphs, metrics | ✅ Dual-mode (serialize coordinates/IDs) |

**Key insight:** Like taffy, parley is **stateful** (contexts, builders) but all **inputs and outputs** are serializable:
- **Input**: Text + StyleProperty spans
- **Output**: Lines with positioned glyphs (coordinates, metrics, indices)
- **State**: FontContext/LayoutContext via UUID handles

---

## Tool Breakdown: 380 Total

### Runtime-Only Tools (160)

UUID-keyed handles for persistent contexts and layouts:

#### Context Management (20)
- `context_create_font` — Create FontContext with default fonts
- `context_create_font_from_paths` — Load specific font files
- `context_create_layout` — Create LayoutContext
- `context_delete_font` — Remove FontContext
- `context_delete_layout` — Remove LayoutContext
- `context_register_font` — Add font to database
- `context_query_fonts` — List available fonts
- And 13 more for context lifecycle

#### Builder Lifecycle (30)
- `builder_create_ranged` — Create RangedBuilder
- `builder_create_tree` — Create TreeBuilder
- `builder_add_text` — Set text content
- `builder_push_default_style` — Set global fallback style
- `builder_push_ranged_style` — Apply style to character range
- `builder_push_tree_style_open` — Start style span (tree builder)
- `builder_push_tree_style_close` — End style span (tree builder)
- `builder_push_inline_box` — Insert non-text element
- `builder_build` — Finalize and create Layout
- `builder_delete` — Clean up builder
- And 20 more for builder operations

#### Layout Operations (40)
- `layout_create` — Create empty Layout
- `layout_delete` — Remove Layout
- `layout_break_lines` — Compute line breaks with max width
- `layout_align` — Apply alignment (start, center, end, justify)
- `layout_width` — Get layout width
- `layout_height` — Get layout height
- `layout_full_width` — Width including trailing whitespace
- `layout_line_count` — Number of lines
- `layout_get_line` — Access line by index
- `layout_is_rtl` — Check dominant text direction
- `layout_scale` — Get scale factor
- And 29 more for layout queries

#### Line Queries (25)
- `line_metrics` — Get baseline, ascent, descent
- `line_text_range` — Get character range for line
- `line_break_reason` — Why line broke
- `line_run_count` — Number of runs in line
- `line_item_count` — Number of positioned items
- `line_runs` — Iterate over runs
- `line_items` — Iterate over positioned glyphs/boxes
- And 18 more for line inspection

#### Glyph/Run Queries (25)
- `run_metrics` — Get run metrics
- `run_font_size` — Get font size for run
- `run_text_range` — Get character range
- `run_glyph_count` — Number of glyphs
- `run_glyphs` — Access positioned glyphs
- `item_position` — Get X/Y coordinates
- `item_glyph_id` — Get glyph ID from font
- `item_advance` — Get horizontal advance
- And 17 more for glyph data

#### Font Queries (20)
- `font_query_by_family` — Find fonts by family name
- `font_query_by_weight` — Find fonts by weight
- `font_metrics` — Get font metrics (ascent, descent, cap height)
- `font_has_glyph` — Check if font contains codepoint
- `font_list_features` — List OpenType features
- `font_list_variations` — List variation axes
- And 14 more for font inspection

### Dual-Mode Tools (180)

Tools that both create style/layout JSON AND generate code:

#### Style Property Creation (50)
- `style_font_stack` — Set font family stack
- `style_font_size` — Set font size (f32)
- `style_font_weight` — Set weight (100-900)
- `style_font_style` — Set style (normal, italic, oblique)
- `style_font_width` — Set width/stretch
- `style_font_variations` — Set OpenType variation axes
- `style_font_features` — Set OpenType features
- `style_brush` — Set text color/fill
- `style_underline` — Enable underline
- `style_underline_offset` — Underline position
- `style_underline_size` — Underline thickness
- `style_underline_brush` — Underline color
- `style_strikethrough` — Enable strikethrough
- `style_strikethrough_offset` — Strikethrough position
- `style_strikethrough_size` — Strikethrough thickness
- `style_strikethrough_brush` — Strikethrough color
- `style_line_height` — Line height multiplier
- `style_letter_spacing` — Character spacing
- `style_word_spacing` — Word spacing
- `style_word_break` — Line break rules
- `style_overflow_wrap` — Emergency line breaking
- `style_text_wrap_mode` — Wrap behavior
- `style_locale` — Language/locale
- And 27 more for style combinations

#### Brush/Color Types (20)
- `brush_solid` — Create solid color brush
- `brush_rgba` — RGBA color
- `brush_rgb` — RGB color
- `brush_hex` — Parse hex color (#RRGGBB)
- `brush_named` — Named CSS colors
- And 15 more for color types

#### Font Property Types (30)
- `font_weight_thin` — Weight 100
- `font_weight_normal` — Weight 400
- `font_weight_bold` — Weight 700
- `font_style_normal` — Normal style
- `font_style_italic` — Italic
- `font_style_oblique` — Oblique with angle
- `font_stretch_condensed` — Condensed width
- `font_stretch_expanded` — Expanded width
- `font_feature_enable` — Enable OpenType feature
- `font_variation_set` — Set variation axis value
- And 20 more for font property values

#### Layout Result Serialization (40)
- `layout_to_json` — Serialize entire Layout
- `layout_lines_to_json` — Lines with metrics
- `layout_glyphs_to_json` — All positioned glyphs
- `layout_text_with_positions` — Text with XY coords
- `line_to_json` — Single line data
- `run_to_json` — Single run data
- `glyph_to_json` — Single glyph position/metrics
- `metrics_to_json` — Line/font metrics
- And 32 more for serialization patterns

#### Text Range Types (20)
- `text_range_all` — Entire text
- `text_range_from_to` — Start to end indices
- `text_range_from` — Start to text end
- `text_range_to` — Text start to end
- `text_range_word` — Word boundaries
- `text_range_line` — Line boundaries
- And 14 more for range patterns

#### Alignment Types (10)
- `align_start` — Left/right based on direction
- `align_center` — Center alignment
- `align_end` — Right/left based on direction
- `align_justify` — Justify (stretch)
- And 6 more for alignment options

#### Line Break Types (10)
- `line_break_auto` — Default Unicode rules
- `line_break_loose` — More permissive
- `line_break_normal` — Standard rules
- `line_break_strict` — Strict rules
- `line_break_anywhere` — Emergency breaking
- And 5 more for break rules

### Fragment Tools (40)

Code generation for text layout computations:

#### Context Construction Code (10)
- `emit_font_context` — Generate FontContext::new()
- `emit_layout_context` — Generate LayoutContext::new()
- `emit_context_setup` — Complete context initialization
- And 7 more for context setup patterns

#### Builder Code Generation (15)
- `emit_ranged_builder` — Generate RangedBuilder code
- `emit_tree_builder` — Generate TreeBuilder code
- `emit_push_default_style` — Generate default style code
- `emit_push_ranged_style` — Generate ranged style code
- `emit_build_layout` — Generate build() call
- And 10 more for builder patterns

#### Layout Computation Code (10)
- `emit_break_lines` — Generate line breaking code
- `emit_align` — Generate alignment code
- `emit_layout_query` — Generate layout access code
- And 7 more for layout computation patterns

#### Complete Assembly (5)
- `assemble_parley_binary` — Generate complete executable
- `emit_text_renderer` — Generate rendering code
- `emit_layout_test` — Generate layout test
- And 2 more for assembly patterns

---

## Serialization Strategy

### StyleProperty JSON

```json
{
  "type": "FontStack",
  "families": ["Helvetica Neue", "Helvetica", "Arial", "sans-serif"]
}

{
  "type": "FontSize",
  "size": 16.0
}

{
  "type": "FontWeight",
  "weight": 700
}

{
  "type": "Brush",
  "color": { "r": 0, "g": 0, "b": 0, "a": 255 }
}

{
  "type": "Underline",
  "enabled": true,
  "offset": 2.0,
  "size": 1.0,
  "brush": { "r": 0, "g": 0, "b": 255, "a": 255 }
}
```

### Ranged Styles

```json
{
  "text": "Hello, world! This is bold and italic.",
  "styles": [
    {
      "property": { "type": "FontSize", "size": 16.0 },
      "range": { "start": 0, "end": 40 }
    },
    {
      "property": { "type": "FontWeight", "weight": 700 },
      "range": { "start": 22, "end": 26 }
    },
    {
      "property": { "type": "FontStyle", "style": "Italic" },
      "range": { "start": 31, "end": 37 }
    }
  ]
}
```

### Layout Output (Lines + Glyphs)

```json
{
  "width": 400.0,
  "height": 120.0,
  "scale": 1.0,
  "is_rtl": false,
  "lines": [
    {
      "index": 0,
      "metrics": {
        "baseline": 14.0,
        "ascent": 12.0,
        "descent": 4.0,
        "leading": 2.0
      },
      "text_range": { "start": 0, "end": 25 },
      "break_reason": "MaxAdvance",
      "runs": [
        {
          "font_size": 16.0,
          "text_range": { "start": 0, "end": 25 },
          "glyphs": [
            {
              "id": 43,
              "x": 0.0,
              "y": 14.0,
              "advance": 8.5,
              "cluster": 0
            },
            {
              "id": 72,
              "x": 8.5,
              "y": 14.0,
              "advance": 9.2,
              "cluster": 1
            }
            // ... more glyphs
          ]
        }
      ]
    }
    // ... more lines
  ]
}
```

### Font Query Results

```json
{
  "fonts": [
    {
      "family": "Helvetica Neue",
      "weight": 400,
      "style": "Normal",
      "stretch": "Normal",
      "features": ["kern", "liga", "calt"],
      "variations": [
        { "tag": "wght", "min": 100.0, "max": 900.0, "default": 400.0 }
      ]
    }
  ]
}
```

---

## Phase 1: Context Management (Runtime-Only)

**Goal:** Establish UUID-keyed registry pattern for FontContext and LayoutContext.

### Crate Structure

```
crates/elicit_parley/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── context_registry.rs    # UUID → context mapping
    ├── builder_registry.rs    # UUID → builder (lifetime management)
    ├── layout_registry.rs     # UUID → Layout mapping
    ├── style_tools.rs         # Dual-mode style creation
    ├── layout_tools.rs        # Dual-mode layout serialization
    ├── fragments.rs           # Code generation tools
    └── serde_types.rs         # JSON wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_parley"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
parley = "0.7"
fontique = "0.4"  # Font querying (used by parley)
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
```

### Runtime Tool Example: Context Creation

```rust
use elicitation_derive::elicit_tool;
use parley::{FontContext, LayoutContext};

pub struct ParleyPlugin {
    font_contexts: Arc<Mutex<HashMap<Uuid, FontContext>>>,
    layout_contexts: Arc<Mutex<HashMap<Uuid, LayoutContext>>>,
    // Builders have lifetimes, so we store them differently
    // Each builder gets a UUID, but we can't store them long-term
    // Instead, we use a session-based approach (create → use → build → delete)
    layouts: Arc<Mutex<HashMap<Uuid, Layout<SimpleBrush>>>>,
}

#[derive(Debug, Clone, Copy)]
pub struct SimpleBrush {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[elicit_tool(
    plugin = "parley_context",
    name = "context_create_font",
    description = "Create FontContext with system fonts"
)]
async fn context_create_font(_: FontContextCreateParams) -> Result<CallToolResult, ErrorData> {
    let font_context = FontContext::new();
    let context_id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.font_contexts.lock().unwrap().insert(context_id, font_context);

    Ok(CallToolResult::success(json!({ "font_context_id": context_id })))
}

#[elicit_tool(
    plugin = "parley_context",
    name = "context_create_layout",
    description = "Create LayoutContext for building layouts"
)]
async fn context_create_layout(_: LayoutContextCreateParams) -> Result<CallToolResult, ErrorData> {
    let layout_context = LayoutContext::new();
    let context_id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.layout_contexts.lock().unwrap().insert(context_id, layout_context);

    Ok(CallToolResult::success(json!({ "layout_context_id": context_id })))
}
```

### Runtime Tool Example: Builder Workflow

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderCreateParams {
    pub font_context_id: Uuid,
    pub layout_context_id: Uuid,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuilderBuildParams {
    pub font_context_id: Uuid,
    pub layout_context_id: Uuid,
    pub text: String,
    pub default_styles: Vec<StylePropertyJson>,
    pub ranged_styles: Vec<RangedStyleJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangedStyleJson {
    pub property: StylePropertyJson,
    pub range: RangeJson,
}

#[elicit_tool(
    plugin = "parley_builder",
    name = "builder_build_ranged",
    description = "Build layout with ranged styles (one-shot operation)"
)]
async fn builder_build_ranged(p: BuilderBuildParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut font_contexts = plugin.font_contexts.lock().unwrap();
    let mut layout_contexts = plugin.layout_contexts.lock().unwrap();

    let font_ctx = font_contexts.get_mut(&p.font_context_id)
        .ok_or_else(|| ErrorData::new("Font context not found"))?;
    let layout_ctx = layout_contexts.get_mut(&p.layout_context_id)
        .ok_or_else(|| ErrorData::new("Layout context not found"))?;

    // Create builder (short-lived)
    let mut builder = layout_ctx.ranged_builder(font_ctx, p.scale);

    // Apply default styles
    for style in &p.default_styles {
        let property = style.to_style_property()?;
        builder.push_default(&property);
    }

    // Apply ranged styles
    for ranged_style in &p.ranged_styles {
        let property = ranged_style.property.to_style_property()?;
        let range = ranged_style.range.to_range()?;
        builder.push(&property, range);
    }

    // Build layout
    let layout = builder.build(&p.text);
    let layout_id = Uuid::new_v4();

    drop(font_contexts);
    drop(layout_contexts);
    plugin.layouts.lock().unwrap().insert(layout_id, layout);

    Ok(CallToolResult::success(json!({ "layout_id": layout_id })))
}
```

---

## Phase 2: Style Property Creation (Dual-Mode)

**Goal:** Tools that both create StyleProperty JSON and emit code.

### StylePropertyJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StylePropertyJson {
    FontStack { families: Vec<String> },
    FontSize { size: f32 },
    FontWeight { weight: u16 },
    FontStyle { style: String },  // "Normal", "Italic", "Oblique"
    FontWidth { width: String },  // "Normal", "Condensed", "Expanded", etc.
    Brush { color: ColorJson },
    Underline { enabled: bool },
    UnderlineOffset { offset: f32 },
    UnderlineSize { size: f32 },
    UnderlineBrush { color: ColorJson },
    Strikethrough { enabled: bool },
    StrikethroughOffset { offset: f32 },
    StrikethroughSize { size: f32 },
    StrikethroughBrush { color: ColorJson },
    LineHeight { height: f32 },
    LetterSpacing { spacing: f32 },
    WordSpacing { spacing: f32 },
    Locale { locale: String },
    // ... and 6 more for FontVariations, FontFeatures, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorJson {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl StylePropertyJson {
    pub fn to_style_property(&self) -> Result<StyleProperty<SimpleBrush>, String> {
        match self {
            StylePropertyJson::FontSize { size } => {
                Ok(StyleProperty::FontSize(*size))
            }
            StylePropertyJson::FontWeight { weight } => {
                Ok(StyleProperty::FontWeight(FontWeight::new(*weight as f32)))
            }
            StylePropertyJson::Brush { color } => {
                Ok(StyleProperty::Brush(SimpleBrush {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                    a: color.a,
                }))
            }
            // ... convert all variants
            _ => todo!(),
        }
    }
}
```

### Dual-Mode Tool Example: Font Size

```rust
#[elicit_tool(
    plugin = "parley_style",
    name = "style_font_size",
    description = "Create font size style property",
    emit = Auto
)]
async fn style_font_size(p: StyleFontSizeParams) -> Result<CallToolResult, ErrorData> {
    let style_json = StylePropertyJson::FontSize { size: p.size };
    Ok(CallToolResult::success(json!({ "style": style_json })))
}

impl CustomEmit<StyleFontSizeParams> for StyleFontSizeEmit {
    fn emit_code(params: &StyleFontSizeParams) -> TokenStream {
        let size = params.size;
        quote! { StyleProperty::FontSize(#size) }
    }
}
```

### Dual-Mode Tool Example: Font Stack

```rust
#[elicit_tool(
    plugin = "parley_style",
    name = "style_font_stack",
    description = "Create font family stack",
    emit = Auto
)]
async fn style_font_stack(p: StyleFontStackParams) -> Result<CallToolResult, ErrorData> {
    let style_json = StylePropertyJson::FontStack {
        families: p.families.clone(),
    };
    Ok(CallToolResult::success(json!({ "style": style_json })))
}

impl CustomEmit<StyleFontStackParams> for StyleFontStackEmit {
    fn emit_code(params: &StyleFontStackParams) -> TokenStream {
        let families: Vec<TokenStream> = params.families.iter()
            .map(|f| {
                let family = f.as_str();
                quote! { #family }
            })
            .collect();

        quote! {
            StyleProperty::FontStack(FontStack::List(&[
                #(FamilyName::Named(#families)),*
            ]))
        }
    }
}
```

---

## Phase 3: Layout Serialization (Dual-Mode)

**Goal:** Serialize layout results with positioned glyphs and metrics.

### LayoutJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutJson {
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub is_rtl: bool,
    pub lines: Vec<LineJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineJson {
    pub index: usize,
    pub metrics: LineMetricsJson,
    pub text_range: RangeJson,
    pub break_reason: String,
    pub runs: Vec<RunJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineMetricsJson {
    pub baseline: f32,
    pub ascent: f32,
    pub descent: f32,
    pub leading: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunJson {
    pub font_size: f32,
    pub text_range: RangeJson,
    pub glyphs: Vec<GlyphJson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlyphJson {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub advance: f32,
    pub cluster: u32,
}
```

### Dual-Mode Tool Example: Layout to JSON

```rust
#[elicit_tool(
    plugin = "parley_layout",
    name = "layout_to_json",
    description = "Serialize entire layout with positioned glyphs",
    emit = Auto
)]
async fn layout_to_json(p: LayoutToJsonParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let layouts = plugin.layouts.lock().unwrap();
    let layout = layouts.get(&p.layout_id)
        .ok_or_else(|| ErrorData::new("Layout not found"))?;

    let layout_json = serialize_layout(layout);
    Ok(CallToolResult::success(json!({ "layout": layout_json })))
}

fn serialize_layout(layout: &Layout<SimpleBrush>) -> LayoutJson {
    let lines: Vec<LineJson> = layout.lines()
        .enumerate()
        .map(|(i, line)| {
            let metrics = line.metrics();
            let runs: Vec<RunJson> = line.runs()
                .map(|run| {
                    // Note: Actual glyph iteration requires more complex API
                    // This is simplified for the plan
                    RunJson {
                        font_size: run.font_size(),
                        text_range: RangeJson::from_range(run.text_range()),
                        glyphs: vec![],  // TODO: extract glyph positions
                    }
                })
                .collect();

            LineJson {
                index: i,
                metrics: LineMetricsJson {
                    baseline: metrics.baseline,
                    ascent: metrics.ascent,
                    descent: metrics.descent,
                    leading: metrics.leading,
                },
                text_range: RangeJson::from_range(line.text_range()),
                break_reason: format!("{:?}", line.break_reason()),
                runs,
            }
        })
        .collect();

    LayoutJson {
        width: layout.width(),
        height: layout.height(),
        scale: layout.scale(),
        is_rtl: layout.is_rtl(),
        lines,
    }
}
```

---

## Phase 4: Layout Operations (Runtime-Only)

**Goal:** Line breaking and alignment operations.

### Runtime Tool Example: Break Lines

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutBreakLinesParams {
    pub layout_id: Uuid,
    pub max_advance: f32,
}

#[elicit_tool(
    plugin = "parley_layout",
    name = "layout_break_lines",
    description = "Compute line breaks with maximum advance (width)"
)]
async fn layout_break_lines(p: LayoutBreakLinesParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut layouts = plugin.layouts.lock().unwrap();
    let layout = layouts.get_mut(&p.layout_id)
        .ok_or_else(|| ErrorData::new("Layout not found"))?;

    layout.break_all_lines(p.max_advance);

    Ok(CallToolResult::success(json!({
        "layout_id": p.layout_id,
        "line_count": layout.len()
    })))
}
```

### Runtime Tool Example: Align

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutAlignParams {
    pub layout_id: Uuid,
    pub container_width: Option<f32>,
    pub alignment: String,  // "Start", "Center", "End", "Justify"
}

#[elicit_tool(
    plugin = "parley_layout",
    name = "layout_align",
    description = "Apply alignment to layout"
)]
async fn layout_align(p: LayoutAlignParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut layouts = plugin.layouts.lock().unwrap();
    let layout = layouts.get_mut(&p.layout_id)
        .ok_or_else(|| ErrorData::new("Layout not found"))?;

    let alignment = match p.alignment.as_str() {
        "Start" => Alignment::Start,
        "Center" => Alignment::Center,
        "End" => Alignment::End,
        "Justify" => Alignment::Justify,
        _ => return Err(ErrorData::new("Unknown alignment")),
    };

    layout.align(p.container_width, alignment);

    Ok(CallToolResult::success(json!({ "layout_id": p.layout_id })))
}
```

---

## Phase 5: Fragment Tools (Code Generation)

**Goal:** Generate complete text layout code.

### Fragment Tool Example: Builder Code

```rust
#[elicit_tool(
    plugin = "parley_fragments",
    name = "emit_ranged_builder",
    description = "Generate RangedBuilder code with styles",
    emit = Auto
)]
async fn emit_ranged_builder(p: EmitRangedBuilderParams) -> Result<CallToolResult, ErrorData> {
    let default_styles = p.default_styles.iter()
        .map(emit_style_property)
        .collect::<Vec<_>>();

    let ranged_styles = p.ranged_styles.iter()
        .map(|rs| {
            let prop = emit_style_property(&rs.property);
            let range = emit_range(&rs.range);
            format!("builder.push(&{}, {});", prop, range)
        })
        .collect::<Vec<_>>()
        .join("\n    ");

    let code = format!(
        r#"let mut builder = layout_ctx.ranged_builder(font_ctx, {});

// Default styles
{}

// Ranged styles
{}

// Build layout
let layout = builder.build("{}");
"#,
        p.scale,
        default_styles.iter()
            .map(|s| format!("builder.push_default(&{});", s))
            .collect::<Vec<_>>()
            .join("\n"),
        ranged_styles,
        p.text.replace("\"", "\\\"")
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Fragment Tool Example: Complete Assembly

```rust
#[elicit_tool(
    plugin = "parley_fragments",
    name = "assemble_parley_binary",
    description = "Generate complete executable with text layout",
    emit = Auto
)]
async fn assemble_parley_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_with_layout(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "description": "Complete parley text layout binary"
    })))
}

fn generate_main_with_layout(p: &AssembleParams) -> String {
    format!(
        r#"use parley::{{FontContext, LayoutContext, StyleProperty}};

fn main() {{
    // Create contexts
    let mut font_ctx = FontContext::new();
    let mut layout_ctx = LayoutContext::new();

    // Build layout
    {}

    // Break lines
    layout.break_all_lines({});

    // Print results
    println!("Width: {{}}", layout.width());
    println!("Height: {{}}", layout.height());
    println!("Lines: {{}}", layout.len());
}}
"#,
        p.builder_code,
        p.max_advance
    )
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Context registry: `context_registry.rs` (20 runtime tools)
3. **Phase 1c** — Builder workflow: one-shot build operations (30 runtime tools)
4. **Phase 1d** — `just check elicit_parley`; fix compilation
5. **Phase 2a** — Style dual-mode tools: font properties (20 dual-mode tools)
6. **Phase 2b** — Style dual-mode tools: text properties (20 dual-mode tools)
7. **Phase 2c** — Style dual-mode tools: color/brush (20 dual-mode tools)
8. **Phase 2d** — `just check elicit_parley`
9. **Phase 3a** — Layout operations: break/align (40 runtime tools)
10. **Phase 3b** — Layout serialization: JSON output (40 dual-mode tools)
11. **Phase 3c** — `just check elicit_parley`
12. **Phase 4a** — Line/run queries: metrics and iteration (50 runtime tools)
13. **Phase 4b** — Font queries: database inspection (20 runtime tools)
14. **Phase 4c** — `just check elicit_parley`
15. **Phase 5a** — Fragment tools: builder code gen (15 fragment tools)
16. **Phase 5b** — Fragment tools: context setup (10 fragment tools)
17. **Phase 5c** — Fragment tools: layout queries (10 fragment tools)
18. **Phase 5d** — Fragment tools: assembly (5 fragment tools)
19. **Phase 5e** — `just check elicit_parley`
20. **Phase 6** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Runtime Context Management | 20 | UUID → FontContext/LayoutContext |
| Runtime Builder Workflow | 30 | One-shot build operations |
| Runtime Layout Operations | 40 | Break lines, align, query |
| Runtime Line/Run Queries | 25 | Metrics, text ranges, glyphs |
| Runtime Glyph Queries | 25 | Positions, advances, IDs |
| Runtime Font Queries | 20 | Font database inspection |
| Dual-Mode Style Properties | 50 | `emit = Auto` + CustomEmit |
| Dual-Mode Brush/Color | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Font Types | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Layout Serialization | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Text Ranges | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Alignment Types | 10 | `emit = Auto` + CustomEmit |
| Dual-Mode Line Break Types | 10 | `emit = Auto` + CustomEmit |
| Fragment Context Setup | 10 | Code generation only |
| Fragment Builder Code | 15 | Code generation only |
| Fragment Layout Computation | 10 | Code generation only |
| Fragment Assembly | 5 | Code generation only |
| **Total** | **380** | |

---

## Key Advantages

1. **Rich Text**: Full typography support (kerning, ligatures, OpenType)
2. **Bidirectional**: RTL/LTR text handling (Arabic, Hebrew)
3. **International**: Locale-aware line breaking, multi-script
4. **High Quality**: HarfBuzz shaping engine integration
5. **Font Control**: OpenType features, variation axes
6. **Positioned Output**: Exact glyph coordinates for rendering
7. **CSS-like**: Familiar styling properties (font-weight, line-height)
8. **Linebender Ecosystem**: Used by xilem, masonry, vello

---

## Comparison to Other Shadow Crates

| Aspect | taffy | parley |
|--------|-------|--------|
| **Domain** | Box layout (flexbox/grid) | Text layout (shaping/breaking) |
| **Statefulness** | Tree of boxes | Contexts + layouts |
| **Complexity** | CSS layout algorithms | Typography (HarfBuzz, OpenType) |
| **Output** | Box positions | Glyph positions |
| **Runtime tools** | 180 (53%) | 160 (42%) |
| **Dual-mode tools** | 120 (35%) | 180 (47%) |
| **Total tools** | 340 | 380 |
| **Use case** | UI layout | Text rendering |

**Both are stateful and "straightforward":**
- Natural JSON serialization for inputs/outputs
- Synchronous operations
- CSS-like properties
- Visual domains (UI vs text)

**Parley adds typography complexity:**
- Font shaping (HarfBuzz)
- OpenType features/variations
- Bidirectional text
- Unicode line breaking

---

## Integration with UI Libraries

### xilem/masonry

xilem uses parley for text. elicit_parley enables:
- AI-generated rich text layouts
- Typography exploration tools
- Font feature testing

### vello

vello renders parley layouts. elicit_parley enables:
- GPU-rendered text generation
- Vector text for design tools

### Custom Renderers

Any renderer can use parley's positioned glyphs:
- PDF generation
- Canvas rendering
- Game engine text
- Terminal UI

---

## Sources

- [parley - Rust (docs.rs)](https://docs.rs/parley/latest/parley/)
- [RangedBuilder - Rust](https://docs.rs/parley/latest/parley/struct.RangedBuilder.html)
- [Layout - Rust](https://docs.rs/parley/latest/parley/layout/struct.Layout.html)
- [Style - Rust](https://docs.rs/parley/latest/parley/layout/struct.Style.html)
- [StyleProperty - Rust](https://docs.rs/parley/latest/parley/style/enum.StyleProperty.html)
- [GitHub - linebender/parley](https://github.com/linebender/parley)
- [parley - crates.io](https://crates.io/crates/parley)
