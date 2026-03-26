# elicit_std

MCP fragment tools for Rust standard library macros — tools that return
composable source code fragments rather than executing at runtime.

## Fragment tools vs runtime tools

Most `elicit_*` crates expose **runtime tools**: an agent calls `url__parse`
and gets back a parsed URL.  Rust macros work differently — they are resolved
at compile time and cannot be "executed" through an MCP boundary.

A **fragment tool** sidesteps this: the tool takes macro parameters as JSON,
validates them, and returns the equivalent Rust source fragment as a string.
Fragments are composable — you can pass a fragment returned by one tool as an
expression argument to another, building up arbitrarily complex expressions
before finally assembling them into a binary with `std__assemble`.

## Fragment pipeline

```text
1. std__env { var: "USER" }
        ↓  returns fragment
   env!("USER")

2. std__format { template: "Hello, {}!", args: ["env!(\"USER\")"] }
        ↓  returns fragment (env! nested inside format!)
   format!("Hello, {}!", env!("USER"))

3. Wrap in a statement: let msg = format!(...);

4. std__assemble { steps: ["let msg = format!(...);"] }
        ↓  terminal step — returns { main_rs, cargo_toml }
   compilable binary
```

## Tools

| Tool | Kind | Description |
|---|---|---|
| `std__format` | fragment | Emit a `format!(template, args…)` expression |
| `std__include_str` | fragment | Embed a file as `&'static str` at compile time |
| `std__env` | fragment | Read an environment variable at compile time |
| `std__concat` | fragment | Join string literals at compile time |
| `std__assemble` | **terminal** | Assemble statement fragments → `main.rs` + `Cargo.toml` |

## Usage

```rust,no_run
use elicit_std::StdMacrosPlugin;
use elicitation::PluginRegistry;

#[tokio::main]
async fn main() {
    let registry = PluginRegistry::new()
        .register("std", StdMacrosPlugin);
}
```

### Fragment tool examples

**`std__format`** — emit a format expression:

```json
{ "template": "Hello, {}! You have {} messages.", "args": ["name", "count"] }
```

Returns: `format!("Hello, {}! You have {} messages.", name, count)`

Pass a prior fragment as an arg:

```json
{ "template": "Hello, {}!", "args": ["env!(\"USER\")"] }
```

Returns: `format!("Hello, {}!", env!("USER"))`

**`std__include_str`** — embed a file:

```json
{ "path": "data/config.toml" }
```

Returns: `include_str!("data/config.toml")`

**`std__env`** — read a compile-time env var:

```json
{ "var": "DATABASE_URL", "error_message": "DATABASE_URL must be set" }
```

Returns: `env!("DATABASE_URL", "DATABASE_URL must be set")`

**`std__concat`** — join string parts:

```json
{ "parts": ["Hello", ", ", "world", "!"] }
```

Returns: `concat!("Hello", ", ", "world", "!")`

### Terminal tool: `std__assemble`

Takes an ordered list of statement-level fragment strings and assembles them
into a compilable binary using `BinaryScaffold`.

```json
{
  "steps": [
    "let greeting = format!(\"Hello, {}!\", env!(\"USER\"));",
    "println!(\"{}\", greeting);"
  ],
  "with_tracing": false,
  "package_name": "my-binary"
}
```

Returns a JSON object:

```json
{
  "main_rs": "/* pretty-printed main.rs */",
  "cargo_toml": "/* generated Cargo.toml */"
}
```

## Emit dispatch

`EmitEntry` registrations allow tools to be resolved globally by name:

```rust,no_run
use elicitation::emit_code::dispatch_emit_from;
use serde_json::json;

let emitter = dispatch_emit_from("format", "elicit_std",
    json!({ "template": "x = {}", "args": ["value"] })).unwrap();
let fragment = emitter.emit_code().to_string();
// → `format ! ("x = {}", value)`
```

Note: `assemble` is not registered in `EmitEntry` — it is a terminal tool,
not a composable fragment.

## Design pattern

This crate is the reference implementation for macro fragment tools.

**Fragment tool:**

1. **Params struct** — `Deserialize + JsonSchema`, no `Elicit` required
2. **`impl EmitCode`** — `emit_code()` returns a `TokenStream` via `quote!`
3. **`inventory::submit!(EmitEntry { … })`** — registers for global dispatch
4. **`#[elicit_tool]` handler** — `p.emit_code().to_string()` → `Content::text`

**Terminal tool (`std__assemble`):**

1. **Params struct** — `steps: Vec<String>` (fragment strings from prior calls)
2. **`assemble()` method** — wraps each string in `RawFragment`, calls
   `BinaryScaffold::new(...).to_source()` + `to_cargo_toml()`
3. **`#[elicit_tool]` handler** — returns JSON `{ main_rs, cargo_toml }`
4. No `EmitEntry` registration (terminal, not composable)

## Feature flags

The `elicitation` dependency is pulled in with `features = ["emit"]`.
