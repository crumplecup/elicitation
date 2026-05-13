# elicit_axum — Complete Web Framework Harvesting Plan

> **Completionist mandate:** Expose the entire axum web framework API as MCP tools.
> **Three-pronged approach:** Runtime tools (JSON boundary) + Fragment tools (code generation) + Factory pattern (trait method exposure).

---

## Executive Summary

**Scope:** Axum 0.8.8 + axum-core 0.5.6 + Tower ecosystem
**Strategy:** Harvest 100% of the API surface using established patterns
**Estimated tools:** 800-1000 MCP tools
**Key insight:** Axum is trait-heavy, but traits can be exposed via factory pattern. Handlers/middleware that take closures can be fragment tools.

---

## The Three Patterns Applied to Axum

### Pattern 1: Runtime Tools (JSON Boundary)

**What crosses JSON:**

- Router handles (UUID registry)
- Extractors with serializable data (Path, Query, Json)
- Response builders with serializable params
- Configuration structs (CorsLayer settings, TimeoutLayer duration)

**Example:**

```rust
#[elicit_tool(plugin = "axum_router", name = "create")]
async fn router_create() -> Result<CallToolResult, ErrorData> {
    let router = Router::new();
    let id = Uuid::new_v4();
    ROUTERS.lock().insert(id, router);
    Ok(CallToolResult::success(json!({ "router_id": id })))
}
```

### Pattern 2: Fragment Tools (Code Generation)

**What becomes fragments:**

- Handler function definitions (take closures)
- Middleware function definitions (take closures)
- Route registration code
- Complete service assembly

**Example:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "handler_fn",
    emit = Auto
)]
async fn emit_handler(p: EmitHandlerParams) -> Result<CallToolResult, ErrorData> {
    // Returns TokenStream for:
    // async fn handler_name(extractor1: Type1, ...) -> impl IntoResponse { body }
    let code = generate_handler_code(p);
    Ok(CallToolResult::success(Content::text(code)))
}
```

### Pattern 3: Factory Pattern (Trait Methods)

**Traits to expose:**

- `Handler<T, S>` - blanket impl for functions
- `FromRequest<S>` / `FromRequestParts<S>` - extractor trait
- `IntoResponse` - response conversion trait
- `Service<Req>` - Tower service trait
- `Layer<S>` - Tower layer trait

**Example:**

```rust
// Wrapper trait with blanket impl
pub trait FromRequestJson<S>: Sized {
    fn from_request_json(req: &str, state: &S) -> Result<Self, String>;
}

impl<T, S> FromRequestJson<S> for T
where
    T: FromRequest<S>,
    T::Rejection: std::fmt::Display,
{
    fn from_request_json(req: &str, state: &S) -> Result<Self, String> {
        // Deserialize req JSON, create Request, call T::from_request
        // ...
    }
}

#[reflect_trait(crate::FromRequestJson)]
pub trait FromRequestJsonTools<S>: Sized {
    fn from_request_json(req: &str, state: &S) -> Result<Self, String>;
}
```

---

## Architecture: Four Shadow Crates

### Crate 1: elicit_tower

**Purpose:** Tower Service/Layer traits and tower-http middleware
**Patterns:** Factory (Service, Layer traits) + Runtime (middleware config)

**Harvest:**

- `Service<Request>` trait via factory pattern
- `Layer<S>` trait via factory pattern
- 25+ tower-http middleware layers:
  - Compression, CORS, Tracing, Timeout, RequestID
  - Validation, SetHeaders, Catch Panic, Rate Limiting
  - Map Request/Response, Buffer, Retry, Load Balancing

### Crate 2: elicit_axum_core

**Purpose:** Core traits (FromRequest, IntoResponse, FromRef)
**Patterns:** Factory (all three traits)

**Harvest:**

- `FromRequest<S>` / `FromRequestParts<S>` via factory
- `IntoResponse` / `IntoResponseParts` via factory
- `FromRef<T>` via factory
- Core body types (Body, BodyDataStream)
- Response building utilities

### Crate 3: elicit_axum

**Purpose:** Main router, extractors, responses, handlers
**Patterns:** Runtime (router handles) + Fragment (handler/route emission) + Factory (Handler trait)

**Harvest:**

- Router/MethodRouter (runtime handles + fragment emission)
- 20+ extractors (Path, Query, Json, Form, Multipart, WebSocket, etc.)
- Response types (Html, Redirect, SSE, AppendHeaders)
- Handler trait via factory
- Middleware builders

### Crate 4: elicit_axum_fragments

**Purpose:** Pure code generation tools
**Patterns:** Fragment only

**Harvest:**

- `handler_fn` - emit handler function
- `route_def` - emit route registration
- `middleware_fn` - emit middleware function
- `service_scaffold` - emit complete service boilerplate
- `state_struct` - emit state type definition
- `assemble_service` - complete binary with Cargo.toml

---

## Phase 1: elicit_tower — Service/Layer Foundation

### 1.1 Service Trait (Factory Pattern)

**Wrapper trait:**

```rust
// Erase the generic Future by using JSON round-trip
pub trait ServiceJson<Req>: Clone + Send + Sync + 'static {
    fn call_json(&mut self, req: &str) -> Result<String, String>;
    fn poll_ready_json(&mut self) -> Result<bool, String>;
}

impl<T, Req, Res> ServiceJson<Req> for T
where
    T: Service<Req, Response = Res> + Clone + Send + Sync + 'static,
    T::Error: std::fmt::Display,
    T::Future: Send + 'static,
    Req: serde::de::DeserializeOwned,
    Res: serde::Serialize,
{
    fn call_json(&mut self, req: &str) -> Result<String, String> {
        let request: Req = serde_json::from_str(req).map_err(|e| e.to_string())?;
        // Poll the future returned by call() - requires async context
        // Return serialized response
        // ...
    }

    fn poll_ready_json(&mut self) -> Result<bool, String> {
        // Create waker, poll poll_ready
        // ...
    }
}

#[reflect_trait(crate::ServiceJson)]
pub trait ServiceJsonTools<Req>: Clone + Send + Sync + 'static {
    fn call_json(&mut self, req: &str) -> Result<String, String>;
    fn poll_ready_json(&mut self) -> Result<bool, String>;
}
```

**Runtime tools** (stateful service registry):

```rust
pub struct TowerServicePlugin {
    services: Arc<Mutex<HashMap<Uuid, Box<dyn ServiceJson<Request>>>>>,
}

#[elicit_tool(plugin = "tower_service", name = "call")]
async fn service_call(
    ctx: Arc<PluginContext>,
    params: ServiceCallParams,
) -> Result<CallToolResult, ErrorData> {
    let mut services = ctx.services.lock().unwrap();
    let svc = services.get_mut(&params.service_id)
        .ok_or_else(|| ErrorData::new("Service not found"))?;

    let result = svc.call_json(&params.request_json)?;
    Ok(CallToolResult::success(Content::text(result)))
}
```

### 1.2 Layer Trait (Factory Pattern)

**Wrapper trait:**

```rust
pub trait LayerJson<S>: Clone + Send + Sync + 'static {
    type Service: ServiceJson<Request>;
    fn layer_json(&self, inner_id: Uuid) -> Self::Service;
}

impl<T, S> LayerJson<S> for T
where
    T: Layer<S> + Clone + Send + Sync + 'static,
    T::Service: ServiceJson<Request>,
{
    type Service = T::Service;
    fn layer_json(&self, inner_id: Uuid) -> Self::Service {
        // Lookup inner service by UUID, apply layer
        // ...
    }
}

#[reflect_trait(crate::LayerJson)]
pub trait LayerJsonTools<S>: Clone + Send + Sync + 'static {
    type Service: ServiceJson<Request>;
    fn layer_json(&self, inner_id: Uuid) -> Self::Service;
}
```

### 1.3 Tower-HTTP Middleware (25+ Layers)

**Runtime configuration tools** (each layer has builder):

#### CompressionLayer

```rust
elicit_newtype!(pub struct CompressionLayer(tower_http::compression::CompressionLayer));

#[reflect_methods]
impl CompressionLayer {
    pub fn new() -> CompressionLayer;
    pub fn quality(self, level: CompressionLevel) -> Self;
    pub fn gzip(self, enable: bool) -> Self;
    pub fn br(self, enable: bool) -> Self;
    pub fn zstd(self, enable: bool) -> Self;
    pub fn deflate(self, enable: bool) -> Self;
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum CompressionLevel {
    Default, Fastest, Best, Precise(u32),
}
```

#### CorsLayer

```rust
#[reflect_methods]
impl CorsLayer {
    pub fn new() -> CorsLayer;
    pub fn permissive() -> CorsLayer;
    pub fn very_permissive() -> CorsLayer;
    pub fn allow_credentials(self, allow: bool) -> Self;
    pub fn allow_headers(self, headers: Vec<String>) -> Self;
    pub fn allow_methods(self, methods: Vec<String>) -> Self;
    pub fn allow_origin(self, origin: AllowOrigin) -> Self;
    pub fn allow_private_network(self, allow: bool) -> Self;
    pub fn expose_headers(self, headers: Vec<String>) -> Self;
    pub fn max_age(self, seconds: u64) -> Self;
    pub fn vary(self, headers: Vec<String>) -> Self;
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum AllowOrigin {
    Any,
    Exact(String),
    List(Vec<String>),
    Mirror,
}
```

**Fragment tools** (emit layer application code):

```rust
#[elicit_tool(
    plugin = "tower_fragments",
    name = "apply_cors",
    emit = Auto
)]
async fn emit_cors_layer(p: EmitCorsParams) -> Result<CallToolResult, ErrorData> {
    // Returns:
    // .layer(CorsLayer::new().allow_origin(AllowOrigin::Any).allow_methods(vec!["GET", "POST"]))
    let code = generate_cors_code(p);
    Ok(CallToolResult::success(Content::text(code)))
}
```

**All 25+ middleware:**

- CompressionLayer / DecompressionLayer
- CorsLayer
- TraceLayer (tracing spans)
- TimeoutLayer
- RequestIdLayer / PropagateRequestIdLayer
- SetRequestHeaderLayer / SetResponseHeaderLayer
- OverrideRequestHeaderLayer / OverrideResponseHeaderLayer
- SetSensitiveHeadersLayer
- CatchPanicLayer
- RequestBodyLimitLayer
- ValidateRequestHeaderLayer (Bearer, Basic, Custom)
- NormalizePathLayer
- AddExtensionLayer
- ClassifyResponseLayer
- RequireAuthorizationLayer / AsyncRequireAuthorizationLayer
- MapRequestBodyLayer / MapResponseBodyLayer
- FollowRedirectLayer
- MetricsLayer

---

## Phase 2: elicit_axum_core — Core Trait Foundation

### 2.1 FromRequest Trait (Factory Pattern)

**Wrapper traits:**

```rust
// FromRequest<S> → FromRequestJson<S>
pub trait FromRequestJson<S>: Sized {
    fn from_request_json(req_json: &str, state_json: &str) -> Result<String, String>;
}

impl<T, S> FromRequestJson<S> for T
where
    T: FromRequest<S>,
    T::Rejection: std::fmt::Display,
    S: serde::de::DeserializeOwned,
{
    fn from_request_json(req_json: &str, state_json: &str) -> Result<String, String> {
        // Deserialize Request and State, call from_request(), serialize result
        // ...
    }
}

#[reflect_trait(crate::FromRequestJson)]
pub trait FromRequestJsonTools<S>: Sized {
    fn from_request_json(req_json: &str, state_json: &str) -> Result<String, String>;
}

// FromRequestParts<S> → FromRequestPartsJson<S>
pub trait FromRequestPartsJson<S>: Sized {
    fn from_request_parts_json(parts_json: &str, state_json: &str) -> Result<String, String>;
}

// Similar blanket impl + #[reflect_trait]
```

### 2.2 IntoResponse Trait (Factory Pattern)

**Wrapper trait:**

```rust
pub trait IntoResponseJson {
    fn into_response_json(&self) -> Result<String, String>;
}

impl<T> IntoResponseJson for T
where
    T: IntoResponse + serde::Serialize,
{
    fn into_response_json(&self) -> Result<String, String> {
        let response = self.clone().into_response();
        // Serialize Response<Body> as JSON
        let json = response_to_json(response)?;
        Ok(json)
    }
}

#[reflect_trait(crate::IntoResponseJson)]
pub trait IntoResponseJsonTools {
    fn into_response_json(&self) -> Result<String, String>;
}
```

### 2.3 FromRef Trait (Factory Pattern)

```rust
pub trait FromRefJson<T> {
    fn from_ref_json(input_json: &str) -> Result<String, String>;
}

impl<T, U> FromRefJson<T> for U
where
    U: FromRef<T> + serde::Serialize,
    T: serde::de::DeserializeOwned,
{
    fn from_ref_json(input_json: &str) -> Result<String, String> {
        let input: T = serde_json::from_str(input_json)?;
        let result = U::from_ref(&input);
        Ok(serde_json::to_string(&result)?)
    }
}

#[reflect_trait(crate::FromRefJson)]
pub trait FromRefJsonTools<T> {
    fn from_ref_json(input_json: &str) -> Result<String, String>;
}
```

---

## Phase 3: elicit_axum — Router (Runtime + Fragment)

### 3.1 Router Runtime Tools

**UUID registry pattern:**

```rust
pub struct AxumRouterPlugin {
    routers: Arc<Mutex<HashMap<Uuid, Router>>>,
    method_routers: Arc<Mutex<HashMap<Uuid, MethodRouter>>>,
}

#[elicit_tool(plugin = "axum_router", name = "create")]
async fn router_create() -> Result<CallToolResult, ErrorData> {
    let router = Router::new();
    let id = Uuid::new_v4();
    ROUTERS.lock().insert(id, router);
    Ok(CallToolResult::success(json!({ "router_id": id })))
}

#[elicit_tool(plugin = "axum_router", name = "route")]
async fn router_route(
    ctx: Arc<PluginContext>,
    params: RouterRouteParams,
) -> Result<CallToolResult, ErrorData> {
    let mut routers = ctx.routers.lock().unwrap();
    let router = routers.remove(&params.router_id)
        .ok_or_else(|| ErrorData::new("Router not found"))?;

    let method_routers = ctx.method_routers.lock().unwrap();
    let method_router = method_routers.get(&params.method_router_id)
        .ok_or_else(|| ErrorData::new("MethodRouter not found"))?
        .clone();

    let new_router = router.route(&params.path, method_router);
    let new_id = Uuid::new_v4();
    routers.insert(new_id, new_router);

    Ok(CallToolResult::success(json!({ "router_id": new_id })))
}

#[elicit_tool(plugin = "axum_router", name = "nest")]
async fn router_nest(...) -> Result<CallToolResult, ErrorData> {
    // Similar pattern
}

#[elicit_tool(plugin = "axum_router", name = "merge")]
async fn router_merge(...) -> Result<CallToolResult, ErrorData> {
    // Similar pattern
}

#[elicit_tool(plugin = "axum_router", name = "layer")]
async fn router_layer(...) -> Result<CallToolResult, ErrorData> {
    // Apply tower Layer to router
}

#[elicit_tool(plugin = "axum_router", name = "with_state")]
async fn router_with_state(...) -> Result<CallToolResult, ErrorData> {
    // Attach state (serialized)
}
```

### 3.2 Router Fragment Tools

**Emit route registration code:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "route_def",
    description = "Emit route registration code: router.route(path, method_router)",
    emit = Auto
)]
async fn emit_route_def(p: EmitRouteParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // .route("/users", get(list_users).post(create_user))
    let code = format!(
        ".route(\"{}\", {})",
        p.path,
        generate_method_router_code(&p.methods)
    );
    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "axum_fragments",
    name = "nest_def",
    emit = Auto
)]
async fn emit_nest_def(p: EmitNestParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // .nest("/api", api_router)
    let code = format!(".nest(\"{}\", {})", p.path, p.router_name);
    Ok(CallToolResult::success(Content::text(code)))
}
```

### 3.3 MethodRouter (Runtime + Fragment)

**Runtime tools:**

```rust
#[elicit_tool(plugin = "axum_method_router", name = "get")]
async fn method_router_get(
    ctx: Arc<PluginContext>,
    params: MethodRouterGetParams,
) -> Result<CallToolResult, ErrorData> {
    // Register handler_id in handler registry
    // Create MethodRouter::new().get(handler)
    // Store in method_routers map
    let id = Uuid::new_v4();
    // ...
    Ok(CallToolResult::success(json!({ "method_router_id": id })))
}
```

**Fragment tools:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "method_chain",
    emit = Auto
)]
async fn emit_method_chain(p: EmitMethodChainParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // get(handler1).post(handler2).delete(handler3)
    let methods = p.methods.iter()
        .map(|(method, handler)| format!(".{}({})", method, handler))
        .collect::<Vec<_>>()
        .join("");
    Ok(CallToolResult::success(Content::text(methods)))
}
```

---

## Phase 4: Extractors (20+ Types)

### 4.1 Extractor Runtime Tools

Each extractor gets:

- Constructor tool
- Accessor tools (getters)
- Validation tools

**Example: Path extractor**

```rust
#[elicit_tool(plugin = "axum_extract", name = "path_string")]
async fn extract_path_string(p: ExtractPathParams) -> Result<CallToolResult, ErrorData> {
    // Parse path template, extract value
    // Returns: { "value": "extracted" }
    Ok(CallToolResult::success(json!({ "value": p.extracted })))
}
```

**All 20+ extractors:**

- State<T> - application state
- Extension<T> - request extensions
- Path<T> - path parameters
- RawPathParams - raw path iterator
- Query<T> - query string parameters
- RawQuery - raw query string
- Json<T> - JSON body
- Form<T> - form data
- Bytes - raw bytes
- String - UTF-8 string
- Multipart - multipart form data
- WebSocketUpgrade - websocket handshake
- ConnectInfo<T> - connection metadata
- MatchedPath - matched route path
- OriginalUri - pre-routing URI
- Request - full HTTP request
- Host - Host header
- HeaderMap - all headers
- TypedHeader<T> - specific header type
- ContentLengthLimit<T> - body size limited extractor

### 4.2 Extractor Fragment Tools

**Emit extractor in handler signature:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "extractor_param",
    emit = Auto
)]
async fn emit_extractor_param(p: EmitExtractorParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // Path(id): Path<u64>
    // Query(params): Query<UserQuery>
    // Json(body): Json<CreateUserRequest>
    let code = match p.extractor_type.as_str() {
        "path" => format!("Path({}): Path<{}>", p.binding, p.type_param),
        "query" => format!("Query({}): Query<{}>", p.binding, p.type_param),
        "json" => format!("Json({}): Json<{}>", p.binding, p.type_param),
        "state" => format!("State({}): State<{}>", p.binding, p.type_param),
        _ => return Err(ErrorData::new("Unknown extractor")),
    };
    Ok(CallToolResult::success(Content::text(code)))
}
```

---

## Phase 5: Response Types (Runtime + Fragment)

### 5.1 Response Runtime Tools

**Json Response:**

```rust
#[elicit_tool(plugin = "axum_response", name = "json")]
async fn response_json(p: JsonResponseParams) -> Result<CallToolResult, ErrorData> {
    // Serialize to Json<T>
    // Return handle or direct JSON
    Ok(CallToolResult::success(json!({
        "type": "json",
        "content": p.value
    })))
}
```

**Html Response:**

```rust
#[elicit_tool(plugin = "axum_response", name = "html")]
async fn response_html(p: HtmlResponseParams) -> Result<CallToolResult, ErrorData> {
    Ok(CallToolResult::success(json!({
        "type": "html",
        "content": p.html
    })))
}
```

**Redirect:**

```rust
#[elicit_tool(plugin = "axum_response", name = "redirect")]
async fn response_redirect(p: RedirectParams) -> Result<CallToolResult, ErrorData> {
    let redirect_type = match p.redirect_type.as_str() {
        "temporary" => "temporary",
        "permanent" => "permanent",
        _ => "to",
    };
    Ok(CallToolResult::success(json!({
        "type": "redirect",
        "redirect_type": redirect_type,
        "uri": p.uri
    })))
}
```

**SSE (Server-Sent Events):**

```rust
#[elicit_tool(plugin = "axum_response", name = "sse_event")]
async fn sse_event(p: SseEventParams) -> Result<CallToolResult, ErrorData> {
    // Build Event with data, event name, id, retry
    Ok(CallToolResult::success(json!({
        "data": p.data,
        "event": p.event,
        "id": p.id,
        "retry": p.retry_ms
    })))
}
```

### 5.2 Response Fragment Tools

**Emit response in handler body:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "return_json",
    emit = Auto
)]
async fn emit_return_json(p: EmitReturnJsonParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // Ok(Json(user))
    let code = format!("Ok(Json({}))", p.value_expr);
    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "axum_fragments",
    name = "return_html",
    emit = Auto
)]
async fn emit_return_html(p: EmitReturnHtmlParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // Ok(Html(html_string))
    let code = format!("Ok(Html({}))", p.html_expr);
    Ok(CallToolResult::success(Content::text(code)))
}
```

---

## Phase 6: Handler Trait + Code Generation

### 6.1 Handler Trait (Factory Pattern)

**Wrapper trait:**

```rust
pub trait HandlerJson<T, S>: Clone + Send + Sync + 'static {
    fn call_handler_json(&self, req_json: &str, state_json: &str) -> Result<String, String>;
}

impl<H, T, S> HandlerJson<T, S> for H
where
    H: Handler<T, S>,
    S: Clone + Send + Sync + 'static,
{
    fn call_handler_json(&self, req_json: &str, state_json: &str) -> Result<String, String> {
        // Deserialize Request and State, call handler, serialize response
        // ...
    }
}

#[reflect_trait(crate::HandlerJson)]
pub trait HandlerJsonTools<T, S>: Clone + Send + Sync + 'static {
    fn call_handler_json(&self, req_json: &str, state_json: &str) -> Result<String, String>;
}
```

### 6.2 Handler Fragment Tools (Complete)

**Emit full handler function:**

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct EmitHandlerParams {
    pub name: String,
    pub extractors: Vec<ExtractorDef>,
    pub return_type: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ExtractorDef {
    pub extractor_type: String,  // "path", "query", "json", "state"
    pub binding: String,          // Variable name
    pub type_param: String,       // Type argument
}

#[elicit_tool(
    plugin = "axum_fragments",
    name = "handler_fn",
    description = "Emit a complete async handler function",
    emit = Auto
)]
async fn emit_handler_fn(p: EmitHandlerParams) -> Result<CallToolResult, ErrorData> {
    let extractors = p.extractors.iter()
        .map(|e| emit_extractor_param_code(e))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"async fn {name}({extractors}) -> Result<{return_type}, AppError> {{
    {body}
}}"#,
        name = p.name,
        extractors = extractors,
        return_type = p.return_type,
        body = indent_body(&p.body)
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**Example agent workflow:**

```text
1. emit_handler_fn({
     name: "create_user",
     extractors: [
       { extractor_type: "state", binding: "state", type_param: "AppState" },
       { extractor_type: "json", binding: "payload", type_param: "CreateUserRequest" }
     ],
     return_type: "Json<User>",
     body: "let user = state.db.create_user(payload).await?;\nOk(Json(user))"
   })

2. route_def({ path: "/users", methods: [{ method: "post", handler: "create_user" }] })

3. assemble_service({ routes: [...], state_type: "AppState", ... })
```

---

## Phase 7: Middleware (Factory + Fragment)

### 7.1 Middleware Fragment Tools

**from_fn middleware:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "middleware_fn",
    description = "Emit middleware function that wraps handler with before/after logic",
    emit = Auto
)]
async fn emit_middleware_fn(p: EmitMiddlewareParams) -> Result<CallToolResult, ErrorData> {
    let extractors = p.extractors.iter()
        .map(|e| emit_extractor_param_code(e))
        .collect::<Vec<_>>()
        .join(", ");

    let code = format!(
        r#"async fn {name}({extractors}, next: Next) -> Result<Response, StatusCode> {{
    // Before
    {before}

    // Call next
    let mut response = next.run(req).await;

    // After
    {after}

    Ok(response)
}}"#,
        name = p.name,
        extractors = extractors,
        before = indent_body(&p.before),
        after = indent_body(&p.after)
    );

    Ok(CallToolResult::success(Content::text(code)))
}
```

**from_extractor middleware:**

```rust
#[elicit_tool(
    plugin = "axum_fragments",
    name = "middleware_from_extractor",
    emit = Auto
)]
async fn emit_middleware_from_extractor(p: EmitFromExtractorParams) -> Result<CallToolResult, ErrorData> {
    // Emits:
    // .layer(middleware::from_extractor::<RequireAuth>())
    let code = format!(".layer(middleware::from_extractor::<{}>())", p.extractor_type);
    Ok(CallToolResult::success(Content::text(code)))
}
```

### 7.2 Middleware Factory Tools

**Wrap middleware functions as trait:**

```rust
pub trait MiddlewareJson {
    fn apply_middleware_json(&self, req_json: &str, next_id: Uuid) -> Result<String, String>;
}

// Blanket impl for middleware functions
// #[reflect_trait] for tool exposure
```

---

## Phase 8: Complete Service Assembly (Fragment)

### 8.1 Service Scaffold Tool

**Emit complete main.rs + Cargo.toml:**

```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AssembleServiceParams {
    pub package_name: String,
    pub routes: Vec<RouteDefinition>,
    pub state_type: Option<StateDefinition>,
    pub middleware: Vec<String>,  // Layer names
    pub port: u16,
    pub with_tracing: bool,
    pub with_graceful_shutdown: bool,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RouteDefinition {
    pub path: String,
    pub handler_name: String,
    pub method: String,  // "GET", "POST", etc.
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StateDefinition {
    pub type_name: String,
    pub fields: Vec<StateField>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct StateField {
    pub name: String,
    pub field_type: String,
}

#[elicit_tool(
    plugin = "axum_fragments",
    name = "assemble_service",
    description = "Generate complete axum web service with main.rs and Cargo.toml",
    emit = Auto
)]
async fn assemble_service(p: AssembleServiceParams) -> Result<CallToolResult, ErrorData> {
    let main_rs = generate_main_rs(&p);
    let cargo_toml = generate_cargo_toml(&p);

    Ok(CallToolResult::success(json!({
        "main_rs": main_rs,
        "cargo_toml": cargo_toml,
        "handlers": extract_handler_names(&p.routes)
    })))
}

fn generate_main_rs(p: &AssembleServiceParams) -> String {
    format!(r#"
use axum::{{
    routing::{{get, post, put, delete}},
    Router,
    Json,
    extract::{{Path, Query, State}},
}};
use tokio::net::TcpListener;
{}

{}

#[tokio::main]
async fn main() {{
    {}

    let app = Router::new()
        {}
        {};

    let listener = TcpListener::bind("0.0.0.0:{}").await.unwrap();
    println!("Listening on {{}}", listener.local_addr().unwrap());

    {}
}}
"#,
        if p.with_tracing { "tracing_subscriber::fmt::init();" } else { "" },
        generate_state_struct(p.state_type.as_ref()),
        generate_state_init(p.state_type.as_ref()),
        generate_routes(&p.routes),
        generate_middleware_chain(&p.middleware),
        p.port,
        generate_serve_call(p.with_graceful_shutdown)
    )
}
```

**Example output:**

```rust
// Generated main.rs
use axum::{
    routing::{get, post},
    Router, Json,
    extract::{Path, State},
};
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    db: DatabasePool,
    api_key: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        db: connect_database().await,
        api_key: std::env::var("API_KEY").unwrap(),
    };

    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/:id", get(get_user).put(update_user).delete(delete_user))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.db.query("SELECT * FROM users").await?;
    Ok(Json(users))
}

// ... other handlers
```

---

## Phase 9: HTTP Re-exports (Runtime)

### 9.1 Status Codes

**Elicit enum:**

```rust
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum StatusCode {
    // 1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,

    // 2xx Success
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,

    // 3xx Redirection
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx Client Errors
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    UnprocessableEntity = 422,
    TooManyRequests = 429,

    // 5xx Server Errors
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
}

// Conversion to/from http::StatusCode
```

### 9.2 Headers

**Header names enum:**

```rust
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum HeaderName {
    Accept,
    AcceptCharset,
    AcceptEncoding,
    AcceptLanguage,
    AcceptRanges,
    AccessControlAllowCredentials,
    AccessControlAllowHeaders,
    AccessControlAllowMethods,
    AccessControlAllowOrigin,
    AccessControlExposeHeaders,
    AccessControlMaxAge,
    AccessControlRequestHeaders,
    AccessControlRequestMethod,
    Age,
    Allow,
    Authorization,
    CacheControl,
    Connection,
    ContentDisposition,
    ContentEncoding,
    ContentLanguage,
    ContentLength,
    ContentLocation,
    ContentRange,
    ContentType,
    Cookie,
    Date,
    Etag,
    Expect,
    Expires,
    Host,
    IfMatch,
    IfModifiedSince,
    IfNoneMatch,
    IfRange,
    IfUnmodifiedSince,
    LastModified,
    Link,
    Location,
    MaxForwards,
    Origin,
    Pragma,
    ProxyAuthenticate,
    ProxyAuthorization,
    Range,
    Referer,
    RetryAfter,
    SecWebsocketAccept,
    SecWebsocketExtensions,
    SecWebsocketKey,
    SecWebsocketProtocol,
    SecWebsocketVersion,
    Server,
    SetCookie,
    StrictTransportSecurity,
    Te,
    Trailer,
    TransferEncoding,
    Upgrade,
    UserAgent,
    Vary,
    Via,
    Warning,
    WwwAuthenticate,
    XContentTypeOptions,
    XFrameOptions,
    Custom(String),
}
```

### 9.3 Methods

```rust
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Method {
    GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, TRACE, CONNECT,
}
```

---

## Estimated Tool Count

| Category | Runtime Tools | Fragment Tools | Factory Tools | Total |
|---|---|---|---|---|
| **elicit_tower** | | | | |
| Service/Layer traits | 10 | 5 | 20 | 35 |
| 25+ middleware layers | 150 | 25 | 0 | 175 |
| **elicit_axum_core** | | | | |
| FromRequest/IntoResponse | 0 | 0 | 30 | 30 |
| Body/Response utilities | 20 | 10 | 0 | 30 |
| **elicit_axum** | | | | |
| Router/MethodRouter | 15 | 20 | 0 | 35 |
| 20+ extractors | 100 | 40 | 0 | 140 |
| Response types | 50 | 30 | 10 | 90 |
| Handler trait | 5 | 15 | 15 | 35 |
| Middleware | 20 | 25 | 10 | 55 |
| Error handling | 10 | 5 | 0 | 15 |
| Server/Serving | 15 | 10 | 5 | 30 |
| **elicit_axum_fragments** | | | | |
| Handler generation | 0 | 50 | 0 | 50 |
| Route generation | 0 | 30 | 0 | 30 |
| Service assembly | 0 | 20 | 0 | 20 |
| **HTTP re-exports** | | | | |
| StatusCode/Headers/Methods | 100 | 20 | 0 | 120 |
| **Total** | **495** | **305** | **90** | **890** |

---

## Implementation Timeline

**Week 1:** elicit_tower (Service/Layer + 10 middleware)
**Week 2:** elicit_tower (remaining 15 middleware)
**Week 3:** elicit_axum_core (all trait factories)
**Week 4:** elicit_axum (Router runtime + fragments)
**Week 5:** elicit_axum (Extractors)
**Week 6:** elicit_axum (Responses + Handler trait)
**Week 7:** elicit_axum (Middleware)
**Week 8:** elicit_axum_fragments (complete code generation)
**Week 9:** HTTP re-exports + Integration testing
**Week 10:** Documentation, examples, refinement

**Total:** 10 weeks for complete implementation

---

## Success Criteria

1. ✅ 100% of axum 0.8.8 public API exposed
2. ✅ All traits exposed via factory pattern
3. ✅ Complete code generation pipeline (handlers → routes → service)
4. ✅ Runtime router manipulation via UUID registry
5. ✅ Agent can generate working web services from scratch
6. ✅ All 890 tools registered and tested
7. ✅ Comprehensive documentation with 20+ examples

---

## Key Innovations

1. **Triple Harvest:** Runtime + Fragment + Factory covers 100% of API
2. **Trait Factories:** Expose generic traits without wrapping every impl
3. **Code Recovery:** Agents compose JSON tools, recover complete Rust code
4. **Workflow Integration:** Router handles + route fragments = deployable service
5. **Zero Compromise:** Nothing lost from original API surface
