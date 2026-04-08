use crate::memory;
use crate::{
    BootModuleDesc, BootRuntime, BootTasking, FrameAllocatorHook, MapKind, MapPerms, UserEntry,
};
use abi::types::StackInfo;
use abi::vm::{VmBackingKind, VmMapFlags, VmProt, VmRegionInfo};
use core::cmp::{max, min};

struct LoaderAllocHook;
impl FrameAllocatorHook for LoaderAllocHook {
    fn alloc_frame(&self) -> Option<u64> {
        memory::alloc_frame()
    }
}

pub fn load_module<R: BootRuntime>(
    rt: &R,
    aspace: <R::Tasking as BootTasking>::AddressSpace,
    module: &BootModuleDesc,
) -> Option<(UserEntry, StackInfo, alloc::vec::Vec<VmRegionInfo>)> {
    crate::kinfo!(
        "LOADER: Loading module '{}' (len={})",
        module.name,
        module.bytes.len()
    );
    if module.bytes.len() >= 16 {
        crate::kinfo!("  Header: {:02x?}", &module.bytes[0..16]);
    }

    let load_addr: u64 = 0x200000;
    let stack_top = 0x0080_0000;
    let reserve_bytes = 2 * 1024 * 1024;
    let guard_pages = 1usize;
    let initial_commit_bytes = 64 * 1024;
    let grow_chunk_bytes = 64 * 1024;

    let hook = LoaderAllocHook;
    let data_perms = MapPerms {
        user: true,
        read: true,
        write: true,
        exec: false,
        kind: MapKind::Normal,
    };

    let page_size = rt.page_size() as u64;
    let mut entry_pc = load_addr;
    let mut regions = alloc::vec::Vec::new();

    // 1. Map ELF segments when available; otherwise fall back to a simple RWX layout.
    if let Some(mut elf) = parse_elf64(module.bytes) {
        // Sort segments by vaddr to ensure we process overlapping pages sequentially
        elf.load_segments.sort_by(|a, b| a.vaddr.cmp(&b.vaddr));

        let load_bias = load_addr.saturating_sub(elf.min_vaddr);
        entry_pc = elf.entry.saturating_add(load_bias);

        let mut last_virt_page = u64::MAX;
        let mut last_phys_page = 0;
        let mut last_perms = MapPerms {
            user: false,
            read: false,
            write: false,
            exec: false,
            kind: MapKind::Normal,
        };

        for ph in elf.load_segments.iter() {
            let seg_vaddr = ph.vaddr.saturating_add(load_bias);
            let seg_mem_end = seg_vaddr.saturating_add(ph.memsz);
            if ph.memsz == 0 {
                continue;
            }

            let seg_start = align_down_u64(seg_vaddr, page_size);
            let seg_end = align_up_u64(seg_mem_end, page_size);
            let mut perms = MapPerms {
                user: true,
                read: ph.read || ph.write || ph.exec,
                write: ph.write,
                exec: ph.exec,
                kind: MapKind::Normal,
            };
            crate::kinfo!("Segment: vaddr={:x} exec={}", seg_vaddr, perms.exec);

            // Record mapping
            let mut prot = VmProt::USER;
            if perms.read {
                prot |= VmProt::READ;
            }
            if perms.write {
                prot |= VmProt::WRITE;
            }
            if perms.exec {
                prot |= VmProt::EXEC;
            }

            regions.push(VmRegionInfo {
                start: seg_start as usize,
                end: seg_end as usize,
                prot,
                flags: VmMapFlags::empty(),
                backing_kind: VmBackingKind::Anonymous,
                _reserved: [0; 7],
            });

            let mut virt = seg_start;
            while virt < seg_end {
                let phys;
                let mut reuse_page = false;
                let mut page_perms = perms;

                if virt == last_virt_page {
                    // Overlap detected! Reuse the previous page and merge permissions.
                    phys = last_phys_page;
                    reuse_page = true;
                    page_perms = match merge_perms(last_perms, perms) {
                        Ok(p) => p,
                        Err(e) => {
                            crate::kinfo!("ERROR: {} at {:x}", e, virt);
                            return None;
                        }
                    };
                    crate::kinfo!(
                        "  Overlap at {:x}: merging perms to r={} w={} x={}",
                        virt,
                        page_perms.read,
                        page_perms.write,
                        page_perms.exec
                    );
                } else {
                    // New page
                    phys = memory::alloc_frame().expect("OOM loading module segment");
                }

                let hhdm_virt = phys + rt.phys_to_virt_offset();

                if !reuse_page {
                    unsafe {
                        core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size as usize);
                    }
                }

                let page_end = virt.saturating_add(page_size);
                let file_start = seg_vaddr;
                let file_end = seg_vaddr.saturating_add(ph.filesz);
                let copy_start = max(virt, file_start);
                let copy_end = min(page_end, file_end);

                if copy_start < copy_end {
                    let src_off = ph.offset.saturating_add(copy_start - seg_vaddr);
                    let len = (copy_end - copy_start) as usize;
                    let dst = (hhdm_virt + (copy_start - virt)) as *mut u8;
                    if src_off as usize + len <= module.bytes.len() {
                        unsafe {
                            core::ptr::copy_nonoverlapping(
                                module.bytes.as_ptr().add(src_off as usize),
                                dst,
                                len,
                            );
                        }

                        if copy_start <= 0x201420 && 0x201420 < copy_end {
                            let off_in_page = (0x201420 - copy_start) as usize;
                            unsafe {
                                let bytes = core::slice::from_raw_parts(dst.add(off_in_page), 8);
                                crate::kinfo!("  COPIED at 0x201420: {:02x?}", bytes);
                            }
                        }
                    } else {
                        return None;
                    }
                }

                rt.tasking()
                    .map_page(aspace, virt, phys, page_perms, MapKind::Normal, &hook)
                    .unwrap();

                last_virt_page = virt;
                last_phys_page = phys;
                last_perms = page_perms;

                virt += page_size;
            }
        }
    } else {
        // Hardcoded load address for simple PIE/or-not-PIE loading.
        // Fixed address 0x200000 is fine for the main executable today.
        let text_perms = MapPerms {
            user: true,
            read: true,
            write: true,
            exec: true,
            kind: MapKind::Normal,
        };
        let mut virt = load_addr as u64;

        // Record mapping
        let prot = VmProt::USER | VmProt::READ | VmProt::WRITE | VmProt::EXEC;
        let len = align_up_u64(module.bytes.len() as u64, page_size);
        regions.push(VmRegionInfo {
            start: load_addr as usize,
            end: (load_addr + len) as usize,
            prot,
            flags: VmMapFlags::empty(),
            backing_kind: VmBackingKind::Anonymous,
            _reserved: [0; 7],
        });

        if page_size == 0 {
            panic!("LOADER: page_size is zero!");
        }

        for chunk in module.bytes.chunks(page_size as usize) {
            let phys = memory::alloc_frame().expect("OOM loading module");
            let hhdm_virt = phys + rt.phys_to_virt_offset();
            unsafe {
                core::ptr::copy_nonoverlapping(chunk.as_ptr(), hhdm_virt as *mut u8, chunk.len());
                if chunk.len() < page_size as usize {
                    core::ptr::write_bytes(
                        (hhdm_virt as *mut u8).add(chunk.len()),
                        0,
                        page_size as usize - chunk.len(),
                    );
                }
            }

            rt.tasking()
                .map_page(aspace, virt, phys, text_perms, MapKind::Normal, &hook)
                .unwrap();
            virt += page_size;
        }
    }

    // 3. Map Stack (guard + reserve with initial commit)
    let guard_bytes = (guard_pages as u64).saturating_mul(page_size);
    let reserve_bytes = align_up_u64(reserve_bytes as u64, page_size);
    let total = guard_bytes.saturating_add(reserve_bytes);
    let reserve_end = stack_top as u64;
    let base = reserve_end.saturating_sub(total);
    let guard_start = base;
    let guard_end = base.saturating_add(guard_bytes);
    let reserve_start = guard_end;
    let commit_len = align_up_u64(initial_commit_bytes as u64, page_size);
    let commit_start = reserve_end.saturating_sub(commit_len);

    // Record stack mapping (committed part)
    regions.push(VmRegionInfo {
        start: commit_start as usize,
        end: reserve_end as usize,
        prot: VmProt::USER | VmProt::READ | VmProt::WRITE,
        flags: VmMapFlags::empty(),
        backing_kind: VmBackingKind::Anonymous,
        _reserved: [0; 7],
    });

    let mut virt = commit_start;
    while virt < reserve_end {
        let phys = memory::alloc_frame().expect("OOM loading stack");
        let hhdm_virt = phys + rt.phys_to_virt_offset();
        unsafe {
            core::ptr::write_bytes(hhdm_virt as *mut u8, 0, page_size as usize);
        }
        rt.tasking()
            .map_page(aspace, virt, phys, data_perms, MapKind::Normal, &hook)
            .unwrap();
        virt += page_size;
    }

    // Ensure instruction cache sees freshly loaded code
    rt.icache_invalidate();

    let stack_info = StackInfo {
        guard_start: guard_start as usize,
        guard_end: guard_end as usize,
        reserve_start: reserve_start as usize,
        reserve_end: reserve_end as usize,
        committed_start: commit_start as usize,
        grow_chunk_bytes,
    };

    Some((
        UserEntry {
            entry_pc: entry_pc as usize,
            user_sp: stack_top,
            arg0: 0,
        },
        stack_info,
        regions,
    ))
}

fn align_up_u64(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}

fn align_down_u64(value: u64, align: u64) -> u64 {
    if align == 0 {
        return value;
    }
    value & !(align - 1)
}

struct ElfLoadSegment {
    offset: u64,
    vaddr: u64,
    filesz: u64,
    memsz: u64,
    read: bool,
    write: bool,
    exec: bool,
}

struct ElfInfo {
    entry: u64,
    min_vaddr: u64,
    load_segments: alloc::vec::Vec<ElfLoadSegment>,
}

fn parse_elf64(bytes: &[u8]) -> Option<ElfInfo> {
    if bytes.len() < 64 {
        return None;
    }
    if &bytes[0..4] != b"\x7fELF" {
        return None;
    }
    if bytes[4] != 2 || bytes[5] != 1 {
        return None;
    }
    let e_entry = read_u64(bytes, 24)?;
    let e_phoff = read_u64(bytes, 32)?;
    let e_phentsize = read_u16(bytes, 54)? as u64;
    let e_phnum = read_u16(bytes, 56)? as u64;
    if e_phoff == 0 || e_phentsize == 0 || e_phnum == 0 {
        return None;
    }

    let mut min_vaddr = u64::MAX;
    let mut load_segments = alloc::vec::Vec::new();
    for i in 0..e_phnum {
        let off = e_phoff.saturating_add(i.saturating_mul(e_phentsize)) as usize;
        let p_type = read_u32(bytes, off)?;
        if p_type != 1 {
            continue;
        }
        let p_flags = read_u32(bytes, off + 4)?;
        let p_offset = read_u64(bytes, off + 8)?;
        let p_vaddr = read_u64(bytes, off + 16)?;
        let p_filesz = read_u64(bytes, off + 32)?;
        let p_memsz = read_u64(bytes, off + 40)?;

        min_vaddr = min(min_vaddr, p_vaddr);
        load_segments.push(ElfLoadSegment {
            offset: p_offset,
            vaddr: p_vaddr,
            filesz: p_filesz,
            memsz: p_memsz,
            read: (p_flags & 0x4) != 0,
            write: (p_flags & 0x2) != 0,
            exec: (p_flags & 0x1) != 0,
        });
    }

    if load_segments.is_empty() || min_vaddr == u64::MAX {
        return None;
    }

    Some(ElfInfo {
        entry: e_entry,
        min_vaddr,
        load_segments,
    })
}

fn read_u16(bytes: &[u8], off: usize) -> Option<u16> {
    let slice = bytes.get(off..off + 2)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}

fn read_u32(bytes: &[u8], off: usize) -> Option<u32> {
    let slice = bytes.get(off..off + 4)?;
    Some(u32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

fn read_u64(bytes: &[u8], off: usize) -> Option<u64> {
    let slice = bytes.get(off..off + 8)?;
    Some(u64::from_le_bytes([
        slice[0], slice[1], slice[2], slice[3], slice[4], slice[5], slice[6], slice[7],
    ]))
}

fn merge_perms(last: MapPerms, next: MapPerms) -> Result<MapPerms, &'static str> {
    let merged = MapPerms {
        user: last.user || next.user,
        read: last.read || next.read,
        write: last.write || next.write,
        exec: last.exec || next.exec,
        kind: last.kind,
    };

    // Enforce W^X: never produce RWX
    if merged.write && merged.exec {
        return Err("Permission conflict: merging results in RWX (W^X violation)");
    }

    Ok(merged)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_perms() {
        let r = MapPerms {
            user: true,
            read: true,
            write: false,
            exec: false,
            kind: MapKind::Normal,
        };
        let rw = MapPerms {
            user: true,
            read: true,
            write: true,
            exec: false,
            kind: MapKind::Normal,
        };
        let rx = MapPerms {
            user: true,
            read: true,
            write: false,
            exec: true,
            kind: MapKind::Normal,
        };
        let x = MapPerms {
            user: true,
            read: false,
            write: false,
            exec: true,
            kind: MapKind::Normal,
        };

        // RX + RW -> Error
        assert!(merge_perms(rx, rw).is_err());

        // RX + R -> RX
        let res = merge_perms(rx, r).expect("RX + R failed");
        assert!(res.read && !res.write && res.exec);

        // RW + R -> RW
        let res = merge_perms(rw, r).expect("RW + R failed");
        assert!(res.read && res.write && !res.exec);

        // RX + X -> RX (normalize)
        let res = merge_perms(rx, x).expect("RX + X failed");
        assert!(res.read && !res.write && res.exec);

        // R + R -> R
        let res = merge_perms(r, r).expect("R + R failed");
        assert!(res.read && !res.write && !res.exec);
    }
}
