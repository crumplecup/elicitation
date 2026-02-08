# Generator Types - Kani Proof Checklist

## Overview

This document tracks Kani verification coverage for all Generator types in the elicitation crate. Generators enable alternate construction paths (like `Instant::now()`, `Uuid::new_v4()`, etc.) and require formal verification to ensure correctness.

**Goal:** Complete Kani proof coverage for all generator types, integrated into the verification harness and CSV tracking system.

---

## Proof Status Legend

- ✅ **Complete** - Kani proofs exist and pass
- 🚧 **Partial** - Some proofs exist, coverage incomplete
- ❌ **Missing** - No Kani proofs
- 🔒 **Feature-gated** - Behind feature flag, needs conditional verification

---

## Core Generators (No Feature Gates)

### ✅ std::time::Duration
- **File:** `crates/elicitation/src/primitives/duration.rs`
- **Generator:** `DurationGenerator`
- **Modes:**
  - `Zero` - Duration::ZERO
  - `Custom(secs, nanos)` - Arbitrary duration
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/durations.rs`
- **Status:** ✅ Complete
- **Notes:** Existing proofs cover all modes

### ✅ std::time::SystemTime
- **File:** `crates/elicitation/src/primitives/systemtime.rs`
- **Generator:** `SystemTimeGenerator`
- **Modes:**
  - `UnixEpoch` - SystemTime::UNIX_EPOCH
  - `Now` - SystemTime::now()
  - `Offset { seconds, nanos }` - Custom offset from reference
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/systemtime.rs`
- **Status:** ✅ Complete
- **Notes:** Existing proofs cover all modes including offset arithmetic

### ✅ uuid::Uuid  
- **File:** `crates/elicitation/src/primitives/uuid.rs`
- **Generator:** `UuidGenerator`
- **Modes:**
  - `V4` - Uuid::new_v4() (random)
  - `Nil` - Uuid::nil()
  - `Max` - Uuid::max()
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/uuid_bytes.rs` (lines 283-387)
- **Status:** ✅ Complete
- **Proofs Added (6 total):**
  - `verify_uuid_generator_nil()` - Nil mode produces all-zero UUID
  - `verify_uuid_generator_max()` - Max mode produces all-ones UUID  
  - `verify_uuid_generator_v4_format()` - V4 mode produces correct version/variant bits
  - `verify_uuid_generator_mode_preserved()` - Generator stores mode correctly
  - `verify_uuid_generator_nil_consistent()` - Nil mode is deterministic
  - `verify_uuid_generator_max_consistent()` - Max mode is deterministic
- **Castle on Cloud Pattern:**
  - Trusts `uuid::Uuid::nil()`, `Uuid::max()`, `Uuid::new_v4()` correctness
  - Verifies our wrapper logic calls them correctly
  - Verifies generated UUIDs have correct format (version 4, RFC 4122 variant)

### ✅ std::io::Error (Core - Complete)
- **File:** `crates/elicitation/src/primitives/errors.rs`
- **Generator:** `IoErrorGenerator`
- **Modes:** 10 ErrorKind variants (NotFound, PermissionDenied, ConnectionRefused, etc.)
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/errors.rs`
- **Status:** ✅ Complete
- **Proofs Added (9 total):**
  - `verify_ioerror_generator_not_found()` - NotFound ErrorKind
  - `verify_ioerror_generator_permission_denied()` - PermissionDenied ErrorKind
  - `verify_ioerror_generator_connection_refused()` - ConnectionRefused ErrorKind
  - `verify_ioerror_generator_broken_pipe()` - BrokenPipe ErrorKind
  - `verify_ioerror_generator_timed_out()` - TimedOut ErrorKind
  - `verify_ioerror_generator_other()` - Other ErrorKind
  - `verify_ioerror_generator_mode_preserved()` - Mode storage correctness
  - `verify_ioerror_mode_helpers()` - Helper method correctness
  - `verify_ioerror_all_kinds_map_correctly()` - All 10 ErrorKind mappings
- **Verification Time:** 2-32s per proof
- **Castle on Cloud Pattern:**
  - Trusts `io::Error::new()` correctness
  - Verifies wrapper calls it with correct ErrorKind
  - Verifies mode → ErrorKind mapping is complete

### ✅ serde_json::Error (Feature-Gated - Complete with Symbolic Verification)
- **File:** `crates/elicitation/src/primitives/errors.rs`
- **Generator:** `JsonErrorGenerator`
- **Modes:** 5 error types (SyntaxError, EofWhileParsing, InvalidNumber, InvalidEscape, InvalidUnicode)
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/errors.rs`
- **Status:** ✅ Complete (symbolic verification)
- **Proofs Added (2 total):**
  - `verify_jsonerror_generator_mode_preserved()` - Mode storage correctness
  - `verify_jsonerror_string_mapping()` - All 5 modes → invalid JSON strings
- **Verification Time:** 0.03-0.7s per proof
- **Castle on Cloud + Symbolic Gate Pattern:**
  - Trusts `serde_json::from_str()` correctly parses/fails JSON
  - Verifies mode → string mapping without calling serde_json (avoids inline asm)
  - Verifies wrapper stores and retrieves mode correctly
  - Does NOT call `generate()` to avoid CPU detection code (inline assembly limitation)
- **What We Prove:** Our wrapper logic is correct (mode storage, string selection)
- **What We Trust:** serde_json fails on our invalid strings (validated by unit tests)

### ❌ Unit Structs (Validator, Formatter, Parser)
- **File:** `crates/elicitation/src/primitives/unit_structs.rs`
- **Generators:** `Validator`, `Formatter`, `Parser`
- **Modes:** 
  - All generators just return `Self` (identity)
- **Kani Proofs:** None needed
- **Status:** N/A (Trivial - identity function)
- **Notes:** Generators just return `Self { }`, no logic to verify

---

## Feature-Gated Generators

### ✅ chrono::DateTime<Utc> (feature = "chrono")
- **File:** `crates/elicitation/src/datetime_chrono.rs`
- **Generator:** `DateTimeUtcGenerator`
- **Modes:**
  - `Now` - Utc::now()
  - `UnixEpoch` - DateTime::UNIX_EPOCH
  - `Offset { seconds }` - Custom offset from reference
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/datetime_chrono.rs`
- **Status:** ✅ Complete
- **Proofs Added (6 total):**
  - `verify_datetime_utc_generator_unix_epoch()` - UnixEpoch mode
  - `verify_datetime_utc_generator_offset_positive()` - Positive offset calculation
  - `verify_datetime_utc_generator_offset_negative()` - Negative offset calculation
  - `verify_datetime_utc_generator_offset_zero()` - Zero offset identity
  - `verify_datetime_utc_generator_mode_preserved()` - Mode storage correctness
  - `verify_datetime_utc_generator_reference_preserved()` - Reference storage correctness
- **Verification Time:** 0.5-1.5s per proof
- **Castle on Cloud Pattern:**
  - Trusts `chrono::Utc::now()`, `DateTime::UNIX_EPOCH` correctness
  - Trusts chrono's Duration and addition/subtraction
  - Verifies wrapper calls correct functions for each mode
  - Verifies offset arithmetic direction (positive vs negative)
  - Does NOT verify Now mode (non-deterministic, trust chrono)

### ✅ chrono::NaiveDateTime (feature = "chrono")
- **File:** `crates/elicitation/src/datetime_chrono.rs`
- **Generator:** `NaiveDateTimeGenerator`
- **Modes:**
  - `Now` - Utc::now().naive_utc()
  - `UnixEpoch` - NaiveDateTime::UNIX_EPOCH
  - `Offset { seconds }` - Custom offset from reference
- **Kani Proofs:** `crates/elicitation/src/verification/types/kani_proofs/datetime_chrono.rs`
- **Status:** ✅ Complete
- **Proofs Added (6 total):**
  - `verify_naive_datetime_generator_unix_epoch()` - UnixEpoch mode
  - `verify_naive_datetime_generator_offset_positive()` - Positive offset calculation
  - `verify_naive_datetime_generator_offset_negative()` - Negative offset calculation
  - `verify_naive_datetime_generator_offset_zero()` - Zero offset identity
  - `verify_naive_datetime_generator_mode_preserved()` - Mode storage correctness
  - `verify_naive_datetime_generator_reference_preserved()` - Reference storage correctness
- **Verification Time:** 0.5-1.5s per proof
- **Castle on Cloud Pattern:**
  - Same pattern as DateTime<Utc>
  - Trusts chrono's naive datetime operations
  - Verifies wrapper logic only

### 🔒 time::Instant (feature = "time")
- **File:** `crates/elicitation/src/datetime_time.rs`
- **Generator:** `InstantGenerator`
- **Modes:**
  - `Now` - Instant::now()
  - `Custom(duration)` - From duration since epoch
- **Kani Proofs:** None
- **Status:** ❌ Missing
- **Missing:**
  - All generator proofs
  - Verification of instant construction
  - Monotonicity checks (if applicable)
- **Action Items:**
  - Create `crates/elicitation/src/verification/types/kani_proofs/datetime_time.rs`
  - Add `verify_instant_generator_custom()` - Duration-based construction
  - Add feature-gated tests to verification harness
- **Notes:** time::Instant is different from std::time::Instant

### 🔒 time::OffsetDateTime (feature = "time")
- **File:** `crates/elicitation/src/datetime_time.rs`
- **Generator:** `OffsetDateTimeGenerator`
- **Modes:**
  - `Now` - OffsetDateTime::now_utc()
  - `Custom(timestamp)` - From UNIX timestamp
- **Kani Proofs:** None
- **Status:** ❌ Missing
- **Missing:**
  - All generator proofs
  - Verification of offset datetime construction
  - UTC offset handling
- **Action Items:**
  - Add to `datetime_time.rs` proof file
  - Add `verify_offset_datetime_generator_custom()` - Timestamp conversion
  - Add `verify_offset_datetime_utc_offset()` - UTC offset correctness
  - Add feature-gated tests to verification harness

### 🔒 jiff::Timestamp (feature = "jiff")
- **File:** `crates/elicitation/src/datetime_jiff.rs`
- **Generator:** `TimestampGenerator`
- **Modes:**
  - `Now` - Timestamp::now()
  - `Custom(timestamp)` - From UNIX timestamp
- **Kani Proofs:** None
- **Status:** ❌ Missing
- **Missing:**
  - All generator proofs
  - Verification of jiff timestamp construction
  - Precision/range checks
- **Action Items:**
  - Create `crates/elicitation/src/verification/types/kani_proofs/datetime_jiff.rs`
  - Add `verify_timestamp_generator_custom()` - Timestamp conversion
  - Add `verify_timestamp_generator_precision()` - Precision checks
  - Add feature-gated tests to verification harness

### 🔒 serde_json::Error (feature = "serde_json")
- **File:** `crates/elicitation/src/primitives/errors.rs`
- **Generator:** `JsonErrorGenerator`
- **Modes:**
  - Various JSON parsing error types
- **Kani Proofs:** None
- **Status:** ❌ Missing
- **Missing:**
  - All generator proofs
  - Verification of JSON error construction
- **Action Items:**
  - Add to `errors.rs` proof file (feature-gated section)
  - Add `verify_json_error_generator()` - Error construction
  - Add feature-gated tests to verification harness
- **Notes:** Low priority - errors are simple constructors

---

## Summary Statistics

| Category | Total | Complete | Partial | Missing | N/A |
|----------|-------|----------|---------|---------|-----|
| **Core Generators** | 5 | 4 | 0 | 1 | 0 |
| **Feature-Gated** | 6 | 3 | 0 | 3 | 0 |
| **Unit Structs** | 3 | 0 | 0 | 0 | 3 |
| **TOTAL** | 14 | 7 | 0 | 4 | 3 |

**Coverage:** 7/11 types complete (63.6%)  
**Work Remaining:** 4 types need proofs (3 datetime, 1 core)

**Phase 3 Complete:** ✅ chrono types (DateTime<Utc> + NaiveDateTime) - 12 proofs added

---

## Action Plan

### Phase 1: Complete Partial Coverage
1. **uuid::Uuid** - Complete generator proofs
   - Add Nil mode proof
   - Add Custom mode proof
   - Add V4 format validation
   - Integrate with existing uuid_bytes proofs

### Phase 2: Core Generators
2. **std::io::Error** - Add generator proofs
   - Create errors.rs proof file
   - Add ErrorKind verification
   - Add message preservation checks

### Phase 3: Feature-Gated DateTime Generators
3. **chrono types** - Add proofs for DateTime<Utc> and NaiveDateTime
   - Create datetime_chrono.rs proof file
   - Add timestamp conversion proofs
   - Add boundary checks

4. **time types** - Add proofs for Instant and OffsetDateTime
   - Create datetime_time.rs proof file
   - Add duration-based construction proofs
   - Add offset handling proofs

5. **jiff types** - Add proofs for Timestamp
   - Create datetime_jiff.rs proof file
   - Add timestamp conversion proofs
   - Add precision checks

6. **serde_json::Error** - Add generator proofs (low priority)
   - Add to errors.rs proof file
   - Feature-gated section

### Phase 4: Verification Harness Integration
7. **Update verification harness**
   - Add all generator types to harness
   - Add feature-gated conditional compilation
   - Ensure all proofs run in CI

8. **CSV Tracking**
   - Add generator types to verification CSV
   - Add feature flag column
   - Track proof status
   - Generate proof records

---

## CSV Tracking Schema

Proposed columns for generator verification tracking:

```csv
type_name,generator_name,feature,has_kani_proof,proof_file,modes_verified,proof_count,status,last_verified
Duration,DurationGenerator,,true,durations.rs,"Zero,Custom",4,complete,2026-02-08
SystemTime,SystemTimeGenerator,,true,systemtime.rs,"UnixEpoch,Now,Offset",6,complete,2026-02-08
Uuid,UuidGenerator,,partial,uuid_bytes.rs,"Custom",2,partial,2026-02-08
IoError,IoErrorGenerator,,false,,,0,missing,
DateTime<Utc>,DateTimeUtcGenerator,chrono,false,,,0,missing,
NaiveDateTime,NaiveDateTimeGenerator,chrono,false,,,0,missing,
Instant,InstantGenerator,time,false,,,0,missing,
OffsetDateTime,OffsetDateTimeGenerator,time,false,,,0,missing,
Timestamp,TimestampGenerator,jiff,false,,,0,missing,
JsonError,JsonErrorGenerator,serde_json,false,,,0,missing,
```

---

## Verification Harness Updates

### Required Changes

1. **Add generator imports:**
```rust
#[cfg(kani)]
mod generator_proofs {
    use crate::*;
    
    // Core generators
    mod uuid;
    mod errors;
    
    // Feature-gated generators
    #[cfg(feature = "chrono")]
    mod datetime_chrono;
    
    #[cfg(feature = "time")]
    mod datetime_time;
    
    #[cfg(feature = "jiff")]
    mod datetime_jiff;
}
```

2. **Add to CI pipeline:**
```yaml
- name: Verify generators (all features)
  run: cargo kani --harness generator_proofs --all-features
```

3. **Add to proof record generation:**
```bash
just verify-generators-all-features > generator_verification_results.csv
```

---

## Priority Ranking

**High Priority (Core functionality):**
1. uuid::Uuid (partial → complete)
2. std::io::Error (missing → complete)

**Medium Priority (Common features):**
3. chrono types (DateTime<Utc>, NaiveDateTime)
4. time types (Instant, OffsetDateTime)

**Low Priority (Less common features):**
5. jiff::Timestamp
6. serde_json::Error

---

## Notes

- **Randomness:** V4 UUID generation uses randomness - we can't verify the random value itself, only that the format is correct
- **Now() functions:** Functions that call `now()` are non-deterministic - we verify construction logic, not the actual timestamp
- **Feature gates:** All feature-gated generators need conditional compilation in proofs
- **Existing proofs:** Don't duplicate existing proofs - integrate generators into existing verification files where possible

---

## References

- [KANI_VERIFICATION_PATTERNS.md](KANI_VERIFICATION_PATTERNS.md) - Verification patterns
- [VERIFICATION_FRAMEWORK_DESIGN.md](VERIFICATION_FRAMEWORK_DESIGN.md) - Framework design
- [crates/elicitation/src/verification/](crates/elicitation/src/verification/) - Existing proofs

---

**Last Updated:** 2026-02-08  
**Status:** Planning phase - tracking document created
