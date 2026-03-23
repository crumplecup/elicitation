# elicit_accesskit — Implementation Plan

> **Premise:** Expose accesskit's accessibility schema as MCP tools for both runtime tree management and code generation.
> **Approach:** Completionist harvesting using dual-mode tools (primary), runtime-only tools (tree state), and fragment tools (accessibility code gen).

---

## Why AccessKit is Unique

**Different from previous shadow crates:**
- **Pure data schema** (not computation like nalgebra/ndarray)
- **Not stateful computation** (not like taffy/parley layout engines)
- **Tree metadata** (semantic structure for assistive tech)
- **No visual output** (no layout, no glyphs, no boxes)
- **Platform abstraction** (screen readers, VoiceOver, JAWS, NVDA)

**Similar to straightforward crates:**
- **Natural serialization**: All properties are primitives/enums
- **Synchronous operations**: Tree construction is deterministic
- **Clear API flow**: Create nodes → Set properties → Build tree → Update
- **No async/lifetimes/closures**: All types are `'static` and serializable

**Domain characteristics:**
- **182 semantic roles** (Button, TextInput, Slider, Table, etc.)
- **22 actions** (Click, Focus, SetValue, ScrollIntoView, etc.)
- **50+ node properties** (name, value, bounds, states, relationships)
- **Assistive technology bridge** (screen readers, automation tools)

**Perfect fit for:**
- AI accessibility auditing (check semantic structure)
- Automated testing (simulate screen reader navigation)
- Accessibility-first UI generation (semantic-first design)
- Documentation generation (extract UI structure)
- Compliance checking (WCAG, Section 508)

---

## Core Constraint: Pure Data Schema

AccessKit's core is a **serializable data schema**:

```rust
pub struct Node {
    role: Role,                    // 182 variants
    children: Vec<NodeId>,         // Tree structure
    name: Option<Box<str>>,        // Accessible name
    value: Option<Box<str>>,       // Current value
    // ... 50+ more properties
}

pub struct Tree {
    root: NodeId,
    app_name: Option<String>,
    toolkit_name: Option<String>,
    toolkit_version: Option<String>,
}

pub struct TreeUpdate {
    nodes: Vec<(NodeId, Node)>,
    tree: Option<Tree>,
    focus: Option<NodeId>,
}
```

**Crossing the JSON boundary:**

| Category | Examples | MCP Strategy |
|----------|----------|--------------|
| Node properties | `name`, `value`, `role`, `bounds` | ✅ Dual-mode (serialize as JSON) |
| Role enum | 182 variants | ✅ Dual-mode (serialize as strings) |
| Action enum | 22 variants | ✅ Dual-mode (serialize as strings) |
| Tree structure | `NodeId`, children relationships | ✅ Dual-mode (serialize IDs as UUIDs) |
| TreeUpdate | Atomic updates | ✅ Dual-mode (serialize entire update) |
| Geometric types | `Rect`, `Point`, `Affine` | ✅ Dual-mode (serialize as JSON) |
| State flags | `is_disabled`, `is_selected`, etc. | ✅ Dual-mode (serialize as booleans) |
| Platform integration | Screen reader bridges | ⚠️ Runtime-only (platform-specific) |

**Key insight:** Unlike taffy/parley, accesskit is **pure data** with no computation:
- **No layout algorithm** (just semantic metadata)
- **No shaping engine** (just text properties)
- **No stateful contexts** (just tree snapshots)
- **Everything serializes** (entire tree → JSON)

This makes accesskit **dual-mode dominated** (90%+ of tools).

---

## Tool Breakdown: 450 Total

### Dual-Mode Tools (390)

Tools that both create Node/Tree JSON AND generate code:

#### Node Creation (30)
- `node_new` — Create node with role
- `node_with_role` — Node with specific role
- `node_button` — Create button node
- `node_text_input` — Create text input node
- `node_checkbox` — Create checkbox node
- `node_slider` — Create slider node
- `node_image` — Create image node
- `node_heading` — Create heading node (with level)
- `node_link` — Create link node
- And 21 more for common roles

#### Node Properties - Content (40)
- `node_set_name` — Set accessible name
- `node_set_value` — Set current value
- `node_set_description` — Set description
- `node_set_tooltip` — Set tooltip
- `node_set_author_id` — Set test ID
- `node_set_placeholder` — Set placeholder text
- `node_set_keyboard_shortcut` — Set keyboard shortcut
- `node_set_language` — Set language code
- `node_set_access_key` — Set access key
- And 31 more for content properties

#### Node Properties - Structure (30)
- `node_add_child` — Add child node ID
- `node_set_children` — Set all children
- `node_remove_child` — Remove child
- `node_clear_children` — Clear all children
- `node_is_hidden` — Check if hidden
- `node_set_hidden` — Set hidden state
- `node_set_bounds` — Set bounding box
- `node_set_transform` — Set affine transform
- `node_clips_children` — Set clipping
- And 21 more for structure properties

#### Node Properties - States (50)
- `node_set_disabled` — Set disabled state
- `node_set_selected` — Set selected state
- `node_set_expanded` — Set expanded state
- `node_set_modal` — Set modal state
- `node_set_busy` — Set busy state
- `node_set_visited` — Set visited state
- `node_set_required` — Set required state
- `node_set_multiselectable` — Set multiselectable
- `node_set_read_only` — Set read-only state
- `node_set_italic` — Set italic text
- `node_set_invalid` — Set invalid state (None/True/Grammar/Spelling)
- `node_set_toggled` — Set toggled state (Off/On/Mixed)
- `node_set_live` — Set live region (Off/Polite/Assertive)
- `node_set_orientation` — Set orientation (Horizontal/Vertical)
- `node_set_sort_direction` — Set sort direction
- And 35 more for state properties

#### Node Properties - Relationships (25)
- `node_set_active_descendant` — Set active descendant
- `node_add_control` — Add controlled node
- `node_set_controls` — Set all controls
- `node_add_labelled_by` — Add label node
- `node_set_labelled_by` — Set all labels
- `node_add_described_by` — Add description node
- `node_set_described_by` — Set all descriptions
- `node_set_owns` — Set owned nodes
- `node_set_radio_group` — Set radio group
- `node_add_flow_to` — Add flow target
- `node_set_flow_to` — Set flow targets
- And 14 more for relationships

#### Node Properties - Text (35)
- `node_set_character_lengths` — Set UTF-8 byte lengths
- `node_set_word_starts` — Set word boundary indices
- `node_set_character_positions` — Set character positions
- `node_set_character_widths` — Set character widths
- `node_set_font_family` — Set font family
- `node_set_font_size` — Set font size
- `node_set_font_weight` — Set font weight
- `node_set_foreground_color` — Set text color
- `node_set_background_color` — Set background color
- `node_set_text_align` — Set text alignment
- `node_set_text_direction` — Set text direction (LTR/RTL)
- `node_set_text_decoration` — Set underline/overline/strikethrough
- And 23 more for text properties

#### Node Properties - Table (20)
- `node_set_row_index` — Set row index
- `node_set_column_index` — Set column index
- `node_set_row_span` — Set row span
- `node_set_column_span` — Set column span
- `node_set_row_count` — Set row count
- `node_set_column_count` — Set column count
- `node_set_aria_row_index` — Set ARIA row index
- `node_set_aria_column_index` — Set ARIA column index
- And 12 more for table properties

#### Node Properties - Lists (15)
- `node_set_position_in_set` — Set position in set
- `node_set_size_of_set` — Set size of set
- `node_set_hierarchical_level` — Set hierarchical level
- And 12 more for list properties

#### Actions (30)
- `node_add_action` — Add supported action
- `node_supports_action` — Check if action supported
- `node_add_click_action` — Add click action
- `node_add_focus_action` — Add focus action
- `node_add_increment_action` — Add increment action
- `node_add_decrement_action` — Add decrement action
- `node_add_scroll_action` — Add scroll actions
- `node_add_set_value_action` — Add set value action
- `node_add_custom_action` — Add custom action
- `action_click` — Create click action
- `action_focus` — Create focus action
- `action_set_value` — Create set value action
- `action_scroll_into_view` — Create scroll into view action
- And 17 more for actions

#### Role Types (182)
One tool per role variant for creating typed nodes:
- `role_button` — Button role
- `role_text_input` — Text input role
- `role_checkbox` — Checkbox role
- `role_slider` — Slider role
- `role_heading` — Heading role
- `role_image` — Image role
- `role_link` — Link role
- `role_table` — Table role
- `role_grid` — Grid role
- `role_list` — List role
- And 172 more for all role variants

#### Tree Management (20)
- `tree_new` — Create tree
- `tree_set_root` — Set root node ID
- `tree_set_app_name` — Set application name
- `tree_set_toolkit_name` — Set toolkit name
- `tree_set_toolkit_version` — Set toolkit version
- `tree_set_focus` — Set focused node
- `tree_update_create` — Create tree update
- `tree_update_add_node` — Add node to update
- `tree_update_remove_node` — Remove node from update
- `tree_update_set_tree` — Set tree in update
- `tree_update_set_focus` — Set focus in update
- And 9 more for tree operations

#### Serialization (10)
- `node_to_json` — Serialize node to JSON
- `tree_to_json` — Serialize tree to JSON
- `tree_update_to_json` — Serialize update to JSON
- `node_from_json` — Deserialize node from JSON
- `tree_from_json` — Deserialize tree from JSON
- And 5 more for serialization

### Runtime-Only Tools (30)

UUID-keyed handles for persistent trees and platform integration:

#### Tree Registry (15)
- `tree_create_handle` — Create persistent tree handle
- `tree_get_handle` — Get tree by handle
- `tree_update_handle` — Update tree by handle
- `tree_delete_handle` — Delete tree handle
- `tree_get_node` — Get node from tree
- `tree_traverse_bfs` — Breadth-first traversal
- `tree_traverse_dfs` — Depth-first traversal
- `tree_find_by_role` — Find nodes by role
- `tree_find_by_name` — Find nodes by name
- And 6 more for tree queries

#### Platform Integration (15)
- `platform_push_update` — Send update to platform
- `platform_handle_action` — Handle incoming action
- `platform_set_focus` — Set platform focus
- `platform_get_focused` — Get platform focused node
- And 11 more for platform bridges

### Fragment Tools (30)

Code generation for accessibility implementations:

#### Node Construction Code (10)
- `emit_node_builder` — Generate Node builder code
- `emit_node_with_properties` — Generate node with properties
- `emit_role_specific_node` — Generate typed node code
- And 7 more for node construction

#### Tree Construction Code (10)
- `emit_tree_builder` — Generate Tree builder code
- `emit_tree_hierarchy` — Generate parent-child code
- `emit_tree_update` — Generate TreeUpdate code
- And 7 more for tree construction

#### Complete Assembly (10)
- `assemble_accesskit_binary` — Generate complete executable
- `emit_accessibility_layer` — Generate accessibility layer
- `emit_test_harness` — Generate accessibility tests
- And 7 more for assembly patterns

---

## Serialization Strategy

### Node JSON

```json
{
  "role": "Button",
  "name": "Submit",
  "description": "Submit the form",
  "bounds": {
    "x": 10.0,
    "y": 20.0,
    "width": 100.0,
    "height": 40.0
  },
  "actions": ["Click", "Focus"],
  "is_disabled": false,
  "is_selected": false,
  "children": ["uuid-child-1", "uuid-child-2"]
}
```

### Complex Node (Text Input)

```json
{
  "role": "TextInput",
  "name": "Email Address",
  "value": "user@example.com",
  "placeholder": "Enter your email",
  "description": "Your email for account creation",
  "is_required": true,
  "is_read_only": false,
  "invalid": "None",
  "actions": [
    "Focus",
    "SetValue",
    "SetTextSelection"
  ],
  "bounds": {
    "x": 0.0,
    "y": 0.0,
    "width": 300.0,
    "height": 32.0
  }
}
```

### Tree Structure

```json
{
  "root": "uuid-root-node",
  "app_name": "My Application",
  "toolkit_name": "MyUI",
  "toolkit_version": "1.0.0",
  "focus": "uuid-focused-node",
  "nodes": {
    "uuid-root-node": {
      "role": "Window",
      "name": "Main Window",
      "children": ["uuid-toolbar", "uuid-content"]
    },
    "uuid-toolbar": {
      "role": "Toolbar",
      "name": "Main Toolbar",
      "children": ["uuid-btn-1", "uuid-btn-2"]
    },
    "uuid-btn-1": {
      "role": "Button",
      "name": "Save",
      "actions": ["Click", "Focus"]
    }
  }
}
```

### TreeUpdate

```json
{
  "nodes": [
    {
      "id": "uuid-node-1",
      "node": {
        "role": "Button",
        "name": "Updated Name",
        "is_disabled": true
      }
    },
    {
      "id": "uuid-node-2",
      "node": {
        "role": "TextInput",
        "value": "New Value"
      }
    }
  ],
  "tree": {
    "root": "uuid-root",
    "focus": "uuid-node-2"
  },
  "focus": "uuid-node-2"
}
```

### Table Example

```json
{
  "role": "Table",
  "name": "User List",
  "row_count": 10,
  "column_count": 3,
  "children": ["uuid-header-row", "uuid-row-1", "uuid-row-2"],
  "nodes": {
    "uuid-header-row": {
      "role": "Row",
      "row_index": 0,
      "children": ["uuid-col-name", "uuid-col-email", "uuid-col-actions"]
    },
    "uuid-col-name": {
      "role": "ColumnHeader",
      "name": "Name",
      "column_index": 0
    }
  }
}
```

---

## Phase 1: Node Property Tools (Dual-Mode)

**Goal:** Establish dual-mode pattern for node creation and property setting.

### Crate Structure

```
crates/elicit_accesskit/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── node_tools.rs        # Dual-mode node creation
    ├── property_tools.rs    # Dual-mode property setters
    ├── role_tools.rs        # Dual-mode role constructors (182 tools)
    ├── action_tools.rs      # Dual-mode action tools
    ├── tree_tools.rs        # Dual-mode tree management
    ├── tree_registry.rs     # Runtime tree handles
    ├── fragments.rs         # Code generation tools
    └── serde_types.rs       # JSON wrappers
```

### Cargo.toml

```toml
[package]
name = "elicit_accesskit"
version = "0.1.0"
edition = "2021"

[dependencies]
elicitation = { workspace = true }
elicitation_derive.workspace = true
accesskit = "0.24"
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid.workspace = true
rmcp.workspace = true
tracing.workspace = true

[features]
emit = ["dep:quote", "elicitation/emit"]
```

### Dual-Mode Tool Example: Node Creation

```rust
use elicitation_derive::elicit_tool;
use accesskit::{Node, Role};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeNewParams {
    pub role: String,  // "Button", "TextInput", etc.
}

#[elicit_tool(
    plugin = "accesskit_node",
    name = "node_new",
    description = "Create new accessibility node with role",
    emit = Auto
)]
async fn node_new(p: NodeNewParams) -> Result<CallToolResult, ErrorData> {
    let role = parse_role(&p.role)?;
    let mut node = Node::new(role);
    let node_json = NodeJson::from_node(&node);

    Ok(CallToolResult::success(json!({ "node": node_json })))
}

impl CustomEmit<NodeNewParams> for NodeNewEmit {
    fn emit_code(params: &NodeNewParams) -> TokenStream {
        let role = params.role.parse::<TokenStream>().unwrap();
        quote! { Node::new(Role::#role) }
    }
}
```

### NodeJson Wrapper

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeJson {
    pub role: String,
    pub name: Option<String>,
    pub value: Option<String>,
    pub description: Option<String>,
    pub children: Vec<String>,  // UUIDs
    pub bounds: Option<RectJson>,
    pub actions: Vec<String>,
    pub is_disabled: bool,
    pub is_selected: bool,
    pub is_expanded: Option<bool>,
    // ... 40+ more fields
}

impl NodeJson {
    pub fn from_node(node: &Node) -> Self {
        NodeJson {
            role: format!("{:?}", node.role()),
            name: node.name().map(|s| s.to_string()),
            value: node.value().map(|s| s.to_string()),
            description: node.description().map(|s| s.to_string()),
            children: node.children()
                .iter()
                .map(|id| format!("{:?}", id))
                .collect(),
            bounds: node.bounds().map(RectJson::from_rect),
            actions: vec![],  // TODO: extract from node
            is_disabled: node.is_disabled(),
            is_selected: node.is_selected(),
            is_expanded: node.is_expanded(),
            // ... serialize all fields
        }
    }

    pub fn to_node(&self) -> Result<Node, String> {
        let role = parse_role(&self.role)?;
        let mut node = Node::new(role);

        if let Some(ref name) = self.name {
            node.set_name(name.clone());
        }
        if let Some(ref value) = self.value {
            node.set_value(value.clone());
        }
        if self.is_disabled {
            node.set_disabled();
        }
        // ... set all properties

        Ok(node)
    }
}
```

### Dual-Mode Tool Example: Set Name

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSetNameParams {
    pub name: String,
}

#[elicit_tool(
    plugin = "accesskit_node",
    name = "node_set_name",
    description = "Set accessible name for node",
    emit = Auto
)]
async fn node_set_name(p: NodeSetNameParams) -> Result<CallToolResult, ErrorData> {
    // Runtime: just returns the property update
    Ok(CallToolResult::success(json!({
        "property": "name",
        "value": p.name
    })))
}

impl CustomEmit<NodeSetNameParams> for NodeSetNameEmit {
    fn emit_code(params: &NodeSetNameParams) -> TokenStream {
        let name = &params.name;
        quote! { node.set_name(#name.to_string()) }
    }
}
```

### Dual-Mode Tool Example: Role-Specific Constructor

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeButtonParams {
    pub label: String,
}

#[elicit_tool(
    plugin = "accesskit_node",
    name = "node_button",
    description = "Create button node with label",
    emit = Auto
)]
async fn node_button(p: NodeButtonParams) -> Result<CallToolResult, ErrorData> {
    let mut node = Node::new(Role::Button);
    node.set_name(p.label.clone());
    node.add_action(Action::Click);
    node.add_action(Action::Focus);

    let node_json = NodeJson::from_node(&node);
    Ok(CallToolResult::success(json!({ "node": node_json })))
}

impl CustomEmit<NodeButtonParams> for NodeButtonEmit {
    fn emit_code(params: &NodeButtonParams) -> TokenStream {
        let label = &params.label;
        quote! {
            {
                let mut node = Node::new(Role::Button);
                node.set_name(#label.to_string());
                node.add_action(Action::Click);
                node.add_action(Action::Focus);
                node
            }
        }
    }
}
```

---

## Phase 2: Role Tools (182 Dual-Mode)

**Goal:** One tool per Role variant for semantic node creation.

### Role Tool Generator Pattern

```rust
// Generate 182 tools programmatically

macro_rules! role_tool {
    ($name:ident, $role:ident, $doc:literal) => {
        #[elicit_tool(
            plugin = "accesskit_role",
            name = stringify!($name),
            description = $doc,
            emit = Auto
        )]
        async fn $name(_: RoleParams) -> Result<CallToolResult, ErrorData> {
            let role_json = RoleJson { role: stringify!($role).to_string() };
            Ok(CallToolResult::success(json!({ "role": role_json })))
        }

        impl CustomEmit<RoleParams> for [<$name Emit>] {
            fn emit_code(_: &RoleParams) -> TokenStream {
                quote! { Role::$role }
            }
        }
    };
}

// Generate all 182 tools
role_tool!(role_button, Button, "Button role");
role_tool!(role_text_input, TextInput, "Text input role");
role_tool!(role_checkbox, CheckBox, "Checkbox role");
role_tool!(role_slider, Slider, "Slider role");
// ... 178 more
```

---

## Phase 3: Tree Management (Dual-Mode)

**Goal:** Tree construction and update serialization.

### Dual-Mode Tool Example: Tree Creation

```rust
#[elicit_tool(
    plugin = "accesskit_tree",
    name = "tree_new",
    description = "Create accessibility tree",
    emit = Auto
)]
async fn tree_new(p: TreeNewParams) -> Result<CallToolResult, ErrorData> {
    let tree_json = TreeJson {
        app_name: p.app_name.clone(),
        toolkit_name: p.toolkit_name.clone(),
        toolkit_version: p.toolkit_version.clone(),
        root: None,
        focus: None,
    };

    Ok(CallToolResult::success(json!({ "tree": tree_json })))
}

impl CustomEmit<TreeNewParams> for TreeNewEmit {
    fn emit_code(params: &TreeNewParams) -> TokenStream {
        let app_name = params.app_name.as_ref();
        let toolkit_name = params.toolkit_name.as_ref();

        quote! {
            {
                let mut tree = Tree::new();
                tree.app_name = Some(#app_name.to_string());
                tree.toolkit_name = Some(#toolkit_name.to_string());
                tree
            }
        }
    }
}
```

### Dual-Mode Tool Example: Tree Update

```rust
#[elicit_tool(
    plugin = "accesskit_tree",
    name = "tree_update_create",
    description = "Create tree update with node changes",
    emit = Auto
)]
async fn tree_update_create(p: TreeUpdateParams) -> Result<CallToolResult, ErrorData> {
    let update_json = TreeUpdateJson {
        nodes: p.nodes.clone(),
        tree: p.tree.clone(),
        focus: p.focus.clone(),
    };

    Ok(CallToolResult::success(json!({ "update": update_json })))
}

impl CustomEmit<TreeUpdateParams> for TreeUpdateCreateEmit {
    fn emit_code(params: &TreeUpdateParams) -> TokenStream {
        let node_updates = params.nodes.iter().map(emit_node_update);

        quote! {
            TreeUpdate {
                nodes: vec![
                    #(#node_updates),*
                ],
                tree: None,  // TODO: emit tree
                focus: None,  // TODO: emit focus
            }
        }
    }
}
```

---

## Phase 4: Runtime Tree Registry

**Goal:** Persistent tree handles for stateful workflows.

### Runtime Tool Example: Tree Handle

```rust
pub struct AccessKitPlugin {
    trees: Arc<Mutex<HashMap<Uuid, Tree>>>,
    nodes: Arc<Mutex<HashMap<Uuid, HashMap<Uuid, Node>>>>,  // tree_id → (node_id → Node)
}

#[elicit_tool(
    plugin = "accesskit_registry",
    name = "tree_create_handle",
    description = "Create persistent tree handle"
)]
async fn tree_create_handle(p: TreeCreateHandleParams) -> Result<CallToolResult, ErrorData> {
    let tree = p.tree.to_tree()?;
    let tree_id = Uuid::new_v4();

    let plugin = get_plugin();
    plugin.trees.lock().unwrap().insert(tree_id, tree);
    plugin.nodes.lock().unwrap().insert(tree_id, HashMap::new());

    Ok(CallToolResult::success(json!({ "tree_id": tree_id })))
}

#[elicit_tool(
    plugin = "accesskit_registry",
    name = "tree_add_node",
    description = "Add node to tree handle"
)]
async fn tree_add_node(p: TreeAddNodeParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let mut nodes = plugin.nodes.lock().unwrap();
    let tree_nodes = nodes.get_mut(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    let node = p.node.to_node()?;
    let node_id = Uuid::new_v4();
    tree_nodes.insert(node_id, node);

    Ok(CallToolResult::success(json!({ "node_id": node_id })))
}

#[elicit_tool(
    plugin = "accesskit_registry",
    name = "tree_traverse_bfs",
    description = "Breadth-first traversal of tree"
)]
async fn tree_traverse_bfs(p: TreeTraverseParams) -> Result<CallToolResult, ErrorData> {
    let plugin = get_plugin();
    let nodes = plugin.nodes.lock().unwrap();
    let tree_nodes = nodes.get(&p.tree_id)
        .ok_or_else(|| ErrorData::new("Tree not found"))?;

    // BFS traversal
    let mut queue = VecDeque::new();
    queue.push_back(p.root_id);
    let mut visited = Vec::new();

    while let Some(node_id) = queue.pop_front() {
        if let Some(node) = tree_nodes.get(&node_id) {
            visited.push(NodeJson::from_node(node));
            // Add children to queue
            for child_id in node.children() {
                // Convert NodeId to Uuid (requires mapping)
                // queue.push_back(child_uuid);
            }
        }
    }

    Ok(CallToolResult::success(json!({ "nodes": visited })))
}
```

---

## Phase 5: Fragment Tools (Code Generation)

**Goal:** Generate complete accessibility implementations.

### Fragment Tool Example: Tree Builder

```rust
#[elicit_tool(
    plugin = "accesskit_fragments",
    name = "emit_tree_builder",
    description = "Generate complete tree construction code",
    emit = Auto
)]
async fn emit_tree_builder(p: EmitTreeBuilderParams) -> Result<CallToolResult, ErrorData> {
    let node_creations = p.nodes.iter()
        .map(|n| emit_node_creation(n))
        .collect::<Vec<_>>()
        .join("\n");

    let hierarchy = p.hierarchy.iter()
        .map(|h| format!("{}.push_child({});", h.parent, h.child))
        .collect::<Vec<_>>()
        .join("\n");

    let code = format!(
        r#"use accesskit::{{Node, Tree, Role, Action}};

fn build_accessibility_tree() -> Tree {{
    // Create nodes
    {}

    // Build hierarchy
    {}

    // Create tree
    let mut tree = Tree::new();
    tree.app_name = Some("{}".to_string());
    tree.root = root_node_id;

    tree
}}
"#,
        node_creations,
        hierarchy,
        p.app_name
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

### Fragment Tool Example: Complete Assembly

```rust
#[elicit_tool(
    plugin = "accesskit_fragments",
    name = "assemble_accesskit_binary",
    description = "Generate complete executable with accessibility tree",
    emit = Auto
)]
async fn assemble_accesskit_binary(p: AssembleParams) -> Result<CallToolResult, ErrorData> {
    let cargo_toml = generate_cargo_toml(&p);
    let main_rs = generate_main_with_tree(&p);

    Ok(CallToolResult::success(json!({
        "Cargo.toml": cargo_toml,
        "src/main.rs": main_rs,
        "description": "Complete accesskit binary project"
    })))
}

fn generate_main_with_tree(p: &AssembleParams) -> String {
    format!(
        r#"use accesskit::{{Node, Tree, TreeUpdate, Role}};

fn main() {{
    // Build tree
    {}

    // Create update
    let update = TreeUpdate {{
        nodes: vec![
            // ... node updates
        ],
        tree: Some(tree),
        focus: None,
    }};

    // In a real app, push update to platform adapter
    println!("Tree created with {{}} nodes", update.nodes.len());
}}
"#,
        p.tree_construction
    )
}
```

---

## Implementation Order

1. **Phase 1a** — Crate scaffold: `Cargo.toml`, `lib.rs`, `serde_types.rs`
2. **Phase 1b** — Node creation dual-mode tools: (30 tools)
3. **Phase 1c** — Node property dual-mode tools: content (40 tools)
4. **Phase 1d** — `just check elicit_accesskit`
5. **Phase 2a** — Node property dual-mode tools: structure (30 tools)
6. **Phase 2b** — Node property dual-mode tools: states (50 tools)
7. **Phase 2c** — Node property dual-mode tools: relationships (25 tools)
8. **Phase 2d** — `just check elicit_accesskit`
9. **Phase 3a** — Node property dual-mode tools: text (35 tools)
10. **Phase 3b** — Node property dual-mode tools: table (20 tools)
11. **Phase 3c** — Node property dual-mode tools: lists (15 tools)
12. **Phase 3d** — `just check elicit_accesskit`
13. **Phase 4a** — Action dual-mode tools: (30 tools)
14. **Phase 4b** — Role dual-mode tools: (182 tools - code generated)
15. **Phase 4c** — `just check elicit_accesskit`
16. **Phase 5a** — Tree management dual-mode tools: (20 tools)
17. **Phase 5b** — Serialization dual-mode tools: (10 tools)
18. **Phase 5c** — `just check elicit_accesskit`
19. **Phase 6a** — Runtime tree registry: (15 tools)
20. **Phase 6b** — Runtime platform integration: (15 tools)
21. **Phase 6c** — `just check elicit_accesskit`
22. **Phase 7a** — Fragment tools: node construction (10 tools)
23. **Phase 7b** — Fragment tools: tree construction (10 tools)
24. **Phase 7c** — Fragment tools: assembly (10 tools)
25. **Phase 7d** — `just check elicit_accesskit`
26. **Phase 8** — Wire into `elicit_server` emit chain

---

## Tool Count Summary

| Category | Count | Implementation Strategy |
|----------|-------|------------------------|
| Dual-Mode Node Creation | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Content Properties | 40 | `emit = Auto` + CustomEmit |
| Dual-Mode Structure Properties | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode State Properties | 50 | `emit = Auto` + CustomEmit |
| Dual-Mode Relationship Properties | 25 | `emit = Auto` + CustomEmit |
| Dual-Mode Text Properties | 35 | `emit = Auto` + CustomEmit |
| Dual-Mode Table Properties | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode List Properties | 15 | `emit = Auto` + CustomEmit |
| Dual-Mode Actions | 30 | `emit = Auto` + CustomEmit |
| Dual-Mode Roles | 182 | `emit = Auto` + CustomEmit (code generated) |
| Dual-Mode Tree Management | 20 | `emit = Auto` + CustomEmit |
| Dual-Mode Serialization | 10 | `emit = Auto` + CustomEmit |
| Runtime Tree Registry | 15 | UUID → Tree/Node mapping |
| Runtime Platform Integration | 15 | Platform-specific bridges |
| Fragment Node Construction | 10 | Code generation only |
| Fragment Tree Construction | 10 | Code generation only |
| Fragment Assembly | 10 | Code generation only |
| **Total** | **450** | |

---

## Key Advantages

1. **Pure Data Schema**: No computation, just semantic metadata
2. **Complete Serialization**: Entire tree serializes to JSON
3. **182 Semantic Roles**: Comprehensive UI element vocabulary
4. **Platform Abstraction**: Works with VoiceOver, JAWS, NVDA, Narrator
5. **WCAG Compliance**: Built-in accessibility semantics
6. **Automated Testing**: Programmatic accessibility inspection
7. **AI Auditing**: Agents can analyze semantic structure
8. **UI Framework Integration**: Used by xilem, egui, dioxus

---

## Comparison to Other Shadow Crates

| Aspect | taffy | parley | accesskit |
|--------|-------|--------|-----------|
| **Domain** | Box layout | Text layout | Accessibility metadata |
| **Computation** | Layout algorithm | Text shaping | None (pure data) |
| **Statefulness** | Tree mutations | Contexts | Optional (can be pure) |
| **Runtime tools** | 53% | 42% | 7% |
| **Dual-mode tools** | 35% | 47% | 87% |
| **Total tools** | 340 | 380 | 450 |
| **Output** | Box positions | Glyph positions | Semantic metadata |
| **Use case** | UI layout | Typography | Screen readers |

**AccessKit is unique:**
- **Dual-mode dominated** (87% vs 35-47% for others)
- **Pure data** (no computation like layout/shaping)
- **Optional statefulness** (can be pure functions)
- **Largest tool count** (450) due to 182 roles + 50+ properties

**All share "straightforward" DNA:**
- Natural JSON serialization
- Synchronous operations
- Clear API taxonomy

---

## Integration with UI Frameworks

### xilem

xilem uses accesskit for accessibility. elicit_accesskit enables:
- AI-generated accessible UIs (semantic-first design)
- Automated accessibility audits
- WCAG compliance checking

### egui

egui has accesskit integration. elicit_accesskit enables:
- Accessibility layer generation
- Screen reader testing
- Compliance validation

### dioxus

dioxus web/desktop uses accesskit. elicit_accesskit enables:
- Accessible component generation
- Semantic HTML mapping
- Automation testing

### Custom Renderers

Any UI toolkit can use accesskit:
- Game engine UIs
- Custom widgets
- Embedded systems

---

## Accessibility Use Cases

### AI Accessibility Auditing

Agents can:
- Traverse accessibility tree
- Check for missing labels/descriptions
- Verify keyboard navigation
- Validate ARIA attributes
- Generate compliance reports

### Automated Testing

```json
{
  "test": "Login form accessibility",
  "assertions": [
    {
      "find": { "role": "TextInput", "name": "Username" },
      "expect": { "is_required": true, "has_label": true }
    },
    {
      "find": { "role": "TextInput", "name": "Password" },
      "expect": { "is_required": true, "type": "password" }
    },
    {
      "find": { "role": "Button", "name": "Log In" },
      "expect": { "action": "Click" }
    }
  ]
}
```

### Semantic-First UI Generation

Agents can generate UIs from semantic specifications:

```json
{
  "intent": "Create user registration form",
  "fields": [
    { "label": "Email", "type": "email", "required": true },
    { "label": "Password", "type": "password", "required": true },
    { "label": "Confirm Password", "type": "password", "required": true }
  ],
  "submit": "Create Account"
}
```

Agent generates:
1. Semantic accessibility tree (accesskit)
2. Layout (taffy)
3. Text rendering (parley)
4. Visual rendering (vello)

---

## Sources

- [accesskit - Rust (docs.rs)](https://docs.rs/accesskit/latest/accesskit/)
- [Node - Rust](https://docs.rs/accesskit/latest/accesskit/struct.Node.html)
- [Role - Rust](https://docs.rs/accesskit/latest/accesskit/enum.Role.html)
- [Action - Rust](https://docs.rs/accesskit/latest/accesskit/enum.Action.html)
- [GitHub - AccessKit/accesskit](https://github.com/AccessKit/accesskit)
- [AccessKit Official Site](https://accesskit.dev/)
- [How it works - AccessKit](https://accesskit.dev/how-it-works/)
- [accesskit - crates.io](https://crates.io/crates/accesskit)
