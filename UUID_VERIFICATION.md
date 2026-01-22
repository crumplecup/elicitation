# UUID Formal Verification

This document describes the UUID byte-level validation architecture and formal verification approach.

## Architecture: Layered Validation

Following the same pattern as UTF-8 validation:

```text
Layer 1: [u8; 16]        - Raw bytes (Kani's native domain)
Layer 2: UuidBytes       - RFC 4122 variant validation
Layer 3: UuidV4Bytes     - Version 4 specific (random)
         UuidV7Bytes     - Version 7 specific (timestamp + random)
Layer 4: UuidV4/UuidV7   - High-level contract types (wrap uuid::Uuid)
```

## RFC 4122 UUID Structure

```
16 bytes (128 bits): xxxxxxxx-xxxx-Mxxx-Nxxx-xxxxxxxxxxxx

Byte 6, bits 4-7: VERSION (M)
  - 0001 (1) = V1 (timestamp + MAC)
  - 0010 (2) = V2 (DCE Security)
  - 0011 (3) = V3 (MD5 hash)
  - 0100 (4) = V4 (random)
  - 0101 (5) = V5 (SHA-1 hash)
  - 0111 (7) = V7 (timestamp + random)

Byte 8, bits 6-7: VARIANT (N)
  - 10xx = RFC 4122 (standard)
  - 0xxx = NCS backward compatibility
  - 110x = Microsoft GUID
  - 111x = Reserved

V4: All other bits SHOULD be random
V7: Bytes 0-5 = unix_ts_ms (48 bits), rest random
```

## What We Prove

### Variant Validation (4 proofs)

1. **Valid RFC 4122 accepted**: `10xx` pattern in byte 8 bits 6-7 → success
2. **NCS variant rejected**: `0xxx` pattern → error
3. **Microsoft variant rejected**: `110x` pattern → error
4. **Reserved variant rejected**: `111x` pattern → error

### Version Detection (1 proof)

5. **Version extraction**: All 16 possible version values correctly extracted from byte 6 bits 4-7

### UUID V4 Validation (3 proofs)

6. **Valid V4 construction**: Version=4 + Variant=10xx → UuidV4Bytes success
7. **Wrong version rejected**: Version≠4 → UuidV4Bytes error
8. **Invalid variant rejected**: Version=4 + Variant≠10xx → UuidV4Bytes error

### UUID V7 Validation (3 proofs)

9. **Valid V7 construction**: Version=7 + Variant=10xx → UuidV7Bytes success
10. **Wrong version rejected**: Version≠7 → UuidV7Bytes error
11. **Timestamp extraction**: First 48 bits correctly extracted as big-endian milliseconds

### Round-Trip Properties (3 proofs)

12. **UuidBytes roundtrip**: `new(bytes)` → `bytes()` preserves bytes
13. **UuidV4Bytes roundtrip**: Bytes preserved through construction/extraction
14. **UuidV7Bytes roundtrip**: Bytes preserved through construction/extraction

## Verification Results

All 14 proofs complete in ~2 seconds each (fast symbolic verification):

```bash
# Run all UUID Kani proofs
cargo kani --features verify-kani,uuid --default-unwind 20

# Run specific proof
cargo kani --harness verify_v4_valid_construction --features verify-kani,uuid --default-unwind 20
```

### Why UUID Proofs Are Fast

Unlike UTF-8 (which has variable-length sequences and loops), UUID validation is:

1. **Fixed size**: Always 16 bytes
2. **Bit-level operations**: Just masks and shifts, no loops
3. **Small state space**: Version (16 options) × Variant (4 patterns) = 64 combinations
4. **No string operations**: No memchr loops or parsing

Result: **Complete symbolic verification in seconds** (not days like UTF-8 3/4-byte proofs).

## Comparison to UTF-8 Verification

| Aspect | UTF-8 | UUID |
|--------|-------|------|
| Size | Variable (1-4 bytes) | Fixed (16 bytes) |
| Operations | Multi-byte sequences, loops | Bit masks, no loops |
| Problem Space | 3,968 → 786K combinations | 64 combinations |
| Proof Time | Hours to weeks | Seconds |
| Complexity | High (encoding rules) | Low (bit patterns) |

## Testing

```bash
# Unit tests
cargo test --features uuid --lib uuid_bytes

# Kani proofs
cargo kani --features verify-kani,uuid --default-unwind 20

# Or use justfile
just verify-kani
```

## Implementation Notes

### Why Not Parse From String?

Current high-level types (`UuidV4`, `UuidV7`) wrap `uuid::Uuid` and trust its parsing. The byte-level foundation (`UuidV4Bytes`, `UuidV7Bytes`) enables:

1. **Provable validation**: We can formally verify the validation logic
2. **Zero trust**: Don't rely on external crate correctness
3. **Composition**: Build higher-level contracts on proven foundation

Future: Refactor high-level types to use byte foundation (like StringNonEmpty uses Utf8Bytes).

### Why No Trait-Based Validation?

Initial design considered traits (`HasValidVersion`, `HasValidVariant`), but:

- Free functions (`has_valid_variant()`, `is_valid_v4()`) are simpler
- No trait overhead for single-implementation types
- Better Kani support (fewer dispatch paths)

Traits would make sense if we had many variant/version combinations.

## References

- RFC 4122: https://www.rfc-editor.org/rfc/rfc4122
- UUID V7 Draft: https://www.ietf.org/archive/id/draft-peabody-dispatch-new-uuid-format-04.html
- Kani: https://github.com/model-checking/kani
