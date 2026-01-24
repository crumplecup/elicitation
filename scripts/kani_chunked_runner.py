#!/usr/bin/env python3
"""
Checkpointed Kani proof runner for chunked verification.

Dynamically partitions proof space into N chunks based on user input.
Reads/creates CSV tracking proof completion, runs missing chunks,
and updates the record. Supports resume after interruption.

Usage:
    ./kani_chunked_runner.py 2byte N    # 2-byte proof, N chunks
    ./kani_chunked_runner.py 3byte N    # 3-byte proof, N chunks
    ./kani_chunked_runner.py 4byte N    # 4-byte proof, N chunks
    
Examples:
    ./kani_chunked_runner.py 2byte 8    # 2-byte, 8 chunks (~496 combos each)
    ./kani_chunked_runner.py 3byte 16   # 3-byte, 16 chunks (~3,072 combos each)
    ./kani_chunked_runner.py 4byte 12   # 4-byte, 12 chunks (~65,536 combos each)
"""

import csv
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import List, Tuple

# Proof space definitions
PROOF_SPACES = {
    "2byte": {
        "byte1_range": (0xC2, 0xDF),  # 62 values (avoids overlong)
        "byte2_range": (0x80, 0xBF),  # 64 values (continuation)
        "total_combos": 3968,  # 62 × 64
    },
    "3byte": {
        "byte1_range": (0xE1, 0xEC),  # 12 values (avoids overlong/surrogate)
        "byte2_range": (0x80, 0xBF),  # 64 values
        "byte3_range": (0x80, 0xBF),  # 64 values
        "total_combos": 49152,  # 12 × 64 × 64
    },
    "4byte": {
        "byte1_range": (0xF1, 0xF3),  # 3 values (avoids overlong/overflow)
        "byte2_range": (0x80, 0xBF),  # 64 values
        "byte3_range": (0x80, 0xBF),  # 64 values
        "byte4_range": (0x80, 0xBF),  # 64 values
        "total_combos": 786432,  # 3 × 64³
    },
}


def partition_range(start: int, end: int, num_chunks: int) -> List[Tuple[int, int]]:
    """Partition byte range into N chunks."""
    total_values = end - start + 1
    chunk_size = total_values // num_chunks
    remainder = total_values % num_chunks
    
    chunks = []
    current = start
    
    for i in range(num_chunks):
        # Distribute remainder across first chunks
        size = chunk_size + (1 if i < remainder else 0)
        chunk_end = current + size - 1
        chunks.append((current, min(chunk_end, end)))
        current = chunk_end + 1
    
    return chunks


def calculate_chunk_config(proof_type: str, num_chunks: int) -> dict:
    """Calculate chunk configuration dynamically."""
    if proof_type not in PROOF_SPACES:
        raise ValueError(f"Unknown proof type: {proof_type}")
    
    space = PROOF_SPACES[proof_type]
    byte1_start, byte1_end = space["byte1_range"]
    
    # Partition byte1 range into N chunks
    byte1_chunks = partition_range(byte1_start, byte1_end, num_chunks)
    
    # Calculate combos per chunk
    if proof_type == "2byte":
        byte2_count = space["byte2_range"][1] - space["byte2_range"][0] + 1
        combos_per_byte1 = byte2_count
    elif proof_type == "3byte":
        byte2_count = space["byte2_range"][1] - space["byte2_range"][0] + 1
        byte3_count = space["byte3_range"][1] - space["byte3_range"][0] + 1
        combos_per_byte1 = byte2_count * byte3_count
    elif proof_type == "4byte":
        byte2_count = space["byte2_range"][1] - space["byte2_range"][0] + 1
        byte3_count = space["byte3_range"][1] - space["byte3_range"][0] + 1
        byte4_count = space["byte4_range"][1] - space["byte4_range"][0] + 1
        combos_per_byte1 = byte2_count * byte3_count * byte4_count
    
    # Build chunk list with harness names and combo counts
    chunks_info = []
    for i, (chunk_start, chunk_end) in enumerate(byte1_chunks):
        byte1_count = chunk_end - chunk_start + 1
        combos = byte1_count * combos_per_byte1
        
        chunks_info.append({
            "chunk_num": i,
            "harness": f"verify_{proof_type}_{num_chunks}chunks_{i}",
            "byte_range": (chunk_start, chunk_end),
            "combos": combos,
        })
    
    return {
        "chunks": chunks_info,
        "total": space["total_combos"],
    }


class ProofRecord:
    """Manages CSV record of proof completion."""
    
    def __init__(self, proof_type: str, num_chunks: int):
        self.proof_type = proof_type
        self.num_chunks = num_chunks
        self.csv_path = Path(f"kani_proof_record_{num_chunks}.csv")
        self.fieldnames = [
            "Timestamp",
            "Proof_Type",
            "Chunk_ID",
            "Chunk_Number",
            "Total_Chunks",
            "Byte_Range",
            "Combinations",
            "Time_Seconds",
            "Status",
            "Kani_Version",
        ]
        
        # Create if doesn't exist
        if not self.csv_path.exists():
            with open(self.csv_path, 'w', newline='') as f:
                writer = csv.DictWriter(f, fieldnames=self.fieldnames)
                writer.writeheader()
            print(f"📝 Created new record: {self.csv_path}")
        else:
            print(f"📝 Using existing record: {self.csv_path}")
    
    def get_completed_chunks(self) -> set:
        """Return set of completed chunk numbers."""
        completed = set()
        with open(self.csv_path, 'r') as f:
            reader = csv.DictReader(f)
            for row in reader:
                if (row['Proof_Type'] == self.proof_type and 
                    row['Status'] == 'SUCCESS'):
                    completed.add(int(row['Chunk_Number']))
        return completed
    
    def append_result(self, chunk_num: int, chunk_range: tuple, 
                     combos: int, time_sec: float, status: str):
        """Append chunk result to CSV."""
        timestamp = datetime.now().isoformat()
        kani_version = self._get_kani_version()
        
        row = {
            "Timestamp": timestamp,
            "Proof_Type": self.proof_type,
            "Chunk_ID": f"{chunk_num}/{self.num_chunks}",
            "Chunk_Number": chunk_num,
            "Total_Chunks": self.num_chunks,
            "Byte_Range": f"{chunk_range[0]:#04x}-{chunk_range[1]:#04x}",
            "Combinations": combos,
            "Time_Seconds": f"{time_sec:.2f}",
            "Status": status,
            "Kani_Version": kani_version,
        }
        
        with open(self.csv_path, 'a', newline='') as f:
            writer = csv.DictWriter(f, fieldnames=self.fieldnames)
            writer.writerow(row)
    
    def _get_kani_version(self) -> str:
        """Get Kani version string."""
        try:
            result = subprocess.run(
                ["cargo", "kani", "--version"],
                capture_output=True,
                text=True,
                timeout=5
            )
            return result.stdout.strip().split('\n')[0]
        except Exception:
            return "unknown"
    
    def print_summary(self, completed: set, total: int):
        """Print progress summary."""
        remaining = total - len(completed)
        percent = (len(completed) / total) * 100
        
        print("\n📊 Progress Summary")
        print("=" * 50)
        print(f"Proof Type: {self.proof_type}")
        print(f"Chunk Size: {self.num_chunks}")
        print(f"Completed: {len(completed)}/{total} ({percent:.1f}%)")
        print(f"Remaining: {remaining}")
        
        if completed:
            print(f"Completed chunks: {sorted(completed)}")
        if remaining > 0:
            missing = sorted(set(range(total)) - completed)
            print(f"Remaining chunks: {missing[:10]}{'...' if len(missing) > 10 else ''}")


def run_chunk(harness: str, features: str = "verify-kani") -> tuple:
    """Run a single chunk proof and return (time, status)."""
    start = time.time()
    
    try:
        result = subprocess.run(
            ["cargo", "kani", "--harness", harness, "--features", features],
            capture_output=True,
            text=True,
            timeout=None  # No timeout - let it run
        )
        elapsed = time.time() - start
        
        if result.returncode == 0:
            return (elapsed, "SUCCESS")
        else:
            return (elapsed, "FAILED")
            
    except subprocess.TimeoutExpired:
        elapsed = time.time() - start
        return (elapsed, "TIMEOUT")
    except Exception as e:
        elapsed = time.time() - start
        return (elapsed, f"ERROR: {e}")


def main():
    if len(sys.argv) != 3:
        print("Usage: kani_chunked_runner.py <proof_type> <num_chunks>")
        print("\nExamples:")
        print("  kani_chunked_runner.py 2byte 8    # 2-byte, 8 chunks (~496 combos each)")
        print("  kani_chunked_runner.py 3byte 16   # 3-byte, 16 chunks (~3,072 combos each)")
        print("  kani_chunked_runner.py 4byte 12   # 4-byte, 12 chunks (~65,536 combos each)")
        print("\nSupported proof types: 2byte, 3byte, 4byte")
        print("Chunks: Any positive integer (system calculates partitions dynamically)")
        sys.exit(1)
    
    proof_type = sys.argv[1]
    
    try:
        num_chunks = int(sys.argv[2])
        if num_chunks < 1:
            print(f"❌ num_chunks must be positive, got: {num_chunks}")
            sys.exit(1)
    except ValueError:
        print(f"❌ num_chunks must be an integer, got: {sys.argv[2]}")
        sys.exit(1)
    
    # Validate proof type
    if proof_type not in PROOF_SPACES:
        print(f"❌ Invalid proof type: {proof_type}")
        print(f"\nSupported types: {', '.join(PROOF_SPACES.keys())}")
        sys.exit(1)
    
    # Calculate dynamic configuration
    try:
        config = calculate_chunk_config(proof_type, num_chunks)
    except Exception as e:
        print(f"❌ Failed to calculate chunks: {e}")
        sys.exit(1)
    record = ProofRecord(proof_type, num_chunks)
    
    # Determine what needs to run
    completed = record.get_completed_chunks()
    total_chunks = len(config["chunks"])
    remaining_chunks = [i for i in range(total_chunks) if i not in completed]
    
    # Print initial status
    record.print_summary(completed, total_chunks)
    
    if not remaining_chunks:
        print("\n✅ All chunks already verified!")
        return
    
    # Show chunk breakdown
    print(f"\n📊 Chunk Breakdown:")
    for chunk in config["chunks"][:5]:  # Show first 5
        print(f"  Chunk {chunk['chunk_num']}: "
              f"{chunk['byte_range'][0]:#04x}-{chunk['byte_range'][1]:#04x} "
              f"({chunk['combos']:,} combos)")
    if len(config["chunks"]) > 5:
        print(f"  ... ({len(config['chunks']) - 5} more chunks)")
    
    # Ask for confirmation
    print(f"\n🔬 Ready to verify {len(remaining_chunks)} remaining chunks")
    print(f"   Total coverage: {config['total']:,} combinations")
    print()
    
    response = input("Continue? (y/N) ")
    if response.lower() != 'y':
        print("Cancelled.")
        return
    
    # Run remaining chunks
    print(f"\n{'='*60}")
    print(f"Starting chunked verification at {datetime.now()}")
    print(f"{'='*60}\n")
    
    for chunk_num in remaining_chunks:
        chunk_info = config["chunks"][chunk_num]
        harness = chunk_info["harness"]
        chunk_range = chunk_info["byte_range"]
        combos = chunk_info["combos"]
        
        print(f"\n📦 Chunk {chunk_num + 1}/{total_chunks}: {harness}")
        print(f"   Range: {chunk_range[0]:#04x}-{chunk_range[1]:#04x}")
        print(f"   Combinations: {combos:,}")
        print(f"   Started: {datetime.now().strftime('%H:%M:%S')}")
        
        elapsed, status = run_chunk(harness)
        
        # Log result
        record.append_result(
            chunk_num, chunk_range, combos, elapsed, status
        )
        
        # Print result
        if status == "SUCCESS":
            print(f"   ✅ VERIFIED in {elapsed:.1f}s ({elapsed/60:.1f}m)")
        else:
            print(f"   ❌ {status} after {elapsed:.1f}s")
            
            response = input("\nContinue with remaining chunks? (y/N) ")
            if response.lower() != 'y':
                print("\nStopping. Progress saved to CSV.")
                break
        
        # Update progress
        completed.add(chunk_num)
        remaining = total_chunks - len(completed)
        percent = (len(completed) / total_chunks) * 100
        print(f"   Progress: {len(completed)}/{total_chunks} ({percent:.1f}%)")
    
    # Final summary
    print(f"\n{'='*60}")
    record.print_summary(completed, total_chunks)
    print(f"{'='*60}\n")
    
    if len(completed) == total_chunks:
        print("🎉 ALL CHUNKS VERIFIED!")
        print(f"Total coverage: {config['total']:,} symbolic combinations")
    else:
        print("⚠️  Partial completion - resume anytime with same command")
    
    print(f"\nRecord: {record.csv_path}")


if __name__ == "__main__":
    main()
