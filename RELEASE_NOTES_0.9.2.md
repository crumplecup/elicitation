# Release Notes — elicitation 0.9.2

**Release date:** TBD

This release closes the proof gap: every elicitation type and every workflow
proposition now carries machine-checkable formal verification proofs across
three independent backends. Six new shadow crates bring tokio, sqlx, egui,
ratatui, AccessKit, and WCAG accessibility under the same contract-and-proof
discipline as the core library. **494 Kani proofs, 31 Creusot modules, and 186
Verus proofs** — all passing, all tracked.

---

## Architecture

The core story of 0.9.2: proof obligations are now enforced, not optional.

### `ElicitComplete` — the proof completeness gate

`ElicitComplete` is a marker supertrait that aggregates every obligation a type
must satisfy before it can participate in the elicitation ecosystem:

```rust
pub trait ElicitComplete:
    Elicitation          // interactive elicitation + kani/verus/creusot proofs
    + ElicitIntrospect   // structural metadata
    + ElicitSpec         // agent-browsable contract specs
    + Serialize + Deserialize + JsonSchema  // wire format
{ }
```

When you write `impl ElicitComplete for MyType {}`, the compiler accepts it
**only** if every supertrait's obligations are met — including non-empty proof
bodies for all three verification backends. This replaced the previous approach
where proof methods had default empty implementations, meaning types could
silently ship without any formal verification coverage.

The `Elicitation` trait's proof methods (`kani_proof()`, `verus_proof()`,
`creusot_proof()`) are now **required** — no defaults. The `#[derive(Elicit)]`
macro generates them mechanically by delegating to each field's proof, so
composition is structural: you cannot forget a field's proofs because the macro
handles it.

Runtime validation catches what the type system cannot:

- `validate_proofs_non_empty()` — asserts all three proof methods return
  non-empty `TokenStream`s
- `kani_proof_contains::<Inner>()` — asserts aggregate proofs delegate to
  constituent proofs (catches regressions where a refactor drops a delegation)

### `VerifiedWorkflow` — proof-carrying propositions

The workflow analogue of `ElicitComplete`. Where `ElicitComplete` gates
*types*, `VerifiedWorkflow` gates *propositions* — the zero-cost proof
witnesses that guard state transitions.

```rust
// Define a proposition with auto-generated proofs
#[derive(Prop)]
struct EmailValidated;

// Register it for workflows — compiler rejects if proofs are empty
impl VerifiedWorkflow for EmailValidated {}

// Compose propositions — And<P, Q> is automatically VerifiedWorkflow
type RegistrationApproved = And<EmailValidated, AgeInRange>;
```

The `#[derive(Prop)]` macro generates uniquely-named trivial proof harnesses
for each proposition. For semantic propositions with real axioms (e.g., "sqlx
connection is established"), you implement `Prop` manually with custom proof
bodies.

**The full chain:**

1. `#[derive(Elicit)]` generates types with delegated proofs
2. `impl ElicitComplete` gates those types for MCP tool use
3. `#[derive(Prop)]` generates propositions with proof harnesses
4. `impl VerifiedWorkflow` gates propositions for state machines
5. `Established<P>` tokens carry proof forward at zero cost
6. `And<P, Q>` composes propositions automatically

Every link in this chain is verified by at least one of Kani, Creusot, or
Verus. No unproven code ships.

### `ElicitPromptTree` — compile-time prompt structure

The new `ElicitPromptTree` trait generates a static, complete representation of
the prompt tree for any `#[derive(Elicit)]` type. No runtime reflection — the
structure is known at compile time.

```rust
let tree = MyConfig::prompt_tree();
let prompts = MyConfig::assembled_prompts();
// Every field, every variant — structurally guaranteed complete
```

Four tree variants cover all shapes: `Leaf` (scalar input), `Select` (enum
choice), `Survey` (struct fields), and `Affirm` (boolean). An AccessKit bridge
(`prompt-tree-accesskit` feature) converts prompt trees directly into
accessibility trees.

### Type graph visualization

A structural type graph system (`TypeGraphPlugin`) visualizes the relationships
between `#[derive(Elicit)]` types at runtime. Automatic registration via
`TypeGraphKey` in the derive macro, with Mermaid and GraphViz renderers.
Available as an MCP tool and via the `elicitation graph` CLI subcommand.

### `#[reflect_trait]` — trait factory generation

The `#[reflect_trait]` proc macro bridges non-serializable third-party traits
into the MCP tool ecosystem. Given a trait and its methods, it generates
parameter structs, a vtable with `for_type::<T>()` constructor, an
`AnyToolFactory` for MCP tool discovery, and static registration via
`inventory`. Used to wrap clap's `CommandFactory`, `FromArgMatches`,
`ValueEnum`, and other traits that cannot be derived from JSON schemas alone.

### Typestate ledger — proof-carrying double-entry bookkeeping

A demonstration of the elicitation typestate pattern applied to financial
accounting. Transfers move through three states:

```text
Transfer<Pending> ──validate()──▸ Transfer<Validated> ──commit()──▸ Transfer<Committed>
```

Each transition requires compile-time proof establishment: `AccountsDistinct`,
`AmountPositive`, `BalancedEntries`, `SufficientFunds`. The `ValidTransfer`
proof composes all four. You cannot commit an unvalidated transfer — the type
system forbids it.

### `RenderBackend` — dual-frontend rendering abstraction

A new `RenderBackend` trait in `elicit_ui` abstracts over rendering backends,
allowing the same verified `AccessKit` layout tree to be rendered to egui or
ratatui without duplication. Both `EguiBackend` and `RatatuiBackend` implement
the trait, and the typestate machine (`Layout → VerifiedLayout → RenderedLayout`)
works identically regardless of which backend is selected.

---

## Ecosystem support

Six new shadow crates extend elicitation's reach into the most-used corners
of the Rust ecosystem.

### `elicit_tokio` — async runtime

12 plugins covering the tokio runtime surface: time, sync primitives
(Semaphore, Notify, Barrier), runtime inspection, fs, net (TCP/UDP), process,
task, channels (mpsc/oneshot/broadcast/watch), signals, I/O (copy,
stdin/stdout/stderr), spawn, and runtime::Builder. Unix domain sockets
included on supported platforms. Proposition types (`Established`, `And`)
ensure runtime liveness proofs compose across plugin boundaries.

### `elicit_sqlx` — database operations

Full shadow crate for sqlx with driver-specific plugins (Postgres, MySQL,
SQLite), connection/pool/transaction management, query fragment composition,
and a `ToSqlxArgs` factory for verified parameter binding. Proposition
combinators prove that a connection is live before executing queries — the
typestate ledger (above) is built on top of this.

### `elicit_ui` — frontend-agnostic accessibility verification

A common support crate that sits between AccessKit and any rendering backend.
Three typestate phases enforce WCAG correctness at the type level:

```text
Layout ──verify_a()──▸ VerifiedLayout ──render()──▸ RenderedLayout
```

- **Layout** — raw AccessKit tree, no guarantees
- **VerifiedLayout** — WCAG Level A checks passed (labels, roles, keyboard
  access, overflow, target size)
- **RenderedLayout** — rendered to a backend, statistics available

Five proposition types (`HasLabel`, `ValidRole`, `KeyboardAccessible`,
`NoOverflow`, `MinTargetSize`) compose into the `AccessibleAA` proof witness.
You cannot render without verifying first — the compiler enforces it.

**Terminal-specific constraints** (Phase 10C/10D) extend this to text-mode
UIs. `TerminalNoOverflow`, `MinReadableSize`, and `TerminalAccessible` check
that terminal layouts satisfy character-grid constraints — minimum readable
dimensions and no-overflow at named breakpoints. The `TerminalBreakpointSet`
struct runs the full constraint suite at every registered breakpoint (e.g.,
VT100 80×24, xterm 132×43, HD 220×50) and returns a `BreakpointReport`
collecting pass/fail per breakpoint, making it straightforward to answer "is
my terminal UI accessible at all common terminal sizes?"

**LayoutBuilder** eliminates the manual `NodeId` allocation, `TreeUpdate`
wiring, and parent–child bookkeeping that AccessKit normally requires:

```rust
let layout = LayoutBuilder::new()
    .form()
        .label("Name")
        .text_input("name").placeholder("Jane Doe")
        .checkbox("agree").checked(false)
        .button("Submit").size(100, 44)
    .end()
    .build();

let verified = layout.verify_a(Viewport::new(1920, 1080))?;
let rendered = verified.render(&mut egui_ctx);
```

33 builder methods cover leaf widgets, container types, and property setters.

### `elicit_egui` — 148 MCP tools for egui 0.34

A complete shadow crate giving AI agents programmatic access to the full egui
widget library. Every tool operates in dual mode: JSON output for runtime
inspection and `EmitCode` for compiled binary generation. Includes a
bidirectional AccessKit bridge: `EguiNode → AccessKit` for building verified
accessibility trees from egui layouts, and `AccessKit → EguiNode` for
roundtrip fidelity testing.

| Category         | Tools | Examples                                    |
|------------------|------:|---------------------------------------------|
| Widgets          |    32 | Label, Button, Checkbox, Slider, ColorPicker|
| Containers       |    14 | Window, Panel, ScrollArea, Collapsing       |
| Layout           |    11 | Horizontal, Vertical, Grid, Columns         |
| Styling          |    29 | FontFamily, TextStyle, Colors, Spacing      |
| Response         |    21 | Clicked, Hovered, Dragged, Focus, Rect      |
| Menus & Popups   |    13 | ContextMenu, Popup, Tooltip, Modal          |
| Input            |    14 | KeyPress, Modifiers, Pointer, Clipboard     |
| **Total**        |**148**|                                             |

### `elicit_ratatui` — TUI backend with AccessKit bridge

A new shadow crate for ratatui that mirrors the `elicit_egui` design. The
core abstraction is `TuiNode` — a serializable tree of `WidgetJson` values that
can be constructed programmatically (e.g., by an AI agent), rendered directly
to a ratatui `Frame`, or translated to and from an AccessKit tree.

Key capabilities:

- **15 `WidgetJson` variants** — `Paragraph`, `Block`, `List`, `Table`,
  `Gauge`, `BarChart`, `LineChart`, `ScatterChart`, `Sparkline`, `Tabs`,
  `Scrollbar`, `Calendar`, `Canvas`, `Clear`, `Placeholder`
- **Rich text** — `ParagraphText` accepts either a plain `String` or a full
  `TextJson` tree (lines of `LineJson`, each a list of `SpanJson` with
  per-span `StyleJson`). This enables mixed-style output such as a cyan
  hostname followed by white message text on the same line.
- **Bidirectional AccessKit bridge** — `TuiNode → AccessKit` for WCAG
  verification; `AccessKit → TuiNode` for roundtrip testing
- **`RatatuiBackend`** — implements `RenderBackend`, plugging the ratatui
  renderer into the `elicit_ui` typestate machine
- **`render_node` / `render_widget`** — public functions for downstream
  crates to drive their own `TuiNode` trees without re-implementing the
  layout recursion
- **`TerminalSizeParams`** — queries the live terminal dimensions so
  breakpoint selection is automatic at runtime

### `elicit_accesskit` — accessibility enum types

`Elicitation`, `ElicitSpec`, and `ElicitComplete` impls for all 17 AccessKit
enum types (Role, Action, AriaCurrent, AutoComplete, HasPopup, Invalid, Live,
ListStyle, Orientation, SortDirection, TextAlign, TextDecoration, TextDirection,
VerticalOffset, Toggled, DefaultActionVerb, NameFrom). AI agents can now elicit
accessibility metadata with the same contract guarantees as any other type.

---

## Formal verification

The verification suite now covers three independent backends with tracked,
reproducible results:

| Backend  | Proofs | Status | Notes                                   |
|----------|-------:|--------|-----------------------------------------|
| Kani     |    494 | ✅ All | CBMC model checking, per-function       |
| Creusot  |     31 | ✅ All | Why3/Alt-Ergo SMT, per-module           |
| Verus    |    186 | ✅ All | Z3 SMT, compile-time                    |

Key improvements this cycle:

- **Proof methods now required** — no default empty implementations on
  `Elicitation` or `Prop`; the compiler enforces proof coverage
- **Builder proofs** — Kani, Creusot, and Verus harnesses for LayoutBuilder
  construction, WCAG verification, and renderer statistics
- **egui type proofs** — all 16 Select enum types verified (label roundtrip,
  known label acceptance, label count, unknown rejection)
- **sqlx + tokio proofs** — proposition combinators, fragment contracts,
  runtime type info roundtrips, error kind enumerations
- **ui_types proofs** — 32 Kani harnesses covering CSS unit resolution
  (`Px`, `Em`, `Rem`, `Vw`, `Vh`, `Percent`), `BoundingBox` spatial
  invariants, WCAG contrast ratios, typestate marker zero-sizing, and
  terminal breakpoint profiles
- **Creusot glob elimination** — replaced all `pub use module::*` with ~300
  explicit named exports; fixed cross-feature re-export gates
- **Creusot de-trusting** — systematic campaign moved proofs from `#[trusted]`
  axioms to real SMT-discharged goals (240+ goals proved by Alt-Ergo)
- **Kani heap-free proofs** — replaced 14 builder proofs that used
  Vec/HashMap/String (CBMC timeout) with lightweight structural proofs on
  Copy types
- **Kani proof correctness** — fixed `kani::assume!()` macro invocations
  (Kani only exposes `kani::assume()` as a function); fixed 9 ui_types proofs
  that timed out or failed due to unbounded `f64` symbolic domains and
  incorrect floating-point epsilon comparisons

---

## Breaking changes

None. All new functionality is additive.

---

## Bug fixes

- Feature-gate egui tests for `check-all` compatibility
- Gate `prompt_tree` datetime/serde_json impls behind `not(kani)` to avoid
  CBMC compilation failures
- Fix `EmitCode` scaffolding `toml::from_str` for toml 1.0 compatibility
- Gate `UnixSignalKind` re-export behind `#[cfg(unix)]` in `elicit_tokio`
- Fix `EmitCode` scope for sqlx workflow and `Box<dyn Error>` in emit rewriter
- Fix `is_none_or` usage in type graph renderers
- Resolve `EmitCode` conflicts and dead code under `check-features`
- Fix approximate PI literal in tests (use `std::f64::consts::PI`)
- Fix `kani::assume!()` macro syntax — Kani only exposes `assume()` as a
  function; affected `geo_types`, `palette_types`, and `ui_types` proof modules
- Fix Creusot feature resolution: `elicit_ui` was missing `geo-types` and
  `palette` feature aliases needed when `cargo creusot` propagates feature
  names workspace-wide (features are now forwarded to the canonical `geo` and
  `color` names)
- Fix `WidgetJson::Paragraph` to accept `ParagraphText` (plain `String` or
  rich `TextJson`) rather than a bare `String`; the `terminal_tools` renderer
  now converts the full `TextJson → ratatui::text::Text<'static>` chain
  including per-span styling and alignment
- Expose `render_node` and `render_widget` as `pub` so downstream crates
  can drive their own `TuiNode` trees without re-implementing layout recursion
- Fix `geo_types_test` missing `#![cfg(feature = "geo-types")]` gate causing
  E0432 unresolved import errors in default-feature builds
- Remove redundant `as f32` casts, collapse nested `if-let` chains, and
  replace manual `impl Default` with `#[derive(Default)]` in `elicit_ui`

---

## Documentation

- Comprehensive READMEs for `elicit_egui`, `elicit_tokio`, `elicit_sqlx`,
  `elicit_clap`, `elicit_serde`, `elicit_accesskit`, `elicitation_derive`,
  `elicit_ui`, and `elicit_ratatui`
- `THIRD_PARTY_SUPPORT_GUIDE.md` — complete integration checklist for wrapping
  external crates
- `VISUALIZATION_GUIDE.md` — ties together type graph, prompt tree, and
  AccessKit visualization layers
- `TYPE_GRAPH_GUIDE.md` — user guide for the structural type graph system
- `CREUSOT_GUIDE.md` — guide for the Creusot 0.10.0 proof suite
- `SHADOW_CRATE_MOTIVATION.md` — rationale for the shadow crate pattern
- README rewrite with soup-to-nuts Getting Started example

---

## Statistics

- **248 commits** since v0.9.1
- **6 new crates** (elicit_tokio, elicit_sqlx, elicit_ui, elicit_egui,
  elicit_accesskit, elicit_ratatui)
- **148 egui MCP tools**
- **15 ratatui WidgetJson variants**
- **711 formal verification proofs** (494 Kani + 31 Creusot modules + 186
  Verus)
- **537 markdownlint errors** fixed across 34 files


---

## Architecture

The core story of 0.9.2: proof obligations are now enforced, not optional.

### `ElicitComplete` — the proof completeness gate

`ElicitComplete` is a marker supertrait that aggregates every obligation a type
must satisfy before it can participate in the elicitation ecosystem:

```rust
pub trait ElicitComplete:
    Elicitation          // interactive elicitation + kani/verus/creusot proofs
    + ElicitIntrospect   // structural metadata
    + ElicitSpec         // agent-browsable contract specs
    + Serialize + Deserialize + JsonSchema  // wire format
{ }
```

When you write `impl ElicitComplete for MyType {}`, the compiler accepts it
**only** if every supertrait's obligations are met — including non-empty proof
bodies for all three verification backends. This replaced the previous approach
where proof methods had default empty implementations, meaning types could
silently ship without any formal verification coverage.

The `Elicitation` trait's proof methods (`kani_proof()`, `verus_proof()`,
`creusot_proof()`) are now **required** — no defaults. The `#[derive(Elicit)]`
macro generates them mechanically by delegating to each field's proof, so
composition is structural: you cannot forget a field's proofs because the macro
handles it.

Runtime validation catches what the type system cannot:

- `validate_proofs_non_empty()` — asserts all three proof methods return
  non-empty `TokenStream`s
- `kani_proof_contains::<Inner>()` — asserts aggregate proofs delegate to
  constituent proofs (catches regressions where a refactor drops a delegation)

### `VerifiedWorkflow` — proof-carrying propositions

The workflow analogue of `ElicitComplete`. Where `ElicitComplete` gates
*types*, `VerifiedWorkflow` gates *propositions* — the zero-cost proof
witnesses that guard state transitions.

```rust
// Define a proposition with auto-generated proofs
#[derive(Prop)]
struct EmailValidated;

// Register it for workflows — compiler rejects if proofs are empty
impl VerifiedWorkflow for EmailValidated {}

// Compose propositions — And<P, Q> is automatically VerifiedWorkflow
type RegistrationApproved = And<EmailValidated, AgeInRange>;
```

The `#[derive(Prop)]` macro generates uniquely-named trivial proof harnesses
for each proposition. For semantic propositions with real axioms (e.g., "sqlx
connection is established"), you implement `Prop` manually with custom proof
bodies.

**The full chain:**

1. `#[derive(Elicit)]` generates types with delegated proofs
2. `impl ElicitComplete` gates those types for MCP tool use
3. `#[derive(Prop)]` generates propositions with proof harnesses
4. `impl VerifiedWorkflow` gates propositions for state machines
5. `Established<P>` tokens carry proof forward at zero cost
6. `And<P, Q>` composes propositions automatically

Every link in this chain is verified by at least one of Kani, Creusot, or
Verus. No unproven code ships.

### `ElicitPromptTree` — compile-time prompt structure

The new `ElicitPromptTree` trait generates a static, complete representation of
the prompt tree for any `#[derive(Elicit)]` type. No runtime reflection — the
structure is known at compile time.

```rust
let tree = MyConfig::prompt_tree();
let prompts = MyConfig::assembled_prompts();
// Every field, every variant — structurally guaranteed complete
```

Four tree variants cover all shapes: `Leaf` (scalar input), `Select` (enum
choice), `Survey` (struct fields), and `Affirm` (boolean). An AccessKit bridge
(`prompt-tree-accesskit` feature) converts prompt trees directly into
accessibility trees.

### Type graph visualization

A structural type graph system (`TypeGraphPlugin`) visualizes the relationships
between `#[derive(Elicit)]` types at runtime. Automatic registration via
`TypeGraphKey` in the derive macro, with Mermaid and GraphViz renderers.
Available as an MCP tool and via the `elicitation graph` CLI subcommand.

### `#[reflect_trait]` — trait factory generation

The `#[reflect_trait]` proc macro bridges non-serializable third-party traits
into the MCP tool ecosystem. Given a trait and its methods, it generates
parameter structs, a vtable with `for_type::<T>()` constructor, an
`AnyToolFactory` for MCP tool discovery, and static registration via
`inventory`. Used to wrap clap's `CommandFactory`, `FromArgMatches`,
`ValueEnum`, and other traits that cannot be derived from JSON schemas alone.

### Typestate ledger — proof-carrying double-entry bookkeeping

A demonstration of the elicitation typestate pattern applied to financial
accounting. Transfers move through three states:

```text
Transfer<Pending> ──validate()──▸ Transfer<Validated> ──commit()──▸ Transfer<Committed>
```

Each transition requires compile-time proof establishment: `AccountsDistinct`,
`AmountPositive`, `BalancedEntries`, `SufficientFunds`. The `ValidTransfer`
proof composes all four. You cannot commit an unvalidated transfer — the type
system forbids it.

---

## Ecosystem support

Five new shadow crates extend elicitation's reach into the most-used corners
of the Rust ecosystem.

### `elicit_tokio` — async runtime

12 plugins covering the tokio runtime surface: time, sync primitives
(Semaphore, Notify, Barrier), runtime inspection, fs, net (TCP/UDP), process,
task, channels (mpsc/oneshot/broadcast/watch), signals, I/O (copy,
stdin/stdout/stderr), spawn, and runtime::Builder. Unix domain sockets
included on supported platforms. Proposition types (`Established`, `And`)
ensure runtime liveness proofs compose across plugin boundaries.

### `elicit_sqlx` — database operations

Full shadow crate for sqlx with driver-specific plugins (Postgres, MySQL,
SQLite), connection/pool/transaction management, query fragment composition,
and a `ToSqlxArgs` factory for verified parameter binding. Proposition
combinators prove that a connection is live before executing queries — the
typestate ledger (above) is built on top of this.

### `elicit_ui` — frontend-agnostic accessibility verification

A common support crate that sits between AccessKit and any rendering backend.
Three typestate phases enforce WCAG correctness at the type level:

```text
Layout ──verify_a()──▸ VerifiedLayout ──render()──▸ RenderedLayout
```

- **Layout** — raw AccessKit tree, no guarantees
- **VerifiedLayout** — WCAG Level A checks passed (labels, roles, keyboard
  access, overflow, target size)
- **RenderedLayout** — rendered to a backend, statistics available

Five proposition types (`HasLabel`, `ValidRole`, `KeyboardAccessible`,
`NoOverflow`, `MinTargetSize`) compose into the `AccessibleAA` proof witness.
You cannot render without verifying first — the compiler enforces it.

`elicit_ui` is deliberately backend-agnostic. It depends on AccessKit alone,
not on egui or any other renderer. Today `elicit_egui` is the only backend;
tomorrow a `elicit_iced` or `elicit_dioxus` crate could plug in by
implementing the same `render()` trait — inheriting all the verification
machinery for free.

**LayoutBuilder** eliminates the manual `NodeId` allocation, `TreeUpdate`
wiring, and parent–child bookkeeping that AccessKit normally requires:

```rust
let layout = LayoutBuilder::new()
    .form()
        .label("Name")
        .text_input("name").placeholder("Jane Doe")
        .checkbox("agree").checked(false)
        .button("Submit").size(100, 44)
    .end()
    .build();

let verified = layout.verify_a(Viewport::new(1920, 1080))?;
let rendered = verified.render(&mut egui_ctx);
```

33 builder methods cover leaf widgets, container types, and property setters.

### `elicit_egui` — 148 MCP tools for egui 0.34

A complete shadow crate giving AI agents programmatic access to the full egui
widget library. Every tool operates in dual mode: JSON output for runtime
inspection and `EmitCode` for compiled binary generation.

| Category         | Tools | Examples                                    |
|------------------|------:|---------------------------------------------|
| Widgets          |    32 | Label, Button, Checkbox, Slider, ColorPicker|
| Containers       |    14 | Window, Panel, ScrollArea, Collapsing       |
| Layout           |    11 | Horizontal, Vertical, Grid, Columns         |
| Styling          |    29 | FontFamily, TextStyle, Colors, Spacing      |
| Response         |    21 | Clicked, Hovered, Dragged, Focus, Rect      |
| Menus & Popups   |    13 | ContextMenu, Popup, Tooltip, Modal          |
| Input            |    14 | KeyPress, Modifiers, Pointer, Clipboard     |
| **Total**        |**148**|                                             |

### `elicit_accesskit` — accessibility enum types

`Elicitation`, `ElicitSpec`, and `ElicitComplete` impls for all 17 AccessKit
enum types (Role, Action, AriaCurrent, AutoComplete, HasPopup, Invalid, Live,
ListStyle, Orientation, SortDirection, TextAlign, TextDecoration, TextDirection,
VerticalOffset, Toggled, DefaultActionVerb, NameFrom). AI agents can now elicit
accessibility metadata with the same contract guarantees as any other type.

---

## Formal verification

The verification suite now covers three independent backends with tracked,
reproducible results:

| Backend  | Proofs | Status | Notes                                   |
|----------|-------:|--------|-----------------------------------------|
| Kani     |    462 | ✅ All | CBMC model checking, per-function       |
| Creusot  |     31 | ✅ All | Why3/Alt-Ergo SMT, per-module           |
| Verus    |    186 | ✅ All | Z3 SMT, compile-time                    |

Key improvements this cycle:

- **Proof methods now required** — no default empty implementations on
  `Elicitation` or `Prop`; the compiler enforces proof coverage
- **Builder proofs** — Kani, Creusot, and Verus harnesses for LayoutBuilder
  construction, WCAG verification, and renderer statistics
- **egui type proofs** — all 16 Select enum types verified (label roundtrip,
  known label acceptance, label count, unknown rejection)
- **sqlx + tokio proofs** — proposition combinators, fragment contracts,
  runtime type info roundtrips, error kind enumerations
- **Creusot glob elimination** — replaced all `pub use module::*` with ~300
  explicit named exports; fixed cross-feature re-export gates
- **Creusot de-trusting** — systematic campaign moved proofs from `#[trusted]`
  axioms to real SMT-discharged goals (240+ goals proved by Alt-Ergo)
- **Kani heap-free proofs** — replaced 14 builder proofs that used
  Vec/HashMap/String (CBMC timeout) with lightweight structural proofs on
  Copy types

---

## Breaking changes

None. All new functionality is additive.

---

## Bug fixes

- Feature-gate egui tests for `check-all` compatibility
- Gate `prompt_tree` datetime/serde_json impls behind `not(kani)` to avoid
  CBMC compilation failures
- Fix `EmitCode` scaffolding `toml::from_str` for toml 1.0 compatibility
- Gate `UnixSignalKind` re-export behind `#[cfg(unix)]` in `elicit_tokio`
- Fix `EmitCode` scope for sqlx workflow and `Box<dyn Error>` in emit rewriter
- Fix `is_none_or` usage in type graph renderers
- Resolve `EmitCode` conflicts and dead code under `check-features`
- Fix approximate PI literal in tests (use `std::f64::consts::PI`)

---

## Documentation

- Comprehensive READMEs for `elicit_egui`, `elicit_tokio`, `elicit_sqlx`,
  `elicit_clap`, `elicit_serde`, `elicit_accesskit`, `elicitation_derive`
- `THIRD_PARTY_SUPPORT_GUIDE.md` — complete integration checklist for wrapping
  external crates
- `VISUALIZATION_GUIDE.md` — ties together type graph, prompt tree, and
  AccessKit visualization layers
- `TYPE_GRAPH_GUIDE.md` — user guide for the structural type graph system
- `CREUSOT_GUIDE.md` — guide for the Creusot 0.10.0 proof suite
- `SHADOW_CRATE_MOTIVATION.md` — rationale for the shadow crate pattern
- README rewrite with soup-to-nuts Getting Started example

---

## Statistics

- **199 commits** since v0.9.1
- **5 new crates** (elicit_tokio, elicit_sqlx, elicit_ui, elicit_egui,
  elicit_accesskit)
- **148 egui MCP tools**
- **679 formal verification proofs** (462 Kani + 31 Creusot modules + 186
  Verus)
- **537 markdownlint errors** fixed across 34 files
