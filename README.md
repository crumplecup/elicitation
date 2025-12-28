# Elicitation

> Conversational elicitation of strongly-typed Rust values via MCP

[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)

`elicitation` is a Rust library that transforms conversational LLM interactions into strongly-typed Rust values through the Model Context Protocol (MCP). It provides a trait-based system for eliciting primitive types, enums, structs, and nested data structures through natural language interaction.

## Features

- **Trait-Based Elicitation** - Simple `Elicitation` trait for all types
- **Derive Macros** - Automatic implementation for enums and structs
- **Primitive Types** - Built-in support for integers, floats, booleans, strings, paths, network addresses
- **Containers** - Generic implementations for `Option<T>` and `Vec<T>`
- **Collections** - Support for `HashMap<K,V>`, `BTreeMap<K,V>`, `HashSet<T>`, `BTreeSet<T>`, `VecDeque<T>`, `LinkedList<T>`
- **Four Interaction Paradigms**:
  - **Select** - Choose from finite options (enum pattern)
  - **Affirm** - Yes/no confirmation (bool pattern)
  - **Survey** - Multi-field elicitation (struct pattern)
  - **Authorize** - Permission policies (planned for v0.2.0)
- **Type-Safe** - Compile-time guarantees for all elicitation
- **Composable** - Nest types arbitrarily deep
- **Async-First** - Built on tokio with Send bounds
- **MCP Integration** - Uses pmcp for high-performance communication

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
elicitation = "0.1"
pmcp = "1.4"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

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

1. Your application creates an MCP client with `StdioTransport::new()`
2. Claude (the MCP client) provides elicitation tools via stdin/stdout
3. When you call `.elicit()`, it sends tool requests to Claude
4. Claude prompts the user and validates responses
5. Your code receives strongly-typed Rust values

**Note**: Examples won't work standalone - they must be invoked by an MCP client.

## Quick Start

```rust
use elicitation::{Elicit, Elicitation, ElicitResult};
use pmcp::StdioTransport;

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
    // Create MCP client
    let transport = StdioTransport::new();
    let client = pmcp::Client::new(transport);

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
```

**Try it**: `claude "Run the simple_types example"`

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

### Enums (Select Pattern)

```rust
#[derive(Debug, Elicit)]
enum Status {
    Pending,
    InProgress,
    Completed,
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

The library uses [pmcp](https://crates.io/crates/pmcp) for MCP communication:

```rust
use pmcp::StdioTransport;

// Create transport (stdio for Claude Desktop)
let transport = StdioTransport::new();
let client = pmcp::Client::new(transport);

// Use with elicitation
let value = MyType::elicit(&client).await?;
```

## Documentation

- [API Documentation](https://docs.rs/elicitation)
- [Examples](./crates/elicitation/examples/)
- [MCP Protocol](https://modelcontextprotocol.io)
- [pmcp crate](https://docs.rs/pmcp)

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

Current version: **0.1.0**

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

- Built with [pmcp](https://crates.io/crates/pmcp) for MCP integration
- Powered by [tokio](https://tokio.rs) for async runtime
- Uses [tracing](https://github.com/tokio-rs/tracing) for observability
