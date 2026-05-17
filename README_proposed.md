# Elicitation

> A Rust dialect for programs correct by construction.

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

## The Story

**Step 1 — Types carry their own proofs.** When a type implements `Elicitation` it gains
`kani_proof()`, `verus_proof()`, and `creusot_proof()`. `#[derive(Elicit)]` composes those proofs
from field types automatically — add a field, get its proof for free.

**Step 2 — State machines are just types.** `Established<P>` is a zero-sized proof token that
proposition `P` holds. Functions that require prior work take it as a parameter; functions that
perform work return it. The compiler enforces transition order.

**Step 3 — Methods become correct by construction.** Program invariants live in the type system.
The agent's role is proof search: find the sequence of verified operations that reaches the goal.
Each step is an auditable tool call with a known contract.

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│  Your domain types                                       │
│  #[derive(Elicit)]  →  agent-navigable, MCP-crossing     │
├─────────────────────────────────────────────────────────┤
│  Shadow crates  (elicit_*)                               │
│  Verified vocabularies for third-party libraries         │
│  Types + Methods + Traits = the agent's dictionary       │
├─────────────────────────────────────────────────────────┤
│  Contracts  (Prop / Established<P> / And<P,Q>)           │
│  Postconditions chain into preconditions                 │
│  Workflow correctness enforced at the type level         │
└─────────────────────────────────────────────────────────┘
```

All three layers are formally verified by Kani, Creusot, and Verus.

---

## Layer 1: Your Types Become Agent-Native

`#[derive(Elicit)]` generates the MCP round-trip, JSON schema, proof methods, and the
`elicit()` async method from your struct or enum:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Environment { Production, Staging, Development }

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DeployConfig {
    #[prompt("Which environment?")]
    pub environment: Environment,

    #[prompt("Number of replicas (1–16):")]
    #[spec_requires("replicas >= 1 && replicas <= 16")]
    pub replicas: u8,

    #[skip]
    pub _internal_id: u64,    // not elicited
}
```

### Style System

Override prompts per audience without changing the type. The derive generates a style enum:

```rust
#[prompt("environment")]
#[prompt("Which environment should we deploy to?", style = "human")]
#[prompt("environment: production|staging|development", style = "agent")]
pub environment: Environment,
```

```rust
// Swap style at the call site — no changes to the type
let config = DeployConfig::elicit(
    &client.with_style(DeployConfigElicitStyle::Human)
).await?;
```

### Action traits

`#[derive(Elicit)]` assigns the right interaction paradigm automatically:

| Trait | Paradigm | Used for |
|---|---|---|
| `Select` | Choose from a finite set | Enums, categorical fields |
| `Affirm` | Binary yes/no | `bool` fields, guard steps |
| `Survey` | Sequential multi-field | Structs, configuration objects |

### Generators

`Generator<Target>` produces values from any source. `elicitation_rand` adds
`#[rand(...)]` field attributes (`bounded(L, H)`, `positive`, `nonzero`, `even`, `odd`,
`and(A,B)`, `or(A,B)`) and derives a seeded `random_generator(seed: u64)` for
reproducible testing — with its own Kani suite.

---

## Layer 2: Shadow Crates — The Agent's Dictionary

A shadow crate (`elicit_*`) is a typed vocabulary for a third-party library:

| Layer | What it provides | Mechanism |
|---|---|---|
| **Types** | `serde` + `JsonSchema` wrappers | Newtypes |
| **Methods** | Instance methods as MCP tools | `#[reflect_methods]` |
| **Traits** | Third-party trait methods as factories | `#[reflect_trait]` |

```rust
#[reflect_methods]
pub struct ElicitArg(pub clap::Arg);
// Generates: arg__get_long, arg__get_short, arg__get_help, ... as MCP tools
```

For compile-time macros (e.g. `sqlx::query!`), fragment tools emit verified Rust source via
`EmitCode`; the macro runs in the consumer binary where a live connection is available.

### Available shadow crates

| Crate | Library |
|---|---|
| `elicit_reqwest` | HTTP client |
| `elicit_sqlx` | Database |
| `elicit_tokio` | Async runtime |
| `elicit_clap` | CLI |
| `elicit_chrono` / `elicit_jiff` / `elicit_time` | Datetimes |
| `elicit_url` / `elicit_regex` / `elicit_uuid` | String types |
| `elicit_serde_json` | JSON values |
| `elicit_std` | Stdlib types |

---

## Layer 3: Contracts — Workflow Correctness at the Type Level

```rust
pub struct DbConnected;
impl Prop for DbConnected {}

fn connect(url: &str) -> (Pool, Established<DbConnected>) { ... }
fn fetch_rows(sql: &str, _pre: Established<DbConnected>) -> Vec<Row> { ... }

// Compose proofs
let both: Established<And<DbConnected, QueryExecuted>> = both(db_proof, query_proof);
```

Shadow crates ship their own propositions: `elicit_sqlx` has `DbConnected`, `QueryExecuted`,
`TransactionOpen`; `elicit_reqwest` has `UrlValid`, `RequestCompleted`,
`FetchSucceeded = And<UrlValid, And<RequestCompleted, StatusSuccess>>`.

The `Tool` trait formalises composable workflow steps with typed `Pre`/`Post` constraints.
Sequential (`then`) and parallel (`both_tools`) composition enforce that each step's postcondition
satisfies the next step's precondition.

---

## Verified State Machines

`#[derive(VerifiedStateMachine)]` combines contracts with formal verification. Four elements:

```rust
// 1. State enum
#[derive(Debug, Clone, VerifiedStateMachine)]
pub enum DbState { Disconnected, Connected, QueryReady, Closed }

// 2. Consistency predicate
pub fn db_consistent(state: &DbState, pool: &Option<Pool>) -> bool {
    match state {
        DbState::Connected | DbState::QueryReady => pool.is_some(),
        _ => pool.is_none(),
    }
}

// 3. Transitions — gated by preconditions, registered for proof generation
impl DbMachine {
    #[formal_method]
    pub fn connect(&mut self, url: &str) -> Result<Established<DbConnected>, DbError> { ... }

    #[formal_method]
    pub fn query(&mut self, sql: &str, _pre: Established<DbConnected>) -> Vec<Row> { ... }
}
```

`#[formal_method]` gates `#[instrument]` under `#[cfg_attr(not(kani), instrument(...))]` and
registers the transition for auto-generated harnesses. Generate and run proofs with:

```bash
elicitation generate kani --crate-path crates/my_vsm/src --out crates/proofs/src/kani/generated
elicitation prove kani      # runs cargo kani --harness-timeout per harness
elicitation prove verus
elicitation prove creusot
# or: just prove
```

---

## Formal Verification

| Verifier | Approach | Current coverage |
|---|---|---|
| **Kani** | Bounded model checking / symbolic execution | 388 passing harnesses |
| **Verus** | SMT-based program logic | 158 passing proofs |
| **Creusot** | Deductive verification | 22,837 valid goals / 19 modules |

**Trust the source, verify the wrapper.** Proofs cover *our* logic: that `from_label()` roundtrips
are complete, `Established::assert()` appears only after the real work, and contracts accurately
describe what each operation establishes. Third-party invariants are accepted via `kani::assume`.

```bash
just verify-kani-tracked
just verify-creusot <file.rs>
just verify-verus-tracked
```

---

## Visibility

| Feature | What it exposes |
|---|---|
| `TypeSpec` | MCP tools `describe_type`/`explore_type` — agents pull spec slices on demand |
| `TypeGraph` | Mermaid / DOT structural graphs of registered types (`graph` feature) |
| `ElicitIntrospect` | Zero-allocation `pattern()` + `metadata()` for tracing spans and Prometheus labels |

`#[derive(Elicit)]` registers types via `inventory::submit!` automatically.

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

Derive `Elicit`, expose tools with `elicit_tools!`, serve:

```rust
#[tool_router]
impl GameServer {
    #[tool(description = "Play a move")]
    pub async fn play(&self, peer: Peer<RoleServer>) -> Result<CallToolResult, ErrorData> {
        let server = ElicitServer::new(peer);

        // Elicit a free-form struct
        let profile = PlayerProfile::elicit(&server).await?;

        // Or constrain to a runtime-computed set — the walled garden pattern
        let bet = ChoiceSet::new(vec![1u32, 5, 10, 25])
            .with_prompt("Choose your bet:")
            .elicit(&server).await?;

        Ok(CallToolResult::success(vec![Content::text(
            format!("{} bets {} on {:?}", profile.name, bet, profile.difficulty),
        )]))
    }

    elicitation::elicit_tools! { Difficulty, PlayerProfile }
}
```

Add shadow crate plugins via `PluginRegistry`:

```rust
PluginRegistry::new()
    .register_flat(GameServer::new())
    .register("http", elicit_reqwest::WorkflowPlugin::default_client())
    .register("db",   elicit_sqlx::SqlxWorkflowPlugin::default())
    .serve(rmcp::transport::stdio())
    .await?;
```

---

## Workspace Crate Map

| Crate | Role |
|---|---|
| `elicitation` | Core: traits, contracts, verification types, MCP plumbing |
| `elicitation_derive` | Proc macros: `#[derive(Elicit)]`, `#[elicit_tool]`, `#[reflect_methods]`, `#[reflect_trait]` |
| `elicit_proofs` | Generated proof harnesses (Kani / Verus / Creusot) |
| `elicitation_rand` | Randomised value generation |
| `elicit_server` | MCP server support |
| `elicit_reqwest` / `elicit_sqlx` / `elicit_tokio` | Workflow vocabularies |
| `elicit_clap` | CLI vocabulary (canonical shadow crate reference) |
| `elicit_chrono` / `elicit_jiff` / `elicit_time` | Datetime vocabularies |
| `elicit_url` / `elicit_regex` / `elicit_uuid` / `elicit_serde_json` / `elicit_std` | Type vocabularies |

---

## Further Reading

| Document | Topic |
|---|---|
| [`SHADOW_CRATE_MOTIVATION.md`](SHADOW_CRATE_MOTIVATION.md) | The inversion thesis |
| [`THIRD_PARTY_SUPPORT_GUIDE.md`](THIRD_PARTY_SUPPORT_GUIDE.md) | Adding a new shadow crate |
| [`ELICITATION_WORKFLOW_ARCHITECTURE.md`](ELICITATION_WORKFLOW_ARCHITECTURE.md) | Workflow infrastructure |
| [`CREUSOT_GUIDE.md`](CREUSOT_GUIDE.md) | Creusot annotation patterns |
| [`FORMAL_VERIFICATION_LEGOS.md`](FORMAL_VERIFICATION_LEGOS.md) | Compositional proof strategy |
| [`crates/elicit_clap/`](crates/elicit_clap/) | Canonical reference shadow crate |
