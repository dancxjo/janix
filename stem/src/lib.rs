#![no_std]

mod panic;


#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // Initialize standard library (console, Heap, etc)
        // This is provided by the standard crate which is linked in by the binary.
        "call __standard_init",
        // Call main (which we expect the bin to provide)
        "call main",
        // Exit with return value
        "mov rdi, rax",
        "mov rax, {syscall_exit}",
        "syscall",
        syscall_exit = const abi::syscall::SYSCALL_EXIT,
    )
}

extern "C" {
    fn main() -> i32;
    fn __standard_init();
}

// Syscall wrappers

#[inline(always)]
pub unsafe fn sys_debug_putchar(c: u8) {
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") c as u64,
        syscall_num = const abi::syscall::SYSCALL_PUTCHAR,
        out("rax") _,
        out("rcx") _,
        out("r11") _,
    );
}

#[inline(always)]
pub unsafe fn sys_yield() {
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        syscall_num = const abi::syscall::SYSCALL_YIELD,
        out("rax") _,
        out("rcx") _,
        out("r11") _,
    );
}

#[inline(always)]
pub unsafe fn sys_ticks() -> u64 {
    let ret: u64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        syscall_num = const abi::syscall::SYSCALL_TICKS,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

// We will add SpawnModule and RtcCmosRead here later when we update ABI
#[inline(always)]
pub unsafe fn sys_spawn_module(path: &str) -> i64 {
    let ret: i64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") path.as_ptr() as u64,
        in("rsi") path.len() as u64,
        syscall_num = const abi::syscall::SYSCALL_SPAWN_MODULE,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn sys_rtc_cmos_read(reg: u8) -> i64 {
    let ret: i64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") reg as u64,
        syscall_num = const abi::syscall::SYSCALL_RTC_CMOS_READ,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

// Root syscalls
#[inline(always)]
pub unsafe fn sys_graph_append(op_ptr: *const abi::root::JournalOp) -> u64 {
    let ret: u64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") op_ptr as u64,
        syscall_num = const abi::syscall::SYSCALL_GRAPH_APPEND,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn sys_watch_create() -> u64 {
    let ret: u64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        syscall_num = const abi::syscall::SYSCALL_WATCH_CREATE,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

#[inline(always)]
pub unsafe fn sys_watch_next(id: u64, out_ptr: *mut abi::root::WatchEvent) -> i64 {
    let ret: i64;
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") id,
        in("rsi") out_ptr as u64,
        syscall_num = const abi::syscall::SYSCALL_WATCH_NEXT,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}
