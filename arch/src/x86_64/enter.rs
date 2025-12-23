use super::super::UserEntryRegs;
use crate::gdt;
use core::arch::global_asm;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::paging::{
    Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};
extern crate alloc;
use alloc::alloc::{Layout, alloc_zeroed};
use x86_64::registers::control::{Cr3, Cr3Flags};
use x86_64::structures::paging::PhysFrame as X86PhysFrame;

#[repr(C)]
struct X86UserEntryRegs {
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub user_cs: u64,
    pub user_ss: u64,
    pub rdi: u64,
}

global_asm!(
    r#"
.global enter_user_mode_asm
enter_user_mode_asm:
    mov rcx, rdi
    mov r10, [rcx + 0]  // rip
    mov r11, [rcx + 8]  // rsp
    mov rdx, [rcx + 16] // rflags
    mov r8,  [rcx + 24] // user_cs
    mov r9,  [rcx + 32] // user_ss
    mov rdi, [rcx + 40] // rdi (arg0)

    push r9      // ss
    push r11     // rsp
    push rdx     // rflags
    push r8      // cs
    push r10     // rip
    swapgs
    iretq

.global resume_user_mode_asm
resume_user_mode_asm:
    // RDI points to context array (TrapFrame layout)
    // We need to push everything to stack to restore (push order: SS .. RIP .. RAX .. R15)
    
    // Interrupt Frame
    mov rax, [rdi + 152] // SS
    push rax
    mov rax, [rdi + 144] // RSP
    push rax
    mov rax, [rdi + 136] // RFLAGS
    push rax
    mov rax, [rdi + 128] // CS
    push rax
    mov rax, [rdi + 120] // RIP
    push rax
    
    // GPRs
    // Push in order: RAX...R15 (offsets 112...0)
    
    mov rax, [rdi + 112] // RAX
    push rax
    mov rax, [rdi + 104] // RDI (This is actually context[13]=RDI if using stack order, but TrapFrame fields define order)
    // TrapFrame order: R15, R14, R13, R12, RBP, RBX, R11, R10, R9, R8, RCX, RDX, RSI, RDI, RAX
    // So RAX is at offset 112. RDI is at offset 104.
    
    mov rax, [rdi + 104] // RDI
    push rax
    mov rax, [rdi + 96]  // RSI
    push rax
    mov rax, [rdi + 88]  // RDX
    push rax
    mov rax, [rdi + 80]  // RCX
    push rax
    mov rax, [rdi + 72]  // R8
    push rax
    mov rax, [rdi + 64]  // R9
    push rax
    mov rax, [rdi + 56]  // R10
    push rax
    mov rax, [rdi + 48]  // R11
    push rax
    mov rax, [rdi + 40]  // RBX
    push rax
    mov rax, [rdi + 32]  // RBP
    push rax
    mov rax, [rdi + 24]  // R12
    push rax
    mov rax, [rdi + 16]  // R13
    push rax
    mov rax, [rdi + 8]   // R14
    push rax
    mov rax, [rdi + 0]   // R15
    push rax
    
    // Restore GPRs
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rax
    
    swapgs
    iretq

.global resume_kernel_mode_asm
resume_kernel_mode_asm:
    // RDI points to context
    // Load RSP first!
    mov rsp, [rdi + 144]

    // Interrupt frame (RIP, CS, RFLAGS). For CPL0 returns, iretq pops only these.
    mov rax, [rdi + 136] // RFLAGS
    push rax
    mov rax, [rdi + 128] // CS
    push rax
    mov rax, [rdi + 120] // RIP
    push rax

    // GPRs (RAX .. R15)
    mov rax, [rdi + 112] // RAX
    push rax
    mov rax, [rdi + 104] // RDI
    push rax
    mov rax, [rdi + 96]  // RSI
    push rax
    mov rax, [rdi + 88]  // RDX
    push rax
    mov rax, [rdi + 80]  // RCX
    push rax
    mov rax, [rdi + 72]  // R8
    push rax
    mov rax, [rdi + 64]  // R9
    push rax
    mov rax, [rdi + 56]  // R10
    push rax
    mov rax, [rdi + 48]  // R11
    push rax
    mov rax, [rdi + 40]  // RBX
    push rax
    mov rax, [rdi + 32]  // RBP
    push rax
    mov rax, [rdi + 24]  // R12
    push rax
    mov rax, [rdi + 16]  // R13
    push rax
    mov rax, [rdi + 8]   // R14
    push rax
    mov rax, [rdi + 0]   // R15
    push rax
    
    // Restore GPRs
    pop r15
    pop r14
    pop r13
    pop r12
    pop rbp
    pop rbx
    pop r11
    pop r10
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rax
    
    // No swapgs for kernel thread!
    iretq
"#
);

unsafe extern "C" {
    fn enter_user_mode_asm(regs: *const X86UserEntryRegs) -> !;
    fn resume_user_mode_asm(context: *const u64) -> !;
    fn resume_kernel_mode_asm(context: *const u64) -> !;
}

pub fn resume_user_mode(context: &[u64], fpu_context: &kernel::sched::FpuContext) -> ! {
    // Manually align a stack buffer to 16 bytes.
    // ... (omitted similar logic)
    // We allocate 512 + 16 bytes to ensure we can find a 16-byte aligned offset.
    let mut raw_buffer = [0u8; 512 + 16];
    let start_addr = raw_buffer.as_ptr() as usize;
    let align_offset = if start_addr % 16 == 0 {
        0
    } else {
        16 - (start_addr % 16)
    };

    let aligned_slice = &mut raw_buffer[align_offset..align_offset + 512];
    aligned_slice.copy_from_slice(&fpu_context.data);
    let mxcsr_offset = 24;
    let default_mxcsr: u32 = 0x1F80;
    aligned_slice[mxcsr_offset..mxcsr_offset + 4].copy_from_slice(&default_mxcsr.to_le_bytes());
    let aligned_ptr = aligned_slice.as_ptr();

    unsafe {
        core::arch::x86_64::_fxrstor(aligned_ptr);
        resume_user_mode_asm(context.as_ptr())
    }
}

pub fn resume_kernel_mode(context: &[u64]) -> ! {
    unsafe {
        resume_kernel_mode_asm(context.as_ptr())
    }
}

pub fn start_kernel_thread(entry: u64, stack_top: u64, arg: u64) -> ! {
    // Set stack, pass arg in RDI, and jump to entry. No iret frame needed for ring0->ring0 start.
    unsafe {
        core::arch::asm!(
            "mov rsp, {stack}",
            "mov rax, {entry}",
            "mov rdi, {arg}",
            "jmp rax",
            stack = in(reg) stack_top,
            entry = in(reg) entry,
            arg = in(reg) arg,
            options(noreturn)
        );
    }
}

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    let selectors = gdt::get_selectors();

    let raw_top = regs.user_stack;

    // Align down to 16
    let mut rsp = raw_top & !0xFu64;

    // Simulate a return address slot like a real `call` would push.
    // This makes many compilers/higher-level assumptions happier.
    rsp -= 8;

    // Optionally write a 0 "return address" (not required, but nice for debuggability)
    // NOTE: This assumes we are entering the address space where `rsp` is valid!
    // Since we are about to iretq to it, we better be in the right Cr3.
    // However, we are in kernel mode here. If SMAP/SMEP is on, this might fault if not handled.
    // Given the user instructions explicitly included it, we will include it but wrapped in unsafe.
    // If it causes faults (e.g. page not mapped in kernel, or S-bit protection), we might need to remove it.
    // But for "user entry", the stack page should be user-accessible. Kernel accessing user page usually requires stac/clac on x86 if SMAP is on.
    // To be safe against SMAP, we should probably SKIP the write unless we know SMAP is off or we use user_access primitives.
    // The user's snippet didn't show stac/clac.
    // I will Include it as requested but with a comment.
    // Actually, if I look at `copy_segment_bytes` in elf_loader, it writes to user memory using direct pointer (with HHDM?).
    // Ah, `copy_segment_bytes` uses `frame_phys + hhdm`. That is a kernel mapping (direct map).
    // `rsp` here is a USER virtual address. safely writing to it requires mapping lookup or `stac`.
    // I'll skip the write to be safe to avoid unneeded faults, satisfying "Optionally".
    // Wait, the user said "If your entry stack alignment is wrong... nonsense".
    // The write is just for debuggability. The adjustment `rsp -= 8` is the fix.

    let x86_regs = X86UserEntryRegs {
        rip: regs.entry_point,
        rsp,           // Use adjusted RSP
        rflags: 0x202, // IF=1
        user_cs: selectors.ucode.0 as u64 | 3,
        user_ss: selectors.udata.0 as u64 | 3,
        rdi: regs.arg0,
    };

    kernel::println!(
        "enter_user_mode selectors: cs={:#x}, ss={:#x}",
        x86_regs.user_cs,
        x86_regs.user_ss,
    );

    // Diagnostic checks: ensure the selectors have RPL==3 and stack/rip look sane.
    if (x86_regs.user_cs & 0x3) != 0x3 || (x86_regs.user_ss & 0x3) != 0x3 {
        kernel::println!(
            "Invalid selector RPLs: user_cs={:#x}, user_ss={:#x}",
            x86_regs.user_cs,
            x86_regs.user_ss,
        );
        // Dump the rcx struct contents to help debugging (rip, rsp, rflags, cs, ss, rdi)
        unsafe {
            let p = &x86_regs as *const X86UserEntryRegs as *const u64;
            for i in 0..6 {
                let v = core::ptr::read(p.add(i));
                kernel::println!("rcx[{}] = {:#x}", i, v);
            }
        }
        kernel::println!("Aborting user entry to avoid GP; spinning.");
        loop {}
    }

    if x86_regs.rsp == 0 {
        kernel::println!("Invalid user RSP == 0; aborting enter_user_mode");
        unsafe {
            let p = &x86_regs as *const X86UserEntryRegs as *const u64;
            for i in 0..6 {
                let v = core::ptr::read(p.add(i));
                kernel::println!("rcx[{}] = {:#x}", i, v);
            }
        }
        loop {}
    }

    // If diagnostics pass, go to assembly entry.
    unsafe { enter_user_mode_asm(&x86_regs as *const _) }
}

static mut PHYS_MEM_OFFSET: u64 = 0;

pub unsafe fn init_user_stack(phys_mem_offset: u64) {
    unsafe {
        PHYS_MEM_OFFSET = phys_mem_offset;
    }

    // Map user_thread_main as user accessible
    // Note: This assumes user_thread_main is available.
    // We might need to pass the address or handle it differently.
    // For now, we will skip mapping user_thread_main here and rely on page fault handler or do it if we can access the symbol.
    // Since user_thread_main is in crate::user, we can access it.

    let level_4_table_ptr = x86_64::registers::control::Cr3::read()
        .0
        .start_address()
        .as_u64();
    let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
    let level_4_table: &mut PageTable = unsafe { &mut *level_4_table_ptr.as_mut_ptr() };

    let mut mapper = unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset)) };

    let code_start = VirtAddr::from_ptr(crate::user::user_thread_main as *const ());
    let code_page = Page::<Size4KiB>::containing_address(code_start);
    let code_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;

    match unsafe { mapper.update_flags(code_page, code_flags) } {
        Ok(flush) => flush.flush(),
        Err(_) => {
            kernel::log("Failed to update user code flags");
        }
    }
}

const USER_STACK_SIZE: usize = 64 * 1024;

pub fn alloc_user_stack() -> u64 {
    let layout = Layout::from_size_align(USER_STACK_SIZE, 16).expect("invalid user stack layout");
    let stack_ptr = unsafe { alloc_zeroed(layout) };
    let stack_ptr = NonNull::new(stack_ptr).expect("alloc_user_stack: allocation failed");
    let stack_addr = stack_ptr.as_ptr() as u64;

    unsafe {
        let phys_mem_offset = PHYS_MEM_OFFSET;
        let level_4_table_ptr = x86_64::registers::control::Cr3::read()
            .0
            .start_address()
            .as_u64();
        let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
        let level_4_table: &mut PageTable = &mut *level_4_table_ptr.as_mut_ptr();

        let mut mapper = OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset));

        let start_page = Page::<Size4KiB>::containing_address(VirtAddr::new(stack_addr));
        let end_page = Page::<Size4KiB>::containing_address(VirtAddr::new(
            stack_addr + USER_STACK_SIZE as u64 - 1,
        ));
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        for page in Page::range_inclusive(start_page, end_page) {
            if let Ok(flush) = mapper.update_flags(page, flags) {
                flush.flush();
            } else {
                kernel::log("Failed to update user stack flags");
            }
        }
    }

    let stack_top = stack_addr + USER_STACK_SIZE as u64;
    kernel::println!(
        "alloc_user_stack: bottom={:#x}, top={:#x}",
        stack_addr,
        stack_top
    );
    stack_top
}
static KERNEL_CR3: AtomicU64 = AtomicU64::new(0);

fn ensure_kernel_cr3_recorded() -> u64 {
    let stored = KERNEL_CR3.load(Ordering::SeqCst);
    if stored != 0 {
        return stored;
    }
    let current = Cr3::read().0.start_address().as_u64();
    KERNEL_CR3.store(current, Ordering::SeqCst);
    current
}

pub fn activate_address_space(token: Option<u64>) {
    let kernel_cr3 = ensure_kernel_cr3_recorded();
    let target = token.unwrap_or(kernel_cr3);
    let current = Cr3::read().0.start_address().as_u64();
    if current == target {
        return;
    }
    let frame =
        X86PhysFrame::from_start_address(PhysAddr::new(target)).expect("Invalid CR3 frame address");
    unsafe {
        Cr3::write(frame, Cr3Flags::empty());
    }
}
