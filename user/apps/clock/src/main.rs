#![no_std]
#![no_main]

extern crate alloc;

use thing_std::{GraphClient, StdoutConsole, Console};

#[derive(serde::Serialize)]
struct TimeReq {}

#[derive(serde::Deserialize)]
struct TimeResp {
    system_ns: u64,
    #[allow(dead_code)]
    monotonic_ns: u64,
}

fn format_hms(system_ns: u64) -> (u64, u64, u64) {
    // seconds since epoch-ish
    let total = system_ns / 1_000_000_000;
    let s = total % 60;
    let m = (total / 60) % 60;
    let h = (total / 3600) % 24;
    (h, m, s)
}

#[no_mangle]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") 0x3F8u16,
            in("al") 0x41u8, // 'A'
            options(nomem, nostack, preserves_flags)
        );
    }
    */
    let g = GraphClient::new();
    let c = StdoutConsole;
    c.write_str("CLOCK: Starting...\n");
    let mut out = [0u8; 256];

    loop {
        match g.call::<TimeReq, TimeResp>("time.now", &TimeReq {}, &mut out) {
            Ok(t) => {
                let (_h, _m, _s) = format_hms(t.system_ns);
                // v0 console: simple print. Later: ANSI/console control.
                // Print as one line so the framebuffer console can show it cleanly.
                // If you have a clear-line primitive, use it later.
                // For now, spam once/sec.
                // (We’ll replace with proper console drawing once the compositor returns.)
                // format without alloc:
                // (If alloc allowed, you can build a String; but keep v0 simple.)
                c.write_str("clock: ");
                // TODO: replace with integer formatting helper in thing_std::console
                // For now, just print raw seconds:
                c.write_str("tick\n");
            }
            Err(_) => {
                c.write_str("clock: time.now failed\n");
            }
        }

        // v0 “sleep”: placeholder. Replace when sleep syscall exists.
        // Crude spin to avoid exploding logs too fast.
        for _ in 0..5_000_000 { core::hint::spin_loop(); }
        for _ in 0..5_000_000 { core::hint::spin_loop(); }
    }
}
