# Elicitation Style System Implementation Plan

**Version:** elicitation 0.2.4+
**Status:** Planning - Revolutionary feature
**Created:** 2026-01-19

## Vision

Enable users to customize elicitation UX through a trait-based style system. Instead of hardcoding "choose ISO vs components" or other UX decisions, let users (and the library) provide pluggable style implementations that control how elicitation prompts are presented.

**Core idea:** Elicitation behavior (the questions asked) is separate from elicitation style (how questions are presented).

## The Problem We're Solving

### Current Limitation

```rust
// Today: One way to elicit
#[derive(Elicit)]
struct Config {
    host: String,  // Always asks "Enter host:"
    port: u16,     // Always asks "Enter port:"
}
```

Every type gets the same UX treatment. No customization without implementing `Elicitation` manually.

### After Style System

```rust
// Default style (unchanged API)
#[derive(Elicit)]
struct Config {
    host: String,
    port: u16,
}

// Compact style (terse prompts)
#[derive(Elicit)]
#[elicit(style = "compact")]
struct Config {
    host: String,  // "host:" instead of "Enter host:"
    port: u16,     // "port:" instead of "Enter port:"
}

// Verbose style (detailed help)
#[derive(Elicit)]
#[elicit(style = "verbose")]
struct Config {
    host: String,  // "Enter host: The hostname or IP address of the server"
    port: u16,     // "Enter port: Network port (1-65535)"
}

// Wizard style (step-by-step)
#[derive(Elicit)]
#[elicit(style = "wizard")]
struct Config {
    host: String,  // "Step 1 of 2: Enter host"
    port: u16,     // "Step 2 of 2: Enter port"
}

// Custom user style
#[derive(Elicit)]
#[elicit(style = MyCustomStyle)]
struct Config {
    host: String,
    port: u16,
}
```

## Architecture

### Core Trait

```rust
/// Controls how elicitation prompts are presented to users.
///
/// Styles customize the UX of elicitation without changing the underlying
/// questions asked. Built-in styles cover common patterns; users can
/// implement custom styles for specialized needs.
pub trait ElicitationStyle: Send + Sync {
    /// Generate prompt text for a field.
    ///
    /// # Parameters
    /// - `field_name`: Field identifier (e.g., "host")
    /// - `field_type`: Type name (e.g., "String", "u16")
    /// - `context`: Additional metadata (position, total fields, etc.)
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        context: &PromptContext,
    ) -> String;

    /// Generate help text for a field (optional).
    ///
    /// Returns `None` for no help, `Some(text)` to display help.
    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String> {
        None
    }

    /// Format validation error messages.
    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("Invalid {}: {}", field_name, error)
    }

    /// Should type hints be shown in prompts?
    fn show_type_hints(&self) -> bool {
        true
    }

    /// Selection menu style for enums/options.
    fn select_style(&self) -> SelectStyle {
        SelectStyle::Menu
    }

    /// Should we use emoji/unicode decorations?
    fn use_decorations(&self) -> bool {
        false
    }

    /// Prefix for prompts (e.g., "? " for inquire style).
    fn prompt_prefix(&self) -> &str {
        ""
    }
}

/// Context provided to style implementations.
#[derive(Debug, Clone)]
pub struct PromptContext {
    /// Field position in struct (0-indexed)
    pub field_index: usize,
    /// Total number of fields
    pub total_fields: usize,
    /// Parent struct/enum name
    pub parent_name: Option<&'static str>,
    /// Nesting depth (for recursive elicitation)
    pub depth: usize,
}

/// Selection menu rendering style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectStyle {
    /// Multi-line menu with arrow navigation
    Menu,
    /// Single-line inline selection
    Inline,
    /// Searchable/filterable list
    Search,
}
```

### Built-In Styles (0.2.4)

**1. DefaultStyle** (unchanged from today)

```rust
pub struct DefaultStyle;

impl ElicitationStyle for DefaultStyle {
    fn prompt_for_field(&self, field_name: &str, field_type: &str, _: &PromptContext) -> String {
        format!("Enter {}:", field_name)
    }

    fn show_type_hints(&self) -> bool {
        true
    }
}
```

**2. CompactStyle** (minimal, terse)

```rust
pub struct CompactStyle;

impl ElicitationStyle for CompactStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        format!("{}:", field_name)
    }

    fn show_type_hints(&self) -> bool {
        false
    }
}
```

**3. VerboseStyle** (detailed, helpful)

```rust
pub struct VerboseStyle;

impl ElicitationStyle for VerboseStyle {
    fn prompt_for_field(&self, field_name: &str, field_type: &str, ctx: &PromptContext) -> String {
        let hint = match field_type {
            "String" => "text",
            "u16" | "u32" | "i32" => "number",
            "bool" => "yes/no",
            _ => field_type,
        };
        
        if ctx.total_fields > 1 {
            format!(
                "Enter {} ({}, field {}/{})",
                field_name, hint, ctx.field_index + 1, ctx.total_fields
            )
        } else {
            format!("Enter {} ({})", field_name, hint)
        }
    }

    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String> {
        let help = match field_type {
            "String" => "Any text value",
            "u16" => "Integer from 0 to 65535",
            "u32" => "Integer from 0 to 4294967295",
            "i32" => "Integer from -2147483648 to 2147483647",
            "bool" => "Enter 'true' or 'false'",
            _ => return None,
        };
        Some(format!("Help: {}", help))
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        format!("❌ Invalid value for '{}': {}\nPlease try again.", field_name, error)
    }

    fn show_type_hints(&self) -> bool {
        true
    }
}
```

**4. WizardStyle** (step-by-step)

```rust
pub struct WizardStyle;

impl ElicitationStyle for WizardStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, ctx: &PromptContext) -> String {
        if ctx.total_fields > 1 {
            format!(
                "Step {} of {}: Enter {}",
                ctx.field_index + 1,
                ctx.total_fields,
                field_name
            )
        } else {
            format!("Enter {}", field_name)
        }
    }

    fn prompt_prefix(&self) -> &str {
        "➤ "
    }

    fn use_decorations(&self) -> bool {
        true
    }

    fn show_type_hints(&self) -> bool {
        true
    }
}
```

### Derive Macro Integration (0.2.4)

**Syntax:**

```rust
// String lookup (built-in styles)
#[derive(Elicit)]
#[elicit(style = "default")]
struct Config { ... }

// Type reference (user-provided style)
#[derive(Elicit)]
#[elicit(style = MyCustomStyle)]
struct Config { ... }

// No annotation = default style (backward compatible)
#[derive(Elicit)]
struct Config { ... }
```

**Implementation approach:**

```rust
// In elicitation_derive crate
fn derive_elicit_impl(input: DeriveInput) -> TokenStream {
    let style_attr = parse_style_attribute(&input.attrs);
    
    let style_impl = match style_attr {
        Some(StyleAttr::Builtin(name)) => {
            quote! { elicitation::styles::#name::new() }
        }
        Some(StyleAttr::Custom(ty)) => {
            quote! { #ty::new() }
        }
        None => {
            quote! { elicitation::styles::DefaultStyle }
        }
    };

    // Generate Elicitation impl using style_impl
    // ...
}
```

**Generated code example:**

```rust
// For: #[derive(Elicit)] #[elicit(style = "compact")]
impl Elicitation for Config {
    fn elicit() -> Result<Self, ElicitationError> {
        let style = elicitation::styles::CompactStyle;
        let ctx = PromptContext {
            field_index: 0,
            total_fields: 2,
            parent_name: Some("Config"),
            depth: 0,
        };

        let host: String = {
            let prompt = style.prompt_for_field("host", "String", &ctx);
            elicitation::prompts::prompt_text(&prompt)?
        };

        let port: u16 = {
            let ctx = PromptContext { field_index: 1, ..ctx };
            let prompt = style.prompt_for_field("port", "u16", &ctx);
            elicitation::prompts::prompt_number(&prompt)?
        };

        Ok(Config { host, port })
    }
}
```

### Style Registry (0.2.4)

```rust
/// Global registry of named styles.
///
/// Allows runtime style selection without compile-time knowledge.
pub struct StyleRegistry {
    styles: HashMap<String, Box<dyn ElicitationStyle>>,
}

impl StyleRegistry {
    /// Register a style by name.
    pub fn register(&mut self, name: impl Into<String>, style: impl ElicitationStyle + 'static) {
        self.styles.insert(name.into(), Box::new(style));
    }

    /// Get style by name (fallback to default).
    pub fn get(&self, name: &str) -> &dyn ElicitationStyle {
        self.styles.get(name)
            .map(|b| b.as_ref())
            .unwrap_or(&DefaultStyle)
    }
}

// Global registry
static REGISTRY: Lazy<Mutex<StyleRegistry>> = Lazy::new(|| {
    let mut registry = StyleRegistry::new();
    registry.register("default", DefaultStyle);
    registry.register("compact", CompactStyle);
    registry.register("verbose", VerboseStyle);
    registry.register("wizard", WizardStyle);
    Mutex::new(registry)
});

/// Register a custom style globally.
pub fn register_style(name: impl Into<String>, style: impl ElicitationStyle + 'static) {
    REGISTRY.lock().unwrap().register(name, style);
}
```

## Field-Level Style Overrides (0.2.5)

**Syntax:**

```rust
#[derive(Elicit)]
#[elicit(style = "default")]
struct Config {
    /// Regular field (uses struct-level "default")
    host: String,

    /// Override with different style
    #[elicit(style = "compact")]
    port: u16,

    /// Nested struct with its own style
    #[elicit(style = "verbose")]
    advanced: AdvancedConfig,
}
```

**Implementation:**

- Parse field-level `#[elicit(style = ...)]` attributes
- Generate per-field style selection in derived code
- Field style overrides struct style

**Generated code:**

```rust
impl Elicitation for Config {
    fn elicit() -> Result<Self, ElicitationError> {
        let default_style = elicitation::styles::DefaultStyle;
        let compact_style = elicitation::styles::CompactStyle;
        let verbose_style = elicitation::styles::VerboseStyle;

        let host: String = {
            let prompt = default_style.prompt_for_field("host", "String", &ctx);
            elicitation::prompts::prompt_text(&prompt)?
        };

        let port: u16 = {
            let prompt = compact_style.prompt_for_field("port", "u16", &ctx);
            elicitation::prompts::prompt_number(&prompt)?
        };

        let advanced: AdvancedConfig = {
            // Nested type uses its own derived Elicitation impl
            AdvancedConfig::elicit()?
        };

        Ok(Config { host, port, advanced })
    }
}
```

## Datetime-Specific Styles (0.2.6)

After implementing datetime support (0.2.3), refactor to use style system:

**Styles:**

```rust
/// ISO 8601 string input only (fast, copy-paste friendly).
pub struct Iso8601Style;

impl ElicitationStyle for Iso8601Style {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        format!("Enter {} (ISO 8601, e.g., '2024-07-11T15:30:00Z'):", field_name)
    }
}

/// Manual component input (guided, validated).
pub struct ComponentsStyle;

impl ElicitationStyle for ComponentsStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        format!("Enter {} components:", field_name)
    }
    // Multi-step prompts for year, month, day, hour, minute, second
}

/// Smart style: user chooses ISO or components.
pub struct SmartDatetimeStyle;

impl ElicitationStyle for SmartDatetimeStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        // Prompts user to select method, then delegates
        format!("How would you like to enter {}?", field_name)
    }
}
```

**Usage:**

```rust
#[derive(Elicit)]
struct Event {
    name: String,

    #[elicit(style = "iso8601")]
    created_at: DateTime<Utc>,  // Fast ISO string input

    #[elicit(style = "components")]
    scheduled_at: DateTime<Utc>,  // Guided component input
}
```

**Implementation:**

- Datetime-specific `ElicitationStyle` implementations
- Registered as "iso8601", "components", "smart"
- Reuses existing datetime parsing/validation logic

## Ecosystem Extensions (Future)

**Third-party style crates:**

```toml
[dependencies]
elicitation = "0.2"
elicitation-styles-fancy = "0.1"  # Colored prompts, emoji, gradients
elicitation-tui-styles = "0.1"    # ratatui-integrated styles
elicitation-web-styles = "0.1"    # HTML form generation
```

**Framework adapters:**

```rust
// egui integration
#[derive(Elicit)]
#[elicit(style = elicitation_egui::EguiStyle)]
struct Config {
    host: String,  // Rendered as egui TextEdit widget
    port: u16,     // Rendered as egui DragValue widget
}

// ratatui integration
#[derive(Elicit)]
#[elicit(style = elicitation_ratatui::RatatuiStyle)]
struct Config {
    host: String,  // Interactive TUI input
    port: u16,
}
```

**Domain-specific styles:**

```rust
// Config file style (TOML-like prompts)
#[derive(Elicit)]
#[elicit(style = elicitation_config::TomlStyle)]
struct Config { ... }

// API key style (masked input, validation)
#[derive(Elicit)]
#[elicit(style = elicitation_secrets::SecretStyle)]
struct Credentials { ... }
```

## Implementation Plan

### Phase 1: Core Trait + Built-in Styles (0.2.4)

**Effort:** 4-6 hours

**Files to create:**

```
crates/elicitation/src/
├── styles/
│   ├── mod.rs              # StyleRegistry + public API
│   ├── trait.rs            # ElicitationStyle trait
│   ├── context.rs          # PromptContext struct
│   ├── default.rs          # DefaultStyle
│   ├── compact.rs          # CompactStyle
│   ├── verbose.rs          # VerboseStyle
│   └── wizard.rs           # WizardStyle
└── lib.rs                  # pub use styles::*;
```

**Steps:**

1. **Define trait** (`styles/trait.rs`)
   - `ElicitationStyle` trait with methods
   - `PromptContext` struct
   - `SelectStyle` enum
   - Documentation with examples

2. **Implement built-in styles** (`styles/*.rs`)
   - `DefaultStyle` - current behavior
   - `CompactStyle` - minimal prompts
   - `VerboseStyle` - detailed help
   - `WizardStyle` - step-by-step

3. **Style registry** (`styles/mod.rs`)
   - `StyleRegistry` struct
   - Global `REGISTRY` with lazy init
   - `register_style()` public API
   - Pre-register built-in styles

4. **Integration** (`lib.rs`)
   - Export styles module
   - Update docs with style examples
   - Add "Styles" section to README

5. **Testing**
   - Unit tests for each style
   - Registry tests
   - Style trait examples compile
   - Backward compatibility (no macro changes yet)

**Success criteria:**

- ✅ All 4 built-in styles implemented
- ✅ Registry works (register/lookup)
- ✅ Documentation complete
- ✅ Tests pass
- ✅ No breaking changes (backward compatible)

### Phase 2: Derive Macro Integration (0.2.4 continued)

**Effort:** 6-8 hours

**Files to modify:**

```
crates/elicitation_derive/src/
├── lib.rs                  # Parse #[elicit(style = ...)]
├── struct_impl.rs          # Generate style-aware code
└── enum_impl.rs            # Generate style-aware code
```

**Steps:**

1. **Parse style attribute**
   - Detect `#[elicit(style = "name")]` on structs/enums
   - Support string literals ("default", "compact", etc.)
   - Support type paths (`MyStyle`, `crate::MyStyle`)
   - Error on invalid syntax

2. **Code generation**
   - Extract style instance at start of `elicit()` impl
   - Pass `PromptContext` to style methods
   - Use `style.prompt_for_field()` for prompts
   - Use `style.validation_error()` for errors
   - Use `style.select_style()` for enum selection

3. **Backward compatibility**
   - No annotation = `DefaultStyle` (unchanged behavior)
   - All existing derives work without modification
   - New feature is opt-in

4. **Testing**
   - Integration tests for each style
   - Test struct-level annotation
   - Test with custom user styles
   - Test backward compatibility (no annotation)

5. **Documentation**
   - Update derive macro docs
   - Add style examples to `Elicit` trait docs
   - Add "Custom Styles" guide to README

**Success criteria:**

- ✅ Macro accepts `#[elicit(style = ...)]`
- ✅ Generated code uses style methods
- ✅ All 4 built-in styles work via derive
- ✅ Custom user styles work
- ✅ Backward compatible (no annotation = default)
- ✅ Tests pass

### Phase 3: Field-Level Style Overrides (0.2.5)

**Effort:** 3-4 hours

**Files to modify:**

```
crates/elicitation_derive/src/
├── lib.rs                  # Parse field-level attributes
├── struct_impl.rs          # Per-field style selection
└── field.rs                # Field metadata extraction
```

**Steps:**

1. **Parse field attributes**
   - Detect `#[elicit(style = ...)]` on struct fields
   - Support same syntax as struct-level
   - Store per-field style in metadata

2. **Code generation**
   - Instantiate multiple styles if needed
   - Select style per field
   - Field style overrides struct style

3. **Testing**
   - Test field-level overrides
   - Test mixed styles in one struct
   - Test nested structs with different styles

4. **Documentation**
   - Update examples
   - Show field override patterns

**Success criteria:**

- ✅ Field-level `#[elicit(style = ...)]` works
- ✅ Field style overrides struct style
- ✅ Mixed styles in one struct work
- ✅ Tests pass

### Phase 4: Datetime Styles Refactor (0.2.6)

**Effort:** 2-3 hours

**Files to create/modify:**

```
crates/elicitation/src/
├── styles/
│   ├── datetime_iso8601.rs    # ISO string only
│   ├── datetime_components.rs # Manual components
│   └── datetime_smart.rs      # User chooses
└── datetime_common.rs          # Refactor to use styles
```

**Steps:**

1. **Extract datetime styles**
   - Move "smart" UX logic to `SmartDatetimeStyle`
   - Create `Iso8601Style` (string only)
   - Create `ComponentsStyle` (components only)

2. **Refactor datetime impls**
   - Use `SmartDatetimeStyle` by default (backward compatible)
   - Support override with `#[elicit(style = "iso8601")]`
   - Reuse existing parsing/validation

3. **Testing**
   - Test all 3 datetime styles
   - Test override on datetime fields
   - Verify backward compatibility

4. **Documentation**
   - Update datetime feature docs
   - Show style override examples

**Success criteria:**

- ✅ 3 datetime styles implemented
- ✅ Default behavior unchanged ("smart")
- ✅ Users can override with "iso8601" or "components"
- ✅ Tests pass

### Phase 5: Documentation & Examples (0.2.6 continued)

**Effort:** 2-3 hours

**Files to create/modify:**

```
crates/elicitation/
├── README.md               # Add "Styles" section
├── examples/
│   ├── styles_basic.rs     # Built-in styles demo
│   ├── styles_custom.rs    # Custom style example
│   ├── styles_field.rs     # Field-level overrides
│   └── styles_datetime.rs  # Datetime styles
└── STYLE_GUIDE.md          # Comprehensive style guide
```

**Steps:**

1. **README updates**
   - Add "Elicitation Styles" section
   - Show quick examples
   - Link to STYLE_GUIDE.md

2. **Example programs**
   - `styles_basic.rs` - All 4 built-in styles
   - `styles_custom.rs` - Implement custom style
   - `styles_field.rs` - Per-field overrides
   - `styles_datetime.rs` - Datetime style variants

3. **Style guide**
   - When to use which style
   - How to implement custom styles
   - Best practices
   - Ecosystem integration patterns

4. **API documentation**
   - Ensure all public items documented
   - Add style examples to trait docs
   - Cross-link related docs

**Success criteria:**

- ✅ README has styles section
- ✅ 4 runnable examples
- ✅ STYLE_GUIDE.md complete
- ✅ All public APIs documented

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_style_prompt() {
        let style = DefaultStyle;
        let ctx = PromptContext {
            field_index: 0,
            total_fields: 2,
            parent_name: None,
            depth: 0,
        };
        assert_eq!(
            style.prompt_for_field("host", "String", &ctx),
            "Enter host:"
        );
    }

    #[test]
    fn compact_style_no_type_hints() {
        let style = CompactStyle;
        assert!(!style.show_type_hints());
    }

    #[test]
    fn wizard_style_step_numbers() {
        let style = WizardStyle;
        let ctx = PromptContext {
            field_index: 1,
            total_fields: 5,
            parent_name: None,
            depth: 0,
        };
        assert_eq!(
            style.prompt_for_field("port", "u16", &ctx),
            "Step 2 of 5: Enter port"
        );
    }

    #[test]
    fn registry_lookup() {
        let registry = StyleRegistry::new();
        registry.register("test", DefaultStyle);
        assert!(registry.get("test").is_some());
        assert!(registry.get("nonexistent").is_none());
    }
}
```

### Integration Tests

```rust
// tests/styles_test.rs
use elicitation::{Elicit, styles::*};

#[test]
fn derive_with_default_style() {
    #[derive(Elicit)]
    struct Config {
        host: String,
        port: u16,
    }
    // Should compile without errors
}

#[test]
fn derive_with_compact_style() {
    #[derive(Elicit)]
    #[elicit(style = "compact")]
    struct Config {
        host: String,
        port: u16,
    }
    // Should compile and use CompactStyle
}

#[test]
fn derive_with_custom_style() {
    struct MyStyle;
    impl ElicitationStyle for MyStyle {
        fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
            format!("CUSTOM: {}", field_name)
        }
    }

    #[derive(Elicit)]
    #[elicit(style = MyStyle)]
    struct Config {
        host: String,
    }
    // Should compile with user style
}

#[test]
fn field_level_override() {
    #[derive(Elicit)]
    #[elicit(style = "default")]
    struct Config {
        host: String,
        
        #[elicit(style = "compact")]
        port: u16,
    }
    // Should compile with mixed styles
}
```

### Example Tests

```rust
// examples/styles_basic.rs
use elicitation::Elicit;

#[derive(Elicit)]
#[elicit(style = "default")]
struct DefaultExample {
    name: String,
    age: u32,
}

#[derive(Elicit)]
#[elicit(style = "compact")]
struct CompactExample {
    name: String,
    age: u32,
}

#[derive(Elicit)]
#[elicit(style = "verbose")]
struct VerboseExample {
    name: String,
    age: u32,
}

#[derive(Elicit)]
#[elicit(style = "wizard")]
struct WizardExample {
    name: String,
    age: u32,
}

fn main() {
    println!("=== Default Style ===");
    let _default = DefaultExample::elicit().unwrap();

    println!("\n=== Compact Style ===");
    let _compact = CompactExample::elicit().unwrap();

    println!("\n=== Verbose Style ===");
    let _verbose = VerboseExample::elicit().unwrap();

    println!("\n=== Wizard Style ===");
    let _wizard = WizardExample::elicit().unwrap();
}
```

## Versioning & Release Strategy

### Version Progression

All additive, non-breaking changes = patch bumps:

- **0.2.2** - serde_json feature
- **0.2.3** - datetime features (chrono, time, jiff)
- **0.2.4** - Style system core (trait + 4 built-ins + derive support)
- **0.2.5** - Field-level style overrides
- **0.2.6** - Datetime-specific styles refactor

### Backward Compatibility

**Guaranteed:**

- No `#[elicit(style = ...)]` = `DefaultStyle` (unchanged UX)
- All existing derives continue to work
- Generated code compatible with old prompts
- No breaking API changes

**Testing:**

```rust
// Verify existing code still works
#[derive(Elicit)]
struct OldStyle {
    field: String,
}
// Must compile and behave identically to 0.2.3
```

### Release Process

**Per version:**

1. Implement features
2. Write tests (unit + integration)
3. Update docs (API + README)
4. Run full test suite
5. Update CHANGELOG.md
6. Bump version in Cargo.toml
7. `cargo publish --dry-run`
8. `cargo publish`
9. Tag release: `git tag v0.2.X`
10. Push: `git push origin v0.2.X`

## Documentation Plan

### API Documentation

**Trait documentation:**

```rust
/// Controls how elicitation prompts are presented to users.
///
/// The `ElicitationStyle` trait allows customization of elicitation UX
/// without changing the underlying questions asked. Styles control:
/// - Prompt formatting (terse vs verbose)
/// - Help text availability
/// - Error message formatting
/// - Type hint visibility
/// - Selection menu rendering
///
/// # Built-in Styles
///
/// - [`DefaultStyle`] - Balanced prompts with type hints (default)
/// - [`CompactStyle`] - Minimal, terse prompts
/// - [`VerboseStyle`] - Detailed prompts with help text
/// - [`WizardStyle`] - Step-by-step guided prompts
///
/// # Examples
///
/// ## Using a built-in style
///
/// ```rust
/// use elicitation::Elicit;
///
/// #[derive(Elicit)]
/// #[elicit(style = "compact")]
/// struct Config {
///     host: String,
///     port: u16,
/// }
/// ```
///
/// ## Implementing a custom style
///
/// ```rust
/// use elicitation::{ElicitationStyle, PromptContext};
///
/// struct MyStyle;
///
/// impl ElicitationStyle for MyStyle {
///     fn prompt_for_field(
///         &self,
///         field_name: &str,
///         field_type: &str,
///         ctx: &PromptContext,
///     ) -> String {
///         format!("Please enter {}: ", field_name)
///     }
/// }
/// ```
///
/// ## Field-level overrides
///
/// ```rust
/// use elicitation::Elicit;
///
/// #[derive(Elicit)]
/// #[elicit(style = "default")]
/// struct Config {
///     /// Uses default style
///     host: String,
///
///     /// Overrides with compact style
///     #[elicit(style = "compact")]
///     port: u16,
/// }
/// ```
pub trait ElicitationStyle: Send + Sync {
    // ...
}
```

### README Updates

```markdown
# Elicitation

A Rust library for deriving interactive prompts from types.

## Features

- ✅ Derive `Elicit` for structs and enums
- ✅ Automatic type conversions and validation
- ✅ **Customizable styles** for different UX needs
- ✅ Field-level style overrides
- ✅ Optional features: serde_json, datetime (chrono/time/jiff)

## Elicitation Styles

Control how prompts are presented without changing what's asked:

```rust
// Default: balanced prompts
#[derive(Elicit)]
struct Config {
    host: String,  // "Enter host:"
    port: u16,     // "Enter port:"
}

// Compact: minimal prompts
#[derive(Elicit)]
#[elicit(style = "compact")]
struct Config {
    host: String,  // "host:"
    port: u16,     // "port:"
}

// Verbose: detailed help
#[derive(Elicit)]
#[elicit(style = "verbose")]
struct Config {
    host: String,  // "Enter host (text, field 1/2)"
    port: u16,     // "Enter port (number, field 2/2)"
}

// Wizard: step-by-step
#[derive(Elicit)]
#[elicit(style = "wizard")]
struct Config {
    host: String,  // "Step 1 of 2: Enter host"
    port: u16,     // "Step 2 of 2: Enter port"
}
```

### Custom Styles

Implement `ElicitationStyle` for complete control:

```rust
use elicitation::{ElicitationStyle, PromptContext, Elicit};

struct FancyStyle;

impl ElicitationStyle for FancyStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        _: &str,
        _: &PromptContext,
    ) -> String {
        format!("✨ {} ✨", field_name)
    }

    fn use_decorations(&self) -> bool {
        true
    }
}

#[derive(Elicit)]
#[elicit(style = FancyStyle)]
struct Config {
    host: String,
}
```

See [STYLE_GUIDE.md](STYLE_GUIDE.md) for more details.
```

### Style Guide (STYLE_GUIDE.md)

```markdown
# Elicitation Style Guide

## Overview

Elicitation styles control **how** prompts are presented, not **what** is asked.

## Built-in Styles

### DefaultStyle

Balanced prompts suitable for most use cases.

**When to use:**
- General-purpose applications
- Mix of technical and non-technical users
- First-time setup wizards

**Characteristics:**
- Clear field labels: "Enter host:"
- Type hints enabled
- Standard error messages

**Example:**
```
Enter host: _
Enter port: _
```

### CompactStyle

Minimal, terse prompts for experienced users.

**When to use:**
- CLI tools for power users
- Repetitive data entry
- Space-constrained interfaces

**Characteristics:**
- Short labels: "host:"
- No type hints
- Concise errors

**Example:**
```
host: _
port: _
```

### VerboseStyle

Detailed prompts with help text and guidance.

**When to use:**
- First-time user experiences
- Complex configuration
- Educational tools

**Characteristics:**
- Detailed labels with type info
- Field position indicators
- Extensive help text
- Friendly error messages

**Example:**
```
Enter host (text, field 1/2): _
Help: Any text value
Enter port (number, field 2/2): _
Help: Integer from 0 to 65535
```

### WizardStyle

Step-by-step guided experience.

**When to use:**
- Multi-step workflows
- Onboarding experiences
- Guided setup processes

**Characteristics:**
- Step counters: "Step 1 of 5"
- Progress indicators
- Decorative elements

**Example:**
```
➤ Step 1 of 2: Enter host
_
➤ Step 2 of 2: Enter port
_
```

## Custom Styles

### Implementing ElicitationStyle

```rust
use elicitation::{ElicitationStyle, PromptContext, SelectStyle};

pub struct MyStyle;

impl ElicitationStyle for MyStyle {
    fn prompt_for_field(
        &self,
        field_name: &str,
        field_type: &str,
        ctx: &PromptContext,
    ) -> String {
        // Required: generate prompt text
        todo!()
    }

    fn help_text(&self, field_name: &str, field_type: &str) -> Option<String> {
        // Optional: provide help text
        None
    }

    fn validation_error(&self, field_name: &str, error: &str) -> String {
        // Optional: customize error messages
        format!("Error in {}: {}", field_name, error)
    }

    fn show_type_hints(&self) -> bool {
        // Optional: enable/disable type hints
        true
    }

    fn select_style(&self) -> SelectStyle {
        // Optional: menu rendering style
        SelectStyle::Menu
    }

    fn use_decorations(&self) -> bool {
        // Optional: emoji/unicode decorations
        false
    }

    fn prompt_prefix(&self) -> &str {
        // Optional: prefix for all prompts
        ""
    }
}
```

### Style Ideas

**SecretStyle** (masked input):
```rust
pub struct SecretStyle;

impl ElicitationStyle for SecretStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        format!("Enter {} (input hidden):", field_name)
    }
    
    // Implementation would use password prompt
}
```

**ColoredStyle** (ANSI colors):
```rust
pub struct ColoredStyle;

impl ElicitationStyle for ColoredStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        format!("\x1b[34m{}\x1b[0m: ", field_name)  // Blue text
    }
}
```

**JsonStyle** (JSON-like prompts):
```rust
pub struct JsonStyle;

impl ElicitationStyle for JsonStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, ctx: &PromptContext) -> String {
        let indent = "  ".repeat(ctx.depth);
        format!("{}\"{}\": ", indent, field_name)
    }
}
```

## Field-Level Overrides

Mix styles within a single struct:

```rust
#[derive(Elicit)]
#[elicit(style = "default")]
struct Config {
    // Uses default style
    host: String,
    port: u16,

    // Override for sensitive field
    #[elicit(style = SecretStyle)]
    api_key: String,

    // Override for advanced section
    #[elicit(style = "verbose")]
    advanced: AdvancedConfig,
}
```

## Ecosystem Integration

### egui Integration

```rust
pub struct EguiStyle {
    ui: egui::Ui,
}

impl ElicitationStyle for EguiStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        // Render egui widget
        // Return value from widget
        todo!()
    }
}
```

### ratatui Integration

```rust
pub struct RatatuiStyle {
    frame: ratatui::Frame,
}

impl ElicitationStyle for RatatuiStyle {
    fn prompt_for_field(&self, field_name: &str, _: &str, _: &PromptContext) -> String {
        // Render TUI widget
        // Handle input events
        // Return value
        todo!()
    }
}
```

## Best Practices

1. **Keep styles focused**: One UX pattern per style
2. **Respect context**: Use `PromptContext` for intelligent prompts
3. **Consistent decorations**: If using emoji, use consistently
4. **Graceful degradation**: Fallback for unsupported terminals
5. **Document your styles**: Clear use case descriptions

## Testing Styles

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_style_prompts() {
        let style = MyStyle;
        let ctx = PromptContext {
            field_index: 0,
            total_fields: 1,
            parent_name: None,
            depth: 0,
        };
        
        let prompt = style.prompt_for_field("test", "String", &ctx);
        assert_eq!(prompt, "Expected prompt text");
    }
}
```
```

## Benefits

### For Library Users

1. **No manual implementation** - Just `#[derive(Elicit)]` with style attribute
2. **Consistent UX** - Pick a style, get cohesive experience
3. **Easy customization** - Implement trait, plug it in
4. **Mix and match** - Different styles per field/section
5. **Ecosystem growth** - Third-party style crates

### For Library Maintainers

1. **Separation of concerns** - UX decoupled from elicitation logic
2. **Extensibility** - New styles without touching core
3. **Testing** - Styles tested independently
4. **Documentation** - Each style documents its UX pattern
5. **Innovation** - Community can experiment

### For Ecosystem

1. **Framework integration** - egui, ratatui, etc. provide styles
2. **Domain expertise** - Config, secrets, web forms, etc. get specialized styles
3. **Best practices** - Styles encode UX patterns
4. **Reusability** - One style implementation, many use cases
5. **Evolution** - New interaction paradigms as new styles

## Risks & Mitigation

### Risk: Trait Object Overhead

**Impact:** `Box<dyn ElicitationStyle>` adds allocation + vtable indirection

**Mitigation:**
- Derive generates monomorphized code (no trait objects)
- Registry uses trait objects only for dynamic lookup
- Per-field style = compile-time dispatch
- Zero-cost for common case (no annotation = static DefaultStyle)

### Risk: Complexity Creep

**Impact:** Too many style hooks = confusing API

**Mitigation:**
- Start minimal (7 trait methods)
- Default implementations for optional methods
- Clear documentation on when to override
- Built-in styles as reference implementations

### Risk: Backward Compatibility

**Impact:** Style trait evolution breaks custom implementations

**Mitigation:**
- Default implementations for new methods (non-breaking)
- Follow semver strictly (breaking = 0.3.0, not 0.2.x)
- Deprecation warnings before removal
- Long transition periods (multiple minor versions)

### Risk: Testing Burden

**Impact:** More styles = more test combinations

**Mitigation:**
- Unit test each style in isolation
- Integration tests cover built-ins only
- Custom styles tested by users
- Example programs serve as smoke tests

## Success Metrics

### Code Quality

- ✅ All trait methods documented
- ✅ 4 built-in styles implemented
- ✅ 100% test coverage for styles
- ✅ Zero compiler warnings
- ✅ Zero clippy warnings

### User Experience

- ✅ 4 runnable examples
- ✅ STYLE_GUIDE.md comprehensive
- ✅ README showcases styles
- ✅ API docs with style examples

### Adoption

- 📈 GitHub stars increase
- 📈 Crates.io downloads increase
- 📈 Third-party style crates emerge
- 📈 Issues/PRs with custom style examples

### Performance

- ✅ No allocation for default case
- ✅ Compile time unchanged (<5% increase)
- ✅ Binary size unchanged (<5% increase)

## Timeline

**Total: 17-24 hours across 4 versions**

### 0.2.4 (10-14 hours)
- Week 1: Core trait + built-in styles (4-6 hours)
- Week 1: Derive macro integration (6-8 hours)
- Release: End of Week 1

### 0.2.5 (3-4 hours)
- Week 2: Field-level overrides
- Release: Mid Week 2

### 0.2.6 (4-6 hours)
- Week 2: Datetime styles refactor (2-3 hours)
- Week 2: Documentation & examples (2-3 hours)
- Release: End of Week 2

**Total: 2-3 weeks for complete style system**

## Conclusion

The elicitation style system provides:

- **Flexibility:** Users control UX without reimplementing elicitation
- **Extensibility:** Ecosystem can create specialized styles
- **Compatibility:** Backward compatible, opt-in feature
- **Quality:** Built-in styles cover common patterns
- **Innovation:** Opens door to framework integration, domain-specific styles

This is a **foundational feature** that will enable ecosystem growth and make elicitation useful in contexts beyond terminal prompts (GUI, web, config files, etc.).

Ready to implement after serde_json (0.2.2) and datetime (0.2.3) features land.
