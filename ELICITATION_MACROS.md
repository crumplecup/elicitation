# Elicitation Macros

A reference catalog of all `macro_rules!` macros in the elicitation workspace.
Proc-derive macros live in `elicitation_derive`; the macros below are all
declarative (`macro_rules!`) and are exported from their respective crates.

---

## `elicitation` crate

### Newtype macros — `src/newtype_macro.rs`

| Macro | Purpose |
|---|---|
| `elicit_newtype!` | Core newtype generator. Wraps a foreign type with a named struct, derives `ElicitComplete`, `Prompt`, `Elicitation`, `KaniCompose`, `ElicitSpec`, `ElicitPromptTree`, and all formal-verification proof hooks. |
| `elicit_newtype_traits!` | Adds supplemental trait impls to an already-declared newtype (Display, From, Into, etc.). |
| `elicit_newtype_trait_flag!` | Adds a single marker-trait impl to a newtype (used for opt-in trait flags). |
| `elicit_newtypes!` | Batch convenience: calls `elicit_newtype!` repeatedly for a list of types. |

### Newtype method macros — `src/newtype_methods_macro.rs`

| Macro | Purpose |
|---|---|
| `elicit_newtype_methods!` | Generates MCP tool methods on a newtype by reflecting the wrapped type's method signatures, producing `#[elicit_tool]`-compatible async wrappers. |

### Router macros — `src/router_macro.rs`

| Macro | Purpose |
|---|---|
| `elicit_router!` | Generates an `rmcp::Router` impl that dispatches MCP tool calls to the correct plugin method. |
| `elicit_tools!` | Generates the `list_tools()` response body from a plugin's registered tool definitions. |

### Select / trenchcoat macros — `src/select_trenchcoat_macro.rs`

| Macro | Purpose |
|---|---|
| `select_trenchcoat!` | Generates an `ElicitComplete` shadow enum that mirrors a foreign enum, adding `From`/`Into` conversions, elicitation dispatch, and formal-verification hooks. |
| `select_trenchcoat_traits!` | Adds supplemental trait impls to a trenchcoat enum. |

### Emit / plugin registration — `src/emit_code.rs`

| Macro | Purpose |
|---|---|
| `register_emit!` | Registers an emission handler (a plugin's `emit()` impl) into the global plugin registry. Called once per plugin in the plugin's module. |

### Styling — `src/default_style.rs`

| Macro | Purpose |
|---|---|
| `default_style!` | Generates a single-variant unit enum that implements `ElicitationStyle`, used to declare the default UI style for a primitive type. |

### Contracts — `src/contracts.rs`

| Macro | Purpose |
|---|---|
| `proof_credential!` | Declares a named proof-credential type (`struct MyCredential;`) and registers it with the `ProvableFrom` lattice. |
| `kani_label!` | Attaches a human-readable label and metadata to a Kani proof harness for documentation and tooling. |

---

## `elicit_accesskit` crate

### `src/lib.rs`

| Macro | Purpose |
|---|---|
| `accesskit_copy_enum!` | Generates an elicitation-compatible shadow of an AccessKit enum, preserving variant names and adding `From`/`Into`, `Serialize`/`Deserialize`, `JsonSchema`, and `Elicit` impls. |

---

## `elicitation_kani` crate

### `src/harness_helpers.rs`

Helper utilities and the `kani_arbitrary!` macro for writing Kani proof
harnesses for types with `String` or `Vec<T>` fields that Kani cannot derive
`Arbitrary` for natively.

| Item | Kind | Purpose |
|---|---|---|
| `bounded_string::<N>()` | `#[cfg(kani)]` fn | Generate an N-byte arbitrary `String`. |
| `bounded_option_string::<N>()` | `#[cfg(kani)]` fn | Generate `None` or an N-byte arbitrary `String`. |
| `bounded_vec::<T, N>()` | `#[cfg(kani)]` fn | Generate a `Vec<T>` with 0..N arbitrary elements. |
| `kani_arbitrary!` | `macro_rules!` | Emit a `kani::Arbitrary` impl that uses the bounded helpers above. |

---

## Internal (non-exported) macros

Many crates contain `macro_rules!` helpers that are **not** `#[macro_export]`
and are only visible within their defining module.  They are intentionally
omitted from this catalog; consult the source files directly.

Notable examples:
- `elicitation/src/primitives/integers.rs` — integer elicitation scaffolding
- `elicitation/src/primitives/floats.rs` — float elicitation scaffolding
- `elicitation/src/verification/types/*.rs` — per-type proof scaffolding
- `elicit_bevy/src/*.rs` — Bevy shadow-type scaffolding

---

## Proc-derive macros

Declarative macros are in-scope above.  For procedural (derive) macros see
the `elicitation_derive` crate:

| Derive | Purpose |
|---|---|
| `#[derive(Elicit)]` | Full `ElicitComplete` impl via proc-macro. |
| `#[derive(ElicitPlugin)]` | MCP plugin scaffolding. |
| `#[derive(KaniCompose)]` | Kani composition proof scaffolding. |
| `#[derive(Prop)]` | Proposition type for verified workflow contracts. |
| `#[derive(ToCodeLiteral)]` | `ToCodeLiteral` impl for code generation. |
| `#[derive(DeriveVsm)]` | Verified state machine scaffolding. |
| `#[elicit_tool]` | Attribute macro: registers an async fn as an MCP tool. |
| `#[formal_method]` | Attribute macro: gates `#[instrument]` under `#[cfg(not(kani))]`. |
