# Creusot 0.2 → 0.9 Migration Guide

## Summary

Creusot was working with version 0.2, but 0.8+ changed significant syntax. Version 0.9 is the current stable release.

## Key Changes

### 1. Dependency Changes

**Before (0.2)**:
```toml
creusot-contracts = "0.2"
```

**After (0.9)**:
```toml
creusot-std = "0.9.0"
```

Note: `creusot-std` provides all contracts (`#[requires]`, `#[ensures]`) in 0.9. No need for `creusot-contracts`.

### 2. Import Changes

**Before**:
```rust
use creusot_contracts::prelude::*;
```

**After**:
```rust
// In lib.rs:
#[cfg(feature = "verify-creusot")]
use creusot_std::prelude::*;

// In proof files: imports come from lib.rs
```

### 3. Logical Value Syntax (`@` operator)

The `@` operator converts runtime values to logical (specification) values.

**Integers, Chars, Strings - Use `@`**:
```rust
// Before (0.2)
#[requires(value > 0)]
#[ensures(result.get() == value)]

// After (0.9)
#[requires(value@ > 0@)]
#[ensures(result.get() == value@)]
```

**Booleans - No `@` needed**:
```rust
// Correct in both versions
#[requires(value)]
#[requires(!value)]
```

### 4. Result Type Postconditions

**Before (0.2)** - Used `^result` for final value:
```rust
#[ensures((^result).is_ok() ==> (^result).as_ref().unwrap().get() == value)]
```

**After (0.9)** - Use `match` expressions:
```rust
#[ensures(match result { Ok(v) => v.get() == value@, Err(_) => false })]
```

Or with quantifiers (see binary_search example):
```rust
#[ensures(forall<x:usize> result == Ok(x) ==> arr[x@] == elem)]
#[ensures(forall<x:usize> result == Err(x) ==> /* condition */)]
```

### 5. Floating Point Limitations

Floats don't implement `OrdLogic` in Creusot 0.9:
```rust
// ❌ This doesn't work:
#[requires(value > 0.0)]

// ✅ Alternative: Check for specific properties
#[requires(value.is_finite())]
```

## Migration Progress

### ✅ Completed
- **bools.rs**: 4 proofs - All migrated
- **integers.rs**: ~130 proofs - All migrated

### 🔧 In Progress
- **floats.rs**: Needs alternative approach (no `OrdLogic`)
- **strings.rs**: Likely needs `@` for length checks
- **chars.rs**: Should be straightforward with `@`
- **collections.rs**: Complex, will need careful migration
- **mechanisms.rs**: Needs `@` operator
- **durations.rs**: Needs `@` operator
- **regexes.rs**: Unknown complexity
- **urls.rs**: Unknown complexity

## Example: Complete Migration

**Before (integers.rs with 0.2)**:
```rust
#[requires(value > 0)]
#[ensures(result.is_ok())]
pub fn verify_i8_positive_valid(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

#[requires(value > 0)]
#[ensures((^result).is_ok() ==> (^result).as_ref().unwrap().get() == value)]
pub fn verify_i8_positive_accessor(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}
```

**After (integers.rs with 0.9)**:
```rust
#[requires(value@ > 0@)]
#[ensures(result.is_ok())]
pub fn verify_i8_positive_valid(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}

#[requires(value@ > 0@)]
#[ensures(match result { Ok(v) => v.get() == value@, Err(_) => false })]
pub fn verify_i8_positive_accessor(value: i8) -> Result<I8Positive, ValidationError> {
    I8Positive::new(value)
}
```

## Testing

```bash
# Build with Creusot feature
cargo creusot -- --features verify-creusot

# Run full proof verification (once compilation works)
cargo creusot prove -- --features verify-creusot
```

## Status: January 2026

- **Compilation**: 80 errors remaining (down from 85)
- **bools.rs**: ✅ Compiling
- **integers.rs**: ✅ Compiling
- **Other files**: Need migration following same patterns
