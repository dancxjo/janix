#![no_std]

mod panic;

use standard::console::ConsoleSink;

struct UserConsoleSink;

impl ConsoleSink for UserConsoleSink {
    fn putchar(&self, c: u8) {
        unsafe {
            sys_debug_putchar(c);
        }
    }
}

static CONSOLE: UserConsoleSink = UserConsoleSink;

#[no_mangle]
#[unsafe(naked)]
pub unsafe extern "C" fn _start() -> ! {
    core::arch::naked_asm!(
        // Initialize the console sink
        "call {init_console}",
        // Call main (which we expect the bin to provide)
        "call main",
        // Exit with return value
        "mov rdi, rax",
        "mov rax, {syscall_exit}",
        "syscall",
        init_console = sym init_console,
        syscall_exit = const abi::SYSCALL_EXIT,
    )
}

extern "C" {
    fn main() -> i32;
}

extern "C" fn init_console() {
    unsafe {
        standard::console::set_console_sink(&CONSOLE);
    }
}

// Syscall wrappers

#[inline(always)]
pub unsafe fn sys_debug_putchar(c: u8) {
    core::arch::asm!(
        "mov rax, {syscall_num}",
        "syscall",
        in("rdi") c as u64,
        syscall_num = const abi::SYSCALL_PUTCHAR,
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
        syscall_num = const abi::SYSCALL_YIELD,
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
        syscall_num = const abi::SYSCALL_TICKS,
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
        syscall_num = const abi::SYSCALL_SPAWN_MODULE,
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
        syscall_num = const abi::SYSCALL_RTC_CMOS_READ,
        out("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}
