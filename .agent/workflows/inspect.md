---
description: How to debug a hung QEMU using make inspect and GDB
---

# Debugging QEMU Hangs with `make inspect`

When QEMU hangs (boot stall, infinite loop, or panic), use the GDB inspection workflow to get register state, backtrace, and current instruction context.

## Prerequisites

- `gdb` for x86_64, or `gdb-multiarch` for other architectures
- A QEMU instance running with GDB stub enabled

## Architecture-Specific Ports

Each architecture has a stable GDB port:

| Architecture  | Port |
|---------------|------|
| x86_64        | 1234 |
| aarch64       | 2234 |
| riscv64       | 3234 |
| loongarch64   | 4234 |

## Workflow

### Step 1: Start QEMU with GDB stub

In **Terminal 1**, launch QEMU with the GDB stub enabled:

```bash
# For x86_64:
make run-gdb-x86_64

# Or for other architectures:
make run-gdb-aarch64
make run-gdb-riscv64
make run-gdb-loongarch64

# Generic (uses KARCH default, usually x86_64):
make run-gdb
```

If you want QEMU to **freeze immediately** (wait for GDB to attach before executing), add `GDB_PAUSE=1`:

```bash
make run-gdb-x86_64 GDB_PAUSE=1
```

### Step 2: Wait for the hang

Let the kernel boot until it reaches the hang point. When it stalls, proceed to Step 3.

### Step 3: Capture GDB output with `make inspect`

In **Terminal 2**, run the inspect target for your architecture:

```bash
# For x86_64:
// turbo
make inspect-x86_64

# Or for other architectures:
// turbo
make inspect-aarch64
// turbo
make inspect-riscv64
// turbo
make inspect-loongarch64
```

This command connects to the running QEMU GDB stub and prints:

- **Registers** — All CPU registers
- **Backtrace** — Stack trace (if symbols are available)
- **Instructions** — Next 10 instructions at the current PC
- **Source** — Source code at the current PC (if debug info available)

### Step 4: Analyze the output

The inspect output will look like:

```
--- REGISTERS ---
rax            0x0                 0
rbx            0xffffffff80100000  -2146435072
rip            0xffffffff80001234  0xffffffff80001234 <kernel_main+0x42>
...

--- BACKTRACE ---
#0  0xffffffff80001234 in kernel_main () at src/main.rs:42
#1  0xffffffff80000100 in _start () at src/boot.rs:10

--- INSTRUCTIONS ---
   0xffffffff80001234 <kernel_main+0x42>:    hlt
   0xffffffff80001235 <kernel_main+0x43>:    jmp    0xffffffff80001234

--- SOURCE ---
42          loop { x86_64::instructions::hlt(); }
```

### Example: Diagnosing a boot hang on AArch64

```bash
# Terminal 1: Start QEMU
make run-gdb-aarch64

# Terminal 2: After observing the hang, inspect
make inspect-aarch64
```

## Interactive GDB Session

For more control, you can start an interactive GDB session instead of the batch `inspect`:

```bash
# Build kernel first
make bran KARCH=aarch64

# Connect manually
gdb-multiarch \
    -ex "set architecture aarch64" \
    -ex "file crates/bran/bin-aarch64/kernel" \
    -ex "target remote :2234"
```

Then use standard GDB commands: `bt`, `info registers`, `x/10i $pc`, `continue`, `stepi`, etc.

## Overriding GDB Binary

If you have a custom GDB binary:

```bash
make inspect-x86_64 GDB_BIN=/usr/local/bin/my-gdb
```

Or for a specific architecture:

```bash
make inspect-aarch64 GDB_BIN_aarch64=/path/to/aarch64-gdb
```

## Troubleshooting

### "Connection refused"

QEMU isn't running or wasn't started with `WITH_GDB=1`. Use `make run-gdb-<arch>`.

### "Remote 'g' packet reply is too long"

Architecture mismatch. Ensure you're using the correct `inspect-<arch>` for your running QEMU.

### No symbols / "??" in backtrace

The kernel was built without debug info. Check that `Cargo.toml` has `debug = true` in the appropriate profile.
