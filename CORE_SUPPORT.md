# Core Support Patterns

This document covers how to implement `ElicitComplete` directly on types from
an upstream library (e.g. `chrono`, `jiff`, `geo`, `reqwest`).  Read
`CONTRACTS.md` for *why* the trait system exists; read this document for *how*
to implement it correctly.

**This document is not for shadow crates.**  Shadow crates wrap upstream types
in newtypes to cross the orphan rule and the MCP boundary.  Core support is
different: you implement traits directly on the upstream type, inside
`elicitation` itself under a `#[cfg(feature = "…")]` gate.  No newtype, no
`From`/`Into` bridge — just direct impls.  See `SHADOW_SUPPORT.md` for the
shadow crate path.

---

## Quick reference

| Situation | Pattern |
|---|---|
| Simple value type (numeric, string-like, opaque struct) | **Concrete leaf** — `Elicitation` + `ElicitIntrospect` + `ElicitPromptTree` + `ToCodeLiteral` |
| Unit-variant enum (no fields) | **Select enum** — add `Select`, then either a file-local macro or hand-written impls |
| Enum with data fields | **Tuple/struct enum** — manual per-variant `Elicitation` |
| Generic wrapper `Foo<T>` | **Generic impl** — no `leaf_impl!`, no `inventory::submit!`; propagate all bounds |
| Upstream error type without public constructors | **Error newtype** — wrap in local `FooWrap`, impl on both |
| Iterator / builder whose state is private | **Parameter capture** — elicitate constructor args, reconstruct in `ToCodeLiteral` |
| Deprecated, orphan-rule-blocked, or unconstructable type | **Intentional skip** — `verif/patches/{crate}.json` entry, no impl |

---

## Communicator and prompt customization

Every `elicit()` method receives a `communicator: &C` (or `_communicator: &C`).
The communicator is the *only* channel through which a caller can swap in a
custom prompt style at runtime.  **Hardcoding the prompt string directly is
wrong** — it silently ignores any style the caller installed.

### The three categories

#### Category A — select / survey types (most types)

These types ask the user something — a numeric value, a string, a choice from
a list, one or more sub-fields.  Every prompt string must come from
`prompt_for_type`, not from a hardcoded literal:

```rust
#[tracing::instrument(skip(communicator))]
async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
    let prompt = communicator
        .style_context()
        .prompt_for_type::<Self>(
            "value",                              // field name
            "foo::Color",                         // field type (for display)
            &crate::style::PromptContext::new(0, 1), // (field_index, total_fields)
        )?
        .unwrap_or_else(|| Self::prompt().unwrap_or("Choose a color:").to_string());
    let params = mcp::select_params(&prompt, &Self::labels());
    // … call_tool, extract result …
}
```

For a **multi-field survey** (struct with several fields), call `prompt_for_type`
once per field, advancing the index:

```rust
let hour_prompt = communicator
    .style_context()
    .prompt_for_type::<Self>("hour", "u8", &crate::style::PromptContext::new(0, 3))?
    .unwrap_or_else(|| "Enter hour (0-23):".to_string());
// … elicit hour …

let minute_prompt = communicator
    .style_context()
    .prompt_for_type::<Self>("minute", "u8", &crate::style::PromptContext::new(1, 3))?
    .unwrap_or_else(|| "Enter minute (0-59):".to_string());
// … elicit minute …

let second_prompt = communicator
    .style_context()
    .prompt_for_type::<Self>("second", "u8", &crate::style::PromptContext::new(2, 3))?
    .unwrap_or_else(|| "Enter second (0-59):".to_string());
// … elicit second …
```

#### Category B — pure delegation types

These types collect sub-values entirely by calling `T::elicit(communicator)` on
their constituent types, with no hardcoded prompt of their own.  Because the
communicator is passed through, the style system already works correctly.  Do
**not** call `prompt_for_type` or `style_or_default` here — just pass the
communicator along:

```rust
async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
    let start = foo::Date::elicit(communicator).await?;
    let end   = foo::Date::elicit(communicator).await?;
    Ok(Self { start, end })
}
```

#### Category C — unit types (single possible value, no user choice)

These types have exactly one value and require no user interaction.  The
communicator is intentionally unused.  Use `_communicator` as the parameter
name (suppresses the unused-variable warning) and use `skip_all` on the
instrument attribute:

```rust
#[tracing::instrument(skip_all)]
async fn elicit<C: ElicitCommunicator>(_communicator: &C) -> ElicitResult<Self> {
    Ok(foo::Utc)
}
```

Unit types arise when:

- The type is a unit struct (e.g. a timezone marker like `chrono::Utc`)
- The type is an error with only one possible value (e.g. an out-of-range error
  that is produced by a single known-invalid operation)
- The type is uninhabited (no valid value exists; `elicit()` always returns `Err`)

### What `style_or_default` is NOT

`style_or_default` fetches the style object associated with `Self`.  Calling it
and then ignoring the return value is just as broken as not calling it at all —
the style is fetched but has no effect on the prompt the user sees.  The style
object is only meaningful when you call `prompt_for_field` on it yourself (via
`prompt_for_type`, which does this internally).

```rust
// ❌ WRONG — style fetched but discarded; prompt is still hardcoded
let _style = communicator.style_or_default::<Self>()?;
let params = mcp::select_params(Self::prompt().unwrap_or("hardcoded"), &labels);

// ❌ WRONG — style fetched, debug-logged, then thrown away
let style = communicator.style_or_default::<Self>()?;
tracing::debug!(?style, "eliciting");
let params = mcp::select_params(Self::prompt().unwrap_or("still hardcoded"), &labels);

// ✅ CORRECT — style is applied to the prompt via prompt_for_type
let prompt = communicator
    .style_context()
    .prompt_for_type::<Self>("value", "foo::Color", &crate::style::PromptContext::new(0, 1))?
    .unwrap_or_else(|| Self::prompt().unwrap_or("Choose a color:").to_string());
let params = mcp::select_params(&prompt, &labels);
```

---

## File layout

For a crate named `foo`:

```text
crates/elicitation/src/
├── foo_support.rs                  ← all Elicitation/ElicitIntrospect/etc. impls
├── type_spec/foo_specs.rs          ← all ElicitSpec impls
├── prompt_tree.rs                  ← leaf_impl! calls for concrete types
└── lib.rs                          ← pub use re-exports of any new wrapper types
```

Gate every item in `foo_support.rs` with `#[cfg(feature = "foo")]`.  In
`type_spec/foo_specs.rs` the same gate applies to each impl block (or to the
whole module via a `mod` declaration in `type_spec/mod.rs`).

The `prompt_tree.rs` blocks are already gated by feature modules; add your
`leaf_impl!` calls inside the correct `#[cfg(feature = "foo")]` block.

---

## Trait checklist for every concrete type

Each type needs the following (check them in order — the compiler error
cascade is predictable):

| Trait | Required? | Notes |
|---|---|---|
| `Prompt` | Yes | `fn prompt() → Option<&'static str>` — one sentence |
| `Elicitation` | Yes | The async `elicit` fn + three proof fns |
| `ElicitIntrospect` | Yes | `pattern()` + `metadata()` |
| `ElicitPromptTree` | Yes | `prompt_tree()` — Leaf or Select shape |
| `ToCodeLiteral` | Yes | `to_code_literal()` + `type_tokens()` |
| `ElicitSpec` | Yes | In `type_spec/foo_specs.rs` |
| `Select` | Only for enum types | `options()`, `labels()`, `from_label()` |
| `leaf_impl!` in prompt_tree.rs | Only for concrete (non-generic) types | Registers in the prompt tree |
| `inventory::submit!` in foo_specs.rs | Only for concrete (non-generic) types | Registers in the spec registry |
| `ElicitComplete` | Automatic | Blanket impl fires once all the above are satisfied |

---

## 1. Concrete leaf type

The most common case: a value type with public fields or constructor.

```rust
// crates/elicitation/src/foo_support.rs

impl Prompt for foo::Scalar {
    fn prompt() -> Option<&'static str> {
        Some("Enter a scalar value:")
    }
}

impl Elicitation for foo::Scalar {
    type Style = ScalarStyle;                     // generated by default_style!

    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting foo::Scalar");
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "foo::Scalar",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| Self::prompt().unwrap_or("Enter scalar:").to_string());
        let params = mcp::number_params(&prompt, i64::MIN, i64::MAX);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_number())
                    .with_arguments(params),
            )
            .await?;
        let value = mcp::extract_value(result)?;
        let raw = mcp::parse_integer::<i64>(value)?;
        foo::Scalar::from_i64(raw).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "foo::Scalar out of range: {raw}"
            )))
        })
    }

    fn kani_proof()    -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_trusted_opaque("foo::Scalar")
    }
    fn verus_proof()   -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::verus_trusted_opaque("foo::Scalar")
    }
    fn creusot_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::creusot_trusted_opaque("foo::Scalar")
    }
}

impl ElicitIntrospect for foo::Scalar {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Primitive }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "foo::Scalar",
            description: Self::prompt(),
            details: PatternDetails::Primitive,
        }
    }
}

impl ElicitPromptTree for foo::Scalar {
    fn prompt_tree() -> PromptTree {
        PromptTree::Leaf {
            prompt: Self::prompt().map(str::to_string)
                .unwrap_or_else(|| "foo::Scalar".to_string()),
            type_name: "foo::Scalar".to_string(),
        }
    }
}

impl ToCodeLiteral for foo::Scalar {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let v = self.as_i64();
        crate::quote::quote! {
            match foo::Scalar::from_i64(#v) {
                Some(s) => s,
                None => return Err("foo::Scalar: value out of range".into()),
            }
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { foo::Scalar }
    }
}

crate::default_style!(foo::Scalar => ScalarStyle);
```

Register in `prompt_tree.rs` (inside the `#[cfg(feature = "foo")]` block):

```rust
leaf_impl!(foo::Scalar, "foo::Scalar");
```

Add `ElicitSpec` in `type_spec/foo_specs.rs`:

```rust
#[cfg(feature = "foo")]
{
    impl crate::ElicitSpec for foo::Scalar {
        fn type_spec() -> crate::TypeSpec {
            crate::TypeSpec::new(
                "foo::Scalar",
                "A scalar value.",
                vec![],
            )
        }
    }
    inventory::submit!(crate::TypeSpecInventoryKey::new(
        "foo::Scalar",
        <foo::Scalar as crate::ElicitSpec>::type_spec,
        std::any::TypeId::of::<foo::Scalar>,
    ));
}
```

### Proof helper selection

| Type shape | `kani_proof` helper |
|---|---|
| Opaque struct / most leaf types | `kani_trusted_opaque("foo::Type")` |
| Unit-variant enum | `kani_select_wrapper("foo::Type", "DefaultVariant")` |
| Multi-variant enum with data | `kani_multi_variant_enum(…)` — check the existing helpers |

Same pattern for `verus_proof` and `creusot_proof`.

---

## 2. Unit-variant enum (Select enum)

For enums whose variants carry no data, `Select` gives you dropdown behavior.
Define a file-local macro if you have several; or hand-write if you have one.

```rust
impl Select for foo::Color {
    fn options() -> Vec<Self> {
        vec![foo::Color::Red, foo::Color::Green, foo::Color::Blue]
    }
    fn labels() -> Vec<String> {
        vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()]
    }
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Red"   => Some(foo::Color::Red),
            "Green" => Some(foo::Color::Green),
            "Blue"  => Some(foo::Color::Blue),
            _       => None,
        }
    }
}

crate::default_style!(foo::Color => ColorStyle);

impl Elicitation for foo::Color {
    type Style = ColorStyle;
    #[tracing::instrument(skip(communicator))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting foo::Color");
        let labels = Self::labels();
        let prompt = communicator
            .style_context()
            .prompt_for_type::<Self>(
                "value",
                "foo::Color",
                &crate::style::PromptContext::new(0, 1),
            )?
            .unwrap_or_else(|| Self::prompt().unwrap_or("Choose a color:").to_string());
        let params = mcp::select_params(&prompt, &labels);
        let result = communicator
            .call_tool(
                rmcp::model::CallToolRequestParams::new(mcp::tool_names::elicit_select())
                    .with_arguments(params),
            )
            .await?;
        let label = mcp::parse_string(mcp::extract_value(result)?)?;
        Self::from_label(&label).ok_or_else(|| {
            ElicitError::new(ElicitErrorKind::ParseError(format!(
                "Invalid foo::Color: {label}"
            )))
        })
    }
    fn kani_proof() -> proc_macro2::TokenStream {
        crate::verification::proof_helpers::kani_select_wrapper("foo::Color", "Red")
    }
    // … verus_proof, creusot_proof
}

impl ElicitIntrospect for foo::Color {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata {
        TypeMetadata {
            type_name: "foo::Color",
            description: Self::prompt(),
            details: PatternDetails::Select {
                variants: Self::labels().into_iter()
                    .map(|label| VariantMetadata { label, fields: vec![] })
                    .collect(),
            },
        }
    }
}

impl ElicitPromptTree for foo::Color {
    fn prompt_tree() -> PromptTree {
        let labels = Self::labels();
        let count  = labels.len();
        PromptTree::Select {
            prompt:    Self::prompt().unwrap_or("Choose a color:").to_string(),
            type_name: "foo::Color".to_string(),
            options:   labels,
            branches:  vec![None; count],
        }
    }
}

impl ToCodeLiteral for foo::Color {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        match self {
            foo::Color::Red   => crate::quote::quote! { foo::Color::Red   },
            foo::Color::Green => crate::quote::quote! { foo::Color::Green },
            foo::Color::Blue  => crate::quote::quote! { foo::Color::Blue  },
        }
    }
    fn type_tokens() -> proc_macro2::TokenStream {
        crate::quote::quote! { foo::Color }
    }
}
```

If you have many such enums in one crate, define a file-local macro (see
`chrono_select_enum!` in `datetime_chrono.rs`) to generate all of the above
from a variant list.

---

## 3. Generic wrapper type

For types like `foo::Result<T>` or `foo::Optional<T>`, implement on the
parameterized form.  Two hard rules:

**Do not call `leaf_impl!`** — `leaf_impl!` requires a concrete type because
`TypeId` cannot be computed for a generic.

**Do not call `inventory::submit!`** — same reason.  Use `TypeSpec::new()`
directly in `ElicitSpec::type_spec()` without submitting.

**Propagate all bounds** — `Elicitation for Foo<T>` requires
`T: Elicitation + Clone + Send`.  `ElicitIntrospect` has `Elicitation` as a
supertrait, so `ElicitIntrospect for Foo<T>` needs `T: ElicitIntrospect + Clone + Send`
(not just `T: ElicitIntrospect`).  The compiler will tell you exactly which
bounds are missing — follow the errors in order.

```rust
impl<T: Elicitation + Clone + Send> Elicitation for foo::Optional<T> {
    type Style = OptionalStyle;
    #[tracing::instrument(skip(communicator), fields(inner_type = std::any::type_name::<T>()))]
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        // …
    }
    fn kani_proof()    -> proc_macro2::TokenStream { <T as Elicitation>::kani_proof() }
    fn verus_proof()   -> proc_macro2::TokenStream { <T as Elicitation>::verus_proof() }
    fn creusot_proof() -> proc_macro2::TokenStream { <T as Elicitation>::creusot_proof() }
}

impl<T: ElicitPromptTree> ElicitPromptTree for foo::Optional<T> {
    fn prompt_tree() -> PromptTree { /* … */ }
}

// ElicitIntrospect: supertrait requires Elicitation, so bounds must include both.
impl<T: ElicitIntrospect + Clone + Send> ElicitIntrospect for foo::Optional<T> {
    fn pattern() -> ElicitationPattern { ElicitationPattern::Select }
    fn metadata() -> TypeMetadata { /* … */ }
}

impl<T: crate::emit_code::ToCodeLiteral> crate::emit_code::ToCodeLiteral for foo::Optional<T> {
    fn to_code_literal(&self) -> proc_macro2::TokenStream { /* … */ }
    fn type_tokens() -> proc_macro2::TokenStream {
        let inner = <T as crate::emit_code::ToCodeLiteral>::type_tokens();
        crate::quote::quote! { foo::Optional<#inner> }
    }
}

// ElicitSpec — no inventory::submit! here
impl<T: crate::ElicitSpec> crate::ElicitSpec for foo::Optional<T> {
    fn type_spec() -> crate::TypeSpec {
        crate::TypeSpec::new(
            "foo::Optional",
            "An optional value.",
            vec![],
        )
    }
}
```

---

## 4. Error wrapper newtype

Upstream error types frequently have no public constructor and no public fields —
the only way to obtain one is from a fallible operation.  However, **never
convert an error to a string to make it serializable.**  Doing so destroys the
error chain (breaking `anyhow`, `miette`, and any library that relies on
`Error::source()` for display), and also destroys all structural variant
information a caller might need to match on programmatically.

Three strategies, in preference order:

### Strategy A — hold the original error directly (preferred)

If the upstream error type is `Copy` or `Clone`, wrap it in a tuple struct and
implement `Serialize`/`Deserialize` manually by serializing structured variant
information, not the display string.

```rust
/// Wrapper around `foo::ParseError` — serializes as a kind-label string.
///
/// Stores the real error (which is `Copy`).  Serializes via `kind()` so that
/// variant information survives the round-trip.  Deserializes by triggering
/// the real parse operation that produces each kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParseErrorWrap(foo::ParseError);

impl serde::Serialize for ParseErrorWrap {
    fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        let label = match self.0.kind() {
            foo::ParseErrorKind::Invalid  => "Invalid",
            foo::ParseErrorKind::TooShort => "TooShort",
            // … all variants
            _ => "Invalid",
        };
        ser.serialize_str(label)
    }
}

impl<'de> serde::Deserialize<'de> for ParseErrorWrap {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let label = String::deserialize(de)?;
        // Reconstruct by triggering the real operation that fails with that kind.
        let err = match label.as_str() {
            "Invalid"  => foo::Date::parse("not-a-date").err().ok_or_else(|| {
                <D::Error as serde::de::Error>::custom("parse unexpectedly succeeded")
            })?,
            "TooShort" => foo::Date::parse("").err().ok_or_else(|| {
                <D::Error as serde::de::Error>::custom("parse unexpectedly succeeded")
            })?,
            other => return Err(D::Error::unknown_variant(other, &["Invalid", "TooShort"])),
        };
        Ok(Self(err))
    }
}

impl From<foo::ParseError> for ParseErrorWrap {
    fn from(e: foo::ParseError) -> Self { Self(e) }
}

impl ParseErrorWrap {
    pub fn into_inner(self) -> foo::ParseError { self.0 }
}
```

The proof must verify that the round-trip preserves `kind()` — the only
publicly observable property.  See `verify_parse_error_wrap_preserves_kind` in
`datetime_chrono.rs` for the canonical hand-written Kani harness.

### Strategy B — `Arc` source field for non-`Clone` errors

If the upstream error is not `Copy`/`Clone` but is `Send + Sync + 'static`
(which all `std::error::Error` implementors should be), hold it under `Arc` in
a `source` field and carry a `message: String` solely for serialization and
display.  The `message` field must be annotated `#[error(not(source))]` so that
`derive_more::Error` threads the error chain through `source`, not `message`.

```rust
#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("foo error: {message}")]
pub struct FooError {
    /// Real error preserved for the error chain (skipped during serialization).
    #[error(source)]
    #[serde(skip)]
    source: Option<std::sync::Arc<foo::Error>>,

    /// Display-only capture — NOT the error source.
    #[error(not(source))]
    message: String,

    pub line: u32,
    #[serde(skip)]
    pub file: &'static str,
}

impl FooError {
    #[track_caller]
    pub fn new(e: foo::Error) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: e.to_string(),      // display only
            source: Some(std::sync::Arc::new(e)),   // chain preserved
            line: loc.line(),
            file: loc.file(),
        }
    }
}
```

The `message` string is a transport artifact.  The error identity and chain live
in `source`.  See `UrlError` in `crates/elicitation/src/error.rs` for the
canonical example.

### Strategy C — unit or trivially-constructable errors

Some upstream error types are effectively unit structs (only one possible value,
no information content).  Serialize as `null` and reconstruct by triggering the
single operation that fails.  See `OutOfRangeWrap(chrono::OutOfRange)` in
`datetime_chrono.rs` for the canonical example.

### What never to do

```rust
// ❌ WRONG — erases variant structure and breaks the error chain
pub struct FooParseErrorWrap {
    pub message: String,
}
impl From<foo::ParseError> for FooParseErrorWrap {
    fn from(e: foo::ParseError) -> Self {
        Self { message: e.to_string() }   // chain severed here
    }
}
```

This pattern makes it impossible to:

- Display the root cause with `anyhow`/`miette` (no `Error::source()`)
- Match programmatically on error kind
- Reconstruct the correct variant on deserialization

No `.expect()` or `.unwrap()` anywhere in wrapper impls, including `From`.

---

## 5. Iterator / builder type

Iterators carry private state that cannot be reconstructed.  Instead, elicitate
the *parameters that produced the iterator* and reconstruct the constructor call
in `ToCodeLiteral`.

```rust
// foo::DayIter is produced by NaiveDate::iter_days(limit)
impl Elicitation for foo::DayIter {
    async fn elicit<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self> {
        tracing::debug!("Eliciting DayIter: start date");
        let start = foo::Date::elicit(communicator).await?;
        // Elicitate limit …
        Ok(start.iter_days(limit))
    }
    // …
}

impl ToCodeLiteral for foo::DayIter {
    fn to_code_literal(&self) -> proc_macro2::TokenStream {
        let start = self.start().to_code_literal();
        let limit = self.limit();
        crate::quote::quote! {
            { let start = #start; start.iter_days(#limit) }
        }
    }
}
```

If the iterator exposes no accessors (fully opaque), still implement
`Elicitation` (which produces values via the constructor path) but produce a
`ToCodeLiteral` that reconstructs the last-known constructor arguments
captured in a local `FooIterWrap` newtype.

---

## Duplicate impl prevention

Before implementing `ToCodeLiteral` or `ElicitPromptTree` for a type,
search for it in:

- `crates/elicitation/src/emit_code.rs` — old `ToCodeLiteral` impls
- `crates/elicitation/src/prompt_tree.rs` — old `leaf_impl!` calls

If a stale impl exists, **remove it** before adding the new one.  The compiler
will produce `E0119` (conflicting implementations) otherwise.  A comment like
`// impl moved to foo_support.rs` is enough to mark the removal.

---

## Feature gate discipline

Every item in `foo_support.rs` must be gated.  The file itself should start
with a top-level attribute, or every `impl` block should carry one:

```rust
// Option A — whole-file gate (preferred for dedicated crate files)
#![cfg(feature = "foo")]

// Option B — per-item gate (required when the file is shared)
#[cfg(feature = "foo")]
impl Elicitation for foo::Type { … }
```

In `type_spec/foo_specs.rs`, all items must be in a `#[cfg(feature = "foo")]`
block.  In `prompt_tree.rs`, `leaf_impl!` calls must be inside the correct
`#[cfg(feature = "foo")]` section.

Feature-gated code should use full paths for `tracing` macros
(`tracing::debug!(…)`) rather than a `use tracing::debug;` that becomes an
orphaned import when the feature is off.

---

## Intentional skips

Three categories of upstream types that should **not** receive impls:

| Category | Example | Reason |
|---|---|---|
| Deprecated | `chrono::Date<Tz>` | Upstream removed; using it warns users |
| Unconstructable | `chrono::format::DelayedFormat<I>` | Private fields, no public ctor, consumed on first use |
| Orphan-rule blocked | `chrono::ParseResult<T>` = `Result<T, ParseError>` | `Result` is from `std`; we own neither side |

For each such type, add an entry to `verif/patches/{crate}.json`:

```json
[
  {
    "path":   "foo::OldType",
    "reason": "deprecated since foo 1.5; upstream recommends NewType instead"
  },
  {
    "path":   "foo::Builder",
    "reason": "private fields with no public constructor; consumed on build()"
  }
]
```

The `elicit_doc` pipeline reads these files and excludes the entries from the
coverage count, displaying them instead in an "Intentionally skipped" section
at the bottom of the checklist.

---

## Formal proofs

The three proof methods on `Elicitation` —

```rust
fn kani_proof()    -> proc_macro2::TokenStream;
fn verus_proof()   -> proc_macro2::TokenStream;
fn creusot_proof() -> proc_macro2::TokenStream;
```

— are not boilerplate to fill in and forget.  They are the machine-checkable
record of what we actually guarantee about each type.  `elicitation_derive`
generates proof harnesses from them; the harnesses run in CI.  An empty or
wrong proof silently degrades coverage without anyone noticing.

### The two proof responsibilities

Every `ElicitComplete` impl carries two distinct obligations:

1. **Third-party type boundary** — What do we assume the upstream library
   guarantees about this type?  We cannot verify the library's internals, so
   we mark the boundary explicitly and state the axiom.

2. **Our own wrapping logic** — What does *our* code add on top of the upstream
   type?  Label round-trips in `Select::from_label`, error-path coverage in
   `Elicitation::elicit`, reconstruction correctness in `ToCodeLiteral`.  This
   is code *we wrote*, so it must be verified, not assumed.

Conflating the two — applying a "trusted boundary" helper to our own code —
is the principal failure mode.  It produces harnesses that look correct but
verify nothing about the behavior we are actually responsible for.

### Proof helper selection matrix

All helpers live in `crate::verification::proof_helpers`.  Pick the most
specific one available.

**For third-party concrete types (opaque internals):**

| Type shape | Kani helper | Notes |
|---|---|---|
| Opaque struct, value type | `kani_trusted_opaque(name)` | Correct choice; documents the trusted boundary explicitly |
| Unit-variant enum via `Select` | `kani_select_wrapper(name, "DefaultVariant")` | Verifies our label→variant mapping; NOT `trusted_opaque` |
| Multi-variant enum | `kani_multi_variant_enum(name, "DefaultVariant")` | Verifies first-variant constructibility |
| Single-variant enum | `kani_single_variant_enum(name)` | |
| Generic wrapper `Foo<T>` | `<T as Elicitation>::kani_proof()` | Delegates to inner type's own proof |

`verus_proof` and `creusot_proof` have parallel families
(`verus_trusted_opaque`, `verus_select_wrapper`, `creusot_trusted_opaque`,
`creusot_select_wrapper`, etc.).

**For our own wrapper newtypes (e.g. `ParseErrorWrap`, `OutOfRangeWrap`):**

Use `kani_newtype_wrapper_harness(name)` / `verus_newtype_wrapper_harness` /
`creusot_newtype_wrapper_harness`.  These prove structural wrapper identity —
that the inner type's own proofs compose correctly through the newtype — and
are the right proof level for types whose primary purpose is carrying a
message string extracted from an upstream error.

**For constrained numeric types:**

| Constraint | Kani helper |
|---|---|
| Value > 0 | `kani_numeric_positive(name, "i64")` |
| Value ≥ 0 | `kani_numeric_nonneg(name, "i64")` |
| Value ≠ 0 | `kani_numeric_nonzero(name, "i64")` |
| Float, finite | `kani_float_finite(name, "f64")` |
| Float > 0 | `kani_float_positive(name, "f64")` |

These generate harnesses that use `kani::any()` to synthesize inputs and
assert that the invariant holds after construction.  They actually prove the
constraint; they are not stubs.

**For formal method contracts:**

Use `kani_formal_method_harness(fn_name, &[inputs], &[outputs])` to link
`Established<P>` contract tokens to a function symbol.  For full input
synthesis, use the `#[formal_method]` attribute macro instead.

### What `kani_trusted_opaque` actually means

`kani_trusted_opaque` emits a proof harness with an empty body and a comment
that says "trusted boundary."  It is not a shortcut and it is not lazy — it
is the *correct and complete* proof for a type whose internals we cannot model.
The comment is load-bearing: it records that we have consciously decided to
accept the upstream library's own test suite and type system as the authority
on this type's correctness, and that our wrapper only adds an elicitation
interface on top.

What makes `trusted_opaque` wrong is applying it to code *we wrote*.  The
test is: whose source code is being relied upon?  If it's upstream's source,
`trusted_opaque` is correct.  If it's ours, there is a more specific helper
that actually verifies our behavior.

### Verifying the label round-trip

For every type that implements `Select`, the label round-trip is our code:

```text
elicit() calls from_label(label) → variant
```

`kani_select_wrapper` generates a harness that proves:

- A known valid label maps to `Some(variant)`.
- An unknown sentinel label maps to `None`.

This is the minimum.  If the enum has constraints on which labels are valid in
which context, add a hand-written harness alongside the generated one.

### Verifying error wrapper construction

Error wrapper newtypes (Section 4 above) have one logical invariant: the
`message` field faithfully captures the upstream error's `Display` output.
The `From<upstream::Error>` impl is our code.  The wrapping logic should be
covered by `kani_newtype_wrapper_harness`, which asserts structural wrapper
identity and composes the inner type's proof.

If the wrapper adds parsing logic (e.g. extracting a field from the message
string), write a hand-written harness for that logic.

### Generic impls: delegate, do not stub

For `impl<T: Elicitation> Elicitation for Foo<T>`, the proof methods must
delegate to `T`:

```rust
fn kani_proof()    -> proc_macro2::TokenStream { <T as Elicitation>::kani_proof() }
fn verus_proof()   -> proc_macro2::TokenStream { <T as Elicitation>::verus_proof() }
fn creusot_proof() -> proc_macro2::TokenStream { <T as Elicitation>::creusot_proof() }
```

Returning `TokenStream::new()` (an empty stream) here is a silent failure: the
derive macro accepts it, no harness is generated, and no one notices.  Always
delegate.  `LocalResult<T>` in `datetime_chrono.rs` is the canonical example.

### Proof coverage and the audit checklist

`elicit_doc` tracks which types have proof harnesses and flags ones that emit
empty token streams.  Before marking a type done:

1. Run `just check elicitation` — confirms the token stream compiles.
2. Search for the generated harness name in `verif/` — confirm it was emitted.
3. For Kani: `cargo kani --harness verify_foo_bar` — confirm it actually runs.

A harness that compiles but is never executed is not a proof.

---

## Hard rules

These caused repeated bugs during the chrono implementation. Treat them as
invariants:

**No `.expect()` or `.unwrap()`.** Use `ok_or_else`, `match`, or `?`.  This
applies to every location in `foo_support.rs`, including `ToCodeLiteral` impls
and `From` conversions.

**Never use `gen` as a variable name.** It is a reserved keyword in Rust 2024.
Use `items`, `output`, `produced`, or any other name.

**Deserialize impls with `D::Error`:** The `serde::de::Error` trait is not
automatically in scope.  Always write:

```rust
Err(<D::Error as serde::de::Error>::custom("message"))
```

not `D::Error::custom("message")`.

**No `#[allow]` directives.** If the compiler warns about dead code, add a
feature gate, a getter, or delete the code.  The suppressor hides problems that
need to be fixed.

---

## Workflow

1. **Start from the checklist** — open `verif/coverage/{crate}.checklist.md`
   (generated by `elicit_doc`).  Items with `[ ]` are missing impls.

2. **Identify each type's shape** — look up the upstream rustdoc to determine
   which of the five patterns above applies.

3. **Implement in batch** — write all traits for one type in `foo_support.rs`
   before moving to the next.  The compiler error cascade is predictable and
   serves as a checklist.

4. **Lint after each type**:

   ```bash
   just lint elicitation --features {crate}
   ```

   Zero warnings before moving on.

5. **Declare intentional skips** — after all implementable types are done,
   add a `verif/patches/{crate}.json` for the remainder.

6. **Export from lib.rs** — any new public wrapper types (error newtypes,
   select wrappers) must be re-exported at the crate root.  Check lib.rs for
   existing re-export blocks to place yours alongside.

7. **Add to `prompt_tree.rs`** — add `leaf_impl!` for every new *concrete*
   type inside the correct `#[cfg(feature = "foo")]` section.

8. **Final check**:

   ```bash
   just check elicitation
   ```
