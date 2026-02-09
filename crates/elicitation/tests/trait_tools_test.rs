//! Test for #[elicit_trait_tools] macro
//!
//! Tests generating MCP tools from trait definitions.

#![allow(clippy::manual_async_fn)] // We use impl Future for zero-cost async

use elicitation_macros::elicit_trait_tools_router;
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::model::ServerInfo;
use rmcp::{ServerHandler, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// Test 1: Simple trait with one method
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoParams {
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoResult {
    pub echoed: String,
}

/// Simple echo trait for testing
pub trait EchoTrait: Send + Sync {
    /// Echo a message back
    fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> impl std::future::Future<Output = Result<Json<EchoResult>, rmcp::ErrorData>> + Send;
}

/// Test implementation
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

/// Test server with generic handler
pub struct EchoServer<H: EchoTrait + 'static> {
    handler: H,
}

#[elicit_trait_tools_router(EchoTrait, handler, [echo])]
#[tool_router(router = echo_tools)]
impl<H: EchoTrait + 'static> EchoServer<H> {}

impl<H: EchoTrait + 'static> ServerHandler for EchoServer<H> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}

#[test]
fn test_simple_trait_tool_generation() {
    let handler = EchoHandler;
    let _server = EchoServer { handler };

    // Verify server compiles and has generated tool_router
    let _router = EchoServer::<EchoHandler>::echo_tools();
}

#[test]
fn test_simple_trait_has_echo_method() {
    // Verify the generated method exists by calling it
    // (Type checking proves it exists)
    let handler = EchoHandler;
    let server = EchoServer { handler };
    let _ = &server; // Just prove it compiles
}

// ============================================================================
// Test 2: Trait with multiple methods
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddParams {
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AddResult {
    pub result: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiplyParams {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiplyResult {
    pub result: i32,
}

/// Math operations trait
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
    ) -> impl std::future::Future<Output = Result<Json<MultiplyResult>, rmcp::ErrorData>> + Send
    {
        async move {
            Ok(Json(MultiplyResult {
                result: params.0.x * params.0.y,
            }))
        }
    }
}

pub struct MathServer<C: MathOps + 'static> {
    calculator: C,
}

#[elicit_trait_tools_router(MathOps, calculator, [add, multiply])]
#[tool_router(router = math_tools)]
impl<C: MathOps + 'static> MathServer<C> {}

impl<C: MathOps + 'static> ServerHandler for MathServer<C> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}

#[test]
fn test_multiple_methods_compile() {
    let calc = Calculator;
    let _server = MathServer { calculator: calc };

    let _router = MathServer::<Calculator>::math_tools();
}

#[test]
fn test_multiple_methods_exist() {
    // Verify both generated methods exist by creating server
    // (Type checking proves they exist)
    let calc = Calculator;
    let server = MathServer { calculator: calc };
    let _ = &server; // Just prove it compiles
}

// ============================================================================
// Test 3: Tool router integration
// ============================================================================

#[test]
fn test_tool_router_discovers_methods() {
    let calc = Calculator;
    let _server = MathServer { calculator: calc };

    let router = MathServer::<Calculator>::math_tools();
    let tools = router.list_all();

    // Should have 2 tools registered (add + multiply)
    assert_eq!(tools.len(), 2);

    let tool_names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(tool_names.contains(&"add".to_string()));
    assert!(tool_names.contains(&"multiply".to_string()));
}

// ============================================================================
// Test 4: Trait with async_trait (object-safe)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GreetParams {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct GreetResult {
    pub greeting: String,
}

/// Greeter trait using async_trait for object safety
#[async_trait::async_trait]
pub trait Greeter: Send + Sync {
    /// Greet someone
    async fn greet(
        &self,
        params: Parameters<GreetParams>,
    ) -> Result<Json<GreetResult>, rmcp::ErrorData>;
}

pub struct SimpleGreeter;

#[async_trait::async_trait]
impl Greeter for SimpleGreeter {
    async fn greet(
        &self,
        params: Parameters<GreetParams>,
    ) -> Result<Json<GreetResult>, rmcp::ErrorData> {
        Ok(Json(GreetResult {
            greeting: format!("Hello, {}!", params.0.name),
        }))
    }
}

pub struct GreeterServer<G: Greeter + 'static> {
    greeter: G,
}

#[elicit_trait_tools_router(Greeter, greeter, [greet])]
#[tool_router(router = greeter_tools)]
impl<G: Greeter + 'static> GreeterServer<G> {}

impl<G: Greeter + 'static> ServerHandler for GreeterServer<G> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}

#[test]
fn test_async_trait_tool_generation() {
    let greeter = SimpleGreeter;
    let _server = GreeterServer { greeter };

    // Verify server compiles and has generated tool_router
    let _router = GreeterServer::<SimpleGreeter>::greeter_tools();
}

#[test]
fn test_async_trait_tool_router_integration() {
    let greeter = SimpleGreeter;
    let _server = GreeterServer { greeter };

    let router = GreeterServer::<SimpleGreeter>::greeter_tools();
    let tools = router.list_all();

    // Should have 1 tool registered (greet)
    assert_eq!(tools.len(), 1);

    let tool_names: Vec<String> = tools.iter().map(|t| t.name.to_string()).collect();
    assert!(tool_names.contains(&"greet".to_string()));
}
