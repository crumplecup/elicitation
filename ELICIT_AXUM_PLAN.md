# Elicit Axum - Complete Web Framework Harvesting Plan

> **Completionist mandate:** Expose the entire axum web framework API (Router, extractors, responses, handlers, middleware, Tower integration) as MCP tools for agent composition of web services. No filtering—harvest everything.

---

## Executive Summary

**Scope:** Axum 0.8.8 + axum-core 0.5.6 complete public API
**Target:** Router building, 20+ extractors, response types, handler patterns, middleware utilities, Tower Service/Layer integration
**Strategy:** Three shadow crates (`elicit_axum`, `elicit_axum_core`, `elicit_tower`) with workflow-based composition
**Challenge:** Axum is trait-heavy (FromRequest, IntoResponse, Handler, Service, Layer) - we expose both runtime values AND type-level patterns

---

## Architecture Decision: Three Crates

### Decision: **Three Shadow Crates**

**Rationale:**
- Axum is built on Tower, which is a distinct ecosystem
- axum-core provides traits that third parties implement
- Clean separation: core traits → main framework → Tower middleware

**Crate Structure:**
1. **`elicit_tower`** - Tower Service/Layer traits and middleware
2. **`elicit_axum_core`** - FromRequest/IntoResponse traits and core types
3. **`elicit_axum`** - Router, handlers, extractors, responses, serve

**Dependencies:**
- `elicit_axum` depends on `elicit_axum_core` and `elicit_tower`
- `elicit_axum_core` is standalone (minimal deps)
- `elicit_tower` is standalone

---

## The Axum Challenge: Type-Level Composition

Axum's power comes from **type-level composition** - handlers are inferred from function signatures, extractors are composed via type parameters, state is threaded through generic parameters. This creates challenges for runtime MCP tools.

### Strategy: Dual Representation

**1. Type-Level Tools** (code generation)
- `generate_handler(extractors, body)` → emits Rust code
- `generate_router(routes)` → emits Rust code
- Agents describe the types, we emit the implementations

**2. Value-Level Tools** (runtime manipulation)
- `router_route(router, path, method_router)` → mutates router
- `method_router_get(handler_id)` → registers handler by ID
- Runtime composition of pre-compiled handlers

**Both approaches are exposed** - agents can choose based on context.

---

## Phase 1: Tower Foundation (elicit_tower)

### 1.1 Core Service Trait

**Trait exposure strategy:** Since `Service` is a trait, we expose it via:
1. **Trait reflection** - `#[reflect_trait(Service)]` for type-level tools
2. **Service wrappers** - newtypes for common service types
3. **Service builders** - runtime construction of services

**Types:**
```rust
// Re-export core trait
pub use tower::Service;

// Wrapper for boxed services
elicit_newtype!(pub struct BoxService<T, U, E>(tower::util::BoxService<T, U, E>));

// Common service utilities
elicit_newtype!(pub struct Oneshot<S, Req>(tower::util::Oneshot<S, Req>));
elicit_newtype!(pub struct Ready<T>(tower::util::Ready<T>));
elicit_newtype!(pub struct ServiceFn<F>(tower::service_fn::ServiceFn<F>));
```

**Functions:**
```rust
pub fn service_fn<F>(f: F) -> ServiceFn<F>
    where F: FnMut(Request) -> Future;

pub fn boxed_service<T, U, E>(svc: S) -> BoxService<T, U, E>;
```

**Trait methods via reflection:**
```rust
#[reflect_trait(Service)]
pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;
    fn call(&mut self, req: Request) -> Self::Future;
}
```

---

### 1.2 Core Layer Trait

**Layer Types:**
```rust
pub use tower::Layer;

elicit_newtype!(pub struct Stack<Inner, Outer>(tower::layer::util::Stack<Inner, Outer>));
elicit_newtype!(pub struct Identity(tower::layer::util::Identity));
```

**Trait reflection:**
```rust
#[reflect_trait(Layer)]
pub trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}
```

---

### 1.3 Tower-HTTP Middleware (20+ layers)

**Compression:**
```rust
elicit_newtype!(pub struct CompressionLayer(tower_http::compression::CompressionLayer));
elicit_newtype!(pub struct Compression<S>(tower_http::compression::Compression<S>));

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum CompressionLevel {
    Default,
    Fastest,
    Best,
    Precise(u32),
}
```

**CORS:**
```rust
elicit_newtype!(pub struct CorsLayer(tower_http::cors::CorsLayer));
elicit_newtype!(pub struct Cors<S>(tower_http::cors::Cors<S>));

#[reflect_methods]
impl CorsLayer {
    pub fn new() -> CorsLayer;
    pub fn permissive() -> CorsLayer;
    pub fn very_permissive() -> CorsLayer;

    pub fn allow_credentials(self, allow: bool) -> Self;
    pub fn allow_headers(self, headers: Vec<HeaderName>) -> Self;
    pub fn allow_methods(self, methods: Vec<Method>) -> Self;
    pub fn allow_origin(self, origin: AllowOrigin) -> Self;
    pub fn allow_private_network(self, allow: bool) -> Self;
    pub fn expose_headers(self, headers: Vec<HeaderName>) -> Self;
    pub fn max_age(self, max_age: Duration) -> Self;
    pub fn vary(self, headers: Vec<HeaderName>) -> Self;
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum AllowOrigin {
    Any,
    Exact(String),
    List(Vec<String>),
    Mirror,
    Predicate(/* function */),
}
```

**Tracing:**
```rust
elicit_newtype!(pub struct TraceLayer<M>(tower_http::trace::TraceLayer<M>));
elicit_newtype!(pub struct Trace<S, M>(tower_http::trace::Trace<S, M>));

#[reflect_methods]
impl TraceLayer<DefaultMakeSpan> {
    pub fn new_for_http() -> TraceLayer<DefaultMakeSpan>;
    pub fn new_for_grpc() -> TraceLayer<DefaultMakeSpan>;
}

#[reflect_methods]
impl<M> TraceLayer<M> {
    pub fn make_span_with<N>(self, new_make_span: N) -> TraceLayer<N>;
    pub fn on_request<N>(self, new_on_request: N) -> TraceLayer<M>;
    pub fn on_response<N>(self, new_on_response: N) -> TraceLayer<M>;
    pub fn on_body_chunk<N>(self, new_on_body_chunk: N) -> TraceLayer<M>;
    pub fn on_eos<N>(self, new_on_eos: N) -> TraceLayer<M>;
    pub fn on_failure<N>(self, new_on_failure: N) -> TraceLayer<M>;
}
```

**Timeout:**
```rust
elicit_newtype!(pub struct TimeoutLayer(tower_http::timeout::TimeoutLayer));
elicit_newtype!(pub struct Timeout<S>(tower_http::timeout::Timeout<S>));

#[reflect_methods]
impl TimeoutLayer {
    pub fn new(timeout: Duration) -> TimeoutLayer;
}
```

**Request ID:**
```rust
elicit_newtype!(pub struct RequestIdLayer<M>(tower_http::request_id::RequestIdLayer<M>));
elicit_newtype!(pub struct PropagateRequestIdLayer(tower_http::request_id::PropagateRequestIdLayer));
elicit_newtype!(pub struct SetRequestIdLayer<M>(tower_http::request_id::SetRequestIdLayer<M>));

#[reflect_methods]
impl RequestIdLayer<MakeRequestUuid> {
    pub fn new() -> RequestIdLayer<MakeRequestUuid>;
}

#[reflect_methods]
impl<M> RequestIdLayer<M> {
    pub fn x_request_id(make_request_id: M) -> RequestIdLayer<M>;
}
```

**Set Headers:**
```rust
elicit_newtype!(pub struct SetRequestHeaderLayer<M>(tower_http::set_header::SetRequestHeaderLayer<M>));
elicit_newtype!(pub struct SetResponseHeaderLayer<M>(tower_http::set_header::SetResponseHeaderLayer<M>));
elicit_newtype!(pub struct OverrideRequestHeaderLayer<M>(tower_http::set_header::OverrideRequestHeaderLayer<M>));
elicit_newtype!(pub struct OverrideResponseHeaderLayer<M>(tower_http::set_header::OverrideResponseHeaderLayer<M>));
```

**Sensitive Headers:**
```rust
elicit_newtype!(pub struct SetSensitiveHeadersLayer(tower_http::sensitive_headers::SetSensitiveHeadersLayer));

#[reflect_methods]
impl SetSensitiveHeadersLayer {
    pub fn new(headers: impl IntoIterator<Item = HeaderName>) -> Self;
    pub fn from_shared(headers: Arc<[HeaderName]>) -> Self;
}
```

**Compression Body:**
```rust
elicit_newtype!(pub struct DecompressionLayer(tower_http::decompression::DecompressionLayer));
elicit_newtype!(pub struct Decompression<S>(tower_http::decompression::Decompression<S>));
```

**Catch Panic:**
```rust
elicit_newtype!(pub struct CatchPanicLayer(tower_http::catch_panic::CatchPanicLayer));
elicit_newtype!(pub struct CatchPanic<S>(tower_http::catch_panic::CatchPanic<S>));

#[reflect_methods]
impl CatchPanicLayer {
    pub fn new() -> CatchPanicLayer;
    pub fn custom<H: ResponseForPanic>(response_for_panic: H) -> CatchPanicLayer;
}
```

**Limit:**
```rust
elicit_newtype!(pub struct RequestBodyLimitLayer(tower_http::limit::RequestBodyLimitLayer));
elicit_newtype!(pub struct RequestBodyLimit<S>(tower_http::limit::RequestBodyLimit<S>));

#[reflect_methods]
impl RequestBodyLimitLayer {
    pub fn new(limit: usize) -> RequestBodyLimitLayer;
}
```

**Validate Request:**
```rust
elicit_newtype!(pub struct ValidateRequestHeaderLayer<T>(tower_http::validate_request::ValidateRequestHeaderLayer<T>));
elicit_newtype!(pub struct ValidateRequestHeader<S, T>(tower_http::validate_request::ValidateRequestHeader<S, T>));

// Bearer token validation
#[reflect_methods]
impl ValidateRequestHeaderLayer<Bearer> {
    pub fn bearer(token: &str) -> ValidateRequestHeaderLayer<Bearer>;
}

// Basic auth validation
#[reflect_methods]
impl ValidateRequestHeaderLayer<Basic> {
    pub fn basic(username: &str, password: &str) -> ValidateRequestHeaderLayer<Basic>;
}

// Custom validation
#[reflect_methods]
impl<ResBody> ValidateRequestHeaderLayer<Custom<ResBody>> {
    pub fn custom<F>(f: F) -> ValidateRequestHeaderLayer<Custom<ResBody>>
        where F: Fn(&HeaderMap) -> Result<(), Response<ResBody>>;
}
```

**Normalize Path:**
```rust
elicit_newtype!(pub struct NormalizePathLayer(tower_http::normalize_path::NormalizePathLayer));
```

**Add Extension:**
```rust
elicit_newtype!(pub struct AddExtensionLayer<T>(tower_http::add_extension::AddExtensionLayer<T>));

#[reflect_methods]
impl<T: Clone + Send + Sync + 'static> AddExtensionLayer<T> {
    pub fn new(value: T) -> AddExtensionLayer<T>;
}
```

**Classify Response:**
```rust
elicit_newtype!(pub struct ClassifyResponseLayer<C>(tower_http::classify::ClassifyResponseLayer<C>));

// Shared classifier types
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ServerErrorsAsFailures { /* ... */ }

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ServerErrorsFailureClass { /* ... */ }
```

**Auth:**
```rust
elicit_newtype!(pub struct RequireAuthorizationLayer<T>(tower_http::auth::RequireAuthorizationLayer<T>));
elicit_newtype!(pub struct AsyncRequireAuthorizationLayer<T>(tower_http::auth::AsyncRequireAuthorizationLayer<T>));
```

**Map Request/Response Body:**
```rust
elicit_newtype!(pub struct MapRequestBodyLayer<F>(tower_http::map_request_body::MapRequestBodyLayer<F>));
elicit_newtype!(pub struct MapResponseBodyLayer<F>(tower_http::map_response_body::MapResponseBodyLayer<F>));
```

**Follow Redirect:**
```rust
elicit_newtype!(pub struct FollowRedirectLayer(tower_http::follow_redirect::FollowRedirectLayer));
elicit_newtype!(pub struct FollowRedirect<S, P>(tower_http::follow_redirect::FollowRedirect<S, P>));

#[reflect_methods]
impl FollowRedirectLayer {
    pub fn new() -> FollowRedirectLayer;
}
```

**Metrics:**
```rust
elicit_newtype!(pub struct MetricsLayer<C>(tower_http::metrics::MetricsLayer<C>));
elicit_newtype!(pub struct Metrics<S, C>(tower_http::metrics::Metrics<S, C>));
```

---

## Phase 2: Axum-Core Foundation (elicit_axum_core)

### 2.1 Extract Traits

**FromRequest Trait:**
```rust
#[reflect_trait(FromRequest)]
pub trait FromRequest<S>: Sized {
    type Rejection: IntoResponse;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection>;
}
```

**FromRequestParts Trait:**
```rust
#[reflect_trait(FromRequestParts)]
pub trait FromRequestParts<S>: Sized {
    type Rejection: IntoResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection>;
}
```

**Optional Variants:**
```rust
#[reflect_trait(OptionalFromRequest)]
pub trait OptionalFromRequest<S>: Sized {
    type Rejection: IntoResponse;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection>;
}

#[reflect_trait(OptionalFromRequestParts)]
pub trait OptionalFromRequestParts<S>: Sized {
    type Rejection: IntoResponse;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection>;
}
```

**FromRef Trait:**
```rust
#[reflect_trait(FromRef)]
pub trait FromRef<T> {
    fn from_ref(input: &T) -> Self;
}
```

---

### 2.2 Response Traits

**IntoResponse Trait:**
```rust
#[reflect_trait(IntoResponse)]
pub trait IntoResponse {
    fn into_response(self) -> Response;
}
```

**IntoResponseParts Trait:**
```rust
#[reflect_trait(IntoResponseParts)]
pub trait IntoResponseParts {
    type Error: IntoResponse;

    fn into_response_parts(
        self,
        res: ResponseParts,
    ) -> Result<ResponseParts, Self::Error>;
}
```

---

### 2.3 Core Types

**Request Extension:**
```rust
#[reflect_trait(RequestExt)]
pub trait RequestExt<B>: Sized {
    async fn extract<E, M>(self) -> Result<E, E::Rejection>
    where
        E: FromRequest<(), M> + 'static,
        M: 'static;

    async fn extract_with_state<E, S, M>(
        self,
        state: &S,
    ) -> Result<E, E::Rejection>
    where
        E: FromRequest<S, M> + 'static,
        S: Send + Sync,
        M: 'static;
}

#[reflect_trait(RequestPartsExt)]
pub trait RequestPartsExt: Sized {
    async fn extract<E>(self) -> Result<E, E::Rejection>
    where
        E: FromRequestParts<()> + 'static;

    async fn extract_with_state<E, S>(
        self,
        state: &S,
    ) -> Result<E, E::Rejection>
    where
        E: FromRequestParts<S> + 'static,
        S: Send + Sync;
}
```

**Response Types:**
```rust
elicit_newtype!(pub struct Response(http::Response<axum_core::body::Body>));
elicit_newtype!(pub struct ResponseParts(axum_core::response::ResponseParts));
elicit_newtype!(pub struct AppendHeaders<I>(axum_core::response::AppendHeaders<I>));
elicit_newtype!(pub struct ErrorResponse(axum_core::response::ErrorResponse));
```

**Body Types:**
```rust
elicit_newtype!(pub struct Body(axum_core::body::Body));
elicit_newtype!(pub struct BodyDataStream(axum_core::body::BodyDataStream));
```

---

## Phase 3: Axum Main - Router & Routing (elicit_axum)

### 3.1 Router Type

**Core Router:**
```rust
elicit_newtype!(pub struct Router<S = ()>(Arc<axum::Router<S>>));

#[reflect_methods]
impl Router<()> {
    /// Create new router with no state.
    pub fn new() -> Router<()>;
}

#[reflect_methods]
impl<S> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    // Route registration
    pub fn route(self, path: &str, method_router: MethodRouter<S>) -> Self;
    pub fn route_service<T>(self, path: &str, service: T) -> Self
        where T: Service<Request, Response = Response> + Clone + Send + 'static;

    // Nesting
    pub fn nest(self, path: &str, router: Router<S>) -> Self;
    pub fn nest_service<T>(self, path: &str, service: T) -> Self
        where T: Service<Request, Response = Response> + Clone + Send + 'static;

    // Merging
    pub fn merge(self, other: Router<S>) -> Self;

    // Fallback
    pub fn fallback<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn fallback_service<T>(self, service: T) -> Self
        where T: Service<Request, Response = Response> + Clone + Send + 'static;

    // Layers
    pub fn layer<L>(self, layer: L) -> Router<S>
        where L: Layer<Route>;
    pub fn route_layer<L>(self, layer: L) -> Router<S>
        where L: Layer<Route>;

    // State management
    pub fn with_state<S2>(self, state: S) -> Router<S2>;

    // Service conversion
    pub fn into_make_service(self) -> IntoMakeService<Router<S>>;
    pub fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<Router<S>, C>;
    pub fn as_service(&mut self) -> RouterAsService<'_, Body, S>;
    pub fn into_service(self) -> RouterIntoService<Body, S>;
}
```

---

### 3.2 MethodRouter Type

**Method Router:**
```rust
elicit_newtype!(pub struct MethodRouter<S = (), E = Infallible>(axum::routing::MethodRouter<S, E>));

#[reflect_methods]
impl<S> MethodRouter<S, Infallible>
where
    S: Clone + Send + Sync + 'static,
{
    // HTTP methods
    pub fn get<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn post<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn put<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn delete<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn patch<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn head<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn options<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn trace<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn connect<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;

    // Service-based routing
    pub fn get_service<T>(self, service: T) -> MethodRouter<S, T::Error>
        where T: Service<Request, Response = Response>;
    // ... (similar for all methods)

    // Fallback
    pub fn fallback<H, T>(self, handler: H) -> Self
        where H: Handler<T, S>;
    pub fn fallback_service<T>(self, service: T) -> MethodRouter<S, T::Error>
        where T: Service<Request, Response = Response>;

    // Layers
    pub fn layer<L>(self, layer: L) -> MethodRouter<S, E>
        where L: Layer<Route<E>>;
    pub fn route_layer<L>(self, layer: L) -> MethodRouter<S, E>
        where L: Layer<Route<E>>;

    // Merging
    pub fn merge(self, other: MethodRouter<S, E>) -> Self;
}
```

**Top-Level Routing Functions:**
```rust
// Handler-based
pub fn get<H, T, S>(handler: H) -> MethodRouter<S, Infallible>
    where H: Handler<T, S>, S: Clone + Send + Sync + 'static;
pub fn post<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn put<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn delete<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn patch<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn head<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn options<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn trace<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn connect<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn any<H, T, S>(handler: H) -> MethodRouter<S, Infallible>;
pub fn on<H, T, S>(filter: MethodFilter, handler: H) -> MethodRouter<S, Infallible>;

// Service-based
pub fn get_service<T, S>(service: T) -> MethodRouter<S, T::Error>;
pub fn post_service<T, S>(service: T) -> MethodRouter<S, T::Error>;
// ... (all methods)
pub fn any_service<T, S>(service: T) -> MethodRouter<S, T::Error>;
pub fn on_service<T, S>(filter: MethodFilter, service: T) -> MethodRouter<S, T::Error>;
```

---

### 3.3 MethodFilter

**Bitflag Type:**
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MethodFilter(u16);

impl MethodFilter {
    pub const GET: MethodFilter = /* ... */;
    pub const POST: MethodFilter = /* ... */;
    pub const PUT: MethodFilter = /* ... */;
    pub const DELETE: MethodFilter = /* ... */;
    pub const PATCH: MethodFilter = /* ... */;
    pub const HEAD: MethodFilter = /* ... */;
    pub const OPTIONS: MethodFilter = /* ... */;
    pub const TRACE: MethodFilter = /* ... */;
    pub const CONNECT: MethodFilter = /* ... */;
}

#[reflect_methods]
impl MethodFilter {
    pub fn or(self, other: MethodFilter) -> MethodFilter;
    pub fn and(self, other: MethodFilter) -> MethodFilter;
    pub fn contains(&self, other: MethodFilter) -> bool;
    pub fn is_empty(&self) -> bool;
}

// Also derive Elicit for enum representation
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum MethodFilterEnum {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    Combined(Vec<MethodFilterEnum>),
}
```

---

## Phase 4: Extractors (20+ types)

### 4.1 State & Extensions

**State Extractor:**
```rust
elicit_newtype!(pub struct State<T>(axum::extract::State<T>));

#[reflect_methods]
impl<T: Clone> State<T> {
    pub fn new(state: T) -> State<T>;
    pub fn into_inner(self) -> T;
}
```

**Extension Extractor (deprecated):**
```rust
elicit_newtype!(pub struct Extension<T>(axum::extract::Extension<T>));

#[reflect_methods]
impl<T: Clone> Extension<T> {
    pub fn new(extension: T) -> Extension<T>;
    pub fn into_inner(self) -> T;
}
```

---

### 4.2 Path Extractors

**Path Extractor:**
```rust
elicit_newtype!(pub struct Path<T>(axum::extract::Path<T>));

#[reflect_methods]
impl<T> Path<T>
where
    T: DeserializeOwned + Send,
{
    pub fn new(value: T) -> Path<T>;
    pub fn into_inner(self) -> T;
}

// Common path types
pub type PathString = Path<String>;
pub type PathU32 = Path<u32>;
pub type PathU64 = Path<u64>;
pub type PathTuple2<T1, T2> = Path<(T1, T2)>;
pub type PathTuple3<T1, T2, T3> = Path<(T1, T2, T3)>;
pub type PathMap = Path<HashMap<String, String>>;
pub type PathVec = Path<Vec<(String, String)>>;
```

**RawPathParams:**
```rust
elicit_newtype!(pub struct RawPathParams(axum::extract::RawPathParams));

#[reflect_methods]
impl RawPathParams {
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)>;
}
```

**Path Rejections:**
```rust
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum PathRejection {
    FailedToDeserializePathParams(FailedToDeserializePathParams),
    MissingPathParams(MissingPathParams),
}

elicit_newtype!(pub struct FailedToDeserializePathParams(
    axum::extract::rejection::FailedToDeserializePathParams
));

#[reflect_methods]
impl FailedToDeserializePathParams {
    pub fn kind(&self) -> &ErrorKind;
    pub fn into_kind(self) -> ErrorKind;
    pub fn body_text(&self) -> String;
    pub fn status(&self) -> StatusCode;
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum ErrorKind {
    WrongNumberOfParameters { got: usize, expected: usize },
    ParseErrorAtKey { key: String, value: String, expected_type: String },
    ParseErrorAtIndex { index: usize, value: String, expected_type: String },
    ParseError { value: String, expected_type: String },
    InvalidUtf8InPathParam { key: String },
    UnsupportedType { name: String },
    DeserializeError { key: String, value: String, message: String },
}
```

---

### 4.3 Query Extractor

**Query:**
```rust
elicit_newtype!(pub struct Query<T>(axum::extract::Query<T>));

#[reflect_methods]
impl<T> Query<T>
where
    T: DeserializeOwned,
{
    pub fn new(value: T) -> Query<T>;
    pub fn try_from_uri(uri: &Uri) -> Result<Query<T>, QueryRejection>;
    pub fn into_inner(self) -> T;
}
```

**RawQuery:**
```rust
elicit_newtype!(pub struct RawQuery(axum::extract::RawQuery));

#[reflect_methods]
impl RawQuery {
    pub fn as_str(&self) -> &str;
}
```

---

### 4.4 Body Extractors

**JSON:**
```rust
elicit_newtype!(pub struct Json<T>(axum::Json<T>));

#[reflect_methods]
impl<T> Json<T>
where
    T: Serialize,
{
    pub fn new(value: T) -> Json<T>;
    pub fn into_inner(self) -> T;
}

#[reflect_methods]
impl<T> Json<T>
where
    T: DeserializeOwned,
{
    pub fn from_bytes(bytes: &[u8]) -> Result<Json<T>, JsonRejection>;
}

// JSON Rejections
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum JsonRejection {
    JsonDataError(String),
    JsonSyntaxError(String),
    MissingJsonContentType,
    BytesRejection(String),
}
```

**Form:**
```rust
elicit_newtype!(pub struct Form<T>(axum::Form<T>));

#[reflect_methods]
impl<T> Form<T>
where
    T: DeserializeOwned,
{
    pub fn new(value: T) -> Form<T>;
    pub fn into_inner(self) -> T;
}

// Form Rejections
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum FormRejection {
    FailedToDeserializeForm(String),
    FailedToDeserializeFormBody(String),
    InvalidFormContentType,
    BytesRejection(String),
}
```

**Bytes & String:**
```rust
elicit_newtype!(pub struct Bytes(bytes::Bytes));
elicit_newtype!(pub struct BytesBody(axum::body::Bytes));

// String body uses std::string::String directly
```

**Multipart:**
```rust
elicit_newtype!(pub struct Multipart(axum::extract::Multipart));

#[reflect_methods]
impl Multipart {
    pub async fn next_field(&mut self) -> Result<Option<Field>, MultipartError>;
}

elicit_newtype!(pub struct Field(axum::extract::multipart::Field));

#[reflect_methods]
impl Field {
    pub fn name(&self) -> Option<&str>;
    pub fn file_name(&self) -> Option<&str>;
    pub fn content_type(&self) -> Option<&str>;
    pub fn headers(&self) -> &HeaderMap;
    pub async fn text(self) -> Result<String, MultipartError>;
    pub async fn bytes(self) -> Result<Bytes, MultipartError>;
    pub async fn chunk(&mut self) -> Result<Option<Bytes>, MultipartError>;
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MultipartError(String);
```

**Request:**
```rust
// Full request uses http::Request<Body> directly
pub type RequestExtractor = Request;
```

---

### 4.5 WebSocket

**WebSocketUpgrade:**
```rust
elicit_newtype!(pub struct WebSocketUpgrade(axum::extract::ws::WebSocketUpgrade));

#[reflect_methods]
impl WebSocketUpgrade {
    pub fn protocols(self, protocols: impl IntoIterator<Item = impl Into<String>>) -> Self;
    pub fn max_frame_size(self, max: usize) -> Self;
    pub fn max_message_size(self, max: usize) -> Self;
    pub fn write_buffer_size(self, size: usize) -> Self;
    pub fn on_upgrade<F, Fut>(self, callback: F) -> Response
        where
            F: FnOnce(WebSocket) -> Fut + Send + 'static,
            Fut: Future<Output = ()> + Send + 'static;
}

elicit_newtype!(pub struct WebSocket(axum::extract::ws::WebSocket));

#[reflect_methods]
impl WebSocket {
    pub async fn recv(&mut self) -> Option<Result<Message, axum::Error>>;
    pub async fn send(&mut self, msg: Message) -> Result<(), axum::Error>;
    pub async fn close(self) -> Result<(), axum::Error>;
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close(Option<CloseFrame>),
}

#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub struct CloseFrame {
    pub code: u16,
    pub reason: String,
}
```

---

### 4.6 Connection & Matching Info

**ConnectInfo:**
```rust
elicit_newtype!(pub struct ConnectInfo<T>(axum::extract::ConnectInfo<T>));

#[reflect_methods]
impl<T: Clone> ConnectInfo<T> {
    pub fn new(info: T) -> ConnectInfo<T>;
    pub fn into_inner(self) -> T;
}

// Common connection info types
pub type ConnectInfoSocketAddr = ConnectInfo<SocketAddr>;
```

**MatchedPath:**
```rust
elicit_newtype!(pub struct MatchedPath(axum::extract::MatchedPath));

#[reflect_methods]
impl MatchedPath {
    pub fn as_str(&self) -> &str;
}
```

**OriginalUri:**
```rust
elicit_newtype!(pub struct OriginalUri(axum::extract::OriginalUri));

#[reflect_methods]
impl OriginalUri {
    pub fn uri(&self) -> &Uri;
    pub fn into_inner(self) -> Uri;
}
```

---

## Phase 5: Response Types

### 5.1 Structured Responses

**Json Response:**
```rust
// Same type as Json extractor, but used for responses
// Already defined in Phase 4.4
```

**Html Response:**
```rust
elicit_newtype!(pub struct Html<T>(axum::response::Html<T>));

#[reflect_methods]
impl<T> Html<T> {
    pub fn new(html: T) -> Html<T>;
    pub fn into_inner(self) -> T;
}
```

**Form Response:**
```rust
// Same type as Form extractor
// Already defined in Phase 4.4
```

---

### 5.2 Status Responses

**NoContent:**
```rust
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub struct NoContent;

// Implements IntoResponse → 204 No Content
```

---

### 5.3 Redirects

**Redirect:**
```rust
elicit_newtype!(pub struct Redirect(axum::response::Redirect));

#[reflect_methods]
impl Redirect {
    pub fn to(uri: &str) -> Redirect;
    pub fn temporary(uri: &str) -> Redirect;
    pub fn permanent(uri: &str) -> Redirect;
}
```

---

### 5.4 Server-Sent Events (SSE)

**Sse Stream:**
```rust
elicit_newtype!(pub struct Sse<S>(axum::response::sse::Sse<S>));

#[reflect_methods]
impl<S> Sse<S> {
    pub fn new(stream: S) -> Sse<S>
        where S: Stream<Item = Result<Event, impl Into<BoxError>>>;
    pub fn keep_alive(self, keep_alive: KeepAlive) -> Self;
}

elicit_newtype!(pub struct Event(axum::response::sse::Event));

#[reflect_methods]
impl Event {
    pub fn new() -> Event;
    pub fn data<T: Into<String>>(mut self, data: T) -> Self;
    pub fn json_data<T: Serialize>(self, data: T) -> Result<Event, serde_json::Error>;
    pub fn event<T: Into<String>>(mut self, event: T) -> Self;
    pub fn id<T: Into<String>>(mut self, id: T) -> Self;
    pub fn retry(mut self, duration: Duration) -> Self;
    pub fn comment<T: Into<String>>(mut self, comment: T) -> Self;
}

elicit_newtype!(pub struct KeepAlive(axum::response::sse::KeepAlive));

#[reflect_methods]
impl KeepAlive {
    pub fn new() -> KeepAlive;
    pub fn interval(duration: Duration) -> KeepAlive;
    pub fn text(text: impl Into<String>) -> KeepAlive;
    pub fn event(event: Event) -> KeepAlive;
}
```

---

### 5.5 Response Building

**AppendHeaders:**
```rust
elicit_newtype!(pub struct AppendHeaders<I>(axum_core::response::AppendHeaders<I>));

#[reflect_methods]
impl<I> AppendHeaders<I> {
    pub fn new(headers: I) -> AppendHeaders<I>
        where I: IntoIterator<Item = (HeaderName, HeaderValue)>;
}
```

**Response Tuple Types:**
Since axum supports tuple responses, expose builder functions:
```rust
pub fn response_with_status<T>(status: StatusCode, body: T) -> (StatusCode, T)
    where T: IntoResponse;

pub fn response_with_headers<T>(headers: HeaderMap, body: T) -> (HeaderMap, T)
    where T: IntoResponse;

pub fn response_with_status_and_headers<T>(
    status: StatusCode,
    headers: HeaderMap,
    body: T,
) -> (StatusCode, HeaderMap, T)
    where T: IntoResponse;
```

---

## Phase 6: Handler Trait & Services

### 6.1 Handler Trait

**Trait Reflection:**
```rust
#[reflect_trait(Handler)]
pub trait Handler<T, S>: Clone + Send + Sync + Sized + 'static {
    type Future: Future<Output = Response> + Send + 'static;

    fn call(self, req: Request, state: S) -> Self::Future;

    fn layer<L>(self, layer: L) -> Layered<L, Self, T, S>
        where L: Layer<HandlerService<Self, T, S>>;

    fn with_state(self, state: S) -> HandlerService<Self, T, S>;
}
```

**Handler Services:**
```rust
elicit_newtype!(pub struct HandlerService<H, T, S>(axum::handler::HandlerService<H, T, S>));
elicit_newtype!(pub struct Layered<L, H, T, S>(axum::handler::Layered<L, H, T, S>));
```

**Extension Trait:**
```rust
#[reflect_trait(HandlerWithoutStateExt)]
pub trait HandlerWithoutStateExt<T>: Handler<T, ()> {
    fn into_service(self) -> HandlerService<Self, T, ()>;
    fn into_make_service(self) -> IntoMakeService<HandlerService<Self, T, ()>>;
    fn into_make_service_with_connect_info<C>(
        self,
    ) -> IntoMakeServiceWithConnectInfo<HandlerService<Self, T, ()>, C>;
}
```

---

### 6.2 Handler Code Generation

**Challenge:** Handlers are inferred from function signatures. Agents can't create Rust functions at runtime.

**Solution:** Handler Builder Pattern
```rust
pub struct HandlerBuilder<S> {
    extractors: Vec<ExtractorSpec>,
    body: HandlerBody,
    state_type: PhantomData<S>,
}

pub enum ExtractorSpec {
    Path(String),     // Path parameter name
    Query,            // Query string
    Json,             // JSON body
    Form,             // Form body
    State,            // Application state
    Extension(TypeId), // Extension type
    Header(HeaderName), // Header value
    Custom(String),   // Custom extractor name
}

pub enum HandlerBody {
    Inline(String),   // Inline Rust code
    Function(String), // Function name to call
}

impl<S> HandlerBuilder<S> {
    pub fn new() -> HandlerBuilder<S>;
    pub fn extract_path(self, name: &str) -> Self;
    pub fn extract_query(self) -> Self;
    pub fn extract_json(self) -> Self;
    pub fn extract_form(self) -> Self;
    pub fn extract_state(self) -> Self;
    pub fn body_inline(self, code: &str) -> Self;
    pub fn body_function(self, name: &str) -> Self;

    // Code generation
    pub fn generate_handler_code(&self) -> String;
    pub fn emit_to_file(&self, path: &Path) -> io::Result<()>;
}
```

**Usage Pattern:**
```rust
// Agent builds handler description
let handler = HandlerBuilder::new()
    .extract_path("id")
    .extract_state()
    .extract_json()
    .body_inline(r#"
        let User { name, email } = json.0;
        let saved = state.db.create_user(name, email).await?;
        Ok(Json(saved))
    "#)
    .generate_handler_code();

// Emits:
// async fn handler(
//     Path(id): Path<String>,
//     State(state): State<AppState>,
//     Json(json): Json<CreateUserRequest>,
// ) -> Result<Json<User>, AppError> {
//     let User { name, email } = json.0;
//     let saved = state.db.create_user(name, email).await?;
//     Ok(Json(saved))
// }
```

---

## Phase 7: Middleware

### 7.1 Function Middleware

**from_fn:**
```rust
pub fn from_fn<F>(f: F) -> FromFn<F, ()>
    where F: /* complex bounds */;

pub fn from_fn_with_state<F, S>(f: F, state: S) -> FromFn<F, S>
    where F: /* complex bounds */;

elicit_newtype!(pub struct FromFn<F, S>(axum::middleware::FromFn<F, S>));
elicit_newtype!(pub struct FromFnLayer<F, S>(axum::middleware::FromFnLayer<F, S>));
```

**Next:**
```rust
elicit_newtype!(pub struct Next(axum::middleware::Next));

#[reflect_methods]
impl Next {
    pub async fn run(self, req: Request) -> Response;
}
```

---

### 7.2 Extractor Middleware

**from_extractor:**
```rust
pub fn from_extractor<E>() -> FromExtractor<E, ()>;
pub fn from_extractor_with_state<E, S>(state: S) -> FromExtractor<E, S>;

elicit_newtype!(pub struct FromExtractor<E, S>(axum::middleware::FromExtractor<E, S>));
elicit_newtype!(pub struct FromExtractorLayer<E, S>(axum::middleware::FromExtractorLayer<E, S>));
```

---

### 7.3 Map Request/Response

**map_request:**
```rust
pub fn map_request<F>(f: F) -> MapRequest<F, ()>;
pub fn map_request_with_state<F, S>(f: F, state: S) -> MapRequest<F, S>;

elicit_newtype!(pub struct MapRequest<F, S>(axum::middleware::MapRequest<F, S>));
elicit_newtype!(pub struct MapRequestLayer<F, S>(axum::middleware::MapRequestLayer<F, S>));
```

**map_response:**
```rust
pub fn map_response<F>(f: F) -> MapResponse<F, ()>;
pub fn map_response_with_state<F, S>(f: F, state: S) -> MapResponse<F, S>;

elicit_newtype!(pub struct MapResponse<F, S>(axum::middleware::MapResponse<F, S>));
elicit_newtype!(pub struct MapResponseLayer<F, S>(axum::middleware::MapResponseLayer<F, S>));
```

---

### 7.4 Middleware Code Generation

**MiddlewareBuilder:**
```rust
pub struct MiddlewareBuilder<S> {
    kind: MiddlewareKind,
    state_type: PhantomData<S>,
}

pub enum MiddlewareKind {
    FromFn {
        extractors: Vec<ExtractorSpec>,
        before: String,    // Code before next.run()
        after: String,     // Code after next.run()
    },
    FromExtractor {
        extractor: ExtractorSpec,
    },
    MapRequest {
        transform: String, // Code to transform request
    },
    MapResponse {
        transform: String, // Code to transform response
    },
}

impl<S> MiddlewareBuilder<S> {
    pub fn from_fn() -> MiddlewareBuilder<S>;
    pub fn from_extractor(extractor: ExtractorSpec) -> MiddlewareBuilder<S>;
    pub fn map_request() -> MiddlewareBuilder<S>;
    pub fn map_response() -> MiddlewareBuilder<S>;

    pub fn before(self, code: &str) -> Self;
    pub fn after(self, code: &str) -> Self;
    pub fn transform(self, code: &str) -> Self;

    pub fn generate_middleware_code(&self) -> String;
}
```

---

## Phase 8: Error Handling

### 8.1 HandleError Layer

**HandleError:**
```rust
elicit_newtype!(pub struct HandleError<S, F, T>(axum::error_handling::HandleError<S, F, T>));
elicit_newtype!(pub struct HandleErrorLayer<F, T>(axum::error_handling::HandleErrorLayer<F, T>));

#[reflect_methods]
impl<F, T> HandleErrorLayer<F, T> {
    pub fn new(f: F) -> HandleErrorLayer<F, T>;
}
```

---

## Phase 9: Server & Serving

### 9.1 Serve Function

**serve:**
```rust
pub async fn serve<M, S>(
    listener: TcpListener,
    make_service: M,
) -> Result<(), std::io::Error>
where
    M: for<'a> Service<IncomingStream<'a, TcpListener>, Response = S>,
    S: Service<Request, Response = Response>;
```

**Serve Types:**
```rust
elicit_newtype!(pub struct Serve<L, M, S>(axum::serve::Serve<L, M, S>));

#[reflect_methods]
impl<L, M, S> Serve<L, M, S> {
    pub fn with_graceful_shutdown<F>(self, signal: F) -> WithGracefulShutdown<L, M, S, F>
        where F: Future<Output = ()> + Send + 'static;

    pub fn local_addr(&self) -> io::Result<<L as Listener>::Addr>
        where L: Listener;
}

elicit_newtype!(pub struct WithGracefulShutdown<L, M, S, F>(
    axum::serve::WithGracefulShutdown<L, M, S, F>
));

#[reflect_methods]
impl<L, M, S, F> WithGracefulShutdown<L, M, S, F> {
    pub fn local_addr(&self) -> io::Result<<L as Listener>::Addr>
        where L: Listener;
}
```

---

### 9.2 Listener Trait

**Listener:**
```rust
#[reflect_trait(Listener)]
pub trait Listener: Sized {
    type Io: AsyncRead + AsyncWrite + Unpin + Send + 'static;
    type Addr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr);
    fn local_addr(&self) -> io::Result<Self::Addr>;
}
```

**Extension Trait:**
```rust
#[reflect_trait(ListenerExt)]
pub trait ListenerExt: Listener {
    fn tap_io<F>(self, tap: F) -> TapIo<Self, F>
        where F: FnMut(&mut Self::Io) + Send + 'static;
}

elicit_newtype!(pub struct TapIo<L, F>(axum::serve::TapIo<L, F>));
```

**IncomingStream:**
```rust
elicit_newtype!(pub struct IncomingStream<'a, L>(axum::serve::IncomingStream<'a, L>));

#[reflect_methods]
impl<'a, L: Listener> IncomingStream<'a, L> {
    pub fn io(&self) -> &L::Io;
    pub fn remote_addr(&self) -> &L::Addr;
}
```

---

## Phase 10: HTTP Re-exports & Utilities

### 10.1 HTTP Crate Types

**Status Codes:**
```rust
pub use http::StatusCode;

// Common status codes as constants
pub const STATUS_OK: StatusCode = StatusCode::OK;
pub const STATUS_CREATED: StatusCode = StatusCode::CREATED;
pub const STATUS_NO_CONTENT: StatusCode = StatusCode::NO_CONTENT;
pub const STATUS_BAD_REQUEST: StatusCode = StatusCode::BAD_REQUEST;
pub const STATUS_UNAUTHORIZED: StatusCode = StatusCode::UNAUTHORIZED;
pub const STATUS_FORBIDDEN: StatusCode = StatusCode::FORBIDDEN;
pub const STATUS_NOT_FOUND: StatusCode = StatusCode::NOT_FOUND;
pub const STATUS_INTERNAL_SERVER_ERROR: StatusCode = StatusCode::INTERNAL_SERVER_ERROR;
// ... (all standard status codes)
```

**Headers:**
```rust
pub use http::{HeaderMap, HeaderName, HeaderValue};

// Common header names
pub const HEADER_AUTHORIZATION: HeaderName = http::header::AUTHORIZATION;
pub const HEADER_CONTENT_TYPE: HeaderName = http::header::CONTENT_TYPE;
pub const HEADER_ACCEPT: HeaderName = http::header::ACCEPT;
pub const HEADER_USER_AGENT: HeaderName = http::header::USER_AGENT;
// ... (all standard headers)
```

**Methods:**
```rust
pub use http::Method;

pub const METHOD_GET: Method = Method::GET;
pub const METHOD_POST: Method = Method::POST;
pub const METHOD_PUT: Method = Method::PUT;
pub const METHOD_DELETE: Method = Method::DELETE;
pub const METHOD_PATCH: Method = Method::PATCH;
pub const METHOD_HEAD: Method = Method::HEAD;
pub const METHOD_OPTIONS: Method = Method::OPTIONS;
```

**URI:**
```rust
pub use http::Uri;

elicit_newtype!(pub struct UriParts(http::uri::Parts));
elicit_newtype!(pub struct Scheme(http::uri::Scheme));
elicit_newtype!(pub struct Authority(http::uri::Authority));
elicit_newtype!(pub struct PathAndQuery(http::uri::PathAndQuery));
```

**Version:**
```rust
pub use http::Version;

pub const HTTP_09: Version = Version::HTTP_09;
pub const HTTP_10: Version = Version::HTTP_10;
pub const HTTP_11: Version = Version::HTTP_11;
pub const HTTP_2: Version = Version::HTTP_2;
pub const HTTP_3: Version = Version::HTTP_3;
```

---

## Workflow-Based Tool Design

The key insight for axum: web services are **workflows**, not individual function calls. Agents should compose workflows, not make isolated tool calls.

### Workflow 1: Create Simple Service

**Tools:**
1. `create_router()` → Router
2. `add_get_route(router, "/", handler_id)` → Router
3. `serve_router(router, addr)` → running server

**Agent Flow:**
```text
1. create_router() → router_1
2. register_handler("root_handler", code) → handler_1
3. add_get_route(router_1, "/", handler_1) → router_2
4. serve_router(router_2, "127.0.0.1:3000")
```

### Workflow 2: Add JSON API Endpoint

**Tools:**
1. `create_json_handler(extractors, body_code)` → handler_id
2. `add_post_route(router, "/api/users", handler_id)` → Router
3. `add_middleware(router, auth_middleware)` → Router

**Agent Flow:**
```text
1. create_json_handler(["State", "Json"], "create_user_logic") → handler_2
2. create_auth_middleware(auth_logic) → middleware_1
3. add_post_route(router, "/api/users", handler_2) → router_3
4. apply_middleware(router_3, middleware_1) → router_4
```

### Workflow 3: Nest Sub-Router

**Tools:**
1. `create_router()` → sub_router
2. `nest_router(main_router, "/api", sub_router)` → Router

**Agent Flow:**
```text
1. create_router() → api_router
2. add_get_route(api_router, "/users", list_users) → api_router_2
3. add_post_route(api_router_2, "/users", create_user) → api_router_3
4. nest_router(main_router, "/api", api_router_3) → main_router_2
```

---

## Verification Strategy

### Per-Tool Contracts

**Router Construction:**
```rust
#[cfg_attr(feature = "verify-kani", kani::proof)]
fn verify_router_route_adds_path() {
    let router = Router::new();
    let method_router = get(|| async { "OK" });
    let router = router.route("/test", method_router);

    // Kani: prove route is registered
    #[cfg(feature = "verify-kani")]
    kani::assert(router_has_route(&router, "/test"));
}
```

**Extractor Composition:**
```rust
#[cfg_attr(feature = "verify-creusot", creusot::proof)]
fn prove_path_extraction_preserves_value() {
    let path = Path::new(42u32);
    let value = path.into_inner();

    // Creusot: prove value identity
    #[cfg(feature = "verify-creusot")]
    creusot::proof_assert!(value == 42);
}
```

**Handler Registration:**
```rust
#[cfg_attr(feature = "verify-kani", kani::proof)]
fn verify_handler_with_state_preserves_state() {
    let state = AppState::new();
    let handler = root_handler.with_state(state.clone());

    // Kani: prove state is accessible in handler
    #[cfg(feature = "verify-kani")]
    kani::assert(handler_has_state(&handler, &state));
}
```

---

## Implementation Strategy

### Step 1: Crate Scaffolding (3 crates)

**elicit_tower:**
```bash
cargo new --lib crates/elicit_tower
```

**Cargo.toml:**
```toml
[package]
name = "elicit_tower"
version = "0.1.0"
edition = "2021"

[dependencies]
tower = "0.5"
tower-http = "0.6"
elicitation = { path = "../elicitation" }
serde = { version = "1", features = ["derive"] }
schemars = "0.8"
http = "1.0"
```

**elicit_axum_core:**
```bash
cargo new --lib crates/elicit_axum_core
```

**Cargo.toml:**
```toml
[package]
name = "elicit_axum_core"
version = "0.1.0"
edition = "2021"

[dependencies]
axum-core = "0.5"
elicitation = { path = "../elicitation" }
http = "1.0"
serde = { version = "1", features = ["derive"] }
schemars = "0.8"
```

**elicit_axum:**
```bash
cargo new --lib crates/elicit_axum
```

**Cargo.toml:**
```toml
[package]
name = "elicit_axum"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = { version = "0.8", features = ["full"] }
elicit_axum_core = { path = "../elicit_axum_core" }
elicit_tower = { path = "../elicit_tower" }
elicitation = { path = "../elicitation" }
serde = { version = "1", features = ["derive"] }
schemars = "0.8"
tokio = { version = "1", features = ["full"] }
tower = "0.5"
http = "1.0"

[features]
default = ["json", "form", "query", "matched-path", "original-uri"]
json = ["axum/json"]
form = ["axum/form"]
query = ["axum/query"]
multipart = ["axum/multipart"]
ws = ["axum/ws"]
macros = ["axum/macros"]
http1 = ["axum/http1"]
http2 = ["axum/http2"]
matched-path = ["axum/matched-path"]
original-uri = ["axum/original-uri"]
tokio-runtime = ["axum/tokio"]
tower-log = ["axum/tower-log"]
tracing-support = ["axum/tracing"]
```

---

### Step 2: Module Structure

**elicit_tower/src/:**
```text
├── lib.rs              # Exports
├── service.rs          # Service trait + wrappers
├── layer.rs            # Layer trait + wrappers
├── middleware/
│   ├── mod.rs
│   ├── compression.rs  # CompressionLayer
│   ├── cors.rs         # CorsLayer
│   ├── trace.rs        # TraceLayer
│   ├── timeout.rs      # TimeoutLayer
│   ├── request_id.rs   # Request ID layers
│   ├── set_header.rs   # Header manipulation
│   ├── validate.rs     # Request validation
│   └── ... (20+ middleware types)
└── plugin.rs           # Plugin registration
```

**elicit_axum_core/src/:**
```text
├── lib.rs              # Exports
├── extract.rs          # FromRequest traits
├── response.rs         # IntoResponse traits
├── body.rs             # Body types
├── ext_traits.rs       # Extension traits
└── plugin.rs           # Plugin registration
```

**elicit_axum/src/:**
```text
├── lib.rs              # Exports
├── routing/
│   ├── mod.rs          # Router, MethodRouter
│   ├── method_filter.rs
│   └── service.rs      # Service conversions
├── extract/
│   ├── mod.rs
│   ├── state.rs
│   ├── path.rs
│   ├── query.rs
│   ├── json.rs
│   ├── form.rs
│   ├── multipart.rs
│   ├── ws.rs
│   └── rejection.rs    # All rejection types
├── response/
│   ├── mod.rs
│   ├── json.rs
│   ├── html.rs
│   ├── redirect.rs
│   ├── sse.rs
│   └── append_headers.rs
├── handler/
│   ├── mod.rs          # Handler trait
│   ├── builder.rs      # HandlerBuilder code gen
│   └── service.rs      # HandlerService
├── middleware/
│   ├── mod.rs
│   ├── from_fn.rs
│   ├── from_extractor.rs
│   ├── map_request.rs
│   ├── map_response.rs
│   └── builder.rs      # MiddlewareBuilder code gen
├── error_handling.rs   # HandleError layer
├── serve.rs            # Server utilities
├── body.rs             # Body utilities
├── http.rs             # HTTP re-exports
└── plugin.rs           # Plugin registration
```

---

### Step 3: Testing Strategy

**Integration Tests:**
```rust
// tests/router_test.rs
use elicit_axum::{Router, get};

#[tokio::test]
async fn test_router_route() {
    let router = Router::new();
    let router = router.route("/", get(|| async { "Hello" }));

    // Test that route is registered
    let response = router_test_request(router, "/").await;
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.body_string().await, "Hello");
}

#[tokio::test]
async fn test_router_nest() {
    let api = Router::new()
        .route("/users", get(|| async { "Users" }));

    let app = Router::new()
        .nest("/api", api);

    let response = router_test_request(app, "/api/users").await;
    assert_eq!(response.body_string().await, "Users");
}
```

**Extractor Tests:**
```rust
// tests/extract_test.rs
use elicit_axum::extract::{Path, Query, Json};

#[tokio::test]
async fn test_path_extraction() {
    let path = Path::new(42u32);
    assert_eq!(path.into_inner(), 42);
}

#[tokio::test]
async fn test_json_extraction() {
    #[derive(Serialize, Deserialize)]
    struct User { name: String }

    let json = Json::new(User { name: "Alice".into() });
    assert_eq!(json.into_inner().name, "Alice");
}
```

---

## Completion Checklist

### Phase 1: Tower Foundation (Week 1)
- [ ] `elicit_tower/service.rs` - Service trait + wrappers
- [ ] `elicit_tower/layer.rs` - Layer trait + wrappers
- [ ] `elicit_tower/middleware/` - 20+ tower-http layers
- [ ] Tests for all middleware
- [ ] Plugin registration
- [ ] Kani proofs for service composition

### Phase 2: Axum-Core (Week 1)
- [ ] `elicit_axum_core/extract.rs` - FromRequest traits
- [ ] `elicit_axum_core/response.rs` - IntoResponse traits
- [ ] `elicit_axum_core/body.rs` - Body types
- [ ] `elicit_axum_core/ext_traits.rs` - Extension traits
- [ ] Tests for trait reflection
- [ ] Plugin registration

### Phase 3: Router & Routing (Week 2)
- [ ] `routing/mod.rs` - Router, MethodRouter
- [ ] `routing/method_filter.rs` - MethodFilter bitflags
- [ ] `routing/service.rs` - Service conversions
- [ ] Top-level routing functions (get, post, etc.)
- [ ] Tests for router construction
- [ ] Tests for nesting, merging

### Phase 4: Extractors (Week 3)
- [ ] All extractor types (20+)
- [ ] All rejection types
- [ ] Path, Query, Json, Form, Multipart
- [ ] WebSocket upgrade
- [ ] ConnectInfo, MatchedPath, OriginalUri
- [ ] Tests for each extractor
- [ ] Creusot proofs for extraction

### Phase 5: Response Types (Week 3)
- [ ] Json, Html, Form responses
- [ ] Redirect types
- [ ] SSE stream and events
- [ ] AppendHeaders
- [ ] Response builder functions
- [ ] Tests for all response types

### Phase 6: Handler & Services (Week 4)
- [ ] Handler trait reflection
- [ ] HandlerService types
- [ ] HandlerBuilder code generation
- [ ] Extension traits
- [ ] Tests for handler registration
- [ ] Code generation tests

### Phase 7: Middleware (Week 4)
- [ ] from_fn, from_extractor
- [ ] map_request, map_response
- [ ] Next type
- [ ] MiddlewareBuilder code generation
- [ ] Tests for all middleware types
- [ ] Middleware chain tests

### Phase 8: Error Handling (Week 5)
- [ ] HandleError layer
- [ ] Error handler builder
- [ ] Tests for error handling

### Phase 9: Server & Serving (Week 5)
- [ ] serve() function
- [ ] Listener trait
- [ ] Graceful shutdown
- [ ] IncomingStream
- [ ] Tests for server startup

### Phase 10: HTTP Re-exports (Week 5)
- [ ] Status codes
- [ ] Headers
- [ ] Methods
- [ ] URI types
- [ ] Version constants

### Phase 11: Workflow Tools (Week 6)
- [ ] Workflow builders
- [ ] Complete service templates
- [ ] Documentation for workflows
- [ ] Integration examples

### Final Integration (Week 6)
- [ ] Complete plugin registry
- [ ] End-to-end web service test
- [ ] Performance benchmarks
- [ ] Documentation review
- [ ] README with examples
- [ ] CHANGELOG
- [ ] Release preparation

---

## Success Metrics

1. **Completeness:** 100% of axum 0.8.8 public API exposed
2. **Workflow Coverage:** All common web service patterns supported
3. **Type Safety:** All extractors/responses type-checked via serde/JsonSchema
4. **Code Generation:** Agents can generate valid Rust handlers and middleware
5. **Verification:** Kani/Creusot proofs for composition contracts
6. **Documentation:** Every tool + 10+ workflow examples
7. **Testing:** >85% code coverage, all workflows tested

---

## Notes for Implementation

1. **Generic Parameters:** Heavy use of type parameters (S for state, E for error)
2. **Trait Bounds:** Complex trait bounds require careful wrapper design
3. **Lifetime Management:** Extractors have lifetime constraints - use `'static` where possible
4. **Code Generation:** Handlers/middleware need Rust code emission, not just runtime values
5. **Tower Integration:** Deep integration with Tower ecosystem requires coordination
6. **Testing:** Use `tower::ServiceBuilder` for integration tests

---

**Total Estimated Time:** 6 weeks for complete implementation
**Estimated Tool Count:** 400-500 MCP tools (router + extractors + responses + Tower middleware)
**Estimated LOC:** 20,000-25,000 lines (including tests and docs)
