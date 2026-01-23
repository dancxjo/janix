# ThingOS System Interface (TSI) v0.5

This document defines the authoritative syscall ABI for ThingOS userland.
All architectures and userspace libraries must adhere to this contract.

## Stability

**Status**: Frozen for v0.5.
**Policy**: Syscall numbers and argument layouts are stable. New syscalls may be added, but existing ones cannot change without a revision bump.

## Calling Convention

### Common (All Arch)
- **Return**: `isize`
    - `>= 0`: Success (value depends on syscall)
    - `< 0`: Error (negated `Errno`, e.g. `-EINVAL`)
- **Arguments**: Up to 6 `usize` values.

### Architecture Registers

| Arch | Syscall Number | Args (0..5) | Return | Trap |
|---|---|---|---|---|
| **x86_64** | `rax` | `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9` | `rax` | `syscall` |
| **AArch64** | `x8` | `x0`..`x5` | `x0` | `svc #0` |
| **RISC-V 64** | `a7` | `a0`..`a5` | `a0` | `ecall` |
| **LoongArch64** | `a7` | `a0`..`a5` | `a0` | `syscall 0` |

## Validation Rules
1. **User Pointers**: Must be validated against the user address space range before access.
2. **Alignment**: Pointers must be naturally aligned for the type they point to.
3. **Strings**: UTF-8 strings are required for text syscalls; invalid UTF-8 is rejected.

## Process / Thread

| ID | Name | Args | Description |
|---|---|---|---|
| 0x01 | `SYS_EXIT` | `code: i32` | Terminate current task with exit code. |
| 0x02 | `SYS_GET_TID` | - | Return current thread id. |
| 0x03 | `SYS_SPAWN_THREAD` | `req_ptr: *const SpawnThreadReq` | Spawn a user thread in the current process. |
| 0x04 | `SYS_SPAWN_PROCESS` | `name_ptr, name_len, arg` | Spawn a named user process with an argument. |
| 0x05 | `SYS_TASK_WAIT` | `tid` | Block until the thread exits; returns exit code. |

### `SpawnThreadReq` layout

```
#[repr(C)]
struct SpawnThreadReq {
    entry: usize,     // User entry function
    sp: usize,        // Initial stack pointer
    arg: usize,       // Argument passed to entry
    stack: StackInfo, // Stack reservation metadata
}
```

## Scheduling + Sleep

| ID | Name | Args | Description |
|---|---|---|---|
| 0x10 | `SYS_YIELD` | - | Yield the CPU. |
| 0x11 | `SYS_SLEEP_NS` | `ns: u64` | Sleep for at least N nanoseconds. |
| 0x12 | `SYS_SLEEP_MS` | `ms: u64` | Sleep for at least N milliseconds. |
| 0x13 | `SYS_TASK_POLL` | `tid` | Return task status + exit code. |
| 0x14 | `SYS_SET_PRIORITY` | `tid, priority` | Set task priority (0-4). |

## Time

| ID | Name | Args | Description |
|---|---|---|---|
| 0x20 | `SYS_TIME_MONOTONIC` | - | Return monotonic time in nanoseconds. |
| 0x21 | `SYS_TIME_NOW` | - | Return wall-clock time if anchored. |
| 0x22 | `SYS_TIME_ANCHOR` | `unix_secs` | Anchor wall clock against monotonic time. |

## Logging / Debug Output

| ID | Name | Args | Description |
|---|---|---|---|
| 0x30 | `SYS_DEBUG_WRITE` | `ptr, len` | Write UTF-8 string to debug output. |
| 0x31 | `SYS_LOG_WRITE` | `ptr, len, level` | Write UTF-8 string to structured log output. |

## Memory

| ID | Name | Args | Description |
|---|---|---|---|
| 0x40 | `SYS_ALLOC_STACK` | `pages` | Allocate a new growing stack region. |
| 0x41 | `SYS_VM_MAP` | `req_ptr, resp_ptr` | Map virtual memory. |
| 0x42 | `SYS_VM_UNMAP` | `req_ptr, resp_ptr` | Unmap virtual memory. |
| 0x43 | `SYS_VM_PROTECT` | `req_ptr` | Update VM protections. |
| 0x44 | `SYS_VM_ADVISE` | `req_ptr` | Provide VM usage hints. |
| 0x45 | `SYS_VM_QUERY` | `req_ptr, resp_ptr` | Query VM regions. |

## Notes

- Streams, ports, and graph syscalls live outside the minimal TSI surface but remain stable.
- The TSI deliberately avoids POSIX semantics; higher-level layers may map onto these primitives.
