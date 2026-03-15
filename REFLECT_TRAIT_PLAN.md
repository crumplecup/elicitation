# `#[reflect_trait]` Implementation Plan

## Problem

The current `elicit_*` newtype approach requires one file per wrapped type.
For trait families (like `Select`) where many types share the same interface,
wrapping each type independently means duplicating the same method signatures
N times. A `#[reflect_trait]` attribute macro lets you write one impl block
per type and have the macro generate both the delegation bodies and MCP tool
registrations.

---

## Core Design

### Where it lives

`#[reflect_trait]` is an **attribute macro** (not a derive). Per project
convention: `_macros` for attribute macros, `_derive` for derive macros.
Goes in **`crates/elicitation_macros/src/`**.

> Note: `#[reflect_methods]` is an attribute macro that lives in
> `elicitation_derive` — a historical inconsistency. We follow the stated
> rule here.

### Syntax

```rust
// 1. Define the wrapper trait by hand (no macro on the definition)
pub trait SelectTools: elicitation::Select {}

// 2. For each concrete type, use #[reflect_trait] on the impl
#[reflect_trait(elicitation::Select)]
impl SelectTools for clap::ColorChoice {
    fn labels() -> Vec<String>;
    fn from_label(label: String) -> Option<clap::ColorChoice>;
    fn options() -> Vec<clap::ColorChoice>;
}
```

The macro argument `elicitation::Select` is the **source trait** whose methods
we delegate to. This is required because without it, calling `Self::labels()`
inside `SelectTools::labels()` would recurse infinitely.

### Name derivation

Tool names are derived from the `for T` type path, stripped of namespace and
lowercased. `clap::ColorChoice` → `color_choice`. Method name appended:
`color_choice_labels`, `color_choice_from_label`, `color_choice_options`.

If the type name contains ambiguity (e.g. `clap::builder::ValueHint`), only
the last segment is used: `value_hint`. An optional `as Name` argument
overrides the derived name:

```rust
#[reflect_trait(elicitation::Select, as = ValueHint)]
impl SelectTools for clap::builder::ValueHint { ... }
```

---

## What the Macro Generates

Given:

```rust
#[reflect_trait(elicitation::Select)]
impl SelectTools for clap::ColorChoice {
    fn labels() -> Vec<String>;
    fn from_label(label: String) -> Option<clap::ColorChoice>;
}
```

The macro emits:

```rust
// 1. The impl block with generated delegation bodies
impl SelectTools for clap::ColorChoice {
    fn labels() -> Vec<String> {
        <clap::ColorChoice as elicitation::Select>::labels()
    }
    fn from_label(label: String) -> Option<clap::ColorChoice> {
        <clap::ColorChoice as elicitation::Select>::from_label(&label)
    }
}

// 2. A param struct for methods with arguments
#[derive(Debug, Clone, elicitation::Elicit, schemars::JsonSchema)]
pub struct ColorChoiceFromLabelParams {
    pub label: String,
}

// 3. MCP tool wrappers in a separate inherent impl
impl clap::ColorChoice {
    #[tool(description = "labels — list all ColorChoice labels")]
    pub fn color_choice_labels_tool(
        &self,
        _params: rmcp::Parameters<()>,
    ) -> Result<rmcp::Json<Vec<String>>, rmcp::ErrorData> {
        Ok(rmcp::Json(<clap::ColorChoice as elicitation::Select>::labels()))
    }

    #[tool(description = "from_label — get ColorChoice from label string")]
    pub fn color_choice_from_label_tool(
        &self,
        params: rmcp::Parameters<ColorChoiceFromLabelParams>,
    ) -> Result<rmcp::Json<Option<clap::ColorChoice>>, rmcp::ErrorData> {
        Ok(rmcp::Json(<clap::ColorChoice as elicitation::Select>::from_label(&params.label)))
    }
}
```

### Differences from `#[reflect_methods]`

| | `#[reflect_methods]` | `#[reflect_trait]` |
|---|---|---|
| Applied to | `impl Type { fn bodies; }` | `impl Trait for Type { fn sigs; }` |
| Method bodies | Written by caller | Generated (delegate to source trait) |
| Delegation | `self.0.method()` (via Deref) | `<Type as SourceTrait>::method()` |
| Type context | Inherent impl, self is known | `for Type` clause gives the type |
| Param source | From existing body | From bare signature |

---

## Argument signature type adaptation

Bare signatures in the impl block may use references that aren't owned.
The macro applies the same conversions as `#[reflect_methods]`:

| Input param type | Param struct field type | Conversion |
|---|---|---|
| `&str` | `String` | `param.as_str()` |
| `&T` | `T` (requires `T: Clone`) | `&param` |
| `String` | `String` | pass-through |
| Owned `T` | `T` | pass-through |

Static methods (no `self`/`&self`) get a unit `()` params type.

---

## Module layout

```
crates/elicitation_macros/src/
├── lib.rs                      ← add pub fn reflect_trait()
└── trait_reflection/
    ├── mod.rs                  ← expand() entry point
    ├── delegation.rs           ← generate delegation bodies
    ├── naming.rs               ← derive tool prefix from type path
    ├── params.rs               ← reuse / adapt from method_reflection::params
    └── wrapper.rs              ← reuse / adapt from method_reflection::wrapper
```

The `params.rs` and `wrapper.rs` from `method_reflection` in `elicitation_derive`
are good references but live in the wrong crate. Either copy and adapt, or
factor the shared logic into a utility crate (see Milestone 3 below).

---

## Milestones

### Milestone 1 — Core delegation + param structs (no tools)

Deliverables:
- `trait_reflection/mod.rs` — parses `impl Trait for Type { fn sigs; }`
- `trait_reflection/delegation.rs` — generates `impl Trait for Type` with bodies
- `trait_reflection/naming.rs` — type path → snake_case prefix
- `trait_reflection/params.rs` — param struct generation for sigs with args
- `lib.rs` — wire up `pub fn reflect_trait()`
- Tests in `crates/elicitation/tests/reflect_trait_basic_test.rs`

Test shape (Milestone 1):

```rust
use elicitation_macros::reflect_trait;

trait Greet {
    fn hello(name: String) -> String;
    fn bye() -> String;
}

struct Foo;
impl Greet for Foo {
    fn hello(name: String) -> String { format!("hello {name}") }
    fn bye() -> String { "bye".to_string() }
}

trait GreetWrapper: Greet {}

#[reflect_trait(Greet)]
impl GreetWrapper for Foo {
    fn hello(name: String) -> String;
    fn bye() -> String;
}

// verify delegation works
assert_eq!(Foo::hello("world".to_string()), "hello world");
assert_eq!(Foo::bye(), "bye");
```

### Milestone 2 — MCP tool wrappers

Deliverables:
- `trait_reflection/wrapper.rs` — generate `#[tool]` methods
- Tool name derivation from type path + method name
- Unit tests verifying generated tool names
- Integration with `#[tool]` attribute (from `rmcp`)

### Milestone 3 — Apply to `elicit_clap` Select types

Replace the 5 `elicit_clap` newtype files that wrap Select enums with
`#[reflect_trait]` impls. Each file goes from ~50 lines to ~10 lines.

Before (in `elicit_clap/src/color_choice.rs`):
```rust
elicit_newtype!(clap::ColorChoice, as ColorChoice);
elicit_newtype_traits!(ColorChoice);

#[reflect_methods]
impl ColorChoice {
    pub fn labels() -> Vec<String> { clap::ColorChoice::labels() }
    pub fn from_label(&self, label: String) -> Option<clap::ColorChoice> { ... }
    pub fn options() -> Vec<clap::ColorChoice> { clap::ColorChoice::options() }
}
```

After:
```rust
#[reflect_trait(elicitation::Select)]
impl SelectTools for clap::ColorChoice {
    fn labels() -> Vec<String>;
    fn from_label(label: String) -> Option<clap::ColorChoice>;
    fn options() -> Vec<clap::ColorChoice>;
}
```

### Milestone 4 — Generic method support

Add bounds propagation for methods with generic type parameters, matching
the `#[reflect_methods]` generic support:

```rust
#[reflect_trait(elicitation::Filter)]
impl FilterTools for MyCollection {
    fn find<T: Serialize + Deserialize + JsonSchema>(&self, item: T) -> Option<T>;
}
```

---

## Open questions for review

1. **Source trait path syntax** — `#[reflect_trait(elicitation::Select)]` uses
   a path. Should it accept `Select` (unqualified, with a `use` in scope) or
   require the full path? Full path is safer for macro expansion context.

2. **Wrapper trait required?** — The design requires a hand-written
   `trait SelectTools: Select {}` before the impl. We could instead have the
   macro generate a single-method trait per delegation or skip the trait
   entirely and add the methods as inherent impls only. The trait-based
   approach is cleaner for type system expressiveness. Confirm?

3. **Static-only methods** — All `Select` methods are static (`fn`, not `fn(&self)`).
   The MCP tool wrappers currently assume `&self`. We need either unit `()` receivers
   or a different registration strategy. `#[reflect_methods]` already handles this
   (see `is_consuming()` path) — confirm the trait version should follow the same rule.

4. **Reuse vs copy of `params.rs` / `wrapper.rs`** — Since the logic is nearly
   identical to `method_reflection`, should we extract a shared `elicitation_macro_utils`
   crate, or keep a copy in `trait_reflection/`? Copying is simpler initially.

5. **`as = Name` vs positional** — Should the tool name override be
   `#[reflect_trait(Select, as = ColorChoice)]` or `#[reflect_trait(Select, ColorChoice)]`?
   Named arg is clearer.

---

## Checklist

- [ ] `crates/elicitation_macros/src/trait_reflection/mod.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/delegation.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/naming.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/params.rs`
- [ ] `crates/elicitation_macros/src/trait_reflection/wrapper.rs`
- [ ] `crates/elicitation_macros/src/lib.rs` — `pub fn reflect_trait()`
- [ ] `crates/elicitation/tests/reflect_trait_basic_test.rs`
- [ ] `crates/elicitation/tests/reflect_trait_tools_test.rs`
- [ ] Apply to `elicit_clap` Select types (5 files)
- [ ] Update `THIRD_PARTY_SUPPORT_GUIDE.md` — add `#[reflect_trait]` as
      alternative to `#[reflect_methods]` for trait-family types
