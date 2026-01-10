pub mod x86_64;

use crate::boot::ArchTrapFrame;

/// Triggers a synchronous yield trap.
/// This should be called by the scheduler or tasks to yield CPU time.
pub unsafe fn yield_trap() {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!("int 0x81");
    }
}

/// Central trap handler called by architecture-specific assembly stubs.
#[unsafe(no_mangle)]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn kernel_trap_handler(_tf: &mut dyn ArchTrapFrame) {
    #[cfg(target_arch = "x86_64")]
    {
        match _tf.trap_num() {
            0x81 => {
                // Yield
                unsafe {
                    let ptr = core::ptr::addr_of_mut!(crate::task::SCHEDULER);
                    if let Some(sched) = (*ptr).as_mut() {
                        sched.schedule(_tf);
                    }
                }
            }
            0xe => {
                // Page Fault - just panic for now to see it
                panic!("Page Fault at {:#x}", _tf.user_ip());
            }
            _ => {
                panic!("Unhandled Trap {:#x} at {:#x}", _tf.trap_num(), _tf.user_ip());
            }
        }
    }
}
