# Prompt Tree — Implementation Plan

> **Premise:** `#[derive(Elicit)]` generates a static, compile-time-checkable
> AccessKit node tree that represents the full prompt structure of a type — what
> the agent will be asked, in what order, with what options — without running an
> elicitation.
>
> **Goal:** Full prompt transparency. Given any `T: Elicitation`, produce the
> exact assembled prompt the agent would receive at each step, as a structured
> tree that the typestate visualizer can annotate and a developer can inspect
> without adding up nodes in their head.

---

## Motivation

Elicitation is a conversation between the framework and an agent. Right now the
developer has to mentally simulate that conversation: trace the derive output,
sum the `#[prompt]` attributes across the type graph, factor in which style is
active, append the options list for enums — and only then know what the agent
actually sees.

The typestate visualizer already renders the *shape* of a type as a graph. This
plan annotates each node in that graph with its prompt text and, for compound
types, exposes the full assembled prompt as a single traversal so the developer
can read the complete script the agent receives.

---

## Design

### The `PromptTree` type

A static, owned tree of prompt nodes. Lives in `crates/elicitation/src/prompt_tree.rs`.

```rust
/// The prompt structure of a type, as a static tree.
///
/// Built entirely from `String` values and `Vec` allocations — no
/// communicator, no async, no runtime elicitation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PromptTree {
    /// A scalar value with a single prompt (bool, numeric, String, etc.).
    Leaf {
        /// The prompt text sent to the agent.
        prompt: String,
        /// The type name, for display.
        type_name: String,
    },

    /// An enum — the agent picks one variant from a finite set.
    Select {
        /// The base prompt text.
        prompt: String,
        /// The type name.
        type_name: String,
        /// Variant labels in declaration order.
        options: Vec<String>,
        /// For variants that carry fields, the sub-tree elicited after selection.
        branches: Vec<Option<Box<PromptTree>>>,
    },

    /// A struct — the agent answers a sequence of field prompts.
    Survey {
        /// The top-level prompt for this struct, if any.
        prompt: Option<String>,
        /// The type name.
        type_name: String,
        /// Ordered list of (field_name, sub-tree) pairs.
        fields: Vec<(String, Box<PromptTree>)>,
    },

    /// A binary yes/no step.
    Affirm {
        /// The prompt text.
        prompt: String,
        /// The type name.
        type_name: String,
    },
}
```

### The `ElicitPromptTree` trait

```rust
/// Types that can describe their prompt structure statically.
pub trait ElicitPromptTree {
    /// Return the static prompt tree for this type.
    ///
    /// Pure function: no allocations beyond the tree itself, no side effects,
    /// same result every call. Safe to call at startup or in tests.
    fn prompt_tree() -> PromptTree;

    /// Return the complete assembled prompt strings in elicitation order.
    ///
    /// For a `Leaf` or `Affirm` this is a single string. For a `Survey` this
    /// is one string per field (in order). For a `Select` this is the base
    /// prompt with options appended — the exact text passed to
    /// `communicator.send_prompt()`.
    fn assembled_prompts() -> Vec<AssembledPrompt> {
        collect_assembled_prompts(&Self::prompt_tree())
    }
}

/// A single assembled prompt, as the agent would receive it.
#[derive(Debug, Clone)]
pub struct AssembledPrompt {
    /// The full prompt string including options list for enums.
    pub text: String,
    /// The path through the type tree to this step (e.g. `["address", "port"]`).
    pub path: Vec<String>,
    /// Which interaction paradigm this step uses.
    pub kind: PromptKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptKind {
    Leaf,
    Select,
    Survey,
    Affirm,
}
```

`collect_assembled_prompts` is a free function that walks the `PromptTree`
depth-first and assembles each prompt exactly as the generated `elicit()` body
does — appending the numbered options list for `Select` variants, etc. This
lives alongside `PromptTree` in `prompt_tree.rs`.

---

## AccessKit Integration

`PromptTree` maps onto an AccessKit node tree with no loss of information.
This is the bridge to the typestate visualizer and to any AT (screen reader)
that wraps an elicitation UI.

### Role mapping

| `PromptTree` variant | AccessKit `Role` |
|---|---|
| `Leaf` | `Role::TextField` |
| `Select` | `Role::ComboBox` (options as `Role::ListBoxOption` children) |
| `Survey` | `Role::Form` (fields as named children) |
| `Affirm` | `Role::CheckBox` |

### Property mapping

| `PromptTree` field | AccessKit `Node` property |
|---|---|
| `prompt` / `prompt: Option` | `node.set_name(prompt)` |
| `type_name` | `node.set_description(type_name)` |
| `options` (Select) | one `ListBoxOption` child per entry, name = label |
| field name (Survey) | child node name = field name |

### `to_accesskit_tree()` method

```rust
impl PromptTree {
    /// Convert to an AccessKit `TreeUpdate` for use in a visualizer or AT bridge.
    ///
    /// The root node receives `NodeId(0)`. Each child gets an auto-incremented
    /// `NodeId`. The returned `TreeUpdate` is self-contained and has no
    /// dependency on a live UI context.
    pub fn to_accesskit_tree(&self) -> accesskit::TreeUpdate {
        // ...
    }
}
```

This method lives behind `#[cfg(feature = "accesskit")]` so the core crate does
not grow a hard dependency on accesskit.

---

## Derive Integration

`#[derive(Elicit)]` currently generates:
- `Prompt` impl
- `Select` / `Survey` / `Affirm` impl
- `Elicitation` impl (the async `elicit()` method)
- `ElicitIntrospect` impl (type graph metadata)
- Proof methods (kani/verus/creusot)

This plan adds: **`ElicitPromptTree` impl**.

### For enums (`enum_impl.rs`)

```rust
impl ElicitPromptTree for #name {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Select {
            prompt: <Self as elicitation::Prompt>::prompt()
                .unwrap_or(stringify!(#name)),
            type_name: stringify!(#name),
            options: vec![#(stringify!(#variant_idents)),*],
            branches: vec![#(#branch_trees),*],
        }
    }
}
```

Unit variants produce `None` branches. Tuple/struct variants produce
`Some(Box::new(PromptTree::Survey { ... }))` with the variant's fields.

### For structs (`struct_impl.rs`)

```rust
impl ElicitPromptTree for #name {
    fn prompt_tree() -> elicitation::PromptTree {
        elicitation::PromptTree::Survey {
            prompt: <Self as elicitation::Prompt>::prompt(),
            type_name: stringify!(#name),
            fields: vec![
                #(
                    (
                        stringify!(#field_idents),
                        Box::new(<#field_types as elicitation::ElicitPromptTree>::prompt_tree()),
                    )
                ),*
            ],
        }
    }
}
```

### Blanket impls for primitives

All types that already implement `Elicitation` manually (bool, numerics, String,
etc.) get blanket `ElicitPromptTree` impls that return `PromptTree::Leaf { prompt,
type_name }` or `PromptTree::Affirm { prompt, type_name }` as appropriate. These
live in `crates/elicitation/src/prompt_tree.rs` alongside the type definition.

---

## `assembled_prompts()` format

The assembled prompt for a `Select` step mirrors exactly what the generated
`elicit()` body constructs and passes to `send_prompt`:

```
{base_prompt}

Options:
1. VariantA
2. VariantB
3. VariantC

Respond with the number (1-3) or exact label:
```

The `Survey` walk produces one `AssembledPrompt` per field, in declaration order,
so the developer reads the script top-to-bottom exactly as the agent receives it.

---

## Typestate Visualizer Integration

The existing type graph (`TypeGraphKey`, `ElicitIntrospect`, Mermaid/DOT
renderers) renders nodes as type names with their interaction pattern
(`Select` / `Survey` / `Affirm`). Prompt tree data adds two annotation layers:

1. **Node label**: append the `prompt` text to the node label in the rendered graph.
2. **Tooltip / hover**: full assembled prompt string as a Mermaid `tooltip`
   or DOT `tooltip` attribute on the node.

The renderer already has a `PatternDetails` enum for node metadata. A new
`prompt_text: Option<String>` field on `PatternDetails` carries the prompt, set
by calling `T::prompt_tree()` during graph construction.

---

## Feature Flag

```toml
# Cargo.toml (elicitation crate)
[features]
prompt-tree = []                        # PromptTree + ElicitPromptTree + derive support
prompt-tree-accesskit = [               # to_accesskit_tree() method
    "prompt-tree",
    "dep:accesskit",
]
```

`prompt-tree` is off by default. The derive generates the `ElicitPromptTree`
impl only when the feature is active (checked at proc-macro crate compile time
via `#[cfg(feature = "prompt-tree")]`).

---

## Compile-Time Checking

Because `PromptTree` is a plain Rust enum with `String` fields, several
properties are checkable without running the program:

- **Prompt completeness**: a `const fn` or test helper that traverses
  `T::prompt_tree()` and asserts every `Leaf` / `Affirm` / `Select` has a
  non-empty `prompt`. Catches `#[prompt("")]` accidents at test time.
- **Depth bound**: assert `prompt_tree_depth::<T>() <= MAX_DEPTH` in tests.
- **Option count**: assert a `Select` node has at least 2 options.

These live in `crates/elicitation/tests/prompt_tree_test.rs`.

---

## Implementation Steps

### Step 1 — Core types (`crates/elicitation`)

- Add `crates/elicitation/src/prompt_tree.rs` with `PromptTree`, `ElicitPromptTree`
  trait, `AssembledPrompt`, `PromptKind`, and `collect_assembled_prompts`.
- Add blanket `ElicitPromptTree` impls for all primitive `Elicitation` types
  (bool, numerics, String, PathBuf, Duration, SystemTime, IpAddr, etc.).
- Export from `lib.rs` under `#[cfg(feature = "prompt-tree")]`.
- Add `prompt-tree` feature to `Cargo.toml`.

### Step 2 — Derive support (`crates/elicitation_derive`)

- In `enum_impl.rs`, generate `ElicitPromptTree` impl for enums (gated on
  `#[cfg(feature = "prompt-tree")]` at proc-macro compile time).
- In `struct_impl.rs`, generate `ElicitPromptTree` impl for structs.
- Recurse into variant fields and struct fields via `<FieldType as ElicitPromptTree>::prompt_tree()`.

### Step 3 — AccessKit bridge (`crates/elicitation`, `prompt-tree-accesskit` feature)

- Add `accesskit` as an optional dependency.
- Implement `PromptTree::to_accesskit_tree() -> accesskit::TreeUpdate`.
- Add `NodeId` allocation helper (sequential counter, no UUID dependency needed).

### Step 4 — Typestate visualizer annotation

- Add `prompt_text: Option<String>` to `PatternDetails` in the type graph.
- In the Mermaid/DOT renderers, append prompt text to node labels and populate
  `tooltip` attributes.
- Call `T::prompt_tree()` (when the feature is active) during graph construction
  via the `ElicitIntrospect` metadata callback.

### Step 5 — Tests

- `crates/elicitation/tests/prompt_tree_test.rs`:
  - Leaf/Affirm/Select/Survey construction and traversal
  - `assembled_prompts()` output matches expected strings
  - Prompt completeness helper
  - Regression: derived unit-variant enums produce non-empty `Select` trees
- `crates/elicitation/tests/prompt_tree_accesskit_test.rs` (feature-gated):
  - `to_accesskit_tree()` produces correct roles, names, child counts

---

## Non-Goals

- **Live elicitation state**: `PromptTree` is a static template. Current values,
  focus, and selection state are not represented. Use the `ElicitCommunicator`
  transcript for that.
- **Style-specific trees**: The initial implementation uses the default style.
  A follow-on can add `prompt_tree_with_style(style: T::Style) -> PromptTree`
  once the core machinery is in place.
- **`elicit_accesskit` shadow crate integration**: That crate exposes AccessKit
  as MCP tools. This plan is orthogonal — it generates AccessKit trees from
  derived type metadata. The two can share the `to_accesskit_tree()` output but
  are architecturally separate.
