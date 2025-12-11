extern crate alloc;

use crate::FRAMEBUFFER_REQUEST;
use abi::{PixelFormat, PropValue, Thing, ThingId};
use alloc::{boxed::Box, string::String, vec::Vec};
use kernel_core::memory::{BootFrameAllocator, PhysFrame, allocate_frame, init_frame_pool};
use kernel_core::model;
use kernel_core::{graph, graph_kinds, log, shared_buffer, time};
use limine::memory_map::EntryType;
use limine::request::{HhdmRequest, MemoryMapRequest, ModuleRequest, MpRequest};
use thing_models::{AlarmRequest, BootProgram, FontModule, TimeSource};

#[used]
#[unsafe(link_section = ".requests")]
static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MP_REQUEST: MpRequest = MpRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
pub static HHDM_REQUEST: HhdmRequest = HhdmRequest::new();

#[used]
#[unsafe(link_section = ".requests")]
static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

pub fn seed_memory_graph_from_limine() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        log("No Limine memory map; skipping memory graph seeding");
        return;
    };

    let hhdm_offset = if let Some(hhdm) = HHDM_REQUEST.get_response() {
        let offset = hhdm.offset();
        // shared_buffer::set_hhdm_offset(offset); // Removed as shared_buffer now uses kernel_core::memory::hhdm
        kernel_core::memory::set_hhdm_offset(offset);
        offset
    } else {
        0
    };

    let mut heap_initialized = false;
    let mut boot_allocator = BootFrameAllocator::new();
    let mut range_index = 0_usize;
    const PAGE_SIZE: u64 = 4096;
    const MIN_PHYS_ALLOC: u64 = 0x10_0000; // avoid using very low memory for page tables/allocations

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        let mut base = entry.base;
        let mut len = entry.length;

        // Skip anything entirely below the minimum and trim low memory that overlaps it.
        let end = base.saturating_add(len);
        if end <= MIN_PHYS_ALLOC {
            continue;
        }
        if base < MIN_PHYS_ALLOC {
            let delta = MIN_PHYS_ALLOC - base;
            base = MIN_PHYS_ALLOC;
            len = len.saturating_sub(delta);
        }

        if !heap_initialized {
            let heap_size = crate::heap::KERNEL_HEAP_SIZE_BYTES as u64;
            if len >= heap_size {
                let heap_start_phys = base;
                let heap_start_virt = (heap_start_phys as u64 + hhdm_offset) as usize;

                unsafe { crate::heap::init_kernel_heap(heap_start_virt, heap_size as usize) };

                let heap_mib = heap_size / (1024 * 1024);
                let msg = alloc::format!("Initialized kernel heap ({} MiB)", heap_mib);
                let leaked: &'static str = Box::leak(msg.into_boxed_str());
                log(leaked);

                base += heap_size as u64;
                len -= heap_size as u64;
                heap_initialized = true;
            }
        }

        if len < PAGE_SIZE {
            continue;
        }

        let region_end = base.saturating_add(len);
        let msg = alloc::format!(
            "[INFO] Range {}: 0x{:016x} - 0x{:016x}",
            range_index,
            base,
            region_end
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
        range_index += 1;

        boot_allocator.add_region(base, len);

        let frame_size = 4096;

        // For now: one pool per usable region.
        let _pool = model::create_frame_pool(base, base + len, frame_size);

        // In future, you can optionally explode this into many PhysFrame Things.
    }

    init_frame_pool(boot_allocator);

    let (total_frames, _used_frames, free_frames) = kernel_core::memory::frame_stats();
    let summary = alloc::format!(
        "[INFO] BootFrameAllocator: total_frames={} free_frames={}",
        total_frames,
        free_frames
    );
    let leaked: &'static str = Box::leak(summary.into_boxed_str());
    log(leaked);

    log("Seeded memory graph from Limine memory map");
}

pub fn seed_cpu_graph_from_limine() {
    if let Some(resp) = MP_REQUEST.get_response() {
        let cores = resp.cpus().len() as u64;
        for idx in 0..cores {
            let _ = model::create_cpu_core(idx);
        }
        log("Seeded CpuCore Things from Limine SMP");
    } else {
        // Fallback: single core
        let _ = model::create_cpu_core(0);
        log("No SMP info; created single CpuCore(0)");
    }
}

pub fn seed_boot_profile() {
    model::init_boot_profile();
}

pub fn seed_display_from_limine() {
    let Some(response) = FRAMEBUFFER_REQUEST.get_response() else {
        log("No framebuffer provided by Limine; skipping display seeding");
        return;
    };

    let Some(fb) = response.framebuffers().next() else {
        log("Framebuffer request returned no framebuffers");
        return;
    };

    let width = fb.width();
    let height = fb.height();
    let pitch = fb.pitch();
    let bpp = fb.bpp();
    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);
    let fb_addr = (fb.addr() as u64).saturating_sub(hhdm_offset);
    let size_bytes = pitch as u64 * height as u64;

    if bpp != 32 {
        log("Unexpected framebuffer bpp; proceeding with assumption of 32bpp");
    }

    let start = fb_addr & !(4096 - 1);
    let end = shared_buffer::align_up(fb_addr.saturating_add(size_bytes), 4096);

    let mut frames: heapless::Vec<PhysFrame, { shared_buffer::MAX_FRAMES_PER_BUFFER }> =
        heapless::Vec::new();

    let mut addr = start;
    while addr < end {
        if frames
            .push(PhysFrame::from_start_address(addr, 4096))
            .is_err()
        {
            log("Framebuffer does not fit in SharedBuffer frame capacity");
            return;
        }
        addr = addr.saturating_add(4096);
    }

    let pixel_format = PixelFormat::Bgra8888;
    let info = abi::SharedBufferInfo {
        width: width as u32,
        height: height as u32,
        stride: pitch as u32,
        pixel_format,
    };

    fn allocate_logical_frames(
        size: u64,
    ) -> Option<heapless::Vec<PhysFrame, { shared_buffer::MAX_FRAMES_PER_BUFFER }>> {
        let needed = shared_buffer::page_count_for_size(size);
        let mut frames: heapless::Vec<PhysFrame, { shared_buffer::MAX_FRAMES_PER_BUFFER }> =
            heapless::Vec::new();
        let mut remaining = needed;
        while remaining > 0 {
            if let Some(frame) = allocate_frame() {
                if frames.push(frame).is_err() {
                    return None;
                }
                remaining -= 1;
            } else {
                return None;
            }
        }
        Some(frames)
    }

    fn register_logical_buffer(
        width: u32,
        height: u32,
        stride: u32,
        pixel_format: PixelFormat,
        size: u64,
    ) -> Option<ThingId> {
        let frames = allocate_logical_frames(size)?;
        shared_buffer::register_shared_buffer(width, height, stride, pixel_format, frames).ok()
    }

    match shared_buffer::register_shared_buffer(
        info.width,
        info.height,
        info.stride,
        pixel_format,
        frames,
    ) {
        Ok(buffer_id) => {
            let Some(display_id) =
                shared_buffer::create_display_for_buffer(buffer_id, "display0", &info)
            else {
                log("Failed to create Display Thing for framebuffer");
                return;
            };

            let buffer_bytes = info.stride as u64 * info.height as u64;
            let Some(front_buffer_id) = register_logical_buffer(
                info.width,
                info.height,
                info.stride,
                pixel_format,
                buffer_bytes,
            ) else {
                log("Failed to allocate front display buffer for double buffering");
                return;
            };

            let Some(back_buffer_id) = register_logical_buffer(
                info.width,
                info.height,
                info.stride,
                pixel_format,
                buffer_bytes,
            ) else {
                log("Failed to allocate back display buffer for double buffering");
                return;
            };

            let _ = graph::add_edge(
                display_id,
                graph_kinds::EDGE_DISPLAY_HAS_FRONT_BUFFER,
                front_buffer_id,
            );
            let _ = graph::add_edge(
                display_id,
                graph_kinds::EDGE_DISPLAY_HAS_BACK_BUFFER,
                back_buffer_id,
            );
            let _ = graph::update_thing(
                display_id,
                &[(
                    graph_kinds::PROP_DISPLAY_ACTIVE_BUFFER_INDEX,
                    PropValue::I64(0),
                )],
            );
            log("Seeded display0 and SharedBuffer from Limine framebuffer");
        }
        Err(msg) => log(msg),
    }
}

pub fn seed_program_images_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        log("No Limine modules found for ProgramImage seeding");
        return;
    };

    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);

    let mut created = 0_u64;
    let mut skipped_fonts = 0_u64;

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = match classify_limine_module((*module).string(), (*module).path(), index) {
            ModuleKind::Program { identifier } => identifier,
            ModuleKind::Font { .. } => {
                skipped_fonts = skipped_fonts.saturating_add(1);
                continue;
            }
            ModuleKind::Raw { .. } => continue,
        };
        let virt_addr = (*module).addr() as u64;
        let base_phys = virt_addr.saturating_sub(hhdm_offset);
        let size = (*module).size() as u64;
        if kernel_core::model::create_program_image(&identifier, index as u64, base_phys, size)
            .is_some()
        {
            created = created.saturating_add(1);
        } else {
            log("Failed to create ProgramImage Thing");
        }
    }

    let mut msg = alloc::format!("Seeded {} ProgramImage Things from Limine modules", created);
    if skipped_fonts > 0 {
        let suffix = alloc::format!(" (ignored {} font module(s))", skipped_fonts);
        msg.push_str(&suffix);
    }
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    log(leaked);
}

pub fn seed_font_modules_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        log("No Limine modules found for FontModule seeding");
        return;
    };

    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);

    let mut created = 0_u64;

    for (index, module) in response.modules().iter().enumerate() {
        let name = match classify_limine_module((*module).string(), (*module).path(), index) {
            ModuleKind::Font { name } => name,
            ModuleKind::Program { .. } => continue,
            ModuleKind::Raw { .. } => continue,
        };

        let virt_addr = (*module).addr() as u64;
        let base_phys = virt_addr.saturating_sub(hhdm_offset);
        let size = (*module).size() as u64;

        let font = FontModule {
            id: ThingId(0),
            name,
            module_index: index as u64,
            base_phys,
            size,
        };
        let mut props_vec = Vec::new();
        font.to_props(&mut props_vec);
        let props_slice = Box::leak(props_vec.into_boxed_slice());

        if graph::create_thing(graph_kinds::KIND_FONT_MODULE, props_slice).is_some() {
            created = created.saturating_add(1);
        }
    }

    let msg = alloc::format!("Seeded {} FontModule Things from Limine modules", created);
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    log(leaked);
}

pub fn seed_boot_programs_from_limine() {
    let Some(profile_id) =
        graph::next_thing_of_kind(graph_kinds::KIND_BOOT_PROFILE, ThingId(u64::MAX))
    else {
        log("No BootProfile found; skipping BootProgram seeding");
        return;
    };

    let Some(response) = MODULE_REQUEST.get_response() else {
        log("No Limine modules found for BootProgram seeding");
        return;
    };

    let mut app_id = 1_u64;
    let mut created = 0_u64;
    let mut skipped_fonts = 0_u64;

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = match classify_limine_module((*module).string(), (*module).path(), index) {
            ModuleKind::Program { identifier } => identifier,
            ModuleKind::Font { .. } => {
                skipped_fonts = skipped_fonts.saturating_add(1);
                continue;
            }
            ModuleKind::Raw { .. } => continue,
        };

        if identifier == "init" {
            continue;
        }

        if !cfg!(feature = "rootfs") && identifier == "rootfs" {
            continue;
        }

        if boot_program_exists(&identifier) {
            continue;
        }

        let boot_program = BootProgram {
            id: ThingId(0),
            name: identifier.clone(),
            app_id,
            priority: 0,
            binary: identifier.clone(),
        };

        let mut props_vec = Vec::new();
        boot_program.to_props(&mut props_vec);
        let props_slice = Box::leak(props_vec.into_boxed_slice());

        if let Some(program) = graph::create_thing(graph_kinds::KIND_BOOT_PROGRAM, props_slice) {
            let _ = graph::add_edge(profile_id, graph_kinds::EDGE_LAUNCHES, program);
            created = created.saturating_add(1);
            app_id = app_id.saturating_add(1);
        } else {
            log("Failed to create BootProgram Thing");
        }
    }

    let mut msg = alloc::format!("Seeded {} BootProgram Things from Limine modules", created);
    if skipped_fonts > 0 {
        let suffix = alloc::format!(" (ignored {} font module(s))", skipped_fonts);
        msg.push_str(&suffix);
    }
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    log(leaked);
}

pub fn seed_raw_modules_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        return;
    };

    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);

    let mut created = 0_u64;

    for (index, module) in response.modules().iter().enumerate() {
        let (kind_str, identifier) =
            match classify_limine_module((*module).string(), (*module).path(), index) {
                ModuleKind::Raw { kind, identifier } => (kind, identifier),
                _ => continue,
            };

        let virt_addr = (*module).addr() as u64;
        let base_phys = virt_addr.saturating_sub(hhdm_offset);
        let size = (*module).size() as u64;

        // Try to create a SharedBuffer for this module
        let buffer_id = if base_phys % 4096 == 0 {
            let mut frames: heapless::Vec<PhysFrame, { shared_buffer::MAX_FRAMES_PER_BUFFER }> =
                heapless::Vec::new();

            let start_addr = base_phys;
            let end_addr = shared_buffer::align_up(base_phys + size, 4096);
            let mut addr = start_addr;
            let mut success = true;

            while addr < end_addr {
                if frames
                    .push(PhysFrame::from_start_address(addr, 4096))
                    .is_err()
                {
                    log("Module too large for SharedBuffer");
                    success = false;
                    break;
                }
                addr += 4096;
            }

            if success {
                // Fake dimensions to satisfy SharedBuffer requirements.
                // We use Rgba8888 (4 bytes/pixel), so width = size / 4.
                // We round size up to multiple of 4.
                let aligned_size = shared_buffer::align_up(size, 4);
                let width = (aligned_size / 4) as u32;
                let height = 1;
                let stride = width * 4;

                match shared_buffer::register_shared_buffer(
                    width,
                    height,
                    stride,
                    abi::PixelFormat::Rgba8888,
                    frames,
                ) {
                    Ok(id) => Some(id),
                    Err(e) => {
                        log(e);
                        None
                    }
                }
            } else {
                None
            }
        } else {
            log("Module not page aligned, cannot create SharedBuffer");
            None
        };

        let mut props_vec = alloc::vec::Vec::new();
        props_vec.push((
            abi::graph_kinds::PROP_IDENTIFIER,
            abi::PropValue::Str(identifier),
        ));
        props_vec.push((
            abi::graph_kinds::PROP_RAW_KIND,
            abi::PropValue::Str(kind_str),
        ));
        props_vec.push((
            abi::graph_kinds::PROP_MODULE_INDEX,
            abi::PropValue::U64(index as u64),
        ));
        props_vec.push((
            abi::graph_kinds::PROP_BASE_PHYS,
            abi::PropValue::U64(base_phys),
        ));
        props_vec.push((abi::graph_kinds::PROP_SIZE, abi::PropValue::U64(size)));

        if let Some(bid) = buffer_id {
            props_vec.push((
                abi::graph_kinds::PROP_FRAMEBUFFER_ID,
                abi::PropValue::U64(bid.0),
            ));
        }

        let props_slice = Box::leak(props_vec.into_boxed_slice());

        if graph::create_thing(graph_kinds::KIND_RAW_MODULE, props_slice).is_some() {
            created = created.saturating_add(1);
        }
    }

    if created > 0 {
        let msg = alloc::format!("Seeded {} RawModule Things from Limine modules", created);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    }
}

pub fn seed_time_graph() {
    let tick_hz = time::tick_hz();
    let epoch_secs = time::rtc_epoch_seconds();
    let time_props = TimeSource::create(tick_hz, epoch_secs, 0);
    if let Some(time_id) = graph::create_thing(graph_kinds::KIND_TIME_SOURCE, &time_props) {
        time::bind_time_source(time_id);
        let msg = alloc::format!(
            "TimeSource created id={} tick_hz={} epoch_seconds={}",
            time_id.0,
            tick_hz,
            epoch_secs
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    } else {
        log("Failed to create TimeSource Thing");
    }

    let boot_alarm_secs = epoch_secs.saturating_add(3);
    let alarm_props = AlarmRequest::create_pending(boot_alarm_secs, 0, ThingId(0), ThingId(0));
    if let Some(alarm_id) = graph::create_thing(graph_kinds::KIND_ALARM_REQUEST, &alarm_props) {
        let msg = alloc::format!(
            "Boot AlarmRequest id={} targeting {} seconds",
            alarm_id.0,
            boot_alarm_secs
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    } else {
        log("Failed to create boot AlarmRequest Thing");
    }
}

enum ModuleKind {
    Program { identifier: String },
    Font { name: String },
    Raw { kind: String, identifier: String },
}

fn classify_limine_module(
    cmdline: &core::ffi::CStr,
    path: &core::ffi::CStr,
    index: usize,
) -> ModuleKind {
    if let Some(name) = parse_font_identifier(cmdline, path) {
        ModuleKind::Font { name }
    } else if let Some((kind, identifier)) = parse_raw_identifier(cmdline, path) {
        ModuleKind::Raw { kind, identifier }
    } else {
        ModuleKind::Program {
            identifier: derive_module_identifier(cmdline, path, index),
        }
    }
}

fn derive_module_identifier(
    cmdline: &core::ffi::CStr,
    path: &core::ffi::CStr,
    index: usize,
) -> String {
    if let Some(cmd_ident) = parse_identifier_from_cmdline(cmdline) {
        return cmd_ident;
    }

    if let Some(path_ident) = parse_identifier_from_path(path) {
        return path_ident;
    }

    let mut s = String::new();
    s.push_str("module_");
    if index == 0 {
        s.push('0');
        return s;
    }
    let mut n = index;
    let mut buf = [0u8; 20];
    let mut i = 0usize;
    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }
    for j in (0..i).rev() {
        s.push(buf[j] as char);
    }
    s
}

fn last_path_component(input: &str) -> &str {
    input
        .rsplit_once('/')
        .map(|(_, tail)| tail)
        .unwrap_or(input)
}

fn parse_identifier_from_cmdline(cmdline: &core::ffi::CStr) -> Option<String> {
    let bytes = cmdline.to_bytes();
    if bytes.is_empty() {
        return None;
    }

    if let Ok(line) = core::str::from_utf8(bytes) {
        if let Some(result) = parse_program_argument(line) {
            return Some(result);
        }
        if !line.is_empty() {
            return Some(String::from(last_path_component(line)));
        }
    } else {
        let owned = String::from_utf8_lossy(bytes);
        let line = owned.as_ref();
        if let Some(result) = parse_program_argument(line) {
            return Some(result);
        }
        if !line.is_empty() {
            return Some(String::from(last_path_component(line)));
        }
    }

    None
}

fn parse_identifier_from_path(path: &core::ffi::CStr) -> Option<String> {
    if let Ok(pstr) = path.to_str() {
        let component = last_path_component(pstr);
        if !component.is_empty() {
            return Some(String::from(component));
        }
    } else if let Ok(s) = core::str::from_utf8(path.to_bytes()) {
        let component = last_path_component(s);
        if !component.is_empty() {
            return Some(String::from(component));
        }
    }
    None
}

fn parse_program_argument(line: &str) -> Option<String> {
    parse_keyed_argument(line, "program=")
}

fn parse_font_argument(line: &str) -> Option<String> {
    parse_keyed_argument(line, "font=")
}

fn parse_keyed_argument(line: &str, prefix: &str) -> Option<String> {
    line.split_whitespace()
        .find(|arg| arg.starts_with(prefix))
        .and_then(|arg| {
            let ident = arg.trim_start_matches(prefix);
            (!ident.is_empty()).then(|| String::from(ident))
        })
}

fn parse_font_identifier(cmdline: &core::ffi::CStr, path: &core::ffi::CStr) -> Option<String> {
    let bytes = cmdline.to_bytes();
    if !bytes.is_empty() {
        if let Ok(line) = core::str::from_utf8(bytes) {
            if let Some(name) = parse_font_argument(line) {
                return Some(name);
            }
        } else {
            let owned = String::from_utf8_lossy(bytes);
            if let Some(name) = parse_font_argument(owned.as_ref()) {
                return Some(name);
            }
        }
    }

    if let Some(path_ident) = parse_identifier_from_path(path) {
        if let Some(stripped) = path_ident.strip_suffix(".ttf") {
            if !stripped.is_empty() {
                return Some(String::from(stripped));
            }
        }
    }

    None
}

fn parse_raw_identifier(
    cmdline: &core::ffi::CStr,
    path: &core::ffi::CStr,
) -> Option<(String, String)> {
    // Check for "image=" or other raw types in cmdline
    let bytes = cmdline.to_bytes();
    if !bytes.is_empty() {
        let line = String::from_utf8_lossy(bytes);
        if let Some(val) = parse_keyed_argument(&line, "image=") {
            return Some((String::from("image"), val));
        }
    }

    // Check extension
    if let Some(path_ident) = parse_identifier_from_path(path) {
        if path_ident.ends_with(".bmp") {
            return Some((String::from("image"), path_ident));
        }
    }

    None
}

fn boot_program_exists(binary: &str) -> bool {
    let mut cursor = ThingId(u64::MAX);
    loop {
        match graph::next_thing_of_kind(graph_kinds::KIND_BOOT_PROGRAM, cursor) {
            Some(id) => {
                if let Some((_kind, props)) = graph::get_thing(id) {
                    for prop in props.iter().flatten() {
                        if prop.0 == "binary" {
                            if let PropValue::Str(ref s) = prop.1 {
                                if s == binary {
                                    return true;
                                }
                            }
                        }
                    }
                }
                cursor = id;
            }
            None => return false,
        }
    }
}
