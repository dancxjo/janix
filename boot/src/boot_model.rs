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

    for (index, module) in response.modules().iter().enumerate() {
        let identifier = derive_module_identifier((*module).cmdline(), (*module).path(), index);
        let base_phys = (*module).addr() as u64;
        let size = (*module).size() as u64;
        if kernel_core::model::create_program_image(&identifier, index as u64, base_phys, size)
            .is_none()
        {
            log("Failed to create ProgramImage Thing");
        }
    }
}

fn derive_module_identifier(cmdline: &[u8], path: &core::ffi::CStr, index: usize) -> String {
    if let Ok(line) = core::str::from_utf8(cmdline) {
        if let Some(arg) = line
            .split_whitespace()
            .find(|arg| arg.starts_with("program="))
        {
            let ident = arg.trim_start_matches("program=");
            if !ident.is_empty() {
                return String::from(ident);
            }
        }
        if !line.is_empty() {
            return String::from(last_path_component(line));
        }
    }

    if let Ok(pstr) = path.to_str() {
        let component = last_path_component(pstr);
        if !component.is_empty() {
            return String::from(component);
        }
    } else if let Ok(s) = core::str::from_utf8(path.to_bytes()) {
        let component = last_path_component(s);
        if !component.is_empty() {
            return String::from(component);
        }
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
