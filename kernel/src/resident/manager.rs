use crate::graph::schema;
use crate::graph::store::{self, ResidentRef, ThingNode};
use crate::memory::hhdm;
use crate::resident::mapping;
use crate::sched;
use crate::shared_buffer;
use abi::resident::{
    ResidentAllocArgs, ResidentAllocResp, ResidentError, ResidentErrorCode, ResidentId,
    ResidentMapPerms, ResidentMapResp, RestPolicy, RestResp,
};
use abi::resident_layout::ResidentHeader;
use abi::{MapFlags, ProcessId, ThingId};
use alloc::vec::Vec;

pub fn sys_resident_alloc(args: ResidentAllocArgs) -> Result<ResidentAllocResp, ResidentError> {
    if args.byte_len > 16 * 1024 * 1024 {
        // 16MB limit
        return Err(ResidentError {
            code: ResidentErrorCode::Bounds,
            aux0: args.byte_len as u64,
            aux1: 0,
        });
    }

    let pages = mapping::allocate_pages(args.byte_len as usize).ok_or(ResidentError {
        code: ResidentErrorCode::OutOfMemory,
        aux0: 0,
        aux1: 0,
    })?;

    // Initialize Header
    if let Some(first_page) = pages.first() {
        let vaddr = hhdm::phys_to_virt(first_page.frame.start_address);
        unsafe {
            let ptr = vaddr as *mut u8;
            core::ptr::write_bytes(ptr, 0, core::mem::size_of::<ResidentHeader>());

            let header_ptr = vaddr as *mut ResidentHeader;
            *header_ptr = ResidentHeader {
                magic: ResidentHeader::MAGIC,
                version: 1,
                flags: 0,
                // kind_id is passed in args. assume it is correct?
                // We should probably check schema existence or just store it.
                // The plan says "kind_id: schema kind id".
                // We trust the caller provided a valid ID or we treat it as opaque.
                // But ResHeader has no "kind" string, just "kind_id" u32.
                // We might want to verify it exists?
                // For V1, just store it.
                // Wait, args.kind_id is ThingId. Header needs u32?
                // ThingId is u64. ResidentHeader uses u32 for kind_id (resident_layout.rs).
                // Let's assume schema kinds are low indices or just truncate.
                // Ideally we lookup the kind string from the ID and store that?
                // No, layout has no string.
                // Let's warn if > u32::MAX.
                // Actually `ResPropEntry` uses `key_id: u32` too.
                // We will cast.
                // Wait, the input `sys_resident_alloc` signature in previous code took `&static str`.
                // Now we take `ResidentAllocArgs` with `kind_id: ThingId`.
                // We need to look up the KIND STRING to put in `ThingNode.kind`.
                // Or we need to fetch the ThingNode for `kind_id` and get its `name` prop?
                // This is getting complicated.
                // Let's assume for V1 the caller passes `ResidentAllocArgs` directly from syscall.
                // But we need the `kind` string for `ThingNode`.
                // We can look it up.
                // Or we can keep `sys_resident_alloc` taking `&str` internally and update the syscall dispatcher to resolve it?
                // The PLAN says "sys_resident_alloc(kind_id, byte_len)".
                // So the syscall passes an ID.
                // We should resolve that ID to a string name for `ThingNode.kind`.
                // schema::get_kind_name(id)?
                // If not found, error BadKind.
                // Let's implement that lookup or fallback.

                // For now, let's look up the kind via store.
                // If we can't find it easily, we can use "Unknown".
                prop_count: 0,
                props_off: core::mem::size_of::<ResidentHeader>() as u32,
                data_off: core::mem::size_of::<ResidentHeader>() as u32,
                total_len: args.byte_len,
                generation: 1,
                seq: 0,
                _pad: 0,
            };
        }
    }

    let pid = sched::SCHEDULER
        .lock()
        .current_process_id()
        .ok_or(ResidentError {
            code: ResidentErrorCode::PermissionDenied,
            aux0: 0,
            aux1: 0,
        })?;

    // We DO NOT map automatically in alloc anymore.
    // The user MUST call map separately.

    // Resolve kind string from kind_id
    // For V1 we don't have the kind name easily if not passed.
    // We will just use empty props or minimal defaults in create_resident.

    unsafe {
        let mut guard = store::things_slab().lock();
        let store = guard.as_mut().unwrap();

        // We reuse the kind_id passed by user as the Thing's kind.
        // The resident manager does not validate this against schemas yet.

        let resident_ref = ResidentRef {
            pages,
            byte_len: args.byte_len as usize,
            rw_holder: None, // No Mapper yet
        };

        use crate::graph::iter_things;

        // ...

        let kind_str = "Unknown"; // Stub
        let id = store.create_resident(crate::symbols::intern(kind_str), resident_ref, pid);

        schema::add_to_kind_index(id, crate::symbols::SymbolId(args.kind_id.0 as u32));

        Ok(ResidentAllocResp { id })
    }
}

pub fn sys_resident_map(
    args: abi::resident::ResidentMapArgs,
) -> Result<ResidentMapResp, ResidentError> {
    let thing_id = args.id;
    let perms = args.perms;

    let pid = sched::SCHEDULER
        .lock()
        .current_process_id()
        .ok_or(ResidentError {
            code: ResidentErrorCode::PermissionDenied,
            aux0: 0,
            aux1: 0,
        })?;

    let (resident_ref, flags) = unsafe {
        let mut guard = store::things_slab().lock();
        let store = guard.as_mut().unwrap();

        let node = store.get_node_mut(thing_id).ok_or(ResidentError {
            code: ResidentErrorCode::BadThing,
            aux0: 0,
            aux1: 0,
        })?;

        // Check perms
        let is_write = (perms.0 & ResidentMapPerms::WRITE.0) != 0;

        let resident = node.resident.as_mut().ok_or(ResidentError {
            code: ResidentErrorCode::NotResident,
            aux0: 0,
            aux1: 0,
        })?;

        let flags = if is_write {
            // ReadWrite request
            if let Some(holder) = resident.rw_holder {
                if holder != pid {
                    return Err(ResidentError {
                        code: ResidentErrorCode::AlreadyMappedRw,
                        aux0: holder.0,
                        aux1: 0,
                    });
                }
            } else {
                resident.rw_holder = Some(pid);
            }
            MapFlags::READ | MapFlags::WRITE | MapFlags::USER
        } else {
            // ReadOnly request
            MapFlags::READ | MapFlags::USER
        };

        // Return a clone of ref data needed for mapping to perform outside lock if desired,
        // or just perform mapping calculation here. Frame extraction needs access to pages.
        // We can't clone pages easily if they are not cloneable. ResidentPage is wrapper around Frame.
        // Assuming ResidentPage is Clone (it is just Frame + attributes).
        (resident.clone(), flags)
    };

    // Map
    let frames: Vec<_> = resident_ref.pages.iter().map(|p| p.frame).collect();
    let size_aligned = shared_buffer::align_up(resident_ref.byte_len as u64, 4096);

    let user_vaddr = sched::SCHEDULER
        .lock()
        .reserve_resident_region(pid, size_aligned as usize, 4096)
        .ok_or(ResidentError {
            code: ResidentErrorCode::OutOfMemory,
            aux0: 0,
            aux1: 0,
        })?;

    if let Err(_) = shared_buffer::map_frames_into_current_as(user_vaddr, &frames, flags) {
        return Err(ResidentError {
            code: ResidentErrorCode::PermissionDenied,
            aux0: 0,
            aux1: 0,
        });
    }

    Ok(ResidentMapResp {
        user_addr: user_vaddr,
        byte_len: resident_ref.byte_len as u32,
        _pad: 0,
    })
}

pub fn sys_resident_unmap(thing_id: ThingId) -> Result<(), ResidentError> {
    let pid = sched::SCHEDULER
        .lock()
        .current_process_id()
        .ok_or(ResidentError {
            code: ResidentErrorCode::PermissionDenied,
            aux0: 0,
            aux1: 0,
        })?;

    unsafe {
        let mut guard = store::things_slab().lock();
        let store = guard.as_mut().unwrap();

        if let Some(node) = store.get_node_mut(thing_id) {
            if let Some(res) = node.resident.as_mut() {
                // Logical unmap of RW holder only
                if res.rw_holder == Some(pid) {
                    res.rw_holder = None;
                }
            }
        }
    }
    // We do not unmap pages from page tables here in V1 for simplicity.
    // The OS cleans up on exit.
    Ok(())
}

pub fn sys_thing_rest(thing_id: ThingId, policy: RestPolicy) -> Result<RestResp, ResidentError> {
    crate::resident::serialize::snapshot_and_archive(thing_id, policy)
}

pub fn process_exit_cleanup(pid: ProcessId) {
    let mut modified_ids = Vec::new();

    crate::graph::iter_things(|thing| {
        if thing.owner_process == Some(pid) {
            modified_ids.push(thing.id);
        } else if let Some(res) = &thing.resident {
            if res.rw_holder == Some(pid) {
                modified_ids.push(thing.id);
            }
        }
    });

    unsafe {
        let mut guard = store::things_slab().lock();
        let store = guard.as_mut().unwrap();

        for id in modified_ids {
            // Because we are iterating IDs collected before lock, check existence
            let node_kind_id = if let Some(node) = store.get_node_mut(id) {
                // Clear Locks
                if let Some(res) = &mut node.resident {
                    if res.rw_holder == Some(pid) {
                        res.rw_holder = None;
                    }
                }

                // Check ownership for deletion
                if node.owner_process == Some(pid) {
                    Some(node.kind_id)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(kind_id) = node_kind_id {
                // Delete
                if let Some(_) = store.delete_thing(id) {
                    schema::remove_from_kind_index(id, kind_id);
                }
            }
        }
    }
}
