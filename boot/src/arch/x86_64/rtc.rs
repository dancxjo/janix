use kernel_core::time::RealTimeClock;
use x86_64::instructions::port::Port;

struct RawRtcTime {
    second: u8,
    minute: u8,
    hour: u8,
    day: u8,
    month: u8,
    year: u8,
    status_b: u8,
}

pub struct CmosRtc;

impl CmosRtc {
    pub const fn new() -> Self {
        CmosRtc
    }

    fn read_register(&self, reg: u8) -> u8 {
        unsafe {
            let mut addr = Port::<u8>::new(0x70);
            let mut data = Port::<u8>::new(0x71);
            addr.write(reg);
            data.read()
        }
    }

    fn is_update_in_progress(&self) -> bool {
        (self.read_register(0x0A) & 0x80) != 0
    }

    fn read_time_raw(&self) -> RawRtcTime {
        while self.is_update_in_progress() {}

        let second = self.read_register(0x00);
        let minute = self.read_register(0x02);
        let hour = self.read_register(0x04);
        let day = self.read_register(0x07);
        let month = self.read_register(0x08);
        let year = self.read_register(0x09);
        let status_b = self.read_register(0x0B);

        RawRtcTime {
            second,
            minute,
            hour,
            day,
            month,
            year,
            status_b,
        }
    }

    fn bcd_to_binary(bcd: u8) -> u8 {
        (bcd & 0x0F) + ((bcd / 16) * 10)
    }

    fn to_unix_epoch(&self, mut t: RawRtcTime) -> (u64, u32) {
        let is_binary = (t.status_b & 0x04) != 0;
        let is_24h = (t.status_b & 0x02) != 0;

        if !is_binary {
            t.second = Self::bcd_to_binary(t.second);
            t.minute = Self::bcd_to_binary(t.minute);
            t.hour = Self::bcd_to_binary(t.hour); // Handle 12h later if needed? No, BCD conversion first.
            t.day = Self::bcd_to_binary(t.day);
            t.month = Self::bcd_to_binary(t.month);
            t.year = Self::bcd_to_binary(t.year);
        }

        if !is_24h && (t.hour & 0x80) != 0 {
            t.hour = ((t.hour & 0x7F) + 12) % 24;
        }

        // Full year calculation
        // Assuming 21st century for simplicity if century register is missing/unreliable
        // Or just 2000 + year
        let full_year = 2000 + t.year as u64;

        // Simple days since epoch calculation
        // 1970 to full_year
        let mut days = 0;
        for y in 1970..full_year {
            days += if (y % 4 == 0 && y % 100 != 0) || (y % 400 == 0) {
                366
            } else {
                365
            };
        }

        // Months in current year
        let is_leap = (full_year % 4 == 0 && full_year % 100 != 0) || (full_year % 400 == 0);
        let days_in_month = [
            0,
            31,
            if is_leap { 29 } else { 28 },
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];

        for m in 1..t.month as usize {
            days += days_in_month[m];
        }

        days += (t.day - 1) as u64;

        let total_seconds =
            days * 86400 + t.hour as u64 * 3600 + t.minute as u64 * 60 + t.second as u64;

        (total_seconds, 0)
    }
}

impl RealTimeClock for CmosRtc {
    fn init(&self) {
        // No-op
    }

    fn now_utc(&self) -> (u64, u32) {
        let raw = self.read_time_raw();
        self.to_unix_epoch(raw)
    }
}

pub fn init_arch_rtc() {
    static CMOS_RTC: CmosRtc = CmosRtc::new();
    CMOS_RTC.init();
    kernel_core::time::register_rtc(&CMOS_RTC);
}
