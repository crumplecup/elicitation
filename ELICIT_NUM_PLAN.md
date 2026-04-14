# elicit_num — Complete Numeric Types Harvesting Plan

> **Completionist mandate:** Expose the entire num ecosystem (traits + concrete types) as MCP tools.
> **Three-pronged approach:** Runtime tools (concrete types) + Fragment tools (generic code) + Factory pattern (trait methods).
> **Key insight:** Traits CAN be exposed via factory pattern. Generic code CAN be emitted as fragments.

---

## Executive Summary

**Scope:** num-traits (all numeric traits) + num-bigint, num-rational, num-complex (concrete types)
**Strategy:** Harvest 100% using Runtime + Fragment + Factory patterns
**Estimated tools:** 400-500 MCP tools
**Challenge:** num is split between compile-time abstractions (traits) and runtime values (concrete types). We harvest BOTH.

---

## The Three Patterns Applied to num

### Pattern 1: Runtime Tools (Concrete Types)

**What works at runtime:**

- BigInt/BigUint arithmetic operations (fully serializable)
- Rational number operations (numerator/denominator as JSON)
- Complex number operations (re/im as JSON)
- All operations with concrete, serializable inputs/outputs

**Example:**

```rust
#[elicit_tool(plugin = "bigint_runtime", name = "add")]
async fn bigint_add(p: BigIntAddParams) -> Result<CallToolResult, ErrorData> {
    let a: BigInt = p.a.parse()?;
    let b: BigInt = p.b.parse()?;
    let result = a + b;
    Ok(CallToolResult::success(json!({ "value": result.to_string() })))
}
```

### Pattern 2: Fragment Tools (Generic Code Generation)

**What becomes fragments:**

- Generic functions with trait bounds (`T: Num`, `T: Integer`, etc.)
- Emit code like `fn add<T: Num>(a: T, b: T) -> T { a + b }`
- Polymorphic number code that works for any numeric type

**Example:**

```rust
#[elicit_tool(
    plugin = "num_fragments",
    name = "emit_add_generic",
    description = "Emit generic addition: fn add<T: Num>(a: T, b: T) -> T",
    emit = Auto
)]
async fn emit_add_generic(p: EmitAddGenericParams) -> Result<CallToolResult, ErrorData> {
    // Emits: fn add<T: Num>(a: T, b: T) -> T { a + b }
    let code = format!(
        "fn {}<T: Num>(a: T, b: T) -> T {{ a + b }}",
        p.function_name
    );
    Ok(CallToolResult::success(Content::text(code)))
}
```

### Pattern 3: Factory Pattern (Trait Methods)

**Traits to expose:**

- `Num` - basic numeric trait (add, sub, mul, zero, one)
- `Zero` / `One` - identity elements
- `Integer` - integer-specific operations
- `Float` - floating-point operations
- `Signed` / `Unsigned` - sign operations
- `Bounded` - min/max values
- `CheckedOps` - checked arithmetic (overflow detection)
- `SaturatingOps` - saturating arithmetic
- `WrappingOps` - wrapping arithmetic

**Example:**

```rust
// Wrapper trait with blanket impl (erases generic T)
pub trait NumJson: Sized {
    fn add_json(a_json: &str, b_json: &str) -> Result<String, String>;
    fn mul_json(a_json: &str, b_json: &str) -> Result<String, String>;
    fn zero_json() -> String;
    fn one_json() -> String;
}

impl<T> NumJson for T
where
    T: Num + serde::Serialize + serde::de::DeserializeOwned,
{
    fn add_json(a_json: &str, b_json: &str) -> Result<String, String> {
        let a: T = serde_json::from_str(a_json)?;
        let b: T = serde_json::from_str(b_json)?;
        let result = a + b;
        Ok(serde_json::to_string(&result)?)
    }

    fn zero_json() -> String {
        serde_json::to_string(&T::zero()).unwrap()
    }

    // ... other methods
}

#[reflect_trait(crate::NumJson)]
pub trait NumJsonTools: Sized {
    fn add_json(a_json: &str, b_json: &str) -> Result<String, String>;
    fn mul_json(a_json: &str, b_json: &str) -> Result<String, String>;
    fn zero_json() -> String;
    fn one_json() -> String;
}
```

---

## Architecture: Four Shadow Crates

### Crate 1: elicit_num_traits

**Purpose:** Expose all num-traits trait methods via factory pattern
**Patterns:** Factory (all traits) + Fragment (generic code generation)

**Harvest:**

- 20+ num-traits traits via factory pattern
- Fragment tools for generic function generation
- Type alias generators for trait-bounded types

### Crate 2: elicit_num_bigint

**Purpose:** Arbitrary-precision integer operations
**Patterns:** Runtime (concrete types) + Dual (operations) + Workflow (propositions)

**Harvest:**

- BigInt/BigUint operations (200+ tools)
- Workflow tools with propositions (ValidBigInt, NonZero, etc.)
- Code generation for bigint literals and operations

### Crate 3: elicit_num_rational

**Purpose:** Exact rational number operations
**Patterns:** Runtime + Dual + Workflow

**Harvest:**

- Rational/BigRational operations (100+ tools)
- Fraction manipulation tools
- Workflow tools with propositions (ValidRational, ReducedForm, etc.)

### Crate 4: elicit_num_complex

**Purpose:** Complex number operations
**Patterns:** Runtime + Dual

**Harvest:**

- Complex32/Complex64 operations (80+ tools)
- Polar/Cartesian conversions
- Mathematical operations (sin, cos, exp, etc.)

---

## Phase 1: elicit_num_traits — Trait Method Factory

### 1.1 Core Trait: Num (Factory Pattern)

**Wrapper trait:**

```rust
pub trait NumJson: Sized + 'static {
    fn type_name() -> &'static str;
    fn add_json(a: &str, b: &str) -> Result<String, String>;
    fn sub_json(a: &str, b: &str) -> Result<String, String>;
    fn mul_json(a: &str, b: &str) -> Result<String, String>;
    fn div_json(a: &str, b: &str) -> Result<String, String>;
    fn rem_json(a: &str, b: &str) -> Result<String, String>;
    fn zero_json() -> String;
    fn one_json() -> String;
    fn is_zero_json(val: &str) -> Result<bool, String>;
    fn from_str_radix_json(s: &str, radix: u32) -> Result<String, String>;
}

impl<T> NumJson for T
where
    T: Num + serde::Serialize + serde::de::DeserializeOwned + 'static,
    T::FromStrRadixErr: std::fmt::Display,
{
    fn type_name() -> &'static str {
        std::any::type_name::<T>()
    }

    fn add_json(a: &str, b: &str) -> Result<String, String> {
        let a_val: T = serde_json::from_str(a).map_err(|e| e.to_string())?;
        let b_val: T = serde_json::from_str(b).map_err(|e| e.to_string())?;
        let result = a_val + b_val;
        serde_json::to_string(&result).map_err(|e| e.to_string())
    }

    // ... implement all other methods similarly
}

#[reflect_trait(crate::NumJson)]
pub trait NumJsonTools: Sized + 'static {
    fn type_name() -> &'static str;
    fn add_json(a: &str, b: &str) -> Result<String, String>;
    fn sub_json(a: &str, b: &str) -> Result<String, String>;
    fn mul_json(a: &str, b: &str) -> Result<String, String>;
    fn div_json(a: &str, b: &str) -> Result<String, String>;
    fn rem_json(a: &str, b: &str) -> Result<String, String>;
    fn zero_json() -> String;
    fn one_json() -> String;
    fn is_zero_json(val: &str) -> Result<bool, String>;
    fn from_str_radix_json(s: &str, radix: u32) -> Result<String, String>;
}
```

**Total Num trait tools:** ~10

### 1.2 Integer Trait (Factory Pattern)

**Wrapper trait:**

```rust
pub trait IntegerJson: NumJson {
    fn div_floor_json(a: &str, b: &str) -> Result<String, String>;
    fn mod_floor_json(a: &str, b: &str) -> Result<String, String>;
    fn gcd_json(a: &str, b: &str) -> Result<String, String>;
    fn lcm_json(a: &str, b: &str) -> Result<String, String>;
    fn divides_json(a: &str, b: &str) -> Result<bool, String>;
    fn is_multiple_of_json(a: &str, b: &str) -> Result<bool, String>;
    fn is_even_json(val: &str) -> Result<bool, String>;
    fn is_odd_json(val: &str) -> Result<bool, String>;
    fn div_rem_json(a: &str, b: &str) -> Result<(String, String), String>;
    fn div_mod_floor_json(a: &str, b: &str) -> Result<(String, String), String>;
}

impl<T> IntegerJson for T
where
    T: Integer + serde::Serialize + serde::de::DeserializeOwned + 'static,
{
    fn div_floor_json(a: &str, b: &str) -> Result<String, String> {
        let a_val: T = serde_json::from_str(a)?;
        let b_val: T = serde_json::from_str(b)?;
        let result = a_val.div_floor(&b_val);
        Ok(serde_json::to_string(&result)?)
    }

    // ... all other methods
}

#[reflect_trait(crate::IntegerJson)]
pub trait IntegerJsonTools: NumJson {
    fn div_floor_json(a: &str, b: &str) -> Result<String, String>;
    fn mod_floor_json(a: &str, b: &str) -> Result<String, String>;
    fn gcd_json(a: &str, b: &str) -> Result<String, String>;
    fn lcm_json(a: &str, b: &str) -> Result<String, String>;
    // ... etc
}
```

**Total Integer trait tools:** ~10

### 1.3 Float Trait (Factory Pattern)

```rust
pub trait FloatJson: NumJson {
    fn nan_json() -> String;
    fn infinity_json() -> String;
    fn neg_infinity_json() -> String;
    fn neg_zero_json() -> String;
    fn min_value_json() -> String;
    fn min_positive_value_json() -> String;
    fn max_value_json() -> String;
    fn is_nan_json(val: &str) -> Result<bool, String>;
    fn is_infinite_json(val: &str) -> Result<bool, String>;
    fn is_finite_json(val: &str) -> Result<bool, String>;
    fn is_normal_json(val: &str) -> Result<bool, String>;
    fn classify_json(val: &str) -> Result<String, String>;
    fn floor_json(val: &str) -> Result<String, String>;
    fn ceil_json(val: &str) -> Result<String, String>;
    fn round_json(val: &str) -> Result<String, String>;
    fn trunc_json(val: &str) -> Result<String, String>;
    fn fract_json(val: &str) -> Result<String, String>;
    fn abs_json(val: &str) -> Result<String, String>;
    fn signum_json(val: &str) -> Result<String, String>;
    fn is_sign_positive_json(val: &str) -> Result<bool, String>;
    fn is_sign_negative_json(val: &str) -> Result<bool, String>;
    fn mul_add_json(a: &str, b: &str, c: &str) -> Result<String, String>;
    fn recip_json(val: &str) -> Result<String, String>;
    fn powi_json(base: &str, exp: i32) -> Result<String, String>;
    fn powf_json(base: &str, exp: &str) -> Result<String, String>;
    fn sqrt_json(val: &str) -> Result<String, String>;
    fn exp_json(val: &str) -> Result<String, String>;
    fn exp2_json(val: &str) -> Result<String, String>;
    fn ln_json(val: &str) -> Result<String, String>;
    fn log_json(val: &str, base: &str) -> Result<String, String>;
    fn log2_json(val: &str) -> Result<String, String>;
    fn log10_json(val: &str) -> Result<String, String>;
    fn max_json(a: &str, b: &str) -> Result<String, String>;
    fn min_json(a: &str, b: &str) -> Result<String, String>;
    fn abs_sub_json(a: &str, b: &str) -> Result<String, String>;
    fn cbrt_json(val: &str) -> Result<String, String>;
    fn hypot_json(x: &str, y: &str) -> Result<String, String>;
    fn sin_json(val: &str) -> Result<String, String>;
    fn cos_json(val: &str) -> Result<String, String>;
    fn tan_json(val: &str) -> Result<String, String>;
    fn asin_json(val: &str) -> Result<String, String>;
    fn acos_json(val: &str) -> Result<String, String>;
    fn atan_json(val: &str) -> Result<String, String>;
    fn atan2_json(y: &str, x: &str) -> Result<String, String>;
    fn sin_cos_json(val: &str) -> Result<(String, String), String>;
    fn exp_m1_json(val: &str) -> Result<String, String>;
    fn ln_1p_json(val: &str) -> Result<String, String>;
    fn sinh_json(val: &str) -> Result<String, String>;
    fn cosh_json(val: &str) -> Result<String, String>;
    fn tanh_json(val: &str) -> Result<String, String>;
    fn asinh_json(val: &str) -> Result<String, String>;
    fn acosh_json(val: &str) -> Result<String, String>;
    fn atanh_json(val: &str) -> Result<String, String>;
    fn integer_decode_json(val: &str) -> Result<(u64, i16, i8), String>;
}
```

**Total Float trait tools:** ~50

### 1.4 Additional Traits (Factory Pattern)

**Zero/One:**

```rust
pub trait ZeroJson: Sized + 'static {
    fn zero_json() -> String;
    fn is_zero_json(val: &str) -> Result<bool, String>;
    fn set_zero_json() -> String;
}

pub trait OneJson: Sized + 'static {
    fn one_json() -> String;
    fn is_one_json(val: &str) -> Result<bool, String>;
    fn set_one_json() -> String;
}
```

**Signed/Unsigned:**

```rust
pub trait SignedJson: NumJson {
    fn abs_json(val: &str) -> Result<String, String>;
    fn abs_sub_json(a: &str, b: &str) -> Result<String, String>;
    fn signum_json(val: &str) -> Result<String, String>;
    fn is_positive_json(val: &str) -> Result<bool, String>;
    fn is_negative_json(val: &str) -> Result<bool, String>;
}

pub trait UnsignedJson: NumJson {
    // No methods beyond Num
}
```

**Bounded:**

```rust
pub trait BoundedJson: Sized + 'static {
    fn min_value_json() -> String;
    fn max_value_json() -> String;
}
```

**Checked/Saturating/Wrapping Ops:**

```rust
pub trait CheckedAddJson: Sized + 'static {
    fn checked_add_json(a: &str, b: &str) -> Result<Option<String>, String>;
}

pub trait CheckedSubJson: Sized + 'static {
    fn checked_sub_json(a: &str, b: &str) -> Result<Option<String>, String>;
}

pub trait CheckedMulJson: Sized + 'static {
    fn checked_mul_json(a: &str, b: &str) -> Result<Option<String>, String>;
}

pub trait CheckedDivJson: Sized + 'static {
    fn checked_div_json(a: &str, b: &str) -> Result<Option<String>, String>;
}

// Similar for Saturating* and Wrapping*
```

**Total additional trait tools:** ~40

### 1.5 Fragment Tools (Generic Code Generation)

**Emit generic functions with trait bounds:**

```rust
#[elicit_tool(
    plugin = "num_fragments",
    name = "emit_function_generic",
    description = "Emit generic function with Num/Integer/Float bound",
    emit = Auto
)]
async fn emit_function_generic(p: EmitGenericFunctionParams) -> Result<CallToolResult, ErrorData> {
    // p.trait_bound: "Num", "Integer", "Float", etc.
    // p.operation: "add", "mul", "gcd", "sqrt", etc.
    // p.function_name: custom name

    let code = match p.operation.as_str() {
        "add" => format!(
            "fn {}<T: {}>(a: T, b: T) -> T {{ a + b }}",
            p.function_name, p.trait_bound
        ),
        "multiply" => format!(
            "fn {}<T: {}>(a: T, b: T) -> T {{ a * b }}",
            p.function_name, p.trait_bound
        ),
        "gcd" => format!(
            "fn {}<T: Integer>(a: T, b: T) -> T {{ a.gcd(&b) }}",
            p.function_name
        ),
        "sqrt" => format!(
            "fn {}<T: Float>(x: T) -> T {{ x.sqrt() }}",
            p.function_name
        ),
        _ => return Err(ErrorData::new("Unknown operation")),
    };

    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "num_fragments",
    name = "emit_type_alias",
    description = "Emit type alias with trait bound: type MyNum = impl Num;",
    emit = Auto
)]
async fn emit_type_alias(p: EmitTypeAliasParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        "type {} = impl {};",
        p.alias_name, p.trait_bound
    );
    Ok(CallToolResult::success(Content::text(code)))
}

#[elicit_tool(
    plugin = "num_fragments",
    name = "emit_struct_generic",
    description = "Emit struct with generic numeric field",
    emit = Auto
)]
async fn emit_struct_generic(p: EmitStructGenericParams) -> Result<CallToolResult, ErrorData> {
    let code = format!(
        r#"pub struct {}<T: {}> {{
    value: T,
}}

impl<T: {}> {} T> {{
    pub fn new(value: T) -> Self {{
        Self {{ value }}
    }}

    pub fn value(&self) -> &T {{
        &self.value
    }}
}}"#,
        p.struct_name, p.trait_bound,
        p.trait_bound, p.struct_name
    );
    Ok(CallToolResult::success(Content::text(code)))
}
```

**Total fragment tools:** ~20

**Total elicit_num_traits tools:** ~130

---

## Phase 2: elicit_num_bigint — Arbitrary Precision Integers

### 2.1 BigInt Type (Runtime + Dual)

**Custom serialization** (string-based, not tuple):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt(num_bigint::BigInt);

impl Serialize for BigInt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BigInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let inner = s.parse().map_err(serde::de::Error::custom)?;
        Ok(BigInt(inner))
    }
}
```

### 2.2 All BigInt Operations (Dual-Mode)

**Arithmetic (~20 tools):**

```rust
#[elicit_tool(plugin = "bigint", name = "add", emit = Auto)]
async fn bigint_add(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "sub", emit = Auto)]
async fn bigint_sub(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "mul", emit = Auto)]
async fn bigint_mul(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "div", emit = Auto)]
async fn bigint_div(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "rem", emit = Auto)]
async fn bigint_rem(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "div_rem", emit = Auto)]
async fn bigint_div_rem(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "pow", emit = Auto)]
async fn bigint_pow(p: BigIntPowParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "modpow", emit = Auto)]
async fn bigint_modpow(p: BigIntModPowParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "gcd", emit = Auto)]
async fn bigint_gcd(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "lcm", emit = Auto)]
async fn bigint_lcm(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "abs", emit = Auto)]
async fn bigint_abs(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "negate", emit = Auto)]
async fn bigint_negate(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "signum", emit = Auto)]
async fn bigint_signum(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 7 more arithmetic operations
```

**Bitwise (~20 tools):**

```rust
#[elicit_tool(plugin = "bigint", name = "bitand", emit = Auto)]
async fn bigint_bitand(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "bitor", emit = Auto)]
async fn bigint_bitor(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "bitxor", emit = Auto)]
async fn bigint_bitxor(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "shl", emit = Auto)]
async fn bigint_shl(p: BigIntShiftParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "shr", emit = Auto)]
async fn bigint_shr(p: BigIntShiftParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "bit_length", emit = Auto)]
async fn bigint_bit_length(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "count_ones", emit = Auto)]
async fn bigint_count_ones(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "count_zeros", emit = Auto)]
async fn bigint_count_zeros(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "trailing_zeros", emit = Auto)]
async fn bigint_trailing_zeros(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "trailing_ones", emit = Auto)]
async fn bigint_trailing_ones(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 10 more bitwise operations
```

**Comparison (~10 tools):**

```rust
#[elicit_tool(plugin = "bigint", name = "eq", emit = Auto)]
async fn bigint_eq(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "lt", emit = Auto)]
async fn bigint_lt(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "gt", emit = Auto)]
async fn bigint_gt(p: BigIntBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_zero", emit = Auto)]
async fn bigint_is_zero(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_positive", emit = Auto)]
async fn bigint_is_positive(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_negative", emit = Auto)]
async fn bigint_is_negative(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_even", emit = Auto)]
async fn bigint_is_even(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_odd", emit = Auto)]
async fn bigint_is_odd(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 2 more comparison operations
```

**Conversion (~20 tools):**

```rust
#[elicit_tool(plugin = "bigint", name = "from_i64", emit = Auto)]
async fn bigint_from_i64(p: FromI64Params) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "from_u64", emit = Auto)]
async fn bigint_from_u64(p: FromU64Params) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "from_bytes_be", emit = Auto)]
async fn bigint_from_bytes_be(p: FromBytesParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "from_bytes_le", emit = Auto)]
async fn bigint_from_bytes_le(p: FromBytesParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_i64", emit = Auto)]
async fn bigint_to_i64(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_u64", emit = Auto)]
async fn bigint_to_u64(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_f64", emit = Auto)]
async fn bigint_to_f64(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_bytes_be", emit = Auto)]
async fn bigint_to_bytes_be(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_bytes_le", emit = Auto)]
async fn bigint_to_bytes_le(p: BigIntUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "to_string_radix", emit = Auto)]
async fn bigint_to_string_radix(p: ToStringRadixParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "from_str_radix", emit = Auto)]
async fn bigint_from_str_radix(p: FromStrRadixParams) -> Result<CallToolResult, ErrorData>

// ... 9 more conversion operations
```

**Number theory (~30 tools):**

```rust
#[elicit_tool(plugin = "bigint", name = "factorial", emit = Auto)]
async fn bigint_factorial(p: FactorialParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "binomial", emit = Auto)]
async fn bigint_binomial(p: BinomialParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "fibonacci", emit = Auto)]
async fn bigint_fibonacci(p: FibonacciParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "lucas", emit = Auto)]
async fn bigint_lucas(p: LucasParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "mod_inverse", emit = Auto)]
async fn bigint_mod_inverse(p: ModInverseParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "extended_gcd", emit = Auto)]
async fn bigint_extended_gcd(p: ExtendedGcdParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "is_prime", emit = Auto)]
async fn bigint_is_prime(p: IsPrimeParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "bigint", name = "next_prime", emit = Auto)]
async fn bigint_next_prime(p: NextPrimeParams) -> Result<CallToolResult, ErrorData>

// ... 22 more number theory operations
```

**Workflow tools (~20 tools with propositions):**

```rust
#[elicit_tool(
    plugin = "bigint_workflow",
    name = "parse_decimal",
    description = "Parse decimal string. Establishes: ValidBigInt."
)]
async fn parse_decimal(p: ParseParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "assert_nonzero",
    description = "Verify value is non-zero. Establishes: NonZero."
)]
async fn assert_nonzero(p: AssertNonzeroParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "assert_positive",
    description = "Verify value is positive. Establishes: Positive."
)]
async fn assert_positive(p: AssertPositiveParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(
    plugin = "bigint_workflow",
    name = "assert_in_range",
    description = "Verify value is in range. Establishes: InRange<MIN, MAX>."
)]
async fn assert_in_range(p: AssertInRangeParams) -> Result<CallToolResult, ErrorData>

// ... 16 more workflow tools
```

**Total BigInt tools:** ~120

### 2.3 BigUint (Similar structure)

Same categories as BigInt, ~100 tools (no sign operations).

**Total elicit_num_bigint tools:** ~220

---

## Phase 3: elicit_num_rational — Exact Fractions

### 3.1 Rational Type (Runtime + Dual)

**Serialization:**

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Rational {
    pub numer: i64,
    pub denom: i64,  // Always > 0, always in reduced form
}

impl From<num_rational::Ratio<i64>> for Rational {
    fn from(r: num_rational::Ratio<i64>) -> Self {
        Rational {
            numer: *r.numer(),
            denom: *r.denom(),
        }
    }
}

impl From<Rational> for num_rational::Ratio<i64> {
    fn from(r: Rational) -> Self {
        num_rational::Ratio::new(r.numer, r.denom)
    }
}
```

### 3.2 All Rational Operations (Dual-Mode)

**Arithmetic (~15 tools):**

```rust
#[elicit_tool(plugin = "rational", name = "add", emit = Auto)]
async fn rational_add(p: RationalBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "sub", emit = Auto)]
async fn rational_sub(p: RationalBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "mul", emit = Auto)]
async fn rational_mul(p: RationalBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "div", emit = Auto)]
async fn rational_div(p: RationalBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "pow", emit = Auto)]
async fn rational_pow(p: RationalPowParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "recip", emit = Auto)]
async fn rational_recip(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "abs", emit = Auto)]
async fn rational_abs(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "negate", emit = Auto)]
async fn rational_negate(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 7 more arithmetic operations
```

**Rounding & Integer Parts (~10 tools):**

```rust
#[elicit_tool(plugin = "rational", name = "floor", emit = Auto)]
async fn rational_floor(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "ceil", emit = Auto)]
async fn rational_ceil(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "round", emit = Auto)]
async fn rational_round(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "trunc", emit = Auto)]
async fn rational_trunc(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "fract", emit = Auto)]
async fn rational_fract(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "to_mixed", emit = Auto)]
async fn rational_to_mixed(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "from_mixed", emit = Auto)]
async fn rational_from_mixed(p: FromMixedParams) -> Result<CallToolResult, ErrorData>

// ... 3 more rounding operations
```

**Properties (~10 tools):**

```rust
#[elicit_tool(plugin = "rational", name = "is_integer", emit = Auto)]
async fn rational_is_integer(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "is_positive", emit = Auto)]
async fn rational_is_positive(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "is_negative", emit = Auto)]
async fn rational_is_negative(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "is_reduced", emit = Auto)]
async fn rational_is_reduced(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "numerator", emit = Auto)]
async fn rational_numerator(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "denominator", emit = Auto)]
async fn rational_denominator(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 4 more property checks
```

**Conversion (~10 tools):**

```rust
#[elicit_tool(plugin = "rational", name = "to_f64", emit = Auto)]
async fn rational_to_f64(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "from_f64", emit = Auto)]
async fn rational_from_f64(p: FromF64Params) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "to_string", emit = Auto)]
async fn rational_to_string(p: RationalUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "rational", name = "to_decimal", emit = Auto)]
async fn rational_to_decimal(p: ToDecimalParams) -> Result<CallToolResult, ErrorData>

// ... 6 more conversion operations
```

**Workflow tools (~10 tools):**

```rust
#[elicit_tool(
    plugin = "rational_workflow",
    name = "create",
    description = "Create rational from numerator/denominator. Establishes: ValidRational."
)]
async fn create_rational(p: CreateRationalParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(
    plugin = "rational_workflow",
    name = "reduce",
    description = "Reduce to lowest terms. Establishes: ReducedForm."
)]
async fn reduce_rational(p: ReduceParams) -> Result<CallToolResult, ErrorData>

// ... 8 more workflow tools
```

**BigRational (Ratio<BigInt>):** ~40 additional tools for arbitrary-precision rationals.

**Total elicit_num_rational tools:** ~95

---

## Phase 4: elicit_num_complex — Complex Numbers

### 4.1 Complex Type (Runtime + Dual)

**Serialization:**

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl From<num_complex::Complex<f64>> for Complex {
    fn from(c: num_complex::Complex<f64>) -> Self {
        Complex { re: c.re, im: c.im }
    }
}

impl From<Complex> for num_complex::Complex<f64> {
    fn from(c: Complex) -> Self {
        num_complex::Complex::new(c.re, c.im)
    }
}
```

### 4.2 All Complex Operations (Dual-Mode)

**Arithmetic (~10 tools):**

```rust
#[elicit_tool(plugin = "complex", name = "add", emit = Auto)]
async fn complex_add(p: ComplexBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "sub", emit = Auto)]
async fn complex_sub(p: ComplexBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "mul", emit = Auto)]
async fn complex_mul(p: ComplexBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "div", emit = Auto)]
async fn complex_div(p: ComplexBinaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "conj", emit = Auto)]
async fn complex_conj(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "norm_sqr", emit = Auto)]
async fn complex_norm_sqr(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "norm", emit = Auto)]
async fn complex_norm(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 3 more arithmetic operations
```

**Polar/Cartesian (~10 tools):**

```rust
#[elicit_tool(plugin = "complex", name = "from_polar", emit = Auto)]
async fn complex_from_polar(p: FromPolarParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "to_polar", emit = Auto)]
async fn complex_to_polar(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "arg", emit = Auto)]
async fn complex_arg(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "scale", emit = Auto)]
async fn complex_scale(p: ScaleParams) -> Result<CallToolResult, ErrorData>

// ... 6 more polar/cartesian operations
```

**Transcendental Functions (~30 tools):**

```rust
#[elicit_tool(plugin = "complex", name = "exp", emit = Auto)]
async fn complex_exp(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "ln", emit = Auto)]
async fn complex_ln(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "sqrt", emit = Auto)]
async fn complex_sqrt(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "cbrt", emit = Auto)]
async fn complex_cbrt(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "powf", emit = Auto)]
async fn complex_powf(p: ComplexPowfParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "powi", emit = Auto)]
async fn complex_powi(p: ComplexPowiParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "sin", emit = Auto)]
async fn complex_sin(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "cos", emit = Auto)]
async fn complex_cos(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "tan", emit = Auto)]
async fn complex_tan(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "asin", emit = Auto)]
async fn complex_asin(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "acos", emit = Auto)]
async fn complex_acos(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "atan", emit = Auto)]
async fn complex_atan(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "sinh", emit = Auto)]
async fn complex_sinh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "cosh", emit = Auto)]
async fn complex_cosh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "tanh", emit = Auto)]
async fn complex_tanh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "asinh", emit = Auto)]
async fn complex_asinh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "acosh", emit = Auto)]
async fn complex_acosh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

#[elicit_tool(plugin = "complex", name = "atanh", emit = Auto)]
async fn complex_atanh(p: ComplexUnaryParams) -> Result<CallToolResult, ErrorData>

// ... 12 more transcendental functions
```

**Total elicit_num_complex tools:** ~80

---

## Estimated Tool Count

| Crate | Factory | Runtime | Dual-Mode | Fragment | Total |
|---|---|---|---|---|---|
| **elicit_num_traits** | 110 | 0 | 0 | 20 | 130 |
| **elicit_num_bigint** | 0 | 0 | 200 | 20 | 220 |
| **elicit_num_rational** | 0 | 0 | 85 | 10 | 95 |
| **elicit_num_complex** | 0 | 0 | 70 | 10 | 80 |
| **Total** | **110** | **0** | **355** | **60** | **525** |

**Breakdown:**

- Factory tools: trait methods exposed via wrapper traits
- Dual-mode tools: concrete type operations (runtime + emit)
- Fragment tools: generic code generation with trait bounds

---

## Implementation Timeline

**Week 1:** elicit_num_traits (Num, Integer, Float traits)
**Week 2:** elicit_num_traits (remaining traits + fragment tools)
**Week 3:** elicit_num_bigint (arithmetic + bitwise)
**Week 4:** elicit_num_bigint (conversion + number theory)
**Week 5:** elicit_num_bigint (workflow tools + BigUint)
**Week 6:** elicit_num_rational (Rational operations)
**Week 7:** elicit_num_rational (BigRational + workflow)
**Week 8:** elicit_num_complex (arithmetic + polar/cartesian)
**Week 9:** elicit_num_complex (transcendental functions)
**Week 10:** Integration testing + documentation

**Total:** 10 weeks for complete implementation

---

## Success Criteria

1. ✅ 100% of num-traits trait methods exposed via factory pattern
2. ✅ All concrete type operations are dual-mode (runtime + emit)
3. ✅ Fragment tools generate valid generic Rust code
4. ✅ Custom serialization works for BigInt (string-based)
5. ✅ Workflow tools properly track propositions
6. ✅ All 525 tools registered and tested
7. ✅ Comprehensive documentation with 50+ examples

---

## Key Innovations

1. **Trait Factory Pattern:** First shadow crate to expose trait methods (not just concrete types)
2. **Generic Code Generation:** Fragment tools emit generic functions with trait bounds
3. **Triple Harvest:** Traits (factory) + Concrete types (dual-mode) + Generic code (fragments) = 100% coverage
4. **String Serialization:** BigInt serializes as decimal strings (cross-language compatibility)
5. **Workflow Propositions:** Type-level proofs for numeric properties (NonZero, Positive, InRange, etc.)
6. **Zero Compromise:** Traits AND concrete types, compile-time AND runtime, all exposed
