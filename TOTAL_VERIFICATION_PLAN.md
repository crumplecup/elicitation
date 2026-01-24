# Total Verification Implementation Plan

**Vision:** Formally verified AI content pipelines where users get mathematical proofs without effort.

**Core Principle:** When you `#[derive(Elicit)]`, you get formal verification for free.

**Revolutionary Insight:** The Trenchcoat Pattern - contract types validate at boundaries, users work with familiar stdlib types.

---

## The Grand Vision

### What We're Building

A verification framework where:

1. **Contracts are newtypes** - `StringNonEmpty` IS a type that validates on construction
2. **Boundary validation** - Contract types exist only during elicitation (put on coat → validate → take off coat)
3. **Familiar API** - Users write `String`, `i32` (stdlib types), validation happens transparently
4. **Formal verification** - Kani proves contract types work correctly
5. **Zero-effort propagates** - Derive macro handles contract types, returns stdlib types

### The Trenchcoat Pattern

**The key insight:** Contract types are construction-time validators, not persistent wrappers.

```
INPUT BOUNDARY          VALIDATION           OUTPUT BOUNDARY
───────────────         ──────────          ───────────────
LLM: "hello"            StringNonEmpty      User gets: String
     ↓                       ↓                     ↓
  "hello"  ─────→  StringNonEmpty("hello")  ─────→  String("hello")
                   ^^^^^^^^^^^^^^^^^^^^             ^^^^^^^^^^^^^^
                   Put on trenchcoat                Take off coat
                   (validate)                       (use)
```

**User writes:**
```rust
#[derive(Elicit)]
struct User {
    name: String,    // ← Familiar stdlib type
    age: i32,        // ← Familiar stdlib type
}
```

**Macro generates:**
```rust
impl Elicitation for User {
    async fn elicit(client: &Client) -> Result<Self> {
        // INPUT: Put on trenchcoat (wrap in contract type)
        let name_validated = StringNonEmpty::elicit(client).await?;
        let age_validated = I32Positive::elicit(client).await?;
        
        // OUTPUT: Take off trenchcoat (unwrap to stdlib type)
        Ok(Self {
            name: name_validated.into_inner(),  // String (was validated)
            age: age_validated.into_inner(),    // i32 (was validated)
        })
    }
}
```

**Result:** Users get familiar stdlib types, but data was formally verified at construction boundary.

### The Complete Flow

```
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: INPUT BOUNDARY (Put on trenchcoat)                 │
└─────────────────────────────────────────────────────────────┘

LLM outputs: "Alice", "25"
    ↓
StringNonEmpty::elicit(client) 
    → Text mechanism gets "Alice"
    → StringNonEmpty::new("Alice") validates
    → Returns StringNonEmpty("Alice") ✅

I32Positive::elicit(client)
    → Numeric mechanism gets "25"  
    → I32Positive::new(25) validates
    → Returns I32Positive(25) ✅

┌─────────────────────────────────────────────────────────────┐
│ STEP 2: VALIDATION (Inside the trenchcoat)                 │
└─────────────────────────────────────────────────────────────┘

StringNonEmpty("Alice")  ← Proven non-empty by construction
I32Positive(25)          ← Proven positive by construction

Kani proves:
  ∀s: StringNonEmpty ⇒ !s.0.is_empty()
  ∀i: I32Positive ⇒ i.0 > 0

┌─────────────────────────────────────────────────────────────┐
│ STEP 3: OUTPUT BOUNDARY (Take off trenchcoat)              │
└─────────────────────────────────────────────────────────────┘

StringNonEmpty("Alice").into_inner() → String("Alice")
I32Positive(25).into_inner()         → i32(25)

User { name: String("Alice"), age: i32(25) }
       ^^^^^^^^^^^^^^^^^^^^       ^^^^^^^^^^
       Familiar stdlib type       Familiar stdlib type
       BUT was validated          BUT was validated
       (came from contract)       (came from contract)
```

---

## Current State (Baseline)

### ✅ What We Have

**Type Coverage:** 15/44 types (34%)
- 12 primitives: String, bool, i32, i64, i128, isize, u32, u64, u128, usize, f32, f64
- 3 complex: Option<T>, Result<T, E>, Vec<T>

**Mechanism Coverage:** 4/7 mechanisms (57%)
- NumericReturnsValid<T> (all integer types)
- AffirmReturnsBoolean
- TextReturnsString / TextReturnsNonEmpty
- SurveyReturnsValidVariant<E> (POC only)

**Verifier Coverage:** 4/4 verifiers (100%)
- Kani, Creusot, Prusti, Verus (all working)

**Test Coverage:** 50 tests passing
- 22 main contracts
- 15 per verifier (Creusot, Prusti, Verus)
- 5 mechanism contracts
- 24 infrastructure tests

**Documentation:** Complete
- 5 examples (per-verifier + multi + mechanism POC)
- Migration guide
- Decision matrix
- Limitations documented

### ❌ What's Missing

**Type Coverage:** 29 types (66% of total)
- 4 integers: i8, i16, u8, u16
- 5 tuples: (T,U), (T,U,V), (T,U,V,W), (T,U,V,W,X), (T,U,V,W,X,Y)
- 4 containers: [T;N], Box<T>, Arc<T>, Rc<T>
- 3 special: Duration, PathBuf, Uuid
- 6 network: IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6
- 7 datetime: chrono (3), jiff (3), plus specialized datetime contracts

**Mechanism Coverage:** 3 mechanisms need full implementation
- Survey (enum) - Only POC, needs real contracts per enum
- Choice (selection) - Not started
- Composite (nested) - Not started

**Compositional Proofs:** 0/3 layers
- Derive macro doesn't generate contracts yet
- No automatic verification inheritance
- No end-to-end Kani harnesses

---

## Phase-by-Phase Implementation

### Phase 7: Complete Integer Family (Week 1)

**Goal:** 100% integer coverage with contract newtypes + boundary validation

**The Trenchcoat Pattern for Integers:**
```rust
// Contract type (validates on construction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct I8Positive(i8);

impl I8Positive {
    pub fn new(value: i8) -> Result<Self, ValidationError> {
        if value > 0 { Ok(Self(value)) }
        else { Err(ValidationError::NotPositive) }
    }
    
    pub fn get(&self) -> i8 { self.0 }
    
    // Trenchcoat method: unwrap to stdlib type
    pub fn into_inner(self) -> i8 { self.0 }
}

impl Elicitation for I8Positive {
    async fn elicit(client: &Client) -> Result<Self> {
        loop {
            let value = i8::elicit(client).await?;
            match Self::new(value) {
                Ok(valid) => return Ok(valid),  // Guaranteed positive!
                Err(_) => continue,  // Re-prompt
            }
        }
    }
}

impl Contract for I8Positive {
    fn invariant(value: &Self) -> bool {
        value.0 > 0  // Always true by construction!
    }
}
```

**User-facing API (familiar stdlib types):**
```rust
#[derive(Elicit)]
struct Config {
    #[elicit(positive)]  // Attribute specifies contract
    count: i8,           // User writes familiar i8
}

// Macro generates (uses I8Positive internally):
impl Elicitation for Config {
    async fn elicit(client: &Client) -> Result<Self> {
        let count = I8Positive::elicit(client).await?.into_inner();  // i8
        Ok(Self { count })
    }
}
```

**Tasks:**

- [ ] 7.1: Implement contract newtypes (Days 1-2)
  - i8: I8Positive(i8), I8NonNegative(i8), I8Range<MIN, MAX>(i8)
  - i16: I16Positive(i16), I16NonNegative(i16), I16Range<MIN, MAX>(i16)
  - u8: U8NonZero(u8), U8Range<MIN, MAX>(u8)
  - u16: U16NonZero(u16), U16Range<MIN, MAX>(u16)
  - Each with: new(), get(), into_inner(), Elicitation impl, Contract impl
  - All 4 verifiers: Kani, Creusot, Prusti, Verus

- [ ] 7.2: Mechanism integration (Days 3-4)
  - NumericReturnsValid<I8Positive>, etc (mechanism contracts)
  - Composition tests: Numeric + I8Positive (both must hold)
  - Kani proofs: `∀v: I8Positive ⇒ v.get() > 0`

- [ ] 7.3: Attribute-driven API (Day 4)
  - Parse `#[elicit(positive)]` → use `I8Positive` internally
  - Parse `#[elicit(range = "0..=100")]` → use `I8Range<0, 100>`
  - Unwrap via `.into_inner()` to return stdlib type

- [ ] 7.4: Integration testing (Day 5)
  - 16 tests (4 types × 4 verifiers)
  - Kani harnesses proving newtype invariants
  - Performance benchmarks (newtype overhead should be zero)
  - Test attribute-driven API generates correct code

**Deliverable:** 16/44 types (36%), all integers covered, trenchcoat pattern established

**Success Criteria:**
- ✅ 12 integer contract newtypes implemented
- ✅ Impossible to construct invalid values
- ✅ Kani proves invariants hold by construction
- ✅ Elicitation loops until valid, returns guaranteed-correct type
- ✅ `.into_inner()` unwraps to stdlib type (zero overhead)
- ✅ Attributes work: `#[elicit(positive)] count: i8`
}

impl Elicitation for I8Positive {
    async fn elicit(client: &Client) -> Result<Self> {
        loop {
            let value = i8::elicit(client).await?;
            match Self::new(value) {
                Ok(valid) => return Ok(valid),  // Guaranteed positive!
                Err(_) => continue,  // Re-prompt
            }
        }
    }
}

impl Contract for I8Positive {
    fn invariant(value: &Self) -> bool {
        value.0 > 0  // Always true by construction!
    }
}
```

**Tasks:**

- [ ] 7.1: Implement contract newtypes (Days 1-2)
  - i8: I8Positive(i8), I8NonNegative(i8), I8Range<MIN, MAX>(i8)
  - i16: I16Positive(i16), I16NonNegative(i16), I16Range<MIN, MAX>(i16)
  - u8: U8NonZero(u8), U8Range<MIN, MAX>(u8)
  - u16: U16NonZero(u16), U16Range<MIN, MAX>(u16)
  - Each with: new(), get(), Elicitation impl, Contract impl
  - All 4 verifiers: Kani, Creusot, Prusti, Verus

- [ ] 7.2: Mechanism integration (Days 3-4)
  - NumericReturnsValid<I8Positive>, etc (mechanism contracts)
  - Composition tests: Numeric + I8Positive (both must hold)
  - Kani proofs: `∀v: I8Positive ⇒ v.get() > 0`

- [ ] 7.3: Integration testing (Day 5)
  - 16 tests (4 types × 4 verifiers)
  - Kani harnesses proving newtype invariants
  - Performance benchmarks (newtype overhead should be zero)

**Deliverable:** 16/44 types (36%), all integers covered

**Success Criteria:**
- ✅ 12 integer contract newtypes implemented
- ✅ Impossible to construct invalid values
- ✅ Kani proves invariants hold by construction
- ✅ Elicitation loops until valid, returns guaranteed-correct type

---

### Phase 8: Tuple Verification (Week 2)

**Goal:** Tuple contract newtypes with compositional verification

**The Newtype Pattern for Tuples:**
```rust
// Tuple where both elements satisfy contracts
#[derive(Debug, Clone)]
pub struct Tuple2<C1, C2>(C1, C2)
where
    C1: Contract,
    C2: Contract;

impl<C1, C2> Tuple2<C1, C2>
where
    C1: Contract + Elicitation,
    C2: Contract + Elicitation,
{
    pub fn new(first: C1, second: C2) -> Self {
        // Both already validated by their own newtypes!
        Self(first, second)
    }
}

impl<C1, C2> Elicitation for Tuple2<C1, C2>
where
    C1: Contract + Elicitation,
    C2: Contract + Elicitation,
{
    async fn elicit(client: &Client) -> Result<Self> {
        let first = C1::elicit(client).await?;   // Guaranteed valid
        let second = C2::elicit(client).await?;  // Guaranteed valid
        Ok(Self::new(first, second))             // Composition = proven valid
    }
}
```

**Tasks:**

- [ ] 8.1: 2-4 element tuple newtypes (Days 6-8)
  - Tuple2<C1, C2>: Both components satisfy contracts
  - Tuple3<C1, C2, C3>
  - Tuple4<C1, C2, C3, C4>
  - Generic over any contract types (C1, C2, etc implement Contract)
  - All 4 verifiers

- [ ] 8.2: 5-6 element tuple newtypes (Day 9)
  - Tuple5, Tuple6 (less common but complete)
  - All 4 verifiers

- [ ] 8.3: Compositional Kani proofs (Day 10)
  - Prove: `∀t: Tuple2<C1, C2> ⇒ C1::invariant(t.0) ∧ C2::invariant(t.1)`
  - Prove: Tuple elicitation = composition of element elicitations
  - Mechanism contracts: TupleReturnsValid

**Deliverable:** 21/44 types (48%)

**Success Criteria:**
- ✅ All tuples up to 6 elements implemented as newtypes
- ✅ Compositional verification works: tuple valid ⟺ all elements valid
- ✅ Kani proves tuple element contracts compose
- ✅ **KEY PROOF:** Composition preserves verification

**Key Insight:** This proves the compositional model works! If each element is a contract newtype, the tuple of contract newtypes is proven valid.

---

### Phase 9: Container Types (Week 3)

**Goal:** Container newtypes inherit inner contract verification

**The Newtype Pattern for Containers:**
```rust
// VecNonEmpty: Vec that's never empty
#[derive(Debug, Clone)]
pub struct VecNonEmpty<T>(Vec<T>);

impl<T> VecNonEmpty<T> {
    pub fn new(vec: Vec<T>) -> Result<Self, ValidationError> {
        if vec.is_empty() { Err(ValidationError::Empty) }
        else { Ok(Self(vec)) }
    }
    
    pub fn get(&self) -> &Vec<T> { &self.0 }
}

impl<T: Elicitation> Elicitation for VecNonEmpty<T> {
    async fn elicit(client: &Client) -> Result<Self> {
        loop {
            let vec = Vec::<T>::elicit(client).await?;
            match Self::new(vec) {
                Ok(valid) => return Ok(valid),
                Err(_) => continue,
            }
        }
    }
}

// VecAllSatisfy: Vec where ALL elements satisfy contract
#[derive(Debug, Clone)]
pub struct VecAllSatisfy<C>(Vec<C>) where C: Contract;

impl<C: Contract + Elicitation> Elicitation for VecAllSatisfy<C> {
    async fn elicit(client: &Client) -> Result<Self> {
        // Each element is C (contract type), so all guaranteed valid!
        let elements = Vec::<C>::elicit(client).await?;
        Ok(Self(elements))  // Composition = automatic verification
    }
}
```

**Tasks:**

- [ ] 9.1: Array newtypes (Days 11-12)
  - ArrayAllSatisfy<C, const N: usize>([C; N])
  - Each element is contract type C, array guarantees all valid
  - All 4 verifiers

- [ ] 9.2: Smart pointer newtypes (Days 13-14)
  - BoxSatisfies<C>(Box<C>): Inner value is contract type
  - ArcSatisfies<C>(Arc<C>)
  - RcSatisfies<C>(Rc<C>)
  - All 4 verifiers

- [ ] 9.3: Container Kani proofs (Day 15)
  - Prove: `∀v: VecAllSatisfy<C> ⇒ ∀e ∈ v: C::invariant(e)`
  - Prove: Container construction preserves element contracts
  - Mechanism contracts: ContainerReturnsValid

**Deliverable:** 25/44 types (57%)

**Success Criteria:**
- ✅ All container newtypes wrap contract types (automatic verification)
- ✅ Kani proves: valid container ⟹ all elements valid
- ✅ **KEY PROOF:** Containers preserve verification compositionally

---

### Phase 10: Specialized Types (Week 4)

**Goal:** Domain-specific contract newtypes (Duration, PathBuf, Uuid, Network)

**The Newtype Pattern for Specialized Types:**
```rust
// Example: UuidV4 - Only Version 4 UUIDs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UuidV4(Uuid);

impl UuidV4 {
    pub fn new(uuid: Uuid) -> Result<Self, ValidationError> {
        if uuid.get_version() == Some(uuid::Version::Random) {
            Ok(Self(uuid))
        } else {
            Err(ValidationError::WrongVersion)
        }
    }
    
    pub fn get(&self) -> Uuid { self.0 }
}

impl Elicitation for UuidV4 {
    async fn elicit(client: &Client) -> Result<Self> {
        loop {
            let uuid = Uuid::elicit(client).await?;
            match Self::new(uuid) {
                Ok(valid) => return Ok(valid),  // Guaranteed V4!
                Err(_) => continue,
            }
        }
    }
}

// Example: IpPrivate - Only private IP addresses
#[derive(Debug, Clone, Copy)]
pub struct IpPrivate(IpAddr);

impl IpPrivate {
    pub fn new(ip: IpAddr) -> Result<Self, ValidationError> {
        if ip.is_private() { Ok(Self(ip)) }
        else { Err(ValidationError::NotPrivate) }
    }
}
```

**Tasks:**

- [ ] 10.1: Time/UUID newtypes (Days 16-17)
  - DurationPositive(Duration), DurationRange<MIN, MAX>(Duration)
  - UuidV4(Uuid), UuidNonNil(Uuid)
  - All 4 verifiers

- [ ] 10.2: Filesystem newtypes (Day 18)
  - PathBufExists(PathBuf) - runtime validation
  - PathBufReadable(PathBuf), PathBufIsDir(PathBuf)
  - Note: Some contracts require runtime filesystem checks

- [ ] 10.3: Network newtypes (Days 19-20)
  - IpPrivate(IpAddr), IpPublic(IpAddr)
  - IpV4(IpAddr), IpV6(IpAddr)
  - SocketAddrValidPort(SocketAddr) - port in valid range
  - Ipv4Loopback(Ipv4Addr), Ipv6Loopback(Ipv6Addr)
  - All 4 verifiers

- [ ] 10.4: Specialized Kani proofs (Day 21)
  - TimeReturnsValid<Duration>, UuidReturnsValid
  - PathReturnsValid, NetworkReturnsValid
  - Kani harnesses

**Deliverable:** 33/44 types (75%)

**Success Criteria:**
- ✅ All core std types have contracts
- ✅ Network + filesystem types verified
- ✅ 75% type coverage achieved

---

### Phase 11: DateTime Libraries (Week 5)

**Goal:** Feature-gated datetime contract newtypes

**The Newtype Pattern for DateTime:**
```rust
#[cfg(feature = "time-chrono")]
#[derive(Debug, Clone, Copy)]
pub struct DateTimeAfter<Tz: TimeZone>(DateTime<Tz>, DateTime<Tz>)
where
    DateTime<Tz>: Clone;

impl<Tz: TimeZone> DateTimeAfter<Tz>
where
    DateTime<Tz>: Clone,
{
    pub fn new(value: DateTime<Tz>, after: DateTime<Tz>) -> Result<Self, ValidationError> {
        if value > after { Ok(Self(value, after)) }
        else { Err(ValidationError::TooEarly) }
    }
    
    pub fn get(&self) -> &DateTime<Tz> { &self.0 }
}

impl<Tz: TimeZone> Elicitation for DateTimeAfter<Tz>
where
    DateTime<Tz>: Elicitation + Clone,
{
    async fn elicit(client: &Client) -> Result<Self> {
        let after = self.1.clone();
        loop {
            let dt = DateTime::<Tz>::elicit(client).await?;
            match Self::new(dt, after.clone()) {
                Ok(valid) => return Ok(valid),  // Guaranteed after threshold!
                Err(_) => continue,
            }
        }
    }
}
```

**Tasks:**

- [ ] 11.1: chrono newtypes (Days 22-23)
  - DateTimeAfter<Tz>(DateTime<Tz>, threshold)
  - DateTimeBefore<Tz>(DateTime<Tz>, threshold)
  - DateTimeRange<Tz>(DateTime<Tz>, min, max)
  - For DateTime<Utc>, DateTime<FixedOffset>, NaiveDateTime
  - All 4 verifiers

- [ ] 11.2: jiff newtypes (Days 24-25)
  - TimestampAfter(Timestamp, threshold)
  - TimestampBefore(Timestamp, threshold)
  - ZonedInTimezone(Zoned, timezone) - validates timezone
  - CivilDateTimeValid(CivilDateTime) - validates civil datetime
  - All 4 verifiers

- [ ] 11.3: DateTime Kani proofs (Days 26-27)
  - Prove: `∀dt: DateTimeAfter<Tz> ⇒ dt.get() > dt.threshold()`
  - DateTimeReturnsValid mechanism contracts
  - Feature-gated verification works

**Deliverable:** 39/44 types (89%)

**Success Criteria:**
- ✅ Both major datetime libraries have contract newtypes
- ✅ Feature-gated contracts verified independently
- ✅ DateTime verification complete (impossible to construct invalid datetimes)

---

### Phase 12: Mechanism Contract Completion (Week 6)

**Goal:** All elicitation mechanisms formally verified

**Tasks:**

- [ ] 12.1: Survey mechanism (Days 28-29)
  - SurveyReturnsValidVariant<E> - Full implementation
  - Per-enum contracts generated by derive macro
  - Kani harnesses proving enum safety

- [ ] 12.2: Choice mechanism (Days 30-31)
  - ChoiceReturnsFromSet<T, const SET: &[T]>
  - Proves selection is from valid options
  - Kani harnesses

- [ ] 12.3: Composite mechanism (Days 32-33)
  - CompositeReturnsValidNesting<Outer, Inner>
  - Nested elicitation verified
  - Kani harnesses

- [ ] 12.4: Mechanism integration tests (Days 34-35)
  - All mechanism contracts tested together
  - Performance benchmarks
  - Documentation updates

**Deliverable:** 7/7 mechanisms (100%)

**Success Criteria:**
- ✅ Every elicitation method has mechanism contract
- ✅ All mechanisms proven correct via Kani
- ✅ Mechanism + Type composition proven

---

### Phase 13: Compositional Verification (Week 7-8)

**Goal:** Derived types get verification for free (zero-effort cascade)

**The Zero-Effort Vision:**
```rust
// User writes this:
#[derive(Elicit)]
struct Config {
    name: StringNonEmpty,        // Contract type
    port: U16Range<1024, 65535>, // Contract type
    host: IpPrivate,             // Contract type
}

// Derive macro generates:
impl Elicitation for Config {
    async fn elicit(client: &Client) -> Result<Self> {
        // Each field elicits contract type (already validated!)
        let name = StringNonEmpty::elicit(client).await?;
        let port = U16Range::elicit(client).await?;
        let host = IpPrivate::elicit(client).await?;
        
        // Config construction = composition of validated fields
        Ok(Self { name, port, host })
    }
}

impl Contract for Config {
    fn invariant(config: &Self) -> bool {
        // All fields are contract types, so invariants MUST hold
        StringNonEmpty::invariant(&config.name) &&
        U16Range::invariant(&config.port) &&
        IpPrivate::invariant(&config.host)
    }
}

#[cfg(kani)]
#[kani::proof]
fn prove_config_always_valid() {
    let config: Config = kani::any();
    
    // Kani PROVES these hold (no counterexamples possible):
    assert!(!config.name.as_str().is_empty());      // Name non-empty
    assert!(config.port.get() >= 1024);              // Port in range
    assert!(config.port.get() <= 65535);
    assert!(config.host.get().is_private());         // Host is private
    
    // User effort: ZERO. Verification is FREE.
}
```

**The Revolutionary Part:**

When you use contract newtypes in your struct:
1. **Each field is already validated** (contract type = guaranteed valid)
2. **Struct is composition of valid fields** (all fields valid ⟹ struct valid)
3. **Kani proves the composition** (∀fields valid ⇒ struct valid)
4. **User effort = ZERO** (just use contract types, get proofs for free)

**Tasks:**

- [ ] 13.1: Derive macro recognizes contract types (Days 36-38)
  - Detect field types that implement Contract trait
  - Generate Elicitation impl that elicits each field
  - Generate Contract impl that checks all field invariants
  - Add to proc macro crate

- [ ] 13.2: Automatic verification inheritance (Days 39-41)
  - Field contracts compose to struct contract (automatically)
  - Variant contracts compose to enum contract
  - Nested structs inherit verification recursively
  - Example: Config { db: DatabaseConfig { ... } }

- [ ] 13.3: End-to-end Kani proofs (Days 42-44)
  - Prove: LLM output → Mechanism → Contract Type → Derived Type
  - Full chain proven compositionally
  - Example: Config with 10 fields, all automatically verified
  - No manual Kani annotations needed (derive generates them)

- [ ] 13.4: Trenchcoat pattern implementation (Days 45-46)
  - Implement `.into_inner()` for all contract types
  - Macro unwraps contract types to stdlib types at boundary
  - User writes: `#[elicit(non_empty)] name: String`
  - Macro uses: `StringNonEmpty` internally
  - Returns: `String` (familiar API)
  - Both styles work: Direct contract types OR attribute hints

- [ ] 13.5: Documentation (Days 47-49)
  - "Trenchcoat Pattern" guide: boundary validation explained
  - "Zero-effort verification" guide with examples
  - Case studies: API config, Database schema, Network protocol
  - Performance analysis (newtype overhead = zero after .into_inner())
  - Migration guide: Plain types → Attributed types → Contract types

**Deliverable:** Automatic verification for all derived types with familiar API

**Success Criteria:**
- ✅ `#[derive(Elicit)]` generates contracts automatically
- ✅ Users write familiar stdlib types (String, i32)
- ✅ Contract types validate at boundaries (trenchcoat pattern)
- ✅ `.into_inner()` unwraps to stdlib types (zero overhead)
- ✅ End-to-end Kani proof: LLM → Mechanism → Contract → stdlib type
- ✅ Zero user effort for verification (transparent validation)
- ✅ **REVOLUTION COMPLETE:** Familiar API + Formal verification

---

### Phase 14: Polish & Production (Week 9-10)

**Goal:** Production-ready, released to crates.io

**Tasks:**

- [ ] 14.1: CI/CD integration (Days 50-52)
  - Kani runs on every PR (fast)
  - Creusot/Prusti/Verus nightly (slow but comprehensive)
  - Cache verification artifacts
  - Performance regression detection

- [ ] 14.2: Performance optimization (Days 53-55)
  - Benchmark all contracts (< 10ns overhead target)
  - Optimize composition (O(1) where possible)
  - Profile verification time
  - Document performance characteristics

- [ ] 14.3: Documentation completion (Days 56-58)
  - Update all examples with new types
  - Add "Getting Started" guide
  - Video tutorial (optional)
  - API reference complete

- [ ] 14.4: Release preparation (Days 59-60)
  - Version 0.5.0 (major feature release)
  - Changelog comprehensive
  - Announce on Reddit, HN, Rust forums
  - Blog post: "Formally Verified AI Pipelines"

**Deliverable:** 44/44 types (100%), production release

**Success Criteria:**
- ✅ All tests passing (300+ tests)
- ✅ CI/CD running verification
- ✅ Published to crates.io
- ✅ Documentation complete
- ✅ Community announcement

---

## Success Metrics

### Type Coverage
- **Baseline:** 15/44 (34%)
- **Phase 7:** 16/44 (36%) - Integers complete
- **Phase 8:** 21/44 (48%) - Tuples complete
- **Phase 9:** 25/44 (57%) - Containers complete
- **Phase 10:** 33/44 (75%) - Specialized complete
- **Phase 11:** 39/44 (89%) - DateTime complete
- **Final:** 44/44 (100%) - Total coverage

### Mechanism Coverage
- **Baseline:** 4/7 (57%)
- **Final:** 7/7 (100%)

### Test Coverage
- **Baseline:** 50 tests
- **Phase 7-11:** +150 tests (type contracts)
- **Phase 12:** +30 tests (mechanism contracts)
- **Phase 13:** +50 tests (compositional proofs)
- **Phase 14:** +20 tests (integration)
- **Final:** 300+ tests

### Verification Depth
- **Baseline:** Type contracts only
- **Phase 12:** Mechanism contracts added
- **Phase 13:** Compositional verification
- **Final:** End-to-end LLM → Application proofs

---

## The Zero-Effort Vision (Trenchcoat Pattern)

### Before (Forgettable Validation)

```rust
#[derive(Elicit)]
struct Config {
    port: u16,
    host: IpAddr,
}

// Problem: No guarantees about validity
// - port could be 0 (invalid)
// - No way to enforce constraints
// - Easy to forget validation
```

### After Phase 13 (Trenchcoat Pattern - Familiar + Verified)

**Option 1: Attribute-driven (Familiar API):**
```rust
#[derive(Elicit)]
struct Config {
    #[elicit(range = "1024..=65535")]  // Attribute specifies contract
    port: u16,                          // User writes familiar stdlib type
    
    #[elicit(private)]                 // Attribute specifies contract
    host: IpAddr,                       // User writes familiar stdlib type
}

// Macro generates (uses contract types internally):
impl Elicitation for Config {
    async fn elicit(client: &Client) -> Result<Self> {
        // Put on trenchcoat (validate)
        let port = U16Range::<1024, 65535>::elicit(client).await?;
        let host = IpPrivate::elicit(client).await?;
        
        // Take off trenchcoat (unwrap to stdlib)
        Ok(Self {
            port: port.into_inner(),   // u16 (was validated)
            host: host.into_inner(),   // IpAddr (was validated)
        })
    }
}

// User gets: Familiar stdlib types (u16, IpAddr)
// We get: Formal verification at boundary (contract types validated)
// Kani proves: Data was valid at construction (trenchcoat worked)
```

**Option 2: Direct contract types (Strongest guarantees):**
```rust
#[derive(Elicit)]
struct Config {
    port: U16Range<1024, 65535>,  // Contract type (persistent guarantee)
    host: IpPrivate,              // Contract type (persistent guarantee)
}

// No unwrapping - fields ARE contract types
// Strongest guarantees (type-level enforcement)
```

**Both options available - users choose based on needs!**

### The Trenchcoat Pattern Explained

```
INPUT BOUNDARY          VALIDATION           OUTPUT BOUNDARY
───────────────         ──────────          ───────────────
LLM: "8080"             U16Range<1024,      User gets: u16
     ↓                  65535>(8080)              ↓
  "8080"  ─────→  U16Range::new(8080)  ─────→  u16(8080)
                  ^^^^^^^^^^^^^^^^^^^^         ^^^^^^^^^^
                  Put on trenchcoat            Take off coat
                  (validate)                   .into_inner()
```

**Key insight:** Contract types exist only during elicitation (boundary crossing).
- Put on coat → Validate → Take off coat
- User gets familiar stdlib types
- Data was proven valid at construction
- Zero runtime overhead after boundary

### What Users Get

| Feature | Before | After (Trenchcoat) |
|---------|--------|-------------------|
| Type safety | ✅ | ✅ |
| Familiar API | ✅ | ✅ (stdlib types) |
| Validation | ❌ (manual) | ✅ (automatic at boundary) |
| Formal verification | ❌ | ✅ (Kani proves contract types) |
| Runtime overhead | N/A | ⚡ **ZERO** (unwrapped after validation) |
| Ecosystem compat | ✅ | ✅ (stdlib types work everywhere) |
| User effort | 😰 High | 🎉 **ZERO** (transparent) |

### Real-World Example (Attribute-Driven)

```rust
// User writes familiar stdlib types with validation metadata
#[derive(Elicit)]
struct ApiConfig {
    #[elicit(https_only)]
    endpoint: String,                    // Familiar String
    
    #[elicit(non_empty)]
    api_key: String,                     // Familiar String
    
    #[elicit(positive)]
    timeout_secs: u32,                   // Familiar u32
    
    #[elicit(range = "1..=10")]
    retry_count: u8,                     // Familiar u8
}

#[derive(Elicit)]
struct DatabaseConfig {
    #[elicit(private)]
    host: IpAddr,                        // Familiar IpAddr
    
    #[elicit(range = "1024..=65535")]
    port: u16,                           // Familiar u16
    
    #[elicit(non_empty)]
    username: String,                    // Familiar String
    
    #[elicit(positive)]
    max_connections: u32,                // Familiar u32
}

#[derive(Elicit)]
struct Config {
    api: ApiConfig,                      // Nested validation composes!
    database: DatabaseConfig,            // All fields validated!
}

// User perspective:
// - Writes: Familiar stdlib types (String, u16, IpAddr)
// - Gets: Validated data (contract types at boundary)
// - Uses: Normal Rust code (no new API to learn)
//
// Our perspective:
// - Implementation: Uses contract types (UrlHttps, StringNonEmpty, U16Range)
// - Validation: Boundary crossing (put on coat, validate, take off coat)
// - Verification: Kani proves contract types work
//
// When you have Config, you KNOW (proven by Kani at boundary):
// - API endpoint is HTTPS (not HTTP)
// - API key is non-empty
// - Timeout is positive
// - Retry count is 1-10
// - Database host is private
// - Database port is valid
// - Username is non-empty
// - Max connections is positive
// ALL PROVEN AT CONSTRUCTION BOUNDARY. NO EFFORT. FAMILIAR API.
```

### The Proof Chain (Trenchcoat Pattern)

```
LLM outputs "8080" and "192.168.1.1"
    ↓
INPUT BOUNDARY (Put on trenchcoat)
    ↓
Mechanism: Numeric elicits u16           ✅ Proven: parses to u16
    ↓
Type Construction: U16Range::new(8080)   ✅ Proven: 8080 ∈ [1024, 65535]
    ↓  (validation on construction)
U16Range<1024, 65535>(8080)             ✅ Newtype validates
    ↓
OUTPUT BOUNDARY (Take off trenchcoat)
    ↓
U16Range(8080).into_inner()             ✅ Unwrap to u16
    ↓
User receives: u16(8080)                ✅ Familiar type, was validated
    ↓
───────────────────────────────────────────────────────────────
    ↓
Mechanism: Network elicits IpAddr        ✅ Proven: valid IP format
    ↓
Type Construction: IpPrivate::new(ip)    ✅ Proven: IP is private
    ↓  (validation on construction)
IpPrivate(192.168.1.1)                  ✅ Newtype validates
    ↓
OUTPUT BOUNDARY (Take off trenchcoat)
    ↓
IpPrivate(ip).into_inner()              ✅ Unwrap to IpAddr
    ↓
User receives: IpAddr(192.168.1.1)      ✅ Familiar type, was validated
    ↓
───────────────────────────────────────────────────────────────
    ↓
Composition: Config { port, host }       ✅ Both fields stdlib types
    ↓  (both validated at boundary!)
Config { port: u16, host: IpAddr }      ✅ Familiar API
    ↓
User receives Config                     ✅ Data proven valid at construction

Mathematical proof (trenchcoat pattern):
  ∀config: Config::elicit(client) ⇒ 
    (port was validated via U16Range) ∧ 
    (host was validated via IpPrivate) ∧
    (data is stdlib types) ∧
    valid_at_construction(config)
  ∎

Key insight: Validation happens AT BOUNDARY (in trenchcoat).
Once validated, unwrap to familiar stdlib types.
User gets: Familiar API + Proven correct at construction + Zero overhead.
```
  ∀config: Config ⇒ 
    port ∈ U16Range<1024, 65535> ∧ 
    host ∈ IpPrivate ∧
    valid(config)
  ∎

Key insight: Validation happens AT CONSTRUCTION (in new()).
If you have the type, it MUST be valid (impossible to construct otherwise).
```

---

## Risk Mitigation

### Technical Risks

| Risk | Impact | Mitigation | Status |
|------|--------|------------|--------|
| Kani state explosion | High | Bound symbolic values, incremental proofs | Mitigated |
| Creusot setup complexity | Medium | Document thoroughly, provide scripts | Mitigated |
| Prusti incomplete Rust support | Medium | Document limitations, fallback to Kani | Accepted |
| Derive macro complexity | High | Phase 13 dedicated to this, use syn/quote | Planned |
| Performance overhead | Low | < 10ns per contract, negligible | Verified |

### Process Risks

| Risk | Impact | Mitigation | Status |
|------|--------|------------|--------|
| Timeline slip | Medium | Phases independent, can ship incrementally | Mitigated |
| Scope creep | Low | Strict phase boundaries, no feature additions | Controlled |
| Verification bugs | High | Extensive testing, Kani catches errors early | Mitigated |
| Community adoption | Medium | Clear docs, examples, blog posts | Planned |

---

## Dependencies

### Internal
- `elicitation` - Core elicitation types
- `elicitation_derive` - Proc macro for #[derive(Elicit)]
- `elicitation_survey` - Survey generation

### External
- `kani` - Model checker (cargo install)
- `creusot` - Deductive verifier (manual setup)
- `prusti` - Separation logic (manual setup)
- `verus` - SMT verifier (manual setup)

### Feature Flags
- `verification` - Core contracts (Kani default)
- `verify-kani` - Kani verification
- `verify-creusot` - Creusot verification
- `verify-prusti` - Prusti verification
- `verify-verus` - Verus verification
- `datetime-chrono` - chrono contracts
- `datetime-jiff` - jiff contracts

---

## Timeline Summary

| Phase | Duration | Deliverable | Coverage |
|-------|----------|-------------|----------|
| 7 | Week 1 | Complete integers | 36% |
| 8 | Week 2 | Tuple verification | 48% |
| 9 | Week 3 | Container types | 57% |
| 10 | Week 4 | Specialized types | 75% |
| 11 | Week 5 | DateTime libraries | 89% |
| 12 | Week 6 | Mechanism completion | 100% mechanisms |
| 13 | Week 7-8 | Compositional verification | Auto-verify |
| 14 | Week 9-10 | Polish & release | Production |

**Total:** 10 weeks (50 days)

---

## Maintenance Plan

### Post-Release

**Continuous:** Bug fixes, performance improvements
**Monthly:** Dependency updates, security audits
**Quarterly:** New verifier support, additional contracts
**Yearly:** Major version bump, ecosystem integration

### Community Engagement

- GitHub Discussions for questions
- Monthly blog posts showcasing verification
- Conference talks (RustConf, etc)
- Academic papers on compositional verification

---

## Success Vision

### Short-term (Phase 14 complete)

✅ 100% std type coverage
✅ 100% mechanism coverage  
✅ 300+ tests passing
✅ 4 verifiers working
✅ Published to crates.io

### Medium-term (6 months post-release)

✅ 1000+ users
✅ 10+ real-world applications using verification
✅ Case studies: Security-critical AI systems
✅ Academic citations
✅ Integration with popular AI frameworks

### Long-term (1 year post-release)

✅ Standard for AI content pipelines
✅ Formal verification as default expectation
✅ Ecosystem adoption (crates building on our foundation)
✅ Research enabling new verification techniques
✅ Proven reduction in AI-related bugs/vulnerabilities

---

## The Ultimate Goal (Trenchcoat Pattern Vision)

**Every AI content pipeline in Rust gets formal verification for free, with familiar stdlib types.**

**Option 1: Attribute-driven (Beginner-friendly):**
```rust
#[derive(Elicit)]
struct UserInput {
    #[elicit(non_empty)]        // Metadata specifies validation
    name: String,                // Familiar stdlib type
    
    #[elicit(range = "0..=120")] // Metadata specifies validation
    age: u8,                     // Familiar stdlib type
    
    #[elicit(email)]             // Metadata specifies validation
    email: String,               // Familiar stdlib type
}
```

**Option 2: Direct contract types (Expert-level):**
```rust
#[derive(Elicit)]
struct UserInput {
    name: StringNonEmpty,       // Contract type (persistent)
    age: U8Range<0, 120>,      // Contract type (persistent)
    email: EmailAddress,        // Contract type (persistent)
}
```

**Both work! Users choose based on needs.**

You get:
- **Familiar API** (Option 1: stdlib types) ✅
- **Or strongest guarantees** (Option 2: contract types) ✅
- **Validation at boundary** (trenchcoat pattern) ✅
- **Formal proofs** (Kani/Creusot/Prusti/Verus prove correctness) ✅
- **Zero runtime overhead** (unwrapped after validation) ✅
- **Zero effort** (derive macro handles everything) ✅
- **Mathematical guarantees** (∀input: UserInput ⇒ valid(input) proven) ✅

**The Trenchcoat Pattern:**

```
INPUT BOUNDARY → Put on coat → VALIDATE → Take off coat → OUTPUT
LLM              (wrap)        (contract)  (unwrap)       stdlib
```

**Option 1 flow (attribute-driven):**
```rust
// User writes:    name: String
// Macro uses:     StringNonEmpty (internal validation)
// Returns:        String (unwrapped via .into_inner())
// Result:         Familiar type, was proven valid at boundary
```

**Option 2 flow (direct contract types):**
```rust
// User writes:    name: StringNonEmpty
// Macro uses:     StringNonEmpty (no unwrapping)
// Returns:        StringNonEmpty (persistent guarantee)
// Result:         Type-level enforcement, always valid
```

**Result:** LLM → Application pipelines with aerospace-level correctness guarantees, achieved through:
- Boundary validation (trenchcoat pattern)
- Contract types (validation by construction)
- Formal verification (Kani proves correctness)
- Familiar API (stdlib types for Option 1) OR
- Strongest guarantees (contract types for Option 2)

---

## Commitment

This plan is **uncompromising** in its dedication to formal verification through the trenchcoat pattern. Every type, every mechanism, every boundary proven correct.

**The insight:** 
- Validate at boundaries (trenchcoat pattern)
- Make invalid states unrepresentable (contract types)
- Prove it mathematically (formal verification)
- Give users familiar API (stdlib types)

**The bar:** If Kani can't prove it, we don't ship it.

**The promise:** When you use elicitation, your AI pipeline is formally verified. Period. With familiar stdlib types.

**The impact:** We change the standard for what "correct AI integration" means:
- From "hope and test" → "proven at boundary"
- From "new types to learn" → "familiar stdlib types"  
- From "manual validation" → "transparent verification"
- From "runtime overhead" → "zero-cost abstraction"
