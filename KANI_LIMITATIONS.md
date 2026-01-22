# Kani Verification Limitations

This document tracks known limitations and workarounds for Kani verification in the elicitation crate.

## memchr Infinite Loop Issue

**Problem:** Kani's bounded model checker hits unbounded loops in `core::slice::memchr::memchr_aligned::runtime` when symbolically executing string operations.

**Root Cause:** String operations (`to_string()`, `parse_str()`, etc.) create deep execution paths through:
1. Display/FromStr trait implementations
2. Formatting infrastructure
3. memchr byte searching loops (100-500+ iterations)

**Solutions Applied:**

### 1. Use Concrete Strings Only
```rust
// ❌ BAD: Symbolic construction
let len: usize = kani::any();
kani::assume(len < 10);
let mut s = String::new();
for _ in 0..len {
    s.push('a');  // Triggers memchr loops
}

// ✅ GOOD: Concrete strings
let empty = String::new();
let non_empty = String::from("a");
```

### 2. Replace .to_string() with String::from()
```rust
// ❌ BAD: Display trait formatting
map.insert(1, "value".to_string());  // Complex formatting paths

// ✅ GOOD: Direct conversion
map.insert(1, String::from("a"));  // Simple memcpy
```

### 3. Replace parse_str() with Direct Construction
```rust
// ❌ BAD: String parsing
let uuid = Uuid::parse_str("550e8400-...").unwrap();  // 400+ loop iterations

// ✅ GOOD: Byte-level construction  
let uuid_bytes = [0x55, 0x0e, ...];
let uuid = Uuid::from_bytes(uuid_bytes);  // Direct construction
```

### 4. Add Explicit Unwind Bounds
```rust
#[kani::proof]
#[kani::unwind(10)]  // Bound all loops to 10 iterations max
fn verify_string_operation() {
    // Even with concrete strings, some operations need bounds
}
```

## Performance Guidelines

| Operation | Max Unwind | Notes |
|-----------|-----------|-------|
| Concrete String::from() | 10 | Simple operations |
| HashMap/BTreeMap with strings | 10 | Insert/lookup |
| UUID from_bytes | 500 | Complex but bounded |
| Regex compilation | 1 | Use string literals only |
| URL parsing | 1 | Use string literals only |

## Types Requiring Special Handling

### ✅ Safe for Kani (with bounds)
- Integer contract types (I8Positive, etc.)
- Boolean contracts
- Simple collection contracts (Vec, Array with primitive types)
- Duration contracts
- Char contracts

### ⚠️ Limited Verification
- **String contracts**: Use concrete strings only
- **UUID contracts**: Use from_bytes, can't verify parse validation
- **URL contracts**: Use string literals, can't verify complex parsing
- **Regex contracts**: Use simple patterns, can't verify full regex engine

### ❌ Not Verifiable with Kani
- Complex regex patterns with backtracking
- Symbolic string operations (push, append, etc.)
- Network I/O (IpAddr parsing)
- File path validation with symbolic components

## Trade-offs

**What We Verify:**
- Contract wrapper logic (new/get/into_inner)
- Basic invariants (non-empty, positive, etc.)
- Accessor correctness
- "Trenchcoat" pattern (value preservation)

**What We Don't Verify:**
- Complex parser correctness (UUID version bits, regex syntax)
- All possible string values
- Performance characteristics
- Symbolic execution of string algorithms

## Alternative Verifiers

For string-heavy verification, consider:
- **Creusot**: Better string support via Why3
- **Prusti**: Can handle more complex string operations
- **Property-based testing**: QuickCheck/proptest for string generation

## Future Work

- [ ] Investigate Kani's `--restrict-vtable` option for string types
- [ ] Consider stub implementations for string parsing
- [ ] Explore Kani's `kani::assume_init()` for bounded strings
- [ ] Add proptest harnesses as complement to Kani proofs
