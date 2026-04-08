//! Bytespace resource handlers.

use crate::BootRuntime;
use crate::root::RootMsg;
use crate::root::resources::{ResourceHandle, bytespace};
use crate::root::resources::bytespace::Provenance;
use core::sync::atomic::Ordering;
use alloc::collections::BTreeMap;

pub type HandlerResult = (i32, u64);

pub struct BytespaceManager {
    next_id: u64,
    pub spaces: BTreeMap<u64, ResourceHandle>,
}

impl BytespaceManager {
    pub fn new() -> Self {
        Self {
            next_id: 1000000,
            spaces: BTreeMap::new(),
        }
    }
    pub fn alloc(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub fn handle_bytespace_create<R: BootRuntime>(
    manager: &mut BytespaceManager,
    len: u64,
    _flags: u64,
    _format: u64,
) -> HandlerResult {
    let rt = crate::runtime::<R>();
    let hhdm_offset = rt.phys_to_virt_offset();
    let alloc_len = if len == 0 { 1 } else { len };

    if let Some(handle) = bytespace::create(alloc_len as usize, hhdm_offset) {
        let id = manager.alloc();
        manager.spaces.insert(id, ResourceHandle::Bytespace(handle));
        (0, id)
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_create_from_ptr<R: BootRuntime>(
    manager: &mut BytespaceManager,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    let rt = crate::runtime::<R>();
    let hhdm_offset = rt.phys_to_virt_offset();
    let id = manager.alloc();
    let handle = bytespace::create_from_ptr(ptr as usize, len as usize, hhdm_offset, Provenance::Boot);
    manager.spaces.insert(id, ResourceHandle::Bytespace(handle));
    (0, id)
}

pub fn handle_bytespace_write(
    manager: &mut BytespaceManager,
    id: u64,
    offset: u64,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let lock = handle.lock();
        if (offset + len) as usize <= lock.len {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    ptr as *const u8,
                    (lock.kernel_va as *mut u8).add(offset as usize),
                    len as usize,
                );
            }
            (0, len)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_read(
    manager: &mut BytespaceManager,
    id: u64,
    offset: u64,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let lock = handle.lock();
        if (offset + len) as usize <= lock.len {
            unsafe {
                core::ptr::copy_nonoverlapping(
                    (lock.kernel_va as *const u8).add(offset as usize),
                    ptr as *mut u8,
                    len as usize,
                );
            }
            (0, len)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_info(manager: &mut BytespaceManager, msg: &RootMsg, id: u64) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let lock = handle.lock();
        if let Some(reply) = msg.reply.as_ref() {
            reply.p0.store(lock.phys_base, Ordering::Relaxed);
            reply.p1.store(lock.flags, Ordering::Relaxed);
        }
        (0, lock.len as u64)
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_map(manager: &mut BytespaceManager, msg: &RootMsg, id: u64, tid: u64) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let lock = handle.lock();
        let user_va = crate::memory::alloc_user_va(lock.len);
        bytespace::record_mapping(id, tid, user_va, lock.len);
        if let Some(reply) = msg.reply.as_ref() {
            reply.p0.store(lock.phys_base, Ordering::Relaxed);
            reply.p1.store(lock.page_count as u64, Ordering::Relaxed);
        }
        (0, user_va)
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_unmap(id: u64, user_va: u64, tid: u64) -> HandlerResult {
    if let Some(_mapping) = bytespace::remove_mapping(id, tid, user_va) {
        (0, 0)
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_phys(manager: &mut BytespaceManager, msg: &RootMsg, id: u64) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let lock = handle.lock();
        if let Some(reply) = msg.reply.as_ref() {
            reply.p0.store(lock.len as u64, Ordering::Relaxed);
        }
        (0, lock.phys_base)
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_truncate(manager: &mut BytespaceManager, id: u64, new_len: u64) -> HandlerResult {
    if let Some(ResourceHandle::Bytespace(handle)) = manager.spaces.get(&id) {
        let mut lock = handle.lock();
        let new_len = new_len as usize;
        if new_len <= lock.page_count * 4096 {
            lock.len = new_len;
            (0, 0)
        } else {
            (-22, 0)
        }
    } else {
        (-2, 0)
    }
}
