# Deep Dive: The `Style` Associated Type System in Elicitation

## Executive Summary

The `Style` associated type system is a **trait-based architecture** that **separates elicitation behavior from presentation**. Every type has a `Style` enum that controls HOW prompts are presented (concise, verbose, TUI-based) without changing WHAT questions are asked. This enables seamless switching between contexts: human TUI (ratatui), CLI, and AI agents (MCP tools).

---

## 1. THE STYLE ASSOCIATED TYPE - Core Definition

### Location
- **Primary trait definition**: `/crates/elicitation/src/traits.rs` (lines 75-84)
- **Associated type bounds**: Lines 76-84

### The Trait Declaration

```rust
pub trait Elicitation: Sized + Prompt + 'static {
    /// The style enum for this type.
    ///
    /// Controls how prompts are presented. For types with multiple styles,
    /// this enum has variants for each style. For types with no custom styles,
    /// this enum has only a `Default` variant.
    ///
    /// The style enum itself implements `Elicitation` (using the Select pattern),
    /// enabling automatic style selection when no style is pre-set.
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    
    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> impl std::future::Future<Output = ElicitResult<Self>> + Send;
}
```

### Key Constraints on `Style`

```
Style: Elicitation     // Style enum itself must be elicitable (recursively)
     + Default         // Provides fallback when no custom style set
     + Clone           // Must be cloneable for context storage
     + Send + Sync     // Must be thread-safe (for Arc storage)
     + 'static         // Required for type-erased storage in StyleContext
```

### Critical Design Insight

> **The Style enum implements `Elicitation` itself**, which means:
> - Style enums can be elicited from users interactively
> - Style selection is "lazy" - only elicited when needed
> - Each Style enum has a corresponding `Style::Style` that points to itself (recursive)
> - This enables runtime style negotiation between human and AI agents

---

## 2. ElicitCommunicator TRAIT - The Bridge to Style

### Location
`/crates/elicitation/src/communicator.rs` (lines 15-190)

### Full Type Signature

```rust
pub trait ElicitCommunicator: Clone + Send + Sync {
    /// Send a prompt and receive a text response.
    fn send_prompt(&self, prompt: &str) 
        -> impl std::future::Future<Output = ElicitResult<String>> + Send;

    /// Call an MCP tool directly with given parameters.
    fn call_tool(&self, params: rmcp::model::CallToolRequestParams)
        -> impl std::future::Future<
            Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>
        > + Send;

    /// Get the style context for type-specific styles.
    fn style_context(&self) -> &StyleContext;

    /// Create a new communicator with a style added for a specific type.
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self;

    /// Get the current style for a type, or use default if not set.
    fn style_or_default<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
    where T::Style: ElicitationStyle;

    /// Get the current style for a type, eliciting if not set.
    fn style_or_elicit<T: Elicitation + 'static>(
        &self,
    ) -> impl std::future::Future<Output = ElicitResult<T::Style>> + Send
    where T::Style: ElicitationStyle;

    /// Get the elicitation context for introspection.
    fn elicitation_context(&self) -> &ElicitationContext;
}
```

### Style Management Methods Breakdown

#### `style_or_default<T: Elicitation>()`
**Purpose**: Retrieve the style for type T, defaulting to `T::Style::default()` if none set

```rust
fn style_or_default<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>
where
    T::Style: ElicitationStyle,
{
    Ok(self
        .style_context()
        .get_style::<T, T::Style>()?
        .unwrap_or_default())
}
```

**Use case**: When starting elicitation, check if user pre-set a style; if not, use the type's default.

#### `style_or_elicit<T: Elicitation>()`
**Purpose**: Retrieve the style for type T, eliciting from the user/agent if not pre-set

```rust
fn style_or_elicit<T: Elicitation + 'static>(
    &self,
) -> impl std::future::Future<Output = ElicitResult<T::Style>> + Send
where
    T::Style: ElicitationStyle,
{
    async move {
        if let Some(style) = self.style_context().get_style::<T, T::Style>()? {
            Ok(style)
        } else {
            T::Style::elicit(self).await
        }
    }
}
```

**Use case**: Interactive style selection - if the user hasn't pre-selected a style, ask them which one they prefer.

#### `with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self`
**Purpose**: Create a new communicator with a custom style for type T

```rust
fn with_style<T: Elicitation + 'static, S: ElicitationStyle>(&self, style: S) -> Self;
```

**Returns**: A new communicator instance with the style added to the context.

**Use case**: One-off style override:
```rust
let client = base_client
    .with_style::<Config, _>(ConfigStyle::Curt)
    .with_style::<i32, _>(VerboseI32Style);
```

### StyleContext - The Internal Storage

```rust
#[derive(Clone, Default)]
pub struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn std::any::Any + Send + Sync>>>>,
}
```

**Key design**:
- Uses `TypeId` as key, allowing each type to have its own style independently
- Type-erased storage with `Box<dyn Any>` to support heterogeneous style types
- `Arc<RwLock<_>>` enables cheap cloning and thread-safe access
- No performance penalty for unused styles - O(1) lookup

---

## 3. #[derive(Elicit)] MACRO - Style Code Generation

### Entry Point
`/crates/elicitation_derive/src/derive_elicit.rs`

### For STRUCTS (Survey Pattern)

**File**: `/crates/elicitation_derive/src/struct_impl.rs`

#### Simple Case (No Custom Styles) - Line 900-920

Generated code for a struct with no `#[prompt(..., style = "...")]` attributes:

```rust
// Generate default-only style enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ConfigStyle {
    /// Default elicitation style.
    #[default]
    Default,
}

impl Prompt for ConfigStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for ConfigStyle {
    type Style = ConfigStyle;  // Self-reference!

    async fn elicit<C: ElicitCommunicator>(
        _communicator: &C,
    ) -> ElicitResult<Self> {
        Ok(Self::Default)  // Always default - no choice needed
    }
}

// Main struct implementation
impl Elicitation for Config {
    type Style = ConfigStyle;

    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> ElicitResult<Self> {
        tracing::debug!(struct_name = "Config", "Eliciting struct");
        let host = <String>::elicit(communicator).await?;
        let port = <u16>::elicit(communicator).await?;
        Ok(Self { host, port })
    }
}
```

#### Styled Case (With Custom Styles) - Line 943-1050+

When a struct has `#[prompt(..., style = "...")]` attributes, the macro generates a multi-variant Style enum:

**Example input**:
```rust
#[derive(Elicit)]
struct GameConfig {
    #[prompt("Enter server name:")]
    #[prompt("Server:", style = "compact")]
    server: String,
    
    #[prompt("Enter port (default 8080):")]
    #[prompt("Port:", style = "compact")]
    port: u16,
}
```

**Generated style enum** (from line 953-975):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GameConfigElicitStyle {
    #[default]
    Default,
    Compact,
    // ... other style variants
}

impl Elicitation for GameConfigElicitStyle {
    type Style = GameConfigElicitStyle;

    async fn elicit<C: ElicitCommunicator>(
        _communicator: &C,
    ) -> ElicitResult<Self> {
        // Select from available styles
        let labels = vec!["default", "compact"];
        // ... user selects style ...
        Ok(Self::Compact)  // Or user's choice
    }
}
```

**Generated field elicitation with style-aware prompts** (from line 977-1050):

```rust
impl Elicitation for GameConfig {
    type Style = GameConfigElicitStyle;

    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> ElicitResult<Self> {
        // Get the style (or let user choose)
        let style = communicator.style_or_elicit::<Self>().await?;
        
        // Elicit server field with style-aware prompt
        let prompt = match style {
            GameConfigElicitStyle::Default => "Enter server name:",
            GameConfigElicitStyle::Compact => "Server:",
        };
        let server = communicator.send_prompt(prompt).await?;
        
        // Elicit port field with style-aware prompt
        let prompt = match style {
            GameConfigElicitStyle::Default => "Enter port (default 8080):",
            GameConfigElicitStyle::Compact => "Port:",
        };
        let port: u16 = communicator.send_prompt(prompt)
            .await?
            .trim()
            .parse()
            .map_err(|_| ElicitError::new(ElicitErrorKind::ParseError(...)))?;
        
        Ok(Self { server, port })
    }
}
```

### For ENUMS (Select Pattern)

**File**: `/crates/elicitation_derive/src/enum_impl.rs`

#### Style Generation (Line 484-513)

Enums always get a simple default-only style enum:

```rust
/// Style enum for this type (default-only for now).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayerActionStyle {
    /// Default elicitation style.
    #[default]
    Default,
}

impl Prompt for PlayerActionStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for PlayerActionStyle {
    type Style = PlayerActionStyle;

    async fn elicit<C: ElicitCommunicator>(
        _communicator: &C,
    ) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}
```

#### Enum Elicitation (Line 327-481)

```rust
impl Elicitation for PlayerAction {
    type Style = PlayerActionStyle;

    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> ElicitResult<Self> {
        // Phase 1: Variant selection
        let base_prompt = Self::prompt().unwrap();
        let labels = Self::labels();  // ["Hit", "Stand", "DoubleDown"]
        
        let options_text = labels.iter()
            .enumerate()
            .map(|(i, label)| format!("{}. {}", i + 1, label))
            .collect::<Vec<_>>()
            .join("\n");
        
        let full_prompt = format!(
            "{}\n\nOptions:\n{}\n\nRespond with the number (1-{}) or exact label:",
            base_prompt, options_text, labels.len()
        );
        
        let response = communicator.send_prompt(&full_prompt).await?;
        let selected = parse_response(response, &labels)?;
        
        // Phase 2: Field elicitation based on variant
        match selected.as_str() {
            "Hit" => Ok(PlayerAction::Hit),
            "Stand" => Ok(PlayerAction::Stand),
            "DoubleDown" => {
                // Elicit nested data if variant has fields
                let bet_amount = <u32>::elicit(communicator).await?;
                Ok(PlayerAction::DoubleDown(bet_amount))
            }
            _ => Err(ElicitError::new(ElicitErrorKind::InvalidOption {...})),
        }
    }
}
```

---

## 4. Existing Style Implementations

### A. Default Style Library

**File**: `/crates/elicitation/src/style.rs`

#### Core Trait - ElicitationStyle

```rust
pub trait ElicitationStyle: Send + Sync {
    /// Generate prompt text for a field.
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String;

    /// Generate optional help text for a field.
    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String> {
        None
    }

    /// Format validation error messages.
    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("Invalid value for {}: {}", field_name, error)
    }

    /// Whether to show type hints in prompts.
    fn show_type_hints(&self) -> bool {
        true
    }

    /// Style for select/dropdown interactions.
    fn select_style(&self) -> SelectStyle {
        SelectStyle::Menu
    }

    /// Whether to use decorative elements (borders, icons, etc.).
    fn use_decorations(&self) -> bool {
        false
    }

    /// Prefix for prompts (e.g., "? ", "> ").
    fn prompt_prefix(&self) -> &str {
        ""
    }
}
```

#### Built-in Implementations

##### 1. **DefaultStyle** (Line 172-192)
```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultStyle;

impl ElicitationStyle for DefaultStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        _context: &PromptContext,
    ) -> String {
        format!("Enter {} ({}):", field_name, field_type)
    }

    fn show_type_hints(&self) -> bool {
        true
    }

    fn select_style(&self) -> SelectStyle {
        SelectStyle::Menu
    }
}
```

**Output**: `"Enter host (String):"`

##### 2. **CompactStyle** (Line 201-225)
```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct CompactStyle;

impl ElicitationStyle for CompactStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        _field_type: &str,
        _context: &PromptContext,
    ) -> String {
        format!("{}:", field_name)
    }

    fn show_type_hints(&self) -> bool {
        false
    }

    fn prompt_prefix(&self) -> &str {
        "> "
    }
}
```

**Output**: `"> host:"`

##### 3. **VerboseStyle** (Line 234-278)
```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct VerboseStyle;

impl ElicitationStyle for VerboseStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String {
        format!(
            "Please enter {} (type: {}, field {}/{})",
            field_name,
            field_type,
            context.field_index + 1,
            context.total_fields
        )
    }

    fn help_text(&self, _field_name: &str, field_type: &str) -> Option<String> {
        Some(match field_type {
            "String" => "Enter any text value".to_string(),
            "u16" | "u32" | "u64" => "Enter a positive number".to_string(),
            "i16" | "i32" | "i64" => "Enter any integer".to_string(),
            "f32" | "f64" => "Enter a decimal number".to_string(),
            "bool" => "Enter yes or no".to_string(),
            _ => format!("Enter a valid {}", field_type),
        })
    }

    fn prompt_prefix(&self) -> &str {
        "? "
    }
}
```

**Output**: `"? Please enter host (type: String, field 1/2)"`

##### 4. **WizardStyle** (Line 287-337)
```rust
#[derive(Debug, Clone, Copy, Default)]
pub struct WizardStyle;

impl ElicitationStyle for WizardStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String {
        format!(
            "Step {} of {}: Enter {} ({})",
            context.field_index + 1,
            context.total_fields,
            field_name,
            field_type
        )
    }

    fn help_text(&self, _field_name: &str, field_type: &str) -> Option<String> {
        Some(match field_type {
            "String" => "Type your answer and press Enter".to_string(),
            ty if ty.starts_with('u') || ty.starts_with('i') || ty.starts_with('f') => {
                "Enter a numeric value".to_string()
            }
            "bool" => "Answer yes or no".to_string(),
            _ => "Enter your response".to_string(),
        })
    }

    fn use_decorations(&self) -> bool {
        true
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("❌ Invalid {}: {}. Let's try again.", field_name, error)
    }

    fn prompt_prefix(&self) -> &str {
        "➤ "
    }
}
```

**Output**: `"➤ Step 1 of 2: Enter host (String)"`

### B. Primitive Type Styles

**File**: `/crates/elicitation/src/primitives/integers.rs` (Line 14-60)

```rust
// Generate default-only style enum for i32
crate::default_style!(i32 => I32Style);

impl Elicitation for i32 {
    type Style = I32Style;

    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> ElicitResult<Self> {
        use crate::verification::types::I32Default;
        let wrapper = I32Default::elicit(communicator).await?;
        Ok(wrapper.into_inner())
    }
}
```

Similarly for:
- `bool` → `BoolStyle` (/crates/elicitation/src/primitives/boolean.rs)
- `u8, u16, u32, u64, u128, usize` → `U8Style`, `U16Style`, etc.
- `i8, i16, i32, i64, i128, isize` → `I8Style`, `I16Style`, etc.
- `f32, f64` → `F32Style`, `F64Style`
- `String` → `StringStyle`

---

## 5. The Prompt Trait

**File**: `/crates/elicitation/src/traits.rs` (lines 43-50)

```rust
/// Shared metadata for prompts across all elicitation patterns.
///
/// This trait provides optional prompt text to guide user interaction.
/// Types can override this to provide custom prompts, or accept the
/// default (None).
pub trait Prompt {
    /// Optional prompt to guide user interaction.
    ///
    /// Returns `None` by default. Implement this to provide a custom prompt
    /// for a type.
    fn prompt() -> Option<&'static str> {
        None
    }
}
```

### How Prompt Relates to Style

1. **Prompt** = WHAT question to ask (the semantic content)
2. **Style** = HOW to present that question (the formatting/UX)

**Example**:
```
Type Config {
    Prompt::prompt() -> "Let's create a Config:"
    
    Then for each field during elicitation:
        DefaultStyle::prompt_for_field("host", "String", ...) 
            -> "Enter host (String):"
        
        CompactStyle::prompt_for_field("host", "String", ...)
            -> "host:"
}
```

---

## 6. The Select Trait

**File**: `/crates/elicitation/src/paradigm.rs` (lines 45-89)

```rust
pub trait Select: Prompt + Sized {
    /// All valid options for this selection.
    fn options() -> Vec<Self>;

    /// Human-readable labels for each option.
    fn labels() -> Vec<String>;

    /// Parse a label back into the type.
    fn from_label(label: &str) -> Option<Self>;

    /// Select from filtered options.
    fn select_with_filter<F, V>(filter: F) -> V
    where
        Self: Filter<F, V>,
    {
        Self::select_filtered(filter)
    }
}
```

### How Select Relates to Style for Enums

For an enum like:
```rust
#[derive(Elicit)]
enum PlayerAction {
    Hit,
    Stand,
    DoubleDown(u32),
}
```

**Generated Select impl**:
```rust
impl Select for PlayerAction {
    fn options() -> Vec<Self> {
        vec![PlayerAction::Hit, PlayerAction::Stand, PlayerAction::DoubleDown(0)]
    }

    fn labels() -> Vec<String> {
        vec!["Hit".to_string(), "Stand".to_string(), "DoubleDown".to_string()]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            "Hit" => Some(PlayerAction::Hit),
            "Stand" => Some(PlayerAction::Stand),
            "DoubleDown" => Some(PlayerAction::DoubleDown(0)),
            _ => None,
        }
    }
}
```

**Style interaction**:
- Style doesn't change the OPTIONS (always Hit/Stand/DoubleDown)
- Style changes HOW they're presented:
  - `SelectStyle::Menu` → "1. Hit\n2. Stand\n3. DoubleDown"
  - `SelectStyle::Inline` → "Choose: Hit, Stand, or DoubleDown?"
  - `SelectStyle::Search` → Searchable/filterable list

---

## 7. Custom Style Example - Game Action

### Problem Statement

A game action enum should present differently:
- **Human TUI**: Pretty ratatui widget showing options with colors/icons
- **AI Agent**: JSON schema for tool parameters

### Definition

```rust
use elicitation::Elicit;

#[derive(Debug, Clone, Copy, Elicit)]
#[prompt("Choose your action:")]
pub enum PlayerAction {
    Hit,
    Stand,
    DoubleDown,
}
```

### Auto-generated Style

```rust
// Generated by #[derive(Elicit)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayerActionStyle {
    #[default]
    Default,
}

impl Prompt for PlayerActionStyle {
    fn prompt() -> Option<&'static str> {
        None
    }
}

impl Elicitation for PlayerActionStyle {
    type Style = PlayerActionStyle;

    async fn elicit<C: ElicitCommunicator>(
        _communicator: &C,
    ) -> ElicitResult<Self> {
        Ok(Self::Default)
    }
}

impl Elicitation for PlayerAction {
    type Style = PlayerActionStyle;

    async fn elicit<C: ElicitCommunicator>(
        communicator: &C,
    ) -> ElicitResult<Self> {
        let labels = vec!["Hit", "Stand", "DoubleDown"];
        let prompt = "Choose your action:\n\n1. Hit\n2. Stand\n3. DoubleDown";
        let response = communicator.send_prompt(prompt).await?;
        
        match parse_selection(&response, &labels)? {
            0 => Ok(PlayerAction::Hit),
            1 => Ok(PlayerAction::Stand),
            2 => Ok(PlayerAction::DoubleDown),
            _ => Err(ElicitError::new(ElicitErrorKind::InvalidOption { ... })),
        }
    }
}
```

### How to Use with Different Contexts

#### Human TUI Context (CLI)

```rust
let client = ElicitClient::new(peer);
let action = PlayerAction::elicit(&client).await?;
// Uses default style, MCP tool sends text prompts
```

#### AI Agent Context (Tool Schema)

```rust
// MCP exposes PlayerAction via #[elicit_tools]
#[elicit_tools(PlayerAction)]
#[tool_router]
impl GameServer {}

// When called via MCP:
// The schema auto-generates from elicitation structure
// Agent receives: 
// {
//   "type": "Select",
//   "options": ["Hit", "Stand", "DoubleDown"],
//   "description": "Choose your action:"
// }
```

---

## 8. Elicitation Flow - The Complete Chain

### Complete Flow When `PlayerAction::elicit(&client).await` is Called

```
1. USER INITIATES
   PlayerAction::elicit(&client).await
   
2. STYLE RESOLUTION
   │
   ├─ client.style_context().get_style::<PlayerAction, PlayerActionStyle>()?
   │  │
   │  ├─ Returns Some(PlayerActionStyle::Default) if pre-set
   │  └─ Returns None if not pre-set
   │
   └─ If None: Prompt user for style choice
      (But PlayerActionStyle has only Default, so returns immediately)

3. ENUM VARIANT SELECTION
   │
   ├─ Build prompt from Prompt::prompt()
   │  └─ "Choose your action:\n\nOptions:\n1. Hit\n2. Stand\n3. DoubleDown"
   │
   ├─ Call communicator.send_prompt(&prompt).await?
   │  │
   │  ├─ [Client-side]: Would call MCP tool (not yet implemented)
   │  └─ [Server-side]: Calls peer.create_message() to send to client
   │
   └─ Parse response to get selected label

4. VARIANT-SPECIFIC FIELD ELICITATION
   │
   ├─ If "Hit" or "Stand" → Unit variant, no fields
   │  └─ Return Ok(PlayerAction::Hit) or Ok(PlayerAction::Stand)
   │
   └─ If "DoubleDown" → Tuple variant with u32 field
      │
      └─ Call <u32>::elicit(communicator).await?
         │
         ├─ Get u32's style (U32Style::Default)
         ├─ Generate prompt: "Please enter a u32 (...)"
         ├─ Call communicator.send_prompt(...).await?
         ├─ Parse response as u32
         └─ Return Ok(value)

5. CONSTRUCT AND RETURN
   Ok(PlayerAction::DoubleDown(bet_amount))
```

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│ Application Code                                        │
│ let action = PlayerAction::elicit(&client).await?       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│ ElicitClient / ElicitServer                             │
│ - Carries StyleContext                                  │
│ - Implements ElicitCommunicator trait                   │
└────────────────────┬────────────────────────────────────┘
                     │
         ┌───────────┴───────────┐
         ▼                       ▼
    ┌─────────────┐      ┌──────────────────┐
    │ StyleContext│      │ ElicitationContext
    │             │      │ (observability)  │
    │ HashMap<    │      └──────────────────┘
    │  TypeId,    │
    │  Box<Style> │
    │>            │
    └─────────────┘
         ▲
         │ style_or_default/style_or_elicit
         │
    ┌────┴─────────────────────────────────────┐
    │ Elicitation impl (generated by macro)    │
    │                                          │
    │ impl Elicitation for PlayerAction {      │
    │   type Style = PlayerActionStyle;        │
    │                                          │
    │   async fn elicit(comm) -> Result<Self> │
    │ }                                        │
    └────┬──────────────────────────────────────┘
         │
         │ send_prompt()
         │
    ┌────▼──────────────────────────────────────┐
    │ MCP Communication Layer                   │
    │                                          │
    │ [Server-side]                            │
    │ peer.create_message() to client          │
    │ ↓ send prompt ↓                          │
    │ ↑ receive response ↑                     │
    │                                          │
    │ [Client-side - not yet implemented]      │
    │ peer.call_tool()                         │
    └────┬──────────────────────────────────────┘
         │
         ▼
    ┌─────────────────────────────────────┐
    │ Response Parsing                    │
    │ - Match on label                    │
    │ - For nested types: recurse         │
    └─────────────────────────────────────┘
```

---

## 9. TUI / Terminal Integration

### Current Status

**No native ratatui integration yet.**

However, the architecture supports it:

1. **Define a custom style for your TUI type**:
   ```rust
   #[derive(Clone, Default)]
   pub struct RatatuiStyle;
   
   impl ElicitationStyle for RatatuiStyle {
       fn prompt_for_field(&self, name: &str, ty: &str, ctx: &PromptContext) -> String {
           // Generate styled prompt
           format!("┌─ {} ({}) ─┐", name, ty)
       }
       
       fn use_decorations(&self) -> bool {
           true
       }
   }
   ```

2. **Apply to struct**:
   ```rust
   #[derive(Elicit)]
   struct Config {
       #[prompt("Server address:", style = "ratatui")]
       host: String,
   }
   ```

3. **Use with client**:
   ```rust
   let client = ElicitClient::new(peer)
       .with_style::<Config, _>(RatatuiStyle);
   let config = Config::elicit(&client).await?;
   ```

### Future Direction

- **TuiStyle** trait for ratatui widget rendering
- **TerminalStyle** for cross-terminal (crossterm, termion) support
- **HumanStyle** for interactive CLI with color/formatting

---

## 10. Type Signatures Summary

### Core Trait Hierarchy

```rust
pub trait Prompt {
    fn prompt() -> Option<&'static str>;
}

pub trait Elicitation: Sized + Prompt + 'static {
    type Style: Elicitation + Default + Clone + Send + Sync + 'static;
    async fn elicit<C: ElicitCommunicator>(communicator: &C) 
        -> impl Future<Output = ElicitResult<Self>> + Send;
}

pub trait ElicitCommunicator: Clone + Send + Sync {
    async fn send_prompt(&self, prompt: &str) -> ElicitResult<String>;
    async fn call_tool(&self, params: CallToolRequestParams) 
        -> Result<CallToolResult, ServiceError>;
    fn style_context(&self) -> &StyleContext;
    fn with_style<T: 'static, S: ElicitationStyle>(&self, style: S) -> Self;
    fn style_or_default<T: Elicitation + 'static>(&self) -> ElicitResult<T::Style>;
    async fn style_or_elicit<T: Elicitation + 'static>(&self) 
        -> ElicitResult<T::Style>;
    fn elicitation_context(&self) -> &ElicitationContext;
}

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

pub trait Select: Prompt + Sized {
    fn options() -> Vec<Self>;
    fn labels() -> Vec<String>;
    fn from_label(label: &str) -> Option<Self>;
}

pub trait Survey: Prompt {
    fn fields() -> Vec<FieldInfo>;
}

pub trait Affirm: Prompt {}
```

### Storage Types

```rust
pub struct StyleContext {
    styles: Arc<RwLock<HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

pub struct ElicitationContext {
    stack: Arc<RwLock<Vec<TypeMetadata>>>,
}

pub struct ElicitClient {
    peer: Arc<Peer<RoleClient>>,
    style_context: StyleContext,
    elicitation_context: ElicitationContext,
}

pub struct ElicitServer {
    peer: Peer<RoleServer>,
    style_context: StyleContext,
    elicitation_context: ElicitationContext,
}
```

---

## 11. Key Design Principles

### 1. **Separation of Concerns**
- **Behavior** (what to ask) vs. **Presentation** (how to ask)
- Style is orthogonal to elicitation logic

### 2. **Type-Safe Style Selection**
- `with_style::<T, S>()` ensures `S` is valid for `T`
- Compiler prevents mixing styles across types

### 3. **Recursive Elegance**
- Style enums implement `Elicitation`, enabling style selection
- `T::Style::elicit()` allows interactive style negotiation
- No special cases needed for style selection

### 4. **Lazy Evaluation**
- Styles only elicited when needed (via `style_or_elicit`)
- Pre-set styles used immediately (via `style_or_default`)
- Minimizes user interaction

### 5. **Zero-Cost Abstractions**
- Type-erased storage via `TypeId` key
- O(1) style lookup, no allocation for unused styles
- `StyleContext::clone()` is cheap (`Arc` clone)

### 6. **Context Agnosticism**
- Same code works for:
  - Human CLI (text prompts)
  - TUI (styled text/widgets)
  - AI agents (MCP tool schemas)
  - Anything implementing `ElicitCommunicator`

---

## Conclusion

The `Style` associated type system is a **masterclass in trait-based extensibility**:

1. ✅ **Enables context-aware prompt customization** without changing elicitation logic
2. ✅ **Supports human TUI, CLI, and AI agent contexts** through the same interface
3. ✅ **Type-safe**: Compiler ensures correct style for each type
4. ✅ **Extensible**: Users can define styles without modifying the library
5. ✅ **Efficient**: O(1) lookup, cheap cloning, no accumulation
6. ✅ **Elegant**: Recursive - style enums are themselves elicitable

The architecture scales from simple one-off styles to complex multi-context applications, all while maintaining type safety and performance.
