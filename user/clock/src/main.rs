#![no_std]
#![no_main]

use thing_os::prelude::*;
use thing_os::sys::raw_syscall;
use thing_os::SystemClock;

#[thing_os::main]
fn main() {
    print_str("clock: starting...\n");

    let clock = match SystemClock::discover() {
        Some(clock) => clock,
        None => {
            print_str("clock: no TimeSource available\n");
            return;
        }
    };

    loop {
        let (unix_seconds, _) = clock.now();
        let (year, month, day, hour, minute, second) = unix_seconds_to_datetime(unix_seconds);

        print_str("clock: ");
        print_num(year as u64);
        print_str("-");
        print_num(month as u64);
        print_str("-");
        print_num(day as u64);
        print_str(" ");
        print_num(hour as u64);
        print_str(":");
        print_num(minute as u64);
        print_str(":");
        print_num(second as u64);
        print_str(" UTC\n");

        sleep_ms(1000);
    }
}

fn unix_seconds_to_datetime(seconds: i64) -> (i32, u32, u32, u32, u32, u32) {
    let mut days = seconds.div_euclid(86_400);
    let mut secs_of_day = seconds.rem_euclid(86_400);
    if secs_of_day < 0 {
        secs_of_day += 86_400;
        days -= 1;
    }

    let mut year = 1970;
    while days >= days_in_year(year) as i64 {
        days -= days_in_year(year) as i64;
        year += 1;
    }

    let mut month = 1;
    while days >= days_in_month(year, month) as i64 {
        days -= days_in_month(year, month) as i64;
        month += 1;
    }

    let day = days as u32 + 1;
    let mut remaining = secs_of_day;
    let hour = (remaining / 3_600) as u32;
    remaining %= 3_600;
    let minute = (remaining / 60) as u32;
    let second = (remaining % 60) as u32;

    (year, month as u32, day, hour, minute, second)
}

fn days_in_year(year: i32) -> i32 {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        366
    } else {
        365
    }
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if days_in_year(year) == 366 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
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
