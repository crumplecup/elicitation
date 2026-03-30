# elicit_accesskit

MCP tool transport for [`accesskit`](https://docs.rs/accesskit) — expose
accessibility tree types and node introspection as agent-callable MCP tools.

## What this crate provides

Three complementary layers:

1. **Copy-enum wrappers** — `serde` + `JsonSchema`-enabled newtypes for all 17
   accesskit copy enums (Role, Action, Invalid, …), so they can appear in MCP
   tool parameter schemas
2. **Struct wrappers** — serializable counterparts for geometry, colour,
   text-decoration, and identity types
3. **JSON intermediates** — `NodeJson` and `TreeUpdateJson`, which bridge the
   gap between `accesskit::Node`/`TreeUpdate` (no `serde`) and the MCP
   boundary, with full `From`/`Into` round-trips

---

## Why wrappers are needed

`accesskit` types are designed for in-process accessibility tree manipulation,
not data exchange.  Most are `Copy` enums with neither `serde` nor `schemars`
impls.  `accesskit::Node` and `accesskit::TreeUpdate` explicitly omit `Clone`
and `Serialize` because they own arena-allocated state.

Without these impls the types cannot appear in MCP tool parameter schemas.
The wrappers add the missing impls without modifying upstream code.  Where
`accesskit` provides `serde`/`schemars` via Cargo features, the wrappers
delegate to the inner type so schemas stay in sync.

---

## Copy-enum wrappers

All 17 accesskit copy enums are wrapped with the `accesskit_copy_enum!` macro.
Every wrapper implements `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`,
`Serialize` (transparent), `Deserialize` (transparent), `JsonSchema`
(delegated to inner), `Deref`, `From`/`Into`.

| Wrapper | Inner type | Notable variants |
|---|---|---|
| [`Role`] | `accesskit::Role` | 182 variants; `Button`, `TextInput`, `List`, … |
| [`Action`] | `accesskit::Action` | `Click`, `Focus`, `SetValue`, … |
| [`Invalid`] | `accesskit::Invalid` | `False`, `True`, `Grammar`, `Spelling` |
| [`Toggled`] | `accesskit::Toggled` | `False`, `True`, `Mixed` |
| [`Orientation`] | `accesskit::Orientation` | `Horizontal`, `Vertical` |
| [`TextDirection`] | `accesskit::TextDirection` | `LeftToRight`, `RightToLeft` |
| [`SortDirection`] | `accesskit::SortDirection` | `Ascending`, `Descending` |
| [`AriaCurrent`] | `accesskit::AriaCurrent` | `False`, `True`, `Page`, `Step`, … |
| [`AutoComplete`] | `accesskit::AutoComplete` | `None`, `Inline`, `List`, `Both` |
| [`Live`] | `accesskit::Live` | `Off`, `Polite`, `Assertive` |
| [`HasPopup`] | `accesskit::HasPopup` | `False`, `True`, `Menu`, `ListBox`, … |
| [`ListStyle`] | `accesskit::ListStyle` | `Circle`, `Disc`, `Image`, … |
| [`TextAlign`] | `accesskit::TextAlign` | `Left`, `Right`, `Center`, `Justify` |
| [`VerticalOffset`] | `accesskit::VerticalOffset` | `Subscript`, `Superscript` |
| [`TextDecorationStyle`] | `accesskit::TextDecorationStyle` | `Solid`, `Dotted`, `Dashed`, … |
| [`ScrollUnit`] | `accesskit::ScrollUnit` | `Page`, `Line` |
| [`ScrollHint`] | `accesskit::ScrollHint` | `TopLeft`, `BottomRight`, `TopEdge`, … |

### Reflect methods on `Role`

```rust
use elicit_accesskit::Role;
use accesskit::Role as AkRole;

let r = Role(AkRole::Button);
assert_eq!(r.name(), "Button");       // Pascal-case variant name
assert!(r.is_form_control());          // button, checkbox, combobox, …
assert!(!r.is_text_input());
assert!(!r.is_container());
assert!(!r.is_landmark());
```

### Reflect methods on `Action`

```rust
use elicit_accesskit::Action;

let a = Action(accesskit::Action::SetValue);
assert_eq!(a.name(), "SetValue");
assert!(a.is_value_action());          // SetValue, SetScrollOffset, …
assert!(!a.is_focus_action());

let f = Action(accesskit::Action::Focus);
assert!(f.is_focus_action());          // Focus, Blur
```

---

## Id wrappers

| Wrapper | Inner | Notes |
|---|---|---|
| [`NodeId`] | `accesskit::NodeId` | Backed by `u64`; serialises as number |
| [`TreeId`] | `accesskit::TreeId` | Backed by UUID; serialises as hex string |

```rust
use elicit_accesskit::NodeId;
use accesskit::NodeId as AkNodeId;

let id = AkNodeId::from(42u64);
let wrapped: NodeId = id.into();
let back: AkNodeId = wrapped.into();
assert_eq!(id, back);
```

---

## Struct wrappers

Simple structs with all fields serializable (delegating to inner where
possible, or mirroring field-by-field where not).

| Wrapper | Notable fields |
|---|---|
| [`Color`] | `red`, `green`, `blue`, `alpha` (all `u8`) |
| [`CustomAction`] | `id: i32`, `description: String` |
| [`TextPosition`] | `node: NodeId`, `character_index: usize` |
| [`TextSelection`] | `anchor: TextPosition`, `focus: TextPosition` |
| [`TextDecoration`] | `style`, `color` |
| [`Tree`] | `root: NodeId`, `toolkit_name`, `toolkit_version` |

---

## Geometry wrappers

Geometry types from the `kurbo` crate re-exported through `accesskit`.
The wrappers delegate `JsonSchema` to the inner type when `accesskit`'s
`schemars` feature is enabled.

| Wrapper | Inner | Fields |
|---|---|---|
| [`Rect`] | `accesskit::Rect` | `x0`, `y0`, `x1`, `y1` (`f64`) |
| [`Point`] | `accesskit::Point` | `x`, `y` (`f64`) |
| [`Size`] | `accesskit::Size` | `width`, `height` (`f64`) |
| [`Vec2`] | `accesskit::Vec2` | `x`, `y` (`f64`) |
| [`Affine`] | `accesskit::Affine` | `[f64; 6]` transform matrix |

---

## JSON intermediates

### `NodeJson`

`accesskit::Node` has no `Clone`, `Serialize`, or `Deserialize` (it owns
arena-allocated children and properties).  `NodeJson` is a fully serializable
mirror with all ~80 public properties:

```rust
use elicit_accesskit::{NodeJson, Role};
use accesskit::Role as AkRole;

// Build from scratch
let node = NodeJson::new(Role(AkRole::Button))
    .with_label("Submit".to_string())
    .with_is_disabled(false);

// Convert to a live accesskit::Node
let ak_node = accesskit::Node::from(node);

// Or extract from a live node
let extracted = NodeJson::from(&ak_node);
```

`NodeJson` implements `Serialize`, `Deserialize`, `JsonSchema`, `Clone`,
`Debug`, and `PartialEq`.  It is the primary type for passing node data
across the MCP boundary.

### `TreeUpdateJson`

`accesskit::TreeUpdate` likewise has no `serde`.  `TreeUpdateJson` provides
a serializable view:

```rust
use elicit_accesskit::{NodeId, TreeUpdateJson};
use accesskit::{Node, NodeId as AkNodeId, Role, Tree, TreeId, TreeUpdate};

let root_id = AkNodeId::from(1u64);
let update = TreeUpdate {
    nodes: vec![(root_id, Node::new(Role::RootWebArea))],
    tree: Some(Tree::new(root_id)),
    tree_id: TreeId::ROOT,
    focus: root_id,
};

let json_update = TreeUpdateJson::from(update);
// json_update is Serialize + Deserialize + JsonSchema
```

---

## Dependency

```toml
[dependencies]
elicit_accesskit = { path = "../elicit_accesskit" }
```

The crate re-exports `accesskit` transitively; no need to add it separately
unless you need `accesskit`-specific items not covered by the wrappers.
