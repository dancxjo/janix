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
    iretq

.global resume_user_mode_asm
resume_user_mode_asm:
    // RDI points to context array (SyscallRegs layout)
    // We need to push everything to stack to restore
    
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
    // GPRs
    // Push in order: RAX...R15 (offsets 0...112)
    // Stack grows down, so to pop R15...RAX later, we must push RAX first (deepest) and R15 last (top).
    
    mov rax, [rdi + 0]   // RAX
    push rax
    mov rax, [rdi + 8]   // RDI
    push rax
    mov rax, [rdi + 16]  // RSI
    push rax
    mov rax, [rdi + 24]  // RDX
    push rax
    mov rax, [rdi + 32]  // RCX
    push rax
    mov rax, [rdi + 40]  // R8
    push rax
    mov rax, [rdi + 48]  // R9
    push rax
    mov rax, [rdi + 56]  // R10
    push rax
    mov rax, [rdi + 64]  // R11
    push rax
    mov rax, [rdi + 72]  // RBX
    push rax
    mov rax, [rdi + 80]  // RBP
    push rax
    mov rax, [rdi + 88]  // R12
    push rax
    mov rax, [rdi + 96]  // R13
    push rax
    mov rax, [rdi + 104] // R14
    push rax
    mov rax, [rdi + 112] // R15
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
    
    iretq
"#
);

unsafe extern "C" {
    fn enter_user_mode_asm(regs: *const X86UserEntryRegs) -> !;
    fn resume_user_mode_asm(context: *const u64) -> !;
}

pub fn resume_user_mode(context: &[u64]) -> ! {
    unsafe { resume_user_mode_asm(context.as_ptr()) }
}

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    let selectors = gdt::get_selectors();

    let x86_regs = X86UserEntryRegs {
        rip: regs.entry_point,
        rsp: regs.user_stack,
        rflags: 0x202, // IF=1
        user_cs: selectors.ucode.0 as u64 | 3,
        user_ss: selectors.udata.0 as u64 | 3,
        rdi: regs.arg0,
    };

    kernel_core::println!(
        "enter_user_mode selectors: cs={:#x}, ss={:#x}",
        x86_regs.user_cs,
        x86_regs.user_ss,
    );

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
            kernel_core::log("Failed to update user code flags");
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
                kernel_core::log("Failed to update user stack flags");
            }
        }
    }

    let stack_top = stack_addr + USER_STACK_SIZE as u64;
    kernel_core::println!(
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
