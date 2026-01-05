#![no_std]
#![no_main]

extern crate alloc;
use thing_std::*;
use thing_std::cap::{grant, Cap, CapOp, CapScope};

#[unsafe(no_mangle)]
pub fn main() {
    log_info("SPROUT: I am alive");

    // Spawn essential services
    log_info("SPROUT: spawning services...");
    spawn_and_grant("bloom");
    spawn_and_grant("clock");
    spawn_and_grant("timed"); // Added timed
    spawn_and_grant("inputd");

    // Spawn validation tools
    spawn_and_grant("thingcheck");
    // spawn_and_grant("bouncer_test");

    #[cfg(target_arch = "x86_64")]
    spawn_and_grant("rtc_cmos");

    #[cfg(target_arch = "aarch64")]
    spawn_and_grant("rtc_pl031");

    log_info("SPROUT: boot sequence complete.");
}

fn spawn_and_grant(name: &str) {
    match spawn(name) {
        Ok(id) => {
            // log_info(&alloc::format!("SPROUT: spawned {}, granting caps...", name));
            configure_policy(id, name);
        }
        Err(e) => {
            log_info(&alloc::format!("SPROUT: failed to spawn {}: error {}", name, e));
        }
    }
}

fn configure_policy(id: ThingId, name: &str) {
    // Helper to grant global cap
    let global = |op| {
        let _ = grant(id, Cap { op, scope: CapScope::Global });
    };

    // Everyone gets Logging
    global(CapOp::Log);

    match name {
        "bloom" => {
            // Graphics pipeline needs massive permissions for now
            global(CapOp::MemManage); // Framebuffer mapping
            global(CapOp::GraphCreate); // Surfaces
            global(CapOp::GraphLink);
            global(CapOp::GraphUnlink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite); // Update display body
            global(CapOp::GraphWatch); // Watch input
        }
        "inputd" => {
            // Driver needs hardware and memory
            global(CapOp::MemManage); // Ring buffer bytespace
            global(CapOp::Hardware);  // Port I/O
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphRead);
            global(CapOp::GraphWrite);
        }
        "clock" => {
            // Clock needs memory for heap! (Fixes 0x9000... crash)
            global(CapOp::MemManage);
            global(CapOp::GraphRead);
        }
        "timed" => {
            // Needs heap
            global(CapOp::MemManage);
            // Needs to find rtc
            global(CapOp::GraphRead);
            // Needs to create system.time
            global(CapOp::GraphCreate);
            global(CapOp::GraphLink);
            global(CapOp::GraphWrite);
        }
        "thingcheck" => {
            // Inspector needs read access
            global(CapOp::GraphRead);
        }
        "rtc_cmos" => {
             global(CapOp::Log);
             global(CapOp::MemManage); // Usually needed for heap/buffers
             global(CapOp::GraphRead); // Find hw thing
             global(CapOp::GraphCreate); // Create device
             global(CapOp::GraphLink);
             global(CapOp::GraphWrite);
             global(CapOp::IoPort);      // The important bit!
        }
        "rtc_pl031" => {
             global(CapOp::Log);
             global(CapOp::GraphRead);
             global(CapOp::GraphCreate);
             global(CapOp::GraphLink);
             global(CapOp::GraphWrite);
             global(CapOp::Hardware); // MMIO needs Hardware cap? Or MemManage?
             // MMIO mapping via sys_space_map needs MemManage
             global(CapOp::MemManage);
        }
        _ => {}
    }
}
