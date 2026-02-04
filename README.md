# Elicitation

> Conversational elicitation of strongly-typed Rust values via MCP

[![Crates.io](https://img.shields.io/crates/v/elicitation.svg)](https://crates.io/crates/elicitation)
[![Documentation](https://docs.rs/elicitation/badge.svg)](https://docs.rs/elicitation)
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Downloads](https://img.shields.io/crates/d/elicitation.svg)](https://crates.io/crates/elicitation)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)

`elicitation` is a Rust library that transforms conversational LLM interactions into strongly-typed Rust values through the Model Context Protocol (MCP). It provides a trait-based system for eliciting primitive types, enums, structs, and nested data structures through natural language interaction.

## Features

- **Comprehensive Type Coverage** - Elicit virtually any Rust standard library type
- **Trait-Based Design** - Simple `Elicitation` trait for all types
- **Derive Macros** - Zero-boilerplate implementation for custom types
- **Type-Safe** - Compile-time guarantees with full type inference
- **Composable** - Nest types arbitrarily deep without limitations
- **Async-First** - Built on tokio with proper Send bounds
- **Four Interaction Paradigms**:
  - **Select** - Choose from finite options (enum pattern)
  - **Affirm** - Yes/no confirmation (bool pattern)
  - **Survey** - Multi-field elicitation (struct pattern)
  - **Authorize** - Permission policies (planned for v0.2.0)
- **MCP Integration** - Uses official rmcp (Rust MCP SDK) for communication
- **🆕 Style System v2** - Revolutionary type-safe style system (v0.4.0)
  - Every type has associated Style enum for compile-time registration
  - Runtime style selection with full type safety
  - Inline elicitation for all primitives with styled prompts
  - Auto-selection with silent defaults (zero ceremony)
  - Ergonomic builder pattern for one-off overrides
  - Extensible: define custom styles for any type (including built-ins)
- **🆕 DateTime Support** - Three major datetime libraries (v0.4.0)
  - `chrono` - DateTime<Utc>, DateTime<FixedOffset>, NaiveDateTime
  - `time` - OffsetDateTime, PrimitiveDateTime
  - `jiff` - Timestamp, Zoned, civil::DateTime
  - Dual input methods: ISO 8601 strings or manual components
- **🆕 JSON Elicitation** - Dynamic JSON value construction (v0.4.0)
  - `serde_json::Value` with all JSON types
  - Recursive elicitation for arrays and objects
  - Depth limits to prevent infinite recursion
- **🆕 Proof-Carrying Composition** - Formally verified agent programs (v0.5.0)
  - Type-level contracts (preconditions/postconditions)
  - Zero-cost proof markers (compile away completely)
  - Sequential and parallel tool composition
  - Verified with Kani model checker (183 symbolic checks)
  - Build multi-step workflows with mathematical guarantees

## Proof-Carrying Composition

**Build verified agent programs, not just validated JSON.**

Elicitation's contract system lets you compose multi-step agent workflows where each step's guarantees are **formally verified** at compile time:

```rust
use elicitation::{
    contracts::{Prop, Established, And, both},
    tool::{Tool, True, then},
};

// Define workflow propositions
struct EmailValidated;
struct ConsentObtained;
impl Prop for EmailValidated {}
impl Prop for ConsentObtained {}

// Functions requiring proofs
fn register_user(
    email: String,
    _proof: Established<And<EmailValidated, ConsentObtained>>
) {
    // Type system guarantees:
    // - Email was validated
    // - Consent was obtained
    // No runtime checks needed!
}

// Compose workflow
let email_proof = validate_email("user@example.com")?;
let consent_proof = get_consent()?;
let combined = both(email_proof, consent_proof);
register_user(email, combined);  // ✅ Type-checked!

// register_user(email, ...);  // ❌ Won't compile without proofs
```

**Why this matters:**
- Traditional approach: Hope validation happens, test extensively
- Contract approach: Type system prevents using unvalidated data
- **Zero cost**: All proofs compile away (formally verified with Kani)

See the [contracts module documentation](https://docs.rs/elicitation/latest/elicitation/contracts/) and [examples](crates/elicitation/examples/) for complete workflows.

## Supported Types

### Primitives
- **Numeric**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `f32`, `f64`
- **Text**: `String`, `bool`
- **Time**: `std::time::Duration`
- **Filesystem**: `std::path::PathBuf`
- **Network**: `IpAddr`, `Ipv4Addr`, `Ipv6Addr`, `SocketAddr`, `SocketAddrV4`, `SocketAddrV6`
- **DateTime** (optional features):
  - `chrono` feature: `DateTime<Utc>`, `DateTime<FixedOffset>`, `NaiveDateTime`
  - `time` feature: `OffsetDateTime`, `PrimitiveDateTime`
  - `jiff` feature: `Timestamp`, `Zoned`, `civil::DateTime`
- **UUID** (optional feature):
  - `uuid` feature: `Uuid` with parsing and random generation
- **JSON** (optional feature):
  - `serde_json` feature: `serde_json::Value` (all JSON types)

### Containers
- **Option**: `Option<T>` - Optional values
- **Result**: `Result<T, E>` - Success/failure outcomes
- **Vec**: `Vec<T>` - Dynamic arrays
- **Arrays**: `[T; N]` - Fixed-size arrays (any size N)
- **Tuples**: `(T1, T2, ...)` - Heterogeneous tuples (up to arity 12)

### Smart Pointers
- **Box**: `Box<T>` - Heap allocation
- **Rc**: `Rc<T>` - Reference counting
- **Arc**: `Arc<T>` - Atomic reference counting

### Collections
- **HashMap**: `HashMap<K, V>` - Hash-based key-value map with duplicate key handling
- **BTreeMap**: `BTreeMap<K, V>` - Ordered key-value map
- **HashSet**: `HashSet<T>` - Hash-based unique set with automatic deduplication
- **BTreeSet**: `BTreeSet<T>` - Ordered unique set
- **VecDeque**: `VecDeque<T>` - Double-ended queue
- **LinkedList**: `LinkedList<T>` - Doubly-linked list

### Custom Types
- **Enums**: Automatic `Select` paradigm via `#[derive(Elicit)]`
- **Structs**: Automatic `Survey` paradigm via `#[derive(Elicit)]`
- **Nested**: Unlimited nesting depth (e.g., `Vec<Option<Result<HashMap<String, Arc<T>>, E>>>`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elicitation = "0.3"
rmcp = "0.12"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Optional Features

Enable additional functionality with feature flags:

```toml
[dependencies]
elicitation = { version = "0.2", features = ["chrono", "time", "jiff", "serde_json"] }
```

Available features:
- `chrono` - Enable chrono datetime types
- `time` - Enable time datetime types  
- `jiff` - Enable jiff datetime types
- `serde_json` - Enable JSON Value elicitation
- `api` - Empty marker for API integration tests

## MCP Setup

This library requires an **MCP client** (like Claude Desktop or Claude CLI) to provide the elicitation tools. Your application runs as an **MCP server** that the client invokes.

### Running with Claude CLI

To run the examples or your own code:

```bash
# Install Claude CLI if you haven't already
# (see https://docs.anthropic.com/en/docs/agents-and-tools)

# Run an example through Claude CLI
claude-cli mcp add elicitation-demo --command "cargo run --example structs"

# Or ask Claude to run it
claude "Run the structs example from the elicitation crate"
```

### Integration with Claude Desktop

Add your MCP server to Claude Desktop's configuration:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "elicitation-app": {
      "command": "/path/to/your/binary",
      "args": [],
      "env": {}
    }
  }
}
```

### How It Works

1. Your application creates an MCP client with `rmcp::transport::stdio()`
2. Claude (the MCP client) provides elicitation tools via stdin/stdout
3. When you call `.elicit()`, it sends tool requests to Claude
4. Claude prompts the user and validates responses
5. Your code receives strongly-typed Rust values

**Note**: Examples won't work standalone - they must be invoked by an MCP client.

## Quick Start

```rust
use elicitation::{Elicit, Elicitation, ElicitResult};
use rmcp::ServiceExt;

// Derive for enums (Select pattern)
#[derive(Debug, Elicit)]
#[prompt("Choose your priority level:")]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// Derive for structs (Survey pattern)
#[derive(Debug, Elicit)]
struct Task {
    #[prompt("What's the task title?")]
    title: String,

    #[prompt("Describe the task:")]
    description: String,

    priority: Priority,

    #[prompt("Estimated hours (optional):")]
    estimated_hours: Option<i32>,
}

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // Create MCP client via stdio transport
    let client = ()
        .serve(rmcp::transport::stdio())
        .await?;

    // Elicit a complete task from the user
    let task = Task::elicit(&client).await?;

    println!("Created task: {:?}", task);

    Ok(())
}
```

## Examples

All examples require an MCP client (Claude Desktop or Claude CLI) to run. See [MCP Setup](#mcp-setup) above.

### Primitive Types

```rust
// Elicit basic Rust types
let age: i32 = i32::elicit(&client).await?;
let name: String = String::elicit(&client).await?;
let confirmed: bool = bool::elicit(&client).await?;
let nickname: Option<String> = Option::<String>::elicit(&client).await?;
let scores: Vec<i32> = Vec::<i32>::elicit(&client).await?;

// Result types for success/failure outcomes
let operation: Result<String, i32> = Result::elicit(&client).await?;
```

**Try it**: `claude "Run the simple_types example"` or `claude "Run the result example"`

### Filesystem Paths

```rust
use std::path::PathBuf;

// Elicit a filesystem path
let file_path: PathBuf = PathBuf::elicit(&client).await?;

// Optional paths work too
let config_path: Option<PathBuf> = Option::<PathBuf>::elicit(&client).await?;
```

**Try it**: `claude "Run the pathbuf example"`

### Network Addresses

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

// Elicit IP addresses with automatic validation
let ip: IpAddr = IpAddr::elicit(&client).await?; // IPv4 or IPv6
let ipv4: Ipv4Addr = Ipv4Addr::elicit(&client).await?; // IPv4 only
let ipv6: Ipv6Addr = Ipv6Addr::elicit(&client).await?; // IPv6 only

// Socket addresses (IP + port)
let socket: SocketAddr = SocketAddr::elicit(&client).await?;
```

**Try it**: `claude "Run the network example"`

### Time Durations

```rust
use std::time::Duration;

// Elicit duration in seconds (supports decimals)
let timeout: Duration = Duration::elicit(&client).await?;

// Works with optional durations
let cache_ttl: Option<Duration> = Option::<Duration>::elicit(&client).await?;

// Collections of durations
let intervals: Vec<Duration> = Vec::<Duration>::elicit(&client).await?;
```

**Try it**: `claude "Run the duration example"`

### Enums (Select Pattern)

#### Unit Variants (Simple Selection)

```rust
#[derive(Debug, Elicit)]
enum Status {
    Pending,
    InProgress,
    Completed,
}

let status = Status::elicit(&client).await?;
```

#### Tuple Variants (Select + Field Elicitation)

```rust
#[derive(Debug, Elicit)]
enum MediaSource {
    Url(String),
    Base64(String),
    Binary(Vec<u8>),
}

// User first selects variant (Url/Base64/Binary), then provides the field value
let source = MediaSource::elicit(&client).await?;
```

#### Struct Variants (Select + Multi-Field Survey)

```rust
#[derive(Debug, Elicit)]
enum Input {
    Text(String),
    Image {
        mime: Option<String>,
        source: MediaSource,
    },
    Document {
        format: String,
        content: String,
    },
}

// User selects variant, then provides each field
let input = Input::elicit(&client).await?;
```

#### Mixed Variants

All three variant types can coexist in the same enum:

```rust
#[derive(Debug, Elicit)]
enum Status {
    Ok,                                     // Unit variant
    Warning(String),                        // Tuple variant
    Error { code: i32, msg: String },      // Struct variant
}

let status = Status::elicit(&client).await?;
```

**Try it**: `claude "Run the enums example"`

### Structs (Survey Pattern)

```rust
#[derive(Debug, Elicit)]
struct Person {
    #[prompt("What's your name?")]
    name: String,

    #[prompt("How old are you?")]
    age: u8,

    #[prompt("What's your email?")]
    email: String,
}

let person = Person::elicit(&client).await?;
```

**Try it**: `claude "Run the structs example"`

### Complex Nested Types

```rust
#[derive(Debug, Elicit)]
struct Project {
    name: String,
    team: Vec<Member>,
    tasks: Vec<Task>,
    budget: Option<f64>,
}

let project = Project::elicit(&client).await?;
```

**Try it**: `claude "Run the complex_survey example"`

### Collections

```rust
use std::collections::{HashMap, HashSet};

// Elicit a HashMap with duplicate key handling
let scores: HashMap<String, i32> = HashMap::elicit(&client).await?;

// Elicit a HashSet with automatic deduplication
let tags: HashSet<String> = HashSet::elicit(&client).await?;

// BTreeMap and BTreeSet also supported for ordered collections
use std::collections::{BTreeMap, BTreeSet};
let config: BTreeMap<String, String> = BTreeMap::elicit(&client).await?;
let priorities: BTreeSet<i32> = BTreeSet::elicit(&client).await?;

// VecDeque and LinkedList for sequential access patterns
use std::collections::{VecDeque, LinkedList};
let queue: VecDeque<String> = VecDeque::elicit(&client).await?;
let linked: LinkedList<i32> = LinkedList::elicit(&client).await?;
```

**Try it**: `claude "Run the collections example"`

### DateTime Types (Feature Flags)

Enable datetime support with feature flags: `chrono`, `time`, or `jiff`.

#### Chrono

```rust
use chrono::{DateTime, Utc, FixedOffset, NaiveDateTime};

// Elicit timezone-aware datetime (UTC)
let timestamp: DateTime<Utc> = DateTime::<Utc>::elicit(&client).await?;

// Elicit timezone-aware datetime (with offset)
let event: DateTime<FixedOffset> = DateTime::<FixedOffset>::elicit(&client).await?;

// Elicit naive datetime (no timezone)
let meeting: NaiveDateTime = NaiveDateTime::elicit(&client).await?;
```

#### Time

```rust
use time::{OffsetDateTime, PrimitiveDateTime};

// Elicit timezone-aware datetime
let event: OffsetDateTime = OffsetDateTime::elicit(&client).await?;

// Elicit datetime without timezone
let schedule: PrimitiveDateTime = PrimitiveDateTime::elicit(&client).await?;
```

#### Jiff

```rust
use jiff::{Timestamp, Zoned, civil::DateTime};

// Elicit Unix timestamp
let ts: Timestamp = Timestamp::elicit(&client).await?;

// Elicit timezone-aware datetime (DST-aware!)
let event: Zoned = Zoned::elicit(&client).await?;

// Elicit calendar datetime (no timezone)
let meeting: DateTime = DateTime::elicit(&client).await?;
```

**Dual input methods**: All datetime types support both ISO 8601 strings OR manual component entry (year, month, day, etc.).

### JSON Values (Feature Flag)

Enable with `serde_json` feature flag to elicit dynamic JSON structures:

```rust
use serde_json::Value;

// Elicit any JSON value (null, bool, number, string, array, object)
let config: Value = Value::elicit(&client).await?;

// Works with nesting
let nested: Vec<Value> = Vec::<Value>::elicit(&client).await?;
let optional: Option<Value> = Option::<Value>::elicit(&client).await?;
```

The elicitation process handles all JSON types recursively:
- `null` - Explicit null value
- `bool` - Boolean true/false
- `number` - Any JSON number
- `string` - Text value
- `array` - List of JSON values (recursive)
- `object` - Key-value map (recursive)

Depth limit of 10 prevents infinite recursion.

### Style System 🎨 (v0.2.2)

**Revolutionary feature**: Customize prompts per field with multiple styles!

```rust
#[derive(Debug, Elicit)]
struct Config {
    // Multiple prompt styles for same field
    #[prompt("Name", style = "curt")]
    #[prompt("What is your full name?", style = "verbose")]
    name: String,
    
    #[prompt("Age?", style = "curt")]
    #[prompt("Please enter your age in years", style = "verbose")]
    age: u32,
    
    // Mix styled and default prompts
    #[prompt("Enter city")]  // Used when style doesn't have override
    #[prompt("City", style = "curt")]
    city: String,
}
```

**How it works**:
1. Collect unique style names from all `#[prompt(..., style = "name")]` attributes
2. Generate `ConfigElicitStyle` enum with `Default` + collected styles
3. At runtime, LLM or user selects style (just another Select elicitation!)
4. Each field uses its style-specific prompt (or falls back to default)

**Style selection is a state machine step** - irrelevant whether LLM or user chooses. The style system separates *what to ask* (behavior) from *how to ask* (presentation).

**Built-in styles available** (for programmatic use):
- `DefaultStyle` - Standard prompts
- `CompactStyle` - Terse, minimal prompts
- `VerboseStyle` - Detailed, explanatory prompts  
- `WizardStyle` - Step-by-step with progress

Currently, only `String` fields support styled prompts with inline elicitation. Other types fall back to default elicitation (support expanding in future versions).

## Interaction Paradigms

### Select

For choosing from a finite set of options (enums):

```rust
#[derive(Elicit)]
#[prompt("Choose your programming language:")]
enum Language {
    Rust,
    Python,
    JavaScript,
}
```

### Affirm

For yes/no questions (booleans):

```rust
let confirmed: bool = bool::elicit(&client).await?;
```

### Survey

For multi-field data collection (structs):

```rust
#[derive(Elicit)]
#[prompt("Let's create your profile:")]
struct Profile {
    name: String,
    age: u8,
    bio: Option<String>,
}
```

### Authorize

Permission-based elicitation (planned for v0.2.0).

## Attributes

### `#[prompt("...")]`

Customize prompts for types or fields:

```rust
#[derive(Elicit)]
#[prompt("Configure your account:")] // Struct-level prompt
struct Account {
    #[prompt("Choose a username:")] // Field-level prompt
    username: String,
}
```

### `#[skip]`

Skip fields during elicitation (uses `Default::default()`):

```rust
#[derive(Default, Elicit)]
struct Task {
    title: String,

    #[skip] // Not elicited, uses Default
    created_at: DateTime<Utc>,
}
```

## Error Handling

The library provides rich error handling with location tracking:

```rust
use elicitation::{ElicitError, ElicitErrorKind, ElicitResult};

match Task::elicit(&client).await {
    Ok(task) => println!("Created: {:?}", task),
    Err(e) => eprintln!("Error: {}", e),
}
```

Error types:

- `InvalidFormat` - Parsing failed
- `OutOfRange` - Value outside valid range
- `InvalidOption` - Invalid enum selection
- `MissingField` - Required field missing
- `Cancelled` - User cancelled operation
- `Mcp` - MCP protocol error
- `Json` - JSON parsing error

## Architecture

### Traits

- `Prompt` - Provides prompt text for a type
- `Elicit` - Implements elicitation logic
- `Select` - For enum types (finite choices)
- `Affirm` - For boolean types (yes/no)
- `Survey` - For struct types (multi-field)

### Type Composition

All elicitation types compose freely:

```rust
// Nested structures
let data: Vec<Option<Task>> = Vec::elicit(&client).await?;

// Complex hierarchies
#[derive(Elicit)]
struct Organization {
    name: String,
    departments: Vec<Department>,
}

#[derive(Elicit)]
struct Department {
    name: String,
    members: Vec<Member>,
    projects: Vec<Project>,
}
```

## MCP Integration

The library uses the official [rmcp](https://crates.io/crates/rmcp) (Rust MCP SDK) for MCP communication:

```rust
use rmcp::ServiceExt;

// Create client via stdio transport (for Claude Desktop/CLI)
let client = ()
    .serve(rmcp::transport::stdio())
    .await?;

// Use with elicitation
let value = MyType::elicit(&client).await?;
```

## Documentation

- [API Documentation](https://docs.rs/elicitation)
- [Examples](./crates/elicitation/examples/)
- [MCP Protocol](https://modelcontextprotocol.io)
- [rmcp crate](https://docs.rs/rmcp)

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development

```bash
# Run all checks
just check-all

# Run tests
cargo test --all

# Check examples
cargo check --examples

# Build documentation
cargo doc --open
```

## Versioning

This project follows [Semantic Versioning](https://semver.org/).

Current version: **0.2.0**

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

- Built with [rmcp](https://crates.io/crates/rmcp) - the official Rust MCP SDK
- Powered by [tokio](https://tokio.rs) for async runtime
- Uses [tracing](https://github.com/tokio-rs/tracing) for observability
