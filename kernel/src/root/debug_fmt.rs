use super::graph::{Graph, ThingId};
use super::symbols::Interner;
use abi::symbols::SymbolId;
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

pub fn fmt_thing(graph: &Graph, interner: &Interner, id: ThingId, w: &mut dyn Write) -> fmt::Result {
    if let Some(node) = graph.nodes.get(&id) {
        let kind_str = interner.resolve(node.kind).unwrap_or("?");
        
        write!(w, "(t{:x}:{} {{ ", id, kind_str)?;
        
        let mut count = 0;
        for (k, v) in node.props.iter() {
            if count > 0 {
                write!(w, ", ")?;
            }
            if count >= 8 {
                write!(w, "...")?;
                break;
            }
            
            let kname = interner.resolve(*k).unwrap_or("p");
            if *v > 0x10000 {
                 write!(w, "{}: 0x{:x}", kname, v)?;
            } else {
                 write!(w, "{}: {}", kname, v)?;
            }
            count += 1;
        }
        write!(w, " }})")
    } else {
        write!(w, "(t{:x}:?)", id)
    }
}

pub fn fmt_edge(graph: &Graph, interner: &Interner, src: ThingId, rel: SymbolId, dst: ThingId, w: &mut dyn Write) -> fmt::Result {
    let src_kind = if let Some(n) = graph.nodes.get(&src) {
        interner.resolve(n.kind).unwrap_or("?")
    } else {
        "?"
    };

    let dst_kind = if let Some(n) = graph.nodes.get(&dst) {
        interner.resolve(n.kind).unwrap_or("?")
    } else {
        "?"
    };

    let rname = interner.resolve(rel).unwrap_or("REL?");
    
    write!(w, "(t{:x}:{})--[:{}]->(t{:x}:{})", src, src_kind, rname, dst, dst_kind)
}
