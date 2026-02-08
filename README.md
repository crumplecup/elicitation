# Elicitation

> **Teaching agents to think in types, not just fill in forms**

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

## The Problem: JSON Forms vs. Domain Languages

Most MCP servers follow a familiar pattern: expose domain objects as JSON schemas, let agents fill in forms. This works, but it's **backwards**:

```rust
// What most MCP servers do:
// "Here's a User form. Fill it in."
let user = agent.call_tool("create_user", json!({
    "name": "Alice",
    "email": "alice@example.com",
    "age": 30
}));
```

The agent is stuck in JSON-land, translating between natural language and key-value pairs. No understanding of **what** a User actually *is*, no concept of validity beyond "did the JSON match?"

## The Vision: Agents That Speak Your Domain

**Elicitation flips the script.** Instead of forms, you give agents the **building blocks** of your domain—the types, the constraints, the compositional rules—and let them *construct* values through conversation:

```rust
// What elicitation does:
// "Here's how to construct a valid User. Go."
#[derive(Elicit)]
struct User {
    name: String,
    email: Email,  // Not String - Email!
    age: u8,       // Not any number - bounded!
}

// Agent now speaks in User-construction steps:
// 1. Select a name (String elicitation)
// 2. Construct a valid Email (format validation built-in)
// 3. Choose an age (0-255, type-guaranteed)
let user = User::elicit(&sampling_context).await?;
```

The difference? **The agent understands the structure.** It's not filling a form—it's *building* a User through a sequence of typed operations.

## What Is Elicitation?

Elicitation is a Rust library that turns **sampling interactions** (calls to LLMs via MCP) into **strongly-typed domain values**. But it's not just type-safe JSON deserialization—it's a framework for teaching agents to:

1. **Think compositionally** - Build complex types from simpler ones
2. **Respect constraints** - Types encode validity (Email formats, bounded numbers)
3. **Follow processes** - Multi-step construction with step-by-step guidance
4. **Verify formally** - Contracts and composition rules checked at compile time
5. **Adapt contextually** - Swap prompts/styles without changing types

Think of it as **a DSL for agent-driven data construction**, where the "syntax" is your Rust types and the "semantics" are guaranteed by the compiler.

---

## Tutorial: From Simple Values to Complex Domains

### Part 1: The Four Interaction Mechanics

Elicitation provides four fundamental ways agents construct values:

#### 1. **Select** - Choose from finite options

Used for enums, where the agent picks one variant:

```rust
#[derive(Elicit)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// Agent sees: "Select Priority: Low, Medium, High, Critical"
let priority = Priority::elicit(&ctx).await?;
```

**When to use:** Finite choice sets, enum variants, discriminated unions.

#### 2. **Affirm** - Yes/no decisions

Used for booleans:

```rust
// Agent sees: "Affirm: Should this task be urgent? (yes/no)"
let urgent: bool = bool::elicit(&ctx).await?;
```

**When to use:** Binary decisions, flags, opt-in/opt-out.

#### 3. **Survey** - Multi-field construction

Used for structs, where the agent builds each field in sequence:

```rust
#[derive(Elicit)]
struct Task {
    title: String,
    priority: Priority,
    urgent: bool,
}

// Agent follows a 3-step process:
// 1. Provide title (String)
// 2. Select priority (Priority enum)
// 3. Affirm urgency (bool)
let task = Task::elicit(&ctx).await?;
```

**When to use:** Product types, records, multi-field structures.

#### 4. **Authorize** - Permission policies *(future)*

For access control and capability-based security.

**Why these four?** They map to fundamental type constructors: sums (Select), booleans (Affirm), products (Survey), and effects (Authorize). Every Rust type decomposes into these primitives.

---

### Part 2: Compositionality - Types All The Way Down

The power of elicitation is **infinite composition**. Every type that implements `Elicitation` can be nested in any other:

```rust
#[derive(Elicit)]
struct Project {
    name: String,
    tasks: Vec<Task>,  // Nested: elicit multiple tasks
    owner: User,       // Nested: elicit a user
}

#[derive(Elicit)]
struct Organization {
    projects: Vec<Project>,  // Nested: elicit multiple projects
}

// Agent can construct an entire organization structure:
let org = Organization::elicit(&ctx).await?;
```

**This works because:**

- `Vec<T>` implements `Elicitation` if `T` does (recursive elicitation)
- `Option<T>` implements `Elicitation` if `T` does (optional fields)
- Your custom structs implement via `#[derive(Elicit)]`
- Primitives implement it built-in

**No depth limit.** Nest 10 levels deep, 100 fields wide—it composes.

---

### Part 3: Validity Guarantees

Elicitation isn't just data entry—it's **construction with guarantees**. Types encode constraints that the agent must respect:

#### Type-Level Constraints

```rust
use elicitation::bounded::Bounded;

#[derive(Elicit)]
struct Port(
    #[elicit(bounded(1024, 65535))]
    u16
);  // Must be in range 1024-65535

#[derive(Elicit)]
struct Email(
    #[elicit(validator = is_valid_email)]
    String
);  // Must pass validation function
```

#### Contract System (Formal Verification)

Elicitation v0.5.0 introduced **contracts**: type-level proofs that operations maintain invariants.

```rust
use elicitation::contracts::{Prop, Established, And};

// Define propositions (contracts)
struct EmailValidated;
struct ConsentObtained;
impl Prop for EmailValidated {}
impl Prop for ConsentObtained {}

// Function requiring proofs
fn register_user(
    email: String,
    _proof: Established<And<EmailValidated, ConsentObtained>>
) {
    // Compiler guarantees email was validated AND consent obtained
    // No runtime checks needed!
}

// Compose workflow with proofs
let email_proof = validate_email(email)?;
let consent_proof = obtain_consent()?;
let both_proofs = both(email_proof, consent_proof);

register_user(email, both_proofs);  // ✓ Compiles
register_user(email, email_proof);  // ✗ Missing consent proof
```

**Verified with Kani:** 183 symbolic execution checks prove the contract system works correctly. Build multi-step agent workflows with **mathematical guarantees**.

---

### Part 4: Style System - Context-Aware Prompts

Agents need context. The same `Email` type might be elicited differently in different scenarios:

```rust
use elicitation::{Style, Styled};

// Define custom styles for Email
#[derive(Style)]
enum EmailStyle {
    Default,
    WorkEmail,
    PersonalEmail,
}

// Use different prompts based on style
let work_email = Email::elicit_styled(&ctx, EmailStyle::WorkEmail).await?;
// Prompt: "Provide work email address (e.g., name@company.com)"

let personal_email = Email::elicit_styled(&ctx, EmailStyle::PersonalEmail).await?;
// Prompt: "Provide personal email address"
```

**Hot-swapping prompts** without changing types. One `Email` type, multiple presentation contexts. Extensible: define custom styles for **any type**, including built-ins like `String`, `i32`, etc.

---

### Part 5: Generators - Alternate Constructors

Sometimes you need to construct values in different ways. Elicitation provides **generators** for alternate construction paths.

**Real-world example:** `std::time::Instant` has a `now()` generator:

```rust
use std::time::Instant;

// Option 1: Agent provides manual timing (default elicitation)
let instant1 = Instant::elicit(&ctx).await?;

// Option 2: Use generator to capture current time
let instant2 = Instant::elicit_with_generator(&ctx, "now").await?;
// Equivalent to: Instant::now()
```

**Why this matters:** Some types have natural "smart constructors" that don't require user input:
- `Instant::now()` - Current timestamp
- `SystemTime::now()` - Current system time  
- `Uuid::new_v4()` - Random UUID
- Factory patterns with defaults

**Custom generators:**

```rust
#[derive(Elicit)]
#[elicit(generators = [from_template, from_env])]
struct Config {
    host: String,
    port: u16,
}

// Agent can choose:
// 1. from_template: Start with defaults
// 2. from_env: Load from environment variables
// 3. (default): Build each field manually
```

**Use cases:**
- Smart constructors (now(), random(), default())
- Environment-based initialization
- Template expansion
- Multi-stage construction

---

### Part 6: Trait-Based MCP Tools (v0.6.0+)

For more complex systems, you might have trait-based APIs. Elicitation supports **automatic tool generation** from traits:

```rust
use elicitation::elicit_trait_tools_router;

#[async_trait]
trait TaskManager: Send + Sync {
    async fn create_task(
        &self,
        params: Parameters<CreateTaskParams>,
    ) -> Result<Json<Task>, ErrorData>;
    
    async fn list_tasks(
        &self,
        params: Parameters<ListParams>,
    ) -> Result<Json<Vec<Task>>, ErrorData>;
}

// Automatically generate MCP tools from trait methods
#[elicit_trait_tools_router(TaskManager, manager, [create_task, list_tasks])]
#[tool_router(router = task_tools)]
impl TaskService {}
```

**Why this matters:**

- Expose entire trait-based APIs as MCP tools
- 80-90% less boilerplate (no manual wrapper functions)
- Supports `async_trait` for object safety (trait objects work!)
- Compose regular tools with elicitation tools seamlessly

---

## The Complete Picture: Agent-Native Domain Languages

Here's what you get when you use elicitation:

1. **Types as Specifications**
   - Your Rust types define *what* is valid
   - The compiler checks correctness
   - Agents see structured operations, not key-value forms

2. **Compositionality as Architecture**
   - Build complex systems from simple pieces
   - Nest types arbitrarily deep
   - Reuse elicitation logic across your domain

3. **Contracts as Guarantees**
   - Express invariants as type-level proofs
   - Compose workflows with verified properties
   - Catch logic errors at compile time, not runtime

4. **Styles as Adaptation**
   - Same types, different contexts
   - Hot-swap prompts without code changes
   - Customize presentation per use case

5. **Verification as Confidence**
   - Formally verified with Kani model checker
   - 183 symbolic checks prove correctness
   - Zero-cost abstractions (proofs compile away)

**The result?** Agents that don't just fill forms—they **construct valid domain values through typed operations**. They speak your domain language, follow your invariants, and produce verified outputs.

---

## Quick Start

### Installation

```toml
[dependencies]
elicitation = "0.6"
rmcp = "0.14"  # Rust MCP SDK
tokio = { version = "1", features = ["full"] }
```

### Basic Example

```rust
use elicitation::Elicit;
use rmcp::client::Client;

#[derive(Debug, Elicit)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Elicit)]
struct Task {
    title: String,
    priority: Priority,
    urgent: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to MCP server (Claude Desktop, CLI, etc.)
    let client = Client::stdio().await?;
    
    // Elicit a complete task from the agent
    let task = Task::elicit(&client).await?;
    
    println!("Created task: {:?}", task);
    Ok(())
}
```

Run with Claude Desktop or CLI:

```bash
cargo run --example basic_task
# or
claude "Run the basic_task example"
```

---

## Requirements and Constraints

### Required Derives

All types using `#[derive(Elicit)]` **must** implement three traits:

```rust
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use elicitation::Elicit;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct Task {
    title: String,
    priority: Priority,
}
```

**Why each derive is required:**

- **`Serialize`** - Convert Rust values to JSON for MCP responses
- **`Deserialize`** - Parse agent selections back into Rust types
- **`JsonSchema`** - Generate JSON schemas for MCP tool definitions
- **`Elicit`** - Generate the elicitation logic (our derive macro)

**Optional but recommended:**
- **`Debug`** - For printing/logging during development
- **`Clone`** - Many async patterns need cloneable values

### Field Type Constraints

All field types in your structs must **also** implement `Elicitation`:

```rust
// ✅ VALID: All fields implement Elicitation
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct User {
    name: String,           // ✅ stdlib type
    age: u8,                // ✅ stdlib type
    email: Option<String>,  // ✅ Option<T> where T: Elicitation
    tags: Vec<String>,      // ✅ Vec<T> where T: Elicitation
}

// ❌ INVALID: CustomEmail doesn't implement Elicitation
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct User {
    name: String,
    email: CustomEmail,  // ❌ Compile error!
}

// ✅ FIX: Derive Elicit for nested types
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct CustomEmail(String);

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct User {
    name: String,
    email: CustomEmail,  // ✅ Now works!
}
```

### Common Pitfalls

#### 1. Missing JsonSchema on Nested Types

```rust
// ❌ BAD: Address missing JsonSchema
#[derive(Serialize, Deserialize)]
struct Address { /* ... */ }

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct User {
    address: Address,  // ❌ Compile error: no JsonSchema for Address
}

// ✅ GOOD: Add JsonSchema to all nested types
#[derive(Serialize, Deserialize, JsonSchema)]
struct Address { /* ... */ }
```

#### 2. Generic Types Need Bounds

```rust
// ❌ BAD: Missing trait bounds
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct Container<T> {
    value: T,  // ❌ T might not implement required traits
}

// ✅ GOOD: Add proper bounds
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct Container<T>
where
    T: Serialize + Deserialize + JsonSchema + Elicitation,
{
    value: T,  // ✅ Guaranteed to work
}
```

#### 3. Enums Must Have Serde Attributes

```rust
// ❌ BAD: Complex enum variants without serde tags
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
enum Status {
    Pending,
    Active { since: String },
    Completed { at: String, by: String },
}

// ✅ GOOD: Add serde tagging for complex enums
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
#[serde(tag = "type")]
enum Status {
    Pending,
    Active { since: String },
    Completed { at: String, by: String },
}
```

#### 4. PhantomData Needs Skip

```rust
// ✅ GOOD: Skip non-serializable fields
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
struct TypedId<T> {
    id: String,
    #[serde(skip)]
    _phantom: PhantomData<T>,
}
```

### Trait Tools Requirements

When using `#[elicit_trait_tools_router]`, parameter and result types need the same derives:

```rust
// Tool parameter types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskParams {
    title: String,
    priority: Priority,
}

// Tool result types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateTaskResult {
    id: String,
    created: bool,
}
```

**Note:** These don't need `Elicit` derive (they're not elicited, just passed as JSON).

### Async Requirements

Traits using `#[elicit_trait_tools_router]` need proper async signatures:

```rust
// Pattern 1: impl Future + Send (zero-cost)
trait MyTrait: Send + Sync {
    fn method(&self, params: Parameters<P>) 
        -> impl Future<Output = Result<Json<R>, ErrorData>> + Send;
}

// Pattern 2: async_trait (object-safe)
#[async_trait]
trait MyTrait: Send + Sync {
    async fn method(&self, params: Parameters<P>) 
        -> Result<Json<R>, ErrorData>;
}
```

See [ELICIT_TRAIT_TOOLS_ROUTER.md](ELICIT_TRAIT_TOOLS_ROUTER.md) for complete details.

### Quick Checklist

Before deriving `Elicit`:

- [ ] Type has `Serialize + Deserialize + JsonSchema`
- [ ] All field types implement `Elicitation`
- [ ] Nested types have all required derives
- [ ] Generic types have proper bounds
- [ ] Complex enums have serde tagging
- [ ] PhantomData fields are marked `#[serde(skip)]`

---

## Architecture

### The Elicitation Trait

The core abstraction:

```rust
#[async_trait]
pub trait Elicitation: Sized {
    /// Elicit a value through sampling interaction
    async fn elicit(ctx: &SamplingContext) -> Result<Self, ElicitError>;
}
```

Every type that implements this trait can be constructed through agent interaction. The derive macro generates the implementation automatically.

### How It Works

1. **At compile time:** `#[derive(Elicit)]` generates:
   - `Elicitation` trait implementation
   - MCP tool definitions (JSON schemas)
   - Prompt templates for each field
   - Validation logic

2. **At runtime:** Agent calls `Type::elicit()`:
   - Library presents structured prompts to agent
   - Agent responds with selections/values
   - Library validates responses against type constraints
   - Process repeats for nested types (recursively)

3. **Result:** Fully constructed, type-checked domain value.

### Supported Types (100+ stdlib types)

**Primitives:** `bool`, `i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `char`, `String`  
**Collections:** `Vec<T>`, `Option<T>`, `Result<T, E>`, `[T; N]`  
**Network:** `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`  
**Filesystem:** `PathBuf`, `Path`  
**Time:** `Duration`, `SystemTime`, `Instant`  
**DateTime:** `chrono`, `time`, `jiff` (3 major datetime libraries)  
**Data:** `serde_json::Value` (dynamic JSON construction)  
**Smart Pointers:** `Box<T>`, `Arc<T>`, `Rc<T>`  
**...and more**

Plus: **Any custom type** via `#[derive(Elicit)]`

---

## Advanced Features

### Feature Flags

```toml
[dependencies]
elicitation = { version = "0.6", features = [
    "chrono",         # chrono datetime types
    "time",           # time datetime types
    "jiff",           # jiff datetime types
    "serde_json",     # JSON value elicitation
    "contracts",      # Contract system
    "verification",   # Kani formal verification
] }
```

### JSON Schema Generation

All elicited types automatically generate JSON schemas for MCP:

```rust
use schemars::JsonSchema;

#[derive(Elicit, JsonSchema)]
struct Config {
    timeout: u32,
}

// Schema is automatically registered with MCP server
```

### Datetime Support

Three major datetime libraries supported:

```rust
// chrono
use chrono::{DateTime, Utc};
let timestamp: DateTime<Utc> = DateTime::elicit(&ctx).await?;

// time
use time::OffsetDateTime;
let time: OffsetDateTime = OffsetDateTime::elicit(&ctx).await?;

// jiff
use jiff::Timestamp;
let jiff_time: Timestamp = Timestamp::elicit(&ctx).await?;
```

### Dynamic JSON Construction

Agents can build arbitrary JSON structures:

```rust
use serde_json::Value;

// Agent constructs JSON interactively
let json: Value = Value::elicit(&ctx).await?;
// Could be: {"name": "Alice", "scores": [95, 87, 92]}
```

---

## Documentation

- **[API Docs](https://docs.rs/elicitation)** - Complete API reference
- **[ELICIT_TRAIT_TOOLS_ROUTER.md](ELICIT_TRAIT_TOOLS_ROUTER.md)** - Trait-based tool generation guide
- **[TOOL_ROUTER_WARNINGS.md](TOOL_ROUTER_WARNINGS.md)** - Addressing rmcp warnings
- **[MIGRATION_0.5_to_0.6.md](MIGRATION_0.5_to_0.6.md)** - Upgrade guide
- **[Examples](examples/)** - 20+ working examples

---

## Why Elicitation?

### For Library Authors

**Expose your entire domain as agent-native operations:**

- One `#[derive(Elicit)]` per type → instant MCP tools
- Agents construct domain values, not JSON blobs
- Type safety = correctness guarantees
- Composition = reusable building blocks

### For Agent Developers

**Stop wrestling with JSON forms:**

- Structured operations > unstructured key-value
- Type-driven exploration (what's valid?)
- Multi-step processes with clear semantics
- Formal verification catches bugs the LLM can't

### For System Architects

**Build verified agent systems:**

- Contracts express invariants precisely
- Composition rules checked at compile time
- Kani verification gives mathematical confidence
- Zero-cost abstractions = production-ready performance

---

## Comparison: Before vs. After

### Traditional MCP (JSON-Centric)

```rust
// Server exposes a form
let schema = json!({
    "type": "object",
    "properties": {
        "title": {"type": "string"},
        "priority": {"enum": ["Low", "Medium", "High"]},
        "urgent": {"type": "bool"}
    }
});

// Agent fills it in (one shot, hope for the best)
let response = agent.call_tool("create_task", json!({
    "title": "Fix bug",
    "priority": "Hgih",  // Typo! Fails validation
    "urgent": true
}));
```

**Problems:**

- Agent guesses field names/values
- Validation happens late (after submission)
- No guidance on nested structures
- No type safety, no composition

### Elicitation (Type-Centric)

```rust
#[derive(Elicit)]
enum Priority { Low, Medium, High }

#[derive(Elicit)]
struct Task {
    title: String,
    priority: Priority,
    urgent: bool,
}

// Agent constructs through typed operations
let task = Task::elicit(&ctx).await?;
// 1. Provide title (String elicitation)
// 2. Select priority from {Low, Medium, High}  ← No typos possible
// 3. Affirm urgency (yes/no)
```

**Benefits:**

- Agent guided step-by-step
- Validation built into types
- Errors impossible to construct
- Composable, reusable, verified

---

## Formal Verification

Elicitation's contract system is verified with [Kani](https://github.com/model-checking/kani), Amazon's Rust model checker:

```bash
just verify-kani  # Run 183 symbolic execution checks
```

**What's verified:**

- Contract composition (sequential and parallel)
- Proof forwarding and combination
- Type-level guarantee preservation
- Zero-cost abstraction (proofs compile to nothing)

See [VERIFICATION_FRAMEWORK_DESIGN.md](VERIFICATION_FRAMEWORK_DESIGN.md) for details.

---

---

## Contributing

We welcome contributions! Areas of interest:

- **New stdlib type support** - More types = more expressiveness
- **Style system extensions** - Custom styles for domain-specific contexts
- **Verification coverage** - More Kani proofs = more confidence
- **Documentation** - Examples, tutorials, guides
- **MCP integration** - Better tooling, better DX

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

## Acknowledgments

Built on:

- [rmcp](https://github.com/zed-industries/mcp) - Rust MCP SDK by Zed Industries
- [Kani](https://github.com/model-checking/kani) - Rust model checker by Amazon
- [Model Context Protocol](https://modelcontextprotocol.io) - Anthropic's agent communication standard

Special thanks to the Rust community for creating the type system that makes this possible.

---

**Elicitation: Where types meet agents, and agents learn to think in types.** 🎯
