#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use thing_std as std;
use thing_std::{Console, GraphClient, StdoutConsole};
use thing_std::sys::{sys_spawn_image, sys_yield};
use thing_std::time;

use abi::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::{CodecId, TypeId, TypedBytes};
use thing_models::builtins::core_kinds::{BootProgramBody, ModuleBody, DisplayFramebufferBody};
use thing_models::boot_state::BootStateBody; // Fixed import
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_BOOT_STATE_KIND, THING_BOOT_STATE_SCHEMA, THING_HAS_MODULE_KIND,
    THING_HAS_DEVICE_KIND, THING_DISPLAY_FRAMEBUFFER_KIND, THING_DISPLAY_FRAMEBUFFER_SCHEMA
};

const LEVEL_INFO: u8 = 0;
const LEVEL_WARN: u8 = 1;
const LEVEL_ERROR: u8 = 2;

struct SproutCtx {
    g: GraphClient,
    console: StdoutConsole,
    boot_state_id: Option<ThingId>,
    step: u32,
    quiet: bool,
}

#[derive(Clone)]
struct ModuleInfo {
    id: ThingId,
    path: String,
    size: u64,
    role: String,
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    // Raw debug - syscall log (Early)
    let msg = "SPROUT: RAW START\n";
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            in("rax") 10, // SYSCALL_LOG
            in("rdi") msg.as_ptr() as usize,
            in("rsi") msg.len(),
            out("rcx") _,
            out("r11") _,
        );
        #[cfg(target_arch = "aarch64")]
        core::arch::asm!(
            "svc #0",
            in("x8") 10, // SYSCALL_LOG
            in("x0") msg.as_ptr() as usize,
            in("x1") msg.len(),
            lateout("x0") _,
            options(nostack)
        );
    }
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();

    let console = StdoutConsole;
    let g = GraphClient::new();
    let mut ctx = SproutCtx {
        g,
        console,
        boot_state_id: None,
        step: 0,
        quiet: false,
    };

    ctx.log("SPROUT: RAW START\n");

    // return; // DEBUG: Stop here to see if it survives.
    
    // 2. Ensure BootState
    /*
    println!("SPROUT: Ensuring BootState...");
    // let mut bs_id = thing_models::builtins::ids::THING_BOOT_STATE_KIND;
    // ensure_boot_state(&mut g, &mut bs_id);
    // println!("SPROUT: BootState OK. ID {:?}", bs_id);
    */
    // ctx.log("SPROUT: start\n");
    // ensure_boot_state(&mut ctx);
    // ctx.publish_state(LEVEL_INFO, "entry", "sprout starting", true);

    let mut buf = alloc::vec![0u8; 16 * 1024];
    let scratch = buf.as_mut_slice();

    // 1. Enumerate Modules
    /*
    println!("SPROUT: Enumerating modules...");
    let modules = enumerate_modules(&mut g);
    println!("SPROUT [modules]: modules enumerated ({})", modules.len());
    */
    let modules = enumerate_modules(&ctx.g, scratch);
    if modules.is_empty() {
        ctx.publish_state(LEVEL_ERROR, "modules", "no modules found (graph query returned 0)", true);
        // Dump debug info?
    } else {
        let msg = format!("modules enumerated ({})", modules.len());
        ctx.publish_state(LEVEL_INFO, "modules", &msg, true);
        for m in &modules {
            let log = format!("module {} role={} size={} path={}\n", m.path, m.role, m.size, m.path);
            ctx.log(&log);
        }
    }

    // 2. Start Drivers
    let drivers: Vec<&ModuleInfo> = modules.iter().filter(|m| m.role == "driver").collect();
    if drivers.is_empty() {
        ctx.publish_state(LEVEL_WARN, "drivers", "no drivers found", true);
    } else {
        for drv in drivers {
            spawn_module(&mut ctx, scratch, drv, "driver");
        }
    }

    // 3. Wait for Machine Interfaces (Framebuffer)
    ctx.publish_state(LEVEL_INFO, "hardware", "waiting for framebuffer...", true);
    let fb_ready = wait_ready(&mut ctx, scratch, 2000, ready_framebuffer);
    if fb_ready {
         ctx.publish_state(LEVEL_INFO, "hardware", "framebuffer available", true);
    } else {
         ctx.publish_state(LEVEL_WARN, "hardware", "framebuffer missing (timeout)", true);
    }

    // 4. Spawn Compositor
    if let Some(comp) = pick_module(&modules, "compositor.elf") {
        spawn_module(&mut ctx, scratch, &comp, "compositor");
    } else {
        ctx.publish_state(LEVEL_WARN, "compositor", "compositor.elf not found", true);
    }

    // 5. Spawn Clock
    // Only if compositor is likely running, or just spawn it anyway and let it wait?
    // We'll spawn it.
    if let Some(clk) = pick_module(&modules, "clock.elf") {
        spawn_module(&mut ctx, scratch, &clk, "clock");
    }

    ctx.publish_state(LEVEL_INFO, "handoff", "init sequence complete", true);
    
    // Supervisor Loop
    loop {
        ctx.wait_ms(1000);
    }
}

// --- Helpers ---

fn ensure_boot_state(ctx: &mut SproutCtx) {
    if ctx.boot_state_id.is_some() {
        return;
    }
    let body = BootStateBody {
        phase: String::from("entry"),
        step: 0,
        message: String::from("sprout"),
        timestamp_ns: timestamp_ns(&ctx.g),
        level: LEVEL_INFO,
    };

    let mut buf = [0u8; 1024];
    let op = GraphOp::CreateThing {
        kind: THING_BOOT_STATE_KIND,
        value: TypedBytes {
            type_id: TypeId(THING_BOOT_STATE_SCHEMA.0 as u128),
            codec_id: CodecId::POSTCARD,
            bytes: postcard::to_allocvec(&body).unwrap_or_default(),
        },
    };

    if let Ok(GraphReply::Created { id }) = ctx.g.call_op(&op, &mut buf) {
        ctx.boot_state_id = Some(id);
        let link = GraphOp::AddLink {
            from: THING_BOOT_ROOT,
            to: id,
            kind: THING_BOOT_STATE_KIND,
        };
        let _ = ctx.g.call_op(&link, &mut buf);
    }
}

fn enumerate_modules(g: &GraphClient, buf: &mut [u8]) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(THING_HAS_MODULE_KIND),
    };

    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        for (_, target, _) in list {
            if let Some(info) = fetch_module(g, target, buf) {
                modules.push(info);
            }
        }
    }
    modules
}

fn fetch_module(g: &GraphClient, id: ThingId, buf: &mut [u8]) -> Option<ModuleInfo> {
    if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&GraphOp::GetThing { id }, buf) {
        if let Ok(body) = postcard::from_bytes::<ModuleBody>(&tb.bytes) {
            return Some(ModuleInfo {
                id,
                path: body.path,
                size: body.size_bytes,
                role: body.role,
            });
        }
    }
    None
}

fn pick_module(mods: &[ModuleInfo], target: &str) -> Option<ModuleInfo> {
    for m in mods {
        if m.path.ends_with(target) || m.path == target {
            return Some(m.clone());
        }
    }
    None
}

fn spawn_module(ctx: &mut SproutCtx, buf: &mut [u8], m: &ModuleInfo, phase: &str) {
    if let Some(bytes) = read_module_bytes(&ctx.g, m, buf) {
        match sys_spawn_image(&bytes, &m.path) {
            Ok(_) => {
                let msg = format!("started {}", m.path);
                ctx.publish_state(LEVEL_INFO, phase, &msg, true);
            }
            Err(_) => {
                 let msg = format!("failed to spawn {}", m.path);
                 ctx.publish_state(LEVEL_ERROR, phase, &msg, true);
            }
        }
    } else {
        let msg = format!("failed to read bytes for {}", m.path);
        ctx.publish_state(LEVEL_ERROR, phase, &msg, true);
    }
}

fn read_module_bytes(g: &GraphClient, m: &ModuleInfo, buf: &mut [u8]) -> Option<Vec<u8>> {
    let len = core::cmp::min(m.size, core::u32::MAX as u64) as u32;
    let op = GraphOp::ReadBytes {
        id: m.id,
        offset: 0,
        len,
    };
    match g.call_op(&op, buf) {
        Ok(GraphReply::Bytes { bytes }) => Some(bytes),
        _ => None,
    }
}

fn wait_ready(
    ctx: &mut SproutCtx,
    buf: &mut [u8],
    timeout_ms: u64,
    check: fn(&GraphClient, &mut [u8]) -> bool,
) -> bool {
    let mut waited = 0;
    loop {
        if check(&ctx.g, buf) {
            return true;
        }
        if waited >= timeout_ms {
            return false;
        }
        ctx.wait_ms(100);
        waited = waited.saturating_add(100);
    }
}

fn ready_framebuffer(g: &GraphClient, buf: &mut [u8]) -> bool {
    // Check for THING_DISPLAY_FRAMEBUFFER_KIND connected to Root via HAS_DEVICE?
    // Or just any DisplayFramebuffer?
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(THING_HAS_DEVICE_KIND),
    };
     if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
         for (_, target, _) in list {
             let get_op = GraphOp::GetThing { id: target };
             if let Ok(GraphReply::TypedValue(tb)) = g.call_op(&get_op, buf) {
                 // Check Schema ID or Kind ID?
                 // Kernel uses THING_DISPLAY_FRAMEBUFFER_KIND for creation.
                 if tb.type_id.0 == THING_DISPLAY_FRAMEBUFFER_KIND.0 as u128 {
                     return true;
                 }
                 // Check Schema too just in case
                 if tb.type_id.0 == THING_DISPLAY_FRAMEBUFFER_SCHEMA.0 as u128 {
                     return true;
                 }
             }
         }
    }
    false
}

fn timestamp_ns(g: &GraphClient) -> u64 {
    time::monotonic_ns(g).unwrap_or(0)
}

impl SproutCtx {
    fn log(&self, msg: &str) {
        if !self.quiet {
            let _ = self.console.write_str(msg);
        }
    }

    fn wait_ms(&self, ms: u64) {
        if time::sleep_ms(&self.g, ms).is_err() {
            for _ in 0..ms {
                sys_yield();
                for _ in 0..1000 {
                    core::hint::spin_loop();
                }
            }
        }
    }

    fn publish_state(&mut self, level: u8, phase: &str, message: &str, advance: bool) {
        if advance {
            self.step = self.step.saturating_add(1);
        }
        let ts = timestamp_ns(&self.g);
        let log_line = format!("SPROUT [{}]: {}\n", phase, message);
        self.log(&log_line);

        if self.boot_state_id.is_none() {
            ensure_boot_state(self);
        }
        if let Some(id) = self.boot_state_id {
            let body = BootStateBody {
                phase: String::from(phase),
                step: self.step,
                message: String::from(message),
                timestamp_ns: ts,
                level,
            };
            let mut buf = [0u8; 1024];
            let _ = self.g.call_op(
                &GraphOp::UpdateThing {
                    id,
                    value: TypedBytes {
                        type_id: TypeId(THING_BOOT_STATE_SCHEMA.0 as u128),
                        codec_id: CodecId::POSTCARD,
                        bytes: postcard::to_allocvec(&body).unwrap_or_default(),
                    },
                },
                &mut buf,
            );
        }
    }
}
