use crate::memory::bytespace::{Bytespace as MemBytespace, BytespaceKind};
use graph::store;
use graph::symbols::{self, sym};
use models::{
    Bytespace, DisplayDevice, Framebuffer, HardwareInfo, Module, MouseStream, Pointer, Service,
    Surface, EventStream,
};
use alloc::vec::Vec;
use abi::cap::CapOp;
use abi::bodies::ThingEnvelopeV1;

pub fn init_graph_system() {
    graph::init();
    graph::seed_minimal();
}

pub fn seed_kernel_permissions() {
    // Ensure graph.permissions exists
    let perm_graph =
        if let Some(p) = store::find_thing_by_name(symbols::intern(b"graph.permissions")) {
            p
        } else {
            let p = store::thing_create(sym::KIND_GRAPH);
            store::thing_register_name(p, symbols::intern(b"graph.permissions"));
            // Link to root if possible
            if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
                store::relationship_create(sym::PRED_CONTAINS, root, p);
            }
            p
        };

    let permissions = [
        "perm.log",
        "perm.create",
        "perm.link",
        "perm.unlink",
        "perm.read",
        "perm.write",
        "perm.watch",
        "perm.mem",
        "perm.dictator",
    ];

    for perm_name in &permissions {
        let sym = symbols::intern(perm_name.as_bytes());
        if store::find_thing_by_name(sym).is_none() {
            let perm = store::thing_create(symbols::intern(b"kind.permission"));
            store::thing_register_name(perm, sym);
            store::relationship_create(sym::PRED_CONTAINS, perm_graph, perm);
        }
    }
}

pub fn seed_service_plan(ctx: &crate::boot::BootContext) {
    let root = store::find_thing_by_name(sym::GRAPH_ROOT);
    let ensure_graph =
        |name: abi::ids::SymbolId, parent: Option<abi::ids::ThingId>| -> abi::ids::ThingId {
            if let Some(existing) = store::find_thing_by_name(name) {
                existing
            } else {
                let g = store::thing_create(sym::KIND_GRAPH);
                store::thing_register_name(g, name);
                if let Some(p) = parent {
                    store::relationship_create(sym::PRED_CONTAINS, p, g);
                }
                g
            }
        };

    let services_root = ensure_graph(symbols::intern(b"graph.services"), root);
    let time_graph = ensure_graph(sym::GRAPH_SERVICES_TIME, Some(services_root));
    let clock_graph = ensure_graph(sym::GRAPH_APPS_CLOCK, Some(services_root));
    let core_graph = ensure_graph(symbols::intern(b"graph.services.core"), Some(services_root));

    let svc_kind = symbols::intern(b"kind.Service");
    let module_kind = symbols::intern(b"kind.Module");

    let create_module =
        |name: &str, parent: abi::ids::ThingId| -> Option<abi::ids::ThingId> {
            let module_sym = symbols::intern(alloc::format!("module.{}", name).as_bytes());
            if let Some(existing) = store::find_thing_by_name(module_sym) {
                return Some(existing);
            }

            let info = ctx.modules.iter().find(|m| m.path.contains(name))?;
            let bs = MemBytespace::new_module(info.phys_addr, info.size as usize);
            let module_thing = store::thing_create(module_kind);
            store::thing_register_name(module_thing, module_sym);

            let mut caps = [CapOp::Log; 8];
            let mut cap_count = 0u8;
            for op in default_caps_for(name).into_iter().flatten() {
                caps[cap_count as usize] = op;
                cap_count += 1;
            }

            let mut deps = [symbols::intern(b""); 8];
            let mut dep_count = 0u8;
            for dep in default_deps_for(name).into_iter().flatten() {
                deps[dep_count as usize] = dep;
                dep_count += 1;
            }

            let module_body = Module {
                name: symbols::intern(name.as_bytes()),
                bytespace: bs.id,
                size: info.size,
                caps,
                cap_count,
                deps,
                dep_count,
                _pad: 0,
            };
            let _ = store::thing_set_body(module_thing, &module_body.encode_full());
            store::relationship_create(sym::PRED_CONTAINS, parent, module_thing);
            
            // Register boot grants for this module
            let boot_caps: Vec<abi::cap::Cap> = caps.iter()
                .take(cap_count as usize)
                .map(|&op| abi::cap::Cap {
                    op,
                    scope: abi::cap::CapScope::Global,
                })
                .collect();
            crate::boot_grants::register_module_grants(
                module_thing,
                Some(alloc::string::String::from(name)),
                boot_caps
            );
            
            Some(module_thing)
        };

    let create_service =
        |graph: abi::ids::ThingId, name: &str| -> Option<abi::ids::ThingId> {
            let sym_name = symbols::intern(alloc::format!("service.{}", name).as_bytes());
            if let Some(existing) = store::find_thing_by_name(sym_name) {
                return Some(existing);
            }

            let s = store::thing_create(svc_kind);
            store::thing_register_name(s, sym_name);
            let svc_body = Service {
                name: sym_name,
                pid: 0,
                state: 0,
            };
            let _ = store::thing_set_body(s, &svc_body.encode_full());
            store::relationship_create(sym::PRED_CONTAINS, graph, s);
            Some(s)
        };

    let wire_service =
        |graph: abi::ids::ThingId, name: &str| -> Option<(abi::ids::ThingId, Option<abi::ids::ThingId>)> {
            let svc = create_service(graph, name)?;
            let module = create_module(name, graph);
            if let Some(m) = module {
                let _ = store::relationship_create(sym::PRED_OWNS, svc, m);
            }
            Some((svc, module))
        };

    // Core services visible to Sprout
    let _ = wire_service(core_graph, "bloom");
    let _ = wire_service(core_graph, "inputd");
    let _ = wire_service(core_graph, "thingcheck");
    let _ = wire_service(core_graph, "hello_window");

    // Time pipeline: RTC driver -> timed -> clock app
    let rtc_dep = {
        #[cfg(target_arch = "x86_64")]
        {
            wire_service(time_graph, "rtc_cmos").map(|(svc, _)| svc)
        }
        #[cfg(target_arch = "aarch64")]
        {
            wire_service(time_graph, "rtc_pl031").map(|(svc, _)| svc)
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            None
        }
    };

    let timed = wire_service(time_graph, "timed").map(|(svc, _)| svc);
    if let (Some(timed_id), Some(rtc_id)) = (timed, rtc_dep) {
        let _ = store::relationship_create(sym::PRED_REFERENCES, timed_id, rtc_id);
    }

    let clock = wire_service(clock_graph, "clock").map(|(svc, _)| svc);
    if let (Some(clock_id), Some(timed_id)) = (clock, timed) {
        let _ = store::relationship_create(sym::PRED_REFERENCES, clock_id, timed_id);
    }

    crate::log::kprintln("BOOT: seeded service plan");
}

fn default_caps_for(name: &str) -> [Option<CapOp>; 8] {
    match name {
        "sprout" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GrantCaps),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphUnlink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
        ],
        "bloom" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphUnlink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            Some(CapOp::GraphWatch),
        ],
        "inputd" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::InputRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            None,
        ],
        "clock" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            None,
            None
        ],
        "timed" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            None,
            None,
        ],
        "rtc_cmos" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            Some(CapOp::IoPort),
            None,
        ],
        "rtc_pl031" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::Hardware),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphWrite),
            None,
        ],
                "hello_window" => [
            Some(CapOp::Log),
            Some(CapOp::MemManage),
            Some(CapOp::GraphCreate),
            Some(CapOp::GraphLink),
            Some(CapOp::GraphRead),
            Some(CapOp::GraphWrite),
            None,
            None,
        ],
        _ => [Some(CapOp::Log), None, None, None, None, None, None, None],
    }
}

fn default_deps_for(name: &str) -> [Option<abi::ids::SymbolId>; 8] {
    match name {
        "timed" => [
            Some(symbols::intern(b"service.rtc_cmos")),
            Some(symbols::intern(b"service.rtc_pl031")),
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        "clock" => [
            Some(symbols::intern(b"service.timed")),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ],
        _ => [None; 8],
    }
}

pub fn wrap_raw(kind: abi::ids::SymbolId, payload: &[u8]) -> Vec<u8> {
    let header = ThingEnvelopeV1 {
        magic: ThingEnvelopeV1::MAGIC,
        env_version: ThingEnvelopeV1::VERSION,
        flags: 0,
        kind: kind.0,
        schema_hash: 0,
        schema_version: 0,
        schema_str_len: 0,
        payload_format: 0,
        reserved0: 0,
        payload_len: payload.len() as u32,
        body_len: (core::mem::size_of::<ThingEnvelopeV1>() + payload.len()) as u32,
        integrity: 0,
    };
    let mut bytes = Vec::with_capacity(header.body_len as usize);
    unsafe {
        let ptr = &header as *const _ as *const u8;
        bytes.extend_from_slice(core::slice::from_raw_parts(
            ptr,
            core::mem::size_of::<ThingEnvelopeV1>(),
        ));
    }
    bytes.extend_from_slice(payload);
    bytes
}

pub fn seed_bloom_ontology() {
    let root = store::find_thing_by_name(sym::GRAPH_ROOT);

    let ensure_graph =
        |name: abi::ids::SymbolId, root: Option<abi::ids::ThingId>| -> abi::ids::ThingId {
            if let Some(existing) = store::find_thing_by_name(name) {
                existing
            } else {
                let p = store::thing_create(sym::KIND_GRAPH);
                store::thing_register_name(p, name);
                if let Some(parent) = root {
                    store::relationship_create(sym::PRED_CONTAINS, parent, p);
                }
                p
            }
        };

    let display_graph = ensure_graph(symbols::intern(b"graph.display"), root);
    let _windows_graph = ensure_graph(symbols::intern(b"graph.windows"), root);

    let input_graph = if let Some(p) = store::find_thing_by_name(sym::GRAPH_INPUT) {
        p
    } else {
        let p = store::thing_create(sym::KIND_GRAPH);
        store::thing_register_name(p, sym::GRAPH_INPUT);
        if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
            store::relationship_create(sym::PRED_CONTAINS, root, p);
        }
        p
    };

    let mut mouse_stream_id: Option<abi::ids::ThingId> = None;
    #[cfg(target_arch = "x86_64")]
    {
        use crate::machine::x86_64::ps2_mouse;
        let mouse_bs = store::thing_create(sym::KIND_BYTE_SPACE);
        let mouse_bs_name = symbols::intern(b"bytespace.mouse_input");
        store::thing_register_name(mouse_bs, mouse_bs_name);
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_bs);

        let phys_addr = ps2_mouse::get_ring_phys_addr();
        let ring_size = ps2_mouse::get_ring_size() as u64;

        MemBytespace::register_existing(
            mouse_bs,
            BytespaceKind::Device,
            ring_size as usize,
            phys_addr,
        );

        let bs_payload = Bytespace {
            len: 0,
            flags: 0,
            _pad: 0,
            phys_base: 0,
        };
        let _ = store::thing_set_body(mouse_bs, &bs_payload.encode_full());

        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStream {
            bytespace: mouse_bs,
            capacity: ps2_mouse::RING_CAPACITY,
            sample_size: core::mem::size_of::<abi::mouse_ring::MouseSample>() as u32,
            write_index: 0,
            dropped: 0,
        };
        let _ = store::thing_set_body(mouse_stream, &ms_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_stream);
        store::relationship_create(sym::PRED_REFERENCES, mouse_stream, mouse_bs);

        let event_stream = store::thing_create(symbols::intern(b"kind.EventStream"));
        store::thing_register_name(event_stream, symbols::intern(b"event_stream.mouse"));
        let es_payload = EventStream {
            bytespace: mouse_bs,
            capacity_bytes: ps2_mouse::RING_CAPACITY,
            max_record_bytes: 64,
            flags: 0,
            name: symbols::intern(b"mouse"),
        };
        let _ = store::thing_set_body(event_stream, &es_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, event_stream);
        store::relationship_create(symbols::intern(b"stream.bytespace"), event_stream, mouse_bs);
        mouse_stream_id = Some(mouse_stream);
    }

    if mouse_stream_id.is_none() {
        let mouse_stream = store::thing_create(symbols::intern(b"kind.MouseStream"));
        store::thing_register_name(mouse_stream, symbols::intern(b"mouse.stream.0"));
        let ms_payload = MouseStream {
            bytespace: abi::ids::ThingId(0),
            capacity: 0,
            sample_size: 0,
            write_index: 0,
            dropped: 0,
        };
        let _ = store::thing_set_body(mouse_stream, &ms_payload.encode_full());
        store::relationship_create(sym::PRED_CONTAINS, input_graph, mouse_stream);
        mouse_stream_id = Some(mouse_stream);
    }

    let pointer_thing = store::thing_create(sym::KIND_POINTER);
    store::thing_register_name(pointer_thing, symbols::intern(b"pointer.0"));
    store::thing_register_name(pointer_thing, symbols::intern(b"pointer.state"));
    store::relationship_create(sym::REL_HAS_POINTER, input_graph, pointer_thing);
    store::relationship_create(sym::PRED_CONTAINS, input_graph, pointer_thing);
    let pointer_payload = Pointer {
        stream: mouse_stream_id.unwrap_or(abi::ids::ThingId(0)),
        x: 0,
        y: 0,
        buttons: 0,
        updated_at_ns: 0,
    };
    let _ = store::thing_set_body(pointer_thing, &pointer_payload.encode_full());

    let ctx = crate::boot::get_boot_ctx();
    let asset_root = store::thing_create(sym::KIND_GRAPH);
    store::thing_register_name(asset_root, sym::GRAPH_ASSETS);
    if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
        store::relationship_create(sym::PRED_CONTAINS, root, asset_root);
    }

    for m in ctx.modules {
        if m.path.is_empty() {
            continue;
        }
        if m.path.contains("/assets/") {
            let filename = m.path.rsplit('/').next().unwrap_or(m.path);
            let thing_name_str = alloc::format!("asset.{}", filename);
            let bs_name_str = alloc::format!("bytespace.asset.{}", filename);
            let asset_thing = store::thing_create(sym::KIND_ASSET);
            store::thing_register_name(asset_thing, symbols::intern(thing_name_str.as_bytes()));
            let bs = store::thing_create(sym::KIND_BYTESPACE_MODULE);
            store::thing_register_name(bs, symbols::intern(bs_name_str.as_bytes()));
            store::relationship_create(sym::PRED_BACKS, asset_thing, bs);
            store::relationship_create(sym::PRED_CONTAINS, asset_root, asset_thing);

            MemBytespace::register_existing(
                bs,
                BytespaceKind::Module,
                m.size as usize,
                m.phys_addr,
            );

            let bs_payload = Bytespace {
                len: 0,
                phys_base: 0,
                flags: 0,
                _pad: 0,
            };
            let _ = store::thing_set_body(bs, &wrap_raw(sym::KIND_BYTE_SPACE, &bs_payload.encode_full()));
        }
    }

    if let Some(fb) = ctx.framebuffer {
        let fb_thing = store::thing_create(sym::KIND_DEVICE_DISPLAY);
        store::thing_register_name(fb_thing, symbols::intern(b"device.display0"));
        store::thing_register_name(fb_thing, symbols::intern(b"display.0"));
        store::relationship_create(sym::PRED_CONTAINS, display_graph, fb_thing);

        let fb_bytespace = store::thing_create(sym::KIND_BYTESPACE_FRAMEBUFFER);
        store::thing_register_name(fb_bytespace, symbols::intern(b"bytespace.display0"));
        store::relationship_create(sym::PRED_CONTAINS, display_graph, fb_bytespace);

        let surface = store::thing_create(sym::KIND_SURFACE);
        store::thing_register_name(surface, symbols::intern(b"surface.display0"));
        store::relationship_create(sym::PRED_PRIMARY, fb_thing, surface);
        store::relationship_create(sym::PRED_BACKS, surface, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_graph, surface);

        let framebuffer_thing = store::thing_create(symbols::intern(b"kind.Framebuffer"));
        store::thing_register_name(framebuffer_thing, symbols::intern(b"framebuffer.display0"));
        store::relationship_create(sym::PRED_REFERENCES, framebuffer_thing, fb_bytespace);
        store::relationship_create(sym::PRED_CONTAINS, display_graph, framebuffer_thing);

        let fb_size = fb.height * fb.pitch;
        MemBytespace::register_existing(
            fb_bytespace,
            BytespaceKind::Framebuffer,
            fb_size as usize,
            fb.addr,
        );
        let fb_bytespace_payload = Bytespace {
            len: 0,
            flags: 0,
            _pad: 0,
            phys_base: 0,
        };
        let _ = store::thing_set_body(fb_bytespace, &fb_bytespace_payload.encode_full());

        let framebuffer_payload = Framebuffer {
            bytespace: fb_bytespace,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
        };
        let _ = store::thing_set_body(framebuffer_thing, &framebuffer_payload.encode_full());

        let surface_payload = Surface {
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            bytespace: fb_bytespace,
        };
        let _ = store::thing_set_body(surface, &surface_payload.encode_full());

        let display_payload = DisplayDevice {
            framebuffer: framebuffer_thing,
            width: fb.width as u32,
            height: fb.height as u32,
            stride_bytes: fb.pitch as u32,
            format: symbols::intern(b"format.bgra8888"),
            refresh_hz: 60,
        };
        let _ = store::thing_set_body(fb_thing, &display_payload.encode_full());

        crate::machine::input::set_mouse_bounds(fb.width as u32, fb.height as u32);
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, fb_thing);
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        let rtc_hw = store::thing_create(symbols::intern(b"kind.HardwareResource"));
        store::thing_register_name(rtc_hw, symbols::intern(b"hw.rtc0"));
        let rtc_info = HardwareInfo {
            name: symbols::intern(b"cmos-rtc"),
            resource_type: symbols::intern(b"ioport"),
            start: 0x70,
            end: 0x71,
            irq: 8,
            _pad: 0,
        };
        let _ = store::thing_set_body(rtc_hw, &rtc_info.encode_full());
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, rtc_hw);
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        let rtc_hw = store::thing_create(symbols::intern(b"kind.HardwareResource"));
        store::thing_register_name(rtc_hw, symbols::intern(b"hw.rtc0"));
        let rtc_info = HardwareInfo {
            name: symbols::intern(b"pl031"),
            resource_type: symbols::intern(b"mmio"),
            start: 0x09010000,
            end: 0x09011000,
            irq: 34,
            _pad: 0,
        };
        let _ = store::thing_set_body(rtc_hw, &rtc_info.encode_full());
        if let Some(devices) = store::find_thing_by_name(sym::GRAPH_DEVICES) {
            store::relationship_create(sym::PRED_CONTAINS, devices, rtc_hw);
        }
    }
}

pub fn seed_platform_graph() {
    let apic_id = crate::machine::machine().local_cpu_id();
    let timer_freq = crate::machine::machine().timer_frequency_hz();
    let resolution_ns = if timer_freq > 0 {
        1_000_000_000u64 / timer_freq as u64
    } else {
        0
    };

    // Create graph.platform
    let platform_graph = if let Some(p) = store::find_thing_by_name(sym::GRAPH_PLATFORM) {
        p
    } else {
        let p = store::thing_create(sym::KIND_GRAPH);
        store::thing_register_name(p, sym::GRAPH_PLATFORM);
        if let Some(root) = store::find_thing_by_name(sym::GRAPH_ROOT) {
            store::relationship_create(sym::PRED_CONTAINS, root, p);
        }
        p
    };

    // Create CPU Thing (cpu.0)
    let cpu_thing = store::thing_create(sym::KIND_CPU);
    let cpu_name = symbols::intern(b"cpu.0");
    store::thing_register_name(cpu_thing, cpu_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_graph, cpu_thing);

    let mut cpu_payload = alloc::vec::Vec::new();
    cpu_payload.extend_from_slice(&apic_id.to_le_bytes());
    cpu_payload.push(1u8); // is_bsp
    store::thing_set_inline_payload(cpu_thing, &cpu_payload);

    // Create Timer Thing
    let timer_thing = store::thing_create(sym::KIND_TIMER);
    let timer_name = symbols::intern(b"lapic_timer.0");
    store::thing_register_name(timer_thing, timer_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_graph, timer_thing);

    let mut timer_payload = alloc::vec::Vec::new();
    timer_payload.extend_from_slice(b"lapic\0\0\0");
    timer_payload.extend_from_slice(&timer_freq.to_le_bytes());
    timer_payload.push(1u8); // periodic
    timer_payload.extend_from_slice(&resolution_ns.to_le_bytes());
    store::thing_set_inline_payload(timer_thing, &timer_payload);

    store::relationship_create(sym::PRED_HAS_TIMER, cpu_thing, timer_thing);

    // Create InterruptController Thing
    let ic_thing = store::thing_create(sym::KIND_INTERRUPT_CONTROLLER);
    let ic_name = symbols::intern(b"lapic.0");
    store::thing_register_name(ic_thing, ic_name);
    store::relationship_create(sym::PRED_CONTAINS, platform_graph, ic_thing);

    let mut ic_payload = alloc::vec::Vec::new();
    ic_payload.extend_from_slice(b"lapic\0\0\0");
    ic_payload.extend_from_slice(b"local\0\0\0");
    store::thing_set_inline_payload(ic_thing, &ic_payload);
}

pub fn register_xhci(
    bar0: u64,
    irq_line: u8,
    vector: u8,
    irq_phys: u64,
    irq_len: u64,
) {
    const XHCI_MMIO_LEN: u64 = 0x10000;

    // Graph plumbing
    let devices_graph = store::find_thing_by_name(sym::GRAPH_DEVICES)
        .unwrap_or_else(|| store::thing_create(sym::KIND_GRAPH));

    // MMIO bytespace
    let mmio_bs = store::thing_create(sym::KIND_BYTE_SPACE);
    store::thing_register_name(mmio_bs, symbols::intern(b"bytespace.usb.xhci0.mmio"));
    store::relationship_create(sym::PRED_CONTAINS, devices_graph, mmio_bs);

    let mmio_payload = Bytespace {
        len: 0,
        flags: 0,
        _pad: 0,
        phys_base: 0,
    };
    store::thing_set_inline_payload(mmio_bs, &mmio_payload.encode());

    MemBytespace::register_existing(
        mmio_bs,
        BytespaceKind::Device,
        XHCI_MMIO_LEN as usize,
        bar0,
    );

    // IRQ bytespace
    let irq_bs = store::thing_create(sym::KIND_BYTE_SPACE);
    store::thing_register_name(irq_bs, symbols::intern(b"bytespace.irq.usb.xhci0"));
    store::relationship_create(sym::PRED_CONTAINS, devices_graph, irq_bs);

    let irq_payload = Bytespace {
        len: 0,
        flags: 0,
        _pad: 0,
        phys_base: 0,
    };
    store::thing_set_inline_payload(irq_bs, &irq_payload.encode());

    MemBytespace::register_existing(
        irq_bs,
        BytespaceKind::Device,
        irq_len as usize,
        irq_phys,
    );

    // Controller Thing
    let ctrl = store::thing_create(sym::KIND_XHCI_CONTROLLER);
    store::thing_register_name(ctrl, symbols::intern(b"device.usb.controller0"));
    store::relationship_create(sym::PRED_CONTAINS, devices_graph, ctrl);
    store::relationship_create(sym::PRED_MMIO, ctrl, mmio_bs);
    store::relationship_create(sym::PRED_IRQ, ctrl, irq_bs);

    // Inline payload: bar0 | irq line | vector
    let mut ctrl_payload = Vec::new();
    ctrl_payload.extend_from_slice(&bar0.to_le_bytes());
    ctrl_payload.push(irq_line);
    ctrl_payload.push(vector);
    store::thing_set_inline_payload(ctrl, &ctrl_payload);
}
