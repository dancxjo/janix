use crate::bridge::{FullMachineBridge, UserAddressSpace, UserPageFlags};
use crate::boot_fs;
use crate::sched::elf::{apply_relative_relocations, find_symbol, load_elf, RelocationError};
use crate::Kernel;
use alloc::format;
use core::cmp::max;
use core::sync::atomic::{AtomicU64, Ordering};
use xmas_elf::program::Flags;

pub struct SpawnSpec<'a> {
    pub name: &'a str,
    pub elf: &'a [u8],
    pub spawn: bool,
    pub fb: Option<FramebufferSpec>,
}

pub struct FramebufferSpec {
    pub phys: u64,
    pub size: u64,
    pub user_virt: u64,
}

pub struct SpawnLayout {
    pub load_base: u64,
    pub stack_bottom: u64,
    pub stack_top: u64,
    pub heap_start: u64,
    pub heap_end: u64,
}

pub const USER_FB_BASE: u64 = 0x80_0000_0000;
pub const USER_LOAD_BASE_START: u64 = 0x40_0000_0000;
pub const USER_STACK_OFFSET: u64 = 0x0800_0000;
pub const USER_HEAP_OFFSET: u64 = 0x0100_0000;

const USER_STACK_SIZE: u64 = 128 * 1024;
const USER_HEAP_SIZE: u64 = 1 * 1024 * 1024;
const LOAD_STRIDE: u64 = 0x1000_0000;
const PAGE_SIZE: u64 = 4096;

static LOAD_BASE_CURSOR: AtomicU64 = AtomicU64::new(USER_LOAD_BASE_START);

fn segment_flags_to_page(flags: Flags) -> UserPageFlags {
    let mut out = UserPageFlags::READ | UserPageFlags::USER;
    if flags.is_write() {
        out |= UserPageFlags::WRITE;
    }
    if flags.is_execute() {
        out |= UserPageFlags::EXEC;
    }
    out
}

fn map_range<B: UserAddressSpace>(
    root: &mut B::Root,
    start: u64,
    end: u64,
    flags: UserPageFlags,
) -> bool {
    let start_page = start & !(PAGE_SIZE - 1);
    let end_page = (end + (PAGE_SIZE - 1)) & !(PAGE_SIZE - 1);
    let mut addr = start_page;

    while addr < end_page {
        let phys = unsafe { B::alloc_frame() };
        if phys == 0 {
            return false;
        }
        unsafe { B::map_user_page(root, addr, phys, flags) };
        addr += PAGE_SIZE;
    }
    true
}

pub fn spawn_user_elf<B: FullMachineBridge>(
    k: &mut Kernel<B>,
    spec: SpawnSpec<'_>,
) -> Option<(u64, SpawnLayout)> {
    if !spec.spawn {
        return None;
    }

    let load_base = LOAD_BASE_CURSOR.fetch_add(LOAD_STRIDE, Ordering::Relaxed);
    k.bridge
        .log(&format!("spawn_user_elf name={} base={:#x}\n", spec.name, load_base));

    let mut root = unsafe { B::create_user_root() };

    let mut max_loaded = load_base;
    let loaded = load_elf(spec.elf, load_base, |vaddr, bytes, flags| {
        let mut page_flags = segment_flags_to_page(flags);
        // Allow writes during load; policy tightening can happen later.
        page_flags |= UserPageFlags::WRITE;
        unsafe {
            B::write_user(
                &mut root,
                load_base + vaddr,
                bytes,
                page_flags | UserPageFlags::USER,
            );
        }
        max_loaded = max(max_loaded, load_base + vaddr + bytes.len() as u64);
        Ok(())
    })?;

    max_loaded = max(max_loaded, loaded.max_vaddr);

    match apply_relative_relocations(spec.elf, load_base, |addr, bytes| {
        unsafe {
            B::write_user(
                &mut root,
                addr,
                bytes,
                UserPageFlags::USER | UserPageFlags::READ | UserPageFlags::WRITE,
            );
        }
        Ok(())
    }) {
        Ok(applied) => {
            k.bridge
                .log(&format!("spawn_user_elf: applied {} relocations\n", applied));
        }
        Err(e) => {
            k.bridge.log("spawn_user_elf: relocation failure\n");
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

    let stack_bottom = load_base + USER_STACK_OFFSET;
    let stack_top = stack_bottom + USER_STACK_SIZE;
    if !map_range::<B>(
        &mut root,
        stack_bottom,
        stack_top,
        UserPageFlags::USER | UserPageFlags::READ | UserPageFlags::WRITE,
    ) {
        k.bridge.log("spawn_user_elf: failed to map stack\n");
        return None;
    }
    k.bridge.log("spawn_user_elf: mapped stack\n");

    let heap_start = load_base + USER_HEAP_OFFSET;
    let heap_end = heap_start + USER_HEAP_SIZE;
    if !map_range::<B>(
        &mut root,
        heap_start,
        heap_end,
        UserPageFlags::USER | UserPageFlags::READ | UserPageFlags::WRITE,
    ) {
        k.bridge.log("spawn_user_elf: failed to map heap\n");
        return None;
    }
    k.bridge.log("spawn_user_elf: mapped heap\n");

    if let Some(fb) = spec.fb {
        let mut offs = 0;
        while offs < fb.size {
            unsafe {
                B::map_user_page(
                    &mut root,
                    fb.user_virt + offs,
                    fb.phys + offs,
                    UserPageFlags::USER
                        | UserPageFlags::READ
                        | UserPageFlags::WRITE
                        | UserPageFlags::DEVICE,
                );
            }
            offs += PAGE_SIZE;
        }
        k.bridge.log("spawn_user_elf: mapped framebuffer\n");
    }

    unsafe {
        B::sync_icache(load_base, (max_loaded - load_base) as usize);
        B::activate_user_root(&root);
    }

    let mut entry = load_base + loaded.entry_point;
    if let Some(offset) = find_symbol(spec.elf, "thingos_driver_init") {
        entry = load_base + offset;
        k.bridge
            .log("spawn_user_elf: using thingos_driver_init override\n");
    }

    k.bridge
        .log(&format!("spawn_user_elf entry={:#x}\n", entry));

    k.scheduler.spawn(
        &k.bridge,
        spec.name,
        entry,
        stack_top - 8,
        heap_start,
        heap_start,
        heap_end,
    );

    let tid = k.scheduler.threads.len() as u64;
    k.bridge
        .log(&format!("spawn_user_elf: scheduled tid={}\n", tid));

    boot_fs::register_boot_process(k, spec.name, entry);

    let layout = SpawnLayout {
        load_base,
        stack_bottom,
        stack_top,
        heap_start,
        heap_end,
    };

    Some((entry, layout))
}
