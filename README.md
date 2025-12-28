# Elicitation

> Conversational elicitation of strongly-typed Rust values via MCP

[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](LICENSE-APACHE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)

`elicitation` is a Rust library that transforms conversational LLM interactions into strongly-typed Rust values through the Model Context Protocol (MCP). It provides a trait-based system for eliciting primitive types, enums, structs, and nested data structures through natural language interaction.

## Features

- **Trait-Based Elicitation** - Simple `Elicit` trait for all types
- **Derive Macros** - Automatic implementation for enums and structs
- **Primitive Types** - Built-in support for integers, floats, booleans, strings
- **Containers** - Generic implementations for `Option<T>` and `Vec<T>`
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

The library includes several comprehensive examples:

### Primitive Types

```rust
// Elicit basic Rust types
let age: i32 = i32::elicit(&client).await?;
let name: String = String::elicit(&client).await?;
let confirmed: bool = bool::elicit(&client).await?;
let nickname: Option<String> = Option::<String>::elicit(&client).await?;
let scores: Vec<i32> = Vec::<i32>::elicit(&client).await?;
```

Run with: `cargo run --example simple_types`

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

Run with: `cargo run --example enums`

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

Run with: `cargo run --example structs`

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

Run with: `cargo run --example complex_survey`

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
