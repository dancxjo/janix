#!/bin/bash

FORBIDDEN_PATHS=(
    "crates/kernel/src/memory"
    "crates/kernel/src/syscall/memory.rs"
    "crates/kernel/src/boot.rs"
    "crates/kernel/src/platform"
)

FORBIDDEN_TOKENS=(
    "graph::store"
    "with_store("
    "PRED_BASE_PHYS"
    "PRED_SIZE"
    "relationship_create("
)

FAILED=0
for path in "${FORBIDDEN_PATHS[@]}"; do
    if [ ! -e "$path" ]; then
        continue
    fi
    for token in "${FORBIDDEN_TOKENS[@]}"; do
        # Use -rn to get line numbers and avoid binary matches
        matches=$(grep -rn "$token" "$path" | grep -v "journal.rs")
        if [ ! -z "$matches" ]; then
            echo "ERROR: Forbidden token '$token' found in path '$path':"
            echo "$matches"
            FAILED=1
        fi
    done
done

if [ $FAILED -eq 1 ]; then
    exit 1
fi

echo "SUCCESS: No forbidden graph usage found in memory/boot/platform paths."
exit 0
