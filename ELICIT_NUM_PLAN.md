# elicit_num — Implementation Plan

> **Critical Finding:** `num-traits` is a **trait library** defining compile-time abstractions
> that cannot cross the JSON boundary. Instead, we target the **concrete type crates**:
> `num-bigint`, `num-rational`, and `num-complex`.

---

## The Trait Library Problem

**Why num-traits is NOT viable:**

| What num-traits provides | Can cross JSON? | Why not? |
|--------------------------|----------------|----------|
| `Num`, `Float`, `Integer` traits | ❌ | Compile-time abstractions |
| Generic implementations | ❌ | No runtime representation |
| Type property queries | ❌ | "Does u32 impl Float?" is compile-time |
| Utility functions (`cast`, `pow`) | ⚠️ | Redundant with std/serde_json |

**The Real Value:** Concrete types like `BigInt`, `Ratio<T>`, `Complex<T>` that num-traits *enables*.

---

## Target Crates (Concrete Types)

### Priority 1: num-bigint ⭐⭐⭐

**What it provides:**
- `BigInt` — arbitrary-precision signed integers
- `BigUint` — arbitrary-precision unsigned integers
- Heap-allocated, variable-size (beyond u64/i128)

**Use cases:**
- Cryptography (RSA, modular exponentiation)
- Combinatorics (factorials, large binomials)
- Number theory (GCD of huge numbers)
- Exact integer arithmetic without overflow

**MCP viability:** ✅ Excellent
- All operations have JSON-serializable inputs/outputs
- Serde support (with caveat — see Challenge 1)
- No closures, no futures, no lifetimes

### Priority 2: num-rational ⭐⭐⭐

**What it provides:**
- `Ratio<T>` — exact rational numbers (numerator/denominator)
- Type aliases: `Rational32`, `Rational64`, `BigRational`
- Automatic reduction to lowest terms

**Use cases:**
- Financial calculations requiring exact precision
- Mathematical notation (display as "22/7" not "3.142857...")
- Educational tools (fraction arithmetic, mixed numbers)
- Symbolic computation

**MCP viability:** ✅ Excellent
- Struct with two fields (numer, denom) — easily serializable
- All operations return new Rational values
- Division-by-zero validation crucial for workflow proofs

### Priority 3: num-complex ⭐⭐

**What it provides:**
- `Complex<T>` — complex numbers (re + im*i)
- Type aliases: `Complex32`, `Complex64`
- Cartesian and polar representations

**Use cases:**
- Engineering (electrical impedance, signal processing)
- Polynomial root finding
- Mathematical explorations (fractals, etc.)

**MCP viability:** ✅ Good
- Struct with two fields (re, im) — serializable
- More specialized than bigint/rational
- Lower priority unless specific use cases emerge

---

## Challenge 1: BigInt Serialization Format

**Problem:** Upstream `num-bigint` serializes as **tuples**, not decimal strings:

```javascript
// Upstream format (hard to use in other languages):
-123456789n → [Sign::Minus, [2899336981, 28744523]]  // (sign, [u32 digits])

// What we want (cross-language compatible):
-123456789n → "-123456789"
```

**Solution:** Custom serde implementation

```rust
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct BigInt(
    #[serde(with = "bigint_as_string")]
    num_bigint::BigInt
);

mod bigint_as_string {
    pub fn serialize<S>(value: &num_bigint::BigInt, s: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        value.to_string().serialize(s)
    }

    pub fn deserialize<'de, D>(d: D) -> Result<num_bigint::BigInt, D::Error>
    where D: Deserializer<'de> {
        let s = String::deserialize(d)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
```

**Benefits:**
- ✅ JSON-compatible: `"123456789"` instead of `[1, [12345, 6789]]`
- ✅ Cross-language: Python/JS can parse strings directly
- ✅ Human-readable: easier debugging

---

## Phase 1: elicit_num_bigint (Weeks 1-2)

### Crate Structure

```
crates/elicit_num_bigint/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── big_int.rs        // BigInt newtype wrapper
    ├── big_uint.rs       // BigUint newtype wrapper
    ├── sign.rs           // Sign enum (Select)
    └── workflow.rs       // BigIntWorkflowPlugin
```

### Cargo.toml

```toml
[dependencies]
num-bigint = { version = "0.4", features = ["serde"] }
elicitation = { workspace = true, features = ["emit"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### Type Wrappers

**BigInt:**
```rust
// src/big_int.rs
use elicitation::elicit_newtype;
use num_bigint::BigInt as InnerBigInt;

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(transparent)]
pub struct BigInt(
    #[serde(with = "bigint_as_string")]
    InnerBigInt
);

#[reflect_methods]
impl BigInt {
    // Arithmetic
    pub fn add(&self, other: &BigInt) -> BigInt;
    pub fn multiply(&self, other: &BigInt) -> BigInt;
    pub fn divide(&self, other: &BigInt) -> Result<BigInt, DivisionError>;
    pub fn modulo(&self, other: &BigInt) -> Result<BigInt, DivisionError>;
    pub fn pow(&self, exp: u32) -> BigInt;
    pub fn abs(&self) -> BigInt;
    pub fn negate(&self) -> BigInt;

    // Bitwise
    pub fn bit_length(&self) -> u64;
    pub fn count_ones(&self) -> u64;
    pub fn trailing_zeros(&self) -> Option<u64>;

    // Comparison
    pub fn is_zero(&self) -> bool;
    pub fn is_positive(&self) -> bool;
    pub fn is_negative(&self) -> bool;
    pub fn is_even(&self) -> bool;
    pub fn is_odd(&self) -> bool;

    // Conversions
    pub fn to_i64(&self) -> Option<i64>;
    pub fn to_u64(&self) -> Option<u64>;
    pub fn to_f64(&self) -> f64;  // May lose precision
    pub fn to_string(&self) -> String;
    pub fn to_hex_string(&self) -> String;
}
```

**BigUint (similar pattern):**
```rust
// src/big_uint.rs
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct BigUint(
    #[serde(with = "biguint_as_string")]
    num_bigint::BigUint
);

#[reflect_methods]
impl BigUint {
    // Similar methods, but no sign-related ones
    // All results are non-negative
}
```

**Sign Enum:**
```rust
// src/sign.rs
#[derive(Serialize, Deserialize, JsonSchema, Elicit)]
pub enum Sign {
    Minus,
    NoSign,  // Zero
    Plus,
}
```

### Workflow Plugin

**BigIntWorkflowPlugin:**
```rust
// src/workflow.rs
#[derive(Clone, ElicitPlugin)]
pub struct BigIntWorkflowPlugin;

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "parse_decimal",
    description = "Parse a decimal string as BigInt. Establishes: ValidBigInt."
)]
async fn parse_decimal(p: ParseDecimalParams) -> Result<CallToolResult, ErrorData> {
    match p.value.parse::<BigInt>() {
        Ok(n) => Ok(CallToolResult::success(json!({
            "value": n,
            "proof": "ValidBigInt"
        }))),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(format!(
            "ValidBigInt not established: {}", e
        ))])),
    }
}

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "parse_hex",
    description = "Parse a hexadecimal string as BigInt. Establishes: ValidBigInt."
)]
async fn parse_hex(p: ParseHexParams) -> Result<CallToolResult, ErrorData> {
    let s = p.value.strip_prefix("0x").unwrap_or(&p.value);
    match BigInt::from_str_radix(s, 16) {
        Ok(n) => Ok(CallToolResult::success(json!({
            "value": n,
            "proof": "ValidBigInt"
        }))),
        Err(e) => Ok(CallToolResult::error(/* ... */)),
    }
}

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "factorial",
    description = "Compute n! for non-negative integers. Requires: NonNegative."
)]
async fn factorial(p: FactorialParams) -> Result<CallToolResult, ErrorData> {
    if p.n > 100000 {
        return Ok(CallToolResult::error(vec![Content::text(
            "Factorial too large (n > 100000)"
        )]));
    }

    let mut result = BigInt::from(1);
    for i in 2..=p.n {
        result = result.multiply(&BigInt::from(i));
    }

    Ok(CallToolResult::success(json!({
        "value": result,
        "proof": "Positive"  // n! >= 1 for all n >= 0
    })))
}

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "gcd",
    description = "Compute greatest common divisor. Establishes: NonNegative."
)]
async fn gcd(p: GcdParams) -> Result<CallToolResult, ErrorData> {
    let result = num_integer::gcd(p.a.0, p.b.0);
    Ok(CallToolResult::success(json!({
        "value": BigInt(result),
        "proof": "NonNegative"  // GCD is always >= 0
    })))
}

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "modpow",
    description = "Modular exponentiation: (base^exp) mod modulus. \
                   Requires: NonZeroModulus."
)]
async fn modpow(p: ModPowParams) -> Result<CallToolResult, ErrorData> {
    if p.modulus.is_zero() {
        return Ok(CallToolResult::error(vec![Content::text(
            "NonZeroModulus not satisfied: modulus cannot be zero"
        )]));
    }

    let result = p.base.0.modpow(&p.exponent.0, &p.modulus.0);
    Ok(CallToolResult::success(json!({
        "value": BigInt(result),
        "proof": "ValidBigInt"
    })))
}
```

### Propositions

```rust
// elicitation/src/primitives/num_types/propositions.rs
pub struct ValidBigInt;      // Successfully parsed/constructed
pub struct NonNegative;      // value >= 0
pub struct Positive;         // value > 0
pub struct NonZero;          // value != 0
pub struct NonZeroModulus;   // modulus != 0 (for modpow)
pub struct InRange<const MIN: i64, const MAX: i64>;  // value in [MIN, MAX]
```

---

## Phase 2: elicit_num_rational (Weeks 3-4)

### Crate Structure

```
crates/elicit_num_rational/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── rational.rs       // Ratio<i64> wrapper
    ├── big_rational.rs   // Ratio<BigInt> wrapper
    └── workflow.rs       // RationalWorkflowPlugin
```

### Type Wrappers

**Rational:**
```rust
// src/rational.rs
use num_rational::Ratio;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Rational {
    pub numer: i64,
    pub denom: i64,
}

impl From<Ratio<i64>> for Rational {
    fn from(r: Ratio<i64>) -> Self {
        Rational {
            numer: *r.numer(),
            denom: *r.denom(),
        }
    }
}

impl From<Rational> for Ratio<i64> {
    fn from(r: Rational) -> Self {
        Ratio::new(r.numer, r.denom)
    }
}

#[reflect_methods]
impl Rational {
    // Arithmetic
    pub fn add(&self, other: &Rational) -> Rational;
    pub fn multiply(&self, other: &Rational) -> Rational;
    pub fn divide(&self, other: &Rational) -> Result<Rational, DivisionError>;
    pub fn pow(&self, exp: i32) -> Rational;
    pub fn recip(&self) -> Result<Rational, DivisionError>;  // 1/x
    pub fn abs(&self) -> Rational;
    pub fn negate(&self) -> Rational;

    // Rounding
    pub fn floor(&self) -> i64;
    pub fn ceil(&self) -> i64;
    pub fn round(&self) -> i64;
    pub fn trunc(&self) -> i64;
    pub fn fract(&self) -> Rational;  // Fractional part

    // Properties
    pub fn is_integer(&self) -> bool;  // denom == 1
    pub fn is_positive(&self) -> bool;
    pub fn is_negative(&self) -> bool;
    pub fn numerator(&self) -> i64;
    pub fn denominator(&self) -> i64;

    // Conversions
    pub fn to_f64(&self) -> f64;
    pub fn to_string(&self) -> String;  // "22/7"
}
```

### Workflow Plugin

**RationalWorkflowPlugin:**
```rust
// src/workflow.rs
#[elicit_tool(
    plugin = "rational_workflow",
    name = "create",
    description = "Create a rational number from numerator and denominator. \
                   Establishes: ValidRational (non-zero denom, reduced form)."
)]
async fn create(p: CreateRationalParams) -> Result<CallToolResult, ErrorData> {
    if p.denominator == 0 {
        return Ok(CallToolResult::error(vec![Content::text(
            "ValidRational not established: denominator cannot be zero"
        )]));
    }

    let ratio = Ratio::new(p.numerator, p.denominator);
    let rational = Rational::from(ratio);

    Ok(CallToolResult::success(json!({
        "value": rational,
        "proof": "ValidRational"
    })))
}

#[elicit_tool(
    plugin = "rational_workflow",
    name = "from_float",
    description = "Approximate a float as a rational. Establishes: ValidRational."
)]
async fn from_float(p: FromFloatParams) -> Result<CallToolResult, ErrorData> {
    if !p.value.is_finite() {
        return Ok(CallToolResult::error(vec![Content::text(
            "ValidRational not established: value must be finite"
        )]));
    }

    let ratio = Ratio::approximate_float(p.value)
        .ok_or_else(|| ErrorData::new("Cannot approximate float"))?;
    let rational = Rational::from(ratio);

    Ok(CallToolResult::success(json!({
        "value": rational,
        "proof": "ValidRational"
    })))
}

#[elicit_tool(
    plugin = "rational_workflow",
    name = "to_mixed_number",
    description = "Convert to mixed number (whole + fraction). \
                   Returns: (whole, numer, denom)."
)]
async fn to_mixed_number(p: ToMixedParams) -> Result<CallToolResult, ErrorData> {
    let ratio = Ratio::from(p.rational);
    let whole = ratio.trunc().to_integer();
    let fract = ratio.fract();

    Ok(CallToolResult::success(json!({
        "whole": whole,
        "numerator": fract.numer(),
        "denominator": fract.denom(),
    })))
}

#[elicit_tool(
    plugin = "rational_workflow",
    name = "simplify",
    description = "Reduce to lowest terms. Establishes: ReducedForm."
)]
async fn simplify(p: SimplifyParams) -> Result<CallToolResult, ErrorData> {
    let ratio = Ratio::new(p.numerator, p.denominator);
    let rational = Rational::from(ratio);

    Ok(CallToolResult::success(json!({
        "value": rational,
        "proof": "ReducedForm"
    })))
}
```

### Propositions

```rust
pub struct ValidRational;    // denom != 0, in reduced form
pub struct ReducedForm;      // gcd(numer, denom) == 1
pub struct ProperFraction;   // |numer| < |denom|
pub struct IsInteger;        // denom == 1
```

---

## Phase 3: elicit_num_complex (Weeks 5-6, Optional)

### Type Wrapper

**Complex64:**
```rust
// src/complex.rs
use num_complex::Complex;

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

#[reflect_methods]
impl Complex64 {
    // Construction
    pub fn new(re: f64, im: f64) -> Complex64;
    pub fn from_polar(r: f64, theta: f64) -> Complex64;

    // Arithmetic
    pub fn add(&self, other: &Complex64) -> Complex64;
    pub fn multiply(&self, other: &Complex64) -> Complex64;
    pub fn divide(&self, other: &Complex64) -> Result<Complex64, DivisionError>;
    pub fn pow(&self, exp: &Complex64) -> Complex64;

    // Transformations
    pub fn conjugate(&self) -> Complex64;
    pub fn reciprocal(&self) -> Result<Complex64, DivisionError>;
    pub fn exp(&self) -> Complex64;
    pub fn ln(&self) -> Complex64;
    pub fn sqrt(&self) -> Complex64;
    pub fn sin(&self) -> Complex64;
    pub fn cos(&self) -> Complex64;

    // Properties
    pub fn norm(&self) -> f64;       // Magnitude |z|
    pub fn norm_sqr(&self) -> f64;   // |z|^2
    pub fn arg(&self) -> f64;        // Phase angle
    pub fn to_polar(&self) -> (f64, f64);  // (magnitude, phase)
    pub fn is_nan(&self) -> bool;
    pub fn is_infinite(&self) -> bool;
    pub fn is_finite(&self) -> bool;
}
```

### Workflow Plugin

**ComplexWorkflowPlugin:**
```rust
#[elicit_tool(
    plugin = "complex_workflow",
    name = "create",
    description = "Create a complex number. Establishes: ValidComplex (finite)."
)]
async fn create(p: CreateComplexParams) -> Result<CallToolResult, ErrorData> {
    if !p.re.is_finite() || !p.im.is_finite() {
        return Ok(CallToolResult::error(vec![Content::text(
            "ValidComplex not established: components must be finite"
        )]));
    }

    Ok(CallToolResult::success(json!({
        "value": Complex64 { re: p.re, im: p.im },
        "proof": "ValidComplex"
    })))
}

#[elicit_tool(
    plugin = "complex_workflow",
    name = "roots_of_unity",
    description = "Compute nth roots of unity. Returns: Vec<Complex64>."
)]
async fn roots_of_unity(p: RootsParams) -> Result<CallToolResult, ErrorData> {
    use std::f64::consts::PI;

    let mut roots = Vec::new();
    for k in 0..p.n {
        let theta = 2.0 * PI * (k as f64) / (p.n as f64);
        roots.push(Complex64::from_polar(1.0, theta));
    }

    Ok(CallToolResult::success(json!({
        "roots": roots,
        "proof": "UnitCircle"  // All have magnitude 1
    })))
}
```

### Propositions

```rust
pub struct ValidComplex;     // Both components finite
pub struct NonZero;          // norm != 0
pub struct UnitCircle;       // norm == 1
pub struct Real;             // im == 0
pub struct Imaginary;        // re == 0
```

---

## Verification Strategy

### What We Verify (Kani/Creusot/Verus)

✅ **Wrapper layer:**
- Parsing correctness (string → BigInt → string roundtrip)
- Error handling (division by zero, invalid input)
- Type conversions (BigInt ↔ i64, Rational ↔ f64)

✅ **Precondition checking:**
- Division by zero prevention (requires `NonZero`)
- Factorial domain (requires `NonNegative`)
- Modpow validation (requires `NonZeroModulus`)

✅ **Proposition establishment:**
- `ValidBigInt` established after successful parse
- `ValidRational` established after create (denom != 0)
- `NonNegative` established after GCD (always >= 0)

### What We Trust (Upstream)

❌ **Internal algorithms:**
- BigInt arithmetic (Karatsuba multiplication, etc.)
- Rational reduction (Euclidean GCD)
- Complex exponential (series expansions)

**Verification pattern:**
```rust
// elicitation_kani/src/num_bigint_proofs.rs
#[kani::proof]
fn verify_bigint_parse_nonzero() {
    let s = kani::any::<String>();
    kani::assume(s.chars().all(|c| c.is_ascii_digit()));
    kani::assume(s != "0");

    if let Ok(n) = BigInt::from_str(&s) {
        // Verify our wrapper preserves non-zero property
        assert!(!n.is_zero());
    }
}

#[kani::proof]
fn verify_rational_denom_nonzero() {
    let numer = kani::any::<i64>();
    let denom = kani::any::<i64>();
    kani::assume(denom != 0);

    let r = Rational::create(numer, denom);

    // Verify our wrapper ensures non-zero denominator
    assert_ne!(r.denominator(), 0);
}
```

---

## Example Agent Workflows

### Workflow 1: RSA Key Generation (Cryptography)

```json
// 1. Generate large prime candidates
{
  "tool": "parse_decimal",
  "params": { "value": "115792089237316195423570985008687907853269984665640564039457584007913129639936" }
}
// → { "value": "...", "proof": "ValidBigInt" }

// 2. Compute modulus n = p * q
{
  "tool": "bigint_multiply",
  "params": { "a": "<p>", "b": "<q>" }
}

// 3. Compute φ(n) = (p-1)(q-1)
// ... more operations

// 4. Modular exponentiation for encryption
{
  "tool": "modpow",
  "params": {
    "base": "<message>",
    "exponent": "<e>",
    "modulus": "<n>"
  }
}
// → { "value": "<ciphertext>", "proof": "ValidBigInt" }
```

### Workflow 2: Exact Financial Calculations

```json
// 1. Create tax rate (7.5%)
{
  "tool": "create",
  "params": { "numerator": 75, "denominator": 1000 }
}
// → { "value": { "numer": 3, "denom": 40 }, "proof": "ValidRational" }

// 2. Multiply price by tax rate
{
  "tool": "rational_multiply",
  "params": {
    "a": { "numer": 12999, "denom": 100 },  // $129.99
    "b": { "numer": 3, "denom": 40 }        // 7.5%
  }
}
// → { "value": { "numer": 38997, "denom": 4000 }, ... }

// 3. Convert to mixed number for display
{
  "tool": "to_mixed_number",
  "params": { "rational": { "numer": 38997, "denom": 4000 } }
}
// → { "whole": 9, "numerator": 2997, "denominator": 4000 }
// Displays as: $9 + 2997/4000 = $9.74925 (exact, no rounding)
```

### Workflow 3: Polynomial Roots (Complex Numbers)

```json
// Solve x^2 + 1 = 0 using quadratic formula
// Roots: ±i

{
  "tool": "create",
  "params": { "re": 0.0, "im": 1.0 }
}
// → { "value": { "re": 0, "im": 1 }, "proof": "ValidComplex" }

{
  "tool": "complex_negate",
  "params": { "z": { "re": 0, "im": 1 } }
}
// → { "value": { "re": 0, "im": -1 }, "proof": "ValidComplex" }

// Verify they're roots by computing z^2 + 1
{
  "tool": "complex_pow",
  "params": {
    "base": { "re": 0, "im": 1 },
    "exp": { "re": 2, "im": 0 }
  }
}
// → { "value": { "re": -1, "im": 0 }, ... }
```

---

## Success Metrics

1. **API Coverage:** 90%+ of practical operations exposed
2. **Serialization:** String-based BigInt/Rational/Complex (cross-language compatible)
3. **Verification:** All preconditions checked, propositions established
4. **Performance:** Acceptable for interactive LLM use (< 1s for most operations)
5. **Documentation:** Examples for crypto, finance, engineering use cases

---

## Timeline

- **Phase 1 (num-bigint):** 2 weeks — ~25 tools
- **Phase 2 (num-rational):** 2 weeks — ~20 tools
- **Phase 3 (num-complex):** 2 weeks — ~15 tools (optional)

**Total:** 4-6 weeks, ~45-60 tools

---

## Key Insights

### Why This Works (vs. num-traits)

**num-traits:**
- ❌ Trait definitions (compile-time only)
- ❌ No runtime values to serialize
- ❌ Zero MCP viability

**Concrete crates:**
- ✅ Actual data structures (BigInt, Ratio, Complex)
- ✅ Full serde support
- ✅ All operations return serializable values
- ✅ No closures, no futures, no lifetimes

### The Value Proposition

**For Agents:**
- Exact arithmetic beyond primitive limits
- Mathematical operations with precision guarantees
- Cross-domain applications (crypto, finance, engineering)

**For Verification:**
- Proof-carrying workflows with typestate
- Precondition validation (NonZero, NonNegative, etc.)
- Contract enforcement at MCP boundary

---

## Conclusion

The pivot from **num-traits** (trait library) to **concrete type crates** (num-bigint, num-rational, num-complex) transforms this from an impossible target into an excellent one.

These crates provide **exactly what MCP needs:**
- Serializable types with meaningful operations
- No closures or async complexity
- Clear value proposition for LLM agents
- Strong verification story via propositions

**Result:** Agents gain access to arbitrary-precision arithmetic, exact rational calculations, and complex number operations—all with proof-carrying contracts.
