use crate::BootRuntime;
use super::{pop_msg, RootMsg, RootOp, SymbolShell};
use core::sync::atomic::Ordering;
use super::graph::Graph;
use super::journal::{Journal, JournalOp};
use super::resources::{bytespace, stream, ResourceHandle};
use super::symbols::Interner;

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
                handle_msg(&mut graph, &mut journal, &mut interner, msg);
                processed += 1;
            } else {
                break;
            }
        }
        
        unsafe { crate::task::scheduler::yield_now_current(); }
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

fn handle_msg(graph: &mut Graph, journal: &mut Journal, interner: &mut Interner, msg: RootMsg) {
    let (status, value) = match msg.op {
        RootOp::Intern { name } => {
            let id = interner.intern(&name);
            (0, id as u64)
        },
        RootOp::GetKind { id } => {
            if let Some(k) = graph.get_kind(id) {
                (0, k as u64)
            } else {
                (-1, 0)
            }
        },
        RootOp::CreateNode { kind } => {
            let kid = resolve_shell(kind, interner);
            let id = graph.alloc(kid);
            // journal type mismatch but ignore for v0.1
            journal.append(JournalOp::CreateResult { id, kind: kid as u64 });
            (0, id)
        },
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
        },
        RootOp::Query { plan, out_buffer, out_len } => {
             // We need a kernel buffer to write results, separate from user pointer.
             let max_rows = (out_len as usize) / core::mem::size_of::<abi::query::QueryRow>();
             let mut krows = alloc::vec![abi::query::QueryRow::default(); max_rows];
             
             let res = super::query::execute(graph, &plan, &mut krows);
             
             if let Ok(count) = res {
                 // Copy back to `out_buffer` (kernel ptr to kbuf in handler)
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
        },
        RootOp::Find { kind, buffer, len } => {
             let kid = resolve_shell(kind, interner);
             // Linear scan for now - cheap enough for startup enumeration
             let mut found_count = 0;
             let out_ptr = buffer as *mut u64;
             let max_entries = (len as usize) / 8;
             
             for (id, node) in &graph.nodes {
                  if node.kind == kid {
                       if found_count < max_entries {
                           unsafe { *out_ptr.add(found_count) = *id; }
                       }
                       found_count += 1;
                  }
             }
             (0, found_count as u64)
        },
        RootOp::BytespaceCreate { len, flags: _, format: _ } => {
            // Need a symbol for Bytespace. Intern it.
            let kid = interner.intern("bytespace");
            let id = graph.alloc(kid);
            let handle = bytespace::create(len as usize);
            if let Some(node) = graph.get_node_mut(id) {
                node.resource = Some(ResourceHandle::Bytespace(handle));
            }
            journal.append(JournalOp::CreateResult { id, kind: kid as u64 });
            (0, id)
        }, 
        RootOp::BytespaceWrite { id, offset, ptr, len } => {
             if let Some(node) = graph.get_node_mut(id) {
                 if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
                      let mut lock = handle.lock();
                      if (offset + len) as usize <= lock.len {
                           unsafe {
                               core::ptr::copy_nonoverlapping(
                                   ptr as *const u8, 
                                   (lock.ptr as *mut u8).add(offset as usize), 
                                   len as usize
                               );
                           }
                           (0, len)
                      } else {
                           (-1, 0) // OOB
                      }
                 } else {
                      (-1, 0) // Not a bytespace
                 }
             } else {
                 (-1, 0) // ENOENT
             }
        },
        RootOp::BytespaceRead { id, offset, ptr, len } => {
             if let Some(node) = graph.get_node_mut(id) {
                 if let Some(ResourceHandle::Bytespace(handle)) = &node.resource {
                      let lock = handle.lock();
                      if (offset + len) as usize <= lock.len {
                           unsafe {
                               core::ptr::copy_nonoverlapping(
                                   (lock.ptr as *const u8).add(offset as usize),
                                   ptr as *mut u8, 
                                   len as usize
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
        },
        RootOp::WatchSubscribe { target_id, mask } => {
             // Need symbol for stream.watch
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
        },
        RootOp::StreamPoll { stream_id, max: _, out_ptr: _ } => {
             if let Some(node) = graph.get_node_mut(stream_id) {
                 if let Some(ResourceHandle::Stream(handle)) = &node.resource {
                      let mut lock = handle.lock();
                      if let Some(evt) = lock.events.pop_front() {
                          msg.reply.p0.store(evt.target, Ordering::Relaxed);
                          // key is SymbolId now, evt.key is u64 (was PropKey).
                          // Wait, WatchEvent struct defines key as u64.
                          msg.reply.p1.store(evt.key, Ordering::Relaxed);
                          msg.reply.p2.store(evt.value, Ordering::Relaxed);
                          (0, 1) // 1 event returned
                      } else {
                          // EAGAIN? or just 0
                          (0, 0)
                      }
                 } else {
                      (-1, 0) // Not a stream
                 }
             } else {
                 (-1, 0) // ENOENT
             }
        },
        RootOp::PropSet { id, key, value } => {
            let kid = resolve_shell(key, interner);
            let watches = if let Some(node) = graph.get_node_mut(id) {
                node.props.insert(kid, value);
                journal.append(JournalOp::UpdateProp { id, key: kid as u64, val: value });
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
                                          value
                                     });
                                }
                           }
                      }
                 }
                 (0, 0)
            } else {
                 (-1, 0)
            }
        },
        RootOp::DescribeThing { id, buffer, len } => {
             let mut fmt = FmtBuffer { ptr: buffer as *mut u8, len: len as usize, pos: 0 };
             let res = super::debug_fmt::fmt_thing(graph, interner, id, &mut fmt);
             if res.is_ok() {
                  (0, fmt.pos as u64)
             } else {
                  (-1, 0)
             }
        },
        RootOp::DescribeEdge { src, rel, dst, buffer, len } => {
             let rid = resolve_shell(rel, interner);
             let mut fmt = FmtBuffer { ptr: buffer as *mut u8, len: len as usize, pos: 0 };
             let res = super::debug_fmt::fmt_edge(graph, interner, src, rid, dst, &mut fmt);
             if res.is_ok() {
                  (0, fmt.pos as u64)
             } else {
                  (-1, 0)
             }
        },
        RootOp::DumpEdges { id, buffer, len } => {
            if let Some(node) = graph.get_kind(id).and_then(|_| graph.nodes.get(&id)) {
                 let mut fmt = FmtBuffer { ptr: buffer as *mut u8, len: len as usize, pos: 0 };
                 let mut count = 0;
                 let edges = node.edges.clone();
                 for (rel, dst) in edges {
                     if count > 0 {
                          let _ = writeln!(fmt);
                     }
                     let _ = super::debug_fmt::fmt_edge(graph, interner, id, rel, dst, &mut fmt);
                     count += 1;
                     if count >= 8 { break; }
                 }
                 (0, fmt.pos as u64)
            } else {
                 (-1, 0)
            }
        },
        RootOp::Link { src, rel, dst } => {
             let rid = resolve_shell(rel, interner);
             graph.link(src, rid, dst);
             (0, 0)
        },
        RootOp::DumpGraph { limit } => {
             crate::kinfo!("ROOT DUMP NODES");
             // Sort nodes by id for deterministic output
             let mut ids: alloc::vec::Vec<_> = graph.nodes.keys().cloned().collect();
             ids.sort();

             let mut count = 0;
             for id in &ids {
                 if count >= limit { 
                     crate::kinfo!("... truncated ...");
                     break; 
                 }
                 let mut buf = [0u8; 256];
                 let mut fmt = FmtBuffer { ptr: buf.as_mut_ptr(), len: buf.len(), pos: 0 };
                 let _ = super::debug_fmt::fmt_thing(graph, interner, *id, &mut fmt);
                 if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                     crate::kprint!("{}\n", s);
                 }
                 count += 1;
             }
             
             crate::kinfo!("ROOT DUMP EDGES");
             count = 0;
             
             // Collect all edges for global sort
             let mut all_edges = alloc::vec::Vec::new();
             for id in &ids {
                 if let Some(node) = graph.nodes.get(id) {
                     for (rel, dst) in &node.edges {
                         all_edges.push((*id, *rel, *dst));
                     }
                 }
             }
             // Tuple sort (src, rel, dst)
             all_edges.sort();

             for (src, rel, dst) in all_edges {
                  if count >= limit { 
                       crate::kinfo!("... truncated ...");
                       break; 
                  }
                  let mut buf = [0u8; 512];
                  let mut fmt = FmtBuffer { ptr: buf.as_mut_ptr(), len: buf.len(), pos: 0 };
                  let _ = super::debug_fmt::fmt_edge(graph, interner, src, rel, dst, &mut fmt);
                  if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                       crate::kprint!("{}\n", s);
                  }
                  count += 1;
             }
             (0, 0)
        },
    };
    
    msg.reply.status.store(status, Ordering::Relaxed);
    msg.reply.value.store(value, Ordering::Relaxed);
    msg.reply.done.store(1, Ordering::Release);
}
