//! kill — send a signal to a process.
//!
//! Usage: kill [-<SIG>] <pid> [<pid>...]
//!
//! Examples:
//!   kill 42           — send SIGTERM to process 42
//!   kill -9 42        — send SIGKILL to process 42
//!   kill -TERM 42     — send SIGTERM to process 42
//!   kill -USR1 42     — send SIGUSR1 to process 42
#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use stem::syscall::signal::kill;
use stem::syscall::vfs_write;

fn write_str(s: &str) {
    let _ = vfs_write(2, s.as_bytes()); // stderr (fd 2)
}

fn write_out(s: &str) {
    let _ = vfs_write(1, s.as_bytes()); // stdout (fd 1)
}

/// Parse a signal name or number string into a u32 signal number.
fn parse_sig(s: &str) -> Option<u32> {
    use abi::signal::*;
    // Try numeric first.
    if let Ok(n) = s.parse::<u32>() {
        return Some(n);
    }
    // Try symbolic names (with or without SIG prefix).
    let upper = s.to_uppercase();
    let name = upper.strip_prefix("SIG").unwrap_or(&upper);
    let sig = match name {
        "HUP" => SIGHUP,
        "INT" => SIGINT,
        "QUIT" => SIGQUIT,
        "ILL" => SIGILL,
        "TRAP" => SIGTRAP,
        "ABRT" | "IOT" => SIGABRT,
        "BUS" => SIGBUS,
        "FPE" => SIGFPE,
        "KILL" => SIGKILL,
        "USR1" => SIGUSR1,
        "SEGV" => SIGSEGV,
        "USR2" => SIGUSR2,
        "PIPE" => SIGPIPE,
        "ALRM" => SIGALRM,
        "TERM" => SIGTERM,
        "CHLD" | "CLD" => SIGCHLD,
        "CONT" => SIGCONT,
        "STOP" => SIGSTOP,
        "TSTP" => SIGTSTP,
        "TTIN" => SIGTTIN,
        "TTOU" => SIGTTOU,
        "WINCH" => SIGWINCH,
        _ => return None,
    };
    Some(sig)
}

fn get_args() -> Vec<String> {
    let mut len = 0;
    if let Ok(l) = stem::syscall::argv_get(&mut []) {
        len = l;
    }
    if len == 0 {
        return Vec::new();
    }
    let mut buf = alloc::vec![0u8; len];
    if stem::syscall::argv_get(&mut buf).is_err() {
        return Vec::new();
    }
    let mut args = Vec::new();
    if buf.len() >= 4 {
        let count = u32::from_le_bytes(buf[0..4].try_into().unwrap()) as usize;
        let mut offset = 4;
        for _ in 0..count {
            if offset + 4 > buf.len() {
                break;
            }
            let len = u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            if offset + len > buf.len() {
                break;
            }
            let s = alloc::str::from_utf8(&buf[offset..offset + len])
                .unwrap_or("")
                .to_string();
            args.push(s);
            offset += len;
        }
    }
    args
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let args = get_args();
    // args[0] is the program name.
    let mut sig: u32 = abi::signal::SIGTERM;
    let mut pids: Vec<i32> = Vec::new();
    let mut i = 1;

    while i < args.len() {
        let arg = &args[i];
        if let Some(sigspec) = arg.strip_prefix('-') {
            // Signal specification.
            match parse_sig(sigspec) {
                Some(s) => sig = s,
                None => {
                    write_str("kill: unknown signal: ");
                    write_str(sigspec);
                    write_str("\n");
                    stem::syscall::exit(-1);
                }
            }
        } else {
            // PID argument.
            match arg.parse::<i32>() {
                Ok(pid) => pids.push(pid),
                Err(_) => {
                    write_str("kill: invalid pid: ");
                    write_str(arg);
                    write_str("\n");
                    stem::syscall::exit(-1);
                }
            }
        }
        i += 1;
    }

    if pids.is_empty() {
        write_str("Usage: kill [-<SIG>] <pid> [<pid>...]\n");
        stem::syscall::exit(1);
    }

    let mut exit_code = 0i32;
    for pid in pids {
        match kill(pid, sig) {
            Ok(()) => {}
            Err(e) => {
                write_str("kill: ");
                write_str(itoa(pid as i64).as_str());
                write_str(": ");
                let _ = e; write_str("error");
                write_str("\n");
                exit_code = 1;
            }
        }
    }

    let _ = exit_code;
    stem::syscall::exit(exit_code);
}

/// Minimal integer-to-decimal string conversion (no_std).
fn itoa(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let neg = n < 0;
    if neg {
        n = -n;
    }
    let mut digits = Vec::new();
    while n > 0 {
        digits.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    if neg {
        digits.push('-');
    }
    digits.iter().rev().collect()
}
