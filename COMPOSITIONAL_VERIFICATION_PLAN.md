# Compositional Verification Implementation Plan

## Vision Statement

**All types that implement `Elicitation` are automatically formally verifiable.**

Since we've already proven all stdlib types (i8-i128, String, Vec, etc.) and users compose their types from these, any type that derives `Elicit` gets formal verification proofs **for free**. Users choose their verification backend (Kani, Creusot, Verus, Prusti) via feature flags—no additional derives needed.

**Core insight:** Elicitation guarantees "this type can be safely obtained from vague inputs." Formal verification proves "this composition preserves those guarantees." These are the same property, just expressed at compile-time vs. runtime.

---

## 1. Current State Analysis

### What We Have ✅

**404 proofs across 4 verifiers (Kani, Verus, Creusot, Prusti):**
- 72+ contract types (I8Positive, StringNonEmpty, etc.)
- Each has constructor proofs: `fn new(T) -> Result<ContractType, ValidationError>`
- All stdlib types covered (i8-i128, f32/f64, String, bool, char, etc.)
- Container types (Vec, HashMap, Option, Result, etc.)
- Third-party types (chrono, jiff, time, uuid, url, regex)

**Verification architecture:**
```rust
// Example: I8Positive contract type
#[derive(Debug, Clone)]
pub struct I8Positive(i8);

impl I8Positive {
    #[kani::requires(value > 0)]
    #[kani::ensures(|result: &Result<Self, _>| 
        result.as_ref().map(|v| v.0 > 0).unwrap_or(true)
    )]
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err(ValidationError::NotPositive)
        }
    }
    
    pub fn get(&self) -> i8 { self.0 }
    pub fn into_inner(self) -> i8 { self.0 }
}

// Kani proof
#[cfg(kani)]
#[kani::proof]
fn verify_i8_positive_new() {
    let value: i8 = kani::any();
    if let Ok(pos) = I8Positive::new(value) {
        assert!(pos.get() > 0);
    }
}
```

**Trenchcoat pattern:**
- Contract types wrap stdlib types at elicitation boundary
- Validation happens on entry (via `new()`)
- Extraction happens on exit (via `get()`/`into_inner()`)
- Guarantees: `StringNonEmpty.into_inner()` is ALWAYS non-empty

### What We're Missing ❌

**No composition for user types:**
```rust
// User wants to write this:
#[derive(Elicit)]
pub struct UserProfile {
    name: String,      // ❌ Not verified
    age: u8,           // ❌ Not verified
    tags: Vec<String>, // ❌ Not verified
}

// But they can't prove that UserProfile is valid
// even though we've proven String, u8, Vec<T> are valid
```

**Gap:** User structs don't benefit from our existing proofs.

---

## 2. Proposed Solution: Verification Built Into `#[derive(Elicit)]`

### 2.1 User-Facing API

```rust
use elicitation::Elicit;

// User writes ONLY this:
#[derive(Elicit)]
pub struct UserProfile {
    name: String,      // String impls Elicit → automatically has Kani/Creusot/Verus/Prusti proofs
    age: u8,           // u8 impls Elicit → automatically has proofs
    tags: Vec<String>, // Vec<T> where T: Elicit → automatically has proofs
}

// That's it! Verification contracts generated automatically.
// Choose verifier via feature flag:
//   cargo kani --features verify-kani
//   cargo creusot --features verify-creusot
// etc.
```

**No separate `#[derive(Verifiable)]` needed.** If you can elicit it, you can verify it.

**What gets generated (automatically by `#[derive(Elicit)]`):**

```rust
// Generated constructor with composed contracts
#[cfg(feature = "verify-kani")]
#[cfg(kani)]
#[kani::requires(
    // Compose requirements from all fields that impl Elicit
    String::elicit_requires(&name) &&
    u8::elicit_requires(&age) &&
    Vec::<String>::elicit_requires(&tags)
)]
#[kani::ensures(|result: &UserProfile| 
    // All fields satisfy their Elicit contracts
    String::elicit_ensures(&result.name) &&
    u8::elicit_ensures(&result.age) &&
    Vec::<String>::elicit_ensures(&result.tags)
)]
fn __make_UserProfile(
    name: String,
    age: u8,
    tags: Vec<String>,
) -> UserProfile {
    UserProfile { name, age, tags }
}

// Generated harness with stubbed leaf proofs
#[cfg(feature = "verify-kani")]
#[cfg(kani)]
#[kani::proof_for_contract(__make_UserProfile)]
#[kani::stub_verified(String::elicit_verify)]
#[kani::stub_verified(u8::elicit_verify)]
#[kani::stub_verified(Vec::<String>::elicit_verify)]
fn __verify_UserProfile_composition() {
    let name: String = kani::any();
    let age: u8 = kani::any();
    let tags: Vec<String> = kani::any();
    let profile = __make_UserProfile(name, age, tags);
    
    // Composed invariants automatically hold
    // because all leaf types already proven
}
```

**Key difference from earlier plan:** User types use **stdlib types directly** (String, u8, Vec), not contract types (StringNonEmpty, U8, VecNonEmpty). The contract validation happens at the **elicitation boundary**, not in the type system.

### 2.2 Optional Customization

```rust
#[derive(Elicit)]
#[elicit_verify(
    requires = "age >= 18",      // Additional constraint
    ensures = "result.is_adult()" // Additional guarantee
)]
pub struct AdultProfile {
    name: String,
    age: u8,
}

impl AdultProfile {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}
```

**Note:** Verification is **not a separate derive**, it's built into `#[derive(Elicit)]`.

### 2.3 Enum Support

```rust
#[derive(Elicit)]
pub enum PaymentMethod {
    CreditCard {
        number: String,     // Verified via Elicit proofs
        cvv: String,        // Verified via Elicit proofs + field constraint
    },
    BankTransfer {
        account: String,    // Verified via Elicit proofs
        routing: String,    // Verified via Elicit proofs + field constraint
    },
}
```

**Automatic verification for ALL variants** because all fields implement `Elicit`.

**Generated:**
```rust
#[cfg(kani)]
#[kani::requires(
    match self {
        Self::CreditCard { number, cvv } => 
            number.get().len() > 0 && cvv.get().len() == 3,
        Self::BankTransfer { account, routing } =>
            account.get().len() > 0 && routing.get().len() == 9,
    }
)]
fn __verify_PaymentMethod_variants() {
    // Per-variant verification
}
```

---

## 3. Implementation Phases

### Phase 1: Extend Elicit Derive for Verification (Week 1)

**Goal:** Add verification contract generation to existing `#[derive(Elicit)]` macro

**Tasks:**
1. Extend `elicitation_derive` crate (don't create new crate)
2. Add verification codegen behind feature flags
3. Implement basic struct verification contract generation
4. Generate stub constructors with contracts
5. Add feature flags: `verify-kani`, `verify-creusot`, etc.

**Files (modify existing):**
- `crates/elicitation_derive/Cargo.toml` - add feature flags
- `crates/elicitation_derive/src/lib.rs` - add verification module
- `crates/elicitation_derive/src/verification/mod.rs` - NEW
- `crates/elicitation_derive/src/verification/kani.rs` - NEW
- `crates/elicitation_derive/src/verification/contracts.rs` - NEW

**Success criteria:**
- Macro compiles with new feature flags
- Basic struct expansion includes verification contracts when enabled
- No verification contracts when features disabled (zero cost)

### Phase 2: Contract Introspection (Week 2)

**Goal:** Extract contract requirements from field types

**Challenge:** How do we know `StringNonEmpty` has requirement `len > 0`?

**Solution A: Convention-based (MVP)**
```rust
// Naming convention: ContractType has Contract trait impl
trait ContractRequirements {
    fn requires_expr() -> &'static str;
    fn ensures_expr() -> &'static str;
}

impl ContractRequirements for StringNonEmpty {
    fn requires_expr() -> &'static str {
        "value.len() > 0"
    }
    fn ensures_expr() -> &'static str {
        "result.get().len() > 0"
    }
}
```

**Solution B: Attribute-based (better)**
```rust
#[contract_type(
    requires = "value.len() > 0",
    ensures = "result.get().len() > 0"
)]
pub struct StringNonEmpty(String);
```

**Implementation:**
1. Add `#[contract_type]` attribute macro
2. Store metadata at compile time (proc-macro-hack or inventory)
3. Derive macro queries metadata for each field type
4. Compose into struct-level requirements

**Tasks:**
- Implement `#[contract_type]` attribute macro
- Add to all 72+ contract types
- Test metadata extraction
- Generate composed `requires` clauses

**Success criteria:**
- Can extract requirements from field types
- Generate correct `#[kani::requires]` for structs

### Phase 3: Kani Contract Generation (Week 3)

**Goal:** Full Kani verification for derived structs

**Tasks:**
1. Generate `#[kani::requires]` from field requirements (AND composition)
2. Generate `#[kani::ensures]` from field guarantees
3. Generate constructor function
4. Generate proof harness with `#[kani::proof_for_contract]`
5. Add `#[kani::stub_verified]` for all field constructors
6. Handle nested containers (Vec<ContractType>, Option<ContractType>)

**Example output:**
```rust
// For struct with 3 fields, generate:

#[cfg(kani)]
#[kani::requires(field1.invariant() && field2.invariant() && field3.invariant())]
#[kani::ensures(|r: &MyStruct| r.field1.invariant() && r.field2.invariant() && r.field3.invariant())]
fn __make_MyStruct(f1: Type1, f2: Type2, f3: Type3) -> MyStruct {
    MyStruct { field1: f1, field2: f2, field3: f3 }
}

#[cfg(kani)]
#[kani::proof_for_contract(__make_MyStruct)]
#[kani::stub_verified(Type1::new)]
#[kani::stub_verified(Type2::new)]
#[kani::stub_verified(Type3::new)]
fn __verify_MyStruct() {
    let f1: Type1 = kani::any();
    let f2: Type2 = kani::any();
    let f3: Type3 = kani::any();
    let _ = __make_MyStruct(f1, f2, f3);
}
```

**Success criteria:**
- Generate valid Kani contracts
- Proofs verify in CI (fast, <1s per struct via stubbing)
- All tests passing

### Phase 4: Enum Support (Week 4)

**Goal:** Handle sum types (OR composition)

**Tasks:**
1. Parse enum variants and fields
2. Generate per-variant requirements (OR composition)
3. Generate constructor for each variant
4. Generate harness covering all variants
5. Handle unit variants, tuple variants, struct variants

**Example:**
```rust
#[derive(Verifiable)]
enum Status {
    Active { since: DateTimeNonEmpty },
    Inactive,
    Suspended { reason: StringNonEmpty },
}

// Generates:
#[kani::requires(
    matches!(self, 
        Self::Active { since } if since.is_valid() |
        Self::Inactive |
        Self::Suspended { reason } if reason.get().len() > 0
    )
)]
```

**Success criteria:**
- Enum verification works
- OR composition correct
- All variant paths covered

### Phase 5: Verifier Adapters (Week 5-8)

**Goal:** Support Creusot, Verus, Prusti

**Strategy:** Adapter pattern
```rust
// Internal trait for verifier backends
trait VerifierBackend {
    fn emit_requires(&self, fields: &[Field]) -> TokenStream;
    fn emit_ensures(&self, fields: &[Field]) -> TokenStream;
    fn emit_constructor(&self, name: &Ident, fields: &[Field]) -> TokenStream;
    fn emit_harness(&self, name: &Ident, fields: &[Field]) -> TokenStream;
}

struct KaniBackend;
struct CreusotBackend;
struct VerusBackend;
struct PrustiBackend;

// Select backend based on feature flags
#[cfg(feature = "verify-kani")]
const BACKEND: KaniBackend = KaniBackend;

#[cfg(feature = "verify-creusot")]
const BACKEND: CreusotBackend = CreusotBackend;
```

**Tasks per verifier:**
1. Research verifier syntax (requires/ensures/proof format)
2. Implement backend adapter
3. Test with existing proofs
4. Document usage (flags, CI setup)

**Success criteria:**
- All 4 verifiers supported
- User can switch via feature flags
- Verification time remains <1s per struct

---

## 4. Technical Design Details

### 4.1 Contract Metadata Storage

**Problem:** Proc macros can't share state between invocations.

**Solution:** Use type-level encoding + helper trait

```rust
// In each contract type file:
pub trait ContractMetadata {
    const REQUIRES: &'static str;
    const ENSURES: &'static str;
}

impl ContractMetadata for StringNonEmpty {
    const REQUIRES: &'static str = "value.len() > 0";
    const ENSURES: &'static str = "result.get().len() > 0";
}

// Derive macro uses trait bounds to access metadata:
fn derive_verifiable(input: DeriveInput) -> TokenStream {
    let fields = extract_fields(&input);
    for field in fields {
        let ty = &field.ty;
        // Generate: <#ty as ContractMetadata>::REQUIRES
    }
}
```

### 4.2 Container Lifting

**Problem:** How to express `Vec<StringNonEmpty>` requirements?

**Solution:** Recursive lifting with helpers

```rust
// Helper trait for container types
pub trait ContractContainer {
    type Element;
    
    fn all_valid(&self) -> bool
    where
        Self::Element: ContractMetadata;
}

impl<T> ContractContainer for Vec<T> {
    type Element = T;
    
    fn all_valid(&self) -> bool
    where
        T: ContractMetadata
    {
        self.iter().all(|elem| elem.is_valid())
    }
}

// Generated requires for Vec<StringNonEmpty>:
#[kani::requires(tags.all_valid())]
```

### 4.3 Handling Generics

**Problem:** User structs might be generic

```rust
#[derive(Verifiable)]
pub struct Container<T> {
    value: T,
}
```

**Solution:** Bound generic params with ContractMetadata

```rust
// Generated:
impl<T> Container<T>
where
    T: ContractMetadata
{
    #[kani::requires(value.is_valid())]
    fn __make_Container(value: T) -> Self {
        Self { value }
    }
}

// Verification only when instantiated with contract type:
let c = Container::<StringNonEmpty>::new(s);
```

### 4.4 Performance: Stubbing Strategy

**Key insight:** Leaf proofs already verified, so stub them.

```rust
// WITHOUT stubbing (slow, may not terminate):
#[kani::proof_for_contract(__make_Profile)]
fn verify_profile() {
    let name = StringNonEmpty::new(kani::any()).unwrap(); // ❌ Verifies StringNonEmpty AGAIN
    let age = U8::new(kani::any()).unwrap();              // ❌ Verifies U8 AGAIN
    // ...
}

// WITH stubbing (fast, <1s):
#[kani::proof_for_contract(__make_Profile)]
#[kani::stub_verified(StringNonEmpty::new)]  // ✅ Assume already verified
#[kani::stub_verified(U8::new)]              // ✅ Assume already verified
fn verify_profile() {
    let name: StringNonEmpty = kani::any();  // ✅ Just assume valid
    let age: U8 = kani::any();               // ✅ Just assume valid
    let profile = __make_Profile(name, age);
    // Only verifying composition, not leaves
}
```

**Implementation:**
- Derive macro emits `#[kani::stub_verified]` for all field constructors
- Applies recursively to nested types
- Result: Verification time linear in struct field count, not proof complexity

---

## 5. Why This Works: Elicitation IS Verification

### 5.1 The Connection

**Elicitation says:** "I can obtain a valid T from vague inputs"
**Verification says:** "I can prove T is valid"

**These are the same property!**

```rust
pub trait Elicitation {
    type Style: ElicitationStyle;
    
    // Runtime: obtain valid value
    async fn elicit(client: &ElicitClient<'_>) -> ElicitResult<Self>;
    
    // Compile-time: prove value is valid (GENERATED AUTOMATICALLY)
    #[cfg(feature = "verify-kani")]
    fn elicit_requires(input: &Self) -> bool;
    
    #[cfg(feature = "verify-kani")]
    fn elicit_ensures(output: &Self) -> bool;
}
```

### 5.2 How It Works In Practice

**Step 1: We prove all stdlib types**
```rust
// We've already done this (404 proofs):
impl Elicitation for String { /* ... */ }
impl Elicitation for u8 { /* ... */ }
impl<T: Elicitation> Elicitation for Vec<T> { /* ... */ }
// etc. for all types
```

**Step 2: User composes types**
```rust
#[derive(Elicit)]
pub struct UserProfile {
    name: String,      // ✅ String::elicit_requires/ensures exist
    age: u8,           // ✅ u8::elicit_requires/ensures exist
    tags: Vec<String>, // ✅ Vec<String>::elicit_requires/ensures exist
}
```

**Step 3: Derive automatically composes proofs**
```rust
// Generated by #[derive(Elicit)] when verify-kani feature enabled:
#[cfg(feature = "verify-kani")]
impl UserProfile {
    fn elicit_requires(&self) -> bool {
        String::elicit_requires(&self.name) &&
        u8::elicit_requires(&self.age) &&
        Vec::<String>::elicit_requires(&self.tags)
    }
    
    fn elicit_ensures(&self) -> bool {
        String::elicit_ensures(&self.name) &&
        u8::elicit_ensures(&self.age) &&
        Vec::<String>::elicit_ensures(&self.tags)
    }
}

#[cfg(feature = "verify-kani")]
#[cfg(kani)]
#[kani::proof_for_contract(__make_UserProfile)]
#[kani::stub_verified(String::elicit_verify)]
#[kani::stub_verified(u8::elicit_verify)]
#[kani::stub_verified(Vec::<String>::elicit_verify)]
fn __verify_UserProfile() { /* ... */ }
```

**Step 4: User runs verification**
```bash
cargo kani --features verify-kani
# VERIFICATION SUCCESSFUL: UserProfile composition proven
```

### 5.3 Benefits

- **No separate derive needed:** One derive (`Elicit`) does both
- **Choose your verifier:** Feature flags select backend
- **Zero cost when disabled:** No verification code without features
- **Automatic composition:** If fields are elicitable, struct is verifiable
- **Same semantics:** Elicitation and verification express the same property

---

## 6. Testing Strategy

### 6.1 Macro Unit Tests

**Using `trybuild` for compile-fail tests:**

```rust
// tests/ui/pass/simple_struct.rs
use elicitation_verifiable::Verifiable;

#[derive(Verifiable)]
struct Simple {
    field: StringNonEmpty,
}

// Should compile and generate valid Kani proof

// tests/ui/fail/no_contract_type.rs
#[derive(Verifiable)]
struct Invalid {
    field: String,  // ❌ String doesn't impl ContractMetadata
}

// Should fail with helpful error
```

**Test coverage:**
- Simple structs (1-5 fields)
- Complex structs (10+ fields)
- Nested structs
- Generic structs
- Unit enums
- Tuple enums
- Struct enums
- Containers (Vec, Option, HashMap)
- Custom contract attributes

### 6.2 Integration Tests

**End-to-end verification tests:**

```rust
// tests/integration/user_types.rs

#[derive(Verifiable)]
struct TestStruct {
    name: StringNonEmpty,
    age: U8,
}

#[test]
fn verify_test_struct() {
    // Run Kani verification
    let output = Command::new("cargo")
        .args(&["kani", "--features", "verify-kani"])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout)
        .contains("VERIFICATION SUCCESSFUL"));
}
```

### 6.3 Performance Tests

**Benchmark verification time:**

```rust
#[derive(Verifiable)]
struct Small { f1: StringNonEmpty }  // Target: <0.5s

#[derive(Verifiable)]
struct Medium {
    f1: StringNonEmpty,
    f2: U8,
    f3: VecNonEmpty<StringNonEmpty>,
    f4: Option<StringNonEmpty>,
}  // Target: <1s

#[derive(Verifiable)]
struct Large {
    // 20 fields
}  // Target: <5s
```

---

## 7. Documentation Plan

### 7.1 User Guide

**`docs/VERIFIABLE_DERIVE.md`:**
- Quick start (5 minute example)
- Field type requirements
- Custom attributes
- Enum support
- Verifier selection
- CI integration
- Troubleshooting

### 7.2 API Documentation

**Rustdoc comments on:**
- `#[derive(Verifiable)]` attribute
- `#[verifiable(requires = "...", ensures = "...")]` options
- `ContractMetadata` trait (for custom contract types)
- `ContractContainer` trait (for custom containers)

### 7.3 Examples

**`examples/verification/`:**
- `simple_struct.rs` - Basic usage
- `nested_struct.rs` - Composition
- `enum_variants.rs` - Sum types
- `generic_struct.rs` - Generic types
- `custom_contract.rs` - User-defined contract type

---

## 8. CI Integration

### 8.1 GitHub Actions Workflow

```yaml
name: Verification

on: [push, pull_request]

jobs:
  verify-kani:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: model-checking/kani-github-action@v1
      - run: cargo kani --features verify-kani
      - run: cargo kani --features verify-kani tests/integration/
  
  verify-creusot:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Setup Creusot
        run: ./scripts/setup-creusot.sh
      - run: cargo build --features verify-creusot
      - run: ./scripts/verify-creusot.sh
  
  # Similar for Verus, Prusti
```

### 8.2 Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Quick verification check (Kani only, <30s)
cargo kani --features verify-kani || {
    echo "❌ Verification failed. Run 'cargo kani --features verify-kani' to debug."
    exit 1
}
```

---

## 9. Migration Path for Existing Code

### 9.1 Zero Migration Required!

**Before (existing code):**
```rust
#[derive(Elicit)]
pub struct User {
    name: String,
    age: u8,
}
```

**After (just enable feature):**
```bash
# Before: No verification
cargo build

# After: Automatic verification
cargo kani --features verify-kani
```

**That's it!** No code changes needed. Verification is automatically available for all existing types that derive `Elicit`.
### 9.2 Optional: Adding Field Constraints

**If you want stricter validation:**
```rust
#[derive(Elicit)]
#[elicit_verify(field_requires = "age >= 18")]
pub struct AdultUser {
    name: String,
    age: u8,
}
```

**But most users won't need this** - stdlib type guarantees are usually sufficient.
```

### 9.3 Rollout Timeline

- **v0.5.0:** Add verification to `#[derive(Elicit)]` behind feature flags (opt-in)
- **v0.6.0:** Encourage verification in CI (documentation, examples)
- **v0.7.0:** Recommend verification for all public types
- **v1.0.0:** Verification considered "standard practice" (still opt-in via features)

---

## 10. Success Metrics

### 10.1 Adoption Metrics

- **Target:** 50% of user-defined structs use `#[derive(Verifiable)]` within 6 months
- **Measure:** Count derives in dependent crates

### 10.2 Performance Metrics

- **Target:** <1s verification per struct (average)
- **Target:** <30s full verification suite in CI
- **Measure:** CI job duration, cargo kani reports

### 10.3 Quality Metrics

- **Target:** 0 verification failures in production
- **Target:** 95%+ contract coverage of user types
- **Measure:** Verification reports, runtime assertion counts

### 10.4 Developer Experience Metrics

- **Target:** <10 lines of code per verified struct
- **Target:** <5 minutes time-to-first-verification
- **Measure:** User surveys, documentation analytics

---

## 11. Risks & Mitigations

### 11.1 Technical Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Kani instability | High | Medium | Pin versions, feature gates |
| Proc macro complexity | Medium | High | Extensive testing, clear error messages |
| Verification time explosion | High | Medium | Aggressive stubbing, timeout limits |
| Generic edge cases | Medium | Medium | Comprehensive test suite, conservative bounds |

### 11.2 Adoption Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Too complex for users | High | Medium | Clear docs, examples, starter templates |
| Verifier setup friction | Medium | High | Setup scripts, Docker images, CI templates |
| Contract type migration cost | Medium | Low | Compatibility layer, gradual migration |

---

## 12. Future Enhancements (Post-MVP)

### 12.1 Advanced Features

- **Dependent types:** Field constraints based on other fields
  ```rust
  #[derive(Verifiable)]
  #[verifiable(requires = "end > start")]
  struct TimeRange {
      start: DateTime,
      end: DateTime,
  }
  ```

- **Refinement types:** Inline predicates
  ```rust
  #[derive(Verifiable)]
  struct User {
      #[verify(|age| *age >= 18)]
      age: U8,
  }
  ```

- **Effect tracking:** IO/mutation tracking in contracts
  ```rust
  #[derive(Verifiable)]
  #[verifiable(effects = "read_only")]
  struct Config { /* ... */ }
  ```

### 12.2 Tooling Enhancements

- **IDE integration:** IntelliJ/VSCode plugins for contract visualization
- **Contract explorer:** Web UI showing proof tree
- **Automated test generation:** Generate property tests from contracts
- **Mutation testing:** Verify proofs detect bugs

### 12.3 Ecosystem Integration

- **Serde integration:** Deserialize into verified types
  ```rust
  let user: User = serde_json::from_str(json)?; // Verified!
  ```

- **Database integration:** Diesel/SQLx with verified schema types
- **Web framework integration:** Actix/Axum with verified request types

---

## 13. Timeline

### Month 1: Foundation
- Week 1: Infrastructure setup
- Week 2: Contract introspection
- Week 3: Kani contract generation
- Week 4: Enum support

### Month 2: Multi-Verifier
- Week 5: Creusot adapter
- Week 6: Verus adapter
- Week 7: Prusti adapter
- Week 8: Integration testing

### Month 3: Polish
- Week 9: Documentation
- Week 10: Examples & guides
- Week 11: Performance optimization
- Week 12: Beta release

### Month 4+: Production
- User feedback incorporation
- Advanced features
- Ecosystem integrations

---

## 14. Open Questions

1. **Should we support runtime contract checking?**
   - Pro: Useful for debugging, external data validation
   - Con: Performance overhead, duplicates verification work
   - Decision: Yes, but `#[cfg(debug_assertions)]` only

2. **How to handle breaking changes in verifiers?**
   - Option A: Pin specific versions, warn on breaking changes
   - Option B: Support multiple verifier versions
   - Decision: TBD based on stability

3. **Should Elicit derive automatically imply Verifiable?**
   - Pro: Zero-friction verification for all elicited types
   - Con: May slow compilation, force verifier setup
   - Decision: Opt-in via feature flag: `verify-elicited`

4. **How to handle third-party types?**
   - Option A: Newtype wrappers with Verifiable
   - Option B: Blanket impls (risky)
   - Option C: Contrib crate with common impls
   - Decision: Option A (safest), Option C (convenience)

---

## 15. Conclusion

This plan provides a **concrete, achievable path** to compositional verification that:
- ✅ Leverages our existing 404 proofs
- ✅ Requires minimal user code (`#[derive(Verifiable)]`)
- ✅ Supports all 4 verifiers via feature flags
- ✅ Maintains fast verification (<1s per struct)
- ✅ Integrates naturally with existing Elicitation system
- ✅ Provides clear migration path

**Next steps:**
1. Review and approve plan
2. Set up `elicitation_verifiable` crate
3. Implement Phase 1 (infrastructure)
4. Iterate based on feedback

**Questions/feedback welcome!**
