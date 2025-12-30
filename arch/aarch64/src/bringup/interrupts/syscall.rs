pub static mut SYSCALL_HOOK: Option<fn(usize, usize, usize, usize, usize, usize, usize) -> isize> =
    None;

pub fn set_syscall_hook(hook: fn(usize, usize, usize, usize, usize, usize, usize) -> isize) {
    unsafe {
        SYSCALL_HOOK = Some(hook);
    }
}
