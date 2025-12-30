pub fn rtc_read(out: &mut abi::wire::time::RtcSample) {
    // QEMU Virt PL031
    let pl031_base = 0x0901_0000 as *const u32;
    let mut t = unsafe { core::ptr::read_volatile(pl031_base) } as u64;

    let mut year = 1970;
    let mut days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days = if is_leap { 366 } else { 365 };
        let sec_year = days * 86_400;
        if t < sec_year {
            break;
        }
        t -= sec_year;
        year += 1;
    }

    out.year = year as u16;
    let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    if is_leap {
        days_in_month[1] = 29;
    }

    let mut mon = 0;
    loop {
        let sec_mon = days_in_month[mon] * 86_400;
        if t < sec_mon {
            break;
        }
        t -= sec_mon;
        mon += 1;
    }
    out.mon = (mon + 1) as u8;

    let days = t / 86_400;
    t %= 86_400;
    out.day = (days + 1) as u8;

    out.hour = (t / 3_600) as u8;
    t %= 3_600;
    out.min = (t / 60) as u8;
    out.sec = (t % 60) as u8;
}
