use crate::gdt;
use core::arch::global_asm;
use super::super::UserEntryRegs;
use x86_64::structures::paging::{
    Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB,
};
use x86_64::{PhysAddr, VirtAddr};
extern crate alloc;
use alloc::boxed::Box;
use x86_64::structures::paging::mapper::MapperAllSizes;

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
"#
);

unsafe extern "C" {
    fn enter_user_mode_asm(regs: *const X86UserEntryRegs) -> !;
}

pub fn enter_user_mode(regs: &UserEntryRegs) -> ! {
    let selectors = gdt::get_selectors();
    
    let x86_regs = X86UserEntryRegs {
        rip: regs.entry_point,
        rsp: regs.user_stack,
        rflags: 0x202, // IF=1
        user_cs: unsafe { selectors.ucode.0 as u64 | 3 },
        user_ss: unsafe { selectors.udata.0 as u64 | 3 },
        rdi: regs.arg0,
    };
    
    unsafe { enter_user_mode_asm(&x86_regs as *const _) }
}

static mut PHYS_MEM_OFFSET: u64 = 0;

pub unsafe fn init_user_stack(phys_mem_offset: u64) {
    unsafe { PHYS_MEM_OFFSET = phys_mem_offset; }
    
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
    let level_4_table: &mut PageTable = &mut *level_4_table_ptr.as_mut_ptr();

    let mut mapper =
        unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset)) };

    let code_start = VirtAddr::from_ptr(crate::user::user_thread_main as *const ());
    let code_page = Page::<Size4KiB>::containing_address(code_start);
    let code_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE;

    match mapper.update_flags(code_page, code_flags) {
        Ok(flush) => flush.flush(),
        Err(_) => {
            kernel_core::log("Failed to update user code flags");
        }
    }
}

pub fn alloc_user_stack() -> u64 {
    let stack = Box::new([0u8; 4096]);
    let stack_ptr = Box::leak(stack).as_mut_ptr();
    let stack_addr = stack_ptr as u64;
    
    unsafe {
        let phys_mem_offset = PHYS_MEM_OFFSET;
        let level_4_table_ptr = x86_64::registers::control::Cr3::read()
            .0
            .start_address()
            .as_u64();
        let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
        let level_4_table: &mut PageTable = &mut *level_4_table_ptr.as_mut_ptr();

        let mut mapper = OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset));
        
        let page = Page::<Size4KiB>::containing_address(VirtAddr::new(stack_addr));
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
        
        if let Ok(flush) = mapper.update_flags(page, flags) {
            flush.flush();
        } else {
            kernel_core::log("Failed to update user stack flags");
        }
    }
    
    stack_addr + 4096
}
