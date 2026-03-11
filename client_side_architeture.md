# Client-Side Tooling Opportunities for the Elicitation Ecosystem

This document outlines promising Rust ecosystem crates and architectural ideas that complement the **elicitation** workflow/typestate agent framework.

The focus is **client-side tooling**, particularly components that enhance agent reasoning, observability, and backend flexibility without duplicating elicitation’s core typestate logic.

---

# Architectural Context

Elicitation provides:

- A **typestate-driven workflow system**
- Deterministic **state machine orchestration**
- Verified transitions for agent/user interaction
- A runtime hosted in `elicit_server`
- MCP compatibility via `rmcp`
- Deep integration with `tracing`

In this architecture:

```
LLM = probabilistic reasoning oracle
elicitation = deterministic runtime
```

The goal of client-side tooling is therefore:

- provider abstraction
- reasoning support
- observability
- context augmentation
- developer tooling

without replacing the elicitation runtime itself.

---

# 1. Multi-Provider LLM Backend

## `llm` crate

Purpose:

Provide a **backend-agnostic abstraction layer** over multiple LLM providers.

Capabilities:

- unified chat/completion API
- tool/function calling
- structured outputs
- streaming
- multi-provider evaluation

Architectural role:

```
elicit_server
    ↓
LLM adapter layer
    ↓
llm crate
    ↓
provider APIs
```

Benefits:

- prevents provider lock-in
- simplifies experimentation with models
- centralizes provider logic

Status: **strong candidate for elicitation client integration**

---

# 2. Structured Output / Typed LLM Responses

## `rstructor`

Concept:

Typed LLM responses derived from Rust structs.

Example:

```rust
#[derive(FromLLM)]
struct TransitionProposal {
    next_state: String,
    arguments: Vec<String>,
}
```

Purpose:

Convert raw LLM output into validated Rust types.

Relevance to elicitation:

Potential overlap with typestate-driven validation logic.

This crate represents **prior art worth examining**, but elicitation's compile-time typestate guarantees may already supersede its core functionality.

Key question:

```
Does elicitation already solve this problem more rigorously?
```

---

# 3. Tokenization and Prompt Budgeting

## `tiktoken-rs`

Purpose:

Token counting and prompt budgeting.

Capabilities:

- model-compatible tokenization
- token count estimation
- context window management

Example use cases:

```
1. Prompt truncation
2. Context budgeting
3. Model selection based on token limits
```

Possible integration:

```
state transition
    ↓
prompt construction
    ↓
token budget check
    ↓
LLM invocation
```

This prevents runtime failures due to token limits.

---

# 4. Semantic Retrieval / Local RAG

## Candidate crates

- `fastembed`
- `qdrant-client`
- `lancedb`
- `tantivy`

Purpose:

Provide **semantic context retrieval** for agents.

Potential uses in elicitation:

### Semantic State Lookup

```
workflow state history
        ↓
embedding index
        ↓
retrieve similar states
        ↓
LLM reasoning context
```

### Tool Discovery

```
tool descriptions
        ↓
embedding index
        ↓
semantic search
        ↓
select tool candidates
```

### Knowledge Augmentation

```
external documents
        ↓
vector store
        ↓
retrieval
        ↓
LLM prompt context
```

This enables **local RAG without external services**.

---

# 5. Research and Web Intelligence

## `exa_api_client`

Purpose:

Access Exa's semantic search API.

Capabilities:

- AI-native search
- semantic query support
- structured search results

Agent workflow:

```
agent reasoning
      ↓
semantic web search
      ↓
structured results
      ↓
LLM analysis
```

This is particularly useful for **research agents** and **knowledge acquisition workflows**.

---

# 6. Codebase Ingestion for Agent Analysis

## `cargo-onefile`

Purpose:

Flatten a Rust project into a single file.

Original use case:

Feeding codebases into LLMs.

Example workflow:

```
cargo onefile
     ↓
single source artifact
     ↓
LLM reasoning
```

Potential agent capabilities:

- code review
- architecture analysis
- automated documentation
- refactoring suggestions

This enables **agent introspection over Rust projects**.

---

# 7. MCP Integration

Elicitation already builds on:

```
rmcp
```

Implication:

Elicitation workflows can naturally integrate with the **Model Context Protocol ecosystem**.

This provides:

```
agent runtime
     ↓
MCP client
     ↓
external tools
```

Tradeoff:

Some **vendor lock-in to the MCP tool protocol**, but the ecosystem benefits likely outweigh this constraint.

---

# 8. Workflow Visualization

This is a particularly promising area.

Candidate crates:

- `petgraph`
- `daggy`
- `graphviz`

Purpose:

Render **typestate workflows as graphs**.

Example visualization:

```
State A
   ├── transition_1 → State B
   ├── transition_2 → State C
   └── transition_3 → State D
```

Applications:

### Agent Debugging

```
workflow graph
      ↓
highlight current state
      ↓
display allowed transitions
```

### Runtime Introspection

```
live agent execution
      ↓
visualize state transitions
```

### Development Tools

```
compile workflow
      ↓
generate graph visualization
```

This could become a **core developer experience feature**.

---

# Potential Developer Tool

A CLI command like:

```
cargo elicitation graph
```

could output:

```
workflow.dot
```

Which renders as:

```
workflow.png
workflow.svg
```

This would allow developers to **visually inspect agent workflows**.

---

# Emerging Stack for Elicitation Clients

A potential ecosystem stack could look like:

```
elicitation
    │
    ├── llm                (multi-provider LLM backend)
    ├── tiktoken-rs        (prompt budgeting)
    ├── fastembed          (embedding generation)
    ├── qdrant-client      (vector storage)
    ├── exa_api_client     (semantic web search)
    ├── cargo-onefile      (codebase ingestion)
    └── petgraph/graphviz  (workflow visualization)
```

This provides:

- backend flexibility
- context retrieval
- external knowledge acquisition
- developer observability

while leaving **elicitation’s typestate runtime intact**.

---

# Key Insight

The most important architectural principle remains:

```
LLM = advisor
elicitation = runtime
```

Client-side tooling should therefore:

- enhance reasoning
- improve context
- increase observability

but **never replace the deterministic workflow engine**.

---

# Future Opportunity

One particularly powerful direction:

Automatically generating:

```
typestate workflow
       ↓
graph visualization
       ↓
LLM tool schemas
       ↓
MCP tool exports
```

This would allow elicitation workflows to become **self-describing, introspectable agent systems**.

```
Rust typestate → Agent runtime → MCP tools → Visualized workflows
```

A compelling ecosystem direction.

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
