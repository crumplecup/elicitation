# elicit_std

MCP tools for Rust standard library macros — **emit-only tools** that return
source code fragments instead of executing at runtime.

## What is an emit-only tool?

Most `elicit_*` crates expose runtime tools: an agent calls `url__parse` and
gets back a parsed URL.  Rust macros work differently — they are resolved at
compile time and cannot be "executed" through an MCP boundary.

An **emit-only tool** sidesteps this: the tool takes macro parameters as JSON,
validates them, and returns the equivalent Rust source fragment.  The agent
assembles these fragments into a program; the macro runs when *that* program is
compiled.

```
Agent calls std__format { template: "Hello, {}!", args: ["name"] }
                 ↓
Tool returns: format!("Hello, {}!", name)
                 ↓
Fragment is embedded in a BinaryScaffold
                 ↓
Compiled binary runs the macro
```

## Tools

| Tool | Wraps | Description |
|---|---|---|
| `std__format` | `format!` | Emit a `format!(template, args…)` expression |
| `std__include_str` | `include_str!` | Embed a file as `&'static str` at compile time |
| `std__env` | `env!` | Read an environment variable at compile time |
| `std__concat` | `concat!` | Join string literals at compile time |

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

### Tool examples

**`std__format`** — emit a format expression:
```json
{ "template": "Hello, {}! You have {} messages.", "args": ["name", "count"] }
```
Returns: `format!("Hello, {}! You have {} messages.", name, count)`

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

## Emit dispatch

The `EmitEntry` inventory integrations enable the global `dispatch_emit_from`
to resolve tools by name at program assembly time:

```rust,no_run
use elicitation::emit_code::dispatch_emit_from;
use serde_json::json;

let emitter = dispatch_emit_from("format", "elicit_std",
    json!({ "template": "x = {}", "args": ["value"] })).unwrap();
let fragment = emitter.emit_code().to_string();
// → `format!("x = {}", value)`
```

## Design pattern

This crate is the reference implementation for emit-only macro tools.  The
pattern is:

1. **Params struct** — `Deserialize + JsonSchema`, no `Elicit` required
2. **`impl EmitCode`** — `emit_code()` returns a `TokenStream` built with
   `quote!`; `crate_deps()` returns `vec![]` for std macros
3. **`inventory::submit!(EmitEntry { … })`** — registers the tool in the global
   emit dispatch table
4. **`#[elicit_tool]` handler** — calls `p.emit_code().to_string()` and returns
   it as `Content::text(…)`; no runtime execution

The key insight: the handler body is just `p.emit_code().to_string()`.  There
is no "runtime execution" path to expose.  The tool *is* the emission step.

## Feature flags

| Feature | Description |
|---|---|
| *(none)* | All tools enabled by default |

The `elicitation` dependency is pulled in with `features = ["emit"]`.
