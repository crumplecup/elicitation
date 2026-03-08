# elicit_server

> **Compositions that no single crate can hold**

The [elicitation](https://docs.rs/elicitation) library teaches agents to
think in types — constructing valid domain values through typed, contract-carrying
operations rather than filling in JSON forms. Each shadow crate (`elicit_reqwest`,
`elicit_url`, `elicit_serde_json`, …) exposes one slice of that language.

`elicit_server` is where those slices compose into something greater.

## What makes cross-crate composition special

Individual crates prove local invariants. Composition proves end-to-end
contracts that span multiple domains. The typestate proof chains here are not
validation — they are compiler-enforced proofs that an agent cannot skip a step:

**SecureFetchPlugin** (`elicit_url` + `elicit_reqwest`)

```text
UnvalidatedUrl → UrlParsed → HttpsRequired → RequestCompleted ∧ StatusSuccess
```

An agent cannot reach `RequestCompleted` without first passing through
`HttpsRequired`. The HTTPS contract is not a runtime check — it is structurally
unreachable to bypass.

**FetchAndParsePlugin** (`elicit_reqwest` + `elicit_serde_json`)

```text
RequestCompleted → JsonParsed → PointerResolved
```

The JSON pointer is only resolvable against a value that was provably fetched and
parsed. The agent builds the proof chain step by step.

## Code recovery (`emit` feature)

When an agent has assembled a verified workflow interactively, it can ask
`EmitBinaryPlugin` to recover that session as a standalone, compilable Rust binary
— with all typestate ceremony, proof tokens, and contract types intact.

```text
Agent builds workflow → calls emit_binary → gets a main.rs → cargo build → ships it
```

The output is not a script. It is idiomatic Rust that compiles and runs without
the MCP server. The agent's exploration becomes production code.

## Plugins

| Plugin | Tools | Feature |
|---|---|---|
| `SecureFetchPlugin` | `secure_fetch`, `validated_api_call` | default |
| `FetchAndParsePlugin` | `fetch_and_extract`, `fetch_and_validate` | default |
| `EmitBinaryPlugin` | `emit_binary` | `emit` |

## Usage

```rust,no_run
use elicitation::PluginRegistry;
use elicit_server::{SecureFetchPlugin, FetchAndParsePlugin};

#[tokio::main]
async fn main() {
    let registry = PluginRegistry::new()
        .register("secure_fetch", SecureFetchPlugin)
        .register("fetch_and_parse", FetchAndParsePlugin);
    // registry.serve(rmcp::transport::stdio()).await.unwrap();
}
```
