# Elicitation Client-Side Plugin Architecture

This document describes a proposed **plugin architecture for the elicitation client library**, designed to allow users to selectively enable advanced capabilities while keeping the core client lightweight and composable.

The guiding principle is:

```
elicitation runtime = deterministic workflow engine
client plugins = optional capability layers
```

Plugins should **extend the client**, not replace the runtime or introduce competing agent frameworks.

---

# Design Goals

The client-side architecture should:

1. **Remain minimal by default**
2. **Allow capability opt-in via Cargo features**
3. **Avoid unnecessary dependencies**
4. **Expose extension points for third-party plugins**
5. **Preserve elicitation’s deterministic runtime model**

This allows users to build highly customized agent environments without pulling in large dependency trees.

---

# Core Architecture

The client sits between the application and the `elicit_server` runtime.

```
application
     │
     ▼
elicit_client
     │
     ▼
rmcp transport
     │
     ▼
elicit_server
```

The client itself contains minimal functionality:

```
elicit_client_core
 ├── transport
 ├── session management
 ├── tracing integration
 └── plugin host
```

Plugins add additional capabilities to the client runtime.

---

# Plugin Model

Plugins extend the client through a simple trait interface.

Example:

```rust
pub trait ClientPlugin {
    fn name(&self) -> &'static str;

    fn initialize(&self, ctx: &mut ClientContext);
}
```

The `ClientContext` exposes extension points.

```rust
pub struct ClientContext {
    pub middleware: MiddlewareStack,
    pub tools: ToolRegistry,
    pub observability: ObservabilityHooks,
}
```

Plugins register their behavior during initialization.

---

# Middleware Pipeline

The client processes requests through a middleware pipeline.

```
prompt construction
       │
       ▼
middleware hooks
       │
       ▼
LLM invocation
       │
       ▼
middleware hooks
       │
       ▼
response handling
```

Plugins can insert logic into the pipeline without controlling execution flow.

Example middleware responsibilities:

- prompt augmentation
- token budgeting
- context injection
- response scoring

---

# Plugin Categories

Plugins typically fall into three categories.

---

## Capability Plugins

Add new reasoning or context capabilities.

Examples:

- semantic retrieval
- web search
- codebase ingestion

Example workflow:

```
client request
      │
      ▼
embedding lookup
      │
      ▼
context injection
      │
      ▼
LLM reasoning
```

---

## Runtime Plugins

Modify how the client interacts with models.

Examples:

- token budgeting
- multi-model routing
- response evaluation

Example:

```
prompt
  │
  ▼
token budget check
  │
  ▼
model selection
```

---

## Developer Tooling Plugins

Improve developer experience and observability.

Examples:

- workflow graph generation
- execution visualization
- trace export

These plugins assist debugging and introspection but do not affect agent reasoning.

---

# Example Plugins

## LLM Backend Plugin

Wraps a multi-provider backend (such as the `llm` crate).

Responsibilities:

- provider abstraction
- model selection
- tool invocation

Architecture:

```
client
  │
  ▼
llm backend plugin
  │
  ▼
provider APIs
```

---

## Token Budget Plugin

Uses tokenization libraries such as `tiktoken-rs`.

Responsibilities:

- token estimation
- prompt truncation
- context budgeting

Example:

```
prompt construction
       │
       ▼
token estimate
       │
       ▼
truncate if necessary
```

---

## Semantic Retrieval Plugin

Uses embedding and vector database libraries.

Candidate stack:

```
fastembed
qdrant-client
```

Capabilities:

- embed conversation context
- store embeddings
- retrieve similar states

Example workflow:

```
agent request
      │
      ▼
embedding lookup
      │
      ▼
retrieve relevant context
      │
      ▼
inject into prompt
```

---

## Web Research Plugin

Uses semantic search APIs such as Exa.

Capabilities:

- semantic web search
- knowledge retrieval
- context augmentation

Example flow:

```
agent question
      │
      ▼
semantic search
      │
      ▼
top results
      │
      ▼
LLM analysis
```

---

## Code Context Plugin

Uses tools such as `cargo-onefile` to flatten Rust projects.

Capabilities:

- codebase ingestion
- repository analysis
- documentation generation

Example:

```
cargo onefile
     │
     ▼
single-file artifact
     │
     ▼
LLM reasoning
```

---

## Workflow Visualization Plugin

Generates visual representations of typestate workflows.

Candidate crates:

```
petgraph
graphviz
```

Capabilities:

- render state machines
- visualize transitions
- display runtime state

Example output:

```
StateA
  ├─ transition_1 → StateB
  └─ transition_2 → StateC
```

Possible CLI command:

```
cargo elicitation graph
```

Output:

```
workflow.svg
```

---

# Cargo Feature Integration

Plugins are enabled via Cargo features.

Example:

```toml
[features]
llm-backend = []
token-budget = []
semantic-retrieval = []
web-search = []
code-context = []
workflow-graph = []
```

Users opt in to functionality as needed.

Example installation:

```
cargo add elicit_client --features semantic-retrieval
```

This ensures minimal dependency overhead.

---

# Recommended Crate Layout

Instead of a single monolithic client crate, the ecosystem can be split into modular components.

```
elicit_client_core
elicit_client_llm
elicit_client_tokens
elicit_client_embeddings
elicit_client_exa
elicit_client_code
elicit_client_graph
```

Each crate implements the `ClientPlugin` trait.

Benefits:

- modular compilation
- independent evolution
- reduced dependency surface

---

# MCP Integration

Since elicitation is built on `rmcp`, plugins can optionally expose new **MCP tools**.

Example:

```
workflow_graph_plugin
```

Could expose:

```
generate_workflow_graph()
```

This allows agents to dynamically request visualization or diagnostics.

---

# Key Architectural Principle

Client plugins enhance agent capabilities but do **not control the runtime**.

```
LLM = reasoning advisor
elicitation = deterministic state machine
plugins = capability extensions
```

Maintaining this separation preserves the safety and predictability of elicitation workflows.

---

# Future Direction

An interesting long-term opportunity is **automatic artifact generation** from typestate workflows.

Possible outputs:

```
typestate definitions
      │
      ├─ workflow graphs
      ├─ MCP tools
      ├─ LLM tool schemas
      └─ developer documentation
```

This would allow elicitation workflows to become **self-describing agent systems**.

---

# Summary

The client plugin architecture enables:

- flexible capability composition
- minimal core client complexity
- ecosystem extensibility
- optional dependency inclusion
- powerful developer tooling

This approach allows the elicitation ecosystem to grow organically while maintaining a clean and deterministic core runtime.
