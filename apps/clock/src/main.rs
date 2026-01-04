#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    thing_std::init(0);
    log_info("CLOCK: Starting...");

    // 1. Create Surface Bytespace
    let width = 320;
    let height = 200;
    let size = width * height * 4;
    let bs_id = bytespace_create(size as u64); // Infallible in current thing_std
    log_info("CLOCK: Bytespace created");

    // 2. Map it
    let vaddr = 0x4000_0000;
    let _ = space_map(bs_id, vaddr, 0, size as u64);
    let buffer = vaddr as *mut u32;

    // 3. Create Graph Things
    let kind_surface = symbol_intern("kind.surface");
    let kind_window = symbol_intern("kind.window");
    let pred_backs = symbol_intern("predicate.backs");
    let pred_targets = symbol_intern("predicate.targets");
    // let pred_primary = symbol_intern("predicate.primary");

    // Create Surface
    let surf = thing_create_under(kind_surface, get_root_place());
    thing_register_name(surf, "surface.clock");

    // Link Backs
    relationship_create(surf, bs_id, pred_backs);

    // Create Window
    let win = thing_create_under(kind_window, get_root_place());
    thing_register_name(win, "window.clock");

    // Link Targets
    relationship_create(win, surf, pred_targets);

    // Add to place.windows?
    if let Some(place_wins) = thing_find("place.windows") {
        let pred_contains = symbol_intern("predicate.contains");
        relationship_create(place_wins, win, pred_contains);
    }

    log_info("CLOCK: Registered window.clock and surface.clock");

    // Drawing Loop
    let mut last_log = 0;
    loop {
        let mono = monotonic_now();
        let system = system_now();

        if mono - last_log > 1_000_000_000 {
            // Log every second
            let mut msg = alloc::string::String::from("CLOCK: Mono=");
            msg.push_str(&u64_to_str(mono));
            msg.push_str(" System=");
            msg.push_str(&i64_to_str(system));
            log_info(&msg);
            last_log = mono;
        }

        // yield
        sleep_ms(100);
    }
}

fn u64_to_str(mut n: u64) -> alloc::string::String {
    if n == 0 {
        return alloc::string::String::from("0");
    }
    let mut s = alloc::string::String::new();
    while n > 0 {
        s.push((b'0' + (n % 10) as u8) as char);
        n /= 10;
    }
    s.chars().rev().collect()
}

fn i64_to_str(n: i64) -> alloc::string::String {
    if n < 0 {
        let mut s = alloc::string::String::from("-");
        s.push_str(&u64_to_str(-n as u64));
        s
    } else {
        u64_to_str(n as u64)
    }
}
