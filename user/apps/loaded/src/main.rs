#![no_std]
#![no_main]

extern crate alloc;
use thing_std as std;
use thing_std::{StdoutConsole, Console, GraphClient};
use core::fmt::Write;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::{TypedBytes, CodecId, TypeId};
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_BOOT_STATE_KIND, THING_BOOT_STATE_SCHEMA
};
use thing_models::schema::boot_state::BootStateBody;

// Local syscall wrapper
pub unsafe fn sys_spawn(path: &str) -> Result<u64, isize> {
    use abi::SYSCALL_SPAWN;
    let ret: isize;
    core::arch::asm!(
        "syscall",
        in("rax") SYSCALL_SPAWN,
        in("rdi") path.as_ptr() as usize,
        in("rsi") path.len(),
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
        options(nostack, preserves_flags)
    );
    if ret >= 0 {
        Ok(ret as u64)
    } else {
        Err(ret)
    }
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();

    let mut c = StdoutConsole;
    let _ = c.write_str("LOADED: Starting Init Sequence...\n");

    let g = GraphClient::new();
    let mut buf = [0u8; 4096];

    // 0. Create BootState Thing
    let mut boot_state_id = None;
    
    let initial_state = BootStateBody {
        phase: alloc::string::String::from("init"),
        step: 0,
        message: alloc::string::String::from("Booting..."),
        level: 0,
    };
    
    if let Ok(bytes) = postcard::to_allocvec(&initial_state) {
        let op = GraphOp::CreateThing {
            kind: THING_BOOT_STATE_KIND,
            value: TypedBytes {
                type_id: TypeId(THING_BOOT_STATE_SCHEMA.0 as u128),
                codec_id: CodecId::POSTCARD,
                bytes,
            },
        };
        if let Ok(GraphReply::Created { id }) = g.call_op(&op, &mut buf) {
            boot_state_id = Some(id);
            // Link to Root
            let link_op = GraphOp::AddLink {
                from: THING_BOOT_ROOT,
                to: id,
                kind: THING_BOOT_STATE_KIND 
            };
            let _ = g.call_op(&link_op, &mut buf);
        }
    }
    
    // Helper to publish state
    // We capture 'c' by value? No by ref or recreate it. Console is ZST usually.
    let publish_milestone = |step: u32, phase: &str, msg: &str| {
        // Serial Log
        let mut console = StdoutConsole; 
        // Use alloc::format to avoid write_fmt trait issues if direct impl is missing or weird
        let log_msg = alloc::format!("LOADED [{}]: {}\n", phase, msg);
        let _ = console.write_str(&log_msg); 
        
        if let Some(bsid) = boot_state_id {
            let body = BootStateBody {
                phase: alloc::string::String::from(phase),
                step,
                message: alloc::string::String::from(msg),
                level: 0,
            };
            if let Ok(bytes) = postcard::to_allocvec(&body) {
                let update_op = GraphOp::UpdateThing { 
                    id: bsid, 
                    value: TypedBytes {
                        type_id: TypeId(THING_BOOT_STATE_SCHEMA.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes
                    }
                };
                let mut tmp_buf = [0u8; 512];
                let g_local = GraphClient::new();
                let _ = g_local.call_op(&update_op, &mut tmp_buf);
            }
        }
    };

    publish_milestone(1, "init", "Loaded Started");
    publish_milestone(2, "init", "Modules Enumerated");

    // Phase B: Start Essentials
    let essentials = [
        ("drivers/ps2_keyboard.elf", "Keyboard Driver"),
        ("drivers/ps2_mouse.elf", "Mouse Driver"),
        ("drivers/rtc_x86.elf", "RTC Driver"),
        ("apps/input_service.elf", "Input Service"),
        ("apps/compositor.elf", "Compositor"),
    ];

    let mut step_count = 3;

    for (path, name) in essentials {
        let msg = alloc::format!("LOADED: Spawning {} ({})\n", name, path);
        let _ = c.write_str(&msg);
        
        let full_path = alloc::format!("boot():/boot/{}", path);
        
        match unsafe { sys_spawn(&full_path) } {
            Ok(_) => {
                publish_milestone(step_count, "essentials", alloc::format!("Started {}", name).as_str());
            }
            Err(e) => {
                 let err_msg = alloc::format!("LOADED: Failed to spawn {}: {:?}\n", name, e);
                 let _ = c.write_str(&err_msg);
                 publish_milestone(step_count, "error", alloc::format!("Failed {}", name).as_str());
            }
        }
        step_count += 1;
        
        for _ in 0..100000 { unsafe { core::arch::asm!("nop"); } }
    }
    
    // Phase C: Mount /boot (Simulated)
    publish_milestone(step_count, "mount", "/boot mounted (simulated)");
    step_count += 1;

    // Phase D: Bulk Loading
    let extras = [
        ("apps/clock.elf", "ClockWidget"),
        ("apps/fb_smoke.elf", "SmokeTest"),
    ];

    for (path, name) in extras {
        let full_path = alloc::format!("boot():/boot/{}", path);
        let _ = unsafe { sys_spawn(&full_path) };
        publish_milestone(step_count, "bulk", alloc::format!("Spawned {}", name).as_str());
        step_count += 1;
    }

    // Phase E: Handoff
    publish_milestone(step_count, "handoff", "Desktop Ready");
    
    let _ = c.write_str("LOADED: Entering Supervisor Loop.\n");
    
    loop {
        for _ in 0..10000000 { unsafe { core::arch::asm!("nop"); } }
    }
}
