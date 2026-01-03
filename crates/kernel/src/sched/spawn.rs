use crate::arch::user_space::{UserPageFlags, UserSpace};
use crate::boot_fs;
use crate::sched::elf::load_elf;
use crate::sched::elf_reloc::{apply_relative_relocations, RelocationError};
use crate::Kernel;
use alloc::format;
use core::cmp::max;
use core::sync::atomic::{AtomicU64, Ordering};

pub struct SpawnResult {
    pub entry: u64,
    pub load_base: u64,
    pub stack_top: u64,
    pub heap_start: u64,
    pub heap_end: u64,
}

pub struct FramebufferMap {
    pub phys: u64,
    pub size: u64,
}

const LOAD_BASE_START: u64 = 0x40_0000_0000;
const LOAD_BASE_STRIDE: u64 = 0x0100_0000; // 16MiB spacing for now

const STACK_OFFSET: u64 = 0x0800_0000;
const STACK_SIZE: u64 = 128 * 1024;

const HEAP_OFFSET: u64 = 0x0100_0000;
const HEAP_SIZE: u64 = 128 * 1024 * 1024; // virtual span

const USER_FB_BASE: u64 = 0x80_0000_0000;

static APP_LOAD_ADDR: AtomicU64 = AtomicU64::new(LOAD_BASE_START);

pub fn spawn_user_elf<A: UserSpace, B: crate::arch::bridge::FullMachineBridge>(
    k: &mut Kernel<B>,
    _arch: &mut A, // Not strictly needed if A functions are static/unsafe, but following signature
    name: &str,
    elf: &[u8],
    fb: Option<FramebufferMap>,
) -> Option<SpawnResult> {
    let load_base = APP_LOAD_ADDR.fetch_add(LOAD_BASE_STRIDE, Ordering::Relaxed);
    k.bridge
        .log(&format!("SPAWN: name={} base={:#x}\n", name, load_base));

    let mut root = unsafe { A::create_root() };

    // Load ELF segments
    let mut max_loaded = load_base;
    let loaded = load_elf(elf, load_base, |vaddr, bytes, flags| {
        unsafe {
            A::write_bytes(&mut root, load_base + vaddr, bytes);
        }
        max_loaded = max(max_loaded, load_base + vaddr + bytes.len() as u64);
        Ok(())
    })?;

    max_loaded = max(max_loaded, loaded.max_vaddr);

    // Apply relocations
    match apply_relative_relocations(elf, load_base, |addr, bytes| {
        unsafe {
            A::write_bytes(&mut root, addr, bytes);
        }
        Ok(())
    }) {
        Ok(applied) => {
            k.bridge
                .log(&format!("SPAWN: applied {} relocations\n", applied));
        }
        Err(e) => {
            k.bridge.log("SPAWN: relocation failure\n");
            let msg = match e {
                RelocationError::UnsupportedType(t) => {
                    format!("unsupported relocation type: {}\n", t)
                }
                RelocationError::MissingSegment => format!("relocation segment missing\n"),
                RelocationError::WriteFault => format!("relocation write fault\n"),
                RelocationError::Parse => format!("relocation parse error\n"),
            };
            k.bridge.log(&msg);
            return None;
        }
    }

    // Map stack pages
    let stack_top = load_base + STACK_OFFSET + STACK_SIZE;
    let stack_bottom = load_base + STACK_OFFSET;
    {
        let mut addr = stack_bottom;
        while addr < stack_top {
            let phys = unsafe { A::alloc_frame_4k() };
            unsafe {
                A::map_4k(
                    &mut root,
                    addr,
                    phys,
                    UserPageFlags {
                        writable: true,
                        executable: false,
                        device: false,
                        user: true,
                    },
                );
            }
            addr += 4096;
        }
    }

    // Map heap pages (lazily or fully - fully for now as requested)
    let heap_start = load_base + HEAP_OFFSET;
    let heap_end = heap_start + HEAP_SIZE;
    {
        // For now map full span as requested "either map first N pages (debug) or map full span"
        // But 128MB is a lot to map fully if we don't have enough memory.
        // The previous implementation mapped 1MB. The plan says "HEAP_SIZE: u64 = 128 * 1024 * 1024; // virtual span".
        // And "Map heap pages lazily is allowed, but for now keep your existing approach".
        // Existing approach mapped 1MB.
        // Let's map 1MB for now to be safe, but define the virtual span as 128MB.
        let map_size = 1 * 1024 * 1024;
        let mut addr = heap_start;
        while addr < heap_start + map_size {
            let phys = unsafe { A::alloc_frame_4k() };
            unsafe {
                A::map_4k(
                    &mut root,
                    addr,
                    phys,
                    UserPageFlags {
                        writable: true,
                        executable: false,
                        device: false,
                        user: true,
                    },
                );
            }
            addr += 4096;
        }
    }

    // Map framebuffer if present
    if let Some(ref fb_map) = fb {
        let mut offs = 0;
        while offs < fb_map.size {
            unsafe {
                A::map_4k(
                    &mut root,
                    USER_FB_BASE + offs,
                    fb_map.phys + offs,
                    UserPageFlags {
                        writable: true,
                        executable: false,
                        device: true,
                        user: true,
                    },
                );
            }
            offs += 4096;
        }
    }

    // Sync I-cache
    unsafe {
        A::sync_icache(load_base, (max_loaded - load_base) as usize + 4096);
    }

    let entry = load_base + loaded.entry_point;

    // Register process
    k.scheduler.spawn(
        &k.bridge,
        name,
        entry,
        stack_top - 16, // safety margin
        heap_start,
        heap_start,
        heap_end,
    );

    boot_fs::register_boot_process(k, name, entry);

    k.bridge.log(&format!(
        "SPAWN: name={} base={:#x} entry={:#x} stack={:#x} heap={:#x} fb={}\n",
        name,
        load_base,
        entry,
        stack_top,
        heap_start,
        if fb.is_some() { "yes" } else { "no" }
    ));

    Some(SpawnResult {
        entry,
        load_base,
        stack_top,
        heap_start,
        heap_end,
    })
}
