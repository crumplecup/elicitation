#!/usr/bin/env python3
"""
Checkpointed Kani proof runner for chunked verification.

Reads/creates CSV tracking proof completion, runs missing chunks,
and updates the record. Supports resume after interruption.

Usage:
    ./kani_chunked_runner.py 2byte 2    # 2-byte proof, 2 chunks
    ./kani_chunked_runner.py 2byte 4    # 2-byte proof, 4 chunks
    ./kani_chunked_runner.py 3byte 4    # 3-byte proof, 4 chunks
    ./kani_chunked_runner.py 3byte 12   # 3-byte proof, 12 chunks
    ./kani_chunked_runner.py 4byte 3    # 4-byte proof, 3 chunks
"""

import csv
import subprocess
import sys
import time
from datetime import datetime
from pathlib import Path
from typing import List, Dict, Optional

# Chunk configurations
CONFIGS = {
    ("2byte", 2): {
        "harnesses": [f"verify_2byte_2chunks_{i}" for i in range(2)],
        "chunks": [(0xC2, 0xD0), (0xD1, 0xDF)],
        "combos": 992,  # Average
        "total": 3968,
    },
    ("2byte", 4): {
        "harnesses": [f"verify_2byte_4chunks_{i}" for i in range(4)],
        "chunks": [(0xC2, 0xCA), (0xCB, 0xD2), (0xD3, 0xDA), (0xDB, 0xDF)],
        "combos": 492,  # Average
        "total": 3968,
    },
    ("3byte", 4): {
        "harnesses": [f"verify_3byte_4chunks_{i}" for i in range(4)],
        "chunks": [(0xE1, 0xE3), (0xE4, 0xE6), (0xE7, 0xE9), (0xEA, 0xEC)],
        "combos": 12288,
        "total": 49152,
    },
    ("3byte", 12): {
        "harnesses": [f"verify_3byte_12chunks_{i}" for i in range(12)],
        "chunks": [(b, b) for b in range(0xE1, 0xED)],
        "combos": 4096,
        "total": 49152,
    },
    ("4byte", 3): {
        "harnesses": [f"verify_4byte_3chunks_{i}" for i in range(3)],
        "chunks": [(0xF1, 0xF1), (0xF2, 0xF2), (0xF3, 0xF3)],
        "combos": 262144,
        "total": 786432,
    },
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
        print("  kani_chunked_runner.py 2byte 2    # 2-byte, 2 chunks")
        print("  kani_chunked_runner.py 2byte 4    # 2-byte, 4 chunks")
        print("  kani_chunked_runner.py 3byte 4    # 3-byte, 4 chunks")
        print("  kani_chunked_runner.py 3byte 12   # 3-byte, 12 chunks")
        print("  kani_chunked_runner.py 4byte 3    # 4-byte, 3 chunks")
        sys.exit(1)
    
    proof_type = sys.argv[1]
    num_chunks = int(sys.argv[2])
    
    # Validate configuration
    config_key = (proof_type, num_chunks)
    if config_key not in CONFIGS:
        print(f"❌ Invalid configuration: {proof_type} with {num_chunks} chunks")
        print(f"\nAvailable configurations:")
        for (pt, nc) in CONFIGS.keys():
            print(f"  {pt} {nc}")
        sys.exit(1)
    
    config = CONFIGS[config_key]
    record = ProofRecord(proof_type, num_chunks)
    
    # Determine what needs to run
    completed = record.get_completed_chunks()
    total_chunks = len(config["harnesses"])
    remaining_chunks = [i for i in range(total_chunks) if i not in completed]
    
    # Print initial status
    record.print_summary(completed, total_chunks)
    
    if not remaining_chunks:
        print("\n✅ All chunks already verified!")
        return
    
    # Ask for confirmation
    print(f"\n🔬 Ready to verify {len(remaining_chunks)} remaining chunks")
    print(f"   Combinations per chunk: {config['combos']:,}")
    print(f"   Total coverage: {config['total']:,}")
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
        harness = config["harnesses"][chunk_num]
        chunk_range = config["chunks"][chunk_num]
        
        print(f"\n📦 Chunk {chunk_num + 1}/{total_chunks}: {harness}")
        print(f"   Range: {chunk_range[0]:#04x}-{chunk_range[1]:#04x}")
        print(f"   Combinations: {config['combos']:,}")
        print(f"   Started: {datetime.now().strftime('%H:%M:%S')}")
        
        elapsed, status = run_chunk(harness)
        
        # Log result
        record.append_result(
            chunk_num, chunk_range, config['combos'], elapsed, status
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
