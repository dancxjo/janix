use core::fmt::{self, Write};
use abi::kinds::*;
use super::graph::{Graph, ThingId};

// Helper buffer for writing without allocation
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

pub fn fmt_kind(kind: ThingKind) -> &'static str {
    match kind {
        KIND_BYTESPACE_BUFFER => "bytespace",
        KIND_STREAM_WATCH => "stream.watch",
        KIND_TEST_NODE => "test.node",
        _ => "unknown"
    }
}

pub fn fmt_rel(rel: RelKey) -> &'static str {
    match rel {
        REL_HAS_BUS => "HAS_BUS",
        REL_HAS_DEVICE => "HAS_DEVICE",
        REL_HAS_RESOURCE => "HAS_RESOURCE",
        REL_BINDS => "BINDS",
        REL_BOUND_TO => "BOUND_TO",
        REL_PROVIDES => "PROVIDES",
        REL_EMITS => "EMITS",
        _ => "REL" // or hex?
    }
}

pub fn fmt_thing(graph: &Graph, id: ThingId, w: &mut dyn Write) -> fmt::Result {
   if let Some(node) = graph.get_kind(id).and_then(|_| graph.nodes.get(&id)) {
       write!(w, "(t{:x}:", id)?;
       let kstr = fmt_kind(node.kind);
       if kstr == "unknown" {
           write!(w, "{:x} {{ ", node.kind)?;
       } else {
           write!(w, "{} {{ ", kstr)?;
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
           write!(w, "{}: {}", k, v)?;
           count += 1;
       }
       write!(w, " }})")
   } else {
       write!(w, "(t{:x}:<enoent>)", id)
   }
}

pub fn fmt_edge(graph: &Graph, src: ThingId, rel: RelKey, dst: ThingId, w: &mut dyn Write) -> fmt::Result {
    fmt_thing(graph, src, w)?;
    write!(w, "--[:")?;
    let rstr = fmt_rel(rel);
    if rstr == "REL" {
        write!(w, "REL={:x}", rel)?;
    } else {
        w.write_str(rstr)?;
    }
    write!(w, "]->")?;
    fmt_thing(graph, dst, w)
}
