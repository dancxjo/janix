use super::graph::Graph;
use super::journal::{Journal, JournalOp};
use super::resources::{ResourceHandle, bytespace, stream};
use super::symbols::Interner;
use super::{RootMsg, RootOp, SymbolShell, pop_msg};
use crate::BootRuntime;
use core::sync::atomic::Ordering;

use abi::symbols::SymbolId;
use core::fmt::Write;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    crate::kinfo!("ROOT: started once");

    let mut graph = Graph::new();
    let mut journal = Journal::new();
    let mut interner = Interner::new();

    loop {
        let mut processed = 0;
        while processed < 16 {
            if let Some(msg) = pop_msg() {
                handle_msg::<R>(&mut graph, &mut journal, &mut interner, msg);
                processed += 1;
            } else {
                break;
            }
        }

        unsafe {
            crate::task::scheduler::yield_now_current();
        }
    }
}

// Helper for DescribeThing formatting
struct FmtBuffer {
    ptr: *mut u8,
    len: usize,
    pos: usize,
}

impl core::fmt::Write for FmtBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let rem = self.len - self.pos;
        let copy_len = core::cmp::min(bytes.len(), rem);
        if copy_len > 0 {
            unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(self.pos), copy_len);
            }
            self.pos += copy_len;
        }
        Ok(())
    }
}

// Helper to resolve Shell
fn resolve_shell(shell: SymbolShell, interner: &mut Interner) -> SymbolId {
    match shell {
        SymbolShell::Id(id) => id,
        SymbolShell::Str(s) => interner.intern(&s),
    }
}

fn handle_msg<R: BootRuntime>(graph: &mut Graph, journal: &mut Journal, interner: &mut Interner, msg: RootMsg) {
    let rt = crate::runtime::<R>();
    let hhdm_offset = rt.phys_to_virt_offset();
    
    let (status, value) = match msg.op {
        RootOp::Intern { name } => {
            let id = interner.intern(&name);
            (0, id as u64)
        }
        RootOp::GetKind { id } => {
            if let Some(k) = graph.get_kind(id) {
                (0, k as u64)
            } else {
                (-1, 0)
            }
        }
        RootOp::CreateNode { kind } => {
            let kid = resolve_shell(kind, interner);
            let id = graph.alloc(kid);
            journal.append(JournalOp::CreateResult {
                id,
                kind: kid as u64,
            });
            (0, id)
        }
        RootOp::PropGet { id, key } => {
            let kid = resolve_shell(key, interner);
            if let Some(node) = graph.get_node_mut(id) {
                if let Some(val) = node.props.get(&kid) {
                    (0, *val)
                } else {
                    (-1, 0)
                }
            } else {
                (-1, 0)
            }
        }
        RootOp::Query {
            plan,
            out_buffer,
            out_len,
        } => {
            let max_rows = (out_len as usize) / core::mem::size_of::<abi::query::QueryRow>();
            let mut krows = alloc::vec![abi::query::QueryRow::default(); max_rows];

            let res = super::query::execute(graph, &plan, &mut krows);

            if let Ok(count) = res {
                unsafe {
                    let dst = out_buffer as *mut abi::query::QueryRow;
                    for i in 0..count {
                        *dst.add(i) = krows[i];
                    }
                }
                (0, count as u64)
            } else {
                (-1, 0)
            }
        }
        RootOp::Find { kind, buffer, len } => {
            let kid = resolve_shell(kind, interner);
            let mut found_count = 0;
            let out_ptr = buffer as *mut u64;
            let max_entries = (len as usize) / 8;

            for (id, node) in &graph.nodes {
                if node.kind == kid {
                    if found_count < max_entries {
                        unsafe {
                            *out_ptr.add(found_count) = *id;
                        }
                    }
                    found_count += 1;
                }
            }
            (0, found_count as u64)
        }
        RootOp::BytespaceCreate {
            len,
            flags: _,
            format: _,
        } => {
            let kid = interner.intern("Bytespace");
            let id = graph.alloc(kid);
            
            if let Some(handle) = bytespace::create(len as usize, hhdm_offset) {
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
                        range_node.props.insert(page_count_key, lock.page_count as u64);
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
        RootOp::BytespaceCreateFromPtr { ptr, len } => {
            let kid = interner.intern("Bytespace");
            let id = graph.alloc(kid);
            let handle = bytespace::create_from_ptr(ptr as usize, len as usize, hhdm_offset);
            
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
        RootOp::BytespaceWrite {
            id,
            offset,
            ptr,
            len,
        } => {
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
        RootOp::BytespaceRead {
            id,
            offset,
            ptr,
            len,
        } => {
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
        RootOp::BytespaceInfo { id } => {
            if let Some(node) = graph.get_node_mut(id) {
                if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
                    let lock = handle.lock();
                    // Return size in value, page_count in p0, flags in p1
                    msg.reply.p0.store(lock.page_count as u64, Ordering::Relaxed);
                    msg.reply.p1.store(lock.flags, Ordering::Relaxed);
                    (0, lock.len as u64)
                } else {
                    (-1, 0)
                }
            } else {
                (-1, 0)
            }
        }
        RootOp::BytespaceMap { id, tid } => {
            // Map bytespace into the caller's address space
            if let Some(node) = graph.get_node_mut(id) {
                if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
                    let lock = handle.lock();
                    
                    // Allocate user VA
                    let user_va = bytespace::alloc_user_va(lock.len);
                    
                    // Map each page into caller's address space
                    // For v0, we need to get the caller's aspace and map pages
                    // This requires accessing the scheduler which is complex from Root
                    // For now: record the mapping and return the VA
                    // The actual page table mapping will be done in the syscall handler
                    
                    bytespace::record_mapping(id, tid, user_va, lock.len);
                    
                    // Store phys_base in p0 for syscall handler to do actual mapping
                    msg.reply.p0.store(lock.phys_base, Ordering::Relaxed);
                    msg.reply.p1.store(lock.page_count as u64, Ordering::Relaxed);
                    
                    (0, user_va)
                } else {
                    (-1, 0)
                }
            } else {
                (-1, 0)
            }
        }
        RootOp::BytespaceUnmap { id, user_va, tid } => {
            if let Some(_mapping) = bytespace::remove_mapping(id, tid, user_va) {
                // Mapping removed. Actual page table unmapping done in syscall handler.
                (0, 0)
            } else {
                (-1, 0) // Mapping not found
            }
        }
        RootOp::BytespacePhys { id } => {
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
        RootOp::WatchSubscribe { target_id, mask } => {
            let kid = interner.intern("stream.watch");
            let exists = graph.get_kind(target_id).is_some();
            if exists {
                let stream_id = graph.alloc(kid);
                let handle = stream::create(64);
                if let Some(stream_node) = graph.get_node_mut(stream_id) {
                    stream_node.resource = Some(ResourceHandle::Stream(handle));
                }
                if let Some(target_node) = graph.get_node_mut(target_id) {
                    target_node.watches.push((mask, stream_id));
                }
                (0, stream_id)
            } else {
                (-1, 0)
            }
        }
        RootOp::StreamPoll {
            stream_id,
            max: _,
            out_ptr: _,
        } => {
            if let Some(node) = graph.get_node_mut(stream_id) {
                if let Some(ResourceHandle::Stream(handle)) = &node.resource {
                    let mut lock = handle.lock();
                    if let Some(evt) = lock.events.pop_front() {
                        msg.reply.p0.store(evt.target, Ordering::Relaxed);
                        msg.reply.p1.store(evt.key, Ordering::Relaxed);
                        msg.reply.p2.store(evt.value, Ordering::Relaxed);
                        (0, 1)
                    } else {
                        (0, 0)
                    }
                } else {
                    (-1, 0)
                }
            } else {
                (-1, 0)
            }
        }
        RootOp::PropSet { id, key, value } => {
            let kid = resolve_shell(key, interner);
            let watches = if let Some(node) = graph.get_node_mut(id) {
                node.props.insert(kid, value);
                journal.append(JournalOp::UpdateProp {
                    id,
                    key: kid as u64,
                    val: value,
                });
                Some(node.watches.clone())
            } else {
                None
            };

            if let Some(watches_vec) = watches {
                for (_mask, stream_id) in watches_vec {
                    if let Some(stream_node) = graph.get_node_mut(stream_id) {
                        if let Some(ResourceHandle::Stream(handle)) = &stream_node.resource {
                            let mut lock = handle.lock();
                            if lock.events.len() < lock.capacity {
                                lock.events.push_back(super::resources::stream::WatchEvent {
                                    target: id,
                                    key: kid as u64,
                                    value,
                                });
                            }
                        }
                    }
                }
                (0, 0)
            } else {
                (-1, 0)
            }
        }
        RootOp::DescribeThing { id, buffer, len } => {
            let mut fmt = FmtBuffer {
                ptr: buffer as *mut u8,
                len: len as usize,
                pos: 0,
            };
            let res = super::debug_fmt::fmt_thing(graph, interner, id, &mut fmt);
            if res.is_ok() {
                (0, fmt.pos as u64)
            } else {
                (-1, 0)
            }
        }
        RootOp::DescribeEdge {
            src,
            rel,
            dst,
            buffer,
            len,
        } => {
            let rid = resolve_shell(rel, interner);
            let mut fmt = FmtBuffer {
                ptr: buffer as *mut u8,
                len: len as usize,
                pos: 0,
            };
            let res = super::debug_fmt::fmt_edge(graph, interner, src, rid, dst, &mut fmt);
            if res.is_ok() {
                (0, fmt.pos as u64)
            } else {
                (-1, 0)
            }
        }
        RootOp::DumpEdges { id, buffer, len } => {
            if let Some(node) = graph.get_kind(id).and_then(|_| graph.nodes.get(&id)) {
                let mut fmt = FmtBuffer {
                    ptr: buffer as *mut u8,
                    len: len as usize,
                    pos: 0,
                };
                let mut count = 0;
                let edges = node.edges.clone();
                for (rel, dst) in edges {
                    if count > 0 {
                        let _ = writeln!(fmt);
                    }
                    let _ = super::debug_fmt::fmt_edge(graph, interner, id, rel, dst, &mut fmt);
                    count += 1;
                    if count >= 8 {
                        break;
                    }
                }
                (0, fmt.pos as u64)
            } else {
                (-1, 0)
            }
        }
        RootOp::Link { src, rel, dst } => {
            let rid = resolve_shell(rel, interner);
            graph.link(src, rid, dst);
            (0, 0)
        }
        RootOp::DumpGraph { limit } => {
            let _txn = crate::logging::LogTransaction::begin("rootdump");

            crate::kinfo!("ROOT DUMP NODES count={}", graph.nodes.len());
            let mut count = 0;
            for (id, _) in &graph.nodes {
                if count >= limit {
                    crate::kinfo!("... truncated ...");
                    break;
                }
                let mut buf = [0u8; 256];
                let mut fmt = FmtBuffer {
                    ptr: buf.as_mut_ptr(),
                    len: buf.len(),
                    pos: 0,
                };
                let _ = super::debug_fmt::fmt_thing(graph, interner, *id, &mut fmt);
                if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                    crate::kprint!("{}\n", s);
                }
                count += 1;
            }

            crate::kinfo!("ROOT DUMP EDGES");
            count = 0;
            'outer: for (src, node) in &graph.nodes {
                for (rel, dst) in &node.edges {
                    if count >= limit {
                        crate::kinfo!("... truncated ...");
                        break 'outer;
                    }
                    let mut buf = [0u8; 512];
                    let mut fmt = FmtBuffer {
                        ptr: buf.as_mut_ptr(),
                        len: buf.len(),
                        pos: 0,
                    };
                    let _ = super::debug_fmt::fmt_edge(graph, interner, *src, *rel, *dst, &mut fmt);
                    if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                        crate::kprint!("{}\n", s);
                    }
                    count += 1;
                }
            }

            (0, 0)
        }
        RootOp::LogEvent {
            level,
            event,
            message,
            timestamp,
            provenance,
            fields,
            about,
        } => {
            let kind_id = interner.intern("log.Entry");
            let entry_id = graph.alloc(kind_id);

            let p_level = interner.intern("level");
            let p_line = interner.intern("line");
            let p_ts = interner.intern("timestamp");
            let p_msg = interner.intern("message");
            let p_evt = interner.intern("event");

            let event_id = resolve_shell(event, interner);
            let msg_id = interner.intern(&message);

            if let Some(node) = graph.get_node_mut(entry_id) {
                node.props.insert(p_level, level as u64);
                node.props.insert(p_line, provenance.line as u64);
                node.props.insert(p_ts, timestamp);
                node.props.insert(p_msg, msg_id as u64);
                node.props.insert(p_evt, event_id as u64);

                for (key_shell, val) in fields {
                    let kid = resolve_shell(key_shell, interner);
                    node.props.insert(kid, val);
                }
            }

            let r_about = interner.intern("ABOUT");
            for subject_id in about {
                graph.link(entry_id, r_about, subject_id);
            }

            let p_file = interner.intern("file");
            let p_module = interner.intern("module");
            let p_tid = interner.intern("tid");

            if let Some(node) = graph.get_node_mut(entry_id) {
                let f_id = interner.intern(&provenance.file);
                let m_id = interner.intern(&provenance.module);
                node.props.insert(p_file, f_id as u64);
                node.props.insert(p_module, m_id as u64);
                node.props.insert(p_tid, provenance.tid);
            }

            (0, entry_id)
        }
    };

    msg.reply.status.store(status, Ordering::Relaxed);
    msg.reply.value.store(value, Ordering::Relaxed);
    msg.reply.done.store(1, Ordering::Release);
}
