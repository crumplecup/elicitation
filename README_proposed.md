# Elicitation

> Structured agent interaction backed by formal verification.

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

## What is Elicitation?

An MCP agent calling your tools can pass anything, in any order. Elicitation inverts that:
instead of the agent pushing arbitrary values into your functions, your types *pull* values from
the agent through a verified round-trip. The agent sees a prompt; you get a validated, type-safe
result. That is what `elicit()` does.

The rest of the framework builds on that foundation:

- **Shadow crates** give agents a verified vocabulary for third-party libraries
- **Contracts** make invalid workflow sequences fail at compile time, not runtime
- **Verified State Machines** combine all three into formally proven programs

Every participating type carries proofs for three independent verifiers (Kani, Verus, Creusot),
composed automatically from its fields by `#[derive(Elicit)]`.

---

## Deriving Agent-Native Types

`#[derive(Elicit)]` generates `elicit()`, the MCP JSON schema, and proof methods by composing
the impls of its fields. Add a field, get its proof for free:

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
    pub internal_id: u64,    // excluded from elicitation
}
```

`DeployConfig::elicit(&server).await?` drives a back-and-forth with the agent, prompting
each field in turn and validating responses. The agent cannot skip fields or bypass constraints.

### Interaction paradigms

The derive picks the right paradigm from the shape of the type:

| Trait | Paradigm | Used for |
|---|---|---|
| `Select` | Choose from a finite set | Enums |
| `Affirm` | Binary yes/no | `bool` fields, guard steps |
| `Survey` | Sequence of field elicitations | Structs |

### Style System

A terse machine prompt is noise to a human; a friendly wizard prompt wastes an agent's context.
Name your audience on the field — the derive generates the style enum; swap it at the call site:

```rust
#[prompt("environment")]
#[prompt("Which environment should we deploy to?",      style = "human")]
#[prompt("environment: production|staging|development", style = "agent")]
pub environment: Environment,
```

```rust
let config = DeployConfig::elicit(&client.with_style(DeployConfigElicitStyle::Human)).await?;
```

### Generators

`Generator<Target>` produces values of a target type from any source — sensor fusion,
lookup tables, physics steps. `elicitation_rand` adds `#[rand(...)]` field attributes
(`bounded(L, H)`, `positive`, `nonzero`, `even`, `odd`) and derives a seeded
`random_generator(seed: u64)` for deterministic property testing, with its own Kani suite.

---

## Shadow Crates — The Agent's Dictionary

Agents can only work with what they can name. A shadow crate (`elicit_*`) is a typed vocabulary
for a third-party library — three layers that together let an agent reason about and compose
the library's behaviour without writing a line of Rust:

| Layer | What it provides | Mechanism |
|---|---|---|
| **Types** | `serde` + `JsonSchema` wrappers | Newtypes |
| **Methods** | Instance methods as MCP tools | `#[reflect_methods]` |
| **Traits** | Third-party trait methods as typed factories | `#[reflect_trait]` |

```rust
#[reflect_methods]
pub struct ElicitArg(pub clap::Arg);
// Generates: arg__get_long, arg__get_short, arg__get_help, ... as MCP tools
```

For compile-time macros (e.g. `sqlx::query!`), fragment tools emit verified Rust source via
`EmitCode`; the macro expands in the consumer binary where a live connection is available.

| Domain | Shadow crates |
|---|---|
| Web / HTTP | `elicit_reqwest`, `elicit_axum`, `elicit_tower` |
| Database | `elicit_sqlx`, `elicit_redb`, `elicit_db` |
| Async runtime | `elicit_tokio` |
| CLI | `elicit_clap` |
| UI / graphics | `elicit_egui`, `elicit_ratatui`, `elicit_bevy`, `elicit_wgpu`, `elicit_winit`, `elicit_accesskit`, `elicit_leptos`, `elicit_ui` |
| GIS / geometry | `elicit_geo`, `elicit_geo_types`, `elicit_geojson`, `elicit_georaster`, `elicit_gis`, `elicit_proj`, `elicit_rstar`, `elicit_wkb`, `elicit_wkt` |
| Date / time | `elicit_chrono`, `elicit_jiff`, `elicit_time` |
| Data formats | `elicit_csv`, `elicit_serde`, `elicit_serde_json`, `elicit_toml` |
| String / type utilities | `elicit_url`, `elicit_regex`, `elicit_uuid`, `elicit_uom`, `elicit_std` |

---

## Contracts — Invariants as Types

Programs have invariants that hold regardless of execution path. Most frameworks leave them as
comments or runtime checks — things developers know but the compiler doesn't. Elicitation makes
them explicit in the type system.

A `Prop` is a marker type: a name for an invariant. `Established<P>` is a zero-sized proof
token that `P` holds — it exists only because something produced it, and the only way to produce
it is through a `ProvableFrom` exchange:

```rust
#[derive(Prop)] pub struct ConnectionReady;
#[derive(Prop)] pub struct QueryValidated;

// ProvableFrom is a sealed exchange: present ConnectionReady, receive QueryValidated
impl ProvableFrom<ConnectionReady> for QueryValidated { ... }
```

The exchange lives in a trait method — and that trait method is the *only door* to the token:

```rust
fn validate_query(
    &self,
    query: &str,
    _pre: Established<ConnectionReady>,   // credential required
) -> Result<(ValidatedQuery, Established<QueryValidated>), DbError> {
    // business logic that maintains the invariant — no other path exists
}
```

Because the trait method is the only constructor for `Established<QueryValidated>`, the business
logic *must* run. The compiler enforces it. Proofs can then verify that this chain is sound.

`And<P,Q>` composes proofs; `both(p, q)` constructs conjunctions. Shadow crates ship their own
`Prop` types: `elicit_sqlx` has `DbConnected`, `QueryExecuted`, `TransactionOpen`;
`elicit_reqwest` has `UrlValid`, `RequestCompleted`, `FetchSucceeded`.

The `Tool` trait formalises composable steps with typed `Pre`/`Post` constraints. Sequential
(`then`) and parallel (`both_tools`) composition check at compile time that each step's
postcondition satisfies the next step's precondition.

---

## Verified State Machines

A Verified State Machine is a state machine where every state implements `Elicitation` and
every transition is a `#[formal_method]`. That dual constraint is exactly what the derive needs
to auto-generate proofs: all inputs and outputs are within the verified type system, and all
transitions follow the `ProvableFrom` contract — so there is no path through the machine that
the framework cannot inspect and verify.

```rust
// All states implement Elicitation
#[derive(Debug, Clone, VerifiedStateMachine)]
pub enum PanelState { Idle, Loading, Ready, Error }

// Consistency predicate — the invariant every transition must preserve
pub fn panel_consistent(state: &PanelState, data: &Option<Data>) -> bool {
    match state {
        PanelState::Ready => data.is_some(),
        PanelState::Idle  => data.is_none(),
        _                 => true,
    }
}

// All transitions are #[formal_method] — the only doors between states
impl PanelMachine {
    #[formal_method]
    pub fn begin_load(&mut self) -> Established<Loading> { ... }

    #[formal_method]
    pub fn finish_load(&mut self, data: Data, _pre: Established<Loading>)
        -> Established<Ready> { ... }
}
```

`#[formal_method]` gates `#[instrument]` so proof toolchains don't chase tracing internals,
and registers the transition for harness generation. The generated proofs verify that every
reachable transition preserves the consistency predicate — correctness by construction.

```bash
elicitation generate kani --crate-path crates/my_vsm/src --out crates/proofs/src/kani/generated
just prove   # runs all three backends
```

---

## Formal Verification

The proof infrastructure is bottom-up. The core crate and shadow crates ship verified proofs for
stdlib and third-party types — capturing meaningful bounds: a `PortNumber` is always in
[1024, 65535]; a `NonEmptyString` always has length > 0; an `Established<FetchSucceeded>` is
always produced from a 2xx response. When you derive `Elicit` on your own types, your proofs
compose from those canonical proofs. A struct's proof is the union of its fields' proofs — add
a field, get its verification bounds for free.

At the VSM level, proofs verify the richer claim: that every transition preserves the
consistency predicate. Three verifiers approach this from independent directions:

| Verifier | Approach | Current workspace coverage |
|---|---|---|
| **Kani** | Bounded model checking — exhaustive within bound | 388 passing harnesses |
| **Verus** | SMT-based program logic | 158 passing proofs |
| **Creusot** | Deductive verification — rich compositional invariants | 22,837 valid goals / 19 modules |

```bash
just verify-kani-tracked
just verify-creusot <file.rs>
just verify-verus-tracked
```

---

## Visibility

An agent working with an unfamiliar codebase can't read source. Three features expose the
vocabulary at runtime without flooding the context window:

**TypeSpec** surfaces `ElicitSpec` annotations as MCP tools. `describe_type` returns a
summary and list of available categories; `explore_type` returns one category — `requires`,
`ensures`, `bounds`, `fields` — on demand. Agents pull only what they need.

**TypeGraph** renders structural hierarchies of registered types as Mermaid or DOT graphs.
`list_types()` → `graph_type("ApplicationConfig")` shows how `NetworkConfig`, `Role`, and
`DeploymentMode` compose into it, in one tool call.

**ElicitIntrospect** exposes zero-allocation `pattern()` and `metadata()` for labelling
tracing spans and Prometheus metrics — no source parsing, no allocation:

```rust
ELICITATION_COUNTER
    .with_label_values(&[T::metadata().type_name, T::pattern().as_str()])
    .inc();
```

`#[derive(Elicit)]` registers types for all three via `inventory::submit!` automatically.

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

Derive `Elicit` on your domain types, call `elicit()` inside a tool handler, and expose
via `elicit_tools!`:

```rust
#[tool_router]
impl GameServer {
    #[tool(description = "Play a move")]
    pub async fn play(&self, peer: Peer<RoleServer>) -> Result<CallToolResult, ErrorData> {
        let server = ElicitServer::new(peer);

        // Elicit a validated struct — the agent cannot produce an invalid PlayerProfile
        let profile = PlayerProfile::elicit(&server).await?;

        // ChoiceSet: constrain to a runtime-computed set — agent cannot pick outside it
        let bet = ChoiceSet::new(vec![1u32, 5, 10, 25])
            .with_prompt("Choose your bet:")
            .elicit(&server).await?;

        Ok(CallToolResult::success(vec![Content::text(
            format!("{} bets {} on {:?}", profile.name, bet, profile.difficulty),
        )]))
    }

    // Auto-generate elicit_difficulty and elicit_player_profile tools
    elicitation::elicit_tools! { Difficulty, PlayerProfile }
}
```

Add shadow crate plugins alongside your own server via `PluginRegistry`:

```rust
PluginRegistry::new()
    .register_flat(GameServer::new())
    .register("http", elicit_reqwest::WorkflowPlugin::default_client())
    .register("db",   elicit_sqlx::SqlxWorkflowPlugin::default())
    .serve(rmcp::transport::stdio())
    .await?;
```

One registry, one transport, zero glue code.

---

## Workspace Crate Map

| Crate | Role |
|---|---|
| `elicitation` | Core: traits, contracts, MCP plumbing |
| `elicitation_derive` | Proc macros: `#[derive(Elicit)]`, `#[formal_method]`, `#[reflect_methods]`, `#[reflect_trait]` |
| `elicitation_rand` | Randomised value generation |
| `elicit_proofs` | Generated harnesses (Kani / Verus / Creusot) |
| `elicitation_kani` / `elicitation_creusot` | Proof gallery and support |
| `elicit_server` | MCP server support |
| `elicit_clap` | CLI vocabulary (canonical shadow crate reference) |
| `elicit_*` | 30+ domain shadow crates — see [Shadow Crates](#shadow-crates--the-agents-dictionary) above |

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
