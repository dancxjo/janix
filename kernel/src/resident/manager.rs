use crate::graph::store::{self, ResidentRef, StorageState, ThingNode};
use crate::graph::schema;
use abi::{ThingId, ProcessId, MapFlags};
use abi::resident::{ResidentAllocResp, ResidentError, ResidentErrorCode, ResidentMapPerms, ResidentMapResp, RestPolicy, RestResp, ResidentHandle};
use crate::resident::mapping::{self, ResidentPage};
use crate::sched;
use crate::shared_buffer;
use abi::resident_layout::ResidentHeader;
use alloc::vec::Vec;
use crate::memory::hhdm;

pub fn sys_resident_alloc(kind: &'static str, byte_len: u32) -> Result<ResidentAllocResp, ResidentError> {
    if byte_len > 256 * 1024 {
         return Err(ResidentError { code: ResidentErrorCode::Bounds, aux0: byte_len as u64, aux1: 0 });
    }

    let pages = mapping::allocate_pages(byte_len as usize).ok_or(ResidentError {
        code: ResidentErrorCode::SerializeFailed, // using as alloc error
        aux0: 0, aux1: 0
    })?;

    let kind_id = schema::ensure_kind_exists(kind);

    // Initialize Header
    if let Some(first_page) = pages.first() {
        let vaddr = hhdm::phys_to_virt(first_page.frame.start_address);
        unsafe {
            let ptr = vaddr as *mut u8;
            // Zero the header area safely
            core::ptr::write_bytes(ptr, 0, core::mem::size_of::<ResidentHeader>());
            
            let header_ptr = vaddr as *mut ResidentHeader;
            *header_ptr = ResidentHeader {
                magic: ResidentHeader::MAGIC,
                version: 1,
                flags: 0,
                kind_id: kind_id.0 as u32,
                prop_count: 0,
                _pad0: 0,
                props_off: core::mem::size_of::<ResidentHeader>() as u32,
                data_off: core::mem::size_of::<ResidentHeader>() as u32,
                total_len: byte_len,
                generation: 1, // Start at 1
                seq: 0,
                _pad1: 0,
            };
        }
    }

    let pid = sched::SCHEDULER.lock().current_process_id().ok_or(ResidentError {
        code: ResidentErrorCode::PermissionDenied, aux0: 0, aux1: 0 
    })?;

    let handle = ResidentHandle(crate::graph::store::peek_next_slab_id().0 as u64);

    // Reserve User Address
    let frames: Vec<_> = pages.iter().map(|p| p.frame).collect();
    let size_aligned = shared_buffer::align_up(byte_len as u64, 4096);
    
    let user_vaddr = sched::SCHEDULER.lock().reserve_user_region(pid, size_aligned as usize, 4096).ok_or(ResidentError {
        code: ResidentErrorCode::SerializeFailed, aux0: 0, aux1: 0
    })?;

    // Map RW for owner
    if let Err(_) = shared_buffer::map_frames_into_current_as(user_vaddr, &frames, MapFlags::READ | MapFlags::WRITE | MapFlags::USER) {
         return Err(ResidentError { code: ResidentErrorCode::PermissionDenied, aux0: 0, aux1: 0 });
    }

    let resident_ref = ResidentRef {
        handle: ResidentHandle(user_vaddr), 
        pages,
        byte_len: byte_len as usize,
        rw_owner: Some(pid),
    };

    let safe_kind = alloc::boxed::Box::leak(alloc::string::String::from(kind).into_boxed_str());

    // Create Thing
    unsafe {
        let slab = store::things_slab();
        let (idx, generation) = slab.alloc();
        let id = ThingId::new(idx, generation);
        
        let slot = &mut slab.slots[idx as usize];
        slot.thing = Some(ThingNode::new_resident(
            id,
            safe_kind,
            kind_id,
            resident_ref,
            pid
        ));
        
        schema::add_to_kind_index(id, kind_id);
        
        Ok(ResidentAllocResp {
            thing_id: id,
            handle: ResidentHandle(id.0),
            user_addr: user_vaddr,
            byte_len,
            _pad: 0,
        })
    }
}

pub fn sys_resident_map(thing_id: ThingId, perms: ResidentMapPerms) -> Result<ResidentMapResp, ResidentError> {
    let pid = sched::SCHEDULER.lock().current_process_id().ok_or(ResidentError {
        code: ResidentErrorCode::PermissionDenied, aux0: 0, aux1: 0
    })?;

    unsafe {
        let slab = store::things_slab();
        let idx = thing_id.index() as usize;
        if idx >= slab.slots.len() {
             return Err(ResidentError { code: ResidentErrorCode::BadThing, aux0: 0, aux1: 0 });
        }
        let slot = &mut slab.slots[idx];
        if slot.generation != thing_id.generation() {
             return Err(ResidentError { code: ResidentErrorCode::BadThing, aux0: 0, aux1: 0 });
        }
        
        let thing = slot.thing.as_mut().ok_or(ResidentError { code: ResidentErrorCode::BadThing, aux0: 0, aux1: 0 })?;
        
        let resident = thing.resident.as_mut().ok_or(ResidentError { code: ResidentErrorCode::NotResident, aux0: 0, aux1: 0 })?;
        
        // Check perms
        let flags = match perms {
            ResidentMapPerms::ReadOnly => MapFlags::READ | MapFlags::USER,
            ResidentMapPerms::ReadWrite => {
                // Single Writer Enforcement
                if let Some(owner) = resident.rw_owner {
                    if owner != pid {
                        return Err(ResidentError { code: ResidentErrorCode::AlreadyMappedRw, aux0: owner.0, aux1: 0 });
                    }
                    // If owner == pid, allow re-entrant (idempotent)
                } else {
                    resident.rw_owner = Some(pid);
                }
                MapFlags::READ | MapFlags::WRITE | MapFlags::USER
            }
        };

        // Map
        let frames: Vec<_> = resident.pages.iter().map(|p| p.frame).collect();
        let size_aligned = shared_buffer::align_up(resident.byte_len as u64, 4096);
        
        let user_vaddr = sched::SCHEDULER.lock().reserve_user_region(pid, size_aligned as usize, 4096).ok_or(ResidentError {
             code: ResidentErrorCode::SerializeFailed, aux0: 0, aux1: 0
        })?;
        
        if let Err(_) = shared_buffer::map_frames_into_current_as(user_vaddr, &frames, flags) {
             return Err(ResidentError { code: ResidentErrorCode::PermissionDenied, aux0: 0, aux1: 0 });
        }

        Ok(ResidentMapResp {
            user_addr: user_vaddr,
            byte_len: resident.byte_len as u32,
            _pad: 0,
        })
    }
}

pub fn sys_resident_unmap(_thing_id: ThingId) -> Result<(), ResidentError> {
    // Unmap not fully implemented deep down in VM yet, but we can release RW lock if held.
    // For v1, explicit unmap might just be a NOP or purely logical.
    // Process exit cleanup is more important.
    Ok(())
}

pub fn sys_thing_rest(thing_id: ThingId, policy: RestPolicy) -> Result<RestResp, ResidentError> {
    // 1. Check permissions / existence
    // 2. Snapshot (serialize)
    // 3. Store archived
    // 4. Update state
    
    // For simplicity, delegating serialization to serialize.rs
    // But we need to handle the state transition here.
    
    crate::resident::serialize::snapshot_and_archive(thing_id, policy)
}

pub fn process_exit_cleanup(pid: ProcessId) {
    let mut to_delete = Vec::new();
    let mut to_unlock = Vec::new();

    store::iter_things(|thing| {
        // Collect owned things for deletion
        if thing.owner_process == Some(pid) {
            to_delete.push(thing.id);
        } else if let Some(res) = &thing.resident {
            // Check for RW ownership to unlock on things we don't own
            if res.rw_owner == Some(pid) {
                to_unlock.push(thing.id);
            }
        }
    });

    for id in to_delete {
        store::delete_thing(id);
    }

    // Release RW locks for things the process mapped but didn't own
    if !to_unlock.is_empty() {
        unsafe {
            let slab = store::things_slab();
            for id in to_unlock {
                if let Some(slot) = slab.slots.get_mut(id.index() as usize) {
                    if slot.generation == id.generation() {
                        if let Some(thing) = &mut slot.thing {
                            if let Some(res) = &mut thing.resident {
                                if res.rw_owner == Some(pid) {
                                    res.rw_owner = None;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
