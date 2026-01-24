# Chunked Verification Strategy for Memory-Constrained Systems

## The Problem

The 3-byte UTF-8 proof explores 49,152 symbolic combinations (12 × 64 × 64), which can exceed RAM limits on memory-constrained systems. Kani/CBMC's symbolic execution holds the entire constraint space in memory during solving.

## The Solution: Proof Partitioning

**Key insight:** Each symbolic combination is **independent**. We can partition the space into N smaller proofs, verify each chunk separately, and the union of verified chunks proves the entire space.

### Mathematical Soundness

Given a proof for input space S, we can partition S into disjoint subsets:
```
S = S₁ ∪ S₂ ∪ ... ∪ Sₙ
```

Where:
- **Disjoint:** Sᵢ ∩ Sⱼ = ∅ for i ≠ j (no overlap)
- **Exhaustive:** ⋃ Sᵢ = S (covers all inputs)

If we prove the validator correct for each Sᵢ, then by induction it's correct for S.

**Why this works:**
- Each chunk is a **complete formal proof** for its range
- No chunk depends on another chunk's result
- Verification outcome is deterministic per input
- Union of correct chunks → entire space correct

### Partitioning Strategies

#### Strategy 1: Partition Byte 1 (Coarse-Grained)

```rust
// Original: 12 × 64 × 64 = 49,152 combinations
let byte1: u8 = kani::any();
kani::assume(byte1 >= 0xE1 && byte1 <= 0xEC); // 12 values

// Chunked: 4 proofs of 3 × 64 × 64 = 12,288 combinations each
```

**Chunk 1:**
```rust
#[kani::proof]
fn verify_3byte_chunk_0() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE1 && byte1 <= 0xE3); // 3 values
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0xBF); // 64 values
    
    let byte3: u8 = kani::any();
    kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
    
    let bytes = [byte1, byte2, byte3];
    assert!(is_valid_utf8(&bytes));
}
```

**Chunk 2:**
```rust
#[kani::proof]
fn verify_3byte_chunk_1() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE4 && byte1 <= 0xE6); // 3 values
    // ... rest same
}
```

**Result:** 4 chunks × 12,288 combinations = 49,152 total ✅

#### Strategy 2: Partition Byte 2 (Medium-Grained)

```rust
// Split 64-value continuation byte into 8 chunks of 8 values
// 12 × 8 × 64 = 6,144 combinations per chunk
```

**Chunk 0:**
```rust
#[kani::proof]
fn verify_3byte_byte2_chunk_0() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE1 && byte1 <= 0xEC); // 12 values
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0x87); // 8 values
    
    let byte3: u8 = kani::any();
    kani::assume(byte3 >= 0x80 && byte3 <= 0xBF); // 64 values
    
    let bytes = [byte1, byte2, byte3];
    assert!(is_valid_utf8(&bytes));
}
```

**Result:** 8 chunks × 6,144 combinations = 49,152 total ✅

#### Strategy 3: Partition Both Byte 2 and Byte 3 (Fine-Grained)

```rust
// Split both continuation bytes into 8 chunks each
// 12 × 8 × 8 = 768 combinations per chunk
// 64 total chunks (8 × 8)
```

**Chunk (0, 0):**
```rust
#[kani::proof]
fn verify_3byte_chunk_0_0() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE1 && byte1 <= 0xEC); // 12 values
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0x87); // 8 values
    
    let byte3: u8 = kani::any();
    kani::assume(byte3 >= 0x80 && byte3 <= 0x87); // 8 values
    
    let bytes = [byte1, byte2, byte3];
    assert!(is_valid_utf8(&bytes));
}
```

**Result:** 64 chunks × 768 combinations = 49,152 total ✅

### Adaptive Chunking

Start coarse, refine if needed:

```
RAM > 16GB → Monolithic (49,152 combos)
RAM = 8-16GB → 4 chunks (12,288 combos each)
RAM = 4-8GB → 16 chunks (3,072 combos each)
RAM = 2-4GB → 64 chunks (768 combos each)
RAM < 2GB → 256 chunks (192 combos each)
```

## Implementation

### Manual Approach (Simple)

Create harnesses by hand:

```rust
// In kani_proofs/utf8.rs

#[kani::proof]
fn verify_3byte_range_0xE1_0xE3() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE1 && byte1 <= 0xE3);
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0xBF);
    
    let byte3: u8 = kani::any();
    kani::assume(byte3 >= 0x80 && byte3 <= 0xBF);
    
    let bytes = [byte1, byte2, byte3];
    assert!(is_valid_utf8(&bytes));
}

#[kani::proof]
fn verify_3byte_range_0xE4_0xE6() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE4 && byte1 <= 0xE6);
    // ... rest
}

#[kani::proof]
fn verify_3byte_range_0xE7_0xE9() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xE7 && byte1 <= 0xE9);
    // ... rest
}

#[kani::proof]
fn verify_3byte_range_0xEA_0xEC() {
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= 0xEA && byte1 <= 0xEC);
    // ... rest
}
```

Run sequentially:
```bash
cargo kani --harness verify_3byte_range_0xE1_0xE3 --features verify-kani
cargo kani --harness verify_3byte_range_0xE4_0xE6 --features verify-kani
cargo kani --harness verify_3byte_range_0xE7_0xE9 --features verify-kani
cargo kani --harness verify_3byte_range_0xEA_0xEC --features verify-kani
```

### Macro-Generated Approach (Scalable)

Generate harnesses programmatically:

```rust
macro_rules! verify_3byte_chunks {
    ($(($name:ident, $start:literal, $end:literal)),* $(,)?) => {
        $(
            #[kani::proof]
            fn $name() {
                let byte1: u8 = kani::any();
                kani::assume(byte1 >= $start && byte1 <= $end);
                
                let byte2: u8 = kani::any();
                kani::assume(byte2 >= 0x80 && byte2 <= 0xBF);
                
                let byte3: u8 = kani::any();
                kani::assume(byte3 >= 0x80 && byte3 <= 0xBF);
                
                let bytes = [byte1, byte2, byte3];
                assert!(is_valid_utf8(&bytes));
            }
        )*
    };
}

verify_3byte_chunks!(
    (verify_3byte_chunk_0, 0xE1, 0xE3),
    (verify_3byte_chunk_1, 0xE4, 0xE6),
    (verify_3byte_chunk_2, 0xE7, 0xE9),
    (verify_3byte_chunk_3, 0xEA, 0xEC),
);
```

### Build Script Approach (Most Flexible)

Generate harnesses in `build.rs`:

```rust
// build.rs
fn main() {
    let chunks = 4;
    let range = 0xE1..=0xEC; // 12 values
    let chunk_size = (range.len() + chunks - 1) / chunks;
    
    let mut code = String::from("// Auto-generated chunked proofs\n\n");
    
    for (i, chunk) in range.clone().collect::<Vec<_>>()
        .chunks(chunk_size)
        .enumerate() 
    {
        let start = chunk[0];
        let end = chunk[chunk.len() - 1];
        
        code.push_str(&format!(
            r#"
#[cfg(kani)]
#[kani::proof]
fn verify_3byte_chunk_{i}() {{
    let byte1: u8 = kani::any();
    kani::assume(byte1 >= {start:#04x} && byte1 <= {end:#04x});
    
    let byte2: u8 = kani::any();
    kani::assume(byte2 >= 0x80 && byte2 <= 0xBF);
    
    let byte3: u8 = kani::any();
    kani::assume(byte3 >= 0x80 && byte3 <= 0xBF);
    
    let bytes = [byte1, byte2, byte3];
    assert!(is_valid_utf8(&bytes));
}}
"#,
            i = i,
            start = start,
            end = end
        ));
    }
    
    std::fs::write("src/verification/types/kani_proofs/utf8_3byte_chunked.rs", code)
        .expect("Failed to write chunked proofs");
}
```

### Justfile Recipe for Sequential Execution

```bash
# Run chunked 3-byte proofs sequentially
kani-3byte-chunked:
    #!/usr/bin/env bash
    set -euo pipefail
    
    echo "🔬 Chunked 3-Byte UTF-8 Proof (4 chunks)"
    echo "========================================"
    
    CHUNKS=(
        "verify_3byte_chunk_0"
        "verify_3byte_chunk_1"
        "verify_3byte_chunk_2"
        "verify_3byte_chunk_3"
    )
    
    for chunk in "${CHUNKS[@]}"; do
        echo ""
        echo "📊 Running $chunk..."
        START=$(date +%s.%N)
        
        if cargo kani --harness "$chunk" --features verify-kani 2>&1 | tee "/tmp/${chunk}.log"; then
            END=$(date +%s.%N)
            TIME=$(echo "$END - $START" | bc)
            echo "✅ $chunk verified in ${TIME}s"
        else
            echo "❌ $chunk failed"
            exit 1
        fi
    done
    
    echo ""
    echo "✅ All chunks verified successfully!"
    echo "Total coverage: 49,152 combinations"
```

### Parallel Execution (GNU Parallel)

```bash
# Run chunks in parallel (if RAM allows)
kani-3byte-parallel:
    #!/usr/bin/env bash
    
    export -f run_chunk
    
    parallel --bar cargo kani --harness {} --features verify-kani \
        ::: verify_3byte_chunk_{0..3}
```

## Benefits

1. **RAM Management:** Each chunk uses less memory (1/4, 1/16, 1/64 of original)
2. **Parallelization:** Run chunks on different machines simultaneously
3. **Progress Tracking:** See incremental progress (chunk 1/4 done, 2/4 done...)
4. **Fault Tolerance:** If one chunk fails, others can continue
5. **Incremental Verification:** Verify critical ranges first, defer others
6. **Flexibility:** Adjust chunk size based on available resources

## Limitations

**None for correctness!** The only limitation is engineering effort:
- More chunks = more harnesses to generate
- More chunks = more invocations to run
- But: Each chunk is a complete formal proof
- Union of chunks = complete coverage

## Recommended Strategy

For the 3-byte proof on memory-constrained systems:

1. **Start with 4 chunks** (12,288 combos each):
   - Partition byte1: [0xE1-0xE3], [0xE4-0xE6], [0xE7-0xE9], [0xEA-0xEC]
   - Run sequentially overnight
   - Expected time: 4 × (hours per chunk)

2. **If still OOM, try 16 chunks** (3,072 combos each):
   - Partition byte1 into 3-value ranges
   - Or partition byte2 into 8-value ranges
   - More chunks = more runs, but smaller RAM footprint

3. **For 4-byte proof** (786,432 combos):
   - Start with 12 chunks (65,536 combos each)
   - Or 64 chunks (12,288 combos each) if RAM limited
   - Or 256 chunks (3,072 combos each) for very tight systems

## Example: 4-Byte Proof Chunked

```rust
// Original: 3 × 64 × 64 × 64 = 786,432 combinations

// Chunked (12 chunks): 1 × 64 × 64 × 64 = 262,144 each
verify_4byte_chunks!(
    (verify_4byte_0xF1, 0xF1, 0xF1),
    (verify_4byte_0xF2, 0xF2, 0xF2),
    (verify_4byte_0xF3, 0xF3, 0xF3),
);
```

Even more fine-grained:
```rust
// 64 chunks: 3 × 8 × 64 × 64 = 12,288 each
// Partition byte2 into 8-value ranges
```

## Conclusion

**Chunking is mathematically sound and practically essential for memory-constrained systems.**

There's no reason NOT to chunk proofs other than:
1. Engineering effort (generating/running more harnesses)
2. Time (sequential execution takes longer than parallel)

But for tractability on limited hardware, chunking is the **correct approach**:
- ✅ Preserves formal correctness
- ✅ Makes verification tractable
- ✅ Enables progress tracking
- ✅ Allows distributed verification

**Recommendation:** Implement macro-generated chunking for 3-byte and 4-byte proofs with configurable chunk sizes based on available RAM.
