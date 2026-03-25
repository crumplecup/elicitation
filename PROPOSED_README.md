# Elicitation

> **Formally verified type-safe state machines. Program invariants you can prove.**

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

---

## The Story

### Step 1: Verifying types (boring, but foundational)

Formal verification research focuses almost entirely on methods — and for good
reason. Methods are interesting; types are not. Who cares that an `i8` is an `i8`?

And yet: verifying types is tractable. A constrained type like `PortNumber` or
`StringNonEmpty` has a crisp, machine-checkable invariant. When a type implements
`Elicitation`, it gains three proof methods as part of the trait — each returning a
`TokenStream` that the corresponding toolchain can verify:

```rust
#[cfg(feature = "proofs")]
fn kani_proof()    -> proc_macro2::TokenStream { /* symbolic execution proof */ }
fn verus_proof()   -> proc_macro2::TokenStream { /* SMT specification proof  */ }
fn creusot_proof() -> proc_macro2::TokenStream { /* deductive contract proof  */ }
```

These are not annotations on a separate proof file. The type *carries* its proof.
For user-defined types, `#[derive(Elicit)]` composes the proof automatically from
the proofs of the constituent field types — add a field, get its proof for free.
Compose two types and you have the materials to compose their proofs.

### Step 2: State machines (not boring at all)

It turns out that verifying types is only a little harder than verifying state
machines — and state machines are far more interesting than plain types. The
contracts system makes state transitions first-class:

```rust
pub struct DbConnected;    // proposition: a connection is open
pub struct QueryExecuted;  // proposition: a query ran successfully
impl Prop for DbConnected {}
impl Prop for QueryExecuted {}
```

`Established<P>` is a zero-sized proof token that proposition `P` holds.
It cannot be constructed except by code that actually performed the work. The
compiler then enforces transition order: you cannot call a function requiring
`Established<QueryExecuted>` without first holding `Established<DbConnected>`.

This is a formally proven type-safe state machine — and the proofs compose. If
`DbConnected` has a verified proof and `QueryExecuted` has a verified proof, their
conjunction `And<DbConnected, QueryExecuted>` has a verified proof too, via
`both()`.

### Step 3: Methods that are correct by construction (the payoff)

State machines compose into methods. The main thing preserved across every proof is
a **program invariant** — a property of the system that holds regardless of what
path execution took to get there.

With invariants expressed in the type system and verified at each step, developer
goals can be projected into the type space: define what "ready to deploy" means as
a proposition, write the functions that establish its preconditions, and the
compiler guarantees that the deployment function can only be called once all
invariants are satisfied.

The agent's role in this is proof search: given a goal type, find the sequence of
verified operations that transforms the current state into the desired state. Every
step is an auditable tool call with a known contract. The resulting tool chain *is*
the method — correct by construction, not verified after the fact.

---

## Architecture

The framework has three layers:

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

For your own domain types, `#[derive(Elicit)]` is all you need. MCP requires all
values that cross the boundary to be `Serialize + DeserializeOwned + JsonSchema`, so
your type must derive all four — the compiler may not always catch a missing impl,
but you will get runtime errors if a type is not fully wired:

```rust
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use elicitation::Elicit;

// All four derives are required for MCP tool use
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
#[prompt("Configure your deployment")]
#[spec_summary("Deployment configuration with environment and scale settings")]
pub struct DeployConfig {
    #[prompt("Which environment?")]
    pub environment: Environment,   // must also impl Elicitation

    #[prompt("Number of replicas (1–16):")]
    #[spec_requires("replicas >= 1 && replicas <= 16")]
    pub replicas: u8,

    #[prompt("Container image URI:")]
    pub image: String,

    #[skip]
    pub _internal_id: u64,          // not elicited; omitted from MCP interaction
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Environment {
    Production,
    Staging,
    Development,
}
```

`#[derive(Elicit)]` generates the full `Elicitation` impl — the `elicit()` async
method, the `Prompt` impl, and the proof functions — by composing the impls of the
constituent field types. This is why `Elicit` can only be derived on types whose
fields already implement `Elicitation`: the derive has nothing to compose from if
a field type is unknown to the framework.

This is also why the core `elicitation` crate manually implements `Elicitation` for
third-party types behind feature gates (e.g. `features = ["sqlx-types"]`): those
types don't derive it themselves, so we provide the impl so they can participate as
fields. The `elicit_*` shadow crates build on top of that foundation — they add
newtype wrappers and expose methods and traits as MCP tools, but require the
`Elicitation` support to already be in place in the core crate.

### Derived proof functions

When you derive `Elicit`, your type automatically gets working `kani_proof()`,
`verus_proof()`, and `creusot_proof()` methods (with `feature = "proofs"`). The
derive walks every field type and extends its own proof token stream with each
field's proof:

```rust
// What the derive generates for DeployConfig:
fn kani_proof() -> proc_macro2::TokenStream {
    let mut ts = TokenStream::new();
    ts.extend(<Environment as Elicitation>::kani_proof());
    ts.extend(<u8 as Elicitation>::kani_proof());
    ts.extend(<String as Elicitation>::kani_proof());
    ts
}
```

A struct's proof is the union of its parts. Add a field, get its proof for free.

### Style System

Every type ships with default prompts, but the Style system means you are never
locked into them. The classic use case is **human vs. AI audience**: a terse
machine-readable prompt is noise to a human; a friendly wizard-style prompt is
equally wasteful to an agent that just needs the field name and constraints.

You don't implement anything — you annotate fields with `#[prompt]` and name
the styles you need. The derive generates the style enum for you:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct DeployConfig {
    // Default prompt (used when no style is active)
    #[prompt("environment")]
    // Named style overrides — derive generates DeployConfigElicitStyle::{ Default, Human, Agent }
    #[prompt("Which environment should we deploy to?", style = "human")]
    #[prompt("environment: production|staging|development", style = "agent")]
    pub environment: Environment,

    #[prompt("replicas")]
    #[prompt("How many replicas? (1–16, default 2):", style = "human")]
    #[prompt("replicas: u8 1..=16", style = "agent")]
    pub replicas: u8,
}
```

Then hot-swap the style per session at the call site — no changes to the type:

```rust
// Human session
let result = DeployConfig::elicit(
    &client.with_style::<DeployConfig, _>(DeployConfigElicitStyle::Human)
).await?;

// Agent session
let result = DeployConfig::elicit(
    &client.with_style::<DeployConfig, _>(DeployConfigElicitStyle::Agent)
).await?;
```

`with_style::<T, _>(style)` works on **any** type `T` with an `Elicitation`
impl, including third-party types the core crate ships impls for. This is the
escape hatch from vendor lock-in: our default prompts are a starting point, not
a constraint.

### Generators

The `Generator` trait is a simple contract: given configuration, produce values
of a target type.

```rust
pub trait Generator {
    type Target;
    fn generate(&self) -> Self::Target;
}
```

Any struct that knows how to produce a `T` can implement it — the source is
entirely up to you. A sensor fusion generator that reads temperature and
humidity to derive barometric pressure, a lookup-table interpolator, a physics
simulation step — all of these are just "methods that generate at `T`", and
they compose identically with everything else in the framework.

```rust
struct BarometricGenerator { altitude_m: f32, lapse_rate: f32 }

impl Generator for BarometricGenerator {
    type Target = Pressure;

    fn generate(&self) -> Pressure {
        let p = SEA_LEVEL_PA * (1.0 - self.lapse_rate * self.altitude_m / 288.15).powf(5.2561);
        Pressure::new(p)
    }
}
```

Because `Generator` is a plain trait, implementations can be exposed as MCP
tools through the shadow crate machinery — agents can request "generate a
`Pressure` reading" without knowing the formula or the sensor source. The
pattern also fits workflows where the agent elicits the *strategy* once and
the program drives generation many times from it:

```rust
let mode = InstantGenerationMode::elicit(communicator).await?;
let generator = InstantGenerator::new(mode);

let t1 = generator.generate();
let t2 = generator.generate();
```

The `elicitation_rand` crate extends this with the `Rand` trait and
`#[rand(...)]` field attributes that encode the same constraints as the type's
formal proofs directly into the sampling strategy — `bounded(L, H)`,
`positive`, `nonzero`, `even`, `odd`, and `and(A, B)` / `or(A, B)`. The
derive generates a seeded `random_generator(seed: u64)` for deterministic,
reproducible output. `elicitation_rand` ships its own Kani suite proving
generators satisfy their declared contracts.

### Action traits: the grammar of elicitation

Every `Elicitation` impl is built on one of three **action traits** that describe
the interaction paradigm. These are the primitives from which all elicitation
behaviour is composed, and they are formally verified:

| Trait | Paradigm | Typical use |
|---|---|---|
| `Select` | Choose one from a finite set | Enums, categorical fields |
| `Affirm` | Binary yes/no confirmation | `bool` fields, guard steps |
| `Survey` | Sequential multi-field elicitation | Structs, configuration objects |

`#[derive(Elicit)]` automatically assigns the correct action trait — enums with
unit variants get `Select`, `bool` fields get `Affirm`, structs get `Survey` —
and the derived state machine sequences the interactions accordingly.

Together with the contract types (`Prop`, `Established<P>`, `And<P,Q>`), the
action traits provide the **grammar for constructing formally verified state
transitions**: `Select` constrains the domain of a transition to a known finite
set, `Affirm` guards a transition on a binary condition, and `Survey` sequences
a set of field transitions into a single composite step. Each interaction has a
verified proof; the composition of interactions inherits those proofs.

---

## Layer 2: Shadow Crates — The Agent's Dictionary

A **shadow crate** (`elicit_*`) is a crate-shaped vocabulary for a third-party library.
It exposes three things:

| Layer | What it provides | Mechanism |
|---|---|---|
| **Types** | `serde` + `JsonSchema` wrappers so values cross the MCP boundary | Newtypes |
| **Methods** | Instance methods exposed as MCP tools | `#[reflect_methods]` |
| **Traits** | Third-party trait methods as typed factories | `#[reflect_trait]` |

Together, these form a **complete vocabulary** for the library. An agent with access
to all three layers can reason about and compose the library's behaviour without
writing a single line of Rust.

### Three Tool Exposure Mechanisms

**`#[reflect_methods]`** — for a newtype with methods you want the agent to call:

```rust
use elicitation_derive::reflect_methods;

#[reflect_methods]
pub struct ElicitArg(pub clap::Arg);
// Generates: arg__get_long, arg__get_short, arg__get_help, ... as MCP tools
```

**`#[reflect_trait]`** — for a third-party trait whose methods are worth calling on
any registered `T: FooTrait`:

```rust
use elicitation_macros::reflect_trait;

#[reflect_trait(clap::ValueEnum)]
pub trait ValueEnumTools {
    fn value_variants(&self) -> Vec<PossibleValue>;
}
// Generates a typed factory: any T: ValueEnum gets value_variants exposed as a tool
```

**Fragment tools + `EmitCode`** — for compile-time macros that cannot run at MCP
time (e.g. `sqlx::query!`, `sqlx::migrate!`). The agent calls a fragment tool that
emits verified Rust source via `EmitCode`. The emitted code is compiled into the
consumer binary where the macro runs with a live database connection:

```rust
#[elicit_tool(
    plugin = "sqlx_frag",
    name   = "query",
    description = "Emit a sqlx::query! call. Establishes: QueryFragmentEmitted.",
    emit_ctx("ctx.db_url" => r#"std::env::var("DATABASE_URL").expect("DATABASE_URL")"#),
)]
async fn emit_query(ctx: Arc<PluginContext>, p: QueryParams) -> Result<CallToolResult, ErrorData> {
    // p.sql is emitted as a sqlx::query!(p.sql, args...) TokenStream
    // ...
}
```

### Available Shadow Crates

| Crate | Library | What it covers |
|---|---|---|
| `elicit_reqwest` | `reqwest` | HTTP client: fetch, post, auth, pagination, workflow |
| `elicit_sqlx` | `sqlx` | Database: connect, execute, fetch, transactions, query fragments |
| `elicit_tokio` | `tokio` | Async: sleep, timeout, semaphores, barriers, file I/O |
| `elicit_clap` | `clap` | CLI: `Arg`, `Command`, `ValueEnum`, `PossibleValue` |
| `elicit_chrono` | `chrono` | Datetimes, durations, timezones |
| `elicit_jiff` | `jiff` | Temporal arithmetic |
| `elicit_time` | `time` | Date/time primitives |
| `elicit_url` | `url` | URL construction and validation |
| `elicit_regex` | `regex` | Pattern matching |
| `elicit_uuid` | `uuid` | UUID generation |
| `elicit_serde_json` | `serde_json` | JSON values, maps, dynamic typing |
| `elicit_std` | `std` | Selected stdlib types |

---

## Layer 3: Contracts — Workflow Correctness at the Type Level

The contracts system makes workflow preconditions impossible to violate at compile
time. A `Prop` is a marker trait; `Established<P>` is a zero-sized proof token that
`P` holds; `And<P,Q>` composes proofs; `both()` constructs a conjunction.

```rust
use elicitation::contracts::{Prop, Established, And, both};

// Domain propositions — unit structs that act as type-level facts
pub struct DbConnected;
pub struct QueryExecuted;
impl Prop for DbConnected {}
impl Prop for QueryExecuted {}

// A function that REQUIRES proof of connection
fn fetch_rows(
    sql: &str,
    _pre: Established<DbConnected>,   // caller must supply this
) -> Vec<Row> {
    // ...
}

// A function that PRODUCES proof of connection
fn connect(url: &str) -> (Pool, Established<DbConnected>) {
    let pool = Pool::connect(url);
    (pool, Established::assert())     // assert: we just did the work
}

// Composing proofs
let (pool, db_proof) = connect(&url);
let (_, query_proof) = execute_query(&pool, db_proof);
let both_proof: Established<And<DbConnected, QueryExecuted>> =
    both(db_proof, query_proof);
```

Shadow crates ship their own domain propositions. `elicit_sqlx` provides
`DbConnected`, `QueryExecuted`, `RowsFetched`, `TransactionOpen`,
`TransactionCommitted`, `TransactionRolledBack`. `elicit_reqwest` provides
`UrlValid`, `RequestCompleted`, `StatusSuccess`, `Authorized`, and the composite
`FetchSucceeded = And<UrlValid, And<RequestCompleted, StatusSuccess>>`.

The `Tool` trait formalises this pattern for composable workflow steps:

```rust
pub trait Tool {
    type Input: Elicitation;
    type Output;
    type Pre: Prop;
    type Post: Prop;

    async fn execute(
        &self,
        input: Self::Input,
        pre: Established<Self::Pre>,
    ) -> ElicitResult<(Self::Output, Established<Self::Post>)>;
}
```

Sequential composition (`then`) and parallel composition (`both_tools`) are
provided, with the type system enforcing that each step's `Post` satisfies the
next step's `Pre`.

---

## Formal Verification

Every type that implements `Elicitation` can carry proofs for three independent
verifiers (with the `proofs` feature). The default implementations return empty
token streams; concrete implementations emit verified source:

```rust
// A constrained integer type carrying its own proofs
impl Elicitation for PortNumber {
    // ...elicit(), prompt(), etc...

    #[cfg(feature = "proofs")]
    fn kani_proof() -> proc_macro2::TokenStream {
        quote! {
            #[kani::proof]
            fn verify_port_number_bounds() {
                let n: u16 = kani::any();
                kani::assume(n >= 1024 && n <= 65535);
                let port = PortNumber::new(n).unwrap();
                assert!(*port >= 1024 && *port <= 65535);
            }
        }
    }

    #[cfg(feature = "proofs")]
    fn verus_proof() -> proc_macro2::TokenStream {
        quote! {
            proof fn port_number_invariant(n: u16)
                requires 1024 <= n <= 65535
                ensures PortNumber::new(n).is_some()
            { /* ... */ }
        }
    }
}
```

### The three verifiers

| Verifier | Approach | Strength | Coverage |
|---|---|---|---|
| **Kani** | Bounded model checking / symbolic execution | Exhaustive within bound; finds real bugs | 388 passing harnesses |
| **Verus** | SMT-based program logic | Arithmetic and data structure properties | 158 passing proofs |
| **Creusot** | Deductive verification with Pearlite annotations | Rich invariants; compositional across functions | 19,909 valid goals across 16 modules |

Results are tracked in `*_verification_results.csv`.

### Trust the source, verify the wrapper

Proofs are scoped to *our* logic, not third-party internals:

- **Trust the source** — `std::collections::HashMap` correctly stores keys;
  `clap::ColorChoice` has the variants its docs claim. That is the upstream
  library's responsibility.
- **Verify the wrapper** — our `from_label()` roundtrip is complete; our
  `Established::assert()` calls appear only after the real work succeeds; our
  contracts accurately describe what each operation establishes.

This keeps proofs tractable, focused, and composable. A proof that accepts a
third-party invariant via `kani::assume` and asserts our dispatch contract on top
is a valid, composable proof node — not a shortcut.

### Composing proofs across the system

Because proof methods live on the type, they compose naturally. Two types'
`kani_proof()` token streams can be combined into a single proof harness that
verifies both together. The contracts system (`Prop`/`Established`/`And`) provides
the state-machine layer; the proof methods provide the type-invariant layer. Both
compose independently and together.

Run verification with:

```bash
just verify-kani-tracked       # Kani — all harnesses with CSV tracking
just verify-kani <harness>     # Single harness
just verify-creusot <file.rs>  # Creusot
just verify-verus-tracked      # Verus
```

---

## Visibility

Formal proofs tell you what properties hold. The visibility layer tells you
*what is actually running* — and makes that information available to both
developers and agents at every level, from static type structure to production
telemetry.

### TypeSpec — contracts on demand

Every elicitation type implements the `ElicitSpec` trait, which is built
alongside the `anodized::spec` `#[spec]` annotations on its constructors —
keeping the formal conditions and the browsable spec in sync by construction.

`TypeSpecPlugin` surfaces these specs as MCP tools using a **lazy dictionary**
pattern: agents pull only the spec slice they need rather than flooding the
context window with schema dumps.

| Tool | What it returns |
|---|---|
| `type_spec__describe_type` | Summary + list of available spec categories |
| `type_spec__explore_type` | One category in full: `requires`, `ensures`, `bounds`, `fields` |

Types register themselves via `inventory::submit!` when `#[derive(Elicit)]` is
used, so the dictionary stays current with the codebase automatically.

### TypeGraph — structure at a glance

`TypeGraphPlugin` (feature: `graph`) renders the structural hierarchy of
registered types as Mermaid diagrams or DOT graphs — without reading source code.

| Tool | What it returns |
|---|---|
| `type_graph__list_types` | All registered graphable type names |
| `type_graph__graph_type` | Mermaid or DOT graph rooted at a given type |
| `type_graph__describe_edges` | Human-readable edge summary for one type |

`#[derive(Elicit)]` on non-generic types auto-registers them via
`inventory::submit!(TypeGraphKey)`. An agent can call `list_types()` to
discover the vocabulary, then `graph_type("ApplicationConfig")` to see how
`NetworkConfig`, `Role`, and `DeploymentMode` compose into it — all in a
single tool call.

### ElicitIntrospect — stateless observability

`ElicitIntrospect` is a trait extending `Elicitation` that exposes static
structural metadata for production instrumentation:

```rust
pub trait ElicitIntrospect: Elicitation {
    fn pattern()  -> ElicitationPattern;  // Survey / Select / Affirm / Primitive
    fn metadata() -> TypeMetadata;        // type_name, description, fields/variants
}
```

Both methods are **pure functions with zero allocation** — ideal for labelling
spans and metrics without overhead:

```rust
// Add type structure to OpenTelemetry / tracing spans
#[tracing::instrument(skip(communicator), fields(
    type_name = %T::metadata().type_name,
    pattern   = %T::pattern().as_str(),
))]
async fn elicit_with_tracing<T: ElicitIntrospect>(
    communicator: &impl ElicitCommunicator
) -> ElicitResult<T> {
    T::elicit(communicator).await
}

// Prometheus counter: one metric, labelled by type + pattern
ELICITATION_COUNTER
    .with_label_values(&[T::metadata().type_name, T::pattern().as_str()])
    .inc();
```

`#[derive(Elicit)]` generates the `ElicitIntrospect` impl automatically,
composing field and variant metadata from the constituent types. The example
`observability_introspection.rs` walks through tracing, metrics, agent
planning, and nested introspection patterns in full.

---

## Getting Started

```toml
[dependencies]
elicitation = "0.9"

# Add shadow crates for the libraries you use
elicit_sqlx    = { version = "0.9", features = ["postgres"] }
elicit_reqwest = "0.9"
```

Implement `Elicitation` on your domain types (or use `#[derive(Elicit)]` for
compatible types), register your shadow crate plugins with `PluginRegistry`, and
serve over MCP:

```rust
use elicitation::{ElicitServer, PluginRegistry};
use elicit_sqlx::SqlxWorkflowPlugin;
use elicit_reqwest::WorkflowPlugin as HttpPlugin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = PluginRegistry::new();
    registry.register(SqlxWorkflowPlugin::default());
    registry.register(HttpPlugin::default());

    ElicitServer::new(registry).serve_stdio().await?;
    Ok(())
}
```

---

## Workspace Crate Map

| Crate | Role |
|---|---|
| `elicitation` | Core library: traits, contracts, verification types, MCP plumbing |
| `elicitation_derive` | Proc macros: `#[derive(Elicit)]`, `#[elicit_tool]`, `#[reflect_methods]` |
| `elicitation_macros` | Additional macros: `#[reflect_trait]` |
| `elicitation_kani` | Kani proof harnesses for all verified operations |
| `elicitation_creusot` | Creusot deductive proofs |
| `elicitation_verus` | Verus SMT proofs |
| `elicitation_rand` | Randomised value generation for property testing |
| `elicit_reqwest` | HTTP workflow vocabulary |
| `elicit_sqlx` | Database workflow vocabulary |
| `elicit_tokio` | Async runtime vocabulary |
| `elicit_clap` | CLI vocabulary |
| `elicit_chrono` / `elicit_jiff` / `elicit_time` | Datetime vocabularies |
| `elicit_url` / `elicit_regex` / `elicit_uuid` | String-type vocabularies |
| `elicit_serde` / `elicit_serde_json` | Serialization vocabulary |
| `elicit_server` | MCP server support |
| `elicit_std` | Stdlib vocabulary |

---

## Further Reading

| Document | Topic |
|---|---|
| [`SHADOW_CRATE_MOTIVATION.md`](SHADOW_CRATE_MOTIVATION.md) | The deep rationale for the inversion thesis |
| [`THIRD_PARTY_SUPPORT_GUIDE.md`](THIRD_PARTY_SUPPORT_GUIDE.md) | How to add a new shadow crate end-to-end |
| [`ELICITATION_WORKFLOW_ARCHITECTURE.md`](ELICITATION_WORKFLOW_ARCHITECTURE.md) | Workflow infrastructure deep dive |
| [`CREUSOT_GUIDE.md`](CREUSOT_GUIDE.md) | Creusot annotation patterns |
| [`FORMAL_VERIFICATION_LEGOS.md`](FORMAL_VERIFICATION_LEGOS.md) | Compositional proof strategy |
| [`crates/elicit_clap/`](crates/elicit_clap/) | Canonical reference shadow crate implementation |
