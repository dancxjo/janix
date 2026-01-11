use crate::BootRuntime;
use super::{pop_msg, RootMsg, RootOp};
use core::sync::atomic::Ordering;
use super::graph::Graph;
use super::journal::{Journal, JournalOp};
use super::resources::{bytespace, stream, ResourceHandle};
use abi::kinds::*;
use core::fmt::Write;

pub extern "C" fn root_main<R: BootRuntime>(_arg: usize) -> ! {
    crate::kinfo!("ROOT: started once");
    
    let mut graph = Graph::new();
    let mut journal = Journal::new();
    
    loop {
        let mut processed = 0;
        while processed < 16 {
            if let Some(msg) = pop_msg() {
                handle_msg(&mut graph, &mut journal, msg);
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

fn handle_msg(graph: &mut Graph, journal: &mut Journal, msg: RootMsg) {
    let (status, value) = match msg.op {
        RootOp::GetKind { id } => {
            if let Some(k) = graph.get_kind(id) {
                (0, k as u64)
            } else {
                (-1, 0)
            }
        },
        RootOp::CreateNode { kind } => {
            let id = graph.alloc(kind);
            journal.append(JournalOp::CreateResult { id, kind });
            (0, id)
        },
        RootOp::BytespaceCreate { len, flags: _, format: _ } => {
            let id = graph.alloc(KIND_BYTESPACE_BUFFER);
            let handle = bytespace::create(len as usize);
            if let Some(node) = graph.get_node_mut(id) {
                node.resource = Some(ResourceHandle::Bytespace(handle));
            }
            journal.append(JournalOp::CreateResult { id, kind: KIND_BYTESPACE_BUFFER });
            (0, id)
        }, 
        RootOp::WatchSubscribe { target_id, mask } => {
            // Check existence first
            let exists = graph.get_kind(target_id).is_some();
            if exists {
                 let stream_id = graph.alloc(KIND_STREAM_WATCH);
                 let handle = stream::create(64);
                 if let Some(stream_node) = graph.get_node_mut(stream_id) {
                     stream_node.resource = Some(ResourceHandle::Stream(handle));
                 }
                 
                 // Re-acquire target to push watch
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
            // Update and Notification
            // Split logic to satisfy borrow checker
            let watches = if let Some(node) = graph.get_node_mut(id) {
                node.props.insert(key, value);
                journal.append(JournalOp::UpdateProp { id, key, val: value });
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
                                          key,
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
             let res = super::debug_fmt::fmt_thing(graph, id, &mut fmt);
             if res.is_ok() {
                  (0, fmt.pos as u64)
             } else {
                  (-1, 0)
             }
        },
        RootOp::DescribeEdge { src, rel, dst, buffer, len } => {
             let mut fmt = FmtBuffer { ptr: buffer as *mut u8, len: len as usize, pos: 0 };
             let res = super::debug_fmt::fmt_edge(graph, src, rel, dst, &mut fmt);
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
                     let _ = super::debug_fmt::fmt_edge(graph, id, rel, dst, &mut fmt);
                     count += 1;
                     if count >= 8 { break; }
                 }
                 (0, fmt.pos as u64)
            } else {
                 (-1, 0)
            }
        },
        RootOp::Link { src, rel, dst } => {
             graph.link(src, rel, dst);
             (0, 0)
        },
        RootOp::DumpGraph { limit } => {
             crate::kinfo!("ROOT DUMP NODES");
             let mut count = 0;
             for (id, _node) in graph.nodes.iter() {
                 if count >= limit { 
                     crate::kinfo!("... truncated ...");
                     break; 
                 }
                 let mut buf = [0u8; 256];
                 let mut fmt = FmtBuffer { ptr: buf.as_mut_ptr(), len: buf.len(), pos: 0 };
                 let _ = super::debug_fmt::fmt_thing(graph, *id, &mut fmt);
                 if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                     crate::kprint!("{}\n", s);
                 }
                 count += 1;
             }
             
             crate::kinfo!("ROOT DUMP EDGES");
             count = 0;
             for (id, node) in graph.nodes.iter() {
                  for (rel, dst) in &node.edges {
                      if count >= limit { break; }
                      let mut buf = [0u8; 512];
                      let mut fmt = FmtBuffer { ptr: buf.as_mut_ptr(), len: buf.len(), pos: 0 };
                      let _ = super::debug_fmt::fmt_edge(graph, *id, *rel, *dst, &mut fmt);
                      if let Ok(s) = core::str::from_utf8(&buf[..fmt.pos]) {
                           crate::kprint!("{}\n", s);
                      }
                      count += 1;
                  }
                  if count >= limit { 
                      crate::kinfo!("... truncated ...");
                      break; 
                  }
             }
             (0, 0)
        },
    };
    
    msg.reply.status.store(status, Ordering::Relaxed);
    msg.reply.value.store(value, Ordering::Relaxed);
    msg.reply.done.store(1, Ordering::Release);
}
