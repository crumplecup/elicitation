//! AxumTestGenPlugin — emit axum handler test and integration test fragments.

use elicitation::{ElicitPlugin, elicit_tool};
use rmcp::ErrorData;
use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ── Types ─────────────────────────────────────────────────────────────────────

/// A step in an integration test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TestStep {
    /// Human-readable description of this step.
    pub description: String,
    /// HTTP method lowercase (e.g. `get`, `post`).
    pub method: String,
    /// URL path (e.g. `/users/1`).
    pub path: String,
    /// Optional JSON request body.
    pub body: Option<String>,
    /// Expected HTTP status code.
    pub expected_status: u16,
}

/// A field name and its default value expression for test state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct FieldDefault {
    /// Field name.
    pub field: String,
    /// Default value expression.
    pub value: String,
}

// ── Params ────────────────────────────────────────────────────────────────────

/// Parameters for emit_handler_test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerTestParams {
    /// Handler function name to test.
    pub handler_name: String,
    /// HTTP method lowercase.
    pub method: String,
    /// URL path.
    pub path: String,
    /// Optional JSON request body.
    pub request_body: Option<String>,
    /// Expected HTTP status code.
    pub expected_status: u16,
}

/// Parameters for emit_integration_test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitIntegrationTestParams {
    /// Test function name.
    pub test_name: String,
    /// Ordered test steps.
    pub steps: Vec<TestStep>,
}

/// Parameters for emit_auth_test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitAuthTestParams {
    /// Authentication scheme/type being tested.
    pub auth_type: String,
    /// A valid token value.
    pub valid_token: String,
    /// An invalid token value.
    pub invalid_token: String,
}

/// Parameters for emit_crud_test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitCrudTestParams {
    /// Resource name in singular lowercase (e.g. `user`).
    pub resource: String,
    /// Example JSON body for create/update operations.
    pub sample_json: String,
}

/// Parameters for emit_websocket_test.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitWsTestParams {
    /// WebSocket endpoint path.
    pub path: String,
    /// Messages to send during the test.
    pub messages: Vec<String>,
}

/// Parameters for emit_test_state.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitTestStateParams {
    /// State struct type name.
    pub state_type: String,
    /// Field names and their default value expressions.
    pub field_defaults: Vec<FieldDefault>,
}

/// Parameters for emit_test_client.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitTestClientParams {
    /// Function name that builds the `Router` (e.g. `app`).
    pub app_fn: String,
}

/// Parameters for emit_test_helpers.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct EmitTestHelpersParams {
    /// HTTP methods to generate helper functions for (e.g. `["get", "post", "delete"]`).
    pub methods: Vec<String>,
}

// ── Tools ─────────────────────────────────────────────────────────────────────

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_handler_test",
    description = "Emit a single handler test using axum-test or reqwest to assert the response status."
)]
#[instrument]
async fn emit_handler_test(p: EmitHandlerTestParams) -> Result<CallToolResult, ErrorData> {
    let body_line = match &p.request_body {
        Some(body) => format!("\n    let body = serde_json::json!({});", body),
        None => String::new(),
    };
    let send_call = match p.method.to_lowercase().as_str() {
        "post" | "put" | "patch" => format!(
            "client.{}(\"{}\").json(&body).send().await.unwrap()",
            p.method.to_lowercase(),
            p.path
        ),
        _ => format!(
            "client.{}(\"{}\").send().await.unwrap()",
            p.method.to_lowercase(),
            p.path
        ),
    };
    let expected = p.expected_status;
    let code = format!(
        r#"#[tokio::test]
async fn test_{handler}() {{
    let (client, _server) = test_client({handler}).await;{body_line}
    let response = {send_call};
    assert_eq!(response.status().as_u16(), {expected}, "Expected HTTP {expected}");
}}"#,
        handler = p.handler_name,
        body_line = body_line,
        send_call = send_call,
        expected = expected,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_integration_test",
    description = "Emit a multi-step integration test asserting each step's response status."
)]
#[instrument]
async fn emit_integration_test(p: EmitIntegrationTestParams) -> Result<CallToolResult, ErrorData> {
    let steps = p
        .steps
        .iter()
        .map(|s| {
            let body_setup = s.body.as_ref().map(|b| format!("\n    let body = serde_json::json!({});", b)).unwrap_or_default();
            let send = match s.method.to_lowercase().as_str() {
                "post" | "put" | "patch" => format!(
                    "client.{}(\"{}\").json(&body).send().await.unwrap()",
                    s.method.to_lowercase(),
                    s.path
                ),
                _ => format!(
                    "client.{}(\"{}\").send().await.unwrap()",
                    s.method.to_lowercase(),
                    s.path
                ),
            };
            format!(
                "    // Step: {}\n    {{\n        {}\n        let response = {};\n        assert_eq!(response.status().as_u16(), {});\n    }}",
                s.description, body_setup.trim_start(), send, s.expected_status
            )
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let code = format!(
        "#[tokio::test]\nasync fn {}() {{\n    let (client, _server) = test_client(app).await;\n\n{}\n}}",
        p.test_name, steps
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_auth_test",
    description = "Emit three auth test functions: valid token (200), invalid token (401), no token (401)."
)]
#[instrument]
async fn emit_auth_test(p: EmitAuthTestParams) -> Result<CallToolResult, ErrorData> {
    let auth_type = &p.auth_type;
    let valid = &p.valid_token;
    let invalid = &p.invalid_token;
    let code = format!(
        r#"#[tokio::test]
async fn test_auth_with_valid_token() {{
    let (client, _server) = test_client(app).await;
    let response = client
        .get("/protected")
        .header("Authorization", "{auth_type} {valid}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
}}

#[tokio::test]
async fn test_auth_with_invalid_token() {{
    let (client, _server) = test_client(app).await;
    let response = client
        .get("/protected")
        .header("Authorization", "{auth_type} {invalid}")
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 401);
}}

#[tokio::test]
async fn test_auth_with_no_token() {{
    let (client, _server) = test_client(app).await;
    let response = client.get("/protected").send().await.unwrap();
    assert_eq!(response.status().as_u16(), 401);
}}"#,
        auth_type = auth_type,
        valid = valid,
        invalid = invalid,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_crud_test",
    description = "Emit a full CRUD test suite: test_list, test_create, test_get, test_update, test_delete."
)]
#[instrument]
async fn emit_crud_test(p: EmitCrudTestParams) -> Result<CallToolResult, ErrorData> {
    let r = &p.resource;
    let json = &p.sample_json;
    let code = format!(
        r#"#[tokio::test]
async fn test_list_{r}s() {{
    let (client, _server) = test_client(app).await;
    let response = client.get("/{r}s").send().await.unwrap();
    assert_eq!(response.status().as_u16(), 200);
}}

#[tokio::test]
async fn test_create_{r}() {{
    let (client, _server) = test_client(app).await;
    let body = serde_json::json!({json});
    let response = client.post("/{r}s").json(&body).send().await.unwrap();
    assert_eq!(response.status().as_u16(), 201);
}}

#[tokio::test]
async fn test_get_{r}() {{
    let (client, _server) = test_client(app).await;
    let response = client.get("/{r}s/1").send().await.unwrap();
    assert!(response.status().as_u16() == 200 || response.status().as_u16() == 404);
}}

#[tokio::test]
async fn test_update_{r}() {{
    let (client, _server) = test_client(app).await;
    let body = serde_json::json!({json});
    let response = client.put("/{r}s/1").json(&body).send().await.unwrap();
    assert!(response.status().as_u16() == 200 || response.status().as_u16() == 404);
}}

#[tokio::test]
async fn test_delete_{r}() {{
    let (client, _server) = test_client(app).await;
    let response = client.delete("/{r}s/1").send().await.unwrap();
    assert!(response.status().as_u16() == 204 || response.status().as_u16() == 404);
}}"#,
        r = r,
        json = json,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_websocket_test",
    description = "Emit a WebSocket test that sends messages and asserts on responses."
)]
#[instrument]
async fn emit_websocket_test(p: EmitWsTestParams) -> Result<CallToolResult, ErrorData> {
    let send_lines = p
        .messages
        .iter()
        .map(|m| {
            format!(
                "    ws.send(Message::Text(\"{}\".to_string())).await.unwrap();\n    let reply = ws.recv().await.unwrap().unwrap();\n    assert!(matches!(reply, Message::Text(_)));",
                m
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        r#"#[tokio::test]
async fn test_websocket() {{
    use tokio_tungstenite::{{connect_async, tungstenite::Message}};

    let (listener, _server) = test_listener(app).await;
    let addr = listener.local_addr().unwrap();
    let url = format!("ws://{{}}{}", addr);
    let (mut ws, _) = connect_async(url).await.unwrap();

{send_lines}

    ws.close(None).await.unwrap();
}}"#,
        p.path,
        send_lines = send_lines,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_test_state",
    description = "Emit a test fixture function that constructs a state value with specified field defaults."
)]
#[instrument]
async fn emit_test_state(p: EmitTestStateParams) -> Result<CallToolResult, ErrorData> {
    let state_lower = p.state_type.to_lowercase();
    let fields = p
        .field_defaults
        .iter()
        .map(|fd| format!("        {}: {},", fd.field, fd.value))
        .collect::<Vec<_>>()
        .join("\n");
    let code = format!(
        "fn test_{state_lower}() -> {state_type} {{\n    {state_type} {{\n{fields}\n    }}\n}}",
        state_lower = state_lower,
        state_type = p.state_type,
        fields = fields,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_test_client",
    description = "Emit a test client setup: binds a listener, spawns axum::serve, returns a reqwest Client."
)]
#[instrument]
async fn emit_test_client(p: EmitTestClientParams) -> Result<CallToolResult, ErrorData> {
    let app_fn = &p.app_fn;
    let code = format!(
        r#"async fn test_client(app: Router) -> (reqwest::Client, tokio::task::JoinHandle<()>) {{
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {{
        axum::serve(listener, {app_fn}()).await.unwrap();
    }});
    let client = reqwest::Client::builder()
        .base_url(format!("http://{{addr}}"))
        .build()
        .unwrap();
    (client, server)
}}"#,
        app_fn = app_fn,
    );
    Ok(CallToolResult::success(vec![Content::text(code)]))
}

#[elicit_tool(
    plugin = "axum_test_gen",
    emit = None,
    name = "emit_test_helpers",
    description = "Emit per-method helper functions (make_get_request, make_post_json, etc.) and assert_status."
)]
#[instrument]
async fn emit_test_helpers(p: EmitTestHelpersParams) -> Result<CallToolResult, ErrorData> {
    let mut helpers = Vec::new();

    for method in &p.methods {
        let helper = match method.to_lowercase().as_str() {
            "get" => r#"async fn make_get_request(client: &reqwest::Client, path: &str) -> reqwest::Response {
    client.get(path).send().await.unwrap()
}"#
            .to_string(),
            "post" => r#"async fn make_post_json(client: &reqwest::Client, path: &str, body: &serde_json::Value) -> reqwest::Response {
    client.post(path).json(body).send().await.unwrap()
}"#
            .to_string(),
            "put" => r#"async fn make_put_json(client: &reqwest::Client, path: &str, body: &serde_json::Value) -> reqwest::Response {
    client.put(path).json(body).send().await.unwrap()
}"#
            .to_string(),
            "patch" => r#"async fn make_patch_json(client: &reqwest::Client, path: &str, body: &serde_json::Value) -> reqwest::Response {
    client.patch(path).json(body).send().await.unwrap()
}"#
            .to_string(),
            "delete" => r#"async fn make_delete_request(client: &reqwest::Client, path: &str) -> reqwest::Response {
    client.delete(path).send().await.unwrap()
}"#
            .to_string(),
            m => format!(
                "async fn make_{m}_request(client: &reqwest::Client, path: &str) -> reqwest::Response {{\n    client.{m}(path).send().await.unwrap()\n}}",
                m = m
            ),
        };
        helpers.push(helper);
    }

    helpers.push(
        r#"fn assert_status(response: &reqwest::Response, expected: u16) {
    assert_eq!(
        response.status().as_u16(),
        expected,
        "Expected HTTP {expected} but got {}",
        response.status()
    );
}"#
        .to_string(),
    );

    Ok(CallToolResult::success(vec![Content::text(
        helpers.join("\n\n"),
    )]))
}

/// Plugin exposing axum handler test and integration test generation tools.
#[derive(Debug, ElicitPlugin)]
#[plugin(name = "axum_test_gen")]
pub struct AxumTestGenPlugin;
