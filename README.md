# Elicitation

> **A Rust dialect for programs that are correct by construction.**

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

Elicitation is a framework for writing Rust programs where **program invariants live in the type system and are formally verified**. The central architectural tool is the **Verified State Machine** (VSM): a state enum whose transitions are pure functions that carry proof tokens from precondition to postcondition. Layer on top an MCP interface so that AI agents and humans alike can elicit structured values from those types — and a library of shadow crates that give agents a verified vocabulary for the ecosystem.

The result is a recognisable dialect: every module has a state enum, a machine struct, a consistency predicate, and a set of `#[formal_method]` transition functions. The compiler enforces transition order; the proof toolchain verifies invariants; `elicitation prove` auto-generates the proof harnesses.

---

## Verified State Machines

The VSM pattern is three declarations and one attribute:

```rust
use elicitation::{
    Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine,
    contracts::ProvableFrom, formal_method,
};

// 1. State enum — KaniCompose + KaniVariantState wire Kani's symbolic engine.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema,
         Elicit, KaniCompose, KaniVariantState)]
pub enum DbState {
    #[default]
    Disconnected,
    Connecting { url: String },
    Connected   { pool: DbPool },
    Error       { message: String },
}

// 2. Machine tag — zero-sized, carries the VSM identity.
#[derive(VerifiedStateMachine)]
#[vsm(state = DbState, transitions = [connect, disconnect, connection_error])]
pub struct DbMachine;

// 3. Consistency predicate — the invariant Kani, Verus and Creusot all verify.
#[derive(Prop)]
#[prop(credential = DbCredential,
       kani_invariant_fn   = "db_consistent",
       verus_invariant_fn  = "db_consistent",
       creusot_invariant_fn = "db_consistent")]
pub struct DbConsistent;

impl ProvableFrom<DbCredential> for DbConsistent {}

#[cfg(kani)]
pub fn db_consistent(s: &DbState) -> bool {
    matches!(s, DbState::Connecting { url } if !url.is_empty()) || true
}

// 4. Transitions annotated with #[formal_method] — proofs auto-generated.
#[formal_method(contracts = [DbConsistent])]
pub fn connect(
    _state: DbState,
    proof:  Established<DbConsistent>,
    url:    String,
) -> (DbState, Established<DbConsistent>) {
    (DbState::Connecting { url }, proof)
}

#[formal_method(contracts = [DbConsistent])]
pub fn disconnect(
    _state: DbState,
    proof:  Established<DbConsistent>,
) -> (DbState, Established<DbConsistent>) {
    (DbState::Disconnected, proof)
}
```

`Established<P>` is a zero-sized proof token: it cannot be constructed except by code that has done the real work. Functions that need a precondition take it as a parameter; functions that establish a postcondition return it. The compiler enforces order; the verifiers check the invariant.

### Auto-generated proofs

`elicitation prove` (or `just prove`) scans all VSMs in a crate path and emits Kani, Verus, and Creusot harnesses into the proof crates:

```bash
just prove            # all three verifiers
just prove --kani     # Kani only  (388 passing harnesses in this workspace)
just prove --verus    # Verus only (158 passing proofs)
just prove --creusot  # Creusot    (22,837 valid goals across 19 modules)
```

No hand-written proof files needed. `#[formal_method]` annotates what the verifier needs to know; the generator turns those annotations into harnesses.

### Contracts and composition

`Prop`, `Established<P>`, and `And<P,Q>` compose proofs the same way types compose structs:

```rust
let (pool, db_proof)    = connect(&url, init_proof, url.clone());
let (_, query_proof)    = run_query(&pool, db_proof);
let combined: Established<And<DbConsistent, QueryExecuted>> =
    both(db_proof, query_proof);
```

`ProvableFrom<C>` lets you derive a proposition from a credential — useful when a proof follows from satisfying a trait bound rather than a runtime check.

---

## Elicitation — MCP-Native Types

Any type that the VSM produces or consumes can cross the MCP boundary once it derives `Elicit`. The derive generates the async `elicit()` method (Survey for structs, Select for enums), JSON schema, and proof scaffolding:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Environment { Production, Staging, Development }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DeployConfig {
    #[prompt("Which environment?")]
    pub environment: Environment,
    #[prompt("Replicas (1–16):")]
    #[spec_requires("replicas >= 1 && replicas <= 16")]
    pub replicas:    u8,
    #[prompt("Container image URI:")]
    pub image:       String,
}
```

Register types as MCP tools with `elicit_tools!` inside any `#[tool_router]` impl:

```rust
#[tool_router]
impl MyServer {
    elicitation::elicit_tools! { Environment, DeployConfig }
}
```

---

## Shadow Crates — Verified Ecosystem Vocabulary

Shadow crates (`elicit_*`) expose third-party libraries as MCP-ready, formally verified tool sets. An agent with access to a shadow crate can reason about and compose the library without writing Rust:

| Crate | Library | What it covers |
|---|---|---|
| `elicit_reqwest` | `reqwest` | HTTP: fetch, post, auth, pagination, workflow |
| `elicit_sqlx`    | `sqlx`    | DB: connect, execute, fetch, transactions |
| `elicit_tokio`   | `tokio`   | Async: sleep, timeout, semaphores, file I/O |
| `elicit_clap`    | `clap`    | CLI: `Arg`, `Command`, `ValueEnum` |
| `elicit_chrono` / `elicit_jiff` / `elicit_time` | datetime crates | Datetimes, durations, timezones |
| `elicit_url` / `elicit_regex` / `elicit_uuid`   | string crates  | URL, pattern, UUID |
| `elicit_serde_json` | `serde_json` | JSON values, maps, dynamic typing |
| `elicit_std`     | `std`     | Selected stdlib types |

Add them to your server via `PluginRegistry`:

```rust
PluginRegistry::new()
    .register_flat(MyServer::new())
    .register("http", elicit_reqwest::WorkflowPlugin::default_client())
    .register("db",   elicit_sqlx::SqlxWorkflowPlugin::default())
    .serve(rmcp::transport::stdio())
    .await?;
```

---

## Getting Started

```toml
[dependencies]
elicitation = "0.11"
rmcp        = "1"
schemars    = "0.8"
serde       = { version = "1", features = ["derive"] }
tokio       = { version = "1", features = ["full"] }
```

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rmcp::service::serve_server(MyServer::new(), rmcp::transport::stdio()).await?;
    Ok(())
}
```

---

## Workspace Crate Map

| Crate | Role |
|---|---|
| `elicitation`          | Core: traits, contracts, proof helpers, MCP plumbing |
| `elicitation_derive`   | Proc macros: `#[derive(Elicit)]`, `#[derive(VerifiedStateMachine)]`, `#[derive(Prop)]`, `#[formal_method]`, `#[reflect_methods]` |
| `elicit_proofs`        | Generated Kani, Verus, and Creusot harnesses |
| `elicitation_kani`     | Hand-written Kani gallery and specs |
| `elicitation_creusot`  | Hand-written Creusot gallery and specs |
| `elicitation_verus`    | Hand-written Verus gallery and specs |
| `elicitation_rand`     | Randomised generation for property testing |
| `elicit_server`        | Reference MCP server (archive module with four VSMs) |
| `elicit_*`             | Shadow crates — see table above |

---

## Further Reading

| Document | Topic |
|---|---|
| [`SHADOW_CRATE_MOTIVATION.md`](SHADOW_CRATE_MOTIVATION.md)     | The inversion thesis: why shadow crates exist |
| [`THIRD_PARTY_SUPPORT_GUIDE.md`](THIRD_PARTY_SUPPORT_GUIDE.md) | How to add a shadow crate end-to-end |
| [`FORMAL_VERIFICATION_LEGOS.md`](FORMAL_VERIFICATION_LEGOS.md) | Compositional proof strategy |
| [`CREUSOT_GUIDE.md`](CREUSOT_GUIDE.md)                         | Creusot annotation patterns |
| [`crates/elicit_clap/`](crates/elicit_clap/)                   | Canonical reference shadow crate |
