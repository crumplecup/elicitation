# Elicitation Examples

This directory contains comprehensive examples demonstrating the `elicitation` library's capabilities.

## ⚠️ Important: MCP Setup Required

**These examples will NOT work standalone.** They must be run through an MCP client (like Claude Desktop or Claude CLI) that provides the elicitation tools.

## Quick Start

### Option 1: Claude CLI (Recommended for Testing)

```bash
# Install Claude CLI (if not already installed)
# See: https://docs.anthropic.com/en/docs/agents-and-tools

# Run an example
claude mcp add elicitation-demo --command "cargo run --example simple_types"

# Or ask Claude to run it
claude "Run the simple_types example from elicitation"
```

### Option 2: Claude Desktop

1. Add your project to Claude Desktop's MCP configuration:

   **macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
   **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
   **Linux**: `~/.config/claude/claude_desktop_config.json`

2. Add configuration:
   ```json
   {
     "mcpServers": {
       "elicitation-examples": {
         "command": "cargo",
         "args": ["run", "--example", "simple_types"],
         "cwd": "/path/to/elicitation",
         "env": {}
       }
     }
   }
   ```

3. Restart Claude Desktop and the examples will be available

## Available Examples

### Basic Types

#### `simple_types.rs`
Demonstrates elicitation of primitive types:
- Integers (`i32`, `u8`, etc.)
- Strings
- Booleans
- Optional values (`Option<T>`)
- Collections (`Vec<T>`)

**Run**: `cargo run --example simple_types`

**Demonstrates**:
- Basic type elicitation
- Optional value handling
- Collection building

---

#### `pathbuf.rs`
Filesystem path elicitation with validation:
- Absolute and relative paths
- Optional paths
- Collections of paths

**Run**: `cargo run --example pathbuf`

**Demonstrates**:
- `PathBuf` type support
- Path validation
- File system integration

---

#### `network.rs`
Network type elicitation with parsing and validation:
- IP addresses (IPv4/IPv6)
- Socket addresses
- Network configuration

**Run**: `cargo run --example network`

**Demonstrates**:
- `IpAddr`, `Ipv4Addr`, `Ipv6Addr`
- `SocketAddr`, `SocketAddrV4`, `SocketAddrV6`
- Automatic parsing and validation

---

#### `duration.rs`
Time duration elicitation:
- Duration from seconds (supports decimals)
- Non-negative validation
- Collections of durations

**Run**: `cargo run --example duration`

**Demonstrates**:
- `std::time::Duration`
- Decimal second input (e.g., 1.5 seconds)
- Range validation

---

### Enum & Struct Patterns

#### `enums.rs`
Enum elicitation using the **Select** paradigm:
- Unit variant enums
- Custom prompts
- Nested enum structures

**Run**: `cargo run --example enums`

**Demonstrates**:
- `#[derive(Elicit)]` for enums
- Select pattern
- `#[prompt("...")]` attribute

---

#### `structs.rs`
Struct elicitation using the **Survey** paradigm:
- Multi-field data collection
- Field-level prompts
- Nested struct composition

**Run**: `cargo run --example structs`

**Demonstrates**:
- `#[derive(Elicit)]` for structs
- Survey pattern
- Field-level `#[prompt]`
- Nested structures

---

#### `complex_survey.rs`
Advanced nested structure elicitation:
- Deep nesting (structs containing structs)
- Mixed types (enums + structs + collections)
- Real-world application modeling

**Run**: `cargo run --example complex_survey`

**Demonstrates**:
- Complex data hierarchies
- Type composition
- Real-world use cases

---

### Container Types

#### `result.rs`
Success/failure outcome elicitation:
- `Result<T, E>` type support
- Variant selection
- Error handling patterns

**Run**: `cargo run --example result`

**Demonstrates**:
- `Result` container
- Discriminated unions
- Error type elicitation

---

#### `tuples.rs`
Heterogeneous tuple elicitation:
- 2-tuples through 12-tuples
- Mixed types within tuples
- Sequential elicitation

**Run**: `cargo run --example tuples`

**Demonstrates**:
- Tuple types `(T1, T2, ...)`
- Type heterogeneity
- Ordered elicitation

---

#### `arrays.rs`
Fixed-size array elicitation:
- Const generic arrays `[T; N]`
- Any size N
- Type safety with compile-time bounds

**Run**: `cargo run --example arrays`

**Demonstrates**:
- Array types `[T; N]`
- Const generics
- Fixed-size collections

---

### Collections

#### `collections.rs`
Standard library collection types:
- `HashMap<K, V>` - with duplicate key handling
- `HashSet<T>` - with automatic deduplication
- `BTreeMap<K, V>` - ordered maps
- `BTreeSet<T>` - ordered sets
- `VecDeque<T>` - double-ended queue
- `LinkedList<T>` - doubly-linked list

**Run**: `cargo run --example collections`

**Demonstrates**:
- All standard collection types
- Duplicate handling
- Ordering behavior

---

#### `smart_pointers.rs`
Heap allocation and reference counting:
- `Box<T>` - heap allocation
- `Rc<T>` - reference counting
- `Arc<T>` - atomic reference counting

**Run**: `cargo run --example smart_pointers`

**Demonstrates**:
- Smart pointer types
- Ownership patterns
- Thread-safe sharing (`Arc`)

---

## Building All Examples

```bash
# Build all examples (verify compilation)
cargo build --examples

# Check specific example
cargo check --example simple_types
```

## Example Structure Pattern

All examples follow this pattern:

```rust
use elicitation::{Elicit, Elicitation, ElicitResult};
use rmcp::ServiceExt;

#[tokio::main]
async fn main() -> ElicitResult<()> {
    // 1. Set up tracing
    tracing_subscriber::fmt::init();

    // 2. Create MCP client
    let client = ()
        .serve(rmcp::transport::stdio())
        .await?;

    // 3. Elicit values
    let value = MyType::elicit(&client).await?;

    // 4. Use the value
    println!("Elicited: {:?}", value);

    Ok(())
}
```

## Troubleshooting

### "No MCP client detected"
**Cause**: Running example directly without MCP client
**Solution**: Run through Claude CLI or Claude Desktop (see [Quick Start](#quick-start))

### "Tool not found: elicit_*"
**Cause**: MCP client not providing elicitation tools
**Solution**: Ensure your MCP setup includes elicitation tool definitions

### "Connection refused" or "Broken pipe"
**Cause**: MCP transport misconfigured
**Solution**: Check stdio transport configuration in your MCP client

## Contributing Examples

When adding new examples:

1. **Name clearly**: Use descriptive names (e.g., `custom_validation.rs`)
2. **Document well**: Include doc comments explaining what's demonstrated
3. **Keep focused**: One concept per example
4. **Add to this README**: Update the list above with your example

## Additional Resources

- [Main Documentation](https://docs.rs/elicitation)
- [MCP Protocol Docs](https://modelcontextprotocol.io)
- [rmcp Crate Docs](https://docs.rs/rmcp)
- [Repository](https://github.com/crumplecup/elicitation)
