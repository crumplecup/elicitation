# elicit_server

Cross-crate workflow plugins for elicitation — server-side orchestration,
multi-crate composition, and code recovery.

## Why this crate

Individual elicitation shadow crates (`elicit_reqwest`, `elicit_url`,
`elicit_serde_json`, etc.) each expose their own MCP tools, but some
high-value workflows need types from two or more crates simultaneously.
`elicit_server` is the home for those compositions — it can depend on all
sibling crates without creating circular dependency chains.

## Plugins

| Plugin | Tools | Crates composed |
|---|---|---|
| [`SecureFetchPlugin`] | `secure_fetch`, `validated_api_call` | `elicit_url` + `elicit_reqwest` |
| [`FetchAndParsePlugin`] | `fetch_and_extract`, `fetch_and_validate` | `elicit_reqwest` + `elicit_serde_json` |
| [`EmitBinaryPlugin`] *(emit feature)* | `emit_binary` | all workflow crates |

## Cross-crate typestate chains

**SecureFetchPlugin**
```text
UnvalidatedUrl → UrlParsed → HttpsRequired → RequestCompleted ∧ StatusSuccess
```

**FetchAndParsePlugin**
```text
RequestCompleted → JsonParsed → PointerResolved
```

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

## Feature flags

| Feature | Enables |
|---|---|
| `emit` | [`EmitBinaryPlugin`] — recover agent tool compositions as standalone Rust binaries |

### Code recovery with `emit`

When an agent has built a verified workflow interactively, it can call
`emit_binary` to recover that session as a compilable `main.rs`:

```rust,no_run
// Tool: emit_binary
// args: {
//   "steps": [
//     { "tool": "fetch", "params": { "url": "https://api.example.com/data", "timeout_secs": 10 } },
//     { "tool": "parse_and_focus", "params": { "pointer": "/results" } }
//   ],
//   "output_dir": "/tmp/my_workflow",
//   "compile": true
// }
// → "/tmp/my_workflow/target/release/my_workflow"
```

The emitted binary preserves the full typestate ceremony, proof tokens, and
contract types from the original interactive session.
