#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::SystemClock;

#[thing_os::main]
fn main() {
    print_str("debug_clock: starting...\n");

    // Try hardcoded ID 16 (seen in logs)
    let id = thing_os::ThingId(16);
    if let Some(ts) = thing_os::load_thing::<thing_os::TimeSource>(id) {
        print_str("Loaded TimeSource 16! ticks=");
        print_num(ts.ticks_since_boot);
        print_str("\n");
        // Create clock manually if discover fails
        // But for now just sleep loop to prove we persist
        // We can't easily construct SystemClock from TimeSource without private fields or helpers
        // So just loop printing raw ticks
        
        loop {
             if let Some(ts_loop) = thing_os::load_thing::<thing_os::TimeSource>(id) {
                 print_str("Ticks: ");
                 print_num(ts_loop.ticks_since_boot);
                 print_str("\n");
             }
             sleep_ms(1000);
        }
    } else {
        print_str("Failed to load TimeSource 16\n");
    }

    /*
    let clock = SystemClock::discover().expect("SystemClock not found");
    let tick_hz = clock.tick_hz().max(1);
    let start_ticks = clock.uptime_ticks();
    let base_seconds = clock.now().0;

    let mut last_logged = None;

    loop {
        let ticks = clock.uptime_ticks();
        let elapsed_secs = ticks.saturating_sub(start_ticks) / tick_hz as u64;
        
        if last_logged.map(|prev| prev != elapsed_secs).unwrap_or(true) {
            let (hour, minute, second) = seconds_to_hms(base_seconds + elapsed_secs as i64);
            
            print_str("debug_clock: Time: ");
            print_num(hour as u64);
            print_str(":");
            print_num(minute as u64);
            print_str(":");
            print_num(second as u64);
            print_str("\n");

            last_logged = Some(elapsed_secs);
        }

        sleep_ms(250);
    }
    */
}

fn seconds_to_hms(seconds: i64) -> (u32, u32, u32) {
    let secs = seconds.rem_euclid(86_400);
    let hour = (secs / 3_600) as u32;
    let minute = ((secs % 3_600) / 60) as u32;
    let second = (secs % 60) as u32;
    (hour, minute, second)
}

fn print_str(s: &str) {
    let ptr = s.as_ptr() as u64;
    let len = s.len() as u64;
    unsafe {
        raw_syscall(thing_os::abi::syscalls::SYSCALL_LOG, ptr, len, 0, 0, 0, 0);
    }
}

fn print_num(mut n: u64) {
    if n == 0 {
        print_str("00");
        return;
    }
    
    // Buffer for u64 (max 20 digits). 
    // We want aligned output for time (00-59), so let's handle padding manually for now or just print raw.
    
    let mut buffer = [0u8; 20];
    let mut i = 20;
    
    // If n < 10, pad with 0
    if n < 10 {
       print_str("0");
    }

    while n > 0 {
        i -= 1;
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
    }
    
    let s = unsafe { core::str::from_utf8_unchecked(&buffer[i..]) };
    print_str(s);
}

#[inline(always)]
unsafe fn raw_syscall(
    num: u64,
    arg0: u64,
    arg1: u64,
    arg2: u64,
    arg3: u64,
    arg4: u64,
    arg5: u64,
) -> u64 {
    let ret;
    core::arch::asm!(
        "syscall",
        inlateout("rax") num => ret,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        in("r10") arg3,
        in("r8") arg4,
        in("r9") arg5,
        clobber_abi("C"),
        options(nostack)
    );
    ret
}
