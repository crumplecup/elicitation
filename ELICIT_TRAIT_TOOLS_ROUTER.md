# `#[elicit_trait_tools_router]` Macro

## Overview

The `#[elicit_trait_tools_router]` macro automatically generates MCP tools from trait methods, eliminating boilerplate wrapper code. This enables a "tool everything" architecture where entire trait-based APIs can be exposed as MCP tools with minimal ceremony.

**Impact**: Reduces tool wrapper code by ~80-90% for trait-heavy APIs.

## Quick Start

```rust
use elicitation::elicit_trait_tools_router;
use rmcp::{Json, Parameters, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Define parameter and result types following naming convention
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoParams {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoResult {
    pub echoed: String,
}

// Define your trait with appropriate signatures
pub trait EchoTrait: Send + Sync {
    /// Echo a message back
    fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> impl std::future::Future<Output = Result<Json<EchoResult>, rmcp::ErrorData>> + Send;
}

// Implement the trait
pub struct EchoHandler;

impl EchoTrait for EchoHandler {
    fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> impl std::future::Future<Output = Result<Json<EchoResult>, rmcp::ErrorData>> + Send {
        async move {
            Ok(Json(EchoResult {
                echoed: params.0.message,
            }))
        }
    }
}

// Server type holding the trait implementation
pub struct EchoServer<H: EchoTrait + 'static> {
    handler: H,
}

// Apply the macro to auto-generate tools
#[elicit_trait_tools_router(EchoTrait, handler, [echo])]
#[tool_router]
impl<H: EchoTrait + 'static> EchoServer<H> {}
```

The macro generates:

```rust
#[tool_router]
impl<H: EchoTrait + 'static> EchoServer<H> {
    /// Echo a message back
    #[tool]
    pub async fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> Result<Json<EchoResult>, rmcp::ErrorData> {
        self.handler.echo(params).await
    }
}
```

## Syntax

```rust
#[elicit_trait_tools_router(TraitName, field_name, [method1, method2, ...])]
```

**Parameters:**
- `TraitName` - The trait to generate tools from
- `field_name` - Field on the impl struct holding the trait implementation
- `[method1, method2, ...]` - List of methods to generate tools for

## Requirements

### Trait Method Signatures

The macro supports **two patterns** for trait methods:

**Pattern 1: `impl Future + Send` (zero-cost)**
```rust
fn method_name(
    &self,
    params: Parameters<MethodParams>,
) -> impl std::future::Future<Output = Result<Json<MethodResult>, rmcp::ErrorData>> + Send;
```

**Pattern 2: `async_trait` (object-safe)**
```rust
#[async_trait::async_trait]
trait MyTrait: Send + Sync {
    async fn method_name(
        &self,
        params: Parameters<MethodParams>,
    ) -> Result<Json<MethodResult>, rmcp::ErrorData>;
}
```

**Key requirements (both patterns):**
1. **Parameters** - Use `rmcp::Parameters<T>` wrapper
2. **Return type** - `Result<Json<T>, rmcp::ErrorData>`
3. **Trait bounds** - Trait must be `Send + Sync`
4. **Type naming** - Follow PascalCase convention (see below)

### Naming Convention

The macro derives type names from method names using PascalCase:

- Method: `echo` → Types: `EchoParams`, `EchoResult`
- Method: `add` → Types: `AddParams`, `AddResult`
- Method: `process_message` → Types: `ProcessMessageParams`, `ProcessMessageResult`

**You must define these types** before using the macro.

### Type Definitions

Parameter and result types must:
- Implement `Serialize`, `Deserialize`, `JsonSchema`
- Be defined before the macro is applied
- Follow the naming convention

```rust
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MethodParams {
    // Your fields
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MethodResult {
    // Your fields
}
```

## Two Patterns for Async Traits

The macro supports both patterns for defining async trait methods:

### Pattern 1: Explicit `impl Future + Send`

Use this for maximum performance (zero-cost abstraction):

```rust
trait MyTrait: Send + Sync {
    fn method(
        &self, 
        params: Parameters<P>
    ) -> impl Future<Output = Result<Json<R>, ErrorData>> + Send;
}

impl MyTrait for Handler {
    fn method(
        &self,
        params: Parameters<P>,
    ) -> impl Future<Output = Result<Json<R>, ErrorData>> + Send {
        async move {
            // Your async code here
            Ok(Json(R { /* ... */ }))
        }
    }
}
```

**Pros:** Zero-cost, no boxing, best performance  
**Cons:** Not object-safe, can't use `Box<dyn MyTrait>`

### Pattern 2: `async_trait` Macro

Use this for object safety (trait objects):

```rust
#[async_trait::async_trait]
trait MyTrait: Send + Sync {
    async fn method(
        &self,
        params: Parameters<P>,
    ) -> Result<Json<R>, ErrorData>;
}

#[async_trait::async_trait]
impl MyTrait for Handler {
    async fn method(
        &self,
        params: Parameters<P>,
    ) -> Result<Json<R>, ErrorData> {
        // Your async code here
        Ok(Json(R { /* ... */ }))
    }
}
```

**Pros:** Object-safe, simpler syntax, works with `Box<dyn MyTrait>`  
**Cons:** Small overhead (boxing), requires `async-trait` dependency

## Object-Safe Traits with `async_trait`

**Good News:** The macro fully supports traits using `#[async_trait]` for object safety!

If you need trait objects (e.g., `Vec<Arc<dyn MyTrait>>`), use `async_trait`:

```rust
use async_trait::async_trait;

// Object-safe trait using async_trait
#[async_trait]
pub trait ResourceHandler: Send + Sync {
    /// List resources
    async fn list(
        &self,
        params: Parameters<ListParams>,
    ) -> Result<Json<ListResult>, rmcp::ErrorData>;
}

// Implementation
pub struct FileResource;

#[async_trait]
impl ResourceHandler for FileResource {
    async fn list(
        &self,
        params: Parameters<ListParams>,
    ) -> Result<Json<ListResult>, rmcp::ErrorData> {
        Ok(Json(ListResult { items: vec![] }))
    }
}

// Server with trait object support
pub struct ResourceServer<R: ResourceHandler + 'static> {
    handler: R,
}

// Macro works with async_trait!
#[elicit_trait_tools_router(ResourceHandler, handler, [list])]
#[tool_router(router = resource_tools)]
impl<R: ResourceHandler + 'static> ResourceServer<R> {}
```

### Choosing Between Patterns

**Use `impl Future + Send` when:**
- Don't need trait objects
- Want zero-cost abstractions (no boxing)
- Performance-critical code

**Use `#[async_trait]` when:**
- Need trait objects (`Box<dyn Trait>`, `Arc<dyn Trait>`)
- Want simpler syntax (`async fn` instead of `impl Future`)
- Working with dynamic dispatch (registries, plugins)

**Both patterns work with the macro!** The generated code calls `.await` which works identically for both.

## Complete Example

```rust
use elicitation::elicit_trait_tools_router;
use rmcp::{Json, Parameters, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Parameter types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddParams {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiplyParams {
    pub x: i32,
    pub y: i32,
}

// Result types
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddResult {
    pub result: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiplyResult {
    pub result: i32,
}

// Trait definition
pub trait MathOps: Send + Sync {
    /// Add two numbers
    fn add(
        &self,
        params: Parameters<AddParams>,
    ) -> impl std::future::Future<Output = Result<Json<AddResult>, rmcp::ErrorData>> + Send;

    /// Multiply two numbers
    fn multiply(
        &self,
        params: Parameters<MultiplyParams>,
    ) -> impl std::future::Future<Output = Result<Json<MultiplyResult>, rmcp::ErrorData>> + Send;
}

// Implementation
pub struct Calculator;

impl MathOps for Calculator {
    fn add(
        &self,
        params: Parameters<AddParams>,
    ) -> impl std::future::Future<Output = Result<Json<AddResult>, rmcp::ErrorData>> + Send {
        async move {
            Ok(Json(AddResult {
                result: params.0.a + params.0.b,
            }))
        }
    }

    fn multiply(
        &self,
        params: Parameters<MultiplyParams>,
    ) -> impl std::future::Future<Output = Result<Json<MultiplyResult>, rmcp::ErrorData>> + Send {
        async move {
            Ok(Json(MultiplyResult {
                result: params.0.x * params.0.y,
            }))
        }
    }
}

// Server with trait field
pub struct MathServer<C: MathOps + 'static> {
    calculator: C,
}

// Generate tools automatically!
#[elicit_trait_tools_router(MathOps, calculator, [add, multiply])]
#[tool_router]
impl<C: MathOps + 'static> MathServer<C> {}
```

## Combining with Regular Tools

You can freely mix generated trait tools with regular `#[tool]` methods:

```rust
#[elicit_trait_tools_router(EventHandler, handler, [process_event])]
#[tool_router]
impl MyServer {
    // Regular tool method
    #[tool(description = "Get server status")]
    pub async fn status(&self) -> Result<Json<Status>, rmcp::ErrorData> {
        Ok(Json(Status { healthy: true }))
    }
    
    // Trait tools generated by macro:
    // - process_event (from EventHandler trait)
}
```

## Benefits

1. **80-90% less boilerplate** - No manual wrapper functions
2. **Type safety** - Compiler checks trait signatures
3. **DRY principle** - Trait methods are single source of truth
4. **Easy maintenance** - Add methods to trait, update macro list
5. **Documentation** - Trait doc comments used for tool descriptions

## Limitations (MVP)

1. **Explicit method list** - Must list methods manually (not auto-scanned)
2. **Naming convention** - Types must follow MethodName → MethodParams/Result pattern
3. **Single field** - Only one trait implementation field supported per macro invocation

Future enhancements may address these limitations.

## Error Messages

**Type not found:**
```
error[E0412]: cannot find type `MethodResult` in this scope
```
→ Define `MethodResult` type following naming convention

**Send bound missing:**
```
error[E0277]: future is not `Send`
```
→ Use `fn` returning `impl Future + Send`, not `async fn`

**Wrong parameter type:**
```
error[E0308]: mismatched types
```
→ Use `Parameters<MethodParams>` not raw type

## See Also

- `#[elicit_tools]` - Generate tools from types implementing `Elicitation`
- `ELICITATION_TRAIT_TOOLS_PROPOSAL.md` - Full design proposal
- Botticelli integration example (in botticelli_mcp crate)

## Future Work

Planned enhancements:
1. Auto-scan trait methods (eliminate explicit list)
2. Flexible type name mapping (override convention)
3. Multiple trait fields (combine multiple trait implementations)
4. Better error messages with suggestions
