# elicit_taffy — Implementation Plan

> **Premise:** Expose taffy's flexbox/grid/block layout engine as MCP tools for both runtime computation and code generation.
> **Approach:** Completionist harvesting using runtime-only tools (tree state), dual-mode tools (style creation), and fragment tools (layout code gen).

---

## Why Taffy is Straightforward

**Shares characteristics with nalgebra/ndarray:**
- **Natural serialization**: Style properties are plain structs/enums
- **Synchronous operations**: Layout computation is pure/deterministic
- **No async/lifetimes/closures**: All types are `'static` and serializable
- **Clear API flow**: Create tree → Set styles → Compute layout → Query results

**Unique characteristics:**
- **Stateful by nature**: TaffyTree manages a node tree (not pure functions)
- **CSS familiarity**: Web developers recognize flexbox/grid properties
- **UI library foundation**: Used by bevy_ui, dioxus, xilem, etc.
- **Small, focused API**: ~50 tree methods + 39 style properties

**Not like other shadow crates:**
- No trait-heavy API (unlike num-traits)
- No macros to harvest (unlike Leptos)
- Not data-parallel (unlike ndarray)
- Not math-heavy (unlike nalgebra)

**Perfect fit for:**
- UI layout agents (generate responsive layouts)
- Design system tools (compose flexbox/grid patterns)
- Accessibility checkers (analyze layout structures)
- Visual regression testing (layout snapshots)

---

## Core Constraint: Stateful Tree

Taffy's core API is a stateful tree with interior mutability:

```rust
pub struct TaffyTree {
    nodes: SlotMap<NodeId, NodeData>,  // Interior mutability
    // NodeData contains: Style, Layout, children, parent, cache
}
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Style properties | `Style { display, size, margin, ... }` | ✅ Dual-mode (serialize as JSON) |
| Layout results | `Layout { location, size, ... }` | ✅ Dual-mode (serialize as JSON) |
| Tree structure | `TaffyTree` with nodes | ✅ Runtime-only (UUID → TaffyTree handle) |
| NodeId references | `NodeId` (internal SlotMap key) | ✅ Runtime-only (map to UUIDs) |
| Measurement functions | `FnMut(Size, AvailableSpace) -> Size` | ⚠️ Fragment tools (generate code) |
| Layout algorithms | Flexbox/Grid computation | ✅ Runtime + Fragment (both) |

**Key insight:** TaffyTree cannot serialize (interior mutability), but **all inputs and outputs** are serializable:
- **Input**: Style struct (39 fields of enums/primitives)
- **Output**: Layout struct (location + size)
- **Tree operations**: Runtime-only via UUID handles

---

## Tool Breakdown: 340 Total

### Runtime-Only Tools (180)

UUID-keyed handles for persistent trees and nodes:

#### Tree Management (20)
- `tree_create` — Create new TaffyTree
- `tree_delete` — Remove tree from registry
- `tree_clear` — Remove all nodes from tree
- `tree_clone` — Deep clone tree
- `tree_print` — Debug print tree structure
- `tree_enable_rounding` — Enable pixel-grid snapping
- `tree_disable_rounding` — Disable rounding
- `tree_total_node_count` — Get node count
- And 12 more for tree configuration

#### Node Creation (25)
- `node_create_leaf` — Create leaf node with style
- `node_create_leaf_with_measure` — Leaf with custom measurement
- `node_create_with_children` — Node with children
- `node_remove` — Remove node from tree
- `node_clone` — Clone node and subtree
- And 20 more for node lifecycle

#### Node Hierarchy (30)
- `node_add_child` — Attach child to parent
- `node_insert_child_at_index` — Insert at specific position
- `node_set_children` — Replace all children
- `node_remove_child` — Detach child
- `node_remove_child_at_index` — Remove by index
- `node_replace_child_at_index` — Swap child
- `node_parent` — Get parent NodeId
- `node_children` — Get child list
- `node_child_at_index` — Get specific child
- And 21 more for hierarchy manipulation

#### Style Management (30)
- `node_set_style` — Update node style
- `node_get_style` — Retrieve style
- `node_set_display` — Set display mode
- `node_set_position` — Set position type
- `node_set_size` — Set width/height
- `node_set_min_size` — Set min constraints
- `node_set_max_size` — Set max constraints
- `node_set_margin` — Set margin
- `node_set_padding` — Set padding
- `node_set_border` — Set border width
- `node_set_gap` — Set gap between items
- `node_set_flex_direction` — Set main axis
- `node_set_flex_wrap` — Set wrapping
- `node_set_align_items` — Set cross-axis alignment
- `node_set_justify_content` — Set main-axis distribution
- And 15 more for individual style properties

#### Layout Computation (25)
- `layout_compute` — Compute layout for tree
- `layout_compute_with_measure` — Compute with custom measure function
- `layout_get` — Retrieve layout for node
- `layout_get_unrounded` — Get unrounded layout values
- `layout_get_detailed` — Get grid-specific layout info
- `layout_mark_dirty` — Invalidate cache
- `layout_is_dirty` — Check if needs recomputation
- And 18 more for layout queries

#### Context & Measurement (20)
- `node_set_context` — Set measurement context data
- `node_get_context` — Retrieve context
- `node_clear_context` — Remove context
- And 17 more for context management

#### Tree Traversal (30)
- `tree_traverse_breadth_first` — BFS iteration
- `tree_traverse_depth_first` — DFS iteration
- `tree_find_by_id` — Locate node
- `tree_filter_nodes` — Filter by predicate
- `tree_map_nodes` — Transform node data
- And 25 more for traversal patterns

### Dual-Mode Tools (120)

Tools that both create Style/Layout JSON AND generate code:

#### Style Creation (40)
- `style_default` — Create default style
- `style_flex` — Create flexbox style
- `style_grid` — Create grid style
- `style_block` — Create block style
- `style_with_size` — Style with dimensions
- `style_with_margin` — Style with margin
- `style_with_padding` — Style with padding
- `style_with_flex_direction` — Style with direction
- `style_with_align_items` — Style with alignment
- And 31 more for style builders

#### Dimension Types (20)
- `dimension_length` — Fixed length (px)
- `dimension_percent` — Percentage
- `dimension_auto` — Automatic sizing
- `dimension_min_content` — Intrinsic minimum
- `dimension_max_content` — Intrinsic maximum
- `dimension_fit_content` — Fit to available space
- `length_percentage` — Length or percentage
- And 13 more for dimension values

#### Spacing Types (15)
- `rect_all` — Same value for all sides
- `rect_vertical_horizontal` — Two-value shorthand
- `rect_top_horizontal_bottom` — Three-value shorthand
- `rect_trbl` — Four-value (top, right, bottom, left)
- And 11 more for spacing patterns

#### Alignment & Distribution (15)
- `align_items_start` — Align to start
- `align_items_end` — Align to end
- `align_items_center` — Center alignment
- `align_items_stretch` — Stretch to fill
- `justify_content_start` — Justify to start
- `justify_content_space_between` — Space between items
- `justify_content_space_around` — Space around items
- And 8 more for alignment values

#### Grid Template Types (15)
- `grid_track_fixed` — Fixed-size track
- `grid_track_fr` — Flexible fraction unit
- `grid_track_minmax` — Min/max constraint
- `grid_track_repeat` — Repeat pattern
- `grid_auto_flow_row` — Row-first auto placement
- `grid_auto_flow_column` — Column-first auto placement
- And 9 more for grid patterns

#### Layout Result Serialization (15)
- `layout_to_json` — Serialize Layout struct
- `layout_from_json` — Deserialize Layout
- `layout_tree_to_json` — Entire tree layout snapshot
- And 12 more for layout serialization

### Fragment Tools (40)

Code generation for layout computations:

#### Style Code Generation (15)
- `emit_style_builder` — Generate Style::builder() chain
- `emit_style_struct_literal` — Generate Style { ... } literal
- `emit_flexbox_container` — Generate flex container code
- `emit_grid_container` — Generate grid container code
- `emit_block_container` — Generate block container code
- And 10 more for style code patterns

#### Tree Construction Code (10)
- `emit_tree_creation` — Generate TaffyTree::new() code
- `emit_node_creation` — Generate new_leaf/new_with_children code
- `emit_tree_builder` — Generate complete tree construction
- And 7 more for tree building patterns

#### Layout Computation Code (10)
- `emit_compute_layout` — Generate compute_layout() call
- `emit_measure_function` — Generate measurement closure
- `emit_layout_query` — Generate layout() access code
- And 7 more for layout computation patterns

#### Complete Assembly (5)
- `assemble_taffy_binary` — Generate complete executable
- `emit_ui_component` — Generate UI component with layout
- `emit_test_harness` — Generate layout test
- And 2 more for assembly patterns

---

## Serialization Strategy

### Style Struct

```json
{
  "display": "Flex",
  "position": "Relative",
  "flex_direction": "Row",
  "flex_wrap": "NoWrap",
  "align_items": "Center",
  "justify_content": "SpaceBetween",
  "size": {
    "width": { "type": "Percent", "value": 100.0 },
    "height": { "type": "Auto" }
  },
  "margin": {
    "top": { "type": "Length", "value": 10.0 },
    "right": { "type": "Auto" },
    "bottom": { "type": "Length", "value": 10.0 },
    "left": { "type": "Auto" }
  },
  "padding": {
    "top": { "type": "Length", "value": 16.0 },
    "right": { "type": "Length", "value": 16.0 },
    "bottom": { "type": "Length", "value": 16.0 },
    "left": { "type": "Length", "value": 16.0 }
  },
  "gap": {
    "width": { "type": "Length", "value": 8.0 },
    "height": { "type": "Length", "value": 8.0 }
  }
}
```

### Layout Result

```json
{
  "location": {
    "x": 0.0,
    "y": 0.0
  },
  "size": {
    "width": 800.0,
    "height": 600.0
  },
  "content_size": {
    "width": 768.0,
    "height": 568.0
  },
  "border": {
    "top": 1.0,
    "right": 1.0,
    "bottom": 1.0,
    "left": 1.0
  },
  "padding": {
    "top": 16.0,
    "right": 16.0,
    "bottom": 16.0,
    "left": 16.0
  }
}
```

### Tree Structure (Runtime Handle)

```json
{
  "tree_id": "uuid-of-tree",
  "root_node_id": "uuid-of-root",
  "nodes": {
    "uuid-of-root": {
      "style": { /* style JSON */ },
      "children": ["uuid-child-1", "uuid-child-2"]
    },
    "uuid-child-1": {
      "style": { /* style JSON */ },
      "children": []
    },
    "uuid-child-2": {
      "style": { /* style JSON */ },
      "children": []
    }
  }
}
```

### Layout Tree Result

```json
{
  "tree_id": "uuid-of-tree",
  "layouts": {
    "uuid-of-root": {
      "location": { "x": 0.0, "y": 0.0 },
      "size": { "width": 800.0, "height": 600.0 }
    },
    "uuid-child-1": {
      "location": { "x": 10.0, "y": 10.0 },
      "size": { "width": 380.0, "height": 580.0 }
    },
    "uuid-child-2": {
      "location": { "x": 410.0, "y": 10.0 },
      "size": { "width": 380.0, "height": 580.0 }
    }
  }
}
```

---

## Phase 1: Tree & Node Management (Runtime-Only)

**Goal:** Establish UUID-keyed registry pattern for persistent TaffyTree instances.

### Crate Structure

```
crates/elicit_taffy/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── tree_registry.rs   # UUID → TaffyTree mapping
    ├── node_mapping.rs    # NodeId ↔ UUID translation
    ├── style_tools.rs     # Dual-mode style creation
    ├── layout_tools.rs    # Dual-mode layout serialization
    ├── fragments.rs       # Code generation tools
    └── serde_types.rs     # JSON wrappers for Style/Layout
```

### Cargo.toml

```toml
[package]
name = "elicit_taffy"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
taffy = { version = "0.9", features = ["serde"] }
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
grid = ["taffy/grid"]
block_layout = ["taffy/block_layout"]
```

### Runtime Tool Example: Tree Creation

```rust
use elicitation_derive::elicit_tool;
use taffy::TaffyTree;

pub struct TaffyPlugin {
    trees: Arc<Mutex<HashMap<Uuid, TaffyTree>>>,
    // Map external UUID to internal NodeId per tree
    node_mappings: Arc<Mutex<HashMap<Uuid, HashMap<Uuid, NodeId>>>>,
}

#[elicit_tool(
    plugin = "taffy_tree",
    name = "tree_create",
    description = "Create new TaffyTree instance"
)]
async fn tree_create(_: TreeCreateParams) -> Result<CallToolResult, ErrorData> {
    let tree = TaffyTree::new();
    let tree_id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.trees.lock().unwrap().insert(tree_id, tree);
    plugin.node_mappings.lock().unwrap().insert(tree_id, HashMap::new());

    Ok(CallToolResult::success(json!({ "tree_id": tree_id })))
}
```

### Runtime Tool Example: Node Creation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCreateLeafParams {
    pub tree_id: Uuid,
    pub style: StyleJson,
}

#[elicit_tool(
    plugin = "taffy_tree",
    name = "node_create_leaf",
    description = "Create leaf node with style in tree"
)]
async fn node_create_leaf(p: NodeCreateLeafParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut trees = plugin.trees.lock().unwrap();
    let tree = trees.get_mut(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    let style = p.style.to_style()?;
    let node_id = tree.new_leaf(style)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    // Map internal NodeId to external UUID
    let node_uuid = Uuid::new_v4();
    let mut mappings = plugin.node_mappings.lock().unwrap();
    mappings.get_mut(&p.tree_id)
        .unwrap()
        .insert(node_uuid, node_id);

    Ok(CallToolResult::success(json!({
        "node_id": node_uuid,
        "tree_id": p.tree_id
    })))
}
```

### Runtime Tool Example: Layout Computation

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutComputeParams {
    pub tree_id: Uuid,
    pub root_node_id: Uuid,
    pub available_space: AvailableSpaceJson,
}

#[elicit_tool(
    plugin = "taffy_layout",
    name = "layout_compute",
    description = "Compute layout for tree"
)]
async fn layout_compute(p: LayoutComputeParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut trees = plugin.trees.lock().unwrap();
    let tree = trees.get_mut(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    let mappings = plugin.node_mappings.lock().unwrap();
    let node_map = mappings.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Node mapping not found"))?;
    let root_id = node_map.get(&p.root_node_id)
        .ok_or_else(|| ErrorData::new("Root node not found"))?;

    let available = p.available_space.to_size()?;
    tree.compute_layout(*root_id, available)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    Ok(CallToolResult::success(json!({
        "success": true,
        "tree_id": p.tree_id
    })))
}

#[elicit_tool(
    plugin = "taffy_layout",
    name = "layout_get",
    description = "Retrieve computed layout for node"
)]
async fn layout_get(p: LayoutGetParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let trees = plugin.trees.lock().unwrap();
    let tree = trees.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    let mappings = plugin.node_mappings.lock().unwrap();
    let node_map = mappings.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Node mapping not found"))?;
    let node_id = node_map.get(&p.node_id)
        .ok_or_else(|| ErrorData::new("Node not found"))?;

    let layout = tree.layout(*node_id)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    let layout_json = LayoutJson::from_layout(layout);
    Ok(CallToolResult::success(json!({ "layout": layout_json })))
}
```

---

## Phase 2: Style Creation (Dual-Mode)

**Goal:** Tools that both create Style JSON and emit Style construction code.

### StyleJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleJson {
    pub display: Option<String>,  // "Flex", "Grid", "Block", "None"
    pub position: Option<String>,  // "Relative", "Absolute"
    pub size: Option<SizeJson>,
    pub min_size: Option<SizeJson>,
    pub max_size: Option<SizeJson>,
    pub margin: Option<RectJson>,
    pub padding: Option<RectJson>,
    pub border: Option<RectJson>,
    pub gap: Option<SizeJson>,
    pub flex_direction: Option<String>,  // "Row", "Column", "RowReverse", "ColumnReverse"
    pub flex_wrap: Option<String>,  // "NoWrap", "Wrap", "WrapReverse"
    pub align_items: Option<String>,
    pub justify_content: Option<String>,
    // ... 20+ more fields for grid, etc.
}

impl StyleJson {
    pub fn to_style(&self) -> Result<Style, String> {
        let mut style = Style::default();

        if let Some(ref display) = self.display {
            style.display = match display.as_str() {
                "Flex" => Display::Flex,
                "Grid" => Display::Grid,
                "Block" => Display::Block,
                "None" => Display::None,
                _ => return Err(format!("Unknown display: {}", display)),
            };
        }

        if let Some(ref size) = self.size {
            style.size = size.to_size()?;
        }

        // ... parse all other fields

        Ok(style)
    }

    pub fn from_style(style: &Style) -> Self {
        StyleJson {
            display: Some(format!("{:?}", style.display)),
            position: Some(format!("{:?}", style.position)),
            size: Some(SizeJson::from_size(&style.size)),
            // ... serialize all fields
        }
    }
}
```

### Dual-Mode Tool Example: Style Builder

```rust
#[elicit_tool(
    plugin = "taffy_style",
    name = "style_flex",
    description = "Create flexbox container style",
    emit = Auto
)]
async fn style_flex(p: StyleFlexParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: construct StyleJson
    let style_json = StyleJson {
        display: Some("Flex".to_string()),
        flex_direction: Some(p.direction.clone()),
        align_items: p.align_items.clone(),
        justify_content: p.justify_content.clone(),
        gap: p.gap.as_ref().map(|g| SizeJson::from_dimension(g)),
        ..Default::default()
    };

    Ok(CallToolResult::success(json!({ "style": style_json })))
}

// Auto-generated CustomEmit impl:
impl CustomEmit<StyleFlexParams> for StyleFlexEmit {
    fn emit_code(params: &StyleFlexParams) -> TokenStream {
        let direction = params.direction.parse::<TokenStream>().unwrap();
        let align = params.align_items.as_ref()
            .map(|a| a.parse::<TokenStream>().unwrap());
        let justify = params.justify_content.as_ref()
            .map(|j| j.parse::<TokenStream>().unwrap());

        let mut fields = vec![
            quote! { display: Display::Flex },
            quote! { flex_direction: FlexDirection::#direction },
        ];

        if let Some(a) = align {
            fields.push(quote! { align_items: Some(AlignItems::#a) });
        }
        if let Some(j) = justify {
            fields.push(quote! { justify_content: Some(JustifyContent::#j) });
        }

        quote! {
            Style {
                #(#fields),*,
                ..Default::default()
            }
        }
    }
}
```

### Dual-Mode Tool Example: Dimension Values

```rust
#[elicit_tool(
    plugin = "taffy_style",
    name = "dimension_percent",
    description = "Create percentage dimension",
    emit = Auto
)]
async fn dimension_percent(p: DimensionPercentParams) -> Result<CallToolResult, ErrorData> {
    let dim_json = DimensionJson::Percent { value: p.percent };
    Ok(CallToolResult::success(json!({ "dimension": dim_json })))
}

impl CustomEmit<DimensionPercentParams> for DimensionPercentEmit {
    fn emit_code(params: &DimensionPercentParams) -> TokenStream {
        let value = params.percent;
        quote! { Dimension::Percent(#value) }
    }
}
```

---

## Phase 3: Layout Serialization (Dual-Mode)

**Goal:** Serialize layout results and emit layout query code.

### LayoutJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutJson {
    pub location: PointJson,
    pub size: SizeJson,
    pub content_size: SizeJson,
    pub border: RectJson,
    pub padding: RectJson,
}

impl LayoutJson {
    pub fn from_layout(layout: &Layout) -> Self {
        LayoutJson {
            location: PointJson {
                x: layout.location.x,
                y: layout.location.y,
            },
            size: SizeJson {
                width: layout.size.width,
                height: layout.size.height,
            },
            content_size: SizeJson {
                width: layout.content_size.width,
                height: layout.content_size.height,
            },
            border: RectJson::from_rect(&layout.border),
            padding: RectJson::from_rect(&layout.padding),
        }
    }
}
```

### Dual-Mode Tool Example: Layout Tree Snapshot

```rust
#[elicit_tool(
    plugin = "taffy_layout",
    name = "layout_tree_snapshot",
    description = "Capture entire tree layout as JSON",
    emit = Auto
)]
async fn layout_tree_snapshot(p: LayoutSnapshotParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let trees = plugin.trees.lock().unwrap();
    let tree = trees.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    let mappings = plugin.node_mappings.lock().unwrap();
    let node_map = mappings.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Node mapping not found"))?;

    let mut layouts = HashMap::new();
    for (uuid, node_id) in node_map.iter() {
        let layout = tree.layout(*node_id)
            .map_err(|e| ErrorData::new(e.to_string()))?;
        layouts.insert(*uuid, LayoutJson::from_layout(layout));
    }

    Ok(CallToolResult::success(json!({
        "tree_id": p.tree_id,
        "layouts": layouts
    })))
}

impl CustomEmit<LayoutSnapshotParams> for LayoutTreeSnapshotEmit {
    fn emit_code(params: &LayoutSnapshotParams) -> TokenStream {
        quote! {
            {
                let mut layouts = HashMap::new();
                for (uuid, node_id) in node_map.iter() {
                    let layout = tree.layout(*node_id)?;
                    layouts.insert(*uuid, layout);
                }
                layouts
            }
        }
    }
}
```

---

## Phase 4: Fragment Tools (Code Generation)

**Goal:** Generate complete layout computation code.

### Fragment Tool Example: Tree Builder

```rust
#[elicit_tool(
    plugin = "taffy_fragments",
    name = "emit_tree_builder",
    description = "Generate complete tree construction code",
    emit = Auto
)]
async fn emit_tree_builder(p: EmitTreeParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"let mut tree = TaffyTree::new();

// Create nodes
{}

// Build hierarchy
{}

// Compute layout
tree.compute_layout(root, Size::MAX_CONTENT)?;

// Access results
{}
"#,
        emit_node_creations(&p.nodes),
        emit_hierarchy(&p.hierarchy),
        emit_layout_queries(&p.queries)
    );

    Ok(CallToolResult::success(Content::text(code)))
}

fn emit_node_creations(nodes: &[NodeDef]) -> String {
    nodes.iter()
        .map(|node| {
            format!(
                "let {} = tree.new_leaf(Style {{ {} }})?;",
                node.name,
                emit_style_fields(&node.style)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Fragment Tool Example: Complete App

```rust
#[elicit_tool(
    plugin = "taffy_fragments",
    name = "assemble_taffy_binary",
    description = "Generate complete executable with taffy layout",
    emit = Auto
)]
async fn assemble_taffy_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_with_layout(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "description": "Complete taffy layout binary"
    })))
}

fn generate_main_with_layout(p: &AssembleParams) -> String {
    format!(
        r#"use taffy::prelude::*;

fn main() -> Result<(), taffy::TaffyError> {{
    // Create tree
    let mut tree = TaffyTree::new();

    {}

    // Compute layout
    tree.compute_layout(root, Size::MAX_CONTENT)?;

    // Print results
    tree.print_tree(root);

    Ok(())
}}
"#,
        p.tree_construction
    )
}
```

---

## Phase 5: Advanced Features

### Grid Layout Support (20 dual-mode tools)

```rust
#[elicit_tool(
    plugin = "taffy_style",
    name = "style_grid",
    description = "Create CSS Grid container style",
    emit = Auto
)]
async fn style_grid(p: StyleGridParams) -> Result<CallToolResult, ErrorData> {
    let style_json = StyleJson {
        display: Some("Grid".to_string()),
        grid_template_columns: Some(p.template_columns.clone()),
        grid_template_rows: Some(p.template_rows.clone()),
        gap: p.gap.as_ref().map(|g| SizeJson::from_dimension(g)),
        ..Default::default()
    };

    Ok(CallToolResult::success(json!({ "style": style_json })))
}
```

### Block Layout Support (15 dual-mode tools)

```rust
#[elicit_tool(
    plugin = "taffy_style",
    name = "style_block",
    description = "Create block layout container style",
    emit = Auto
)]
async fn style_block(p: StyleBlockParams) -> Result<CallToolResult, ErrorData> {
    let style_json = StyleJson {
        display: Some("Block".to_string()),
        text_align: p.text_align.clone(),
        ..Default::default()
    };

    Ok(CallToolResult::success(json!({ "style": style_json })))
}
```

### Measurement Functions (10 runtime tools)

```rust
#[elicit_tool(
    plugin = "taffy_layout",
    name = "layout_compute_with_measure",
    description = "Compute layout with custom text measurement"
)]
async fn layout_compute_with_measure(
    p: LayoutComputeMeasureParams,
) -> Result<CallToolResult, ErrorData> {
    // For text measurement, agent provides:
    // - font family, size, weight
    // - text content per node
    // We compute text dimensions and return

    // This is runtime-only because the measure function is a closure
    // Fragment tools can emit code that calls a user-provided measure fn

    let plugin = get_plugin();
    let mut trees = plugin.trees.lock().unwrap();
    let tree = trees.get_mut(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    // Simplified: use provided dimensions
    let measure_fn = |known_dims: Size<Option<f32>>, _available: Size<AvailableSpace>| {
        Size {
            width: known_dims.width.unwrap_or(p.default_width),
            height: known_dims.height.unwrap_or(p.default_height),
        }
    };

    let mappings = plugin.node_mappings.lock().unwrap();
    let node_map = mappings.get(&p.tree_id).unwrap();
    let root_id = node_map.get(&p.root_node_id).unwrap();

    tree.compute_layout_with_measure(*root_id, p.available_space.to_size()?, measure_fn)
        .map_err(|e| ErrorData::new(e.to_string()))?;

    Ok(CallToolResult::success(json!({ "success": true })))
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Tree registry: `tree_registry.rs`, `node_mapping.rs` (20 runtime tools)
3. **Phase 1c** — Node management: creation, hierarchy (55 runtime tools)
4. **Phase 1d** — `just check elicit_taffy`; fix compilation
5. **Phase 2a** — Style dual-mode tools: builders (40 dual-mode tools)
6. **Phase 2b** — Style runtime tools: setters (30 runtime tools)
7. **Phase 2c** — Dimension/spacing dual-mode tools: (35 dual-mode tools)
8. **Phase 2d** — `just check elicit_taffy`
9. **Phase 3a** — Layout computation runtime tools: (25 runtime tools)
10. **Phase 3b** — Layout serialization dual-mode tools: (15 dual-mode tools)
11. **Phase 3c** — Tree traversal runtime tools: (30 runtime tools)
12. **Phase 3d** — `just check elicit_taffy`
13. **Phase 4a** — Fragment tools: style code gen (15 fragment tools)
14. **Phase 4b** — Fragment tools: tree construction (10 fragment tools)
15. **Phase 4c** — Fragment tools: assembly (5 fragment tools)
16. **Phase 4d** — `just check elicit_taffy`
17. **Phase 5a** — Grid layout support: (20 dual-mode tools)
18. **Phase 5b** — Block layout support: (15 dual-mode tools)
19. **Phase 5c** — Measurement functions: (10 runtime tools)
20. **Phase 5d** — Alignment/distribution dual-mode tools: (15 dual-mode tools)
21. **Phase 5e** — `just check elicit_taffy`
22. **Phase 6** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Runtime Tree Management | 20 | UUID → TaffyTree registry |
| Runtime Node Lifecycle | 25 | NodeId ↔ UUID mapping |
| Runtime Node Hierarchy | 30 | Tree manipulation |
| Runtime Style Setters | 30 | Individual property updates |
| Runtime Layout Computation | 25 | Compute + query |
| Runtime Context/Measurement | 20 | Custom measure functions |
| Runtime Tree Traversal | 30 | BFS/DFS/filter/map |
| Dual-Mode Style Builders | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Dimensions | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Spacing | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Alignment | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Grid Types | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Layout Serialization | 15 | `emit = Auto` + CustomEmit |
| Fragment Style Code | 15 | Code generation only |
| Fragment Tree Construction | 10 | Code generation only |
| Fragment Layout Computation | 10 | Code generation only |
| Fragment Assembly | 5 | Code generation only |
| **Total** | **340** | |

---

## Key Advantages

1. **CSS Familiarity**: Web developers recognize flexbox/grid immediately
2. **Natural Serialization**: All style/layout types are plain structs
3. **Deterministic**: Layout computation is pure (same inputs → same outputs)
4. **UI Library Foundation**: Used by bevy_ui, dioxus, xilem, cosmic
5. **Small API Surface**: 50 tree methods + 39 style properties = focused scope
6. **Visual Workflows**: Agents can compose responsive layouts visually
7. **Design System Ready**: Perfect for generating component libraries
8. **Accessibility Testing**: Layout analysis for a11y validation

---

## Comparison to Other Shadow Crates

| Aspect | nalgebra | ndarray | taffy |
|--------|----------|---------|-------|
| **Domain** | Linear algebra | N-D arrays | UI layout |
| **Statefulness** | Stateless (pure functions) | Stateless (pure functions) | Stateful (tree structure) |
| **Serialization** | Matrices → JSON | Arrays → JSON | Style → JSON, Tree → UUID |
| **Primary strategy** | Dual-mode 73% | Dual-mode 77% | Runtime 53%, Dual-mode 35% |
| **Tools** | 480 | 520 | 340 |
| **Key feature** | Geometric types | Broadcasting | CSS layout algorithms |
| **Use case** | Graphics, physics | Scientific computing | UI frameworks |

**All three are "straightforward":**
- Natural JSON serialization for inputs/outputs
- Synchronous operations
- No async, no lifetimes, no closures (in core API)
- Clear API flow

**Taffy is unique:**
- **Stateful by design** (tree of nodes, not pure functions)
- **Runtime-heavy** (180/340 tools are runtime-only)
- **Smaller API** (340 tools vs 480-520 for nalgebra/ndarray)
- **Domain-specific** (UI layout, not general math)

---

## Integration with UI Libraries

### bevy_ui

bevy_ui uses taffy internally. elicit_taffy enables:
- AI-driven UI layout generation
- Responsive design composition
- Accessibility analysis
- Visual regression testing

### dioxus

dioxus uses taffy for native rendering. elicit_taffy enables:
- Component layout optimization
- Flexbox/grid pattern libraries
- Design system tooling

### xilem

xilem's layout engine is taffy-based. elicit_taffy enables:
- Declarative layout DSL generation
- Layout snapshot testing

---

## Sources

- [taffy - Rust (docs.rs)](https://docs.rs/taffy)
- [TaffyTree - Rust](https://docs.rs/taffy/latest/taffy/struct.TaffyTree.html)
- [Style - Rust](https://docs.rs/taffy/latest/taffy/style/struct.Style.html)
- [GitHub - DioxusLabs/taffy](https://github.com/DioxusLabs/taffy)
- [Taffy Layout (official site)](https://taffylayout.com/)
- [taffy - crates.io](https://crates.io/crates/taffy)
