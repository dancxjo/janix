use crate::trap::x86_64::TrapFrame;
use crate::user::abi::{SYSCALL_PUTCHAR, SYSCALL_TICKS, SYSCALL_YIELD, SYSCALL_EXIT, SYSCALL_SPAWN_MODULE, SYSCALL_RTC_CMOS_READ};

#[unsafe(no_mangle)]
pub extern "C" fn syscall_dispatch(tf: &mut TrapFrame) {
    let nr = tf.rax;
    let ret = match nr {
        SYSCALL_PUTCHAR => {
            let c = tf.rdi as u8;
            crate::runtime().putchar(c);
            0
        },
        SYSCALL_TICKS => {
            crate::runtime().mono_ticks()
        },
        SYSCALL_YIELD => {
            crate::task::yield_now();
            0
        },
        SYSCALL_EXIT => {
            let _code = tf.rdi as i32;
            crate::kinfo!("User task exited with code {}", _code);
            // Mark task dead or just halt for now since we don't have task destruction
            loop {
                crate::task::yield_now();
            }
        },
        SYSCALL_SPAWN_MODULE => {
            let path_ptr = tf.rdi as *const u8;
            let path_len = tf.rsi as usize;
            
            // Validate user pointer (rudimentary)
            if path_ptr as u64 >= 0x8000_0000_0000_0000 {
                 crate::kwarn!("SpawnModule: invalid pointer {:p}", path_ptr);
                 u64::MAX // error
            } else {
                 match crate::user::sys_spawn_module(path_ptr, path_len) {
                     Ok(tid) => tid as u64,
                     Err(e) => {
                         crate::kwarn!("SpawnModule failed: {}", e);
                         match e {
                             // Map some errors to negative numbers if needed
                             _ => u64::MAX // -1 mostly
                         }
                     }
                 }
            }
        },
        SYSCALL_RTC_CMOS_READ => {
            let reg = tf.rdi as u8;
            match crate::user::sys_rtc_cmos_read(reg) {
                Ok(val) => val as u64,
                Err(_) => u64::MAX,
            }
        },
        _ => {
            crate::kinfo!("Unknown syscall: {}", nr);
            u64::MAX
        }
    };
    tf.rax = ret;
}
