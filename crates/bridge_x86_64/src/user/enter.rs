use super::UserEntryRegs;
use alloc::alloc::{alloc_zeroed, Layout};
use core::sync::atomic::{AtomicU64, Ordering};
use kernel_core::sched::fpu::FpuContext;
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PhysFrame as X86PhysFrame;
use x86_64::PhysAddr;

// Hardcoded selectors corresponding to standard ThingOS layout (KCode=8, KData=16, TSS=24, UData=40, UCode=48)
const USER_CODE_SELECTOR: u16 = 0x30 | 3; // RPL 3
const USER_DATA_SELECTOR: u16 = 0x28 | 3; // RPL 3

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    let x86_regs = X86UserEntryRegs {
        rip: regs.entry_point,
        rsp: regs.user_stack & !0xF, // Align 16
        rflags: 0x202,               // IF=1, bit 1=1
        user_cs: USER_CODE_SELECTOR as u64,
        user_ss: USER_DATA_SELECTOR as u64,
        rdi: regs.arg0,
    };

    // Safety: Jumping to user mode. Assumes generic bridge setup calls this.
    unsafe { enter_user_mode_asm(&x86_regs as *const _) }
}

#[repr(C)]
struct X86UserEntryRegs {
    rip: u64,
    rsp: u64,
    rflags: u64,
    user_cs: u64,
    user_ss: u64,
    rdi: u64,
}

#[unsafe(naked)]
unsafe extern "C" fn enter_user_mode_asm(_regs: *const X86UserEntryRegs) -> ! {
    // rcx = regs pointer
    // ABI: rdi = regs pointer (System V). Wait, `extern "C"` on x86_64 (Linux/SystemV) uses RDI.
    // Confirm target conv.
    core::arch::naked_asm!(
        "mov rcx, rdi", // Move regs ptr to rcx (sysret convention placeholder or just GPR)
        // Load target registers
        "mov rdi, [rcx + 40]", // rdi = arg0 (offset 5 * 8)
        "mov r11, [rcx + 16]", // r11 = rflags
        // iretq stack setup
        "mov rax, [rcx + 32]", // ss
        "push rax",
        "mov rax, [rcx + 8]", // rsp
        "push rax",
        "push r11",            // rflags
        "mov rax, [rcx + 24]", // cs
        "push rax",
        "mov rax, [rcx]", // rip
        "push rax",
        // Clear segments?
        "mov ax, 0x2b", // User data 0x28 | 3
        "mov ds, ax",
        "mov es, ax",
        "mov fs, ax",
        "mov gs, ax",
        // swapgs if we were in kernel? usually yes if we return to user.
        "swapgs",
        "iretq"
    )
}

pub fn resume_user_mode(context: &[u64], _fpu_context: &FpuContext) -> ! {
    // Context layout: [r15...rax, rip, cs, rflags, rsp, ss]
    unsafe { resume_user_mode_asm(context.as_ptr()) }
}

#[unsafe(naked)]
unsafe extern "C" fn resume_user_mode_asm(_context: *const u64) -> ! {
    core::arch::naked_asm!(
        "mov rsp, rdi", // context ptr
        // Restore GPRs
        "pop r15",
        "pop r14",
        "pop r13",
        "pop r12",
        "pop rbp",
        "pop rbx",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rcx",
        "pop rdx",
        "pop rsi",
        "pop rdi",
        "pop rax",
        // Stack now has [rip, cs, rflags, rsp, ss]
        
        // Check CS (at rsp + 8) to decide on swapgs
        "test byte ptr [rsp + 8], 3",
        "jz 1f", // Jump if kernel (0)
        "swapgs",
        "iretq",

        "1:",
        // Kernel Return logic.
        // We have [RIP, CS, RFLAGS, GarbageRSP, GarbageSS] on stack.
        // iretq only pops top 3. We must move top 3 into bottom 3 slots to consume garbage.
        
        "push rax", // Save RAX as scratch
        
        // Stack offsets now +8
        // [rsp+24] = RFLAGS
        // [rsp+40] = Target for RFLAGS (SS slot)
        "mov rax, [rsp + 24]",
        "mov [rsp + 40], rax",
        
        // [rsp+16] = CS
        // [rsp+32] = Target for CS (RSP slot)
        "mov rax, [rsp + 16]",
        "mov [rsp + 32], rax",
        
        // [rsp+8] = RIP
        // [rsp+24] = Target for RIP (RFLAGS slot)
        "mov rax, [rsp + 8]",
        "mov [rsp + 24], rax",
        
        "pop rax", // Restore RAX
        
        "add rsp, 16", // Skip old RIP/CS slots
        "iretq"
    )
}

static mut PHYS_MEM_OFFSET: u64 = 0;

pub unsafe fn init_user_stack(phys_mem_offset: u64) {
    unsafe {
        PHYS_MEM_OFFSET = phys_mem_offset;
    }
    // Stub
}

pub fn alloc_user_stack() -> u64 {
    const USER_STACK_SIZE: usize = 64 * 1024;
    let layout = Layout::from_size_align(USER_STACK_SIZE, 16).unwrap();
    let ptr = unsafe { alloc_zeroed(layout) };
    let addr = ptr as u64;

    // Start/End mapping logic stubbed for now or reuse init code
    addr + USER_STACK_SIZE as u64
}

static KERNEL_CR3: AtomicU64 = AtomicU64::new(0);

pub fn activate_address_space(token: Option<u64>) {
    let current = Cr3::read().0.start_address().as_u64();
    if KERNEL_CR3.load(Ordering::Relaxed) == 0 {
        KERNEL_CR3.store(current, Ordering::Relaxed);
    }
    let target = token.unwrap_or_else(|| KERNEL_CR3.load(Ordering::Relaxed));
    if current != target {
        unsafe {
            let frame = X86PhysFrame::from_start_address(PhysAddr::new(target)).unwrap();
            Cr3::write(frame, Cr3Flags::empty());
        }
    }
}
