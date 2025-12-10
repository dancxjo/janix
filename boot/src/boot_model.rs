extern crate alloc;

use alloc::string::String;
use kernel_core::log;
use kernel_core::memory::{BootFrameAllocator, init_frame_pool};
use kernel_core::model;
use limine::memory_map::EntryType;
use limine::request::{HhdmRequest, MemoryMapRequest, ModuleRequest, MpRequest};

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
        hhdm.offset()
    } else {
        0
    };

    let mut heap_initialized = false;
    let mut boot_allocator = BootFrameAllocator::new();

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        let mut base = entry.base;
        let mut len = entry.length;

        if !heap_initialized && len >= 2 * 1024 * 1024 {
            let heap_size = 1024 * 1024; // 1 MiB
            let heap_start_phys = base;
            let heap_start_virt = (heap_start_phys as u64 + hhdm_offset) as usize;

            unsafe {
                crate::heap::KERNEL_ALLOCATOR.init(heap_start_virt, heap_size);
            }

            log("Initialized kernel heap (1MiB)");

            base += heap_size as u64;
            len -= heap_size as u64;
            heap_initialized = true;
        }

        boot_allocator.add_region(base, len);

        let frame_size = 4096;

        // For now: one pool per usable region.
        let _pool = model::create_frame_pool(base, base + len, frame_size);

        // In future, you can optionally explode this into many PhysFrame Things.
    }

    init_frame_pool(boot_allocator);

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

pub fn seed_program_images_from_limine() {
    let Some(response) = MODULE_REQUEST.get_response() else {
        log("No Limine modules found for ProgramImage seeding");
        return;
    };

    let hhdm_offset = HHDM_REQUEST
        .get_response()
        .map(|resp| resp.offset())
        .unwrap_or(0);

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = derive_module_identifier((*module).string(), (*module).path(), index);
        let virt_addr = (*module).addr() as u64;
        let base_phys = virt_addr.saturating_sub(hhdm_offset);
        let size = (*module).size() as u64;
        if kernel_core::model::create_program_image(&identifier, index as u64, base_phys, size)
            .is_none()
        {
            log("Failed to create ProgramImage Thing");
        }
    }
}

fn derive_module_identifier(cmdline: &core::ffi::CStr, path: &core::ffi::CStr, index: usize) -> String {
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
    line.split_whitespace()
        .find(|arg| arg.starts_with("program="))
        .and_then(|arg| {
            let ident = arg.trim_start_matches("program=");
            (!ident.is_empty()).then(|| String::from(ident))
        })
}
