# Style Associated Type System - Quick Reference

## ONE-LINER
> Every type has a `type Style: Elicitation + Default` that controls HOW prompts are presented, enabling seamless switching between human TUI, CLI, and AI agent contexts.

---

## 5-MINUTE OVERVIEW

### The Problem
Same struct needs different presentations:
```
Database Config
├─ Human TUI: "Enter server name: "
├─ AI Agent: JSON schema for tool params
└─ Compact CLI: "server: "
```

### The Solution: Style Enum
```rust
#[derive(Elicit)]
struct Config {
    #[prompt("Enter server:", style = "compact")]
    host: String,
}

// Generates:
pub enum ConfigElicitStyle {
    Default,      // DefaultStyle formatting
    Compact,      // CompactStyle formatting
}
```

### Usage
```rust
let client = ElicitClient::new(peer)
    .with_style::<Config, _>(ConfigElicitStyle::Compact);
let config = Config::elicit(&client).await?;
```

---

## TYPE SIGNATURES

### Core Trait
```rust
pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    async fn elicit<C: ElicitCommunicator>(comm: &C) 
        -> impl Future<Output = ElicitResult<Self>> + Send;
}
```

### Style Trait
```rust
pub trait ElicitationStyle: Clone + Send + Sync + Default + 'static {
    fn prompt_for_field(&self, field_name: &str, field_type: &str, 
                        context: &PromptContext) -> String;
    fn help_text(&self, field_name: &str, field_type: &str) 
        -> Option<String>;
    fn validation_error(&self, field_name: &str, error: &str) -> String;
    fn show_type_hints(&self) -> bool;
    fn select_style(&self) -> SelectStyle;
    fn use_decorations(&self) -> bool;
    fn prompt_prefix(&self) -> &str;
}
```

### Communicator Methods
```rust
pub trait ElicitCommunicator: Clone + Send + Sync {
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self;
    fn style_or_default<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>;
    async fn style_or_elicit<T: Elicitation + 'static>(&self) 
        -> ElicitResult<T::Style>;
}
```

---

## FILES & LOCATIONS

| Component | File | Lines |
|-----------|------|-------|
| Trait definition | traits.rs | 75-84 |
| Communicator trait | communicator.rs | 15-190 |
| Style trait | style.rs | 40-158 |
| Built-in styles | style.rs | 172-338 |
| Struct macro gen | struct_impl.rs | 900-1000+ |
| Enum macro gen | enum_impl.rs | 484-513 |
| Default style macro | default_style.rs | 8-34 |
| Client impl | client.rs | 40-222 |
| Server impl | server.rs | 33-141 |

---

## BUILT-IN STYLES

| Style | Output | Use Case |
|-------|--------|----------|
| `DefaultStyle` | `"Enter host (String):"` | Balanced, standard |
| `CompactStyle` | `"host:"` | Minimal, terse |
| `VerboseStyle` | `"Please enter host (type: String, field 1/3)"` | Detailed help |
| `WizardStyle` | `"➤ Step 1 of 3: Enter host (String)"` | Step-by-step progress |

---

## MACRO GENERATION

### Simple (No Styles)
```rust
#[derive(Elicit)]
struct Config { host: String }

// Generates:
pub enum ConfigStyle { #[default] Default }
impl Elicitation for ConfigStyle { type Style = ConfigStyle; /* always Default */ }
impl Elicitation for Config { type Style = ConfigStyle; /* elicit fields */ }
```

### Styled (Multiple Styles)
```rust
#[derive(Elicit)]
struct Config {
    #[prompt("Server:", style = "compact")]
    host: String,
}

// Generates:
pub enum ConfigElicitStyle { Default, Compact }
impl Elicitation for ConfigElicitStyle { /* select from options */ }
impl Elicitation for Config {
    async fn elicit(comm) {
        let style = comm.style_or_elicit::<Self>().await?;
        let prompt = match style {
            ConfigElicitStyle::Default => "Enter server:",
            ConfigElicitStyle::Compact => "Server:",
        };
        let host = comm.send_prompt(prompt).await?;
        Ok(Self { host })
    }
}
```

---

## FLOW: SomeType::elicit(&client).await

```
1. STYLE RESOLUTION
   style = client.style_or_default::<SomeType>()?
   OR
   style = client.style_or_elicit::<SomeType>().await?

2. QUESTION GENERATION
   Use Prompt::prompt() + chosen Style to build question text
   
3. MCP COMMUNICATION
   Send via communicator.send_prompt() or call_tool()
   
4. RESPONSE PARSING
   Parse response according to type's pattern:
   - Primitive: Parse directly
   - Select: Match label to variant
   - Survey: Elicit each field
   - Affirm: Parse yes/no
   
5. RECURSIVE FIELD ELICITATION
   For each field: <FieldType>::elicit(communicator).await?
   
6. CONSTRUCT AND RETURN
   Build final value with elicited fields
```

---

## STYLE CONTEXT (Internal Storage)

```rust
pub struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}
```

**Why this design?**
- ✅ Type-erased: Supports heterogeneous styles
- ✅ O(1) lookup: `TypeId` key is simple hash
- ✅ Cheap clone: `Arc` clone is instant
- ✅ Thread-safe: `RwLock` for concurrent access
- ✅ No wasted space: Unused styles aren't stored

---

## CONCRETE EXAMPLE: Blackjack PlayerAction

### Definition
```rust
#[derive(Elicit)]
enum PlayerAction {
    Hit,
    Stand,
    DoubleDown,
}
```

### Generated Code (Simplified)
```rust
pub enum PlayerActionStyle { #[default] Default }

impl Elicitation for PlayerAction {
    type Style = PlayerActionStyle;
    
    async fn elicit<C: ElicitCommunicator>(comm: &C) -> ElicitResult<Self> {
        let prompt = "Please select a PlayerAction:\n\nOptions:\n1. Hit\n2. Stand\n3. DoubleDown\n\nRespond with the number (1-3) or exact label:";
        let response = comm.send_prompt(prompt).await?;
        
        match parse_label(&response, vec!["Hit", "Stand", "DoubleDown"])? {
            "Hit" => Ok(Self::Hit),
            "Stand" => Ok(Self::Stand),
            "DoubleDown" => Ok(Self::DoubleDown),
            _ => Err(ElicitError::InvalidOption { ... }),
        }
    }
}

impl Select for PlayerAction {
    fn options() -> Vec<Self> {
        vec![Self::Hit, Self::Stand, Self::DoubleDown]
    }
    
    fn labels() -> Vec<String> {
        vec!["Hit".to_string(), "Stand".to_string(), "DoubleDown".to_string()]
    }
    
    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Hit" => Some(Self::Hit),
            "Stand" => Some(Self::Stand),
            "DoubleDown" => Some(Self::DoubleDown),
            _ => None,
        }
    }
}
```

### Usage
```rust
// Human TUI context
let action = PlayerAction::elicit(&client).await?;
// Presents: "Please select a PlayerAction: ..."

// AI agent context (via MCP tool)
#[elicit_tools(PlayerAction)]
#[tool_router]
impl GameServer {}
// Tool schema auto-generates from Select impl
```

---

## DESIGN PRINCIPLES

### 1. **Trait-Based Extensibility**
Users can implement custom styles without modifying library:
```rust
#[derive(Clone, Default)]
pub struct MyCompanyStyle;

impl ElicitationStyle for MyCompanyStyle {
    fn prompt_for_field(&self, name: &str, ty: &str, ctx: &PromptContext) -> String {
        format!("🔹 {} [{}]", name, ty)
    }
}

let client = client.with_style::<Config, _>(MyCompanyStyle);
```

### 2. **Recursive Elegance**
Style enums implement Elicitation, so users can select styles:
```rust
pub enum ConfigElicitStyle { Default, Compact }

impl Elicitation for ConfigElicitStyle {
    type Style = ConfigElicitStyle;  // Self-reference
    async fn elicit<C: ElicitCommunicator>(comm: &C) -> ElicitResult<Self> {
        // Elicit style choice from user - same mechanism as any type!
    }
}
```

### 3. **Zero-Cost Abstraction**
- No overhead if you don't use styles (single Default variant)
- O(1) lookup in StyleContext
- Cheap cloning (Arc)
- No feature flags needed

### 4. **Context Agnosticism**
Same code works for:
- **Human CLI**: Text prompts
- **TUI**: Styled prompts with colors/emojis
- **AI Agent**: JSON schema for MCP tools
- **Anything**: Custom `ElicitCommunicator` impl

---

## FUTURE: TUI INTEGRATION

Not implemented yet, but architecture supports:

```rust
// Define TUI style
pub struct TuiStyle;

impl ElicitationStyle for TuiStyle {
    fn prompt_for_field(&self, name: &str, ty: &str, ctx: &PromptContext) -> String {
        // Format for ratatui widget
        format!("┌─ {} ({}) ─┐", name, ty)
    }
    
    fn use_decorations(&self) -> bool {
        true  // Enable borders/colors/icons
    }
}

// Use in struct
#[derive(Elicit)]
struct Config {
    #[prompt("Server:", style = "tui")]
    host: String,
}

// Apply
let client = ElicitClient::new(peer).with_style::<Config, _>(TuiStyle);
```

---

## KEY TAKEAWAYS

| Aspect | Design |
|--------|--------|
| **What** | Control HOW prompts are formatted |
| **Where** | Defined in type hierarchy via associated type `Style` |
| **How** | Each type generates default-only or multi-variant Style enum |
| **Scope** | Type-specific - each type can have its own Style independently |
| **Fallback** | `T::Style::default()` if no custom style provided |
| **Extensibility** | Users define custom styles by implementing `ElicitationStyle` |
| **Context** | Works for human TUI, CLI, AI agents - same mechanism |
| **Performance** | O(1) lookup, cheap cloning, zero overhead if unused |
| **Safety** | Compiler ensures correct Style type for each Type |

