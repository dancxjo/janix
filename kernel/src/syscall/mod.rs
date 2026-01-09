use crate::trap::x86_64::TrapFrame;
use crate::user::abi::{SYSCALL_PUTCHAR, SYSCALL_TICKS, SYSCALL_YIELD, SYSCALL_EXIT};

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
        _ => {
            crate::kinfo!("Unknown syscall: {}", nr);
            u64::MAX
        }
    };
    tf.rax = ret;
}
