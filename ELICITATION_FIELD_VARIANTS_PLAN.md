# Elicitation Crate Enhancement Plan
## Support for Enum Variants with Fields

## Core Philosophy: Values as State Machines

**Central Principle:** Every value provokes an elicitation. Complex values aren't "big prompts" - they're **state machines** where:
- Each state represents "what information satisfied so far?"
- Each transition represents "elicit the next piece"
- Terminal state is "value fully constructed"

### The Pattern Hierarchy

**Primitive → Single State**
```rust
i32::elicit() → "Enter number:" → 42
// State machine: [Start] → [Elicit] → [Terminal]
```

**Enum Unit Variant → Select (one transition)**
```rust
Role::elicit() → Select(System|User|Assistant) → Role::User
// State machine: [Start] → [Select] → [Terminal]
```

**Enum Tuple Variant → Select + Sequential States**
```rust
MediaSource::elicit() 
  → Select(Url|Base64|Binary)     // State 1: variant selected
  → String::elicit()               // State 2: field_0 satisfied
  → MediaSource::Url(value)        // Terminal: value complete

// State machine: [Start] → [Select Variant] → [Elicit Field] → [Construct] → [Terminal]
```

**Enum Struct Variant → Select + Multi-Field States**
```rust
Input::elicit()
  → Select(Text|Image|Document)          // State 1: variant = Image
  → Option<String>::elicit() for mime    // State 2: mime satisfied
  → MediaSource::elicit() for source     // State 3: source satisfied (NESTED STATE MACHINE)
  → Input::Image { mime, source }        // Terminal

// Each field is a state transition. Nested types run their own state machines.
```

**Struct → Survey (N states for N fields)**
```rust
ActConfig::elicit()
  → name: String::elicit()               // State 1
  → prompt: Option<String>::elicit()     // State 2
  → max_tokens: Option<u32>::elicit()    // State 3
  → ActConfig { name, prompt, max_tokens }  // Terminal
```

### The Beautiful Recursion

**Key:** Complex values compose because elicitation is recursive by construction. Each field type knows how to elicit itself:

```rust
struct Image {
    mime: Option<String>,      // Option<T> handles its own state machine
    source: MediaSource,       // MediaSource handles its own state machine
}

// Image elicitation just delegates to field state machines:
impl Elicitation for Image {
    async fn elicit(client) -> Result<Self> {
        let mime = Option::<String>::elicit(client).await?;    // Delegate
        let source = MediaSource::elicit(client).await?;       // Delegate
        Ok(Image { mime, source })                             // Compose
    }
}
```

**The derive macro codifies this recursion pattern.**

### State Machine Visualization

**MediaSource::Url(String):**
```
┌─────────────┐
│  Start      │
└──────┬──────┘
       │ "Select media source:"
       ▼
┌──────────────┐
│ State 1:     │ Select from: [Url, Base64, Binary]
│ Variant      │
└──────┬───────┘
       │ User: "Url"
       ▼
┌──────────────┐
│ State 2:     │ String::elicit() - "Enter URL:"
│ field_0      │
└──────┬───────┘
       │ User: "https://..."
       ▼
┌──────────────┐
│ Terminal:    │ MediaSource::Url("https://...")
│ Construct    │
└──────────────┘
```

**Input::Image (nested state machines):**
```
┌─────────────┐
│  Start      │
└──────┬──────┘
       │ "Select input type:"
       ▼
┌──────────────┐
│ State 1:     │ Select: [Text, Image, Document]
│ Variant      │
└──────┬───────┘
       │ User: "Image"
       ▼
┌──────────────┐
│ State 2:     │ Option<String>::elicit()
│ field: mime  │   └→ Some("image/png")
└──────┬───────┘
       │
       ▼
┌──────────────────────────────┐
│ State 3:                     │ MediaSource::elicit()
│ field: source                │   └→ NESTED STATE MACHINE:
│                              │       [Start] → [Select Variant: Url]
│  ┌─────────────────────┐    │              → [Elicit String]
│  │ MediaSource state   │    │              → [Return Url(...)]
│  │ machine runs        │    │
│  └─────────────────────┘    │
└──────────────┬───────────────┘
               │
               ▼
┌──────────────────────────────┐
│ Terminal:                    │
│ Input::Image {               │
│   mime: Some("image/png"),   │
│   source: Url("https://...")│
│ }                            │
└──────────────────────────────┘
```

### Why State Machines?

**1. Composability:** Each type is self-contained. Complex types = nested state machines.

**2. Uniform Interface:** Everything is `.elicit()`. Whether `i32` or `Input::Image`, same API.

**3. Automatic Orchestration:** Derive macro generates state transitions from structure.

**4. Type Safety:** Compiler ensures all fields implement `Elicitation` - composition guaranteed.

**5. Observable:** Each state transition traced → complete visibility into flow.

**6. Interruptible:** Failure at any state preserves partial progress + clear error context.

### Design Implications

**We're not "adding tuple variant support"** - we're **extending the state machine generator** to handle more transition types.

The macro's job:
1. **Analyze value structure** → How many states? What transitions?
2. **Generate state transitions** → What elicitation at each state?
3. **Compose final value** → Terminal state constructor

Unit variant = 1 state (select), tuple variant = N+1 states (select + N fields), struct variant = N+1 states (select + N named fields). Same pattern, different arity.

### Answers to Design Questions

**Field prompts?** Each field is its own state, uses type's default prompt. Field-level `#[prompt]` can be v0.3.0.

**Error handling?** Fail-fast at any state. Stack preserves partial progress for debugging.

**Recursion depth?** Unlimited. Each recursive call is just another state machine. User controls termination.

**Large tuples?** Sequential states work fine - just a longer state machine (like structs with many fields).

**Variant labels?** Use ident as-is (current). Human-friendly can be v0.3.0 if needed.

---

## Executive Summary

The elicitation crate currently supports unit-variant enums (Select pattern) and structs with named fields (Survey pattern). This document outlines a detailed implementation plan to extend support to enum variants with fields - both tuple variants `Variant(T)` and struct variants `Variant { field: T }`.

**Impact:** Enables elicitation for 5 additional Botticelli types (MediaSource, Input, Output, HealthStatus, ExporterBackend) that are currently blocked.

**Effort:** Estimated 6-10 hours (implementation + tests + docs)

**Risk:** Low - additive changes, existing functionality unaffected

---

## Current State Analysis

### What Works Today ✅

**Unit-variant enums** (Select pattern):
```rust
#[derive(Elicit)]
enum Role {
    System,
    User,
    Assistant,
}

// Generated traits: Select + Prompt + Elicitation
// User experience: Select from "System", "User", "Assistant"
```

**Structs with named fields** (Survey pattern):
```rust
#[derive(Elicit)]
struct Config {
    #[prompt("Enter host:")]
    host: String,
    port: u16,
}

// Generated traits: Survey + Prompt + Elicitation  
// User experience: Elicit each field sequentially
```

### What's Blocked ❌

**Tuple variants:**
```rust
enum MediaSource {
    Url(String),        // ❌ Error: "Variants with fields not supported in v0.1.0"
    Base64(String),     
    Binary(Vec<u8>),    
}
```

**Struct variants:**
```rust
enum Input {
    Text(String),       // ❌ Tuple variant
    Image {             // ❌ Struct variant
        mime: Option<String>,
        source: MediaSource,
    },
}
```

**Mixed variants:**
```rust
enum HealthStatus {
    Healthy,            // ✅ Unit variant (works)
    Degraded {          // ❌ Struct variant (blocked)
        message: String,
    },
}
```

### Impact on Botticelli

**Currently working (7 types):**
- ✅ Role, HistoryRetention, TableFormat, StopReason
- ✅ ExecutionStatus, BotState, FinishReason

**Blocked (5 types):**
- ❌ MediaSource (tuple variants)
- ❌ Input (complex struct variants)
- ❌ Output (tuple variants)
- ❌ HealthStatus (struct variants)
- ❌ ExporterBackend (struct variant with feature gate)

---

## Design Vision

### Two-Phase Elicitation Pattern

**Phase 1: Variant Selection** (existing Select logic)
```
Prompt: "Select MediaSource type:"
Options: ["Url", "Base64", "Binary"]
User selects: "Url"
```

**Phase 2: Field Elicitation** (NEW logic)
```
For Url(String):
  → String::elicit(client) 
  → MediaSource::Url(value)

For Image { mime, source }:
  → Option<String>::elicit(client) for mime
  → MediaSource::elicit(client) for source
  → Input::Image { mime, source }
```

### User Experience Example

**Eliciting MediaSource::Url:**
```
1. AI: "Select media source type: Url, Base64, or Binary?"
   User: "Url"
   
2. AI: "Enter URL:"
   User: "https://example.com/image.png"
   
3. Returns: MediaSource::Url("https://example.com/image.png")
```

**Eliciting Input::Image:**
```
1. AI: "Select input type: Text, Image, Document, or Table?"
   User: "Image"
   
2. AI: "Enter MIME type (optional):"
   User: "image/png"
   
3. AI: "Select media source type: Url, Base64, or Binary?"
   User: "Url"
   
4. AI: "Enter URL:"
   User: "https://example.com/cat.png"
   
5. Returns: Input::Image {
       mime: Some("image/png".to_string()),
       source: MediaSource::Url("https://example.com/cat.png".to_string())
   }
```

---

## Implementation Steps

### Step 1: Extend Variant Parsing (enum_impl.rs)

**Current code (lines 25-39):**
```rust
// Extract only unit variants (no fields)
let unit_variants: Vec<_> = data_enum
    .variants
    .iter()
    .filter(|v| matches!(v.fields, Fields::Unit))
    .collect();

if unit_variants.is_empty() {
    return error("requires at least one unit variant").into();
}
```

**Enhanced code:**
```rust
/// Information about an enum variant and its fields.
struct VariantInfo {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    fields: VariantFields,
}

enum VariantFields {
    Unit,
    Tuple(Vec<syn::Type>),
    Struct(Vec<FieldInfo>),
}

struct FieldInfo {
    ident: Option<syn::Ident>,  // None for tuple fields
    ty: syn::Type,
    attrs: Vec<syn::Attribute>,
}

// Parse all variants, categorizing by field type
let variants: Vec<VariantInfo> = data_enum
    .variants
    .iter()
    .map(|v| VariantInfo {
        ident: v.ident.clone(),
        attrs: v.attrs.clone(),
        fields: match &v.fields {
            Fields::Unit => VariantFields::Unit,
            
            Fields::Unnamed(f) => {
                let types = f.unnamed.iter()
                    .map(|field| FieldInfo {
                        ident: None,
                        ty: field.ty.clone(),
                        attrs: field.attrs.clone(),
                    })
                    .collect();
                VariantFields::Tuple(types)
            },
            
            Fields::Named(f) => {
                let fields = f.named.iter()
                    .map(|field| FieldInfo {
                        ident: field.ident.clone(),
                        ty: field.ty.clone(),
                        attrs: field.attrs.clone(),
                    })
                    .collect();
                VariantFields::Struct(fields)
            },
        },
    })
    .collect();

if variants.is_empty() {
    return error("enum must have at least one variant").into();
}
```

**Key changes:**
1. Remove unit-only filter - accept all variants
2. Parse tuple and struct fields
3. Store field metadata (types, attributes)
4. Prepare for code generation

---

### Step 2: Generate Field Elicitation Code

**Add helper function:**
```rust
/// Generate match arm for a single variant.
fn generate_variant_match_arm(
    variant: &VariantInfo,
    enum_ident: &syn::Ident,
) -> TokenStream2 {
    let variant_ident = &variant.ident;
    let label = variant_ident.to_string();
    
    match &variant.fields {
        VariantFields::Unit => {
            // No fields - just construct variant
            quote! {
                #label => {
                    tracing::debug!(variant = #label, "Constructing unit variant");
                    Ok(#enum_ident::#variant_ident)
                }
            }
        }
        
        VariantFields::Tuple(fields) => {
            // Generate sequential field elicitation
            let field_names: Vec<_> = (0..fields.len())
                .map(|i| syn::Ident::new(
                    &format!("field_{}", i), 
                    variant_ident.span()
                ))
                .collect();
            
            let elicit_stmts = fields.iter().enumerate().map(|(i, field)| {
                let field_name = &field_names[i];
                let field_ty = &field.ty;
                
                quote! {
                    tracing::debug!(
                        variant = #label,
                        field_index = #i,
                        field_type = stringify!(#field_ty),
                        "Eliciting tuple field"
                    );
                    let #field_name = <#field_ty as elicitation::Elicitation>::elicit(client).await
                        .map_err(|e| {
                            tracing::error!(
                                variant = #label,
                                field_index = #i,
                                error = %e,
                                "Field elicitation failed"
                            );
                            e
                        })?;
                }
            });
            
            quote! {
                #label => {
                    tracing::debug!(variant = #label, field_count = #(fields.len()), "Eliciting tuple variant");
                    #(#elicit_stmts)*
                    Ok(#enum_ident::#variant_ident(#(#field_names),*))
                }
            }
        }
        
        VariantFields::Struct(fields) => {
            // Generate named field elicitation
            let field_idents: Vec<_> = fields.iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();
            
            let elicit_stmts = fields.iter().map(|field| {
                let field_ident = field.ident.as_ref().unwrap();
                let field_ty = &field.ty;
                let field_name_str = field_ident.to_string();
                
                quote! {
                    tracing::debug!(
                        variant = #label,
                        field = #field_name_str,
                        field_type = stringify!(#field_ty),
                        "Eliciting struct field"
                    );
                    let #field_ident = <#field_ty as elicitation::Elicitation>::elicit(client).await
                        .map_err(|e| {
                            tracing::error!(
                                variant = #label,
                                field = #field_name_str,
                                error = %e,
                                "Field elicitation failed"
                            );
                            e
                        })?;
                }
            });
            
            quote! {
                #label => {
                    tracing::debug!(variant = #label, field_count = #(fields.len()), "Eliciting struct variant");
                    #(#elicit_stmts)*
                    Ok(#enum_ident::#variant_ident { #(#field_idents),* })
                }
            }
        }
    }
}
```

**Key features:**
- Full instrumentation on each elicitation step
- Error context preserved
- Works for any field type that implements Elicitation
- Handles unit, tuple, and struct variants uniformly

---

### Step 3: Update Elicit Implementation Generation

**Modify `generate_elicit_impl()` function:**
```rust
fn generate_elicit_impl(
    enum_ident: &syn::Ident,
    variants: &[VariantInfo],
) -> TokenStream2 {
    let variant_labels: Vec<String> = variants.iter()
        .map(|v| v.ident.to_string())
        .collect();
    
    // Phase 1: Variant selection (existing logic - unchanged)
    let selection_code = quote! {
        let prompt = Self::prompt().unwrap_or("Select an option:");
        let labels = Self::labels();
        
        tracing::debug!(
            enum_name = stringify!(#enum_ident),
            options = ?labels,
            "Eliciting enum variant selection"
        );
        
        let params = elicitation::mcp::select_params(prompt, labels);
        let result = client
            .call_tool(elicitation::rmcp::model::CallToolRequestParam {
                name: elicitation::mcp::tool_names::elicit_select().into(),
                arguments: Some(params),
            })
            .await
            .map_err(|e| {
                tracing::error!(error = ?e, "MCP tool call failed");
                elicitation::ElicitError::from(e)
            })?;
        
        let value = elicitation::mcp::extract_value(result)?;
        let selected = elicitation::mcp::parse_string(value)?;
        
        tracing::debug!(
            selected = %selected,
            "User selected variant"
        );
    };
    
    // Phase 2: Field elicitation based on variant (NEW)
    let match_arms = variants.iter()
        .map(|v| generate_variant_match_arm(v, enum_ident));
    
    quote! {
        #[automatically_derived]
        impl elicitation::Elicitation for #enum_ident {
            #[tracing::instrument(
                skip(client),
                fields(
                    enum_name = stringify!(#enum_ident),
                    variant = tracing::field::Empty
                )
            )]
            async fn elicit(
                client: &elicitation::rmcp::service::Peer<
                    elicitation::rmcp::service::RoleClient
                >,
            ) -> elicitation::ElicitResult<Self> {
                #selection_code
                
                // Record selected variant in span
                tracing::Span::current().record("variant", &selected.as_str());
                
                // Match on selected variant and elicit fields
                match selected.as_str() {
                    #(#match_arms,)*
                    _ => {
                        tracing::error!(
                            selected = %selected,
                            valid_options = ?Self::labels(),
                            "Invalid variant selected"
                        );
                        Err(elicitation::ElicitError::new(
                            elicitation::ElicitErrorKind::InvalidOption {
                                value: selected,
                                options: Self::labels()
                                    .iter()
                                    .map(|s| s.to_string())
                                    .collect(),
                            }
                        ))
                    }
                }
            }
        }
    }
}
```

**Enhancements:**
- Full tracing instrumentation
- Variant recorded in span for observability
- Error handling with context
- Automatic derivation attribute

---

### Step 4: Support #[prompt] on Variants

**Allow variant-specific prompts:**
```rust
#[derive(Elicit)]
#[prompt("Select media source:")]
enum MediaSource {
    #[prompt("URL to remote content")]
    Url(String),
    
    #[prompt("Base64-encoded data")]
    Base64(String),
    
    #[prompt("Raw binary bytes")]
    Binary(Vec<u8>),
}
```

**Implementation:**
```rust
// In VariantInfo, store custom prompt
struct VariantInfo {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    custom_prompt: Option<String>,  // NEW
    fields: VariantFields,
}

// Extract during parsing
fn parse_variant(variant: &syn::Variant) -> VariantInfo {
    let custom_prompt = extract_prompt_attr(&variant.attrs);
    
    VariantInfo {
        ident: variant.ident.clone(),
        attrs: variant.attrs.clone(),
        custom_prompt,
        fields: /* ... */,
    }
}

// Use in label generation for Select trait
fn generate_select_impl(enum_ident: &syn::Ident, variants: &[VariantInfo]) -> TokenStream2 {
    let variant_idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    
    let variant_labels: Vec<String> = variants.iter()
        .map(|v| {
            v.custom_prompt
                .clone()
                .unwrap_or_else(|| v.ident.to_string())
        })
        .collect();
    
    quote! {
        impl elicitation::Select for #enum_ident {
            fn options() -> &'static [Self] {
                &[#(Self::#variant_idents),*]
            }
            
            fn labels() -> &'static [&'static str] {
                &[#(#variant_labels),*]
            }
            
            fn from_label(label: &str) -> Option<Self> {
                match label {
                    #(#variant_labels => Some(Self::#variant_idents),)*
                    _ => None,
                }
            }
        }
    }
}
```

---

### Step 5: Support #[prompt] on Fields (Struct Variants)

**Allow field-level prompts:**
```rust
#[derive(Elicit)]
enum Input {
    Image {
        #[prompt("MIME type (e.g., image/png):")]
        mime: Option<String>,
        
        #[prompt("Where is the image located?")]
        source: MediaSource,
    },
}
```

**Challenge:** Elicitation trait doesn't currently support prompt override

**Solution Option A - Simplest:** Use type's default prompt, document field prompt as comment only

**Solution Option B - Extend trait:**
```rust
// In elicitation crate traits.rs
pub trait Elicitation: Sized + Prompt {
    fn elicit(client: &Peer<RoleClient>) -> impl Future<Output = ElicitResult<Self>> + Send;
    
    // NEW: Optional override
    fn elicit_with_prompt(
        client: &Peer<RoleClient>,
        prompt: &str,
    ) -> impl Future<Output = ElicitResult<Self>> + Send {
        // Default: call MCP tool with custom prompt
        // Each type can override if needed
    }
}
```

Then in generated code:
```rust
let #field_ident = if let Some(custom_prompt) = #field_custom_prompt {
    <#field_ty>::elicit_with_prompt(client, custom_prompt).await?
} else {
    <#field_ty>::elicit(client).await?
};
```

**Recommendation:** Start with Option A (ignore field prompts), add Option B in v0.3.0 if needed.

---

## Testing Strategy

### Unit Tests (Compilation)

**File:** `crates/elicitation_derive/tests/enum_derive_test.rs`

```rust
// Test 1: Simple tuple variant
#[derive(Debug, PartialEq, Elicit)]
enum SimpleTuple {
    Value(String),
}

#[test]
fn test_simple_tuple_compiles() {
    fn requires_elicit<T: Elicitation>() {}
    requires_elicit::<SimpleTuple>();
}

// Test 2: Multi-field tuple
#[derive(Debug, PartialEq, Elicit)]
enum MultiTuple {
    Pair(String, i32),
    Triple(String, i32, bool),
}

#[test]
fn test_multi_tuple_compiles() {
    let labels = MultiTuple::labels();
    assert_eq!(labels, &["Pair", "Triple"]);
}

// Test 3: Struct variant
#[derive(Debug, PartialEq, Elicit)]
enum StructVariant {
    Config {
        host: String,
        port: u16,
    },
}

#[test]
fn test_struct_variant_compiles() {
    fn requires_select<T: Select>() {}
    requires_select::<StructVariant>();
}

// Test 4: Mixed variants
#[derive(Debug, PartialEq, Elicit)]
enum Mixed {
    Unit,
    Tuple(String),
    Struct { value: i32 },
}

#[test]
fn test_mixed_variants() {
    let labels = Mixed::labels();
    assert_eq!(labels.len(), 3);
    assert!(labels.contains(&"Unit"));
    assert!(labels.contains(&"Tuple"));
    assert!(labels.contains(&"Struct"));
}

// Test 5: Nested enum
#[derive(Debug, PartialEq, Elicit)]
enum Inner {
    A,
    B,
}

#[derive(Debug, PartialEq, Elicit)]
enum Outer {
    Contains(Inner),
    Struct { inner: Inner },
}

#[test]
fn test_nested_enum_compiles() {
    fn requires_elicit<T: Elicitation>() {}
    requires_elicit::<Outer>();
}
```

### Integration Tests (Mock MCP)

**File:** `crates/elicitation/tests/field_variants_test.rs`

```rust
use elicitation::{Elicit, Elicitation};

#[derive(Debug, PartialEq, Elicit)]
enum TestEnum {
    Simple(String),
    Complex {
        name: String,
        value: i32,
    },
}

#[tokio::test]
async fn test_elicit_simple_tuple() {
    // Mock MCP client that simulates user interaction
    let mock = MockMcpClient::new()
        .expect_select("TestEnum", vec!["Simple", "Complex"])
        .respond("Simple")
        .expect_elicit::<String>()
        .respond("test value");
    
    let result = TestEnum::elicit(&mock).await.unwrap();
    assert_eq!(result, TestEnum::Simple("test value".to_string()));
}

#[tokio::test]
async fn test_elicit_struct_variant() {
    let mock = MockMcpClient::new()
        .expect_select("TestEnum", vec!["Simple", "Complex"])
        .respond("Complex")
        .expect_elicit::<String>()
        .respond("test name")
        .expect_elicit::<i32>()
        .respond(42);
    
    let result = TestEnum::elicit(&mock).await.unwrap();
    assert_eq!(result, TestEnum::Complex {
        name: "test name".to_string(),
        value: 42,
    });
}

#[tokio::test]
async fn test_elicit_nested_enum() {
    #[derive(Debug, PartialEq, Elicit)]
    enum Inner { A, B }
    
    #[derive(Debug, PartialEq, Elicit)]
    enum Outer {
        Contains(Inner),
    }
    
    let mock = MockMcpClient::new()
        .expect_select("Outer", vec!["Contains"])
        .respond("Contains")
        .expect_select("Inner", vec!["A", "B"])
        .respond("B");
    
    let result = Outer::elicit(&mock).await.unwrap();
    assert_eq!(result, Outer::Contains(Inner::B));
}
```

### Real-World Tests (Botticelli)

**File:** `crates/botticelli_core/tests/elicitation_test.rs`

```rust
use botticelli_core::{MediaSource, Input};
use elicitation::Elicitation;

#[tokio::test]
#[ignore] // Requires real MCP client
async fn test_media_source_real() {
    let client = setup_mcp_client().await;
    
    let source = MediaSource::elicit(&client).await.unwrap();
    
    match source {
        MediaSource::Url(url) => {
            println!("User provided URL: {}", url);
            assert!(!url.is_empty());
        }
        MediaSource::Base64(data) => {
            println!("User provided base64 data: {} bytes", data.len());
        }
        MediaSource::Binary(bytes) => {
            println!("User provided binary: {} bytes", bytes.len());
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_input_image_real() {
    let client = setup_mcp_client().await;
    
    let input = Input::elicit(&client).await.unwrap();
    
    if let Input::Image { mime, source } = input {
        println!("MIME: {:?}", mime);
        println!("Source: {:?}", source);
    } else {
        panic!("Expected Image variant");
    }
}
```

---

## Documentation Updates

### README.md

**Add section after "Supported Types":**
```markdown
## Enum Variants with Fields

Starting in v0.2.0, elicitation supports enums with tuple and struct variants:

### Tuple Variants

```rust
use elicitation::Elicit;

#[derive(Debug, Elicit)]
enum MediaSource {
    Url(String),
    Base64(String),
    Binary(Vec<u8>),
}

// User first selects variant, then provides the field value
let source = MediaSource::elicit(&client).await?;
```

### Struct Variants

```rust
#[derive(Debug, Elicit)]
enum Input {
    Image {
        mime: Option<String>,
        source: MediaSource,
    },
    Text(String),
}

// User selects variant, then provides each field
let input = Input::elicit(&client).await?;
```

### Mixed Variants

All three variant types can coexist:

```rust
#[derive(Debug, Elicit)]
enum Status {
    Ok,                          // Unit variant
    Warning(String),             // Tuple variant
    Error { code: i32, msg: String }, // Struct variant
}
```

The derive macro automatically generates the appropriate elicitation logic based on each variant's structure.
```

### derive macro docs (lib.rs)

**Update Elicit derive documentation:**
```rust
/// # Variant Types
///
/// The derive macro supports three types of enum variants:
///
/// ## Unit Variants (Simple Selection)
/// ```
/// # use elicitation::Elicit;
/// #[derive(Elicit)]
/// enum Role {
///     System,
///     User,
///     Assistant,
/// }
/// ```
/// User sees: "System", "User", "Assistant" - single selection.
///
/// ## Tuple Variants (Select + Field Elicitation)
/// ```
/// # use elicitation::Elicit;
/// #[derive(Elicit)]
/// enum MediaSource {
///     Url(String),
///     Base64(String),
/// }
/// ```
/// User: 1) Selects "Url" or "Base64", 2) Provides String value.
///
/// ## Struct Variants (Select + Multi-Field Survey)
/// ```
/// # use elicitation::Elicit;
/// #[derive(Elicit)]
/// enum Input {
///     Image {
///         mime: Option<String>,
///         source: MediaSource,
///     },
/// }
/// ```
/// User: 1) Selects "Image", 2) Provides each field (mime, then source).
///
/// All three variant types can appear in the same enum. The macro generates
/// appropriate elicitation logic for each variant.
```

### CHANGELOG.md

```markdown
## [0.2.0] - 2026-01-XX

### Added
- **Enum variants with fields** - `#[derive(Elicit)]` now supports:
  - Tuple variants: `Variant(T1, T2, ...)`
  - Struct variants: `Variant { field1: T1, field2: T2 }`
  - Mixed enums with unit, tuple, and struct variants
- Full instrumentation with tracing for field elicitation
- Support for nested enums (enum fields in variants)

### Changed
- Removed v0.1.0 restriction on unit-only enum variants
- Enhanced error messages for field elicitation failures

### Technical Details
- Two-phase elicitation: variant selection → field elicitation
- Recursive elicitation for nested types
- Each field type must implement `Elicitation` trait
```

---

## Implementation Checklist

### Code Changes
- [ ] `enum_impl.rs`: Remove unit-variant filter (lines 26-39)
- [ ] `enum_impl.rs`: Add `VariantInfo` and `FieldInfo` structs
- [ ] `enum_impl.rs`: Parse all variant types (unit, tuple, struct)
- [ ] `enum_impl.rs`: Implement `generate_variant_match_arm()`
- [ ] `enum_impl.rs`: Update `generate_elicit_impl()` with match logic
- [ ] `enum_impl.rs`: Extract variant-level `#[prompt]` attributes
- [ ] `enum_impl.rs`: Add full tracing instrumentation
- [ ] `enum_impl.rs`: Handle error contexts properly

### Testing
- [ ] Unit: Simple tuple variant
- [ ] Unit: Multi-field tuple variant
- [ ] Unit: Simple struct variant
- [ ] Unit: Multi-field struct variant
- [ ] Unit: Mixed variants (unit + tuple + struct)
- [ ] Unit: Nested enums
- [ ] Integration: Mock MCP - tuple variant
- [ ] Integration: Mock MCP - struct variant
- [ ] Integration: Mock MCP - error handling
- [ ] Real-world: MediaSource with actual client
- [ ] Real-world: Input enum with actual client

### Documentation
- [ ] README: Add "Enum Variants with Fields" section
- [ ] README: Update examples
- [ ] lib.rs: Document variant types in Elicit derive
- [ ] paradigm.rs: Clarify Select trait behavior
- [ ] CHANGELOG: Document v0.2.0 additions
- [ ] Migration guide for v0.1.0 → v0.2.0

### Quality
- [ ] All existing tests pass
- [ ] New tests pass
- [ ] `cargo expand` - verify generated code quality
- [ ] Clippy clean
- [ ] Rustfmt applied
- [ ] Documentation builds without warnings
- [ ] CI green on all platforms

### Release
- [ ] Bump version to 0.2.0 in all Cargo.toml
- [ ] Update CHANGELOG with date
- [ ] Create git tag: `v0.2.0`
- [ ] Publish to crates.io
- [ ] Announce in Botticelli updates

---

## Timeline

**Core Implementation:** 3-4 hours
- Modify enum_impl.rs
- Generate match arms
- Basic tuple/struct support

**Testing & Polish:** 2-3 hours
- Write comprehensive tests
- Fix edge cases
- Add instrumentation

**Documentation:** 1-2 hours
- README updates
- API docs
- Examples

**Total:** 6-9 hours for complete implementation

**Minimum viable (tuple only):** 4-5 hours

---

## Open Design Questions

### All Questions Answered via State Machine Philosophy

**1. Field-Level Prompts**

~~Question: Should we extend Elicitation trait with `elicit_with_prompt()`?~~

**Answer:** Each field is its own state transition. The field's type controls its prompt via `Prompt` trait. Field-level `#[prompt]` attributes are sugar for documentation - the type's prompt is what matters. Can add override API in v0.3.0 if UX demands it.

**v0.2.0 approach:** Use type's default prompt. Field docstrings explain purpose.

**2. Variant Label Generation**

~~Question: Ident as-is vs human-friendly vs configurable?~~

**Answer:** Use ident as-is (current behavior). The variant name IS the label. If users want "URL" instead of "Url", they name the variant `URL`. Keep it simple - one source of truth.

**v0.2.0 approach:** `ident.to_string()` for labels. No magic transformations.

**3. Large Tuple Variants**

~~Question: Elicit all vs warn vs allow skip?~~

**Answer:** Sequential states work regardless of count. A 10-field tuple is just a state machine with 10 transitions. Same as a 10-field struct. No artificial limits. If it's overwhelming, user refactors to named struct.

**v0.2.0 approach:** Elicit all fields sequentially. No warnings, no limits.

**4. Error Handling**

~~Question: Abort vs retry vs collect-then-validate?~~

**Answer:** Fail-fast at any state. The call stack preserves partial progress (which states completed). Error context shows which transition failed. Clean semantics: either you get a value or an error.

**v0.2.0 approach:** First elicitation error aborts, returns with context.

**5. Recursive Enums**

~~Question: Unlimited vs depth limit vs manual impl?~~

**Answer:** Unlimited recursion. Each recursive call is another state machine. For `Tree` with `Node { left: Box<Tree>, right: Box<Tree> }`, user creates leaves by selecting a non-recursive variant (e.g., `Leaf`). Natural termination via variant choice.

**v0.2.0 approach:** Allow unbounded recursion. Trust user to terminate via variant selection.

---

## Success Metrics

1. ✅ All 5 Botticelli blocked types work with `#[derive(Elicit)]`
2. ✅ Zero regressions in existing tests
3. ✅ New field-variant tests pass
4. ✅ Generated code is clean (<50 lines per variant)
5. ✅ Documentation is comprehensive
6. ✅ Real MCP usage in Botticelli succeeds

---

## Notes

- **Additive change** - no breaking changes to v0.1.0 API
- **Backward compatible** - existing unit-variant code unchanged
- **Composable** - works with all existing Elicitation impls
- **Well-instrumented** - full tracing for debugging
- **Type-safe** - compiler ensures field types implement Elicitation

This enhancement completes the derive macro's support for common Rust enum patterns and unblocks aggressive elicitation adoption in Botticelli. 

**The foundation is already there - we're just removing an artificial MVP limitation and letting the architecture shine.**
