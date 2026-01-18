//! Boot Register Handler
//!
//! Handles registration of initial resources (like Memory/ACPI tables) into the Graph.

use crate::root::{enqueue, RootOp, SymbolShell};
use abi::wire::{ThingId, SymbolId};
use alloc::string::String;
use core::sync::atomic::Ordering;
use crate::boot_info::BootSyscallInfo;

// Re-export BootSyscallInfo as BootInfo for compatibility with lib.rs
pub use crate::boot_info::BootSyscallInfo as BootInfo;

pub struct Inventory {
    pub host: ThingId,
    pub kernel: ThingId,
    pub root: ThingId,
}

pub fn register_boot_resources() -> ThingId {
    let mut mem_map_id = ThingId::default();

    let reply = enqueue(RootOp::CreateNode {
        kind: SymbolShell::Static("mem.Map"),
        out_ptr: &mut mem_map_id as *mut _ as u64,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
    mem_map_id
}

pub fn register_range(base: u64, len: u64, type_name: &str) {
    let mut id = ThingId::default();

    let mut kind_id = SymbolId::default();
    let reply = enqueue(RootOp::Intern {
        name: String::from(type_name),
        out_ptr: &mut kind_id as *mut _ as u64,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }

    let reply = enqueue(RootOp::CreateNode {
        kind: SymbolShell::Id(kind_id),
        out_ptr: &mut id as *mut _ as u64,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }

    let mut val = [0u8; 16];
    val[0..8].copy_from_slice(&base.to_le_bytes());
    let reply = enqueue(RootOp::PropSet {
        id,
        key: SymbolShell::Str(String::from("phys_base")),
        value: val,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }

    let mut val_len = [0u8; 16];
    val_len[0..8].copy_from_slice(&len.to_le_bytes());
    let reply = enqueue(RootOp::PropSet {
        id,
        key: SymbolShell::Str(String::from("len")),
        value: val_len,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn register_all(info: &BootSyscallInfo) -> Inventory {
    let _mem_map = register_boot_resources();

    for range in info.memory_map {
        register_range(range.start, range.end - range.start, "mem.Range");
    }

    // Create dummy inventory for lib.rs
    // TODO: Actually create Host/Kernel/Root nodes
    Inventory {
        host: ThingId::default(),
        kernel: ThingId::default(),
        root: ThingId::default(),
    }
}

pub fn link_boot_resource(src: ThingId, rel: &str, dst: ThingId) {
    let reply = enqueue(RootOp::Link {
        src,
        rel: SymbolShell::Str(String::from(rel)),
        dst,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }
}

pub fn create_bytespace_from_ptr(ptr: u64, len: u64, name: &str) -> ThingId {
    let mut id = ThingId::default();

    let reply = enqueue(RootOp::BytespaceCreateFromPtr {
        ptr,
        len,
        out_ptr: &mut id as *mut _ as u64,
    });
    while reply.done.load(Ordering::Acquire) == 0 {
        unsafe { crate::task::scheduler::yield_now_current(); }
    }

    if !name.is_empty() {
        let mut name_sym = SymbolId::default();
        let reply = enqueue(RootOp::Intern {
            name: String::from(name),
            out_ptr: &mut name_sym as *mut _ as u64,
        });
        while reply.done.load(Ordering::Acquire) == 0 {
            unsafe { crate::task::scheduler::yield_now_current(); }
        }

        let mut val = [0u8; 16];
        val.copy_from_slice(&name_sym.0);

        let reply = enqueue(RootOp::PropSet {
            id,
            key: SymbolShell::Str(String::from("name")),
            value: val,
        });
        while reply.done.load(Ordering::Acquire) == 0 {
            unsafe { crate::task::scheduler::yield_now_current(); }
        }
    }

    id
}
