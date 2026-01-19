# Changelog

All notable changes to the `elicitation` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Add Value elicitation support
- Add chrono datetime elicitation support
- Add time datetime elicitation support
- Add jiff datetime elicitation support
- Add elicitation style system - REVOLUTIONARY
- Add field-level style support with derive integration
- Update derive macro for Style System v2
- Add ElicitationStyle trait for extensible style system
- Complete Phase 3 - Inline elicitation for all primitives
- Complete Phase 4 - Auto-selection for styles
- Complete Phase 5 - Builder pattern for ergonomic style overrides

### Documentation

- Add serde_json::Value elicitation implementation plan
- Add comprehensive datetime elicitation implementation plan
- Add datetime plan to index
- Add comprehensive elicitation style system implementation plan
- Document all 0.2.2 features and bump version
- Plan Style System v2 with associated types
- Add custom_style example demonstrating ElicitationStyle
- Update STYLE_SYSTEM_V2_PLAN with Phase 1-2 completion status
- Mark all phases complete - Style System v2 ready for v0.3.0

### Fixed

- Update all examples to use ElicitClient
- Fix example imports and styled struct derive
- Add allow(dead_code) to test structs
- Update release.toml to remove deprecated [workspace] section

### Styling

- Run cargo fmt to fix formatting

### Testing

- Delete worthless style_derive_test.rs

### Wip

- Start Style System v2 - primitives done
- Complete primitives including tuples
- Add Option and Result style implementations
- Complete all containers
- Complete all collections with Style v2
- Complete all datetime types with Style v2
- Complete serde_json::Value with Style v2

## [0.2.2] - 2026-01-19

### Added - Revolutionary Style System 🎨

- **Field-level style customization** via `#[prompt("text", style = "name")]` syntax
  - Multiple prompt styles per field (e.g., "curt", "verbose", "wizard")
  - Runtime style selection by LLM or user (just another Select elicitation!)
  - Sensible fallback strategy: missing style prompts use default
  - Generated style enum: `{StructName}ElicitStyle` with `Default` + collected styles
  - Style selection is a state machine step - separates *what* to ask from *how* to ask
- **Built-in ElicitationStyle trait** with 4 implementations:
  - `DefaultStyle` - Standard prompts
  - `CompactStyle` - Terse, minimal prompts
  - `VerboseStyle` - Detailed, explanatory prompts
  - `WizardStyle` - Step-by-step with progress tracking
- **Inline elicitation** for String fields with custom styled prompts
  - Complex types fall back to default elicit() (expanding in future versions)

### Added - DateTime Support (3 Libraries!)

- **chrono feature**: `DateTime<Utc>`, `DateTime<FixedOffset>`, `NaiveDateTime`
- **time feature**: `OffsetDateTime`, `PrimitiveDateTime`
- **jiff feature**: `Timestamp`, `Zoned`, `civil::DateTime`
  - Jiff validates offset matches timezone (DST-aware!)
- **Dual input methods**: ISO 8601 strings OR manual component entry
  - Shared `DateTimeInputMethod` and `DateTimeComponents` patterns
- **Comprehensive tests**: 11 chrono, 10 time, 13 jiff unit tests

### Added - JSON Value Elicitation

- **serde_json feature**: Full `serde_json::Value` support
  - All JSON types: null, bool, number, string, array, object
  - Recursive elicitation for nested arrays and objects
  - Depth limit (10 levels) prevents infinite recursion
  - Proper async recursion via `Box::pin` for futures
- **Error handling**: `InvalidSelection`, `ParseError`, `RecursionDepthExceeded`

### Changed

- **derive macro**: Major refactoring for style system integration
  - `FieldInfo` struct now includes `styled_prompts: HashMap<String, String>`
  - Custom attribute parser for `#[prompt("text", style = "name")]` syntax
  - Split simple vs styled implementation generation
  - Fully qualified trait method calls (`<Self as Trait>::method()`)
- **Workspace dependencies**: Added chrono, time, jiff as optional
  - Cannot mark as optional at workspace level, must use package-level `optional = true`

### Fixed

- Clippy warnings in derive macro (option_as_ref_deref, unnecessary_closure)
- Trait method resolution in generated code via fully qualified syntax
- Orphaned code blocks from incomplete edits

### Documentation

- Updated README with comprehensive feature documentation
- Added examples for all datetime libraries (3 sets)
- Added JSON Value usage examples
- Added detailed style system examples and explanation
- Updated installation section with feature flags
- Added "🆕" markers for v0.2.2 features

### Testing

- 3 style+derive integration tests
- 8 datetime chrono integration tests (ignored, require MCP)
- 6 datetime time integration tests (ignored, require MCP)
- 8 datetime jiff integration tests (ignored, require MCP)
- All existing tests still passing
- Zero clippy warnings across workspace

## [0.2.1] - 2026-01-18

### Added

- **Enum variants with fields** - `#[derive(Elicit)]` now supports:
  - Tuple variants: `Variant(T1, T2, ...)`
  - Struct variants: `Variant { field1: T1, field2: T2 }`
  - Mixed enums with unit, tuple, and struct variants
- Full tracing instrumentation for field elicitation
- Support for nested enums (enum fields in variants)
- Automatic recursive elicitation for complex field types

### Changed

- Enhanced `Elicitation` implementation for enums:
  - Two-phase elicitation: variant selection → field elicitation
  - Each field type must implement `Elicitation` trait
  - Error context preserved for field elicitation failures
- Updated documentation with variant type examples
- Enhanced error messages for invalid variant selections

### Technical Details

- New internal structures: `VariantInfo`, `VariantFields`, `FieldInfo`
- Generated code includes full tracing spans with variant context
- Variant field elicitation is sequential (tuple fields by index, struct fields by name)
- Each field type's `Elicitation` impl handles its own prompting

## [0.2.0] - 2025-12-29

### Changed

**BREAKING CHANGES**: Migration from pmcp to rmcp (official Rust MCP SDK)

#### Core API Changes
- **Client Type**: Changed from `pmcp::Client<T>` to `rmcp::service::Peer<RoleClient>`
  - Removed generic transport parameter (simpler API)
  - All `Elicitation::elicit` methods now use `&Peer<RoleClient>` instead of `&Client<T>`
- **Client Creation**: New pattern using `ServiceExt::serve()`
  ```rust
  // Old (pmcp):
  let transport = StdioTransport::new();
  let client = pmcp::Client::new(transport);

  // New (rmcp):
  let client = ()
      .serve(rmcp::transport::stdio(
          tokio::io::stdin(),
          tokio::io::stdout(),
      ))
      .await?;
  ```

#### Error Types
- Added `RmcpError` wrapper for `rmcp::ErrorData`
- Added `ServiceError` wrapper for `rmcp::service::ServiceError`
- Removed `PmcpError` (replaced by `RmcpError`)
- Updated `ElicitErrorKind` enum:
  - Changed: `Mcp(PmcpError)` → `Rmcp(RmcpError)`
  - Added: `Service(ServiceError)`

#### Internal Changes
- MCP tool parameter builders now return `Map<String, Value>` instead of `Value`
- Content extraction updated for `Annotated<RawContent>` structure
- All implementations updated across primitives, containers, and collections

#### Dependencies
- **Removed**: `pmcp = "1.4"` and 100+ transitive dependencies
- **Added**: `rmcp = "0.12"` (official Rust MCP SDK)
- Reduced dependency tree significantly

### Migration Guide

To upgrade from 0.1.0 to 0.2.0:

1. **Update Cargo.toml**:
   ```toml
   [dependencies]
   elicitation = "0.2"
   rmcp = "0.12"  # Changed from pmcp = "1.4"
   ```

2. **Update imports**:
   ```rust
   // Remove:
   use pmcp::StdioTransport;

   // Add:
   use rmcp::ServiceExt;
   ```

3. **Update client creation**:
   ```rust
   // Old:
   let transport = StdioTransport::new();
   let client = pmcp::Client::new(transport);

   // New:
   let client = ()
       .serve(rmcp::transport::stdio(
           tokio::io::stdin(),
           tokio::io::stdout(),
       ))
       .await?;
   ```

4. **Update function signatures** (if you implemented `Elicitation` manually):
   ```rust
   // Old:
   async fn elicit<T: pmcp::shared::transport::Transport>(
       client: &pmcp::Client<T>,
   ) -> ElicitResult<Self> { ... }

   // New:
   async fn elicit(
       client: &rmcp::service::Peer<rmcp::service::RoleClient>,
   ) -> ElicitResult<Self> { ... }
   ```

### Benefits
- Official SDK support and maintenance from the MCP team
- Cleaner API without generic type parameters
- Better type safety with `Peer<RoleClient>`
- Significantly reduced dependency tree
- Improved performance and reliability

---

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
- `rmcp = "0.12"` - Official Rust MCP SDK (changed from pmcp)
- `tracing = "0.1"` - Structured logging
- `tokio = "1"` - Async runtime
- `derive_more = "1"` - Derive utilities
- `derive-getters = "0.5"` - Field accessors
- `serde = "1"` - Serialization (MCP protocol)

### Compatibility
- **Rust Version**: 1.70+ (2021 edition)
- **MCP Clients**: Claude Desktop, Claude CLI
- **Platforms**: All platforms supported by Rust and rmcp

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

[0.2.0]: https://github.com/crumplecup/elicitation/releases/tag/v0.2.0
[0.1.0]: https://github.com/crumplecup/elicitation/releases/tag/v0.1.0
[Unreleased]: https://github.com/crumplecup/elicitation/compare/v0.2.0...HEAD
