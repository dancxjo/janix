//! canonical syscall numbers and calling conventions

// Calling Conventions
// -------------------
//
// We aim for a unified ABI across architectures where possible, though
// the register set differs.
//
// Arguments:
//   x86_64:      rdi, rsi, rdx, r10, r8, r9
//   aarch64:     x0, x1, x2, x3, x4, x5
//   riscv64:     a0, a1, a2, a3, a4, a5
//   loongarch64: a0, a1, a2, a3, a4, a5
//
// Return Values:
//   All architectures return distinct (status, val0, val1).
//
//   x86_64:      rax (status), rdx (val0), rcx (val1)
//                *Note: rcx is usually destroyed by syscall, so we might need to pick another for val1 if we want to preserve it,
//                 BUT currently SyscallResult uses (u64, u64, u64).
//                 Linux uses rax for ret. ThingOS returns a struct.
//                 We need to verify the low-level asm return path maps these correctly.
//
//   aarch64:     x0 (status), x1 (val0), x2 (val1)
//
// Note: The `SyscallResult` struct in Rust is returned in registers according
// to the "small struct return" ABI of the platform.
// For (u64, u64, u64), it likely spills to memory on some ABIs if not careful,
// but our ASM trampolines specifically handle unpacking.

/// Syscall Numbers
pub mod nr {
    // === Core / Management ===
    /// Get kernel version.
    pub const SYS_VERSION_GET: u32 = 0;
    /// Machine debug/low-level ops (kernel tasks only).
    pub const SYS_MACHINE: u32 = 1;

    // === Logging ===
    /// Emit a log entry.
    pub const SYS_LOG: u32 = 10;

    // === Capabilities ===
    // (Implicit in operations, but maybe we need inspection later?)

    // === Graph Mutation ===
    /// Create a new Thing.
    pub const SYS_THING_CREATE: u32 = 20;
    /// Create a Relationship.
    pub const SYS_REL_CREATE: u32 = 21;
    /// Delete a Relationship.
    pub const SYS_REL_DELETE: u32 = 22;

    // === Graph Observation ===
    /// Get Thing metadata/header.
    pub const SYS_THING_GET: u32 = 30;
    /// Iterate relationships (from).
    pub const SYS_REL_GET_FROM: u32 = 31;
    /// Iterate relationships (to).
    pub const SYS_REL_GET_TO: u32 = 32;

    /// Find a Thing by name (returns ID or 0 if not found).
    /// Input: a0=name_ptr, a1=name_len.
    pub const SYS_THING_FIND: u32 = 33;
    
    /// Register a name for a Thing.
    /// Input: a0=thing_id.low (or handle?), a1=name_ptr, a2=name_len.
    pub const SYS_THING_REGISTER_NAME: u32 = 34;


    // === Memory ===
    /// Create a Bytespace.
    pub const SYS_BYTESPACE_CREATE: u32 = 40;
    /// Map a Bytespace to an AddressSpace.
    pub const SYS_SPACE_MAP: u32 = 41;
    /// Unmap a region.
    pub const SYS_SPACE_UNMAP: u32 = 42;
    /// Grow heap (Legacy/Convenience for thing_std).
    pub const SYS_HEAP_GROW: u32 = 48; // Keeping distinct from map for now

    // === Watch ===
    /// Create a Watch.
    pub const SYS_WATCH_CREATE: u32 = 50;
    /// Poll a Watch.
    pub const SYS_WATCH_POLL: u32 = 51;

    // === Process / Task ===
    /// Spawn a process.
    pub const SYS_PROC_SPAWN: u32 = 100;
    /// Exit current process.
    pub const SYS_PROC_EXIT: u32 = 101;
    /// Yield time slice.
    pub const SYS_SCHED_YIELD: u32 = 102;
    
    // === Symbol Management ===
    /// Intern a symbol.
    pub const SYS_SYMBOL_INTERN: u32 = 110;
}

/// Standard Error Codes
pub mod err {
    pub const EPERM: i32 = -1;
    pub const ENOENT: i32 = -2;
    pub const ESRCH: i32 = -3;
    pub const EINTR: i32 = -4;
    pub const EIO: i32 = -5;
    pub const ENXIO: i32 = -6;
    pub const E2BIG: i32 = -7;
    pub const ENOEXEC: i32 = -8;
    pub const EBADF: i32 = -9;
    pub const ECHILD: i32 = -10;
    pub const EAGAIN: i32 = -11;
    pub const ENOMEM: i32 = -12;
    pub const EACCES: i32 = -13;
    pub const EFAULT: i32 = -14;
    pub const EINVAL: i32 = -22;
    pub const ENOSYS: i32 = -38;
}
