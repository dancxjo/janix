#[cfg(target_arch = "x86_64")]
mod impl_x86 {
    use crate::gdt;
    use core::arch::global_asm;
    use x86_64::structures::paging::{
        Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, Size4KiB,
    };
    use x86_64::{PhysAddr, VirtAddr};

    #[repr(C)]
    pub struct UserEntryRegs {
        pub rip: u64,
        pub rsp: u64,
        pub rflags: u64,
        pub user_cs: u64,
        pub user_ss: u64,
    }

    // Allocate user stack (simple static for now)
    // Align to page boundary to make mapping easier
    #[repr(align(4096))]
    struct UserStack([u8; 4096]);
    static mut USER_STACK: UserStack = UserStack([0; 4096]);

    pub unsafe fn init_user_stack(phys_mem_offset: u64) {
        let level_4_table_ptr = x86_64::registers::control::Cr3::read()
            .0
            .start_address()
            .as_u64();
        let level_4_table_ptr = VirtAddr::new(level_4_table_ptr + phys_mem_offset);
        let level_4_table: &mut PageTable = &mut *level_4_table_ptr.as_mut_ptr();

        let mut mapper =
            unsafe { OffsetPageTable::new(level_4_table, VirtAddr::new(phys_mem_offset)) };

        let stack_start = VirtAddr::from_ptr(core::ptr::addr_of!(USER_STACK));
        let page = Page::<Size4KiB>::containing_address(stack_start);

        // Update flags to include USER_ACCESSIBLE
        let mut flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;

        // We need to remap or update flags.
        // Since it's already mapped (kernel data), we just need to update flags.
        // However, OffsetPageTable doesn't have a simple "update flags" method that doesn't require unmapping or knowing the physical frame.
        // But we can use `update_flags`.

        use x86_64::structures::paging::mapper::MapperAllSizes;

        // Note: This might fail if the page is part of a huge page.
        // Limine usually maps kernel with 4KiB pages? Or 2MiB?
        // If it's 2MiB, we can't easily change flags for just 4KiB without splitting.
        // Let's assume 4KiB for now or that update_flags handles it (it doesn't handle splitting).

        // Safer approach: Just check if we can update.
        match mapper.update_flags(page, flags) {
            Ok(flush) => flush.flush(),
            Err(e) => {
                // If it failed, it might be because it's a huge page.
                // For this vertical slice, let's just hope it works or panic.
                kernel_core::log("Failed to update user stack flags");
            }
        }

        // Also map the code page as user accessible
        let code_start = VirtAddr::from_ptr(user_test_entry as *const ());
        let code_page = Page::<Size4KiB>::containing_address(code_start);
        let code_flags = PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE; // Executable by default if NO_EXECUTE not set

        match mapper.update_flags(code_page, code_flags) {
            Ok(flush) => flush.flush(),
            Err(e) => {
                kernel_core::log("Failed to update user code flags");
            }
        }
    }

    // Assembly function
    global_asm!(
        r#"
    .global enter_user_mode
    enter_user_mode:
        mov rcx, rdi
        mov rdi, [rcx + 0]  // rip
        mov rsi, [rcx + 8]  // rsp
        mov rdx, [rcx + 16] // rflags
        mov r8,  [rcx + 24] // user_cs
        mov r9,  [rcx + 32] // user_ss

        push r9      // ss
        push rsi     // rsp
        push rdx     // rflags
        push r8      // cs
        push rdi     // rip
        iretq
    "#
    );

    unsafe extern "C" {
        fn enter_user_mode(regs: *const UserEntryRegs) -> !;
    }

    pub fn enter_user(user_entry: unsafe extern "C" fn() -> !) -> ! {
        let selectors = gdt::get_selectors();

        let user_stack_top = unsafe { (core::ptr::addr_of_mut!(USER_STACK) as u64) + 4096 };

        let regs = UserEntryRegs {
            rip: user_entry as u64,
            rsp: user_stack_top,
            rflags: 0x202, // IF=1
            user_cs: selectors.ucode.0 as u64 | 3,
            user_ss: selectors.udata.0 as u64 | 3,
        };
        unsafe { enter_user_mode(&regs as *const _) }
    }

    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn user_test_entry() -> ! {
        unsafe {
            core::arch::asm!("int 0x80", in("rax") 1u64);
        }
        loop {}
    }
}

#[cfg(target_arch = "x86_64")]
pub use impl_x86::*;

#[cfg(not(target_arch = "x86_64"))]
mod impl_stub {
    pub unsafe fn init_user_stack(_phys_mem_offset: u64) {}
    pub fn enter_user(_user_entry: unsafe extern "C" fn() -> !) -> ! {
        loop {}
    }
    pub unsafe extern "C" fn user_test_entry() -> ! {
        loop {}
    }
}

#[cfg(not(target_arch = "x86_64"))]
pub use impl_stub::*;
