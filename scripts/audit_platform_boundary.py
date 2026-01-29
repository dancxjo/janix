#!/usr/bin/env python3
"""
Platform Boundary Audit - ensures no_std compliance for kernel/userspace.

This script verifies that kernel and userspace crates do not accidentally
depend on std, maintaining the platform layer contract defined by stem::pal.
"""

import json
import subprocess
import sys
from pathlib import Path

# Crates that are allowed to use std (build-time tools only)
ALLOWED_STD_CRATES = {
    "xtask",
    "pciids", 
    "bdd",
    "unifont-gen",
    "display_proto_tests",
    # Proc-macro crates run at compile time and legitimately use std
    "abi-macros",
    "stem-macros",
}

# Crates that must be no_std (kernel and userspace)
REQUIRED_NOSTD_CRATES = {
    "kernel",
    "stem",
    "stem-macros",
    "abi",
    "abi-macros",
    "bran",
}

def get_cargo_metadata():
    """Get cargo metadata for all workspace members."""
    result = subprocess.run(
        ["cargo", "metadata", "--format-version=1", "--no-deps"],
        capture_output=True,
        text=True,
        check=True
    )
    return json.loads(result.stdout)

def is_nostd_crate(crate_path):
    """Check if a crate declares #![no_std]."""
    # Check lib.rs or main.rs for #![no_std]
    for file_name in ["lib.rs", "main.rs"]:
        src_file = Path(crate_path) / "src" / file_name
        if src_file.exists():
            content = src_file.read_text()
            # Look for #![no_std] in first ~20 lines
            for line in content.split('\n')[:20]:
                if '#![no_std]' in line or '#![cfg_attr(not(test), no_std)]' in line:
                    return True
    return False

def check_std_dependency(package):
    """Check if a package has std as a dependency."""
    # This is a simple heuristic - std would appear in resolved features
    # More robust: actually check if the crate links to std
    name = package['name']
    
    # Check if it's in userspace - all userspace should be no_std
    manifest_path = Path(package['manifest_path'])
    is_userspace = 'userspace' in manifest_path.parts
    is_kernel_or_core = name in REQUIRED_NOSTD_CRATES
    
    return (is_userspace or is_kernel_or_core), is_nostd_crate(manifest_path.parent)

def main():
    print("🔍 Platform Boundary Audit")
    print("=" * 60)
    
    metadata = get_cargo_metadata()
    
    errors = []
    warnings = []
    checked_count = 0
    
    for package in metadata['packages']:
        name = package['name']
        
        # Skip allowed std crates
        if name in ALLOWED_STD_CRATES:
            print(f"✓ {name:30} [std allowed - build tool]")
            continue
        
        should_be_nostd, is_nostd = check_std_dependency(package)
        
        if should_be_nostd:
            checked_count += 1
            if is_nostd:
                print(f"✓ {name:30} [no_std compliant]")
            else:
                error_msg = f"✗ {name:30} [MISSING #![no_std]]"
                print(error_msg)
                errors.append(f"{name} is missing #![no_std] declaration")
    
    print("=" * 60)
    print(f"Checked {checked_count} crates for no_std compliance")
    
    if errors:
        print("\n❌ ERRORS:")
        for error in errors:
            print(f"  - {error}")
        return 1
    
    if warnings:
        print("\n⚠️  WARNINGS:")
        for warning in warnings:
            print(f"  - {warning}")
    
    print("\n✅ Platform boundary audit passed!")
    print(f"   All kernel/userspace crates are no_std compliant.")
    return 0

if __name__ == "__main__":
    sys.exit(main())
