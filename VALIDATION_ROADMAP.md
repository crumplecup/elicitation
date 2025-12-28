# Custom Validation & Interactive Features Roadmap

## Overview

This document explores validation strategies and interactive features for the elicitation library, distinguishing between:
1. **Attribute-based validation** - Declarative rules checked after elicitation
2. **Interactive validation** - Real-time feedback during elicitation  
3. **Context-aware suggestions** - Using system/environment state for autocomplete

---

## Design Philosophy: Where to Validate?

### Option A: Post-Elicitation Validation (Attribute-Based)
```rust
#[derive(Elicit)]
struct User {
    #[validate(email)]
    email: String,
    
    #[validate(range(min = 0, max = 120))]
    age: u8,
}
```

**Pros:**
- Clean separation: elicitation vs validation
- Works with existing MCP tools
- No MCP protocol changes needed
- Easy to implement in proc macro

**Cons:**
- User sees error after full input
- Requires retry loop
- Less interactive experience

---

### Option B: Interactive Validation (MCP-Native)
```rust
// MCP tool provides validation during elicitation
let email = String::elicit(&client)
    .with_validator(email_validator)
    .await?;
```

**Pros:**
- Real-time feedback
- Better UX (catch errors early)
- Leverages MCP capabilities

**Cons:**
- Requires MCP tool support for validation
- More complex implementation
- Depends on MCP client features

---

### Option C: Context-Aware Suggestions (Hybrid)
```rust
let path = PathBuf::elicit(&client)
    .with_suggestions(filesystem_provider)
    .await?;

// MCP tool could offer:
// - Autocomplete from actual filesystem
// - Only show valid directories
// - Recent paths from history
```

**Pros:**
- Best UX (like IDE autocomplete)
- Prevents errors before they happen
- Leverages system context

**Cons:**
- Requires sophisticated MCP tooling
- Privacy/security considerations
- Platform-specific implementations

---

## Validation Opportunities Analysis

### 1. PathBuf Validation

#### Current State
- Accepts any string
- No filesystem interaction
- No validation

#### Potential Enhancements

**A. Basic Validation (Post-elicitation)**
```rust
#[derive(Elicit)]
struct Config {
    #[validate(path_exists)]
    input_file: PathBuf,
    
    #[validate(is_directory)]
    output_dir: PathBuf,
    
    #[validate(is_writable)]
    log_file: PathBuf,
}
```

**B. Interactive Filesystem Navigation**
```rust
// MCP tool provides directory listing
let dir = PathBuf::elicit(&client)
    .with_filesystem_browser()
    .await?;

// User sees:
// 📁 Documents/
// 📁 Downloads/
// 📁 Projects/
// 📄 notes.txt
```

**C. Path Autocomplete**
- Tab completion like bash
- Real-time directory suggestions
- Recent paths history

**Recommendation:** Start with (A), design for (B/C) as MCP evolves.

---

### 2. Network Address Validation

#### Current State
- Parse validation (correct format)
- No network connectivity checks

#### Potential Enhancements

**A. Reachability Validation**
```rust
#[validate(is_reachable)]
host: IpAddr,

#[validate(port_open)]
server: SocketAddr,
```

**Issues:**
- Network calls during validation = slow
- Security/privacy concerns
- May not reflect actual runtime conditions

**B. Common Address Suggestions**
```rust
// Suggest common development addresses
// - localhost (127.0.0.1)
// - Any (0.0.0.0)
// - Recent connections
```

**Recommendation:** Skip reachability (too slow/unreliable). Consider suggestions for common cases.

---

### 3. Email/URL Validation

#### Current State
- No validation (just String)

#### Potential Enhancements

**A. Format Validation**
```rust
#[validate(email)]
email: String,

#[validate(url)]
website: String,
```

**Implementation:**
- Regex patterns
- Use `email_address` or `url` crates
- Validate format, not deliverability

**Recommendation:** YES - High value, low complexity, no external calls.

---

### 4. Numeric Range Validation

#### Current State
- Type-level constraints only (u8 = 0-255)

#### Potential Enhancements

**A. Custom Ranges**
```rust
#[validate(range(min = 0, max = 120))]
age: u8,

#[validate(range(min = 0.0, max = 100.0))]
percentage: f64,
```

**B. Interactive Range Selection**
```rust
// MCP tool could show slider
// Or suggest common values
```

**Recommendation:** YES for (A) - Common need, easy to implement.

---

### 5. String Content Validation

#### Current State
- No constraints

#### Potential Enhancements

**A. Length Constraints**
```rust
#[validate(min_length = 8, max_length = 64)]
password: String,

#[validate(not_empty)]
username: String,
```

**B. Pattern Matching**
```rust
#[validate(regex = r"^[a-zA-Z0-9_]+$")]
identifier: String,

#[validate(alphanumeric)]
code: String,
```

**Recommendation:** YES - Very common validation needs.

---

### 6. Collection Constraints

#### Current State
- No size constraints

#### Potential Enhancements

**A. Size Limits**
```rust
#[validate(min_length = 1, max_length = 10)]
items: Vec<String>,

#[validate(unique)]
tags: Vec<String>,
```

**B. Element Validation**
```rust
// Apply validation to each element
#[validate(each(email))]
recipients: Vec<String>,
```

**Recommendation:** YES - Practical and implementable.

---

## Proposed Implementation Strategy

### Phase 1: Attribute-Based Validation (v0.4.0)
**Focus:** Post-elicitation validation via proc macro attributes

**Implementation:**
1. Add `#[validate(...)]` attribute parsing to proc macro
2. Generate validation code in derived `elicit()` method
3. Validation happens after inner type elicitation
4. Return ElicitError with validation details on failure

**Validators to implement:**
- `email` - Email format validation
- `url` - URL format validation  
- `range(min, max)` - Numeric range
- `min_length`, `max_length` - String length
- `not_empty` - Non-empty strings
- `regex(pattern)` - Regex matching
- `path_exists` - Path exists on filesystem
- `is_file`, `is_directory` - Path type checks
- `min_items`, `max_items` - Collection size
- `unique` - No duplicate items in collection

**Architecture:**
```rust
// In derive macro
impl Elicitation for User {
    async fn elicit(...) -> ElicitResult<Self> {
        let email = String::elicit(client).await?;
        
        // Generated validation code
        if !validate_email(&email) {
            return Err(ElicitError::validation(
                "email",
                "Invalid email format"
            ));
        }
        
        // ... rest of fields
    }
}
```

---

### Phase 2: Interactive Validators (v0.5.0)
**Focus:** Builder pattern for runtime validation

**API Design:**
```rust
use elicitation::validators::*;

let email = String::elicit(&client)
    .with_validator(email())
    .await?;

let age = u8::elicit(&client)
    .with_validator(range(0..=120))
    .await?;

let path = PathBuf::elicit(&client)
    .with_validator(path_exists())
    .await?;
```

**Benefits:**
- More flexible than attributes
- Composable validators
- Custom validation logic

**Implementation:**
- Add `ElicitationBuilder` wrapper
- Chain validators
- Execute during elicitation

---

### Phase 3: Context-Aware Features (v0.6.0)
**Focus:** System integration for suggestions

**Features:**
1. **Filesystem Browser**
   ```rust
   let path = PathBuf::elicit(&client)
       .with_filesystem_context()
       .await?;
   ```

2. **History/Recents**
   ```rust
   let ip = IpAddr::elicit(&client)
       .with_recent_values()
       .await?;
   ```

3. **Environment Suggestions**
   ```rust
   let port = u16::elicit(&client)
       .suggest_available_ports()
       .await?;
   ```

**Dependencies:**
- Requires MCP tool enhancements
- May need new MCP protocol features
- Platform-specific code

---

## Recommendations

### Priority 1: DO NOW (v0.4.0)
✅ **Attribute-based validation** - High value, clear implementation path
- Email, URL, numeric ranges
- String constraints (length, patterns)
- Collection size constraints
- Basic path validation (exists, type)

### Priority 2: CONSIDER LATER (v0.5.0)
⚠️ **Builder-based validators** - Good for advanced use cases
- More flexible than attributes
- Better for custom logic
- May be overkill for simple cases

### Priority 3: DEFER (v0.6.0+)
❌ **Interactive/context-aware features** - Wait for MCP ecosystem
- Filesystem browsing
- Network reachability  
- Autocomplete/suggestions
- These need MCP protocol support
- Security/privacy considerations
- Platform-specific complications

---

## Open Questions

1. **Validation Errors:** Retry loop or fail-fast?
   - Should validation errors trigger re-elicitation?
   - Or return error to caller?

2. **Async Validators:** Support async validation?
   ```rust
   #[validate(async = check_database)]
   username: String,
   ```
   Probably not - keep validation synchronous.

3. **Custom Validators:** Allow user-defined validators?
   ```rust
   #[validate(custom = "my_validator")]
   field: String,
   ```
   Yes - but in Phase 2.

4. **Validation Messages:** Custom error messages?
   ```rust
   #[validate(email, message = "Please provide a valid email")]
   email: String,
   ```
   Yes - good UX improvement.

---

## Next Steps

1. Review this roadmap and decide on scope for v0.4.0
2. Design validator trait/interface
3. Implement proc macro attribute parsing
4. Add core validators (email, range, length)
5. Write comprehensive tests
6. Document validation patterns


---

## Ecosystem Analysis: validator vs elicitation

### Key Insight: Different Problem Domains

**validator crate** (Keats/validator):
- **Purpose**: Post-construction validation of existing data
- **Use case**: Web APIs, form submissions, deserialization
- **Pattern**: `data.validate()` after construction
- **Returns**: Validation errors for display

**elicitation** (our crate):
- **Purpose**: Guide interactive data collection
- **Use case**: CLIs, MCP tools, conversational interfaces
- **Pattern**: Validate during `.elicit()` construction
- **Returns**: Valid data or elicitation error

### Analogy
- **validator** = Bouncer checking IDs at entrance
- **elicitation** = GPS preventing wrong turns

### Decision: HYBRID APPROACH ✅

**Use validator for validation logic:**
```toml
[dependencies]
validator = "0.20"  # Validation functions only
```

**Write our own proc macro for elicitation:**
```rust
#[derive(Elicit)]  // Our derive macro
struct User {
    #[validate(email)]  // Our attribute, validator's function
    email: String,
}

// Generated code:
impl Elicitation for User {
    async fn elicit(...) -> ElicitResult<Self> {
        let email = String::elicit(client).await?;
        
        // Call validator's function
        if !validator::validate_email(&email) {
            return Err(ElicitError::validation(...));
        }
        
        Ok(User { email })
    }
}
```

### Benefits
1. ✅ Reuse battle-tested validation logic
2. ✅ Consistency with web ecosystem
3. ✅ Less code to maintain
4. ✅ Our proc macro tailored to elicitation
5. ✅ Best of both worlds

### What We Build
- Elicitation-specific proc macro
- Retry logic for failed validation
- MCP integration
- Conversational error messages

### What We Reuse
- Email/URL/phone validation
- Range/length validators
- Pattern matching
- Proven validation rules

### v0.4.0 Scope (Revised)

**Integrate validator crate:**
1. Add dependency on `validator` (not `validator_derive`)
2. Parse `#[validate(...)]` attributes in our proc macro
3. Generate code calling `validator::validate_*` functions
4. Add elicitation-specific validators:
   - `path_exists`, `is_file`, `is_directory`
   - Custom retry behavior
   - Conversational error messages

**Validators from ecosystem:**
- email, url, phone
- range (min/max for numbers)
- length (min/max for strings)
- regex patterns
- Custom predicates

**Elicitation-specific:**
- Filesystem validation
- Retry loops on failure
- Context-aware error messages
- MCP integration

