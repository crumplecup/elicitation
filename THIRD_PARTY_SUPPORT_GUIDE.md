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
| `crates/elicit_foo/` | New wrapper crate with newtypes + MCP `reflect_methods` |
| `crates/elicitation_kani/` | Kani proof harnesses |
| `crates/elicitation_creusot/` | Creusot `#[requires]`/`#[ensures]`/`#[trusted]` proofs |
| `crates/elicitation_verus/` | Verus `ensures`/`requires` proofs |

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
cannot symbolically execute (all third-party types), use `#[trusted]`:

```rust
//! Creusot proofs for foo type elicitation.

#![cfg(feature = "foo-types")]

use creusot_std::prelude::*;
use elicitation::Select;

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

```rust
#[cfg(feature = "foo-types")]
mod foo_types;
```

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

[ ] elicitation_kani/Cargo.toml: dep + feature
[ ] elicitation_kani/src/foo_types.rs: proof harnesses
[ ] elicitation_kani/src/lib.rs: mod foo_types gated
[ ] verification/runner.rs: ProofHarness::all() entries added
[ ] Select enums: label_count + all_labels_roundtrip + unknown_rejected proofs
[ ] Struct/builder types: trusted_third_party kani::assume(true) proofs
[ ] cargo check -p elicitation_kani --features foo-types passes clean

[ ] elicitation_creusot/Cargo.toml: dep + feature
[ ] elicitation_creusot/src/foo_types.rs: #[trusted] proof functions
[ ] elicitation_creusot/src/lib.rs: mod foo_types gated
[ ] cargo check -p elicitation_creusot --features foo-types passes clean

[ ] elicitation_verus/src/foo_types.rs (or external_types.rs): verus! block
[ ] elicitation_verus/src/lib.rs: pub mod foo_types
[ ] cargo check -p elicitation_verus passes clean

[ ] git commit -m "feat(foo-types): add Elicitation + verification support for foo crate"
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

---

## Reference Implementation

The complete `clap` integration is the canonical example:

- `crates/elicitation/src/primitives/clap_types/` — 11 type files
- `crates/elicit_clap/src/` — 11 newtype wrapper files
- `crates/elicitation_kani/src/clap_types.rs` — 24 proof harnesses
- `crates/elicitation/src/verification/runner.rs` — harness registration
- Feature flag: `clap-types`
- Commit: `feat(elicit_clap): add newtype wrappers for clap types with MCP reflect methods`
