# Third-Party Crate Support Guide

This document is the authoritative checklist for adding elicitation support for a
third-party crate. Follow every section in order. The `clap` integration is the
canonical reference implementation for all patterns described here.

---

## Core Verification Principle

> **Trust the source. Verify the wrapper.**

This principle applies uniformly across all three verifiers (Kani, Creusot, Verus)
and to all type origins (std lib, third-party crates, our own contract types):

- **Trust the source** — assume the stdlib and third-party crates uphold their own
  invariants. We do not re-verify that `clap::ColorChoice` has exactly three variants,
  or that `std::collections::HashMap` correctly stores keys. That is the responsibility
  of the upstream library and its own test suite.

- **Verify the wrapper** — prove that *our* business logic is correct: that every label
  produced by `labels()` is accepted by `from_label()`, that the roundtrip is complete,
  that unknown inputs are rejected, that our `Elicitation` impl delegates correctly.

This keeps proofs focused, tractable, and meaningful. A `kani::assume(true)` for a
third-party builder type is not a cop-out — it is an explicit, documented architectural
decision that we have placed that type in the "trusted" category and are not claiming
to verify its internals.

---

## Overview

Adding full support for a third-party crate `foo` involves work across **six locations**:

| Location | What you add |
|---|---|
| `Cargo.toml` (workspace root) | Optional dep + `elicit_foo` workspace member |
| `crates/elicitation/` | `Elicitation` trait impls (feature-gated `foo-types`) |
| `crates/elicit_foo/` | Newtypes + `reflect_methods` + (optionally) trait factories |
| `crates/elicitation_kani/` | Kani proof harnesses |
| `crates/elicitation_creusot/` | Creusot `#[requires]`/`#[ensures]`/`#[trusted]` proofs |
| `crates/elicitation_verus/` | Verus `ensures`/`requires` proofs |

There are **three distinct mechanisms** for exposing types as MCP tools:

| Mechanism | Use when | Location |
|---|---|---|
| `#[reflect_methods]` | Your newtype has methods you want to expose | `elicitation_derive` |
| `#[reflect_trait]` | A *third-party trait* has methods worth calling on any `T: FooTrait` | `elicitation_macros` |
| Fragment tool + `EmitCode` | A *Rust macro* that runs at compile time, not at runtime | `elicitation::emit_code` |

The fragment tool mechanism bypasses Phases 2, 4, 5, and 6 — no `Elicitation`
trait impls, no Kani/Creusot/Verus verification.  See the [Fragment Tools](#fragment-tools-macros) section.

Never skip a section. Never add an `#[allow]` attribute. Fix root causes.

---

## Phase 1 — Workspace Root (`Cargo.toml`)

```toml
# Add to [workspace.dependencies]
foo = { version = "X.Y", features = ["..."] }
elicit_foo = { path = "crates/elicit_foo", version = "0.9" }

# Add to [workspace.members]
"crates/elicit_foo",
```

**Notes:**
- Match the version constraint convention: major-only for `>=1.0`, major.minor for `>=0.1.0`
- Add all features the `Elicitation` impls will need (e.g. `"string"` for `clap::Id`)
- Do not add `elicit_foo` to `default-members` unless it has its own binary

---

## Phase 2 — Core Trait Impls in `crates/elicitation/`

### 2.1 Feature flag

In `crates/elicitation/Cargo.toml`:

```toml
[dependencies]
foo = { workspace = true, optional = true }

[features]
foo-types = ["dep:foo"]
```

### 2.2 Type files

Create `crates/elicitation/src/primitives/foo_types/` with one file per type.

**For `Select` enum types** (user picks from a fixed list):

```rust
// src/primitives/foo_types/my_enum.rs
use crate::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitIntrospect, ElicitResult,
    Elicitation, ElicitationPattern, PatternDetails, Prompt, Select, TypeMetadata,
    VariantMetadata, mcp,
};
use foo::MyEnum;

impl Prompt for MyEnum {
    fn prompt() -> Option<&'static str> {
        Some("Choose a MyEnum value:")
    }
}

impl Select for MyEnum {
    fn options() -> Vec<Self> {
        vec![MyEnum::VariantA, MyEnum::VariantB]
    }

    fn labels() -> Vec<String> {
        vec!["Variant A description".to_string(), "Variant B description".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Variant A description" => Some(MyEnum::VariantA),
            "Variant B description" => Some(MyEnum::VariantB),
            _ => None,
        }
    }
}

crate::default_style!(MyEnum => MyEnumStyle);

impl Elicitation for MyEnum {
    type Style = MyEnumStyle;

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::select_params(
            Self::prompt().unwrap_or("Choose value:"),
            &Self::labels(),
        );
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let label = mcp::parse_string(value)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid MyEnum: {}", label
            )))
        })
    }
}

impl ElicitIntrospect for MyEnum {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }

    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "foo::MyEnum",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels()
                    .into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}
```

**For text/primitive types** (user types a value):

```rust
impl Elicitation for MyType {
    type Style = MyTypeStyle;

    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        let params = mcp::text_params(Self::prompt().unwrap_or("Enter value:"));
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_text())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let s = mcp::parse_string(value)?.trim().to_string();
        MyType::new(s).map_err(|e| ElicitError::new(ElicitErrorKind::ParseError(e.to_string())))
    }
}
```

**Key rules for both patterns:**
- `.trim().to_string()` before passing to constructors — `.trim()` returns `&str`,
  but constructors need `String` (no `From<&str>` without `'static`)
- Use `mcp::select_params` + `elicit_select` for enums
- Use `mcp::text_params` + `elicit_text` for primitives/survey types
- `#[tracing::instrument(skip(communicator))]` on all `async fn elicit`
- Never use `#[allow]` — gate feature-dependent code with `#[cfg(feature = "foo-types")]`

### 2.3 Module wiring

In `src/primitives/mod.rs`:
```rust
#[cfg(feature = "foo-types")]
pub mod foo_types;
```

In `src/lib.rs`:
```rust
#[cfg(feature = "foo-types")]
pub use primitives::foo_types::{MyEnum, MyType, /* ... */};
```

### 2.4 TypeSpec (`ElicitSpec`) in `src/type_spec/`

Create `src/type_spec/foo_specs.rs`. This adds agent-browsable contract
descriptions to the global inventory, complementing the structural metadata
from `ElicitIntrospect`. Agents use this via the `describe_type` / `explore_type`
MCP tools.

Two macros handle the boilerplate:

```rust
//! ElicitSpec impls for foo types. Available with the `foo-types` feature.

#[cfg(feature = "foo-types")]
mod foo_impls {
    use crate::{
        ElicitSpec, SpecCategoryBuilder, SpecEntryBuilder, TypeSpec, TypeSpecBuilder,
        TypeSpecInventoryKey,
    };

    // For Select enums — list each variant with a description
    macro_rules! impl_select_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            variants = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let variants = SpecCategoryBuilder::default()
                        .name("variants".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("foo — third-party crate".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Select — choose one variant from the list".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![variants, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    // For builder/struct types — list key fields
    macro_rules! impl_builder_spec {
        (
            type    = $ty:ty,
            name    = $name:literal,
            summary = $summary:literal,
            fields  = [$(($label:literal, $desc:literal)),+ $(,)?]
        ) => {
            impl ElicitSpec for $ty {
                fn type_spec() -> TypeSpec {
                    let fields = SpecCategoryBuilder::default()
                        .name("fields".to_string())
                        .entries(vec![
                            $(
                                SpecEntryBuilder::default()
                                    .label($label.to_string())
                                    .description($desc.to_string())
                                    .build()
                                    .expect("valid SpecEntry"),
                            )+
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    let source = SpecCategoryBuilder::default()
                        .name("source".to_string())
                        .entries(vec![
                            SpecEntryBuilder::default()
                                .label("crate".to_string())
                                .description("foo — third-party crate".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                            SpecEntryBuilder::default()
                                .label("pattern".to_string())
                                .description("Survey — builder type elicited field by field".to_string())
                                .build()
                                .expect("valid SpecEntry"),
                        ])
                        .build()
                        .expect("valid SpecCategory");
                    TypeSpecBuilder::default()
                        .type_name($name.to_string())
                        .summary($summary.to_string())
                        .categories(vec![fields, source])
                        .build()
                        .expect("valid TypeSpec")
                }
            }
            inventory::submit!(TypeSpecInventoryKey::new(
                $name,
                <$ty as ElicitSpec>::type_spec,
                std::any::TypeId::of::<$ty>
            ));
        };
    }

    impl_select_spec!(
        type    = foo::MyEnum,
        name    = "foo::MyEnum",
        summary = "One-line description of what MyEnum controls.",
        variants = [
            ("VariantA", "Description of what VariantA does"),
            ("VariantB", "Description of what VariantB does"),
        ]
    );

    impl_builder_spec!(
        type    = foo::MyBuilder,
        name    = "foo::MyBuilder",
        summary = "One-line description of the builder type.",
        fields = [
            ("field_one", "What field_one controls"),
            ("field_two", "What field_two controls"),
        ]
    );
}
```

Then wire the new file in `src/type_spec/mod.rs`:

```rust
mod foo_specs;
```

The module declaration is unconditional — the `#[cfg(feature = "foo-types")]`
lives inside the file. This matches the pattern of all other `*_specs.rs` files.

---

## Phase 3 — Wrapper Crate `crates/elicit_foo/`

### 3.1 `Cargo.toml`

```toml
[package]
name = "elicit_foo"
version.workspace = true
edition.workspace = true
# ... other workspace fields

[dependencies]
elicitation = { workspace = true, features = ["foo-types"] }
elicitation_derive = { workspace = true }
foo = { workspace = true }
schemars = { workspace = true }
tracing = { workspace = true }
rmcp = { workspace = true }
```

### 3.2 `src/lib.rs`

```rust
//! Elicitation-enabled newtype wrappers for the `foo` crate.
//!
//! Each wrapper provides:
//! - Implements [`schemars::JsonSchema`] so the type can appear in MCP tool schemas
//! - Transparent [`Deref`]/[`DerefMut`] access to the inner type
//! - [`reflect_methods`] impl exposing public API as MCP tools

mod my_enum;
mod my_type;

pub use my_enum::MyEnum;
pub use my_type::MyType;
```

### 3.3 Type files

**Template for all types:**

```rust
//! [`foo::MyType`] newtype wrapper.

use elicitation::{elicit_newtype, elicit_newtype_traits};
use elicitation_derive::reflect_methods;

elicit_newtype!(foo::MyType, as MyType);
// Add trait flags as applicable. Common flags:
// [eq]       → PartialEq + Eq
// [eq_hash]  → PartialEq + Eq + Hash
// [ord]      → PartialEq + Eq + PartialOrd + Ord
// [cmp]      → PartialEq + Eq + Hash + PartialOrd + Ord
// [display]  → Display
// [from_str] → FromStr
elicit_newtype_traits!(MyType, foo::MyType, [eq, display]);

#[reflect_methods]
impl MyType {
    /// Short description.
    #[tracing::instrument(skip(self))]
    pub fn some_method(&self) -> ReturnType {
        self.0.some_method()
    }
}
```

**Key rules:**
- `elicit_newtype!` generates: `Arc<T>` wrapper struct + generic object `JsonSchema` + `Deref`/`DerefMut`/`AsRef`/`From` impls — do **not** add a custom `JsonSchema` impl; the macro's object schema is correct for MCP
- `elicit_newtype_traits!` only lists traits the inner type actually implements — check the crate docs
- All methods in `#[reflect_methods]` must be `pub`, `#[tracing::instrument(skip(self))]`, return owned types (not references into inner type — return `String` not `&str`, `Option<String>` not `Option<&str>`)
- Do not wrap methods that return crate-internal types with no `JsonSchema` — return `String` equivalents instead
- For `#[non_exhaustive]` enums, add a `_ => "Unknown"` wildcard arm in match expressions

**`elicit_newtype!` variants:**

| Syntax | JsonSchema | Serialize/Deserialize |
|---|---|---|
| `elicit_newtype!(T, as Name)` | Generic object schema (use this) | No |
| `elicit_newtype!(T, as Name, serde)` | Delegated to `T` | Yes (`T: Serialize`) |

Never add `no_schema` — that variant was removed because the object schema is what MCP needs.

---

## Phase 3B — Trait Factories (`#[reflect_trait]`)

Use this phase when the third-party crate exposes *derive traits* (like `clap::ValueEnum`,
`clap::CommandFactory`) whose methods are worth calling as standalone MCP tools — independently
of any particular instance of the newtype wrapper.

Skip this phase if no derive traits are worth exposing.

### 3B.1 When to use `#[reflect_trait]` vs `#[reflect_methods]`

| Situation | Use |
|---|---|
| You own a newtype and want to expose its methods | `#[reflect_methods]` on `impl MyType` |
| A third-party trait has static or instance methods you want callable for *any* `T: FooTrait` | `#[reflect_trait(foo::FooTrait)]` |
| Both | Both — they compose |

`#[reflect_trait]` generates a **factory**: a struct that implements `AnyToolFactory` and is
submitted to `inventory` at link time. At runtime, the `DynamicToolRegistry` discovers all
factories, and the agent can instantiate tools for any registered concrete type.

### 3B.2 The orphan rule and `type_map`

The orphan rule prevents writing `impl ElicitProxy for foo::ForeignType` in `elicit_foo`
(neither trait nor type is local). The `type_map` attribute solves this by substituting
foreign types with your `elicit_foo` newtype wrappers that already have the right `From` impls:

```rust
// WRONG — orphan rule violation, won't compile:
impl ElicitProxy for foo::Command { ... }

// CORRECT — use type_map to declare the substitution:
#[reflect_trait(foo::CommandFactory,
    type_map(foo::Command => crate::Command))]
pub trait CommandFactoryTools {
    fn command() -> foo::Command;
}
// The macro uses crate::Command in the param struct and generates
// foo::Command::from(crate_command) / crate::Command::from(foo_command) conversions.
```

**Requirements for a mapped type:**
- The proxy type must implement `Serialize + Deserialize + JsonSchema`
- `From<ProxyType> for OriginalType` must exist (for params going *in*)
- `From<OriginalType> for ProxyType` must exist (for return values coming *out*)

`elicit_newtype!` already provides `From<Original> for Wrapper` (wraps in Arc) and
`From<Wrapper> for Arc<Original>`. You must manually add `From<Wrapper> for Original`
for the param direction — see the `command.rs`, `id.rs`, `possible_value.rs` files in
`elicit_clap` for the pattern (use `Arc::try_unwrap` with `clone` fallback).

### 3B.3 `ElicitProxy` for stdlib and user-owned types

For types that don't need `type_map` — stdlib types and types you own — implement
`ElicitProxy` instead:

```rust
// For types you own — use the derive macro:
use elicitation_derive::ElicitProxy;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit, ElicitProxy)]
pub struct MyConfig { pub name: String }
```

`ElicitProxy` is already implemented for all stdlib primitives, `String`, `Vec<T>`,
`Option<T>`, `Result<T, E>`, and common types. You only need to implement it for your
own domain types.

**Do not** implement `ElicitProxy` for foreign types — use `type_map` instead.

### 3B.4 Macro syntax

```rust
use elicitation_macros::reflect_trait;

// Simplest form — no type substitution needed:
#[reflect_trait(foo::MyTrait)]
pub trait MyTraitTools {
    fn static_method(arg: String) -> bool;
    fn instance_method(&self, arg: i32) -> String;
}

// With type substitution (orphan-rule workaround):
#[reflect_trait(foo::MyTrait,
    type_map(foo::TypeA => crate::TypeA, foo::TypeB => crate::TypeB))]
pub trait MyTraitTools {
    fn method_a(input: foo::TypeA) -> foo::TypeB;
}
```

**Syntax notes:**
- The marker trait block is *consumed* by the macro — it is not emitted as a real trait
- Method signatures must match the real trait exactly (same names, same types)
- `&self` receivers are supported — the agent passes `{"target": <serialized T>}` in the params
- `&str` params are automatically handled: the param struct stores `String`, the vtable calls `.as_str()`
- `&[T]` return types are automatically handled: `.to_vec()` is called to make them owned
- Multiple `type_map` entries are comma-separated

**What the macro generates:**
- One param struct per method (implements `Deserialize + JsonSchema`)
- A vtable struct with one `Arc<dyn Fn(Value) -> BoxFuture<...>>` per method
- A factory struct implementing `AnyToolFactory`
- An `inventory::submit!` call registering the factory at link time
- A free `pub fn prime_foo__mytrait::<T>()` function for startup registration

### 3B.5 Naming conventions

The generated names are derived from the fully-qualified trait path:

| Input | Generated name |
|---|---|
| `foo::MyTrait` | Factory: `MyTraitFactory`, prime fn: `prime_foo__my_trait::<T>()` |
| `clap::ValueEnum` | Factory: `ValueEnumFactory`, prime fn: `prime_clap__value_enum::<T>()` |
| `clap::CommandFactory` | Factory: `CommandFactoryFactory`, prime fn: `prime_clap__command_factory::<T>()` |

Tool names at runtime: `{prefix}__{method_name}` — e.g. `myapp__value_variants`.

### 3B.6 Startup registration (prime + register_type)

At server startup, for each concrete type `T` implementing the trait:

```rust
use elicit_foo::trait_factories::prime_foo__my_trait;
use elicitation::DynamicToolRegistry;

// Prime each factory for each type (monomorphizes the vtable closures)
prime_foo__my_trait::<MyConcreteType>();

// Register each type under a prefix
let registry = DynamicToolRegistry::new()
    .register_type::<MyConcreteType>("mytype");

// The agent calls the factory meta-tool to instantiate tools at runtime:
// registry.instantiate("foo::MyTrait", "mytype").await?;
// → creates "mytype__method_one", "mytype__method_two", etc.
```

The agent sees factory meta-tools in `list_tools` immediately. After calling a meta-tool
(or `registry.instantiate(...)` programmatically), the method tools appear in `list_tools`.

### 3B.7 `Cargo.toml` additions for trait factories

```toml
[dependencies]
elicitation_macros = { workspace = true }
inventory = { workspace = true }
serde_json = { workspace = true }

[dev-dependencies]
tokio = { workspace = true }
serde_json = { workspace = true }
```

### 3B.8 Testing trait factories

Each trait factory must be tested at three levels (see `crates/elicit_clap/tests/trait_factories_test.rs`):

```rust
use elicit_foo::trait_factories::prime_foo__my_trait;
use elicitation::{DynamicToolRegistry, Elicit, ElicitPlugin, ToolFactoryRegistration};

// 1. Inventory registration
#[test]
fn my_trait_factory_in_inventory() {
    let found = inventory::iter::<ToolFactoryRegistration>
        .into_iter()
        .any(|r| r.trait_name == "foo::MyTrait");
    assert!(found);
}

// 2. Prime + instantiate lifecycle
#[tokio::test]
async fn my_trait_instantiate_creates_tools() {
    prime_foo__my_trait::<MyConcreteType>();
    let registry = DynamicToolRegistry::new()
        .register_type::<MyConcreteType>("t");
    registry.instantiate("foo::MyTrait", "t").await.unwrap();
    let names: Vec<_> = registry.list_tools().iter()
        .map(|t| t.name.to_string()).collect();
    assert!(names.contains(&"t__method_name".to_string()));
}

// 3. Handler invocation
#[tokio::test]
async fn my_trait_method_returns_expected_value() {
    prime_foo__my_trait::<MyConcreteType>();
    let registry = DynamicToolRegistry::new()
        .register_type::<MyConcreteType>("t");
    registry.instantiate("foo::MyTrait", "t").await.unwrap();
    let result = registry
        .invoke_dynamic("t__method_name", serde_json::json!({"arg": "value"}))
        .await.expect("tool exists").expect("tool succeeds");
    // check result content...
}
```

Use `DynamicToolRegistry::invoke_dynamic(name, args)` to call tools directly without
an MCP connection.

### 3B.9 Deferred traits

Some traits cannot be exposed as MCP tools because their method signatures are
fundamentally incompatible:

| Trait | Blocker |
|---|---|
| `clap::FromArgMatches` | Takes `&ArgMatches` — not `Serialize`, not `Clone` |
| `clap::Parser` | Extends `FromArgMatches` — same blocker |
| Any trait with `&mut self` methods that borrow internal state | Mutable borrows conflict with `Arc<T>` wrapper |

For these, document the deferral in a comment at the top of `trait_factories.rs`.

### 3B.10 clap reference implementation

```rust
// crates/elicit_clap/src/trait_factories.rs

use elicitation_macros::reflect_trait;

#[reflect_trait(clap::CommandFactory,
    type_map(clap::Command => crate::Command))]
pub trait CommandFactoryTools {
    fn command() -> clap::Command;
    fn command_for_update() -> clap::Command;
}

#[reflect_trait(clap::Subcommand,
    type_map(clap::Command => crate::Command))]
pub trait SubcommandTools {
    fn augment_subcommands(cmd: clap::Command) -> clap::Command;
    fn augment_subcommands_for_update(cmd: clap::Command) -> clap::Command;
    fn has_subcommand(name: &str) -> bool;
}

#[reflect_trait(clap::ValueEnum,
    type_map(clap::builder::PossibleValue => crate::PossibleValue))]
pub trait ValueEnumTools {
    fn value_variants() -> &'static [Self];
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue>;
    fn from_str(input: &str, ignore_case: bool) -> Result<Self, String>;
}

#[reflect_trait(clap::Args,
    type_map(clap::Command => crate::Command, clap::Id => crate::Id))]
pub trait ArgsTools {
    fn augment_args(cmd: clap::Command) -> clap::Command;
    fn augment_args_for_update(cmd: clap::Command) -> clap::Command;
    fn group_id() -> Option<clap::Id>;
}
```

User-side startup:

```rust
use elicit_clap::trait_factories::{
    prime_clap__command_factory,
    prime_clap__value_enum,
    prime_clap__args,
    prime_clap__subcommand,
};

// Call once per type at startup:
prime_clap__command_factory::<MyCli>();
prime_clap__value_enum::<MyOutputFormat>();
prime_clap__args::<MyArgs>();
prime_clap__subcommand::<MySubcmd>();

let registry = DynamicToolRegistry::new()
    .register_type::<MyCli>("cli")
    .register_type::<MyOutputFormat>("fmt")
    .register_type::<MyArgs>("args")
    .register_type::<MySubcmd>("cmd");
```

---

## Phase 4 — Kani Verification (`crates/elicitation_kani/`)

### 4.1 `Cargo.toml`

```toml
[dependencies]
foo = { workspace = true, optional = true }

[features]
foo-types = ["dep:foo", "elicitation/foo-types"]
```

### 4.2 Proof file `src/foo_types.rs`

**For `Select` enum types** — verify the three invariants the MCP machinery depends on:

```rust
//! Kani proofs for foo type elicitation.

#[cfg(feature = "foo-types")]
use elicitation::Select;

// Label count: labels().len() == options().len()
#[cfg(feature = "foo-types")]
#[kani::proof]
fn verify_my_enum_label_count() {
    let labels = foo::MyEnum::labels();
    let options = foo::MyEnum::options();
    assert!(labels.len() == options.len(), "labels and options have equal length");
    assert!(labels.len() == N, "MyEnum has N variants");
}

// Full roundtrip: every label is accepted by from_label()
#[cfg(feature = "foo-types")]
#[kani::proof]
fn verify_my_enum_all_labels_roundtrip() {
    let labels = foo::MyEnum::labels();
    for label in &labels {
        let result = foo::MyEnum::from_label(label);
        assert!(result.is_some(), "MyEnum label roundtrips");
    }
}

// Unknown rejection
#[cfg(feature = "foo-types")]
#[kani::proof]
fn verify_my_enum_unknown_rejected() {
    let result = foo::MyEnum::from_label("__unknown__");
    assert!(result.is_none(), "MyEnum rejects unknown labels");
}
```

**For third-party struct/builder types** — explicit trusted assumption:

```rust
#[cfg(feature = "foo-types")]
#[kani::proof]
fn verify_foo_my_builder_trusted_third_party() {
    // foo::MyBuilder is a third-party type from the foo crate.
    // We trust its construction invariants are upheld by the foo library.
    // Our Elicitation impl only adds interactive prompting behavior.
    kani::assume(true);
}
```

### 4.3 Wire in `src/lib.rs`

```rust
#[cfg(all(kani, feature = "foo-types"))]
mod foo_types;
```

### 4.4 Register in `ProofHarness::all()` (`src/verification/runner.rs` in `elicitation`)

```rust
// foo_types
Self::new("foo_types", "verify_my_enum_label_count"),
Self::new("foo_types", "verify_my_enum_all_labels_roundtrip"),
Self::new("foo_types", "verify_my_enum_unknown_rejected"),
Self::new("foo_types", "verify_foo_my_builder_trusted_third_party"),
```

### 4.5 Run the proofs — verify they pass

**Do not skip this step.** `cargo check` only confirms compilation; it does not run
the model checker. Unrun proofs are not verified proofs.

```bash
# Smoke-test one harness first to confirm it doesn't hang or fail
cargo kani --harness verify_my_enum_label_count \
    -p elicitation_kani --all-features --default-unwind 20

# Then run all foo_types harnesses via the tracker (writes kani_verification_results.csv)
just verify-kani-tracked
```

Expected output for each harness: `VERIFICATION:- SUCCESSFUL`.
Any `FAILED` or timeout means the proof code needs fixing before committing.

The CSV (`kani_verification_results.csv`) is a local artifact — it is gitignored and
regenerated by `just verify-kani-tracked`. Do not commit it.

---

## Phase 5 — Creusot Verification (`crates/elicitation_creusot/`)

### 5.1 `Cargo.toml`

```toml
[dependencies]
foo = { workspace = true, optional = true }

[features]
foo-types = ["dep:foo", "elicitation/foo-types"]
```

### 5.2 Proof file `src/foo_types.rs`

Creusot uses `#[requires]`/`#[ensures]`/`#[trusted]` from `creusot-std`. For types we
cannot symbolically execute (all third-party types), use `#[trusted]`. The label count
proof is the **one exception** — it can be de-trusted after adding extern_specs (see §5.4).

```rust
//! Creusot proofs for foo type elicitation.

#![cfg(feature = "foo-types")]

use creusot_std::prelude::*;
use elicitation::Select;

/// Verify MyEnum label count equals option count.
///
/// De-trusted: Alt-Ergo discharges this after extern_spec axioms supply the
/// concrete lengths. See extern_specs.rs §5.4.
#[requires(true)]
#[ensures(result == true)]
pub fn verify_my_enum_label_count() -> bool {
    foo::MyEnum::labels().len() == foo::MyEnum::options().len()
}

/// Verify MyEnum from_label returns Some for a known valid label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_my_enum_known_label_accepted() -> bool {
    let labels = foo::MyEnum::labels();
    !labels.is_empty() && foo::MyEnum::from_label(&labels[0]).is_some()
}

/// Verify MyEnum from_label returns None for an unknown label.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_my_enum_unknown_label_rejected() -> bool {
    foo::MyEnum::from_label("__unknown__").is_none()
}

/// Trust axiom: foo::MyBuilder invariants are maintained by the foo crate.
#[requires(true)]
#[ensures(result == true)]
#[trusted]
pub fn verify_foo_builder_trusted() -> bool {
    true
}
```

### 5.3 Wire in `src/lib.rs`

Both `mod` and `pub use` are required — all other feature-gated modules export their
proof functions at the crate root for discoverability.

```rust
#[cfg(feature = "foo-types")]
mod foo_types;

#[cfg(feature = "foo-types")]
pub use foo_types::*;
```

### 5.4 Add extern_spec axioms for label count de-trusting

The `verify_my_enum_label_count()` proof above has no `#[trusted]`, so Alt-Ergo must
discharge the `len() == len()` goal. Without axioms, `labels()` and `options()` are
opaque program functions and the goal is unprovable.

Add `extern_spec!` blocks in `extern_specs.rs` stating the concrete variant count:

```rust
// extern_specs.rs — bottom of file, gated on the same feature

#[cfg(feature = "foo-types")]
extern_spec! {
    impl elicitation::Select for foo::MyEnum {
        #[ensures(result@.len() == N)]   // N = number of variants
        fn labels() -> Vec<String>;

        #[ensures(result@.len() == N)]
        fn options() -> Vec<foo::MyEnum>;
    }
}
```

Repeat for each `Select` enum in the crate. Count variants by inspecting the
`Select` impl's `from_label` match arms in `src/primitives/foo_types/`.

### 5.5 Register in `CreusotModule::all()` (`src/verification/creusot_runner.rs` in `elicitation`)

```rust
Self::with_feature("foo_types", "foo-types"),
```

### 5.6 Run the proofs — verify they pass

**Do not skip this step.** `cargo check` only confirms compilation; it does not
discharge any proof obligations. Unproved goals are not verified proofs.

```bash
# Step 1: regenerate .coma WhyML files
cargo creusot -- -p elicitation_creusot --features foo-types

# Step 2: confirm .coma files were generated for de-trusted proofs
ls verif/elicitation_creusot_rlib/foo_types/

# Step 3: prove each de-trusted goal
cargo creusot prove \
    "verif/elicitation_creusot_rlib/foo_types/verify_my_enum_label_count.coma" \
    -- -p elicitation_creusot --features foo-types
```

Expected output: `Library verif.elicitation_creusot_rlib.foo_types.verify_my_enum_label_count.Coma: ✔`

Any `✘` means either the extern_spec variant count is wrong or the proof body is
incorrect. `#[trusted]` proofs generate no `.coma` files — only de-trusted proofs do.

---

## Phase 6 — Verus Verification (`crates/elicitation_verus/`)

### 6.1 `Cargo.toml`

Verus does not use workspace deps the same way — check existing patterns in the crate.

### 6.2 Proof file `src/foo_types.rs` (added to `external_types.rs` or new file)

Verus proofs are written inside `verus! { ... }` blocks and use abstract boolean
parameters to represent third-party state that cannot be symbolically computed:

```rust
use verus_builtin_macros::verus;

verus! {

// ============================================================================
// foo crate types
// ============================================================================

/// Proof that MyEnum from_label succeeds for a known valid label.
pub fn verify_my_enum_label_roundtrip(label_is_known: bool) -> (result: bool)
    ensures result == label_is_known,
{
    label_is_known
}

/// Proof that MyEnum rejects unknown labels.
pub fn verify_my_enum_unknown_rejected(label_is_unknown: bool) -> (result: bool)
    ensures result == label_is_unknown,
{
    label_is_unknown
}

/// Trust axiom: foo::MyBuilder is a trusted third-party type.
pub fn verify_foo_builder_trusted() -> (result: bool)
    ensures result == true,
{
    true
}

} // verus!
```

### 6.3 Wire in `src/lib.rs`

```rust
pub mod foo_types;
```

### 6.4 Register in `VerusProof::all()` (`src/verification/verus_runner.rs` in `elicitation`)

Add one entry per proof function:

```rust
// foo_types (N proofs)
Self::new("foo_types", "verify_my_enum_label_roundtrip"),
Self::new("foo_types", "verify_my_enum_unknown_rejected"),
Self::new("foo_types", "verify_foo_builder_trusted"),
```

### 6.5 Run the proofs — verify they pass

**Do not skip this step.** `cargo check` does not run Verus. The verifier must be
invoked explicitly; an unrun proof is not a verified proof.

```bash
# Run Verus directly on the new module (fast, isolated)
verus --crate-type=lib crates/elicitation_verus/src/foo_types.rs
```

Expected: `verification results:: N verified, 0 errors`

Any errors mean the `ensures`/`requires` contracts or the proof body need fixing.
Once all proofs pass, the tracker (`just verify-verus-tracked`) picks them up via
`VerusProof::all()`.

---

## Checklist Summary

Use this to track progress when adding a new crate:

```
Crate: foo
Feature flag: foo-types

[ ] Workspace Cargo.toml: dep + member added
[ ] elicitation/Cargo.toml: optional dep + feature flag
[ ] elicitation/src/primitives/foo_types/ directory created
[ ] Each type file: Prompt + Select (or text) + Elicitation + ElicitIntrospect
[ ] primitives/mod.rs: mod foo_types gated
[ ] lib.rs: pub use exports gated
[ ] type_spec/foo_specs.rs: ElicitSpec impls (impl_select_spec! / impl_builder_spec!)
[ ] type_spec/mod.rs: mod foo_specs added
[ ] just check-all elicitation passes clean

[ ] elicit_foo/Cargo.toml created
[ ] elicit_foo/src/lib.rs created (mod + pub use only)
[ ] Each type file: elicit_newtype! + elicit_newtype_traits! + #[reflect_methods]
[ ] Workspace wired (members + workspace.dependencies)
[ ] just check-all elicit_foo passes clean

(Optional — if the crate exposes derive traits worth wrapping as MCP tools:)
[ ] elicit_foo/Cargo.toml: add elicitation_macros + inventory + serde_json deps
[ ] For each foreign type used as a param/return: add From<Wrapper> for Original impl
[ ] elicit_foo/src/trait_factories.rs: #[reflect_trait] for each trait
[ ] Startup docs updated with prime_foo__trait_name::<T>() + register_type::<T>() pattern
[ ] elicit_foo/tests/trait_factories_test.rs: inventory + lifecycle + invocation tests

[ ] elicitation_kani/Cargo.toml: dep + feature
[ ] elicitation_kani/src/foo_types.rs: proof harnesses
[ ] elicitation_kani/src/lib.rs: mod foo_types gated
[ ] verification/runner.rs: ProofHarness::all() entries added
[ ] Select enums: label_count + all_labels_roundtrip + unknown_rejected proofs
[ ] Struct/builder types: trusted_third_party kani::assume(true) proofs
[ ] cargo kani --harness verify_... -p elicitation_kani --all-features --default-unwind 20 passes (smoke test)
[ ] just verify-kani-tracked — all new harnesses show SUCCESSFUL in CSV

[ ] elicitation_creusot/Cargo.toml: dep + feature
[ ] elicitation_creusot/src/foo_types.rs: proof functions
[ ] elicitation_creusot/src/lib.rs: mod foo_types AND pub use foo_types::* (both required)
[ ] extern_specs.rs: extern_spec! blocks for each Select enum label count (N variants)
[ ] Label count proofs: #[trusted] removed (de-trusted via extern_specs)
[ ] All other proofs: #[trusted] retained (string literal opacity wall)
[ ] creusot_runner.rs: CreusotModule::all() entry added (Self::with_feature("foo_types", "foo-types"))
[ ] cargo creusot -- -p elicitation_creusot --features foo-types compiles (generates .coma)
[ ] cargo creusot prove "verif/.../foo_types/verify_*_label_count.coma" — all return ✔

[ ] elicitation_verus/src/foo_types.rs: verus! block
[ ] elicitation_verus/src/lib.rs: pub mod foo_types
[ ] verus_runner.rs: VerusProof::all() entries added (one per proof function)
[ ] verus --crate-type=lib crates/elicitation_verus/src/foo_types.rs → N verified, 0 errors

[ ] git commit -m "feat(foo-types): add Elicitation + verification support for foo crate"
[ ] git push
```

For **macro / fragment-only crates** (skip Phases 2, 4, 5, 6):

```
Crate: foo_macros
(no feature flag needed — no elicitation primitives)

[ ] Workspace Cargo.toml: member added
[ ] elicit_foo/Cargo.toml: elicitation (features=["emit"]), proc-macro2, quote, inventory
[ ] For each macro:
    [ ] src/my_macro.rs: params struct (Deserialize + JsonSchema)
    [ ] impl EmitCode: emit_code() with quote!, crate_deps()
    [ ] inventory::submit!(EmitEntry { tool, crate_name, constructor })
[ ] src/plugin.rs: #[derive(ElicitPlugin)] + #[elicit_tool] handlers calling p.emit_code().to_string()
[ ] src/lib.rs: mod + pub use only
[ ] just check-all elicit_foo passes clean

[ ] Assemble: use shared std__assemble or add domain-specific terminal tool
    (see Fragment Tools section — no separate assemble needed for most crates)

[ ] tests/macro_tools_test.rs: EmitCode output, serde roundtrip, dispatch_emit_from, fragment composition

[ ] git commit -m "feat(elicit_foo): fragment tools for foo macros"
[ ] git push
```

---

## Type Category Reference

The decision of what to verify follows directly from the core principle:
**we own the `from_label`/`Elicitation` logic — verify that. We don't own the
variant definitions — trust those.**

| Type shape | Elicitation pattern | What we verify | What we trust |
|---|---|---|---|
| Enum with known variants | `Select` | `from_label` roundtrip + unknown rejection | Variant definitions from upstream |
| `#[non_exhaustive]` enum | `Select` + wildcard `_ =>` | Same roundtrip invariants on our labels | Any new variants added by upstream |
| Newtype around `String`/primitive | text + `T::new(s)` | Our parse/construction path | Inner type's own invariants |
| Builder struct | text prompt + builder API | `kani::assume(true)` — trust upstream entirely | All builder invariants |
| Complex owned struct | Survey (multi-field) | `kani::assume(true)` — trust upstream entirely | All struct invariants |
| Third-party derive trait | `#[reflect_trait]` factory | Inventory registration + prime/instantiate lifecycle | Trait method implementations in upstream |

---

## Fragment Tools (Macros)

Some Rust APIs are macros that execute at **compile time** and cannot be called
through an MCP boundary at runtime.  Use the **fragment tool** pattern for
these.

### Two tool kinds

| Kind | Returns | Composable? | EmitEntry? |
|---|---|---|---|
| **Fragment tool** | TokenStream as string | ✅ Pass to other tools | ✅ Yes |
| **Terminal tool** (assemble) | `{ main_rs, cargo_toml }` | ❌ Final step only | ❌ No |

### When to use fragment tools

Use this pattern when:
- The API is a Rust macro (`format!`, `query!`, `include_str!`, etc.)
- The API generates code, not runtime data
- The fragment may be composed with other fragments before assembly

### Fragment composition model

Fragment tools produce TokenStream strings that can be chained:

- **Expression-level nesting**: pass a fragment as an arg/field to another tool
- **Statement-level assembly**: collect fragments as steps in `std__assemble`

```
std__env { var: "USER" }               →  env!("USER")
                                              ↓ pass as arg
std__format { template: "Hi, {}!", args: ["env!(\"USER\")"] }
                                       →  format!("Hi, {}!", env!("USER"))
                                              ↓ wrap as statement + pass to
std__assemble { steps: ["let msg = format!(...); println!(\"{}\", msg);"] }
                                       →  { main_rs, cargo_toml }
```

### Implementation checklist — Fragment tool

**1. Params struct** (in `crates/elicit_foo/src/my_macro.rs`):

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MyMacroParams {
    pub arg: String,
}

impl EmitCode for MyMacroParams {
    fn emit_code(&self) -> TokenStream {
        let arg = &self.arg;
        quote! { my_macro!(#arg) }
    }

    fn crate_deps(&self) -> Vec<CrateDep> {
        // Return any non-std crates needed in the emitted binary's Cargo.toml
        vec![CrateDep { name: "my_crate", version: "1" }]
    }
}

inventory::submit! {
    EmitEntry {
        tool: "my_macro",
        crate_name: "elicit_foo",
        constructor: |v| {
            serde_json::from_value::<MyMacroParams>(v)
                .map(|p| Box::new(p) as Box<dyn EmitCode>)
                .map_err(|e| e.to_string())
        },
    }
}
```

**2. Handler** (in `crates/elicit_foo/src/plugin.rs`):

```rust
#[elicit_tool(
    plugin = "foo",
    name = "my_macro",
    description = "Emit a my_macro!(…) expression as a Rust source fragment. \
                   Pass the returned string as an expression to another tool, \
                   or collect as a step for std__assemble."
)]
#[instrument(skip_all)]
async fn emit_my_macro(p: MyMacroParams) -> Result<CallToolResult, ErrorData> {
    let source = p.emit_code().to_string();
    Ok(CallToolResult::success(vec![Content::text(source)]))
}
```

**3. Cargo.toml** — add `emit` feature to `elicitation` dep:

```toml
elicitation = { workspace = true, features = ["emit"] }
proc-macro2.workspace = true
quote.workspace = true
inventory.workspace = true
```

**4. No Phase 2 (elicitation core)** — fragment tools do not need `Elicit`
impls in `crates/elicitation/src/primitives/`.  Skip Phase 2 entirely.

**5. No Kani/Creusot verification** — fragment tools return `TokenStream`; the
verification story is handled when the emitted program is verified, not here.

### Implementation checklist — Terminal tool (`assemble`)

Each macro crate does **not** need its own assemble tool.  The shared
`std__assemble` in `elicit_std` works for any fragments because it uses
`RawFragment` to re-parse pre-rendered strings:

```rust
// AssembleParams wraps each step in RawFragment and passes to BinaryScaffold
let steps: Vec<Box<dyn EmitCode>> = self.steps.iter()
    .map(|s| Box::new(RawFragment(s.clone())) as Box<dyn EmitCode>)
    .collect();
let scaffold = BinaryScaffold::new(steps, self.with_tracing);
let main_rs = scaffold.to_source()?;
let cargo_toml = scaffold.to_cargo_toml(&self.package_name);
```

If a crate needs a domain-specific assembly tool (e.g. pre-wired `use`
statements or custom error handling), use the same `RawFragment` pattern.
Note: crate deps from raw fragments are NOT automatically detected — callers
must ensure the assembled source only uses crates already in the generated
`Cargo.toml`.

### Reference implementation

`crates/elicit_std` — five tools (four fragment, one terminal):

| Tool | Kind | Params |
|---|---|---|
| `std__format` | fragment | `{ template, args[] }` |
| `std__include_str` | fragment | `{ path }` |
| `std__env` | fragment | `{ var, error_message? }` |
| `std__concat` | fragment | `{ parts[] }` |
| `std__assemble` | terminal | `{ steps[], with_tracing?, package_name? }` |

Tests: `crates/elicit_std/tests/macro_tools_test.rs` (31 tests)

---

## Reference Implementation

The complete `clap` integration is the canonical example:


**Elicitation core:**
- `crates/elicitation/src/primitives/clap_types/` — 11 type files
- Feature flag: `clap-types`

**Wrapper crate (newtypes + trait factories):**
- `crates/elicit_clap/src/` — 11 newtype wrapper files + `trait_factories.rs`
- `crates/elicit_clap/tests/trait_factories_test.rs` — 20 integration tests
- Traits covered: `CommandFactory`, `Subcommand`, `ValueEnum`, `Args`
- Traits deferred (incompatible signatures): `FromArgMatches`, `Parser`

**Verification:**
- `crates/elicitation_kani/src/clap_types.rs` — 24 proof harnesses
- `crates/elicitation/src/verification/runner.rs` — harness registration

**Key commits:**
- `feat(elicit_clap): add newtype wrappers for clap types with MCP reflect methods`
- `feat(elicitation_macros): #[reflect_trait] macro + DynamicToolRegistry`
- `feat(elicit_clap): clap trait factories with type_map bridging`
- `test(elicit_clap): 20 integration tests for all clap trait factories`

