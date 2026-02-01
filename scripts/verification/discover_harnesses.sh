#!/usr/bin/env bash
# Discover all Kani proof harnesses in the codebase
#
# Output format: MODULE,HARNESS_NAME
# Example: ipaddr_bytes,verify_ipv4_10_network_is_private

set -euo pipefail

# Find all files with Kani proofs
PROOF_DIR="crates/elicitation/src/verification/types/kani_proofs"

if [ ! -d "$PROOF_DIR" ]; then
    echo "Error: Proof directory not found: $PROOF_DIR" >&2
    exit 1
fi

# Temporary file for results
TEMP_FILE=$(mktemp)
trap "rm -f $TEMP_FILE" EXIT

# Process each .rs file in the proofs directory
find "$PROOF_DIR" -name "*.rs" -type f | while read -r file; do
    # Extract module name from filename (remove path and .rs extension)
    module=$(basename "$file" .rs)
    
    # Skip mod.rs
    if [ "$module" = "mod" ]; then
        continue
    fi
    
    # Find all #[kani::proof] annotations and extract function names
    # Look for the pattern: #[kani::proof] followed by optional attributes, then fn name
    if grep -q '#\[kani::proof\]' "$file"; then
        grep -A 10 '#\[kani::proof\]' "$file" | \
            grep '^fn ' | \
            sed 's/^fn \([a-zA-Z0-9_]*\).*/\1/' | \
            while read -r harness; do
                echo "$module,$harness"
            done
    fi
done | sort

exit 0
