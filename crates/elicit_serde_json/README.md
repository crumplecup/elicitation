# elicit\_serde\_json

MCP tool transport for [`serde_json`](https://docs.rs/serde_json) types — expose
`Value`, `Number`, and JSON navigation as MCP tools for agent workflows.

## What it does

`elicit_serde_json` has two layers:

1. **Atomic tools** — `JsonValue` and `JsonNumber` newtypes with every method
   exposed as an MCP tool via `#[reflect_methods]`
2. **Verified workflows** — `JsonWorkflowPlugin` composes primitives into
   multi-step operations with proof-carrying invariants from the elicitation
   contract system

## Atomic newtypes

```rust,ignore
use elicit_serde_json::{JsonValue, JsonNumber};
use serde_json::json;

let v: JsonValue = json!({"name": "Alice", "age": 30}).into();

assert_eq!(v.type_name(), "object");
assert_eq!(v.len(), Some(2));

// JSON Pointer navigation
let name = v.pointer("/name".to_string());
let json_str = v.to_json_string();
```

## Verified workflows

`JsonWorkflowPlugin` provides phrase-level compositions using the same
typestate + contract pattern as `strictly_tictactoe`:

```text
RawJson ──parse()──→ ParsedJson ──assert_object()──→ ObjectJson
                          │                                │
                     focus(ptr)                 validate_required()
                          │                                │
                     FocusedJson                    ObjectJson (validated)
                          │
                     .extract()  ← returns Value, NOT Option<Value>
```

**Key invariant**: `FocusedJson::extract()` returns `Value`, never `Option<Value>`.
This mirrors `GameFinished::outcome()` returning `Outcome` (not `Option<Outcome>`):
the type *carries the proof* that the pointer resolved.

### Propositions

| Proposition | Meaning |
|---|---|
| `JsonParsed` | The input string is syntactically valid JSON |
| `IsObject` | The JSON value is an object (`Map<String, Value>`) |
| `PointerResolved` | An RFC 6901 pointer path exists in the document |
| `RequiredKeysPresent` | All specified required keys exist in the object |
| `UpdateApplied` | A pointer-targeted write was successfully completed |

### Workflow tools

| Tool | Contract established |
|---|---|
| `parse_and_focus` | `JsonParsed ∧ PointerResolved` |
| `validate_object` | `JsonParsed ∧ IsObject ∧ RequiredKeysPresent` |
| `safe_merge` | `IsObject(base) ∧ IsObject(patch) ⟹ IsObject(result)` |
| `pointer_update` | `JsonParsed ∧ PointerResolved ⟹ UpdateApplied` |
| `field_chain` | `∀ key ∈ path. PointerResolved(root, key)` |

### `safe_merge` — the two-precondition pattern

Merging two JSON objects requires proving BOTH operands are objects, preventing
silent failures when an agent passes an array or null as the base:

```text
parse_as_object(base)  → (ObjectJson, Established<ParsedObject>)
parse_as_object(patch) → (ObjectJson, Established<ParsedObject>)
                                    │
                              both(base_proof, patch_proof)
                                    │
                             execute merge — type-enforced
```

This directly mirrors tic-tac-toe's `validate_square_empty ∧ validate_player_turn → both(…)`.

### Select-pattern enums

| Enum | Variants |
|---|---|
| `ObjectMergeMode` | `merge_patch` (RFC 7396, nulls delete), `deep_merge` (recursive overwrite) |
| `MissingKeyPolicy` | `error` (strict), `create_path` (permissive) |

## Generated tools — atomic layer

### `JsonValue`

| Method | Description |
|---|---|
| `is_null` / `is_boolean` / `is_number` / `is_string` / `is_array` / `is_object` | Type checks |
| `as_bool` / `as_str` / `as_i64` / `as_u64` / `as_f64` | Value extraction |
| `len` / `is_empty` | Size (array/object) |
| `type_name` | Type string: `"null"`, `"bool"`, `"number"`, `"string"`, `"array"`, `"object"` |
| `pointer` | RFC 6901 JSON Pointer lookup |
| `as_number` | Returns `JsonNumber` |
| `to_json_string` / `to_json_string_pretty` | Serialization |

### `JsonNumber`

| Method | Description |
|---|---|
| `is_i64` / `is_u64` / `is_f64` | Numeric type checks |
| `as_i64` / `as_u64` / `as_f64` | Value extraction |
| `to_json_string` | JSON representation |

## Relationship to other crates

| Crate | Role |
|---|---|
| `elicit_reqwest` | Fetches HTTP responses, returns JSON payloads |
| `elicit_serde_json` | Navigates, validates, and transforms those payloads |
| `elicit_serde` | Validates typed round-trips for registered types |

## License

Apache-2.0 OR MIT
