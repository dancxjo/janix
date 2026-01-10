#![no_std]

mod panic;

#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64"
)))]
compile_error!("Stem supports x86_64, aarch64, riscv64, and loongarch64");

#[cfg(target_arch = "x86_64")]
mod arch {
    use abi::syscall;

    #[no_mangle]
    #[unsafe(naked)]
    pub unsafe extern "C" fn _start() -> ! {
        core::arch::naked_asm!(
            "call __standard_init",
            "call main",
            "mov rdi, rax",
            "mov rax, {syscall_exit}",
            "syscall",
            syscall_exit = const syscall::SYSCALL_EXIT,
        )
    }

    #[inline(always)]
    pub unsafe fn syscall0(n: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall",
            in("rax") n,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: u64, a1: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: u64, a1: u64, a2: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            in("rsi") a2,
            lateout("rax") ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[cfg(target_arch = "aarch64")]
mod arch {
    use abi::syscall;

    #[no_mangle]
    #[unsafe(naked)]
    pub unsafe extern "C" fn _start() -> ! {
        core::arch::naked_asm!(
            "bl __standard_init",
            "bl main",
            "mov x0, x0",
            "mov x8, {syscall_exit}",
            "svc #0",
            syscall_exit = const syscall::SYSCALL_EXIT,
        )
    }

    #[inline(always)]
    pub unsafe fn syscall0(n: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            lateout("x0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: u64, a1: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            lateout("x0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: u64, a1: u64, a2: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "svc #0",
            in("x8") n,
            in("x0") a1,
            in("x1") a2,
            lateout("x0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[cfg(target_arch = "riscv64")]
mod arch {
    use abi::syscall;

    #[no_mangle]
    #[unsafe(naked)]
    pub unsafe extern "C" fn _start() -> ! {
        core::arch::naked_asm!(
            "call __standard_init",
            "call main",
            "mv a0, a0",
            "li a7, {syscall_exit}",
            "ecall",
            syscall_exit = const syscall::SYSCALL_EXIT,
        )
    }

    #[inline(always)]
    pub unsafe fn syscall0(n: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "ecall",
            in("a7") n,
            lateout("a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: u64, a1: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "ecall",
            in("a7") n,
            in("a0") a1,
            lateout("a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: u64, a1: u64, a2: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "ecall",
            in("a7") n,
            in("a0") a1,
            in("a1") a2,
            lateout("a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }
}

#[cfg(target_arch = "loongarch64")]
mod arch {
    use abi::syscall;

    #[no_mangle]
    #[unsafe(naked)]
    pub unsafe extern "C" fn _start() -> ! {
        core::arch::naked_asm!(
            "bl __standard_init",
            "bl main",
            "move $a0, $a0",
            "li.d $a7, {syscall_exit}",
            "syscall 0",
            syscall_exit = const syscall::SYSCALL_EXIT,
        )
    }

    #[inline(always)]
    pub unsafe fn syscall0(n: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall 0",
            in("$a7") n,
            lateout("$a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: u64, a1: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall 0",
            in("$a7") n,
            in("$a0") a1,
            lateout("$a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall2(n: u64, a1: u64, a2: u64) -> u64 {
        let ret: u64;
        core::arch::asm!(
            "syscall 0",
            in("$a7") n,
            in("$a0") a1,
            in("$a1") a2,
            lateout("$a0") ret,
            options(nostack, preserves_flags)
        );
        ret
    }
}

use arch::*;

extern "C" {
    fn main() -> i32;
    fn __standard_init();
}

// Syscall wrappers

#[inline(always)]
pub unsafe fn sys_exit(code: i32) -> ! {
    syscall1(abi::syscall::SYSCALL_EXIT, code as u64);
    loop {}
}

#[inline(always)]
pub unsafe fn sys_debug_putchar(c: u8) {
    syscall1(abi::syscall::SYSCALL_PUTCHAR, c as u64);
}

#[inline(always)]
pub unsafe fn sys_yield() {
    syscall0(abi::syscall::SYSCALL_YIELD);
}

#[inline(always)]
pub unsafe fn sys_ticks() -> u64 {
    syscall0(abi::syscall::SYSCALL_TICKS)
}

#[inline(always)]
pub unsafe fn sys_spawn_module(path: &str) -> i64 {
    syscall2(
        abi::syscall::SYSCALL_SPAWN_MODULE,
        path.as_ptr() as u64,
        path.len() as u64
    ) as i64
}

#[inline(always)]
pub unsafe fn sys_rtc_cmos_read(reg: u8) -> i64 {
    syscall1(abi::syscall::SYSCALL_RTC_CMOS_READ, reg as u64) as i64
}

// Root syscalls
#[inline(always)]
pub unsafe fn sys_graph_append(op_ptr: *const abi::root::JournalOp) -> u64 {
    syscall1(abi::syscall::SYSCALL_GRAPH_APPEND, op_ptr as u64)
}

#[inline(always)]
pub unsafe fn sys_watch_create() -> u64 {
    syscall0(abi::syscall::SYSCALL_WATCH_CREATE)
}

#[inline(always)]
pub unsafe fn sys_watch_next(id: u64, out_ptr: *mut abi::root::WatchEvent) -> i64 {
    syscall2(abi::syscall::SYSCALL_WATCH_NEXT, id, out_ptr as u64) as i64
}
