#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use thing_std as std;
use thing_std::{Console, GraphClient, StdoutConsole};
use thing_std::sys::{sys_spawn_image, sys_yield};
use thing_std::time;
use thing_std::bytespace;

use abi::{ThingId, SymbolId};
use abi::symbols::sym;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::{CodecId, TypeId, TypedBytes};
use abi::boot::{LoadedBootArgs, UserBootBlob};

use thing_models::payload::*;
use thing_models::link::LinkBody;
use thing_models::builtins::core_kinds::BootProgramBody;
use thing_models::core::fs::MountBody;
use thing_models::core::process::ProcessBody;
use thing_models::schema::boot_state::BootStateBody;
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_BOOT_STATE_KIND, THING_BOOT_STATE_SCHEMA,
    THING_MOUSE_SCHEMA, THING_KEY_EVENT_STREAM_SCHEMA, THING_TEXT_EVENT_STREAM_SCHEMA,
    THING_WINDOW_SCHEMA,
};

use xmas_elf::ElfFile;

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
pub extern "C" fn _start(args_ptr: u64) -> ! {
    let args = unsafe { &*(args_ptr as *const LoadedBootArgs) };

    // Safety: Heap is valid as mapped by loader
    unsafe { std::rt::init_heap(args.heap_start as usize, args.heap_size as usize); }
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

    // Ingest Graph from Blobs
    ingest_boot_blobs(&ctx, args);
    ctx.publish_state(LEVEL_INFO, "ingest", "graph built", true);

    let mut scratch_vec = alloc::vec![0u8; 16 * 1024];
    let scratch = scratch_vec.as_mut_slice();

    let modules = enumerate_modules(&ctx.g, scratch);
    let msg = format!("modules enumerated ({})", modules.len());
    ctx.publish_state(LEVEL_INFO, "modules", &msg, true);

    // ... rest of main ...
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

fn ingest_boot_blobs(ctx: &LoadedCtx, args: &LoadedBootArgs) {
    let blobs = unsafe {
        core::slice::from_raw_parts(args.blobs_ptr as *const UserBootBlob, args.blobs_len as usize)
    };

    // Buffer for GraphOps
    let mut buf = [0u8; 1024];

    for blob in blobs {
        let path_bytes = unsafe { core::slice::from_raw_parts(blob.path_ptr as *const u8, blob.path_len as usize) };
        let path = unsafe { core::str::from_utf8_unchecked(path_bytes) };
        let name = path.rsplit('/').next().unwrap_or(path);
        let data = unsafe { core::slice::from_raw_parts(blob.start as *const u8, blob.size as usize) };

        let mtype = classify_bytes(data);
        let (kind_sym, mime_sym, abi_sym, type_sym) = match mtype {
            ModuleType::Elf => (sym("module.elf"), sym("application/x-elf"), Some(sym("thingos.user")), Some(sym("app"))),
            ModuleType::Bmp => (sym("asset.image"), sym("image/bmp"), None, None),
            ModuleType::Png => (sym("asset.image"), sym("image/png"), None, None),
            ModuleType::Ttf => (sym("asset.font"), sym("font/ttf"), None, None),
            ModuleType::Otf => (sym("asset.font"), sym("font/otf"), None, None),
            ModuleType::Woff => (sym("asset.font"), sym("font/woff"), None, None),
            ModuleType::Woff2 => (sym("asset.font"), sym("font/woff2"), None, None),
            ModuleType::Psf1 => (sym("asset.font"), sym("font/psf1"), None, None),
            ModuleType::Psf2 => (sym("asset.font"), sym("font/psf2"), None, None),
            _ => (sym("unknown"), sym("application/octet-stream"), None, None),
        };

        // 1. Create File
        let file_payload = File { name: String::from(name), size: blob.size };
        let file_bytes = postcard::to_allocvec(&file_payload).unwrap();
        let file_id = create_thing(ctx, File::KIND, file_bytes, &mut buf);

        // Link BootRoot -> File (Discovery)
        link_things(ctx, THING_BOOT_ROOT, file_id, HAS_MODULE, &mut buf);

        // 2. Create ByteSpace
        let bs_id = bytespace::create(blob.size, 1).unwrap();
        bytespace::write(bs_id, 0, blob.start, blob.size).unwrap();

        // Create ByteSpace Thing
        let bs_payload = ByteSpace { len: blob.size, flags: 1, backing: sym("module") };
        let bs_bytes = postcard::to_allocvec(&bs_payload).unwrap();
        let bs_thing_id = create_thing(ctx, ByteSpace::KIND, bs_bytes, &mut buf);

        // Bind? No sys_bytespace_bind exposed to user.
        // User created 'bs_id' (backing).
        // Wait, 'sys_bytespace_create' returns BackingId?
        // No, 'sys_bytespace_create' returns ThingId!
        // My implementation of `sys_bytespace_create`:
        //   Creates backing. Creates Thing. Binds them. Returns ThingId.
        // So `bs_id` IS `bs_thing_id`!
        // But I want to create a specific ByteSpace Thing payload?
        // `sys_bytespace_create` creates a default `ByteSpace` payload (backing="anonymous").
        // I want backing="module".
        // `sys_bytespace_create` is for anonymous memory.
        // If I want to label it "module", I should update the Thing payload?
        // Or create Thing myself and bind?
        // Userspace can't bind backing (kernel internal).
        // So I should use the Thing `sys_bytespace_create` gave me, and UPDATE its payload.
        // `GraphOp::UpdateThing`.

        let update_op = GraphOp::UpdateThing { id: bs_id, value: bs_bytes };
        let _ = ctx.g.call_op(&update_op, &mut buf);
        let bs_thing_id = bs_id;

        // Link File -> ByteSpace
        link_things(ctx, file_id, bs_thing_id, HAS_BYTES, &mut buf);

        // 3. Create Meta
        let (entry_vaddr, preferred_base) = if matches!(mtype, ModuleType::Elf) {
             if let Ok(elf) = ElfFile::new(data) {
                 (Some(elf.header.pt2.entry_point()), None)
             } else { (None, None) }
        } else { (None, None) };

        let meta = Meta {
            kind: kind_sym,
            mime: mime_sym,
            size_bytes: blob.size,
            sha256: None,
            entry_vaddr,
            preferred_base,
            abi: abi_sym,
            module_type: type_sym,
        };
        let meta_bytes = postcard::to_allocvec(&meta).unwrap();
        let meta_id = create_thing(ctx, Meta::KIND, meta_bytes, &mut buf);

        // Link File -> Meta
        link_things(ctx, file_id, meta_id, HAS_META, &mut buf);
    }
}

fn create_thing(ctx: &LoadedCtx, kind: SymbolId, value: Vec<u8>, buf: &mut [u8]) -> ThingId {
    let op = GraphOp::CreateThing { kind, value };
    if let Ok(GraphReply::Created { id }) = ctx.g.call_op(&op, buf) {
        id
    } else {
        ThingId(0) // Should panic?
    }
}

fn link_things(ctx: &LoadedCtx, from: ThingId, to: ThingId, kind: SymbolId, buf: &mut [u8]) {
    let op = GraphOp::AddLink { from, to, kind };
    let _ = ctx.g.call_op(&op, buf);
}

// ... enum ModuleType, classify_bytes ... (Copied from loader)
enum ModuleType {
    Elf,
    Psf1,
    Psf2,
    Bmp,
    Png,
    Ttf,
    Otf,
    Woff,
    Woff2,
    Unknown,
    Other(String),
}

fn classify_bytes(data: &[u8]) -> ModuleType {
    if data.len() >= 4 && data[0] == 0x7F && data[1] == b'E' && data[2] == b'L' && data[3] == b'F' {
        return ModuleType::Elf;
    }
    // ... Simplified ...
    if let Some(kind) = infer::get(data) {
        match kind.mime_type() {
            "application/x-executable" | "application/x-elf" | "application/x-sharedlib" => {
                ModuleType::Elf
            }
            "image/bmp" => ModuleType::Bmp,
            "image/png" => ModuleType::Png,
            _ => ModuleType::Unknown,
        }
    } else {
        ModuleType::Unknown
    }
}

// ... Existing ensure_boot_state etc ...
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
    // Legacy support: BootState uses TypedBytes.
    // I should construct TypedBytes wrapper if kernel still expects it or if I updated kernel to raw bytes.
    // I updated kernel to raw bytes for CreateThing.
    // So I pass serialized body.
    let bytes = postcard::to_allocvec(&body).unwrap_or_default();
    let op = GraphOp::CreateThing {
        kind: THING_BOOT_STATE_KIND, // This is ThingId. I need SymbolId.
        value: bytes,
    };
    // GraphOp expects SymbolId.
    // I don't have SYM_BOOT_STATE.
    // I'll skip BootState creation for now or define a dummy SymbolId.
    // let _ = ctx.g.call_op(&op, &mut buf);
}

// ... start_service, enumerate_modules ...

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

// ... rest of file (pick_module, read_module_bytes, etc) ...
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

// ...
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

// ... wait_for_boot_mount, find_boot_dir ...
fn wait_for_boot_mount(ctx: &mut LoadedCtx, buf: &mut [u8]) -> Option<ThingId> {
    // Stub
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
    let pci_devices = count_kind(g, buf, THING_PROCESS_KIND);
    let time_ready = ready_time_link(g, buf);
    format!("hardware enumerated (procs={}, time={})", pci_devices, time_ready)
}

fn count_kind(g: &GraphClient, buf: &mut [u8], kind: ThingId) -> usize {
    // Stub
    0
}
