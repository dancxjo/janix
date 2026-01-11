use super::graph::{Graph, ThingId};
use abi::kinds::*;
use abi::names::*;
use core::fmt::{self, Write};

pub struct FmtBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub pos: usize,
}

impl Write for FmtBuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
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

pub fn fmt_thing(graph: &Graph, id: ThingId, w: &mut dyn Write) -> fmt::Result {
    if let Some(node) = graph.nodes.get(&id) {
        let kind_str = kind_name(node.kind);
        
        if kind_str == "unknown" {
             write!(w, "(t{:x}:{:x} {{ ", id, node.kind)?;
        } else {
             write!(w, "(t{:x}:{} {{ ", id, kind_str)?;
        }
        
        let mut count = 0;
        for (k, v) in node.props.iter() {
            if count > 0 {
                write!(w, ", ")?;
            }
            if count >= 8 {
                write!(w, "...")?;
                break;
            }
            
            let kname = prop_name(*k);
            if kname == "p" {
                 write!(w, "{}: {:x}", k, v)?;
            } else {
                 // Try to print value heuristically? No, just hex or dec.
                 // Assuming dec fits most numbers well, hex for addresses.
                 // Simple heuristic: if looks like pointer (> 0x100000), print hex.
                 if *v > 0x10000 {
                      write!(w, "{}: 0x{:x}", kname, v)?;
                 } else {
                      write!(w, "{}: {}", kname, v)?;
                 }
            }
            count += 1;
        }
        write!(w, " }})")
    } else {
        write!(w, "(t{:x}:?)", id)
    }
}

pub fn fmt_edge(graph: &Graph, src: ThingId, rel: RelKey, dst: ThingId, w: &mut dyn Write) -> fmt::Result {
    // (src)--[:REL]->(dst)
    // Minimally: (t1:Host)--[:RUNS_ON]->(t2:Kernel)
    // Full: dump src node, rel, dst node? No, verbose.
    // Format: (tX:Kind)--[:REL_NAME]->(tY:Kind)
    
    let get_kind_name = |id: ThingId| -> &'static str {
        if let Some(n) = graph.nodes.get(&id) {
            kind_name(n.kind)
        } else {
            "?"
        }
    };

    let src_kind = get_kind_name(src);
    let dst_kind = get_kind_name(dst);
    let rname = rel_name(rel);
    
    if rname == "REL_UNKNOWN" {
         write!(w, "(t{:x}:{})--[:0x{:x}]->(t{:x}:{})", src, src_kind, rel, dst, dst_kind)
    } else {
         write!(w, "(t{:x}:{})--[:{}]->(t{:x}:{})", src, src_kind, rname, dst, dst_kind)
    }
}
