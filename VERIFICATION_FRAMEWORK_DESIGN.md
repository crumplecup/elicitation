# Formal Verification Framework Design

## Vision

Generic contract interface supporting **multiple formal verification tools** in the Rust ecosystem, similar to our datetime support strategy.

## Verification Tools Landscape

| Tool | Technique | Coverage | Specification | Unsafe? | Status |
|------|-----------|----------|---------------|---------|--------|
| **Kani** | Model checking | Safe + Some Unsafe | Rust attributes | Some | ✅ Mature |
| **Creusot** | Deductive (Why3) | Safe + Some Unsafe | Prophecies/contracts | Yes | ✅ Mature |
| **Prusti** | Separation logic | Safe only | Custom annotations | No | ✅ Mature |
| **Verus** | SMT + linearity | Low-level + advanced | Rust-native modes | Yes | ✅ Mature |
| **Flux** | Liquid types | Experimental | Type refinements | No | 🚧 Research |

## Architecture

### Core: Tool-Agnostic Contract Trait

```rust
/// Generic contract for formal verification.
///
/// Implementations bridge to specific verification tools (Kani, Creusot, etc).
pub trait Contract {
    type Input: Elicitation;
    type Output: Elicitation;

    /// Precondition: What must hold before execution
    fn requires(input: &Self::Input) -> bool;

    /// Postcondition: What must hold after execution
    fn ensures(input: &Self::Input, output: &Self::Output) -> bool;

    /// Invariant: What must hold throughout execution
    fn invariant(&self) -> bool { true }
}
```

### Tool-Specific Adapters (Feature-Gated)

```
src/verification/
├── mod.rs           # Core Contract trait
├── kani.rs          # Kani adapter (#[cfg(feature = "verify-kani")])
├── creusot.rs       # Creusot adapter (#[cfg(feature = "verify-creusot")])
├── prusti.rs        # Prusti adapter (#[cfg(feature = "verify-prusti")])
├── verus.rs         # Verus adapter (#[cfg(feature = "verify-verus")])
└── flux.rs          # Flux adapter (future)
```

### Feature Flags

```toml
[features]
# Core verification (just the trait)
verification = []

# Tool-specific verification
verify-kani = ["verification"]
verify-creusot = ["verification"]
verify-prusti = ["verification"]
verify-verus = ["verification"]
verify-flux = ["verification"]

# Convenience: enable all verifiers
verify-all = ["verify-kani", "verify-creusot", "verify-prusti", "verify-verus"]
```

## Implementation Strategy

### Phase 1: Refactor Kani (Current)
1. ✅ Rename `KaniContract` → `Contract` (generic)
2. ✅ Move Kani-specific code to `verification/kani.rs`
3. ✅ Add `verify-kani` feature flag
4. ✅ Update examples to use generic `Contract` trait

### Phase 2: Add Creusot Support
1. Research Creusot prophecy/contract syntax
2. Implement `Contract` → Creusot adapter
3. Add verification harnesses using Why3
4. Document Creusot-specific requirements

### Phase 3: Add Prusti Support
1. Research Prusti separation logic annotations
2. Implement `Contract` → Prusti adapter
3. Add verification with Prusti annotations
4. Document safe-only limitations

### Phase 4: Add Verus Support
1. Research Verus mode system (spec/exec/proof)
2. Implement `Contract` → Verus adapter
3. Add SMT-based verification harnesses
4. Document low-level verification capabilities

### Phase 5: Community Contributions
- Document extension points
- Provide template for new verifier adapters
- Accept PRs for Flux, F*, Coq extraction, etc.

## Tool Trait Pattern

Each tool gets an adapter trait:

```rust
#[cfg(feature = "verify-kani")]
pub trait KaniVerifiable: Contract {
    /// Generate Kani verification harness
    fn kani_proof_harness() -> impl Fn();
}

#[cfg(feature = "verify-creusot")]
pub trait CreusotVerifiable: Contract {
    /// Generate Creusot prophecy annotations
    fn creusot_prophecy() -> String;
}

#[cfg(feature = "verify-prusti")]
pub trait PrustiVerifiable: Contract {
    /// Generate Prusti spec comments
    fn prusti_spec() -> String;
}
```

## Example: Multi-Tool Contract

```rust
use elicitation::verification::Contract;

struct ValidateEmail;

impl Contract for ValidateEmail {
    type Input = String;
    type Output = String;

    fn requires(input: &String) -> bool {
        input.contains('@') && input.len() > 2
    }

    fn ensures(_input: &String, output: &String) -> bool {
        output.contains('@')
    }
}

// Kani verification
#[cfg(feature = "verify-kani")]
mod kani_tests {
    use super::*;
    use elicitation::verification::kani::KaniVerifiable;

    #[kani::proof]
    fn verify_with_kani() {
        // Kani-specific harness
    }
}

// Creusot verification
#[cfg(feature = "verify-creusot")]
mod creusot_tests {
    use super::*;
    use elicitation::verification::creusot::CreusotVerifiable;

    #[creusot::prophecy]
    fn verify_with_creusot() {
        // Creusot-specific harness
    }
}
```

## Benefits

1. **Tool Choice**: Users pick verifier based on needs (safe vs unsafe, performance, etc)
2. **Future-Proof**: Easy to add new tools as ecosystem evolves
3. **Incremental Adoption**: Start with Kani, add Creusot for unsafe code
4. **Community**: Accept verifier adapters from tool maintainers
5. **Consistency**: Same `Contract` trait across all tools

## Migration Path

### Current Kani Code
```rust
impl KaniContract for MyTool { ... }
```

### After Refactor
```rust
impl Contract for MyTool { ... }

#[cfg(feature = "verify-kani")]
impl KaniVerifiable for MyTool { ... }
```

## Documentation Structure

```
docs/verification/
├── overview.md           # High-level architecture
├── choosing-a-tool.md    # Decision matrix
├── kani-guide.md         # Kani-specific guide
├── creusot-guide.md      # Creusot-specific guide
├── prusti-guide.md       # Prusti-specific guide
├── verus-guide.md        # Verus-specific guide
└── extending.md          # Adding new verifiers
```

## Success Criteria

- [ ] Generic `Contract` trait works with all tools
- [ ] At least 2 verifiers fully supported (Kani ✅ + Creusot)
- [ ] Examples demonstrate multi-tool verification
- [ ] Clear migration guide from Kani-only design
- [ ] Extension points for community verifiers

## Next Steps

1. Design approval from user
2. Refactor existing Kani code
3. Add Creusot support (proof-of-concept)
4. Update documentation
5. Publish with multi-verifier support
