# Changelog

All notable changes to the `elicitation` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-XX

### Added

#### Core Traits and Derive Macros
- `Elicitation` trait for type-safe conversational elicitation via MCP
- `Prompt` trait for customizable prompt text
- `#[derive(Elicit)]` proc macro for enums (Select paradigm)
- `#[derive(Elicit)]` proc macro for structs (Survey paradigm)
- `#[prompt("...")]` attribute for custom prompts (struct and field level)
- `#[skip]` attribute for skipping fields during elicitation

#### Interaction Paradigms
- **Select** - Choose from finite enum variants
- **Affirm** - Yes/no boolean confirmation
- **Survey** - Multi-field struct elicitation
- **Authorize** - Permission policies (trait only, implementation planned for v0.2.0)

#### Primitive Types
- All signed integers: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- All unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Floating point: `f32`, `f64`
- Text and boolean: `String`, `bool`
- Time duration: `std::time::Duration` (f64 seconds with validation)
- Filesystem paths: `std::path::PathBuf`
- Network types:
  - `std::net::IpAddr` (IPv4 or IPv6 with string parsing)
  - `std::net::Ipv4Addr` (IPv4 only)
  - `std::net::Ipv6Addr` (IPv6 only)
  - `std::net::SocketAddr` (IP + port)
  - `std::net::SocketAddrV4` (IPv4 socket)
  - `std::net::SocketAddrV6` (IPv6 socket)

#### Container Types
- `Option<T>` - Optional value elicitation with affirm-then-elicit pattern
- `Vec<T>` - Dynamic array with loop-based elicitation
- `Result<T, E>` - Success/failure outcomes with variant selection
- `[T; N]` - Fixed-size arrays using const generics (any size N)
- Tuples from arity 1 to 12: `(T1)`, `(T1, T2)`, ..., `(T1, ..., T12)`

#### Smart Pointers
- `Box<T>` - Heap-allocated values
- `Rc<T>` - Reference-counted shared ownership
- `Arc<T>` - Atomically reference-counted thread-safe sharing

#### Collections
- `HashMap<K, V>` - Hash-based key-value map with duplicate key handling
- `BTreeMap<K, V>` - Ordered key-value map
- `HashSet<T>` - Hash-based unique set with automatic deduplication
- `BTreeSet<T>` - Ordered unique set
- `VecDeque<T>` - Double-ended queue
- `LinkedList<T>` - Doubly-linked list

#### Error Handling
- `ElicitError` with location tracking (`#[track_caller]`)
- `ElicitErrorKind` enum covering all error scenarios:
  - `InvalidFormat` - Parsing/validation failures
  - `OutOfRange` - Value outside valid bounds
  - `InvalidOption` - Invalid enum selection
  - `MissingField` - Required struct field missing
  - `Cancelled` - User-initiated cancellation
  - `Mcp` - MCP protocol errors (wraps `PmcpError`)
  - `Json` - JSON serialization errors
- Derived `Display` and `Error` implementations via `derive_more`
- Automatic conversion from MCP and JSON errors

#### MCP Integration
- Full integration with `pmcp` crate (v1.4+)
- Stdio transport support for Claude Desktop/CLI
- Async-first design with tokio runtime
- `Send` trait bounds for thread-safe elicitation

#### Testing & Examples
- 64 unit tests covering all type implementations
- 8 integration tests for derive macros
- 15 comprehensive examples:
  - `simple_types.rs` - Basic primitive elicitation
  - `enums.rs` - Select paradigm demonstration
  - `structs.rs` - Survey paradigm with custom prompts
  - `pathbuf.rs` - Filesystem path elicitation
  - `network.rs` - IP address and socket elicitation
  - `duration.rs` - Time duration elicitation
  - `result.rs` - Success/failure outcome elicitation
  - `collections.rs` - HashMap, HashSet, BTreeMap, BTreeSet examples
  - `tuples.rs` - Tuple type elicitation
  - `arrays.rs` - Fixed-size array elicitation
  - `smart_pointers.rs` - Box, Rc, Arc examples
  - `complex_survey.rs` - Deeply nested struct elicitation
  - And more!

#### Documentation
- Comprehensive README with MCP setup guide
- API documentation with doctests for all public items
- ROADMAP.md outlining future enhancements (validation, advanced patterns)
- Full tracing integration for observability

### Infrastructure
- Workspace structure with `elicitation` and `elicitation_derive` crates
- Just-based development workflow with:
  - `just check-all` - Complete verification suite
  - `just test-api` - Feature-gated API tests
  - `just pre-release` - Full CI pipeline
  - `just audit` - Security vulnerability scanning
  - `just dist-*` - Release management (cargo-dist)
- Dual licensing: Apache-2.0 OR MIT
- CI/CD ready with comprehensive testing
- Zero unsafe code (`#![forbid(unsafe_code)]`)

### Design Decisions
- Never use `#[allow]` directives - fix root causes instead
- All public functions instrumented with `#[tracing::instrument]`
- Builder pattern required for all struct construction (no literals)
- Crate-level exports only (`use crate::Type` not `use crate::module::Type`)
- Tests in `tests/` directory, never inline `#[cfg(test)]` modules
- derive_more for all `Display` and `Error` implementations

### Dependencies
- `pmcp = "1.4"` - MCP client
- `tracing = "0.1"` - Structured logging
- `tokio = "1"` - Async runtime
- `derive_more = "1"` - Derive utilities
- `derive-getters = "0.5"` - Field accessors
- `serde = "1"` - Serialization (MCP protocol)

### Compatibility
- **Rust Version**: 1.70+ (2021 edition)
- **MCP Clients**: Claude Desktop, Claude CLI
- **Platforms**: All platforms supported by Rust and pmcp

---

## [Unreleased]

### Planned for v0.2.0
- Attribute-based validation (`#[validate(...)]`)
- Integration with `validator` crate
- Filesystem validators (path_exists, is_file, is_directory, etc.)
- Email, URL, phone number validation
- Range and length constraints
- Custom validator functions
- Validation error messages with retry logic

### Planned for v0.3.0+
- Conditional field elicitation (`#[elicit_if(...)]`)
- Multi-select enum support
- Ranked choice paradigm
- Dynamic form generation
- Cross-field validation
- Interactive features (pending MCP protocol enhancements)

[0.1.0]: https://github.com/crumplecup/elicitation/releases/tag/v0.1.0
[Unreleased]: https://github.com/crumplecup/elicitation/compare/v0.1.0...HEAD
