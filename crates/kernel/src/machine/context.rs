//! Cross-architecture context switching abstraction.
//!
//! This module defines the "preemption contract" that all architectures must implement.
//! The scheduler consumes these traits without knowing arch-specific details.

/// CPU execution mode (privilege level abstraction).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuMode {
    Kernel,
    User,
}

/// Portable trap information extracted from arch-specific TrapFrame.
#[derive(Debug, Clone, Copy)]
pub struct TrapInfo {
    pub vector: u32,
    pub mode: CpuMode,
    pub pc: u64,   // RIP / ELR / SEPC / ERA
    pub sp: u64,   // Stack pointer
    pub flags: u64, // RFLAGS / SPSR / SSTATUS / CRMD
}

/// Portable resume specification for returning to a task.
#[derive(Debug, Clone, Copy)]
pub struct ResumeSpec {
    pub mode: CpuMode,
    pub interrupts_enabled: bool,
}

impl Default for ResumeSpec {
    fn default() -> Self {
        Self {
            mode: CpuMode::Kernel,
            interrupts_enabled: true,
        }
    }
}

/// Task construction (spawning new tasks).
///
/// Separated from trap handling to avoid coupling spawn changes with interrupt paths.
pub trait ArchTask {
    /// Opaque task context stored in task struct.
    type TaskContext: 'static + Default;

    /// Initialize a new task context for kernel or user entry.
    ///
    /// # Arguments
    /// * `ctx` - Output task context to initialize
    /// * `entry` - Entry point address
    /// * `stack_top` - Top of stack (highest address)
    /// * `mode` - Kernel or User mode
    /// * `arg0` - First argument to pass to entry point
    fn init_task_context(
        ctx: &mut Self::TaskContext,
        entry: u64,
        stack_top: u64,
        mode: CpuMode,
        arg0: u64,
    );
}

/// Trap handling (interrupt entry/exit).
///
/// # TrapFrame Invariant
///
/// The `TrapFrame` type MUST represent a "normalized" full frame created by
/// architecture entry stubs. This means:
/// - x86_64: Always 5-word iretq frame (SS/RSP/RFLAGS/CS/RIP) even for kernel→kernel
/// - aarch64: Full ExceptionContext with all GPRs, ELR_EL1, SPSR_EL1
/// - riscv64: Full TrapContext with all regs, sepc, sstatus
/// - loongarch64: Full TrapContext with all regs, era, prmd
///
/// Raw CPU-pushed mini-frames (e.g. x86 3-word kernel interrupt) must be
/// normalized by the entry stub before calling into Rust.
pub trait ArchTrap {
    /// Normalized trap frame. MUST be canonical full layout.
    type TrapFrame: 'static;
    /// Task context (usually same as ArchTask::TaskContext).
    type TaskContext: 'static;

    /// Extract portable trap info from arch-specific frame.
    fn summarize(tf: &Self::TrapFrame) -> TrapInfo;

    /// Get CPU mode from trap frame.
    fn mode(tf: &Self::TrapFrame) -> CpuMode;

    /// Save current execution into task context from trap frame.
    fn save_from_trap(tf: &Self::TrapFrame, out: &mut Self::TaskContext);

    /// Load task context into trap frame for return.
    fn load_into_trap(ctx: &Self::TaskContext, tf: &mut Self::TrapFrame);

    /// Apply resume specification to trap frame (mode, interrupt flags).
    fn apply_resume_spec(tf: &mut Self::TrapFrame, spec: ResumeSpec);

    /// Return from trap. Never returns.
    ///
    /// # Safety
    /// * `tf` must point to a canonical normalized TrapFrame on the stack
    /// * The frame must be in the exact layout expected by arch return instruction
    unsafe fn return_from_trap(tf: *const Self::TrapFrame) -> !;
}

/// Combined context trait for architectures implementing both task and trap handling.
pub trait ArchContext: ArchTask + ArchTrap<TaskContext = <Self as ArchTask>::TaskContext> {}

// Blanket implementation
impl<T> ArchContext for T where T: ArchTask + ArchTrap<TaskContext = <T as ArchTask>::TaskContext> {}
