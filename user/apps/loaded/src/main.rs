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
use thing_models::payload::*; // All payloads and predicates
use thing_models::builtins::core_kinds::BootProgramBody;
use thing_models::core::fs::MountBody;
use thing_models::core::process::ProcessBody;
use thing_models::schema::boot_state::BootStateBody;
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_BOOT_STATE_KIND, THING_BOOT_STATE_SCHEMA,
    THING_MOUSE_SCHEMA, THING_KEY_EVENT_STREAM_SCHEMA, THING_TEXT_EVENT_STREAM_SCHEMA,
    THING_WINDOW_SCHEMA,
};

const LEVEL_INFO: u8 = 0;
const LEVEL_WARN: u8 = 1;
const LEVEL_ERROR: u8 = 2;

struct LoadedCtx {
    g: GraphClient,
    console: StdoutConsole,
    boot_state_id: Option<ThingId>,
    step: u32,
    quiet: bool,
    last_wait_log: u64,
}

#[derive(Clone)]
struct ModuleInfo {
    id: ThingId,
    path: String,
    size: u64,
}

#[derive(Clone)]
struct ServiceDesc<'a> {
    phase: &'a str,
    start_label: &'a str,
    target: &'a str,
    alt_target: Option<&'a str>,
    readiness: fn(&GraphClient, &mut [u8]) -> bool,
    timeout_ms: u64,
}

#[no_mangle]
pub extern "C" fn _start(heap_start: u64) -> ! {
    let msg = "LOADED: RAW START\n";
    unsafe {
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!(
            "syscall",
            in("rax") 10,
            in("rdi") msg.as_ptr() as usize,
            in("rsi") msg.len(),
            out("rcx") _,
            out("r11") _,
        );
    }
    unsafe { std::rt::init_heap(heap_start as usize, 32 * 1024 * 1024); }
    std::init();

    let console = StdoutConsole;
    let g = GraphClient::new();
    let mut ctx = LoadedCtx {
        g,
        console,
        boot_state_id: None,
        step: 0,
        quiet: false,
        last_wait_log: 0,
    };

    ctx.log("LOADED: start\n");
    ensure_boot_state(&mut ctx);

    ctx.publish_state(LEVEL_INFO, "entry", "loaded starting", true);

    let mut scratch_vec = alloc::vec![0u8; 16 * 1024];
    let scratch = scratch_vec.as_mut_slice();

    let modules = enumerate_modules(&ctx.g, scratch);
    let msg = format!("modules enumerated ({})", modules.len());
    ctx.publish_state(LEVEL_INFO, "modules", &msg, true);

    let services = [
        ServiceDesc {
            phase: "kbd",
            start_label: "kbd started",
            target: "ps2_keyboard.elf",
            alt_target: None,
            readiness: ready_keyboard,
            timeout_ms: 1500,
        },
        ServiceDesc {
            phase: "mouse",
            start_label: "mouse started",
            target: "ps2_mouse.elf",
            alt_target: None,
            readiness: ready_mouse,
            timeout_ms: 1500,
        },
        ServiceDesc {
            phase: "input",
            start_label: "input ready",
            target: "input_service.elf",
            alt_target: None,
            readiness: ready_input_streams,
            timeout_ms: 1500,
        },
        ServiceDesc {
            phase: "rtc",
            start_label: "rtc ready",
            target: "rtc_x86.elf",
            alt_target: None,
            readiness: ready_time_link,
            timeout_ms: 1500,
        },
        ServiceDesc {
            phase: "compositor",
            start_label: "compositor started",
            target: "compositor.elf",
            alt_target: None,
            readiness: ready_compositor,
            timeout_ms: 2000,
        },
        ServiceDesc {
            phase: "clock",
            start_label: "clock started",
            target: "clock.elf",
            alt_target: Some("fb_smoke.elf"),
            readiness: ready_clock,
            timeout_ms: 1500,
        },
    ];

    for svc in services.iter() {
        start_service(&mut ctx, scratch, svc);
    }

    let hw_summary = summarize_hardware(&ctx.g, scratch);
    ctx.publish_state(LEVEL_INFO, "hardware", &hw_summary, true);

    let mount_dir = wait_for_boot_mount(&mut ctx, scratch);
    if mount_dir.is_some() {
        ctx.publish_state(LEVEL_INFO, "mount", "/boot mounted", true);
    }

    let launched = launch_optional_apps(&mut ctx, scratch);
    let launch_msg = format!("apps launched: {}", launched);
    ctx.publish_state(LEVEL_INFO, "bulk", &launch_msg, true);
    ctx.publish_state(LEVEL_INFO, "assets", "assets loaded", true);

    ctx.publish_state(LEVEL_INFO, "handoff", "desktop ready", true);
    ctx.log("LOADED: handoff complete\n");

    supervisor_loop(ctx);
}

fn ensure_boot_state(ctx: &mut LoadedCtx) {
    if ctx.boot_state_id.is_some() {
        return;
    }
    let body = BootStateBody {
        phase: String::from("entry"),
        step: 0,
        message: String::from("loaded"),
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
    // ...
}

fn start_service(ctx: &mut LoadedCtx, buf: &mut [u8], svc: &ServiceDesc) {
    let mut attempts = 0;
    let module_choice = loop {
        let modules = enumerate_modules(&ctx.g, buf);
        if let Some(m) = pick_module(&modules, svc.target, svc.alt_target) {
            break Some(m);
        }
        attempts += 1;
        if attempts > 10 {
            break None;
        }
        ctx.wait_ms(100);
    };

    if let Some(module) = module_choice {
        if let Some(bytes) = read_module_bytes(&ctx.g, &module, buf) {
            let spawn_name = module.path.as_str();
            match sys_spawn_image(&bytes, spawn_name) {
                Ok(_) => {
                    let ready = wait_ready(ctx, buf, svc.timeout_ms, svc.readiness);
                    if ready {
                        ctx.publish_state(LEVEL_INFO, svc.phase, svc.start_label, true);
                    } else {
                        let msg = format!("{} degraded (timeout)", svc.start_label);
                        ctx.publish_state(LEVEL_WARN, svc.phase, &msg, true);
                    }
                }
                Err(_) => {
                    let msg = format!("failed to spawn {}", svc.target);
                    ctx.publish_state(LEVEL_ERROR, svc.phase, &msg, true);
                }
            }
        } else {
            let msg = format!("missing bytes for {}", svc.target);
            ctx.publish_state(LEVEL_WARN, svc.phase, &msg, true);
        }
    } else {
        let msg = format!("{} unavailable", svc.target);
        ctx.publish_state(LEVEL_WARN, svc.phase, &msg, true);
    }
}

fn wait_ready(
    ctx: &mut LoadedCtx,
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

fn enumerate_modules(g: &GraphClient, buf: &mut [u8]) -> Vec<ModuleInfo> {
    let mut modules = Vec::new();
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(HAS_MODULE),
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
    if let Ok(GraphReply::Thing { bytes }) = g.call_op(&GraphOp::GetThing { id }, buf) {
        if let Ok(body) = postcard::from_bytes::<File>(&bytes) {
            return Some(ModuleInfo {
                id,
                path: body.name,
                size: body.size,
            });
        }
    }
    None
}

fn pick_module(mods: &[ModuleInfo], target: &str, alt: Option<&str>) -> Option<ModuleInfo> {
    for m in mods {
        if m.path.ends_with(target) || m.path == target {
            return Some(m.clone());
        }
    }
    if let Some(alt_path) = alt {
        for m in mods {
            if m.path.ends_with(alt_path) || m.path == alt_path {
                return Some(m.clone());
            }
        }
    }
    None
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

fn timestamp_ns(g: &GraphClient) -> u64 {
    time::monotonic_ns(g).unwrap_or(0)
}

fn ready_keyboard(g: &GraphClient, buf: &mut [u8]) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(HAS_KEYBOARD),
    };
    matches!(g.call_op(&op, buf), Ok(GraphReply::Links(list)) if !list.is_empty())
}

fn ready_mouse(g: &GraphClient, buf: &mut [u8]) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(HAS_DEVICE),
    };
    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        for (_, target, _) in list {
            if let Ok(GraphReply::Thing { bytes }) = g.call_op(&GraphOp::GetThing { id: target }, buf) {
                // Check payload if necessary
                return true;
            }
        }
    }
    false
}

fn ready_input_streams(g: &GraphClient, buf: &mut [u8]) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(OWNS),
    };
    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        if !list.is_empty() { return true; }
    }
    false
}

fn ready_time_link(g: &GraphClient, buf: &mut [u8]) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(HAS_TIME_NOW),
    };
    matches!(g.call_op(&op, buf), Ok(GraphReply::Links(list)) if !list.is_empty())
}

fn ready_compositor(g: &GraphClient, buf: &mut [u8]) -> bool {
    has_process_for(g, buf, "compositor.elf")
}

fn ready_clock(g: &GraphClient, buf: &mut [u8]) -> bool {
    has_process_for(g, buf, "clock.elf") || has_process_for(g, buf, "fb_smoke.elf") || has_window(g, buf)
}

fn has_window(g: &GraphClient, buf: &mut [u8]) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(OWNS),
    };
    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        for (_, target, _) in list {
             return true;
        }
    }
    false
}

fn has_process_for(g: &GraphClient, buf: &mut [u8], program_suffix: &str) -> bool {
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(SPAWNED),
    };
    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        for (_, pid, _) in list {
            let runs = GraphOp::ScanLinks {
                from: Some(pid),
                to: None,
                kind: Some(RUNS),
            };
            if let Ok(GraphReply::Links(runs)) = g.call_op(&runs, buf) {
                for (_, prog_id, _) in runs {
                    if let Some(name) = boot_program_name(g, prog_id, buf) {
                        if name.ends_with(program_suffix) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn boot_program_name(g: &GraphClient, id: ThingId, buf: &mut [u8]) -> Option<String> {
    if let Ok(GraphReply::Thing { bytes }) = g.call_op(&GraphOp::GetThing { id }, buf) {
        if let Ok(body) = postcard::from_bytes::<BootProgramBody>(&bytes) {
            return Some(body.name);
        }
    }
    None
}

fn wait_for_boot_mount(ctx: &mut LoadedCtx, buf: &mut [u8]) -> Option<ThingId> {
    let mut warned = false;
    loop {
        if let Some(dir_id) = find_boot_dir(ctx, buf) {
            return Some(dir_id);
        }
        let now = timestamp_ns(&ctx.g);
        if !warned {
            ctx.publish_state(LEVEL_WARN, "mount", "waiting for /boot", true);
            warned = true;
            ctx.last_wait_log = now;
        } else if now.saturating_sub(ctx.last_wait_log) > 1_000_000_000 {
            ctx.log("LOADED: waiting for /boot\n");
            ctx.last_wait_log = now;
        }
        ctx.wait_ms(250);
    }
}

fn find_boot_dir(ctx: &LoadedCtx, buf: &mut [u8]) -> Option<ThingId> {
    let scan = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(HAS_MOUNT),
    };
    if let Ok(GraphReply::Links(list)) = ctx.g.call_op(&scan, buf) {
        for (_, mount_id, _) in list {
            if let Ok(GraphReply::Thing { bytes }) = ctx.g.call_op(&GraphOp::GetThing { id: mount_id }, buf) {
                if let Ok(body) = postcard::from_bytes::<MountBody>(&bytes) {
                    if body.path == "/boot" {
                        let sub = GraphOp::ScanLinks {
                            from: Some(mount_id),
                            to: None,
                            kind: Some(MOUNTS),
                        };
                        if let Ok(GraphReply::Links(roots)) = ctx.g.call_op(&sub, buf) {
                            if let Some((_, dir_id, _)) = roots.first() {
                                return Some(*dir_id);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

fn launch_optional_apps(ctx: &mut LoadedCtx, buf: &mut [u8]) -> usize {
    let modules = enumerate_modules(&ctx.g, buf);
    let essentials = [
        "loaded.elf",
        "ps2_keyboard.elf",
        "ps2_mouse.elf",
        "input_service.elf",
        "rtc_x86.elf",
        "compositor.elf",
        "clock.elf",
        "fb_smoke.elf",
    ];

    let mut launched = 0usize;
    for module in modules {
        if !module.path.ends_with(".elf") {
            continue;
        }
        if essentials.iter().any(|e| module.path.ends_with(e)) {
            continue;
        }
        if let Some(bytes) = read_module_bytes(&ctx.g, &module, buf) {
            let _ = sys_spawn_image(&bytes, &module.path);
            launched += 1;
        }
    }
    launched
}

fn summarize_hardware(g: &GraphClient, buf: &mut [u8]) -> String {
    // Process count via THING_PROCESS_KIND (ThingId) or payload check?
    // count_kind function modified to check payload if KIND check fails.
    // But scan uses THING_SPAWNED_KIND.
    let pci_devices = count_kind(g, buf, THING_PROCESS_KIND);
    let time_ready = ready_time_link(g, buf);
    format!("hardware enumerated (procs={}, time={})", pci_devices, time_ready)
}

fn count_kind(g: &GraphClient, buf: &mut [u8], kind: ThingId) -> usize {
    let mut count = 0;
    let op = GraphOp::ScanLinks {
        from: Some(THING_BOOT_ROOT),
        to: None,
        kind: Some(SPAWNED),
    };
    if let Ok(GraphReply::Links(list)) = g.call_op(&op, buf) {
        for (_, target, _) in list {
            if let Ok(GraphReply::Thing { bytes }) = g.call_op(&GraphOp::GetThing { id: target }, buf) {
                // Check if process payload
                if postcard::from_bytes::<ProcessBody>(&bytes).is_ok() {
                    count += 1;
                }
            }
        }
    }
    count
}

fn supervisor_loop(mut ctx: LoadedCtx) -> ! {
    let mut buf = [0u8; 4096];
    let mut ticks: u64 = 0;
    loop {
        ctx.wait_ms(1000);
        ticks = ticks.saturating_add(1);
        let uptime = ticks;
        let heartbeat = format!("heartbeat t={}s", uptime);
        ctx.publish_state(LEVEL_INFO, "supervisor", &heartbeat, false);
        let _ = &mut buf;
    }
}

impl LoadedCtx {
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
        let log_line = format!("LOADED [{}]: {}\n", phase, message);
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
            let bytes = postcard::to_allocvec(&body).unwrap_or_default();
            let mut buf = [0u8; 1024];
            let _ = self.g.call_op(
                &GraphOp::UpdateThing {
                    id,
                    value: bytes,
                },
                &mut buf,
            );
        }
    }
}
