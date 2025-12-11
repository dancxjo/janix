use alloc::boxed::Box;

pub fn log_rtc_epoch(seconds: i64) {
    let (year, month, day, hour, minute, second) = unix_seconds_to_datetime(seconds);
    let msg = alloc::format!(
        "RTC epoch (raw): {:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC ({}s)",
        year,
        month,
        day,
        hour,
        minute,
        second,
        seconds
    );
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    kernel::log(leaked);
}

pub fn unix_seconds_to_datetime(seconds: i64) -> (i32, u32, u32, u32, u32, u32) {
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

pub fn days_in_year(year: i32) -> i32 {
    if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
        366
    } else {
        365
    }
}

pub fn days_in_month(year: i32, month: i32) -> i32 {
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
