use core::arch::naked_asm;
use x86_64::registers::model_specific::{Efer, EferFlags, LStar, SFMask, Star};
use x86_64::registers::rflags::RFlags;
use x86_64::VirtAddr;
use crate::gdt::{KERNEL_CODE_SELECTOR, KERNEL_DATA_SELECTOR, USER_CODE_SELECTOR, USER_DATA_SELECTOR};

pub unsafe fn init() {
    // 1. Enable syscall/sysret instruction via EFER
    let mut efer = Efer::read();
    efer.insert(EferFlags::SYSTEM_CALL_EXTENSIONS);
    unsafe {
        Efer::write(efer);
    }

    // 2. Setup LSTAR (Target address for syscall)
    LStar::write(VirtAddr::new(syscall_handler_naked as *const () as u64));

    // 3. Setup STAR (Segment selectors)
    // Star::write check:
    // fn write(cs_sysret, ss_sysret, cs_syscall, ss_syscall)
    // We pass UserCode, UserData, KernelCode, KernelData.
    // Logic:
    // Sysret: CS = Sysret_Field + 16, SS = Sysret_Field + 8.
    // If we pass UserCode (idx 4) and UserData (idx 3).
    // The underlying `write` implementation does the math to find Base.
    // It verifies that CS = SS + 8.
    
    // Safety: we trust our GDT layout.
    unsafe {
        Star::write(
            USER_CODE_SELECTOR,
            USER_DATA_SELECTOR,
            KERNEL_CODE_SELECTOR,
            KERNEL_DATA_SELECTOR
        ).expect("Failed to setup STAR");
    }

    // 4. Setup SFMask (RFlags mask)


    // 4. Setup SFMask (RFlags mask)
    // Disables interrupts (IF=0x200), Direction (DF=0x400)?
    // Usually we clear IF, TF, DF, AC.
    SFMask::write(RFlags::INTERRUPT_FLAG | RFlags::TRAP_FLAG); 
}

// ABI:
// Syscall # in RAX.
// Args: RDI, RSI, RDX, R10, R8, R9.
// Return: RAX.
// Clobbers: RCX, R11.
// Kernel Stack: We must switch to kernel stack!
// Where is it?
// We utilize `swapgs` to get GS Base, which points to PCR (Processor Control Region) or just Thread pointer.
// We need to find the Kernel Stack for the current thread.
//
// For V0 grafting:
// We might assume single core and use a global TSS scratch or something?
// No, `bridge` has `TICK_HOOK` but lacks a dedicated "Get Current Thread" function easily accessible from assembly?
// `enter.rs` uses `KERNEL_CR3`.
//
// We need a place to save user RSP.
// Recommended: GS Base points to a per-cpu struct containing `kernel_stack_top` and `user_rsp_scratch`.
//
// How to set GS Base?
// `Kernel::scheduler` manages threads.
// When switching to user, we should set GS Base to the Thread Control Block (TCB) or similar.
//
// In `sched/mod.rs` (grafting), `Thread` struct exists.
// We should update GS Base on context switch.
//
// If we don't have GS switch logic yet:
// We can use a global scratch variable for single core.
// `static mut KERNEL_STACK_SCRATCH: u64 = 0;`
// `static mut USER_RSP_SCRATCH: u64 = 0;`
// This is not thread-safe but ok for v0 single core.
//
// Wait, `bridge` needs to handle this.
// `timer_interrupt_naked` handles `iretq` stack, but `syscall` doesn't use stack for transition.
// `syscall` overwrites RCX (RIP) and R11 (RFlags).
// Does NOT touch Stack. RSP is still User Stack.
// We MUST `mov [SCRATCH], rsp` then `mov rsp, [KERNEL_STACK]`.
//
// Where do we get KERNEL_STACK?
// From `TSS.privilege_stack_table[0]`? (Rsp0).
// We can access TSS? No strict address known.
//
// We really need GS.
//
// Let's verify if `init` sets GS.
// `main.rs`: `Kernel::new(Bridge)`.
// `scheduler` is inside `Kernel`.
//
// Let's look at `crates/bridge_x86_64/src/interrupts/idt.rs`. It uses `iretq`.
// `timer_interrupt_naked` uses `swapgs`. This implies GS IS being used.
// If `swapgs` is used, then the kernel expects GS to swap between correct Kernel GS and User GS.
// User GS: ???
// Kernel GS: Per-CPU data.
//
// Whatever `timer_interrupt_naked` relies on (Interrupt Stack Table?), it assumes `swapgs` works.
// Note: `enter.rs` `resume_user_mode_asm` also performs `swapgs`.
//
// So we assume that when in User Mode, `swapgs` switches us to Kernel GS.
// And Kernel GS contains our kernel stack?
//
// If we haven't set up Kernel GS, `swapgs` will swap valid `GS_Base` with `KernelGS_Base` (MSR 0xC0000102 vs 0xC0000101).
// `limine` might set something?
//
// Actually `bridge_x86_64::gdt::init` initialized TSS.
// But we didn't see GS setup.
//
// If GS is not set up, `swapgs` swaps 0 with 0.
// And we have no scratch space.
//
// Workaround for V0 without Per-CPU struct:
// Use a fixed global address for scratch.
// `static mut SYSCALL_SCRATCH: u64 = 0;`
// `static mut SYSCALL_KSTACK: u64 = ...`
//
// We can export `set_syscall_stack(u64)`?
//
// Let's define the naked handler using global scratch symbols defined in this file.

#[unsafe(no_mangle)]
static mut SYSCALL_USER_RSP: u64 = 0;
#[unsafe(no_mangle)]
static mut SYSCALL_KERNEL_RSP: u64 = 0;

// Caller (Kernel/Scheduler) must set `SYSCALL_KERNEL_RSP` before running user code?
// Or we set it on context switch.
// But `bridge` doesn't know about `Thread`.
// `resume_user_mode` takes `context` which includes `rsp`.
// But it doesn't inspect "kernel stack" of target thread.
//
// Wait! `TrapFrame` / interrupt logic uses `TSS.RSP0` for ring switch.
// `gdt.rs` set `TSS.privilege_stack_table[0] = ...`.
//
// Syscall does NOT use TSS.
// So we must manually load RSP0 from TSS or cached variable.
//
// Let's add `update_syscall_stack(u64)` to `Bridge` trait or impl.
// And call it from scheduler.
//
// For now, let's implement the handler assuming `SYSCALL_KERNEL_RSP` is valid.
// `kernels/x86_64/src/main.rs`: `BOOT_STACK`.
//
// BUT: We have multiple threads (idle, init, drivers, etc.).
// Each has its own kernel stack.
// If we use a global static, we are assuming single thread or we update it on switch.
//
// The correct way: Update on switch.
// `resume_user_mode` in `bridge` should updated it?
// `bridge` `resume_user_mode` takes `context`.
// It doesn't take "kernel stack top".
//
// In V0, everything shares BOOT_STACK? No. `spawn` creates stacks.
//
// OK, `syscall_handler_naked` logic:
// 1. swapgs (Assuming KernelGS setup? If not, we skip GS dependency and use fixed globals).
//    Since we don't have GS setup code visible, I'll rely on fixed globals because we are single-core.
//    Atomic swap of RSP?
//
//    "mov [SYSCALL_USER_RSP], rsp"
//    "mov rsp, [SYSCALL_KERNEL_RSP]"
//    "sti" (Syscall disables interrupts via SFMASK, we want them enabled in kernel eventually? Or just handle syscall with interrupts off?)
//    Usually enable.
//
//    "call syscall_rust_entry"
//
//    "cli"
//    "mov rsp, [SYSCALL_USER_RSP]"
//    "sysretq"
//
// We need to EXPOSE `SYSCALL_KERNEL_RSP` so scheduler can update it.
//
// Let's implement `set_kernel_stack` in this module.

#[unsafe(naked)]
unsafe extern "C" fn syscall_handler_naked() {
    naked_asm!(
        // 1. Save User RSP
        "mov gs:[0], rsp", // Wait, do we have GS? 
        // If no GS, use RIP-relative addressing to global? 
        // Only if mapped.
        // Let's try RIP-relative to `SYSCALL_USER_RSP`.
        "mov [rip + {0}], rsp",
        
        // 2. Load Kernel RSP
        "mov rsp, [rip + {1}]",
        
        // 3. Save User context preserved by Syscall (RCX=RIP, R11=RFlags)
        // We need to preserve them across Rust call.
        "push rcx", // User RIP
        "push r11", // User RFLAGS
        "push rbp", // Callee-saved
        "push rbx",
        "push r12", 
        "push r13", 
        "push r14", 
        "push r15",

        // 4. Setup arguments for Rust function
        // Syscall ABI: RAX (num), RDI, RSI, RDX, R10, R8, R9.
        // Rust dispatch: fn(num, a1, a2, a3, a4, a5, a6)
        // RDI = RAX (Move RAX to RDI)
        // RSI = RDI
        // RDX = RSI
        // RCX = RDX
        // R8  = R10
        // R9  = R8
        // Stack = R9
        
        // Let's just pass a pointer to a struct or use registers directly.
        // Rust ABI (SystemV): RDI, RSI, RDX, RCX, R8, R9.
        // We have 7 inputs (Num + 6 args).
        // Let's simplify: fn(num, a1, a2, a3) -> SysRet.
        // RDI = RAX
        // RSI = RDI
        // RDX = RSI
        // RCX = RDX
        // R8 = R10
        // R9 = R8
        
        "mov r9, r8",
        "mov r8, r10",
        "mov rcx, rdx",
        "mov rdx, rsi",
        "mov rsi, rdi",
        "mov rdi, rax",
        
        // Enable interrupts?
        "sti",
        
        "call syscall_dispatch",
        
        "cli",
        
        // Restore
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbx",
        "pop rbp",
        "pop r11",
        "pop rcx",
        
        // Restore RSP
        "mov rsp, [rip + {0}]",
        
        // Return (RCX=RIP, R11=RFLAGS handled by sysret)
        "sysretq",
        
        sym SYSCALL_USER_RSP,
        sym SYSCALL_KERNEL_RSP
    );
}

#[no_mangle]
extern "C" fn syscall_dispatch(
    num: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize, a6: usize
) -> isize {
    unsafe {
        if let Some(hook) = SYSCALL_HOOK {
            hook(num, a1, a2, a3, a4, a5, a6)
        } else {
            -1
        }
    }
}

pub static mut SYSCALL_HOOK: Option<fn(usize, usize, usize, usize, usize, usize, usize) -> isize> = None;

pub fn set_syscall_hook(hook: fn(usize, usize, usize, usize, usize, usize, usize) -> isize) {
    unsafe { SYSCALL_HOOK = Some(hook); }
}

pub fn set_kernel_stack(stack_top: u64) {
    unsafe {
        SYSCALL_KERNEL_RSP = stack_top;
    }
}

