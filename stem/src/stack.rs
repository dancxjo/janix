use abi::errors::Errno;
use abi::vm::{VmBacking, VmMapFlags, VmMapReq, VmProt};
use abi::types::StackInfo;

use crate::vm::vm_map;

const PAGE_SIZE: usize = 4096;

fn align_up(value: usize, align: usize) -> usize {
    if align == 0 {
        return value;
    }
    (value + align - 1) & !(align - 1)
}

#[derive(Clone, Copy, Debug)]
pub struct StackSpec {
    pub reserve_bytes: usize,
    pub initial_commit_bytes: usize,
    pub guard_pages: usize,
    pub grow_chunk_bytes: usize,
}

impl Default for StackSpec {
    fn default() -> Self {
        Self {
            reserve_bytes: 2 * 1024 * 1024,
            initial_commit_bytes: 64 * 1024,
            guard_pages: 1,
            grow_chunk_bytes: 64 * 1024,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Stack {
    pub sp: *mut u8,
    pub reserve_start: *mut u8,
    pub reserve_end: *mut u8,
    pub committed_start: *mut u8,
    pub guard_start: *mut u8,
    pub guard_end: *mut u8,
    pub info: StackInfo,
}

impl Stack {
    pub fn alloc_growing_stack(spec: StackSpec) -> Result<Self, Errno> {
        let guard_bytes = spec.guard_pages.saturating_mul(PAGE_SIZE);
        let reserve_bytes = align_up(spec.reserve_bytes, PAGE_SIZE);
        let total = guard_bytes.saturating_add(reserve_bytes);

        let reserve_req = VmMapReq {
            addr_hint: 0,
            len: total,
            prot: VmProt::USER,
            flags: VmMapFlags::GUARD | VmMapFlags::PRIVATE,
            backing: VmBacking::Anonymous { zeroed: true },
        };
        let reserve_resp = vm_map(&reserve_req)?;
        let base = reserve_resp.addr;

        let guard_start = base;
        let guard_end = base.saturating_add(guard_bytes);
        let reserve_start = guard_end;
        let reserve_end = base.saturating_add(total);

        let commit_len = align_up(spec.initial_commit_bytes, PAGE_SIZE);
        let commit_start = reserve_end.saturating_sub(commit_len);

        let commit_req = VmMapReq {
            addr_hint: commit_start,
            len: commit_len,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::FIXED | VmMapFlags::PRIVATE,
            backing: VmBacking::Anonymous { zeroed: true },
        };
        let _ = vm_map(&commit_req)?;

        let info = StackInfo {
            guard_start,
            guard_end,
            reserve_start,
            reserve_end,
            committed_start: commit_start,
            grow_chunk_bytes: align_up(spec.grow_chunk_bytes, PAGE_SIZE),
        };

        Ok(Stack {
            sp: reserve_end as *mut u8,
            reserve_start: reserve_start as *mut u8,
            reserve_end: reserve_end as *mut u8,
            committed_start: commit_start as *mut u8,
            guard_start: guard_start as *mut u8,
            guard_end: guard_end as *mut u8,
            info,
        })
    }

    pub fn handle_stack_fault(_fault_addr: usize) -> Result<(), Errno> {
        Err(Errno::NotSupported)
    }
}
