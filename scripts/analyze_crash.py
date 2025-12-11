#!/usr/bin/env python3
import sys
import re
import os
import subprocess
import argparse

def find_crash_rip(log_path):
    """
    Parses the QEMU log to find the RIP/IP at the time of the crash.
    Looks for 'Triple fault' or 'PANIC' and scans backwards for the register dump.
    """
    if not os.path.exists(log_path):
        print(f"Error: Log file '{log_path}' not found.")
        return None

    with open(log_path, 'r') as f:
        lines = f.readlines()

    crash_idx = -1
    # Scan for known crash patterns
    for i, line in enumerate(lines):
        if "Triple fault" in line or "PANIC" in line or "DOUBLE FAULT" in line:
            crash_idx = i
            # Keep searching for the *last* crash if multiple (though usually QEMU exits)
            # But usually the last one is the one that killed it.
    
    if crash_idx == -1:
        print("No crash detected in log (searched for 'Triple fault', 'PANIC', 'DOUBLE FAULT').")
        return None

    print(f"Crash detected at line {crash_idx + 1}: {lines[crash_idx].strip()}")

    # Search backwards for RIP/PC
    # x86_64: RIP=000000000020fdb2
    # aarch64: PC=... (format varies, usually 'PC=...' or 'pc=...')
    # riscv64: pc ...
    
    # We'll try a few regexes
    rip_regexes = [
        r'RIP=([0-9a-fA-F]+)', # x86
        r'PC=([0-9a-fA-F]+)',  # ARM/others
        r'pc\s+([0-9a-fA-F]+)', # RISC-V often
    ]

    for i in range(crash_idx, max(-1, crash_idx - 100), -1):
        line = lines[i]
        for rx in rip_regexes:
            match = re.search(rx, line, re.IGNORECASE)
            if match:
                addr_str = match.group(1)
                print(f"Found crash address: 0x{addr_str}")
                return int(addr_str, 16)
    
    print("Could not find register dump (RIP/PC) preceding the crash.")
    return None

def find_binary_for_addr(addr, search_path):
    """
    Scans ELF files in search_path to find which one contains the given address.
    Uses `addr2line` to check if the address resolves to a source line.
    """
    print(f"Scanning binaries in {search_path}...")
    
    candidates = []
    
    for root, dirs, files in os.walk(search_path):
        for file in files:
            filepath = os.path.join(root, file)
            # Skip obvious non-binaries
            if file.endswith('.d') or file.endswith('.rlib') or file.endswith('.rmeta'):
                continue
            
            # Check if executable
            if not os.access(filepath, os.X_OK):
                continue

            try:
                # Use addr2line to check if address maps to something
                # -e <file> <addr>
                res = subprocess.run(
                    ['addr2line', '-e', filepath, hex(addr)], 
                    capture_output=True, 
                    text=True
                )
                
                if res.returncode == 0:
                    output = res.stdout.strip()
                    if output and not output.startswith('??'):
                        # Found a match!
                        candidates.append((filepath, output))
            except Exception as e:
                pass

    return candidates

def disassemble_around(binary, addr):
    """
    Runs objdump to disassemble instructions around the address.
    """
    print(f"\nDisassembly for {binary} around 0x{addr:x}:")
    
    # Determine architecture for objdump if needed, but usually auto-detects.
    # We want -d (disassemble), -S (source), -C (demangle)
    # --start-address and --stop-address
    
    start_addr = max(0, addr - 32)
    stop_addr = addr + 32
    
    cmd = [
        'objdump', 
        '-d', 
        '-C', 
        '--start-address', hex(start_addr),
        '--stop-address', hex(stop_addr),
        binary
    ]
    
    try:
        subprocess.run(cmd)
    except Exception as e:
        print(f"Failed to run objdump: {e}")

def main():
    parser = argparse.ArgumentParser(description="Analyze QEMU crash logs.")
    parser.add_argument("log_file", help="Path to qemu.log")
    parser.add_argument("target_dir", help="Directory containing binaries (e.g. target/x86_64-unknown-none/debug)")
    
    args = parser.parse_args()
    
    addr = find_crash_rip(args.log_file)
    if addr is None:
        sys.exit(1)
        
    candidates = find_binary_for_addr(addr, args.target_dir)
    
    if not candidates:
        print("No binary found containing this address.")
        # It might be in the kernel if we didn't check it, but kernel is usually in the target dir too?
        # Or maybe it's dynamically loaded code?
        sys.exit(1)
        
    print(f"\nFound {len(candidates)} candidate binary(ies):")
    for binary, source_loc in candidates:
        print(f"  {binary} -> {source_loc}")
        
    # Disassemble the first one (or all?)
    # Usually there should be only one unless multiple binaries map the same address (unlikely for static OS unless PIE/PIC at 0)
    # But here user apps are at 0x200000 approx.
    
    if candidates:
        binary = candidates[0][0]
        disassemble_around(binary, addr)

if __name__ == "__main__":
    main()
