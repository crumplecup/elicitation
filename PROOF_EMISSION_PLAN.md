# Proof Emission Redesign

## Problem

The existing `kani_proof()` / `verus_proof()` / `creusot_proof()` methods on the
`Elicitation` trait are useless:

- `#[cfg(kani/verus/creusot)]` gates mean they only exist under special toolchain builds
- Default implementations are `assert!(true, "...")` tautologies
- No primitive type overrides them — they all fall through to the tautology
- Recursive derive-macro composition terminates in tautologies, not real proofs
- The real harnesses in `elicitation_kani/verus/creusot` are completely disconnected

## Proposed Fix: TokenStream-Based Proof Emission

Replace the cfg-gated `() -> ()` stubs with `"emit"`-feature-gated methods that
return `proc_macro2::TokenStream` — making each type a **proof code generator**.

```
emit_kani_proof()     -> TokenStream   // complete #[kani::proof] harness
emit_verus_proof()    -> TokenStream   // complete Verus spec + proof
emit_creusot_proof()  -> TokenStream   // complete Creusot contract function

emit_kani_fragment(var_name: &str) -> TokenStream   // construction fragment for composites
```

At runtime, an agent calls an MCP tool like `emit_kani_proof("I8Positive")`, which:

1. Resolves the type by name via inventory
2. Calls `<I8Positive as Elicitation>::emit_kani_proof()`
3. Returns the proof source code for agent inspection or disk write

## Architecture

**Trait** (`crates/elicitation/src/traits.rs`):

- Add 4 new methods, all `#[cfg(feature = "emit")]`, all default to empty `TokenStream`
- Keep the old cfg-gated stubs (they are harmless under verifier builds)

**Primitive impls** — each type returns a quote!-generated harness matching the
existing harnesses in `elicitation_kani/src/`:

- `emit_kani_proof()` → the full `#[kani::proof] fn verify_*()` harness
- `emit_kani_fragment(var_name)` → construction code: `kani::any()` + `kani::assume()` + constructor
- Verus/Creusot equivalents following same pattern

**Derive macro** (`struct_impl.rs`, `enum_impl.rs`) generates:

```rust
fn emit_kani_proof() -> TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(<FieldType1 as Elicitation>::emit_kani_proof()); // sub-proofs
    ts.extend(<FieldType2 as Elicitation>::emit_kani_proof());
    // Composite harness using fragments:
    let frag1 = <FieldType1 as Elicitation>::emit_kani_fragment("field1");
    let frag2 = <FieldType2 as Elicitation>::emit_kani_fragment("field2");
    ts.extend(quote! {
        #[kani::proof]
        fn verify_StructName() { #frag1 #frag2 let _s = StructName { field1, field2 }; }
    });
    ts
}
```

**MCP ProofPlugin** (`crates/elicit_server/src/proof_plugin.rs`):

- Tools: `emit_kani_proof`, `emit_verus_proof`, `emit_creusot_proof`
- Input: `type_name: String`
- Output: proof source as string; optionally writes to file
- Registered types use inventory to dispatch by name

## Phases

### Phase 1 — Trait update + integer/bool/float primitives

1. Add 4 methods to `Elicitation` trait (gated "emit")
2. Write internal `impl_numeric_proof!` macro for repetitive patterns
3. Implement `emit_kani_proof` + `emit_kani_fragment` for all numeric types
   (I8Positive, I8NonNegative, I8NonZero, I8Range, U8*, I16*, U16*, bool variants,
    F32*/F64* floats)
4. Implement `emit_verus_proof` for numeric types
5. Implement `emit_creusot_proof` for numeric types

### Phase 2 — String, collection, network type primitives

- StringNonEmpty: uses fixed-size `[u8; N]` symbolic seed (Kani limitation on String)
- VecNonEmpty<T>: fragment generates `vec![T::from_fragment()]` (requires T: Elicitation)
- OptionSome<T>: trivial fragment using T's fragment
- Network/Path/UUID types: emit_kani_proof returns empty or stub (filesystem/RNG not symbolic)

### Phase 3 — Derive macro update

- Update `struct_impl.rs` and `enum_impl.rs`
- Replace `#[cfg(kani)] fn kani_proof()` generated code with `#[cfg(feature="emit")] fn emit_kani_proof()`
- Same for verus/creusot
- Use fragment assembly pattern shown above

### Phase 4 — ProofPlugin MCP tool

- `crates/elicit_server/src/proof_plugin.rs`
- Tools: `emit_kani_proof(type_name)`, `emit_verus_proof(type_name)`, `emit_creusot_proof(type_name)`
- Wire into `dispatch_step` and `list_tools`
- Add smoke tests

### Phase 5 — Remove old stubs (cleanup)

- Remove the `#[cfg(kani)]`, `#[cfg(verus)]`, `#[cfg(creusot)]` stubs from traits.rs
  (or keep for real inline verification if toolchains are installed)

## Key Technical Details

- `proc_macro2` + `quote` already available under `"emit"` feature in `elicitation` crate
- The derive macro emits RUNTIME code that calls `emit_kani_fragment()` — the quote!
  inside the generated function body runs at binary execution time, not at rustc time
- Verifier toolchains NOT needed to generate proof code; just needed to verify it
- The emitted proof file can be added to a workspace alongside user code and verified
  with `cargo kani` / Verus / Creusot independently
- Field names in composite harnesses are known at derive-macro expansion time

---

# Serde Integration Plan (prior — lower priority)

## Problem

The elicitation framework lives in the MCP world where every type already satisfies
`Elicitation + JsonSchema`. Adding `Serialize + Deserialize` to that surface unlocks
two things:

1. **Round-trip transport** — an elicited value can be serialized to JSON, passed through
   an MCP tool response, received by an agent, and fed back into a tool call without
   re-eliciting. This is the backbone of stateful multi-step workflows.

2. **JSON-bypass elicitation** — instead of prompting field-by-field, show an agent the
   JSON schema and ask for a JSON blob in one shot. The framework deserializes it. More
   efficient for complex types; the agent can see the whole shape at once.

## Why Now

Generic support was added to `#[derive(Elicit)]` and the `reflect_methods!` pipeline
during Phase 2. That removed the last blocker: we can now write
`impl<T: Elicitation + JsonSchema + Serialize + DeserializeOwned> ElicitJson for T`
and have it work for all generic container types (Vec, Option, HashMap, …) that already
impl Elicitation.

`serde` is already a core workspace dependency (not feature-gated). `schemars` is already
in the workspace. `ElicitToolOutput<T>` already derives all three. The primitives and
contract types are the gap — 23 of ~30 primitive modules lack `Serialize + Deserialize`.

## Approach

Four phases, each shippable independently:

### Phase 1 — Serde derives on elicitation's own types

Add `Serialize, Deserialize` to:

- All `primitives/` types (integers, floats, booleans, char, strings, collections,
  network, url, pathbuf, duration, tuples, unit structs)
- All contract types in `contracts.rs` and `verification/types/`
- Style enums generated by `#[derive(Elicit)]` (these are already JsonSchema)
- `ElicitError` and `ElicitResult` — needed for tool call round-trips

Feature gate: add a `serde` feature to `elicitation/Cargo.toml` that enables the
derives. Gate with `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
so downstream crates that don't need it aren't forced to pull in serde (even though
it's already a workspace dep, this keeps the API surface explicit).

### Phase 2 — `#[derive(Elicit)]` emits serde derives

When the `serde` feature is enabled, `elicitation_derive` emits
`#[derive(serde::Serialize, serde::Deserialize)]` on the struct/enum being derived.

This means any user type that writes `#[derive(Elicit)]` automatically becomes
serializable — zero extra work. The generated `Style` enum also gets serde derives.

The derive macro already handles generics; the generated serde impls inherit the same
`where T: Elicitation` bounds and add `T: Serialize + DeserializeOwned`.

### Phase 3 — `ElicitJson` trait — single-shot JSON elicitation

New trait in `elicitation` crate:

```rust
pub trait ElicitJson: Sized {
    async fn elicit_json<C: ElicitCommunicator>(communicator: &C) -> ElicitResult<Self>;
}
```

Blanket impl for `T: Elicitation + JsonSchema + DeserializeOwned`:

1. Call `schemars::schema_for!(T)` to get the JSON schema
2. Format it into a prompt: "Produce a JSON object matching this schema: <schema>"
3. Send to communicator, receive string response
4. Parse with `serde_json::from_str::<T>(&response)` — re-prompt on parse error
5. Return the deserialized value

This is strictly additive — `Elicitation::elicit()` (field-by-field) remains the
default. `ElicitJson::elicit_json()` is the fast path for agents that prefer JSON.

New MCP tool in `TypeSpecPlugin`: `elicit_from_json(type_name: &str, json: &str)`
that looks up the registered type by name (via inventory), deserializes, and returns
`ElicitToolOutput<serde_json::Value>` (since we can't return `dyn Any` through MCP).
The agent gets back the validated JSON to confirm the round-trip succeeded.

### Phase 4 — Serde support in `elicit_newtype!` and `reflect_methods!`

Newtype wrappers generated by `elicit_newtype!` get:

- `#[derive(Serialize, Deserialize)]` on the wrapper struct (delegates through Deref)
- MCP tool parameter structs generated by `reflect_methods!` gain serde derives

This means tool inputs and outputs are fully round-trippable, enabling workflows to
checkpoint and resume state through serialized MCP messages.

## Key Design Constraints

| Concern | Decision |
|---|---|
| Bound requirement | `T: Elicitation + JsonSchema` — everything in MCP world already satisfies this |
| Generics | Generic support already in derive macros; serde bounds propagate naturally |
| Feature gate | `serde` feature in `elicitation/Cargo.toml`; `serde_json` for Value elicitation |
| JSON schema source | `schemars::schema_for!(T)` — already a workspace dep |
| Parse failure | Re-prompt with the parse error message (same loop pattern as string parsing) |
| `ElicitJson` vs `Elicitation` | Additive — users choose the mode; field-by-field remains default |
| Inventory / dynamic dispatch | `elicit_from_json` MCP tool uses string name → registered type fn; works without `Any` |

## What This Unlocks

- Agents can elicit a complex `RequestConfig` in one JSON shot instead of 12 field prompts
- Workflow state can be checkpointed as JSON between tool calls
- Third-party types wrapped by `elicit_newtype!` become serializable transport values
- The `ElicitToolOutput<T>` pattern already used everywhere gains full round-trip fidelity
- Testing: elicited values can be snapshot-tested as JSON

## Out of Scope

- A blanket `impl Elicitation for T where T: DeserializeOwned` — too broad, breaks coherence
- `serde_yaml` / `serde_toml` modes — JSON only (MCP is JSON-native)
- Automatic schema validation beyond `serde_json::from_str` — schemars validation is a separate crate

---

## Phase 5 — `elicit_serde` and `elicit_serde_json` crates

Two separate crates following the `elicit_reqwest` pattern:

### `elicit_serde` — the generic Serialize/Deserialize interface

Wraps the *format-agnostic* serde interface. Tools operate on any registered type
that implements `Serialize + Deserialize`, using JSON as the MCP wire format.

```
crates/elicit_serde/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    └── serde_plugin.rs   # SerdePlugin
```

**MCP tools** (namespace `serde__`):

| Tool | Description |
|---|---|
| `serde__serialize(type_name, value_json)` | Round-trip: deserialize `value_json` into the named type, then serialize back to canonical JSON |
| `serde__deserialize(type_name, json)` | Parse `json` and validate it deserializes cleanly into `type_name` |
| `serde__round_trip_check(type_name, json)` | Returns `true` if the JSON survives a full `from_str → to_string → from_str` round-trip without data loss |
| `serde__list_types()` | Lists all registered types that impl `Serialize + Deserialize` |

These tools are format-agnostic at the serde level but JSON-backed at the MCP level —
matching serde's own design: abstract trait surface, concrete format as a backend.

The registry uses the same `inventory`-based type lookup already used by `TypeSpecPlugin`.

### `elicit_serde_json` — `serde_json` concrete types as MCP tools

Wraps `serde_json`'s concrete types using `elicit_newtype!` + `reflect_methods!`:

```
crates/elicit_serde_json/
├── Cargo.toml
├── README.md
└── src/
    ├── lib.rs
    ├── value.rs        # elicit_newtype!(JsonValue, serde_json::Value)
    ├── map.rs          # elicit_newtype!(JsonMap, serde_json::Map<String, Value>)
    ├── number.rs       # elicit_newtype!(JsonNumber, serde_json::Number)
    └── json_plugin.rs  # JsonPlugin — top-level serde_json functions as tools
```

**Wrapped types and tool counts:**

| Type | Key methods exposed |
|---|---|
| `serde_json::Value` | `is_null/bool/string/array/object`, `as_str/i64/f64/bool`, `pointer`, `to_string` (~13) |
| `serde_json::Map<String, Value>` | `get`, `contains_key`, `insert`, `remove`, `len`, `is_empty`, `keys`, `values` (~8) |
| `serde_json::Number` | `as_i64`, `as_u64`, `as_f64`, `is_i64`, `is_u64`, `is_f64` (~6) |

**`JsonPlugin` standalone tools** (namespace `serde_json__`):

| Tool | Description |
|---|---|
| `serde_json__from_str(json)` | Parse JSON string to `Value` |
| `serde_json__to_string(value)` | Serialize `Value` to compact JSON string |
| `serde_json__to_string_pretty(value)` | Serialize `Value` to pretty-printed JSON |
| `serde_json__merge(base, patch)` | RFC 7396 JSON merge patch |
| `serde_json__pointer(value, ptr)` | RFC 6901 JSON Pointer lookup |

### Together

`elicit_reqwest` fetches JSON payloads.
`elicit_serde_json` navigates and transforms them.
`elicit_serde` validates that typed round-trips hold.

Each layer is independently useful; together they form a complete JSON-over-HTTP
workflow toolkit.
