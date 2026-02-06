# Unified Elicitation Trait: Client + Server Support

## Goal

Make the `Elicitation` trait work with **both** `ElicitClient` and `ElicitServer` so all existing elicitation implementations work in both contexts.

## Current State

### What Works ✅
- `ElicitClient` wraps `Peer<RoleClient>` (client-side)
- `ElicitServer` wraps `Peer<RoleServer>` (server-side)
- Both have identical API surface (with_style, style_or_default, etc.)
- `elicit_checked()` creates `ElicitServer` wrapper

### What's Blocked ⏸️
- `Elicitation` trait only accepts `&ElicitClient`
- Can't call `Type::elicit(&server)` yet
- All 100+ trait implementations need updating

## Design Options

### Option A: Make Trait Generic Over Context

```rust
pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    
    fn elicit<C: ElicitContext>(
        context: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}

// Trait that both wrappers implement
pub trait ElicitContext {
    fn peer(&self) -> &dyn PeerLike;  // Or similar abstraction
    fn with_style<T, S>(&self, style: S) -> Self;
    // ... other methods
}

impl ElicitContext for ElicitClient { /* ... */ }
impl ElicitContext for ElicitServer { /* ... */ }
```

**Pros:**
- Single trait, works everywhere
- Type-safe context abstraction

**Cons:**
- Big refactor (100+ implementations)
- Complex trait bounds
- PeerLike abstraction needed

### Option B: Dual Traits (Separate Client/Server)

```rust
pub trait ElicitFromClient: Sized + Prompt + 'static {
    type Style: ElicitFromClient + Default + Clone + Send + Sync + 'static;
    fn elicit(client: &ElicitClient) -> impl Future<Output = ElicitResult<Self>> + Send;
}

pub trait ElicitFromServer: Sized + Prompt + 'static {
    type Style: ElicitFromServer + Default + Clone + Send + Sync + 'static;
    fn elicit(server: &ElicitServer) -> impl Future<Output = ElicitResult<Self>> + Send;
}

// Blanket impl: if you impl client-side, you get server-side for free
impl<T> ElicitFromServer for T
where
    T: ElicitFromClient,
{
    type Style = <T as ElicitFromClient>::Style;
    
    fn elicit(server: &ElicitServer) -> impl Future<Output = ElicitResult<Self>> + Send {
        // Bridge: convert server context to client-like interface
        Self::elicit(&server.as_client_like()).await
    }
}
```

**Pros:**
- Explicit about context
- Allows different implementations per context
- Blanket impl reduces duplication

**Cons:**
- Two traits to understand
- Still need bridging abstraction

### Option C: Enum Wrapper (Runtime Dispatch)

```rust
pub enum ElicitPeer {
    Client(ElicitClient),
    Server(ElicitServer),
}

impl ElicitPeer {
    pub fn peer(&self) -> PeerVariant {
        match self {
            Self::Client(c) => PeerVariant::Client(c.peer()),
            Self::Server(s) => PeerVariant::Server(s.peer()),
        }
    }
    // ... delegate all methods
}

// Trait stays simple:
pub trait Elicitation: Sized + Prompt + 'static {
    fn elicit(peer: &ElicitPeer) -> impl Future<Output = ElicitResult<Self>> + Send;
}
```

**Pros:**
- Simple trait signature
- Easy migration (just wrap in enum)
- Runtime flexibility

**Cons:**
- Runtime overhead (match on every call)
- Type erasure (lose compile-time guarantees)
- Boxing might be needed

### Option D: Communication Trait Abstraction (Recommended)

```rust
/// Abstraction for elicitation communication (client or server)
pub trait ElicitCommunicator: Clone {
    /// Send a message and get response
    async fn communicate(&self, prompt: &str) -> ElicitResult<String>;
    
    /// Get style context
    fn style_context(&self) -> &StyleContext;
}

impl ElicitCommunicator for ElicitClient {
    async fn communicate(&self, prompt: &str) -> ElicitResult<String> {
        // Use client peer to send tool call
        // ...
    }
    
    fn style_context(&self) -> &StyleContext {
        &self.style_context
    }
}

impl ElicitCommunicator for ElicitServer {
    async fn communicate(&self, prompt: &str) -> ElicitResult<String> {
        // Use server peer to call create_message()
        // ...
    }
    
    fn style_context(&self) -> &StyleContext {
        &self.style_context
    }
}

// Trait becomes:
pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    
    fn elicit<C: ElicitCommunicator>(
        context: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}
```

**Pros:**
- Clean abstraction (just communication + style)
- Implementations don't care about client vs server
- Easy to test (mock communicator)
- Minimal trait bounds

**Cons:**
- All implementations need update (but straightforward)
- Need to identify communication points

## Recommended Approach: Option D

**Why?**
1. **Minimal abstraction**: Just `communicate()` and `style_context()`
2. **Clean semantics**: Implementations work with "communicator", not "client" or "server"
3. **Testable**: Easy to mock `ElicitCommunicator` for tests
4. **Future-proof**: Easy to add other contexts (file, env vars, etc.)

## Implementation Plan

### Phase 1: Create ElicitCommunicator Trait
```rust
// In src/communicator.rs
pub trait ElicitCommunicator: Clone + Send + Sync {
    /// Send a prompt and receive text response
    async fn send_prompt(&self, prompt: &str) -> ElicitResult<String>;
    
    /// Get style context for type-specific styles
    fn style_context(&self) -> &StyleContext;
    
    /// Create new communicator with style added
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self;
}
```

### Phase 2: Implement for Client and Server
```rust
impl ElicitCommunicator for ElicitClient {
    async fn send_prompt(&self, prompt: &str) -> ElicitResult<String> {
        // Implementation using self.peer() (Peer<RoleClient>)
        // Call MCP tool, get response
        todo!()
    }
    
    fn style_context(&self) -> &StyleContext {
        &self.style_context
    }
    
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self {
        // Existing implementation
        let mut ctx = self.style_context.clone();
        ctx.set_style::<T, S>(style);
        Self {
            peer: self.peer.clone(),
            style_context: ctx,
        }
    }
}

impl ElicitCommunicator for ElicitServer {
    async fn send_prompt(&self, prompt: &str) -> ElicitResult<String> {
        // Implementation using self.peer() (Peer<RoleServer>)
        // Call peer.create_message(), parse response
        let params = rmcp::model::CreateMessageRequestParams {
            messages: vec![rmcp::model::SamplingMessage {
                role: rmcp::model::Role::User,
                content: rmcp::model::Content::text(prompt),
            }],
            max_tokens: 1000,
            // ... other fields
        };
        
        let result = self.peer.create_message(params).await?;
        
        match result.message.content {
            rmcp::model::Content::Text { text } => Ok(text),
            _ => Err(/* InvalidFormat */),
        }
    }
    
    // ... same implementations as client
}
```

### Phase 3: Update Elicitation Trait
```rust
pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    
    fn elicit<C: ElicitCommunicator>(
        comm: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}
```

### Phase 4: Update Implementations (Incremental)

Start with simple types, migrate gradually:

```rust
// Before:
impl Elicitation for String {
    type Style = StringStyle;
    
    async fn elicit(client: &ElicitClient) -> ElicitResult<Self> {
        let prompt = "Enter a string:";
        // ... use client.peer() somehow
    }
}

// After:
impl Elicitation for String {
    type Style = StringStyle;
    
    async fn elicit<C: ElicitCommunicator>(comm: &C) -> ElicitResult<Self> {
        let prompt = "Enter a string:";
        let response = comm.send_prompt(prompt).await?;
        Ok(response)
    }
}
```

### Phase 5: Update elicit_checked()
```rust
#[elicitation::rmcp::tool]
pub async fn elicit_checked(
    peer: elicitation::rmcp::service::Peer<elicitation::rmcp::service::RoleServer>,
) -> Result<Self, elicitation::ElicitError> {
    let server = elicitation::ElicitServer::new(peer);
    Self::elicit(&server).await  // ✅ Now works!
}
```

## Migration Strategy

### Non-Breaking Approach

Keep both old and new traits temporarily:

```rust
// Old trait (deprecated)
#[deprecated(note = "Use Elicitation with ElicitCommunicator instead")]
pub trait ElicitationLegacy: Sized + Prompt + 'static {
    fn elicit_client(client: &ElicitClient) -> impl Future<Output = ElicitResult<Self>> + Send;
}

// New trait
pub trait Elicitation: Sized + Prompt + 'static {
    fn elicit<C: ElicitCommunicator>(comm: &C) -> impl Future<Output = ElicitResult<Self>> + Send;
}

// Blanket impl: old → new
impl<T> Elicitation for T
where
    T: ElicitationLegacy,
{
    fn elicit<C: ElicitCommunicator>(comm: &C) -> impl Future<Output = ElicitResult<Self>> + Send {
        // If it's a client, use legacy impl
        // Otherwise, error or provide default
        todo!()
    }
}
```

But honestly, **just breaking change it**. It's v0.6, and it's a clean improvement.

## Timeline

- **Phase 1** (Trait): 1 hour
- **Phase 2** (Impls): 2 hours
- **Phase 3** (Trait update): 30 min
- **Phase 4** (Migrate primitives): 2-3 hours
- **Phase 5** (elicit_checked): 15 min

**Total**: ~1 day of focused work

## Testing Strategy

1. **Unit tests**: Mock `ElicitCommunicator` with fixed responses
2. **Integration tests**: Test with both ElicitClient and ElicitServer
3. **Regression tests**: Ensure all existing tests still pass

## Success Criteria

- [ ] `ElicitCommunicator` trait defined
- [ ] Both wrappers implement it
- [ ] Trait signature updated
- [ ] At least 10 primitives migrated
- [ ] Tests pass for both contexts
- [ ] `elicit_checked()` calls `Self::elicit(&server)`

## Conclusion

**Option D (ElicitCommunicator)** is the cleanest path forward. It's a straightforward abstraction that makes the code more testable and flexible while solving the client/server duality.

The migration is mechanical: replace direct peer access with `comm.send_prompt()`. Start with primitives (String, i32, bool), validate the pattern, then scale to all types.

This unifies client and server elicitation under one trait, achieving the "everybody wins" goal.
