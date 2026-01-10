pub mod x86_64;

use crate::arch::ArchTrapFrame;

/// Triggers a synchronous yield trap.
/// This should be called by the scheduler or tasks to yield CPU time.
pub unsafe fn yield_trap() {
    #[cfg(target_arch = "x86_64")]
    core::arch::asm!("int 0x81");
}

/// Central trap handler called by architecture-specific assembly stubs.
#[no_mangle]
pub extern "C" fn kernel_trap_handler(tf: &mut ArchTrapFrame) {
    #[cfg(target_arch = "x86_64")]
    {
        match tf.trap_num {
            0x81 => {
                // Yield
                unsafe {
                    let ptr = core::ptr::addr_of_mut!(crate::task::SCHEDULER);
                    if let Some(sched) = (*ptr).as_mut() {
                        sched.schedule(tf);
                    }
                }
            }
            0xe => {
                // Page Fault - just panic for now to see it
                panic!("Page Fault at {:#x}", tf.rip);
            }
            _ => {
                panic!("Unhandled Trap {:#x} at {:#x}", tf.trap_num, tf.rip);
            }
        }
    }
}
