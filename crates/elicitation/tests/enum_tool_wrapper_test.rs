//! Test that enum types work with #[elicit_tools] via auto-generated wrappers
//!
//! MCP requires object schemas ("type": "object"), but enums generate
//! enum schemas without type fields. The macro now wraps all types in
//! a struct to ensure proper schema generation.

use elicitation::{Affirm, Elicit, ElicitToolOutput, Prompt, Select};
use elicitation_macros::elicit_tools;
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::service::RoleServer;
use rmcp::{ServerHandler, tool, tool_router};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Test enum that would fail without wrapper
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Another test enum with data
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Status {
    Pending,
    InProgress { progress: u8 },
    Completed { result: String },
    Failed { error: String },
}

/// Test struct (should work with or without wrapper)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct Task {
    pub name: String,
    pub priority: Priority,
    pub status: Status,
}

/// Test server with enum tools
pub struct EnumTestServer;

#[elicit_tools(Priority, Status, Task)]
#[tool_router]
impl EnumTestServer {
    // No methods needed - just testing enum tool generation
}

impl ServerHandler for EnumTestServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[test]
fn test_enum_wrapper_structs_generated() {
    // Verify wrapper struct exists and works
    let _priority_wrapper = ElicitToolOutput::new(Priority::High);
    let _status_wrapper = ElicitToolOutput::new(Status::Pending);
    let _task_wrapper = ElicitToolOutput::new(Task {
        name: "Test".to_string(),
        priority: Priority::High,
        status: Status::Pending,
    });
}

#[test]
fn test_enum_wrapper_serialization() {
    use serde_json;

    let wrapper = ElicitToolOutput::new(Priority::High);

    let json = serde_json::to_value(&wrapper).expect("Serialize wrapper");

    // Should be an object with "value" field
    assert!(json.is_object());
    assert!(json.get("value").is_some());
    assert_eq!(json["value"], "high");
}

#[test]
fn test_enum_wrapper_schema_is_object() {
    use schemars::schema_for;

    let schema = schema_for!(ElicitToolOutput<Priority>);

    // MCP requires root schema to be object type
    assert!(schema.as_object().is_some(), "Schema should be an object");

    // Verify it has properties (not an enum at root)
    let obj = schema.as_object().unwrap();
    assert!(
        obj.get("properties").is_some(),
        "Should have properties field"
    );
}

#[test]
fn test_status_wrapper_schema_is_object() {
    use schemars::schema_for;

    let schema = schema_for!(ElicitToolOutput<Status>);

    // Even complex enums with data should have object wrapper
    assert!(schema.as_object().is_some(), "Schema should be an object");
}

#[test]
fn test_server_compiles() {
    // Just verify the server with enum tools compiles
    // Actual runtime registration tested in integration tests
    let server = EnumTestServer;
    let _info = server.get_info();
}

#[test]
fn test_tool_methods_exist() {
    // Verify all expected methods exist
    // Type system proves they exist if this compiles

    // Priority tool
    let _: fn(rmcp::service::Peer<RoleServer>) -> _ = EnumTestServer::elicit_priority;

    // Status tool
    let _: fn(rmcp::service::Peer<RoleServer>) -> _ = EnumTestServer::elicit_status;

    // Task tool
    let _: fn(rmcp::service::Peer<RoleServer>) -> _ = EnumTestServer::elicit_task;
}

#[test]
fn test_wrapper_derives() {
    // Verify wrapper has all necessary derives
    use std::fmt::Debug;

    fn assert_serde<T: Serialize + for<'de> Deserialize<'de>>() {}
    fn assert_schema<T: JsonSchema>() {}

    assert_serde::<ElicitToolOutput<Priority>>();
    assert_serde::<ElicitToolOutput<Status>>();
    assert_serde::<ElicitToolOutput<Task>>();

    assert_schema::<ElicitToolOutput<Priority>>();
    assert_schema::<ElicitToolOutput<Status>>();
    assert_schema::<ElicitToolOutput<Task>>();
}

#[test]
fn test_nested_enum_in_struct() {
    // Verify that struct containing enums still works
    let task = Task {
        name: "Deploy".to_string(),
        priority: Priority::Critical,
        status: Status::InProgress { progress: 75 },
    };

    let wrapper = ElicitToolOutput::new(task);

    let json = serde_json::to_value(&wrapper).expect("Serialize");
    assert!(json.is_object());
    assert!(json["value"].is_object());
    assert_eq!(json["value"]["name"], "Deploy");
}
