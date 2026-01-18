# Claude Project Instructions

## Quick Reference

| Category      | Key Rule                                                   | Section                                           |
| ------------- | ---------------------------------------------------------- | ------------------------------------------------- |
| **Testing**   | No `#[cfg(test)]` in source files → use `tests/` directory | [Testing](#testing)                               |
| **Errors**    | Use `derive_more::Display` + `derive_more::Error`          | [Error Handling](#error-handling)                 |
| **Tracing**   | All public functions have `#[instrument]`                  | [Logging](#logging-and-tracing)                   |
| **Builders**  | Always use builders, never struct literals                 | [Type Construction](#type-construction)           |
| **Imports**   | `use crate::{Type}` not `use crate::module::Type`          | [Module Organization](#module-organization)       |
| **lib.rs**    | Only `mod` and `pub use` statements                        | [Module Organization](#module-organization)       |
| **Workspace** | No re-exports between workspace crates                     | [Workspace Organization](#workspace-organization) |
| **Commits**   | Fix all errors/warnings before committing                  | [Workflow](#workflow)                             |
| **Linting**   | Never use `#[allow]` - fix root cause instead              | [Linting](#linting)                               |

---

## Workflow

### Development Cycle

1. **Plan** → Use planning document (.md) with implementation steps

- Add the planning document to PLANNING_INDEX.md for tracking.

2. **For each step:**
   - Generate code
   - Fix cargo check errors/warnings
   - Run all checks (see below)
   - Commit with audit-friendly message
   - Push to branch
3. **Update** planning document to serve as user guide

### Pre-Commit Verification

Fix all issues before committing, even if "unrelated":

```bash
# only run these after code changes:
# prefer running on package when possible
just check [package]                    # Basic compilation
# check-all recipe takes too long on full workspace
# prefer testing the package in isolation
just test-package [package]
just check-all [package]                 # clippy, fmt & test

# only run this if markdown files changed:
markdownlint-cli2 "**/*.md"   # Markdown (if changed)
```

Use `just` recipes (not raw `cargo` commands) to ensure justfile stays current.

Pre-merge commits, in addition:

- run `just check-all`
- run `just audit`
- run `just check-features`

Zero tolerance: all tests passing, zero clippy warnings, zero errors.

### Why "Unrelated" Issues Matter

````rust
// You export Input at crate level
pub use input::Input;

// Existing doctest breaks (was using module path):
/// ```
/// use crate::module::Input;  // ❌ Now ambiguous!
/// ```

// Fix immediately:
/// ```
/// use crate::Input;  // ✅ Crate-level import
/// ```
````

Common pitfalls:

- Export changes → doctest import paths break
- New exports → name conflicts with existing types
- Feature additions → missing `#[cfg(feature)]` gates
- Struct field additions → doctests missing new required fields

### API Testing (Rate-Limited)

Only run when:

- Explicitly requested
- Before merge to main
- Targeted integration testing

```bash
just test-api
```

---

## Type Construction

### Builder Pattern

Always use builders for struct construction. Never use struct literals.

```rust
// ❌ BAD: Struct literal (breaks on field additions, order-dependent)
let config = Config {
    host: "localhost".to_string(),
    port: 8080,
    timeout: Duration::from_secs(30),
};

// ✅ GOOD: Builder pattern (self-documenting, future-proof)
let config = Config::builder()
    .host("localhost")
    .port(8080)
    .timeout(Duration::from_secs(30))
    .build();
```

Benefits: Self-documenting, optional fields, validation, IDE support

### Builder Types in This Codebase

**1. derive_builder** (`#[derive(derive_builder::Builder)]`):

```rust
use crate::MessageBuilder;  // Import Builder struct

let msg = MessageBuilder::default()
    .role(Role::User)
    .content(vec![Input::Text("test".to_string())])
    .build()
    .expect("Valid message");
```

**2. derive_new** (`#[derive(derive_new::new)]`):

For simple constructors with few arguments, use `derive_new` to generate a `new()` method:

```rust
use derive_new::new;

#[derive(Debug, Clone, new)]
pub struct Config {
    host: String,
    port: u16,
}

// Usage:
let config = Config::new("localhost".to_string(), 8080);
```

**3. Manual builders** (`impl Type { pub fn builder() }`):

For complex types with many optional fields or validation:

```rust
// Do not import Builder struct

let request = GenerateRequest::builder()  // Type provides builder()
    .messages(vec![msg])
    .build()
    .expect("Valid request");
```

**Test pattern:** Extract `.build()` calls that return `Result`:

```rust
// ✅ GOOD: Separate statements
let message = MessageBuilder::default()
    .role(Role::User)
    .build()
    .expect("Valid");

let request = GenerateRequest::builder()
    .messages(vec![message])
    .build()
    .expect("Valid");

// ❌ BAD: Nested builds (Result handling fails)
let request = GenerateRequest::builder()
    .messages(vec![MessageBuilder::default()...build()])
    .build();
```

---

## Derive Policies

### Standard Derives

Data structures:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MyType { /* ... */ }
```

Use derive_more for:

- `Display`, `FromStr`, `From`, `Deref`, `DerefMut`, `AsRef`, `AsMut`

Enums (no fields):

```rust
#[derive(Debug, Clone, strum::EnumIter)]
pub enum Status { Active, Inactive }
```

### Field Access

Private fields + derive-based access:

```rust
use derive_getters::Getters;
use derive_setters::Setters;

#[derive(Debug, Clone, Getters, Setters)]
#[setters(prefix = "with_")]  // Avoid getter/setter name conflicts
pub struct SecurityContext {
    /// User ID (propagated to getter docs)
    #[setters(doc = "Sets user ID")]  // Separate setter docs
    user_id: Option<UserId>,

    #[setters(skip)]  // Read-only field
    created_at: DateTime<Utc>,
}

// Usage with manual constructor:
impl SecurityContext {
    pub fn new(user_id: Option<UserId>) -> Self {
        Self {
            user_id,
            created_at: Utc::now(),
        }
    }
}

ctx.user_id();           // Getter
ctx.with_user_id(new_id); // Setter
```

When to use:

- **derive_getters**: Always for private fields
- **derive_setters**: Mutable config/state objects
- **Manual constructors**: For initialization (connections, validation, resources)

### Exception: Error Types

ErrorKind (specific conditions):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
pub enum StorageErrorKind {
    #[display("Media not found: {}", _0)]
    NotFound(String),
}
```

Wrapper (error + location):

```rust
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Storage: {} at {}:{}", kind, file, line)]
pub struct StorageError {
    pub kind: StorageErrorKind,
    pub line: u32,
    pub file: &'static str,  // ✅ &'static str, not String
}
```

Do not derive `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord` on wrapper errors (location tracking makes comparison confusing).

---

## Error Handling

### Use derive_more

Do not write manual `impl Display` or `impl Error` for error types.

Use `derive_more::Display` + `derive_more::Error` on all errors.

Audit checklist:

- ✅ All error structs use `derive_more::Display` with `#[display(...)]`
- ✅ All error structs use `derive_more::Error`
- ✅ All ErrorKind variants have `#[display(...)]`
- ✅ No manual `impl std::fmt::Display`
- ✅ No manual `impl std::error::Error`
- ✅ All constructors use `#[track_caller]`
- ✅ Error `file` fields use `&'static str`

### Pattern 1: Simple Error (message + location)

```rust
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("HTTP Error: {} at {}:{}", message, file, line)]
pub struct HttpError {
    pub message: String,
    pub line: u32,
    pub file: &'static str,
}

impl HttpError {
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: message.into(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}
```

### Pattern 2: ErrorKind Enum

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, derive_more::Display)]
pub enum StorageErrorKind {
    #[display("Media not found: {}", _0)]
    NotFound(String),  // Tuple: use _0, _1

    #[display("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },  // Struct: use names
}
```

### Pattern 3: Wrapper (ErrorKind + location)

```rust
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Storage: {} at {}:{}", kind, file, line)]
pub struct StorageError {
    pub kind: StorageErrorKind,
    pub line: u32,
    pub file: &'static str,
}

impl StorageError {
    #[track_caller]
    pub fn new(kind: StorageErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self { kind, line: loc.line(), file: loc.file() }
    }
}
```

### Pattern 4: Crate-Level Aggregation

```rust
#[derive(Debug, derive_more::From, derive_more::Display, derive_more::Error)]
pub enum CrateErrorKind {
    #[from(HttpError)]
    Http(HttpError),

    #[from(StorageError)]
    Storage(StorageError),
}

#[derive(Debug, derive_more::Display, derive_more::Error)]
#[display("Crate Error: {}", _0)]
pub struct CrateError(Box<CrateErrorKind>);

// Generic blanket From
impl<T> From<T> for CrateError
where T: Into<CrateErrorKind>
{
    fn from(err: T) -> Self {
        Self(Box::new(err.into()))
    }
}
```

### Pattern 5: Error Conversion Macros (Monomorphization)

**Problem:** Blanket `impl<T> From<T>` creates type inference ambiguity,
requiring manual conversions everywhere.

**Solution:** Macros generate monomorphized `From` impls for perfect
inference and ergonomic `?` operator.

**Two-macro system:**

```rust
/// Bridge external errors through wrappers into ErrorKind
macro_rules! bridge_error {
    ($external:ty => $wrapper:ty) => {
        impl From<$external> for ErrorKind {
            #[track_caller]
            fn from(err: $external) -> Self {
                <$wrapper>::from(err).into()
            }
        }
    };
}

/// Complete the chain: external error → ErrorKind → Error
macro_rules! error_from {
    ($source:ty) => {
        impl From<$source> for Error {
            #[track_caller]
            fn from(err: $source) -> Self {
                let kind = ErrorKind::from(err);
                tracing::error!(error_kind = %kind, "Error created");
                Self(Box::new(kind))
            }
        }
    };
}

// Usage: One line per external error type
bridge_error!(reqwest::Error => HttpError);
bridge_error!(serde_json::Error => JsonError);
bridge_error!(url::ParseError => UrlError);

// Complete the conversion chain
error_from!(reqwest::Error);
error_from!(serde_json::Error);
error_from!(url::ParseError);

// From<ErrorKind> for Error (manual, not macro)
impl From<ErrorKind> for Error {
    #[track_caller]
    fn from(kind: ErrorKind) -> Self {
        tracing::error!(error_kind = %kind, "Error created");
        Self(Box::new(kind))
    }
}
```

**Why macros are required:**

Without macros, the compiler cannot infer which `Into<ErrorKind>` impl to use:

```rust
// ❌ Blanket impl causes ambiguity
impl<T> From<T> for Error where T: Into<ErrorKind> { ... }

// At call sites:
let response = reqwest::get(url).await?;
//                                   ^ ERROR: type annotations needed

// Forces manual conversion everywhere:
let response = reqwest::get(url)
    .await
    .map_err(|e| Error::from(ErrorKind::from(HttpError::from(e))))?;  // Boilerplate!
```

With macros (monomorphized):

```rust
// ✅ Concrete implementations via macros
error_from!(reqwest::Error);  // Generates impl From<reqwest::Error> for Error

// At call sites:
let response = reqwest::get(url).await?;  // ✅ Just works! Perfect inference.
```

**Conversion chain:**

```text
reqwest::Error
  ↓ (error_from! macro generates From impl)
ErrorKind::from(reqwest::Error)
  ↓ (bridge_error! impl: reqwest::Error → HttpError → ErrorKind)
ErrorKind::Http(HttpError)
  ↓ (From<ErrorKind> for Error)
Error(Box<ErrorKind>)
```

**Benefits:**

- Zero boilerplate at call sites (just use `?`)
- Perfect type inference (compiler knows exact impl)
- Explicit error type list (self-documenting)
- `#[track_caller]` preserved on all conversions
- Type-safe (only declared types convert)

Reference: See `crates/botticelli_error` for complete implementation.

---

## Logging and Tracing

### Instrumentation

All public functions have `#[instrument]`.

Observability is critical for debugging, performance monitoring, and error tracking. Missing instrumentation is a defect.

```rust
#[instrument(skip(conn), fields(table_name, limit))]
pub fn list_content(
    conn: &mut PgConnection,
    table_name: &str,
    limit: i64,
) -> DatabaseResult<Vec<ContentRow>> {
    debug!("Querying content table");
    debug!(sql = %query, "Executing query");

    match result {
        Ok(rows) => {
            debug!(count = rows.len(), "Retrieved rows");
            Ok(rows)
        }
        Err(e) => {
            error!(error = ?e, "Query failed");
            Err(e.into())
        }
    }
}
```

### Instrumentation Requirements

All public functions:

1. Use `#[instrument]` for automatic span creation
2. Skip large params: `skip(connection, data)`
3. Include context: `fields(table_name, limit)`
4. Emit events at key points (debug/info/warn/error)
5. Log SQL at debug level
6. Track errors before returning

### Log Levels

- `trace!()` - Loop iterations, fine-grained detail
- `debug!()` - Function entry/exit, state changes, SQL
- `info!()` - Major events, initialization
- `warn!()` - Recoverable unusual conditions
- `error!()` - Errors requiring investigation

### Structured Logging

```rust
debug!(count = items.len(), "Processing");        // Field
debug!(value = ?self.field(), "State");          // Debug format
info!(table = %table_name, "Creating");          // Display format
#[instrument(skip(connection, large_json))]     // Skip large data
```

**Feature-gated code**: Use full path `tracing::warn!()` instead of `use tracing::warn;` to avoid orphaned imports when features are disabled.

### Audit Checklist

- ✅ Every public function has `#[instrument]`
- ✅ Span fields include context (IDs, counts)
- ✅ Large structures skipped
- ✅ Key operations emit events
- ✅ Errors logged before return
- ✅ SQL at debug level
- ✅ Span names follow `module.function`

### Why Tracing Matters for AI Development

Without tracing:

```
Human: "Bot command failed"
AI: "Run with RUST_LOG=debug"
Human: <500 lines>
AI: "Check guild_id?"
Human: "Oh, forgot that"
```

Result: 3+ messages, 10+ minutes

With tracing:

```
Human: "Bot command failed"
Human: <5 lines>
ERROR discord.execute: Missing arg command="server.get_stats" missing_arg="guild_id"
AI: "Add guild_id = '123456' to TOML [bot.args]"
```

Result: 1 message, instant fix

Benefits:

- Location tracking → AI jumps to code
- Structured fields → AI sees exact problem
- Spans → AI understands which layer failed
- Error context → AI knows what was attempted
- Performance data → AI diagnoses slowness
- Cache observability → AI debugs stale data

Design principle: Write traces for an AI reader. Ask: "Would this single trace give enough info to diagnose?"

---

## Testing

### No Inline Test Modules

`#[cfg(test)] mod tests` in source files is not allowed.

All tests go in `tests/` directory.

Rationale: Centralized tests, easier to maintain, no source file clutter

Audit: Flag any `#[cfg(test)]` or `mod tests` in source as violations

### Test Organization

Naming: `{module}_{component}_test.rs`

- Examples: `storage_filesystem_test.rs`, `narrative_executor_test.rs`

Imports: Use crate-level exports

```rust
use botticelli::{Type, OtherType};  // ✅ Crate-level
// NOT: use botticelli::module::Type;
```

Independence: Self-contained, no inter-test dependencies

Helpers: Create within test files to reduce duplication

### API Rate Limit Conservation

Design tests to minimize:

- Tokens: Minimal prompts, low `max_tokens` (e.g., 10)
- Requests: Fewest calls possible (1-3, not 20+)
- Time: Short duration

Feature gating:

```rust
#[test]
#[cfg_attr(not(feature = "api"), ignore)]
fn test_gemini_api() {
    // Uses API tokens
}
```

Run with: `just test-api`

Do not use `#[ignore]` - reserved for:

- Unimplemented features
- Broken tests needing fixes
- Temporarily disabled during refactoring

Consider:

- Mocking API responses
- Separate "expensive" test suite with warnings
- Local test doubles

---

## Module Organization

### Rules

lib.rs: Only `mod` and `pub use` statements. No type definitions, traits, or impl blocks.

Module declarations: Private (not `pub mod`)

Crate-level exports: Re-export all public types at crate root

Imports: Always `use crate::{Type}`, never `use crate::module::Type` or `super::Type`

### Structure Example

```rust
// src/lib.rs (✅ ONLY mod + pub use)
mod error;
mod models;

pub use error::{MyError, MyResult};
pub use models::{Model, NewModel};

// src/models.rs
use crate::{MyError, MyResult};  // ✅ Crate-level imports

pub struct Model { /* ... */ }
```

### When to Split Modules

Guideline: When file exceeds ~500-1000 lines

Structure:

```
src/mymodule/
├── mod.rs           # ONLY mod + pub use
├── core.rs
├── io.rs
└── helpers.rs       # Internal, not exported
```

mod.rs pattern:

```rust
mod core;
mod io;
mod helpers;  // Internal

pub use core::{Type1, Type2};
pub use io::{Reader, Writer};
// helpers not exported
```

### Import Patterns

Crate-level types:

```rust
use crate::{Type1, Type2};  // ✅ Always
```

Internal helpers:

```rust
use crate::module::helper::function;     // ✅ OK for internal
use crate::database::schema::users;       // ✅ OK for schema
```

Forbidden:

```rust
use crate::module::Type;  // ❌ Module path
use super::Type;          // ❌ Super path
use module::*;            // ❌ Wildcard
```

### Benefits

1. Single import path per type
2. No ambiguity
3. Hidden internal structure
4. Easy refactoring
5. Better IDE support

---

## Workspace Organization

### lib.rs in Workspace Crates

lib.rs only contains `mod` declarations and `pub use` exports.

Even small crates (100-200 lines) should separate into modules:

```
crates/my_crate/src/
├── lib.rs       # mod + pub use only
├── role.rs
├── input.rs
└── output.rs
```

### No Re-Exports Between Workspace Crates

Forbidden:

```rust
// crates/botticelli_database/src/lib.rs
pub use botticelli_error::DatabaseError;  // ❌ Creates ambiguity

// Now users have two import paths:
use botticelli_error::DatabaseError;      // Source
use botticelli_database::DatabaseError;   // Re-exported ❌
```

Required:

```rust
// crates/botticelli_database/src/lib.rs
// NO re-exports of dependency types

// Users import from source:
use botticelli_database::Repository;   // Database's own types
use botticelli_error::DatabaseError;  // Error from error crate
```

Rationale:

- Creates ambiguity (two paths to same type)
- Breaks "single import path" principle
- Makes refactoring difficult
- IDE confusion
- Unclear ownership

Type aliases OK:

```rust
use botticelli_error::DatabaseError;

/// Convenience alias for database results.
pub type DatabaseResult<T> = Result<T, DatabaseError>;  // ✅ Alias OK
```

Exception: Only top-level binary/library crate may re-export for user convenience:

```rust
// crates/botticelli/src/lib.rs (top-level only)
pub use botticelli_core::{Role, Input};
pub use botticelli_error::BotticelliError;
// Internal crates NEVER re-export from each other
```

### Module Responsibilities

One clear responsibility per module:

- Single type (simple case)
- Related types (common case - e.g., enum + helper struct)
- Shared dependencies (used by multiple modules)

### Import Patterns in Workspace

Same rules as single crates:

```rust
use crate::{Type};           // ✅ Crate-level
// NOT: use crate::module::Type;
```

Cross-crate imports:

```rust
use other_crate::{Type};     // ✅ Direct from source crate
```

---

## Serialization

Derive: `Serialize`, `Deserialize` for persisted/transmitted types

Attributes:

- `#[serde(skip)]` - Runtime state, caches, UI state, handles
- `#[serde(default)]` - Use `Default` when missing
- `#[serde(default = "fn_name")]` - Custom default
- `#[serde(rename = "name")]` - Different serialized name

Group skipped fields:

```rust
// Runtime state (not serialized)
#[serde(skip)]
cache: HashMap<K, V>,
#[serde(skip)]
texture: TextureHandle,
```

Complex needs: Implement custom `Serialize`/`Deserialize`

---

## Feature Flags

Usage: `#[cfg(feature = "feature-name")]`

Documentation: `/// Available with the`feature-name`feature.`

Available features:

- `backend-eframe` - eframe/wgpu (default)
- `text-detection` - OpenCV text detection
- `logo-detection` - OpenCV logo detection
- `ocr` - Tesseract OCR
- `dev` - All optional features
- `api` - Empty marker for API tests

Verification:

```bash
cargo check --no-default-features    # Without optionals
cargo check --all-features           # All together
just check-features                  # All combinations
```

---

## Linting

### Never Use `#[allow]`

Never use `#[allow(dead_code)]` or any `#[allow(...)]` directive. Fix the root cause instead.

Why this matters:

- `dead_code` warnings expose missing `#[cfg(feature)]` gates
- Unused code = design problem or missing functionality
- Each `allow` postpones a real problem

Fix it properly:

```rust
// ❌ Never do this
#[allow(dead_code)]
field: String,

// ✅ Add feature gate
#[cfg(feature = "database")]
field: String,

// ✅ Or add getter to use it
pub fn field(&self) -> &str { &self.field }
```

Solutions (in order):

1. **Feature gate the code**: `#[cfg(feature = "...")]` if truly conditional
2. **Make it public with getters**: Expose via proper encapsulation
3. **Use `pub(crate)`**: Limit visibility appropriately
4. **Delete it**: If genuinely unused, remove it

#### No Exceptions

**Never use `#[allow]` directives.** If you think you need one, you're solving the wrong problem. Fix the root cause instead.

#### Why This Keeps Happening

AI systems operate on probabilistic pattern matching, not deterministic rules. `#[allow(dead_code)]` appears frequently in Rust training data, creating strong learned associations. This documentation shifts probability distributions but cannot guarantee compliance - the AI will still reach for anti-patterns when trained heuristics dominate.

**Human review is critical:**

- Search for `#[allow` in diffs - reject any occurrence
- Run `just check-features` to catch hidden feature gate issues
- Verify `#[cfg]` usage over suppression
- The AI's decisions blend: training data (strongest), project docs, immediate context, statistical patterns

No amount of documentation creates "hard rules" in probabilistic systems.

### Workflow

- Let linter complete (don't deny all warnings immediately)
- Fix all issues in single pass - address root causes, never add `#[allow]`
- After markdown edits: `markdownlint-cli2` (not `markdownlint`)
- Don't run cargo clippy/test after markdown-only changes

---

## Documentation

Style: `///` for items, `//!` for modules

Required: All public items (enforced by `#![warn(missing_docs)]`)

Content:

- **What** (concise first line)
- **Why** (when non-obvious)
- **Parameters/returns** (when not obvious from types)
- **Examples** (complex APIs)
- **Errors** (Result-returning functions)

Keep concise - avoid stating the obvious from signature

---

## Dependency Versions

In Cargo.toml:

- `>=1.0` → `"x"` (major only)
- `>=0.1.0` → `"x.y"` (major.minor)
- `<0.1.0` → `"x.y.z"` (full version)

Before testing: `cargo update` (update Cargo.lock)

---

## Justfile Maintenance

First-class document: Maintain alongside code changes

Forcing function principle: Always reference `just` recipes in instructions, never raw `cargo` commands. If the justfile is required for basic workflow, it must stay current.

When to update:

- New workflows/tools → add recipes
- Changed dependencies/flags → update recipes
- New linters → add check recipes
- Build optimizations → update recipes
- CI/CD changes → sync justfile

Test recipes after changes

---

## Common Patterns

State machine extraction: Multiple booleans representing mutually exclusive states → extract to enum

Borrow checker: Simultaneous immutable/mutable borrows → extract needed values before mutable borrow:

```rust
let value = *self.field();  // Extract first
self.mutate();              // Then mutably borrow
```

---

## Unsafe

Forbidden: Use `#![forbid(unsafe_code)]` in `lib.rs`

---

## Commit Best Practices

When to commit:

- Only when user explicitly requests
- After all checks pass
- All tests passing, zero warnings

Commit message format:

```bash
git commit -m "$(cat <<'EOF'
type(scope): Brief description

Detailed explanation of what and why.

Key changes:
- Bullet point 1
- Bullet point 2

Testing:
- Test coverage details

Files modified:
- path/to/file.rs - what changed

🤖 Generated with [Claude Code](https://claude.com/claude-code)

Co-Authored-By: Claude <noreply@anthropic.com>
EOF
)"
```

Git safety:

- Never update git config
- Never destructive commands (force push, hard reset) unless explicit
- Never skip hooks (--no-verify) unless explicit
- Never force push to main/master
- Check authorship before amend: `git log -1 --format='%an %ae'`

---

## Release Management

### Development Tools Setup

Install all required tools:

```bash
just setup  # Installs just, cargo-audit, cargo-dist, cargo-release, git-cliff, etc.
```

### Changelog Management (git-cliff)

Configuration: `cliff.toml`

Commands:

```bash
just changelog-preview  # Preview unreleased changes
just changelog-update   # Prepend unreleased to CHANGELOG.md
just changelog-full     # Regenerate entire CHANGELOG.md
```

### Release Process (cargo-release)

Configuration: `release.toml`

Commands:

```bash
just release-dry-run [patch|minor|major]  # Preview without changes
just release [patch|minor|major]          # Execute release
```

### Pre-Release Checklist

Run the complete pre-release workflow:

```bash
just pre-release
```

This will:
1. Run CI pipeline (fmt, lint, features, tests, audit)
2. Run security checks (audit, omnibor)
3. Generate changelog preview
4. Test release dry-run
5. Build release artifacts

Then manually:
1. Review changelog preview
2. Update changelog: `just changelog-update`
3. Commit changelog updates
4. Execute release: `just release [patch|minor|major]`

### cargo-dist

Files:

- `dist-workspace.toml` - config
- `.github/workflows/release.yml` - auto-generated

Commands:

```bash
just dist-build     # Build artifacts
just dist-check     # Verify without upload
just dist-plan      # Preview release
just dist-generate  # Update CI workflow
```

Process:

1. Run `just pre-release`
2. Update changelog with `just changelog-update`
3. Commit changelog
4. Release: `just release [patch|minor|major]`
5. GitHub Actions builds and publishes artifacts

### Supply Chain Security

```bash
just audit       # Check vulnerabilities
just omnibor     # Generate artifact tree
just security    # All checks
```

Before release:

- `just security`
- Update deps: `just update-deps`
- Re-run full test suite

---

## Summary

Top priorities:

1. ✅ Fix all issues before commit
2. ✅ All public functions instrumented
3. ✅ Use derive_more for all errors
4. ✅ Always use builders, never literals
5. ✅ Tests in `tests/`, never inline
6. ✅ lib.rs only has mod + pub use
7. ✅ Import as `use crate::{Type}`
8. ✅ No re-exports between workspace crates
