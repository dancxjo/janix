#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
mod app {
    extern crate alloc;
    use thing_os::prelude::*;
    use examples_support::{log, sleep_ms, UserlandSys};
    use alloc::format;
    use alloc::vec::Vec;

    #[unsafe(no_mangle)]
    pub fn main() {
        let mut sys = examples_support::init();
        if let Err(_) = run(&mut sys) {
            log(&mut sys, "debug_thread", "Error running app");
        }
    }

    fn run(sys: &mut UserlandSys) -> Result<(), ()> {
        log(sys, "debug_thread", "starting");

        let start = sys.time_monotonic_ns();
        let mut tick: u64 = 0;

        loop {
            let now = sys.time_monotonic_ns();
            let elapsed_ms = (now - start) / 1_000_000;

            log(sys, "debug_thread", &format!("=== Dashboard tick {} ({} ms) ===", tick, elapsed_ms));

            let threads: Vec<ThreadThing> = list_things_by_kind::<UserlandSys, ThreadThing>(sys);
            if threads.is_empty() {
                log(sys, "debug_thread", "  (no ThreadInfo Things found)");
            } else {
                for t in threads {
                    log(sys, "debug_thread", &format!(
                        "  tid={:<4} state={:<10} priority={} runtime_ns={}",
                        t.tid, t.state, t.priority, t.runtime_ns,
                    ));
                }
            }

            tick = tick.wrapping_add(1);
            sleep_ms(sys, 1000);
        }
    }
}

#[cfg(not(target_os = "none"))]
fn main() {}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
