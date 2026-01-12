use abi::device::RtcTime;
use core::arch::asm;

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let mut val: u8;
    asm!("in al, dx", in("dx") port, out("al") val, options(nomem, nostack, preserves_flags));
    val
}

fn cmos_read(reg: u8) -> u8 {
    unsafe {
        outb(0x70, reg);
        inb(0x71)
    }
}

fn cmos_wait_update_in_progress() {
    unsafe {
        outb(0x70, 0x0A);
        while (inb(0x71) & 0x80) != 0 {}
    }
}

// BCD to binary
fn bcd_to_binary(val: u8) -> u8 {
    (val & 0x0F) + ((val / 16) * 10)
}

pub unsafe fn read_rtc() -> RtcTime {
    cmos_wait_update_in_progress();

    let mut second = cmos_read(0x00);
    let mut minute = cmos_read(0x02);
    let mut hour = cmos_read(0x04);
    let mut day = cmos_read(0x07);
    let mut month = cmos_read(0x08);
    let mut year = cmos_read(0x09);

    let register_b = cmos_read(0x0B);

    // Convert BCD if needed
    if (register_b & 0x04) == 0 {
        second = bcd_to_binary(second);
        minute = bcd_to_binary(minute);
        hour = bcd_to_binary(hour & 0x7F); // Mask out AM/PM bit likely? Or handle 12h mode logic later.
        day = bcd_to_binary(day);
        month = bcd_to_binary(month);
        year = bcd_to_binary(year);
    }

    // Century register? Without ACPI assume 2000+.
    let full_year = 2000 + (year as u16);

    RtcTime {
        year: full_year,
        month,
        day,
        hour,
        minute,
        second,
        weekday: 0,
        flags: 0,
    }
}
