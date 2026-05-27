# `unexpected_cfgs` in Proc Macro Output: A White Paper

## The Problem

Elicitation's proc macros (`#[formal_method]`, `#[derive(Elicit)]`) emit code that contains
`#[cfg(kani)]` and `#[cfg(creusot)]` cfg conditions. These flags are meaningful inside the
elicitation workspace (which declares them), but they are **unknown** to downstream user crates
like valinoreth. Rust's `unexpected_cfgs` lint fires on every cfg condition that is not declared
in the crate's own `Cargo.toml` or via `rustc --check-cfg`.

The result: a downstream user annotates a function with `#[formal_method(...)]` and immediately
receives 2–4 `unexpected_cfg` warnings per annotation site, for `kani` and `creusot` flags they
have never heard of. This is the library pissing on the user and telling them to bring a towel.

The fix belongs entirely in the macro — zero burden on the user.

---

## Why `#[allow(unexpected_cfgs)]` Is Non-Trivial

The intuitive fix — just slap `#[allow(unexpected_cfgs)]` on or near the cfg item — does not
work from proc macro output. This is not a Rust bug; it is a deliberate property of how the lint
checker walks the attribute stack.

The lint is fired **at the expansion site** of the cfg condition, not at the top of the file.
When an attribute macro transforms a function and the resulting token stream contains
`#[cfg(kani)]`, the allow must be on an **enclosing item** in the expanded output — not a
sibling attribute, not a preceding statement.

---

## The Gallery

A test gallery (`crates/elicitation_derive/tests/cfg_allow_gallery_test.rs`) was built to
empirically determine which allow placements suppress the lint. Each case emits `cfg(foo)` — a
cfg name unknown to the test crate — mimicking exactly what `cfg(kani)` looks like downstream.

| Case | Pattern | Warning? |
|------|---------|----------|
| A | No allow at all | **YES** |
| B | `#[allow]` as sibling attr on the **same** item as `#[cfg(foo)]` | **YES** |
| C | `#[allow]` on a separate preceding `const` item | **YES** |
| D | `#[allow]` on an outer `const _: () = { ... }` wrapping the cfg item | **NO** |
| E | `cfg_attr(foo, ...)` with no allow | **YES** |
| F | `#[allow]` + `cfg_attr` as siblings on the same item | **YES** |
| G | Attr macro: push allow then cfg_attr at end of `func.attrs` | **YES** |
| H | Attr macro: insert allow at position 0, cfg_attr at end | **YES** |
| I | Attr macro: emit preceding allow const, then modified fn | **YES** |
| J | Attr macro: wrap entire fn in `#[allow] const _: () = { fn }` | **NO** *(function inaccessible)* |
| K | Attr macro: `#[allow] mod { fn }` + `pub use` outside | **NO** |
| L | Attr macro: outer `#[allow] #[cfg(foo)]` on bare fn + mod wrapper | **YES** |
| M | Attr macro: mod wrapper only, no outer cfg gates | **NO** |

### Key Findings

**What does NOT work:**

- `#[allow(unexpected_cfgs)] #[cfg(kani)] fn foo()` — allow as sibling to cfg (Case B). This is
  the most tempting and most wrong approach. The lint fires regardless.

- `#[allow(unexpected_cfgs)] const _GUARD: () = (); #[cfg(kani)] fn foo()` — allow on an
  adjacent preceding item (Case C). Completely ineffective.

- Pushing `#[allow]` onto `func.attrs` from inside an attribute macro (Cases G, H) — the macro
  expansion context prevents sibling allows from working even when placed first.

- Any pattern where a `#[cfg]` gate appears **outside** the wrapping allow item (Case L) — the
  outer cfg itself triggers the lint before the allow scope begins.

**What DOES work:**

- `#[allow(unexpected_cfgs)] const _: () = { items_with_cfg };` — the allow on a `const` block
  propagates to everything inside it (Case D). Works for derive macros emitting new `impl` blocks.
  **Limitation**: items inside a `const` block are not exportable; cannot be referenced from
  outside. Use for `impl` blocks and inline harnesses only.

- `#[allow(unexpected_cfgs)] mod _name { use super::*; items_with_cfg }` + `pub use _name::item`
  outside — the allow on the `mod` block propagates to all items at any depth inside it, including
  cfg conditions on items AND cfg conditions inside function bodies (Case K, M). The `pub use`
  outside the mod re-exports the item unconditionally. This is the correct pattern for attribute
  macros that transform functions.

---

## The Two Canonical Patterns

### Pattern 1: `const _: () = {}` — For Derive Macros

Use when the macro emits new `impl` blocks or other items that do not need to be exported:

```rust
// In proc macro output:
#[allow(unexpected_cfgs)]
const _: () = {
    #[cfg(kani)]
    #[::kani::proof]
    fn kani_harness() { /* ... */ }

    #[cfg(creusot)]
    impl SomeSpec for SomeType { /* ... */ }
};
```

Used in: `enum_impl.rs`, `struct_impl.rs` for derive-emitted impl blocks and inline harnesses.

### Pattern 2: `#[allow] mod { fn } + pub use` — For Attribute Macros

Use when the macro transforms an existing function that must remain accessible at its original
path:

```rust
// In proc macro output (formal_method):
#[allow(unexpected_cfgs)]
mod _formal_method_compat_my_transition {
    use super::*;

    pub fn my_transition(/* ... */) -> /* ... */ {
        // function body — may contain #[cfg(not(kani))] let _span = ...
        // the allow on the mod propagates here too
    }

    // Both cfg branches of contracted fn live inside the mod:
    #[cfg(kani)]
    #[::kani::requires(invariant(...))]
    #[::kani::ensures(|result| invariant(...))]
    pub fn my_transition_kani_contracted(/* ... */) -> /* ... */ { /* ... */ }

    #[cfg(not(kani))]
    pub fn my_transition_kani_contracted(/* ... */) -> /* ... */ { /* ... */ }
}
// Re-exports — no cfg gate needed because both variants exist above:
pub use _formal_method_compat_my_transition::my_transition;
pub use _formal_method_compat_my_transition::my_transition_kani_contracted;
```

**Why both cfg variants of the contracted fn are needed**: If only the `#[cfg(kani)]` variant
existed inside the mod, the `pub use` of `my_transition_kani_contracted` would fail to compile
in a non-kani build (item doesn't exist). Having both variants means the item always exists and
the `pub use` needs no cfg guard — zero cfg tokens escape to user module scope.

---

## The Tracing Span Gotcha

The `#[formal_method]` macro conditionally injects a tracing span to avoid Kani timeout:

```rust
// Inside compat_func body — lives inside the allow mod:
#[cfg(not(kani))]
let _tracing_span = tracing::info_span!("...").entered();
```

This `#[cfg]` is inside a function *body*, not on an item. The gallery confirmed that the
`#[allow]` on the enclosing `mod` propagates to cfg conditions inside function bodies as well
as to item-level cfg attributes. The tracing span statement is safe inside the compat mod.

**If the tracing span is moved outside the compat mod**, it becomes a naked cfg inside the
expanded code in the user crate → warning returns. Never hoist it out.

---

## How `needs_compat_mod` Is Computed

In `formal_method.rs`, the compat mod is only emitted when the expanded output will contain cfg
tokens:

```rust
let needs_compat_mod = has_instrument || !kani_harness.is_empty();
```

- `has_instrument`: true when `#[instrument]` would have been applied — injects the
  `#[cfg(not(kani))]` span statement into the function body.
- `!kani_harness.is_empty()`: true when the function has a `contracts = [...]` argument —
  generates the contracted fn pair with cfg(kani)/cfg(not(kani)).

If neither is true (bare `#[formal_method]` with no contracts and no instrument), the function is
emitted directly with no wrapping — no cfg tokens, no warning, no overhead.

---

## Anti-Pattern Reference: What NOT To Do

Future agents: if you are tempted to do any of the following, stop.

### ❌ Sibling allow on cfg item

```rust
// DOES NOT SUPPRESS — Case B / L
quote! {
    #[allow(unexpected_cfgs)]
    #[cfg(kani)]
    fn my_fn_kani_contracted(...) { ... }
}
```

### ❌ Adjacent allow const before cfg item

```rust
// DOES NOT SUPPRESS — Case C / I
quote! {
    #[allow(unexpected_cfgs)]
    const _: () = ();
    #[cfg(kani)]
    fn my_fn_kani_contracted(...) { ... }
}
```

### ❌ Outer cfg gate around the allow mod

```rust
// DOES NOT SUPPRESS — Case L
// The outer #[cfg(kani)] fires BEFORE the allow scope is entered
quote! {
    #[cfg(kani)]
    #[allow(unexpected_cfgs)]
    mod _compat { ... }
}
```

### ❌ Allow on the function that gets the cfg attribute pushed onto it

```rust
// From an attribute macro — DOES NOT SUPPRESS — Cases G / H
fn expand(...) -> TokenStream {
    func.attrs.insert(0, parse_quote!(#[allow(unexpected_cfgs)]));
    func.attrs.push(parse_quote!(#[cfg(not(kani))]));
    quote! { #func }
}
```

### ❌ `#[allow]` in `elicit_server` Cargo.toml / source

This only hides the warnings in `elicit_server`. Every other downstream crate that depends on
elicitation still gets them. Suppress at the macro, not at any one consumer.

---

## Diagnosing a Recurrence

If `unexpected_cfgs` warnings return in a downstream crate after a macro change:

1. **Identify the call site** — which macro annotation is the source?  
   Look at the warning's file/line: it will be in the user crate at a `#[formal_method(...)]`
   or `#[derive(Elicit)]` annotation.

2. **Expand the macro** — `cargo expand -p elicit_server` (or the relevant crate) and search for
   naked `#[cfg(kani)]` or `#[cfg(creusot)]` tokens outside any `#[allow(unexpected_cfgs)]`
   enclosing block.

3. **Check `needs_compat_mod`** in `formal_method.rs` — did a refactor hardcode it to `false`?
   Did someone add a new cfg-bearing path that bypasses the compat mod emission?

4. **Check the gallery** — `cargo test -p elicitation_derive --test cfg_allow_gallery_test`.
   If the gallery test fails or new cases are needed, extend it before changing the macros.

5. **Test in valinoreth** — `cd /home/erik/repos/valinoreth && cargo check 2>&1 | tee /tmp/v.txt`
   then `grep unexpected_cfg /tmp/v.txt`. Zero results = fixed. valinoreth does NOT add any
   allows or check-cfg declarations; it is the canary.

---

## Timeline of How This Went Wrong

This section exists so future agents do not repeat the same 8-hour detour.

**Original state (V9 era):** `#[formal_method]` injected the tracing span unconditionally (no
`#[cfg]`), and `needs_compat_mod` was computed correctly. Zero cfg tokens leaked.

**FV timeout fix:** Kani was timing out because the tracing prologue/epilogue (emitted by
`#[instrument]`) was inlined into the DFCC harness with no contracts. The fix added
`#[cfg(not(kani))] let _span = ...` inside the function body to skip tracing under Kani. This
cfg token was placed inside the compat mod, so it was correctly suppressed.

**Regression:** During the same FV fix session, `needs_compat_mod` was hardcoded to `false`
(to simplify a different bug), and the contracted fn pair was emitted with a "Case B" sibling
allow pattern that does not suppress. Both cfg tokens escaped the compat mod. Valinoreth began
reporting 28 warnings.

**Fix (this session, commit `d7ccaba0`):**
- Removed `let needs_compat_mod = false`
- `needs_compat_mod` recomputed as `has_instrument || !kani_harness.is_empty()`
- Contracted fn pair rebuilt with both cfg branches inside the compat mod
- `pub use` outside the mod is unconditional (no cfg gate)
- valinoreth: zero `unexpected_cfg` warnings

---

## The Invariant to Maintain

> **No `#[cfg(kani)]` or `#[cfg(creusot)]` token may appear in proc macro output outside an
> `#[allow(unexpected_cfgs)]` enclosing `mod` or `const _: () = {}` block.**

Every time a macro change adds a new cfg-conditional item or statement to the generated output,
ask: is this token inside an enclosing allow wrapper? If not, it will leak into every downstream
crate. Add it to the compat mod or create a new const wrapper.

The gallery test file is the source of truth for which wrapper patterns actually work. Run it.
Read it. Extend it if you are unsure about a new pattern before shipping.
