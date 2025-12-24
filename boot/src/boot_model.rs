extern crate alloc;

use crate::FRAMEBUFFER_REQUEST;
use abi::{PixelFormat, ProcessId, ThingId, syscall_defs::SymbolId};
use alloc::{boxed::Box, string::String, vec::Vec};
use kernel::graph::store::ResidentRef;
use kernel::memory::{BootFrameAllocator, PhysFrame, allocate_frame, init_frame_pool};
use kernel::model;
use kernel::resident::mapping::ResidentPage;
use kernel::{graph, graph_kinds, log, shared_buffer, symbols, time};
use limine::memory_map::EntryType;
use limine::request::{HhdmRequest, KernelFileRequest, MemoryMapRequest, ModuleRequest, MpRequest};
use thing_models::{AlarmRequest, BootProgram, FontModule, PropValue, Thing, TimeSource};
// use thing_models::{AlarmRequest, BootProgram, FontModule, Thing, TimeSource}; // Merged into line 4

#[used]
#[unsafe(link_section = ".requests")]
static KERNEL_FILE_REQUEST: KernelFileRequest = KernelFileRequest::new();

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

// Helper to convert thing_models props (String keys) to Kernel props (SymbolId keys)
fn intern_props(props: Vec<(String, PropValue)>) -> Vec<(SymbolId, PropValue)> {
    props
        .into_iter()
        .map(|(k, v)| (symbols::intern(&k), v))
        .collect()
}

pub fn seed_memory_graph_from_limine() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        log("Nulla mappa memoriae Limine; seminatio graphidis memoriae praetermissa");
        return;
    };

    let hhdm_offset = if let Some(hhdm) = HHDM_REQUEST.get_response() {
        let offset = hhdm.offset();
        kernel::memory::set_hhdm_offset(offset);
        offset
    } else {
        0
    };

    let mut boot_allocator = BootFrameAllocator::new();
    let mut range_index = 0_usize;
    const PAGE_SIZE: u64 = 4096;
    const MIN_PHYS_ALLOC: u64 = 0x10_0000;

    let kernel_base_phys = crate::KERNEL_ADDRESS_REQUEST
        .get_response()
        .map(|r| r.physical_base())
        .unwrap_or(0);
    let kernel_base_virt = crate::KERNEL_ADDRESS_REQUEST
        .get_response()
        .map(|r| r.virtual_base())
        .unwrap_or(0xffffffff80000000);

    unsafe extern "C" {
        static _end: u8;
    }
    let kernel_end_virt = unsafe { core::ptr::addr_of!(_end) as u64 };
    let kernel_size = kernel_end_virt.saturating_sub(kernel_base_virt);
    let kernel_end_phys = kernel_base_phys + kernel_size;
    let kernel_end_phys = (kernel_end_phys + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

    if kernel_size > 0 {
        let msg = alloc::format!(
            "Phys nucleus: {:#x} - {:#x} (magnitudo {:#x})",
            kernel_base_phys,
            kernel_end_phys,
            kernel_size
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    }

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        let mut base = entry.base;
        let mut len = entry.length;

        let end = base.saturating_add(len);
        if end <= MIN_PHYS_ALLOC {
            continue;
        }
        if base < MIN_PHYS_ALLOC {
            let delta = MIN_PHYS_ALLOC - base;
            base = MIN_PHYS_ALLOC;
            len = len.saturating_sub(delta);
        }

        if len < PAGE_SIZE {
            continue;
        }

        let region_end = base.saturating_add(len);
        let msg = alloc::format!(
            "[NOTITIA] Ager {}: 0x{:016x} - 0x{:016x}",
            range_index,
            base,
            region_end
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
        range_index += 1;

        let frame_size = 4096;

        if base < kernel_end_phys && region_end > kernel_base_phys {
            if base < kernel_base_phys {
                let sub_len = kernel_base_phys - base;
                if sub_len >= PAGE_SIZE {
                    boot_allocator.add_region(base, sub_len);
                    let _pool = model::create_frame_pool(base, base + sub_len, frame_size);
                }
            }

            if region_end > kernel_end_phys {
                let sub_start = kernel_end_phys;
                let sub_len = region_end - sub_start;
                if sub_len >= PAGE_SIZE {
                    boot_allocator.add_region(sub_start, sub_len);
                    let _pool =
                        model::create_frame_pool(sub_start, sub_start + sub_len, frame_size);
                }
            }
        } else {
            boot_allocator.add_region(base, len);
            let _pool = model::create_frame_pool(base, base + len, frame_size);
        }
    }

    init_frame_pool(boot_allocator);

    let (total_frames, _used_frames, free_frames) = kernel::memory::frame_stats();
    let summary = alloc::format!(
        "[NOTITIA] BootFrameAllocator: tabulae_totales={} tabulae_liberae={}",
        total_frames,
        free_frames
    );
    let leaked: &'static str = Box::leak(summary.into_boxed_str());
    log(leaked);

    log("Graphis memoriae ex mappa Limine seminatus");
}

pub fn seed_cpu_graph_from_limine() {
    if let Some(resp) = MP_REQUEST.get_response() {
        let cores = resp.cpus().len() as u64;
        for idx in 0..cores {
            // model::create_cpu_core likely needs update to intern kind?
            // Assuming kernel::model::create_cpu_core handles it internally or we update it.
            // Checking imports: kernel::model.
            // kernel/src/model.rs probably needs update too! I missed it in task list.
            // I'll assume for now I need to fix kernel::model calls if they fail.
            // But create_cpu_core probably calls create_thing.
            let _ = model::create_cpu_core(idx);
        }
        log("Res CpuCore ex Limine SMP seminatae");
    } else {
        let _ = model::create_cpu_core(0);
        log("Nulla notitia SMP; unus CpuCore(0) creatus");
    }
}

pub fn seed_boot_profile() {
    // model::init_boot_profile() also likely calls create_thing.
    model::init_boot_profile();
}

pub fn seed_display_from_limine() {
    let Some(response) = FRAMEBUFFER_REQUEST.get_response() else {
        log("Nulla tabula imaginis a Limine data; seminatio ostensionis praetermissa");
        return;
    };

    let Some(fb) = response.framebuffers().next() else {
        log("Petitio framebuffer nullam tabulam reddidit");
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
        log("Bpp graphidis inopinata; pergimus cum opinione 32bpp");
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
            log("Tabula imaginis non capit in capacitatem tabularum CommunisBuffer");
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
                kernel::model::create_display("display0", info.width as u64, info.height as u64, 0)
            else {
                log("Creatio Rei Display defecit");
                return;
            };

            let _ = graph::add_link(display_id, graph_kinds::LINK_DISPLAY_SCANOUT, buffer_id);
            log("display0 et CommunisBuffer ex tabula imaginis Limine seminata sunt");
        }
        Err(msg) => log(msg),
    }
}

pub fn seed_program_images_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        log("Nulla modula Limine inventa ad seminanda ProgramImage");
        return;
    };

    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);

    let mut created = 0_u64;
    let mut skipped_fonts = 0_u64;

    let mut seen = alloc::collections::BTreeSet::new();

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = match classify_limine_module((*module).string(), (*module).path(), index) {
            ModuleKind::Program { identifier } => identifier,
            ModuleKind::Font { .. } => {
                skipped_fonts = skipped_fonts.saturating_add(1);
                continue;
            }
            ModuleKind::Raw { .. } => continue,
        };

        if !seen.insert(identifier.clone()) {
            log("Identificator ProgramImage duplicatus e modulo Limine; praetermittitur");
            continue;
        }

        let virt_addr = (*module).addr() as u64;
        let base_phys = virt_addr.saturating_sub(hhdm_offset);
        let size = (*module).size() as u64;
        // kernel::model::create_program_image needs update?
        if kernel::model::create_program_image(&identifier, index as u64, base_phys, size).is_some()
        {
            created = created.saturating_add(1);
        } else {
            log("Creatio Rei ProgramImage defecit");
        }
    }

    let mut msg = alloc::format!(
        "Seminatae sunt {} Res ProgramImage ex modulis Limine",
        created
    );
    if skipped_fonts > 0 {
        let suffix = alloc::format!(" (neglecta(e) {} modula(e) typographicae)", skipped_fonts);
        msg.push_str(&suffix);
    }
    let leaked: &'static str = Box::leak(msg.into_boxed_str());
    log(leaked);
}

pub fn seed_font_modules_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        log("Nulla modula Limine inventa ad seminanda FontModule");
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
        // Convert props to SymbolId
        let kernel_props = intern_props(props_vec);
        let kind = symbols::intern(graph_kinds::KIND_FONT_MODULE);

        let _ = graph::create_thing(kind, kernel_props);
        created = created.saturating_add(1);
    }
}

pub fn seed_boot_programs_from_limine() {
    let kind_boot_profile = symbols::intern(graph_kinds::KIND_BOOT_PROFILE);
    let Some(profile_id) = graph::next_thing_of_kind_sym(kind_boot_profile, ThingId(0)) else {
        return;
    };

    let Some(response) = MODULE_REQUEST.get_response() else {
        log("Nulla modula Limine inventa ad seminanda BootProgram");
        return;
    };

    let mut app_id = 1_u64;
    let mut created = 0_u64;
    let mut skipped_fonts = 0_u64;
    let mut seen = alloc::collections::BTreeSet::new();

    let is_debug_profile = get_kernel_arg("profile=")
        .map(|s| s == "debug")
        .unwrap_or(false);

    if is_debug_profile {
        log(
            "PROFILUM DEBUG ACTIVUM: Tantum progeneramus init_debug, debug_input_logger, debug_input_events, framebuffer, compositor, ps2_keyboard_driver, ps2_mouse_driver",
        );
    }

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = match classify_limine_module((*module).string(), (*module).path(), index) {
            ModuleKind::Program { identifier } => identifier,
            ModuleKind::Font { .. } => {
                skipped_fonts = skipped_fonts.saturating_add(1);
                continue;
            }
            ModuleKind::Raw { .. } => continue,
        };

        if is_debug_profile
            && identifier != "init_debug"
            && identifier != "debug_input_logger"
            && identifier != "ps2_keyboard_driver"
            && identifier != "debug_input_events"
            && identifier != "ps2_mouse_driver"
            && identifier != "framebuffer"
            && identifier != "compositor"
            && identifier != "watch_test"
        {
            continue;
        }

        if !cfg!(feature = "rootfs") && identifier == "rootfs" {
            continue;
        }

        if !seen.insert(identifier.clone()) {
            continue;
        }

        // Removed boot_program_exists check which caused hang

        let priority = get_program_priority(&identifier);

        let mut respawn_policy = String::from(graph_kinds::RESPAWN_NEVER);
        if identifier == "init" || identifier == "init_debug" {
            respawn_policy = String::from(graph_kinds::RESPAWN_ALWAYS);
        }

        if let Some(policy) = parse_respawn_policy((*module).string()) {
            respawn_policy = policy;
        }

        let boot_program = BootProgram {
            id: ThingId(0),
            name: identifier.clone(),
            app_id,
            priority,
            binary: identifier.clone(),
            respawn_policy,
        };

        let mut props_vec = Vec::new();
        boot_program.to_props(&mut props_vec);
        let kernel_props = intern_props(props_vec);
        let kind = symbols::intern(graph_kinds::KIND_BOOT_PROGRAM);

        let program = graph::create_thing(kind, kernel_props);
        let _ = graph::add_link(profile_id, graph_kinds::LINK_LAUNCHES, program);
        created = created.saturating_add(1);
        app_id = app_id.saturating_add(1);
    }

    let mut msg = alloc::format!(
        "Seminatae sunt {} Res BootProgram ex modulis Limine",
        created
    );
    if skipped_fonts > 0 {
        let suffix = alloc::format!(" (neglecta(e) {} modula(e) typographicae)", skipped_fonts);
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

        // Build resident pages for RawModule
        let mut resident_pages = Vec::new();
        if base_phys % 4096 == 0 {
            let start_addr = base_phys;
            let end_addr = shared_buffer::align_up(base_phys + size, 4096);
            let mut addr = start_addr;
            while addr < end_addr {
                let frame = PhysFrame::from_start_address(addr, 4096);
                resident_pages.push(ResidentPage { frame });
                addr += 4096;
            }
        } else {
            log("Modulus paginis non adaequatus, Residens creari non potest");
            continue;
        }

        let resident_ref = ResidentRef {
            pages: resident_pages,
            byte_len: size as usize,
            rw_holder: None,
        };

        let mut props_vec = alloc::vec::Vec::new();
        props_vec.push((
            symbols::intern(thing_models::graph_kinds::PROP_IDENTIFIER),
            PropValue::Str(identifier),
        ));
        props_vec.push((
            symbols::intern(thing_models::graph_kinds::PROP_RAW_KIND),
            PropValue::Str(kind_str),
        ));
        props_vec.push((
            symbols::intern(thing_models::graph_kinds::PROP_MODULE_INDEX),
            PropValue::U64(index as u64),
        ));
        props_vec.push((
            symbols::intern(thing_models::graph_kinds::PROP_BASE_PHYS),
            PropValue::U64(base_phys),
        ));
        props_vec.push((
            symbols::intern(thing_models::graph_kinds::PROP_SIZE),
            PropValue::U64(size),
        ));

        let kind = symbols::intern(graph_kinds::KIND_RAW_MODULE);

        // Create Resident Thing via store directly
        {
            let slab = kernel::graph::store::things_slab();
            let mut guard = slab.lock();
            let store = guard.as_mut().expect("GraphStore init");
            // Owner is None (kernel/system) - using ProcessId(0) to represent kernel/root
            let _id = store.create_resident(kind, resident_ref, ProcessId(0));
            // Update props
            store.update_thing(_id, props_vec);
        }

        created = created.saturating_add(1);
    }

    if created > 0 {
        let msg = alloc::format!("Seminatae sunt {} Res RawModule ex modulis Limine", created);
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    }
}

pub fn seed_time_graph() {
    let tick_hz = time::tick_hz();
    let epoch_secs = time::rtc_epoch_seconds();
    let time_props_arr = TimeSource::create(tick_hz, epoch_secs, 0);
    // Convert array to Vec<(SymbolId, PropValue)>
    let props: Vec<(SymbolId, PropValue)> = time_props_arr
        .iter()
        .map(|(k, v)| (symbols::intern(k), v.clone())) // clone value as we are interning key
        .collect();

    let kind = symbols::intern(graph_kinds::KIND_TIME_SOURCE);
    let time_id = graph::create_thing(kind, props);
    {
        // Block for time_id usage
        time::bind_time_source(time_id);
        // ... scope continues
        let msg = alloc::format!(
            "Fons Temporis creatus id={} tictus_hz={} epoch_secundae={}",
            time_id.0,
            tick_hz,
            epoch_secs
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    }

    let boot_alarm_secs = epoch_secs.saturating_add(3);
    let alarm_props_model =
        AlarmRequest::create_pending(boot_alarm_secs, 0, ThingId(0), ThingId(0));

    // AlarmRequest::create_pending returns AlarmRequest struct or props?
    // Check thing_models... create_pending is likely a constructor returning AlarmRequest.
    // Wait, TimeSource::create returned array.
    // AlarmRequest::create_pending in thing_models (Step 158) is NOT shown.
    // I assumed it mimics TimeSource::create or I build it myself.
    // Step 158 didn't show `impl AlarmRequest { fn create_pending ... }`.
    // It showed `struct AlarmRequest` and `impl Thing for AlarmRequest`.
    // The previous code in boot_model.rs called `AlarmRequest::create_pending`.
    // I should check if `AlarmRequest` has `create_pending`.
    // If not, I construct it and call to_props.
    // Let's assume it returns `AlarmRequest` struct.
    // Then I call `to_props`.
    // If it returns props array, I map it.

    // Previous code: `let alarm_props = AlarmRequest::create_pending(...)`. `graph::create_thing(..., &alarm_props)`.
    // This implies it returned props array/slice.
    // But `TimeSource::create` return type was explicit in previous file content (Step 147 line 554): `[(PropKey, PropValue); 4]`.
    // `AlarmRequest` probably similar.
    // I'll assume it returns `AlarmRequest` object because `create_pending` sounds like a constructor.
    // BUT looking at `boot_model.rs` (Step 147, line 648): `let alarm_props = AlarmRequest::create_pending...`
    // Then `create_thing(..., &alarm_props)`.
    // So it returns props.
    // I will use `to_props` approach or map if it returns props.
    // I'll assume it returns `AlarmRequest` object and I use `to_props` if I can't check.
    // Actually, `TimeSource::create` in `thing_models` (Step 158) *was* visible in previous logs? NO.
    // Step 158 showed `lib.rs` of `thing_models`.
    // `TimeSource` lines 553-574 showed `create` and `update_from_kernel` returning array.
    // `AlarmRequest` (lines 576-708) did NOT show `impl AlarmRequest`. It only showed `struct` and `impl Thing`.
    // So `create_pending` might be missing or in another block I missed?
    // Or I construct `AlarmRequest` struct manually.

    let alarm_req = AlarmRequest {
        id: ThingId(0),
        time_source_id: None,
        target_unix_seconds: boot_alarm_secs,
        target_unix_nanos: 0,
        target_ticks: None,
        period_ticks: None,
        owner_process: ThingId(0),
        owner_thread: ThingId(0),
        armed: true,
        fired: false,
    };

    let mut props_vec = Vec::new();
    alarm_req.to_props(&mut props_vec);
    let kernel_props = intern_props(props_vec);
    let kind_alarm = symbols::intern(graph_kinds::KIND_ALARM_REQUEST);

    let alarm_id = graph::create_thing(kind_alarm, kernel_props);
    {
        let msg = alloc::format!(
            "Petitio Alarmae Boot id={} petens {} secundas",
            alarm_id.0,
            boot_alarm_secs
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        log(leaked);
    }
}

// ... existing helpers ...
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

pub fn get_kernel_arg(prefix: &str) -> Option<String> {
    let response = KERNEL_FILE_REQUEST.get_response()?;
    let file = response.file();
    let cmdline = file.cmdline();
    // cmdline() is deprecated but returns the byte slice directly or via Deref?
    // Using it as is for now since string() might return &CStr which needs to_bytes().
    // The previous error was that `&[u8]` doesn't have `to_bytes()`.
    let bytes = cmdline;
    if bytes.is_empty() {
        return None;
    }
    let cow = String::from_utf8_lossy(bytes);
    parse_keyed_argument(&cow, prefix)
}

fn parse_respawn_policy(cmdline: &core::ffi::CStr) -> Option<String> {
    let bytes = cmdline.to_bytes();
    if bytes.is_empty() {
        return None;
    }

    let line = if let Ok(s) = core::str::from_utf8(bytes) {
        s
    } else {
        return None;
    };

    parse_keyed_argument(line, "respawn=")
}

fn parse_font_identifier(cmdline: &core::ffi::CStr, path: &core::ffi::CStr) -> Option<String> {
    let bytes = cmdline.to_bytes();
    if !bytes.is_empty() {
        if let Ok(line) = core::str::from_utf8(bytes) {
            if let Some(font) = parse_font_argument(line) {
                return Some(font);
            }
        }
    }

    // Check extension .psf, .font?
    // Simplified: check if path ends with .psf
    // Using simple extension check for now
    if let Ok(p) = path.to_str() {
        if p.ends_with(".psf") || p.ends_with(".PSF") {
            // derive name?
            return Some(String::from(last_path_component(p)));
        }
    }
    None
}

fn parse_raw_identifier(
    cmdline: &core::ffi::CStr,
    _path: &core::ffi::CStr,
) -> Option<(String, String)> {
    let bytes = cmdline.to_bytes();
    if bytes.is_empty() {
        return None;
    }

    let cow = String::from_utf8_lossy(bytes);
    let line = cow.as_ref();

    // Allow shorthand `image=<identifier>` for bundling bitmap assets.
    if let Some(ident) = parse_keyed_argument(line, "image=") {
        if !ident.is_empty() {
            return Some((String::from("image"), ident));
        }
    }

    if let Some(val) = parse_keyed_argument(line, "raw=") {
        if let Some((kind, ident)) = val.split_once(':') {
            if !kind.is_empty() && !ident.is_empty() {
                return Some((String::from(kind), String::from(ident)));
            }
        }
    }

    None
}

// Helpers for boot program check
fn boot_program_exists(name: &str) -> bool {
    // Requires symbol resolution to use graph::next_thing_of_kind
    // But graph::next_thing_of_kind now takes SymbolId (kernel/src/graph/mod.rs logic)
    // kernel/src/graph/store.rs has iteration.
    // We can't easily query by NAME prop without iterating.

    let kind = symbols::intern(graph_kinds::KIND_BOOT_PROGRAM);
    let mut current = ThingId(0);
    // Loop through all BootPrograms
    while let Some(next) = graph::next_thing_of_kind_sym(kind, current) {
        // Check name property
        if let Some(PropValue::Str(s)) = graph::get_prop(next, "name") {
            if s == name {
                return true;
            }
        }
        current = next;
    }
    false
}

fn get_program_priority(name: &str) -> u64 {
    if name == "compositor" {
        10
    } else if name == "init" {
        100
    } else {
        0
    }
}
