//! Bytespace resource handlers.

use crate::BootRuntime;
use crate::root::RootMsg;
use crate::root::graph::Graph;
use crate::root::journal::{Journal, JournalOp};
use crate::root::resources::bytespace::Provenance;
use crate::root::resources::{ResourceHandle, bytespace};
use crate::root::symbols::Interner;
#[allow(unused_imports)]
use abi::symbols::SymbolId;
use core::sync::atomic::Ordering;

/// Result type for handler operations: (status, value)
pub type HandlerResult = (i32, u64);

pub fn handle_bytespace_create<R: BootRuntime>(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    _msg: &RootMsg,
    len: u64,
    _flags: u64,
    _format: u64,
) -> HandlerResult {
    let rt = crate::runtime::<R>();
    let hhdm_offset = rt.phys_to_virt_offset();

    let kid = interner.intern("Bytespace");
    let id = graph.alloc(kid);

    // Ensure at least 1 page for v0 robustiness with empty strings
    let alloc_len = if len == 0 { 1 } else { len };

    if let Some(handle) = bytespace::create(alloc_len as usize, hhdm_offset) {
        // Create backing mem.Range node
        let range_kid = interner.intern("mem.Range");
        let range_id = graph.alloc(range_kid);

        // Set properties on mem.Range
        let phys_base_key = interner.intern("phys_base");
        let size_key = interner.intern("size_bytes");
        let page_count_key = interner.intern("page_count");

        {
            let lock = handle.lock();
            if let Some(range_node) = graph.get_node_mut(range_id) {
                range_node.props.insert(phys_base_key, lock.phys_base);
                range_node.props.insert(size_key, lock.len as u64);
                range_node
                    .props
                    .insert(page_count_key, lock.page_count as u64);
            }
        }

        // Link bytespace to mem.Range with BACKED_BY
        let backed_by = interner.intern("BACKED_BY");
        graph.link(id, backed_by, range_id);

        if let Some(node) = graph.get_node_mut(id) {
            node.resource = Some(ResourceHandle::Bytespace(handle));
        }
        journal.append(JournalOp::CreateResult {
            id,
            kind: kid as u64,
        });
        (0, id)
    } else {
        (-1, 0) // Allocation failed
    }
}

pub fn handle_bytespace_create_from_ptr<R: BootRuntime>(
    graph: &mut Graph,
    journal: &mut Journal,
    interner: &mut Interner,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    let rt = crate::runtime::<R>();
    let hhdm_offset = rt.phys_to_virt_offset();

    let kid = interner.intern("Bytespace");
    let id = graph.alloc(kid);
    // Boot assets created via this path get Boot provenance
    let handle =
        bytespace::create_from_ptr(ptr as usize, len as usize, hhdm_offset, Provenance::Boot);

    // Create backing mem.Range node
    let range_kid = interner.intern("mem.Range");
    let range_id = graph.alloc(range_kid);

    let phys_base_key = interner.intern("phys_base");
    let size_key = interner.intern("size_bytes");

    {
        let lock = handle.lock();
        if let Some(range_node) = graph.get_node_mut(range_id) {
            range_node.props.insert(phys_base_key, lock.phys_base);
            range_node.props.insert(size_key, lock.len as u64);
        }
    }

    let backed_by = interner.intern("BACKED_BY");
    graph.link(id, backed_by, range_id);

    if let Some(node) = graph.get_node_mut(id) {
        node.resource = Some(ResourceHandle::Bytespace(handle));
    }
    journal.append(JournalOp::CreateResult {
        id,
        kind: kid as u64,
    });
    (0, id)
}

pub fn handle_bytespace_write(
    graph: &mut Graph,
    id: u64,
    offset: u64,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
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
                (-1, 0) // OOB
            }
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_read(
    graph: &mut Graph,
    id: u64,
    offset: u64,
    ptr: u64,
    len: u64,
) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
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
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_info(graph: &mut Graph, msg: &RootMsg, id: u64) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
            let lock = handle.lock();
            // Return size in value, page_count in p0, flags in p1
            msg.reply
                .p0
                .store(lock.page_count as u64, Ordering::Relaxed);
            msg.reply.p1.store(lock.flags, Ordering::Relaxed);
            (0, lock.len as u64)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_map(graph: &mut Graph, msg: &RootMsg, id: u64, tid: u64) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
            let lock = handle.lock();

            // Allocate user VA
            let user_va = crate::memory::alloc_user_va(lock.len);

            bytespace::record_mapping(id, tid, user_va, lock.len);

            // Store phys_base in p0 for syscall handler to do actual mapping
            msg.reply.p0.store(lock.phys_base, Ordering::Relaxed);
            msg.reply
                .p1
                .store(lock.page_count as u64, Ordering::Relaxed);

            (0, user_va)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

pub fn handle_bytespace_unmap(id: u64, user_va: u64, tid: u64) -> HandlerResult {
    if let Some(_mapping) = bytespace::remove_mapping(id, tid, user_va) {
        (0, 0)
    } else {
        (-1, 0) // Mapping not found
    }
}

pub fn handle_bytespace_phys(graph: &mut Graph, msg: &RootMsg, id: u64) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
            let lock = handle.lock();
            // Return phys_base in value, len in p0
            msg.reply.p0.store(lock.len as u64, Ordering::Relaxed);
            (0, lock.phys_base)
        } else {
            (-1, 0)
        }
    } else {
        (-1, 0)
    }
}

/// Truncate a bytespace to a new length.
/// For v0, we only support truncating to a smaller size (i.e., updating the logical length).
pub fn handle_bytespace_truncate(
    graph: &mut Graph,
    id: u64,
    new_len: u64,
) -> HandlerResult {
    if let Some(node) = graph.get_node_mut(id) {
        if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
            let mut lock = handle.lock();
            let new_len = new_len as usize;
            if new_len <= lock.page_count * 4096 {
                // Just update the logical length (within allocated pages)
                lock.len = new_len;
                (0, 0)
            } else {
                (-22, 0) // EINVAL — can't grow beyond allocated pages
            }
        } else {
            (-2, 0) // ENOENT
        }
    } else {
        (-2, 0) // ENOENT
    }
}

/// Resolve a `/`-separated path to a ThingId by walking `content.contains` edges.
///
/// Starts at the root service node (graph_anchors::root_service()).
/// For each path segment, looks at outgoing edges with the `content.contains`
/// relation and checks if the target node has a matching `file.name` or `dir.name`.
pub fn handle_resolve_path(
    graph: &mut Graph,
    interner: &mut Interner,
    path: &str,
) -> HandlerResult {
    let root_id = match crate::root::graph_anchors::root_service() {
        Some(id) if id != 0 => id,
        _ => return (-2, 0), // ENOENT
    };

    let contains_sym = interner.intern("content.contains");
    let file_name_sym = interner.intern("file.name");
    let dir_name_sym = interner.intern("dir.name");

    // Strip leading /
    let path = path.strip_prefix('/').unwrap_or(path);
    if path.is_empty() {
        return (0, root_id); // Root itself
    }

    let mut current = root_id;

    for segment in path.split('/') {
        if segment.is_empty() {
            continue;
        }

        let seg_sym = interner.intern(segment);

        // Collect targets of content.contains edges (avoids double borrow)
        let targets: alloc::vec::Vec<u64> = if let Some(node) = graph.get_node_mut(current) {
            node.edges
                .iter()
                .filter(|&&(rel, _)| rel == contains_sym)
                .map(|&(_, target)| target)
                .collect()
        } else {
            return (-2, 0); // ENOENT
        };

        // Now search targets for matching name
        let mut found = false;
        for target in targets {
            if let Some(target_node) = graph.get_node_mut(target) {
                let name_match = target_node
                    .props
                    .get(&file_name_sym)
                    .or_else(|| target_node.props.get(&dir_name_sym))
                    .copied();
                if name_match == Some(seg_sym as u64) {
                    current = target;
                    found = true;
                    break;
                }
            }
        }

        if !found {
            return (-2, 0); // ENOENT
        }
    }

    (0, current)
}


