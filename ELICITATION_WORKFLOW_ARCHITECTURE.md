# Elicitation Repository: Workflow Infrastructure Deep Dive

## Executive Summary

The **elicitation** repository implements a sophisticated **agent-centric type system** that teaches LLMs to construct domain values through composable, formally-verified steps. The architecture revolves around three core concepts:

1. **Workflow as Type-Safe Tool Composition** - Each workflow is a sequence of contracted steps with compile-time proof of preconditions/postconditions
2. **Typestate Patterns for HTTP Workflows** - HTTP operations emit proofs (UrlValid, RequestCompleted, StatusSuccess) that compose into stronger guarantees
3. **Code Recovery & Emission** - Agents' tool calls emit verified Rust source, creating a bridge between agent actions and compiled binaries

---

## 1. CRATES IN THE WORKSPACE

### Crate Listing (from Cargo.toml)

```
crates/
├── elicitation/                    # Core library (traits, contracts, type graph)
├── elicitation_derive/             # Proc macros (#[derive(Elicit)], #[elicit_tool])
├── elicitation_macros/             # Additional macros
├── elicitation_kani/               # Kani verification integration
├── elicitation_creusot/            # Creusot verification integration
├── elicitation_rand/               # Random value generation
├── elicit_reqwest/                 # HTTP client wrapper (MAJOR: workflow plugin)
├── elicit_serde/                   # Serde integration
├── elicit_serde_json/              # JSON utilities
├── elicit_server/                  # MCP server support
├── elicit_uuid/                    # UUID support
├── elicit_time/                    # time crate support
├── elicit_regex/                   # Regex support
├── elicit_chrono/                  # Chrono datetime support
├── elicit_url/                     # URL support
└── elicit_jiff/                    # Jiff datetime support
```

---

## 2. WORKFLOWS IN THIS CONTEXT

### What Is a "Workflow"?

A **workflow in elicitation** is NOT a state machine or DAG executor. Instead, it's:

> **A type-safe composition of MCP tool calls where each step's postcondition proofs ensure the next step's preconditions are met.**

### The Workflow Definition Pattern

**File:** `/home/erik/repos/elicitation/crates/elicitation/src/tool.rs` (lines 1-482)

Core trait:
```rust
pub trait Tool {
    type Input: Elicitation;
    type Output;
    type Pre: Prop;        // Precondition (what must be true BEFORE)
    type Post: Prop;       // Postcondition (what becomes true AFTER)

    async fn execute(
        &self,
        input: Self::Input,
        _pre: Established<Self::Pre>,
    ) -> ElicitResult<(Self::Output, Established<Self::Post>)>;
}
```

### How Workflows Are Composed

**Primitive Composition Function:** `then(tool1, tool2)`

```rust
pub async fn then<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    pre1: Established<T1::Pre>,
) -> ElicitResult<(T2::Output, Established<T2::Post>)>
where
    T1: Tool,
    T2: Tool<Input = T1::Output>,
    T1::Post: Implies<T2::Pre>,  // KEY: Postcondition must imply next precondition
```

**Example:**
```rust
// Step 1: Validate email (pre=True, post=EmailValidated)
let (validated_email, proof1) = validate_tool.execute(email, True::axiom()).await?;

// Step 2: Send email (pre=EmailValidated, post=True)
let (_, proof2) = send_tool.execute(validated_email, proof1.weaken()).await?;
// Cannot compile if Step 2 doesn't require EmailValidated!
```

### Parallel Composition Function: `both_tools(t1, t2)`

```rust
pub async fn both_tools<T1, T2>(
    tool1: &T1,
    tool2: &T2,
    input1: T1::Input,
    input2: T2::Input,
    pre: Established<And<T1::Pre, T2::Pre>>,
) -> ElicitResult<(
    (T1::Output, T2::Output),
    Established<And<T1::Post, T2::Post>>,
)>
```

---

## 3. ELICIT_REQWEST: THE MAJOR HTTP WORKFLOW PLUGIN

### File Location
`/home/erik/repos/elicitation/crates/elicit_reqwest/src/plugins/workflow.rs` (1,200+ lines)

### Plugin Registration
```rust
#[derive(ElicitPlugin)]
#[plugin(name = "workflow")]
pub struct WorkflowPlugin(pub Arc<PluginContext>);
```

Registers as namespace prefix `"workflow"`, exposing 10 tools:
- `workflow__url_build`
- `workflow__fetch`
- `workflow__fetch_json`
- `workflow__fetch_auth`
- `workflow__post_json`
- `workflow__api_call`
- `workflow__health_check`
- `workflow__build_request`
- `workflow__status_summary`
- `workflow__paginated_get`

### Propositions / Contracts

The workflow plugin defines **domain-specific propositions** that compose HTTP guarantees:

**File:** Lines 50-72

```rust
// Atomic propositions
pub struct UrlValid;
impl Prop for UrlValid {}

pub struct RequestCompleted;
impl Prop for RequestCompleted {}

pub struct StatusSuccess;
impl Prop for StatusSuccess {}

pub struct Authorized;
impl Prop for Authorized {}

// Composite propositions
pub type FetchSucceeded = And<UrlValid, And<RequestCompleted, StatusSuccess>>;
pub type AuthFetchSucceeded = And<Authorized, FetchSucceeded>;
```

### Tool Implementations

Each tool is defined with the `#[elicit_tool]` macro, which:
1. Takes a `name`, `description`
2. Emits contract documentation (what it assumes, what it establishes)
3. May include `emit_ctx` for code recovery
4. Is instrumented with tracing

#### Example: `url_build` (Lines 698-730)

```rust
#[elicit_tool(
    plugin = "workflow",
    name = "url_build",
    description = "Build a validated URL from base, optional path, and query parameters. \
                   Assumes: base is a well-formed URL string. \
                   Establishes: UrlValid — the result parses without error."
)]
#[instrument(skip_all, fields(base = %p.base.get()))]
async fn wf_url_build(
    ctx: Arc<PluginContext>,
    p: UrlBuildParams,
) -> Result<CallToolResult, ErrorData> {
    let _ = &ctx; // stateless — no HTTP call
    let mut url = p.base.into_inner();
    if let Some(path) = &p.path {
        url.set_path(path);
    }
    if let Some(query) = &p.query {
        let qs: String = query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding_simple(k), urlencoding_simple(v)))
            .collect::<Vec<_>>()
            .join("&");
        url.set_query(if qs.is_empty() { None } else { Some(&qs) });
    }
    let result = serde_json::json!({
        "url": url.to_string(),
        "contract": "UrlValid",
    });
    Ok(CallToolResult::success(vec![Content::text(result.to_string())]))
}
```

#### Example: `fetch` (Lines 732-756)

```rust
#[elicit_tool(
    plugin = "workflow",
    name = "fetch",
    description = "GET a URL and return the response body. \
                   Assumes: url is a valid URL; host is reachable; response is 2xx. \
                   Establishes: UrlValid ∧ RequestCompleted ∧ StatusSuccess (FetchSucceeded).",
    emit_ctx("ctx.http" => "reqwest::Client::new()")
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch(ctx: Arc<PluginContext>, p: FetchParams) -> Result<CallToolResult, ErrorData> {
    match do_fetch(&ctx.http, p.url.get().as_str(), HeaderMap::new(), timeout(p.timeout_secs)).await {
        Ok((r, _proof)) => {
            let json = serde_json::to_string(&r).unwrap_or_default();
            Ok(CallToolResult::success(vec![Content::text(json)]))
        }
        Err(err_result) => return Ok(err_result),
    }
}
```

#### Example: `fetch_auth` (Lines 789-844)

```rust
#[elicit_tool(
    plugin = "workflow",
    name = "fetch_auth",
    description = "GET a URL with authorization (Bearer/Basic/ApiKey) and return the body. \
                   Assumes: url is valid; token is non-empty; response is 2xx. \
                   Establishes: Authorized ∧ FetchSucceeded (AuthFetchSucceeded).",
    emit_ctx("ctx.http" => "reqwest::Client::new()")
)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch_auth(ctx: Arc<PluginContext>, p: AuthFetchParams) -> Result<CallToolResult, ErrorData> {
    let url_proof: Established<UrlValid> = Established::assert();
    let rb = ctx.http.get(p.url.get().as_str()).timeout(timeout(p.timeout_secs));
    let (rb, auth_proof_opt) = apply_auth(rb, &p.auth_type, Some(&p.token));
    
    let resp = rb.send().await.map_err(|e| format!("Request failed: {e}"))?;
    let req_proof: Established<RequestCompleted> = Established::assert();
    
    if !resp.status().is_success() {
        return Err(format!("StatusSuccess not established: got {}", resp.status().as_u16()));
    }
    let status_proof: Established<StatusSuccess> = Established::assert();
    let combined = both(url_proof, both(req_proof, status_proof));
    
    // Returns (FetchResult, proof of FetchSucceeded)
}
```

### All 10 Tool Definitions (Summary)

| Line | Tool | Description | Establishes |
|------|------|-------------|-------------|
| 698-730 | `url_build` | Parse/construct URL | `UrlValid` |
| 732-756 | `fetch` | HTTP GET + 2xx check | `FetchSucceeded` |
| 758-787 | `fetch_json` | GET with `Accept: application/json` | `FetchSucceeded` |
| 789-844 | `fetch_auth` | GET with auth header | `AuthFetchSucceeded` |
| 850-877 | `post_json` | POST with body + 2xx check | `FetchSucceeded` |
| 879-945 | `api_call` | POST with Bearer token | `FetchSucceeded` |
| 946-989 | `health_check` | GET → bool (no proof) | _(pure, no side-effects)_ |
| 990-1040 | `build_request` | Compose HTTP request (GET/POST/etc.) | `FetchSucceeded` |
| 1041-1084 | `status_summary` | Classify status code | _(pure, classifies)_ |
| 1086-1124 | `paginated_get` | GET + parse RFC 5988 Link header | `FetchSucceeded` |

---

## 4. CORE ELICITATION CRATE: WORKFLOW-RELATED TYPES

### File: `/home/erik/repos/elicitation/crates/elicitation/src/traits.rs` (519 lines)

#### Elicitation Trait

```rust
pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    
    fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl Future<Output = ElicitResult<Self>> + Send;
    
    fn elicit_checked(
        peer: Peer<RoleServer>,
    ) -> impl Future<Output = ElicitResult<Self>> + Send { ... }
    
    fn elicit_proven<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl Future<Output = ElicitResult<(Self, Established<Is<Self>>)>> + Send { ... }
}
```

#### ElicitIntrospect Trait (for Type Graph Inspection)

```rust
pub trait ElicitIntrospect: Elicitation {
    /// Return compile-time metadata about this type's structure.
    fn metadata() -> TypeMetadata;
}
```

**What Does Metadata Include?**

```rust
pub struct TypeMetadata {
    pub type_name: String,
    pub description: Option<String>,
    pub details: PatternDetails,
}

pub enum PatternDetails {
    Survey { fields: Vec<FieldInfo> },
    Select { variants: Vec<VariantMetadata> },
    Affirm,
    Primitive,
}

pub struct FieldInfo {
    pub name: String,
    pub type_name: String,
}

pub struct VariantMetadata {
    pub label: String,
    pub fields: Vec<FieldInfo>,
}
```

### Contract System

**File:** `/home/erik/repos/elicitation/crates/elicitation/src/contracts.rs` (800+ lines)

#### Core Propositions

```rust
pub trait Prop: 'static {}

pub struct Established<P: Prop> {
    _phantom: PhantomData<P>,  // Zero-cost proof token!
}

impl<P: Prop> Established<P> {
    pub fn assert() -> Self {
        Established { _phantom: PhantomData }
    }
}
```

#### Logical Operators

```rust
pub struct And<P: Prop, Q: Prop> { /* ... */ }

pub trait Implies<Q: Prop>: Prop {}

pub struct Refines<Base> { /* ... */ }

pub struct InVariant<E, V> { /* ... */ }
```

#### Composition Functions

```rust
pub fn both<P: Prop, Q: Prop>(
    p: Established<P>,
    q: Established<Q>,
) -> Established<And<P, Q>> { ... }

pub fn fst<P: Prop, Q: Prop>(
    pq: Established<And<P, Q>>,
) -> Established<P> { ... }

pub fn snd<P: Prop, Q: Prop>(
    pq: Established<And<P, Q>>,
) -> Established<Q> { ... }
```

---

## 5. DERIVE MACRO: #[derive(Elicit)]

### File: `/home/erik/repos/elicitation/crates/elicitation_derive/src/derive_elicit.rs`

The `#[derive(Elicit)]` macro expands to:

1. **Elicitation impl** - implements the main `elicit()` method
2. **ElicitIntrospect impl** - provides `metadata()`
3. **Prompt impl** - provides custom prompts
4. **TypeGraphKey registration** - registers with type graph (if `graph` feature)

### What Does "Eliciting a Move" Mean?

**Context:** This phrase appears in `emit_code.rs` and refers to the **emission strategy**.

**Meaning:** An "elicit move" is an **MCP tool call** that the agent makes during workflow execution. When the agent calls `workflow__fetch`, it's making an "elicit move" in the workflow state space.

The `emit_code` system captures these moves and **recovers Rust source** that would reproduce them:

```
Agent calls:
  workflow__fetch({ url: "https://api.example.com", timeout_secs: 30 })

Emitted Rust:
  let (fetch_result, proof) = WorkflowPlugin::fetch(
      ctx,
      FetchParams {
          url: UrlValid::new("https://api.example.com")?,
          timeout_secs: Some(F64Positive::new(30.0)?),
      },
  ).await?;
```

**File:** `/home/erik/repos/elicitation/crates/elicitation/src/emit_code.rs` (lines 1-350)

Core trait:
```rust
pub trait EmitCode {
    /// Emit Rust source for this value (async context, ? available).
    fn emit_code(&self) -> TokenStream;
    
    /// Crate dependencies required by emitted code.
    fn crate_deps(&self) -> Vec<CrateDep> { vec![] }
}
```

---

## 6. EXAMPLE: END-TO-END WORKFLOW

### File: `/home/erik/repos/elicitation/examples/tool_composition.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: u32,
}

#[elicit_tools(ServerConfig, UserInfo)]
#[tool_router]
impl ExampleServer {
    #[tool(description = "Get the current server status")]
    pub async fn status(_peer: Peer<RoleServer>) -> Result<Json<StatusResponse>, ErrorData> {
        Ok(Json(StatusResponse {
            status: "running".to_string(),
            uptime_seconds: 42,
            active_connections: 3,
        }))
    }

    #[tool(description = "Restart the server with a new configuration")]
    pub async fn restart(
        _peer: Peer<RoleServer>,
        config: ServerConfig,  // Agent elicits this!
    ) -> Result<Json<RestartResponse>, ErrorData> {
        Ok(Json(RestartResponse {
            success: true,
            message: format!("Server restarted on {}:{}", config.host, config.port),
        }))
    }
}

// Example workflow:
// 1. LLM calls elicit_server_config (generated by #[elicit_tools] macro)
// 2. Library prompts user: host? port? max_connections?
// 3. User provides: localhost, 8080, 100
// 4. LLM receives validated ServerConfig
// 5. LLM calls restart with the config
// 6. Server restarts with new settings
```

---

## 7. TYPE GRAPH GUIDE

**File:** `/home/erik/repos/elicitation/TYPE_GRAPH_GUIDE.md` (476 lines)

The type graph system is **NOT workflow-specific** but is **crucial for agents to understand domain types**.

### Architecture Layers

```
#[derive(Elicit)]                      (proc macro on types)
        ↓
inventory::submit!(TypeGraphKey)      (zero-cost static registration)
        ↓
TypeGraphKey registry                 (maps type names → metadata)
        ↓
TypeGraph::from_root("Foo")           (BFS traversal of registry)
        ↓
MermaidRenderer / DotRenderer         (render to Mermaid/DOT)
        ↓
CLI + MCP tools + Programmatic API
```

### MCP Tools for Type Graph

Register with `TypeGraphPlugin`:

```rust
let registry = PluginRegistry::new()
    .register("type_graph", TypeGraphPlugin::new());
```

Exposes three tools:
- `type_graph__list_types` - List all registered types
- `type_graph__graph_type` - Render graph for a type
- `type_graph__describe_edges` - Text summary of type edges

### Example Output

For type `ApplicationConfig`:

```
type_graph__describe_edges({ type_name: "ApplicationConfig" })

**ApplicationConfig** (survey, 3 connection(s))

  `network` → `NetworkConfig` [survey]
  `role`    → `Role` [select]
  `timeout` → `Duration` [survey]
```

---

## 8. METRICS & EVENTS: TRACING INSTRUMENTATION

### Tracing Integration

**Pattern:** All workflow tools use `#[instrument]` from the `tracing` crate.

Example from `workflow.rs`:
```rust
#[elicit_tool(name = "fetch", ...)]
#[instrument(skip(ctx, p), fields(url = %p.url.get()))]
async fn wf_fetch(ctx: Arc<PluginContext>, p: FetchParams) -> Result<CallToolResult, ErrorData> {
    // Automatically logs:
    // - Entry with url field
    // - Execution time
    // - Exit with result
}
```

**Tracing Usage in Core Elicitation:**

Collections (`btreemap.rs`, `btreeset.rs`, `hashmap.rs`, etc.) emit:
```rust
tracing::debug!("Eliciting BTreeMap");
tracing::debug!(count = map.len(), "Prompting for additional entry");
tracing::debug!(final_count = map.len(), "Map complete");
tracing::warn!("Key already exists in map");
```

### Workflow Metrics via Tracing

Each tool execution emits:
- **Entry event**: Tool name, input parameters (via span fields)
- **Execution metrics**: Duration (automatic from `#[instrument]`)
- **Exit event**: Success/error status
- **Custom fields**: URL, status code, contract propositions

**Example Trace**:
```
Elapsed time: 0.125s
[DEBUG] elicit_reqwest::workflow: wf_fetch {url: "https://api.example.com"}
  [DEBUG] RequestCompleted proof established
  [DEBUG] StatusSuccess proof established
  [DEBUG] Establishing FetchSucceeded (conjunction of all proofs)
[DEBUG] elicit_reqwest::workflow: wf_fetch exited
```

### No Custom Event Types

The framework doesn't define custom `WorkflowEvent` structs. Instead:
- **Contracts themselves are events** - establishing a proof is an event
- **Tracing spans capture the timeline** - plug into any tracing subscriber
- **Result types carry metadata** - FetchResult includes `contract: "UrlValid ∧ RequestCompleted ∧ StatusSuccess"`

---

## 9. TYPESTATE PATTERN USAGE

### What is Typestate?

Typestate is an **abstract pattern** where the **type encodes the runtime state**. In elicitation:

> **The proposition type (Pre/Post) IS the state.**

### HTTP Workflow Typestate Example

```rust
// State 0: UrlValid not yet established
let url_str = "https://api.example.com";

// State 1: UrlValid established
let url: UrlValidType = UrlValid::parse(url_str)?;
let proof1: Established<UrlValid> = Established::assert();

// State 2: RequestCompleted established
let resp = client.get(url_str).send().await?;
let proof2: Established<RequestCompleted> = Established::assert();

// State 3: StatusSuccess established
if resp.status().is_success() {
    let proof3: Established<StatusSuccess> = Established::assert();
}

// State 4: All proofs combined
let full_proof = both(proof1, both(proof2, proof3)); // FetchSucceeded
```

### Typestate in Contracts

```rust
pub trait Tool {
    type Pre: Prop;   // Input state (type constraint)
    type Post: Prop;  // Output state (type constraint)
}

// Composition enforces state transitions at compile time
let tool1: impl Tool<Pre=True, Post=EmailValidated>;
let tool2: impl Tool<Pre=EmailValidated, Post=EmailSent>;

// Compile error if tool2.Pre ≠ tool1.Post!
let pipeline = then(tool1, tool2);
```

### Typestate + HTTP

```
UrlValid ∧ RequestCompleted ∧ StatusSuccess
             ↑
         FetchSucceeded
         
Can't use FetchResult until all three proofs held (type-checked)
Can't proceed to next step without prior step's proof
```

---

## 10. PLUGIN & MCP INTEGRATION

### File: `/home/erik/repos/elicitation/crates/elicitation/src/plugin/mod.rs`

Core plugin trait:
```rust
pub trait ElicitPlugin: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn list_tools(&self) -> Vec<Tool>;
    fn call_tool<'a>(
        &'a self,
        params: CallToolRequestParams,
        ctx: RequestContext<RoleServer>,
    ) -> BoxFuture<'a, Result<CallToolResult, ErrorData>>;
}
```

### Plugin Registry

```rust
use elicitation::{PluginRegistry, WorkflowPlugin};

let registry = PluginRegistry::new()
    .register("workflow", WorkflowPlugin::default_client())
    .register("type_graph", TypeGraphPlugin::new());
```

All plugins expose tools with namespace prefix:
- `workflow__fetch`
- `workflow__url_build`
- `type_graph__list_types`
- `type_graph__graph_type`

---

## 11. ARCHITECTURAL PATTERNS

### Pattern 1: Contract-Carrying Results

```rust
pub struct FetchResult {
    pub status: u16,
    pub url: String,
    pub body: String,
    pub contract: String,  // Human-readable contract established
}
```

Result carries proof metadata for downstream inspection.

### Pattern 2: Hierarchical Contracts

```
UrlValid                       (single proposition)
RequestCompleted               (single proposition)
StatusSuccess                  (single proposition)
    ↓
And<UrlValid, RequestCompleted>  (conjunction)
    ↓
FetchSucceeded                 (alias for full conjunction)
```

### Pattern 3: Proposition-Based Composition

```rust
// Define workflow as sequence of propositions
struct Step1 -> Established<P1>;
struct Step2(Pre: P1) -> Established<P2>;
struct Step3(Pre: P2) -> Established<P3>;

// Compose: automatically type-safe
let result = then(then(step1, step2), step3);
```

### Pattern 4: Code Recovery via Emission

```
MCP Tool Call (JSON)
  ↓
dispatch_emit() lookup
  ↓
EmitCode impl on params type
  ↓
TokenStream (Rust code)
  ↓
BinaryScaffold assembly
  ↓
Compiled binary
```

---

## 12. FILE STRUCTURE & KEY LOCATIONS

```
/home/erik/repos/elicitation/
├── crates/
│   ├── elicitation/src/
│   │   ├── tool.rs                    # Tool trait, then(), both_tools()
│   │   ├── contracts.rs               # Prop, Established, And, Implies, etc.
│   │   ├── traits.rs                  # Elicitation trait, ElicitIntrospect
│   │   ├── emit_code.rs               # EmitCode, code recovery system
│   │   ├── type_graph/
│   │   │   ├── mod.rs                 # TypeGraph builder
│   │   │   ├── render/mermaid.rs      # MermaidRenderer
│   │   │   └── render/dot.rs          # DotRenderer
│   │   └── plugin/
│   │       ├── mod.rs                 # ElicitPlugin trait
│   │       ├── descriptor.rs          # ToolDescriptor
│   │       └── descriptor_plugin.rs   # DescriptorPlugin blanket impl
│   ├── elicit_reqwest/src/
│   │   ├── plugins/
│   │   │   ├── workflow.rs            # WorkflowPlugin (10 HTTP tools)
│   │   │   ├── request_builder.rs     # RequestBuilder wrapper
│   │   │   └── mod.rs                 # Plugin exports
│   │   └── lib.rs
│   └── elicitation_derive/src/
│       ├── derive_elicit.rs           # #[derive(Elicit)]
│       ├── struct_impl.rs             # Expand for structs
│       ├── enum_impl.rs               # Expand for enums
│       ├── elicit_tool.rs             # #[elicit_tool] macro
│       └── emit_rewriter.rs           # Code emission support
├── examples/
│   ├── tool_composition.rs            # Complete end-to-end example
│   └── tool_composition.md            # Documentation
├── TYPE_GRAPH_GUIDE.md                # Type graph system documentation
├── ELICIT_REQWEST_PLAN.md             # HTTP workflow design
└── README.md                          # Overview & tutorial
```

---

## 13. SUMMARY: FOUR WORKFLOW CONCEPTS

### 1. **Elicitation Workflows** (Type Construction)
- Multi-step construction of domain types
- Each step elicits one field or variant
- Entirely client-driven (agent in charge)

### 2. **Contract Workflows** (Proof Composition)
- Sequence of steps with formal preconditions/postconditions
- Each step establishes a proposition
- Composition checked at compile-time

### 3. **HTTP Workflows** (WorkflowPlugin)
- 10 composable HTTP operations
- Each establishes atomic propositions (UrlValid, RequestCompleted, StatusSuccess)
- Results compose into FetchSucceeded guarantee

### 4. **Code Emission Workflows** (Tool → Rust)
- Agent calls workflow tools
- Each call captured as EmitCode params
- BinaryScaffold assembles into compilable Rust source

---

## 14. WHAT THE TYPE GRAPH GUIDE SAYS ABOUT WORKFLOWS

**Relevance:** TYPE_GRAPH_GUIDE.md doesn't discuss workflows explicitly but is CRITICAL for agents to understand workflow type dependencies.

Key quote from the guide:
> "Without reading source code, you can ask: 'What fields does ApplicationConfig have? What types compose it? How deep does the graph go?'"

**Application to Workflows:**

Agents use type graph to:
1. **Discover composable types** - which types can be elicited?
2. **Understand nestings** - does Config contain NetworkConfig?
3. **Plan elicitation strategy** - what fields need what order?
4. **Browse constraints** - what bounds/validators apply?

Example:
```
Agent calls: type_graph__graph_type({ root: "ServerConfig" })
Response shows: ServerConfig → String, u16, u32
Agent now knows: Must elicit 3 fields, in order
```

---

## 15. VISUALIZATION CAPABILITY

The framework CAN feed a TUI with:

1. **Span-based metrics** (from tracing)
   - Tool entry/exit
   - Execution time per tool
   - Contract establishment sequence

2. **Contract state visualization**
   ```
   Step 1: UrlValid → ✓
   Step 2: RequestCompleted → ✓
   Step 3: StatusSuccess → ✓
   Combined: FetchSucceeded → ✓
   ```

3. **Type graph rendering**
   - Mermaid flowcharts
   - DOT graphs
   - Text edge descriptions

4. **Error propagation**
   ```
   Step 1: ✓ FetchSucceeded
   Step 2: ✗ StatusSuccess not established (got 404)
   Proof chain broken → rollback
   ```

Implementation would:
- Attach a custom tracing subscriber to capture spans
- Render span fields (url, status, contract) in TUI
- Show state machine progression (propositions established)
- Display type graph as sidebar

