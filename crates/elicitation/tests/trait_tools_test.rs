//! Test for #[elicit_trait_tools] macro
//!
//! Tests generating MCP tools from trait definitions.

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
    async fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> Result<Json<EchoResult>, rmcp::ErrorData>;
}

/// Test implementation
pub struct EchoHandler;

impl EchoTrait for EchoHandler {
    async fn echo(
        &self,
        params: Parameters<EchoParams>,
    ) -> Result<Json<EchoResult>, rmcp::ErrorData> {
        Ok(Json(EchoResult {
            echoed: params.0.message,
        }))
    }
}

/// Test server with generic handler
pub struct EchoServer<H: EchoTrait> {
    handler: H,
}

#[elicit_trait_tools_router(EchoTrait, handler)]
#[tool_router(router = echo_tools)]
impl<H: EchoTrait> EchoServer<H> {}

impl<H: EchoTrait> ServerHandler for EchoServer<H> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}

#[test]
fn test_simple_trait_tool_generation() {
    let handler = EchoHandler;
    let server = EchoServer { handler };
    
    // Verify server compiles and has generated tool_router
    let _router = EchoServer::<EchoHandler>::echo_tools();
}

#[test]
fn test_simple_trait_has_echo_method() {
    // Verify the generated method exists
    let _: fn(&EchoServer<EchoHandler>, Parameters<EchoParams>) -> _ = EchoServer::echo;
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
pub struct MathResult {
    pub result: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiplyParams {
    pub x: i32,
    pub y: i32,
}

/// Math operations trait
pub trait MathOps: Send + Sync {
    /// Add two numbers
    async fn add(
        &self,
        params: Parameters<AddParams>,
    ) -> Result<Json<MathResult>, rmcp::ErrorData>;

    /// Multiply two numbers
    async fn multiply(
        &self,
        params: Parameters<MultiplyParams>,
    ) -> Result<Json<MathResult>, rmcp::ErrorData>;
}

pub struct Calculator;

impl MathOps for Calculator {
    async fn add(
        &self,
        params: Parameters<AddParams>,
    ) -> Result<Json<MathResult>, rmcp::ErrorData> {
        Ok(Json(MathResult {
            result: params.0.a + params.0.b,
        }))
    }

    async fn multiply(
        &self,
        params: Parameters<MultiplyParams>,
    ) -> Result<Json<MathResult>, rmcp::ErrorData> {
        Ok(Json(MathResult {
            result: params.0.x * params.0.y,
        }))
    }
}

pub struct MathServer<C: MathOps> {
    calculator: C,
}

#[elicit_trait_tools_router(MathOps, calculator)]
#[tool_router(router = math_tools)]
impl<C: MathOps> MathServer<C> {}

impl<C: MathOps> ServerHandler for MathServer<C> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::default()
    }
}

#[test]
fn test_multiple_methods_compile() {
    let calc = Calculator;
    let server = MathServer { calculator: calc };
    
    let _router = MathServer::<Calculator>::math_tools();
}

#[test]
fn test_multiple_methods_exist() {
    // Verify both generated methods exist
    let _: fn(&MathServer<Calculator>, Parameters<AddParams>) -> _ = MathServer::add;
    let _: fn(&MathServer<Calculator>, Parameters<MultiplyParams>) -> _ = MathServer::multiply;
}

// ============================================================================
// Test 3: Tool router integration
// ============================================================================

#[test]
fn test_tool_router_discovers_methods() {
    let calc = Calculator;
    let server = MathServer { calculator: calc };
    
    let router = MathServer::<Calculator>::math_tools();
    let tools = router.list_all();
    
    // Should have 2 tools registered (add + multiply)
    assert_eq!(tools.len(), 2);
    
    let tool_names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(tool_names.contains(&"add"));
    assert!(tool_names.contains(&"multiply"));
}
