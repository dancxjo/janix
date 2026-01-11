use super::{ThingId, ThingRef, Thing};
use core::fmt;

pub struct DebugEdge {
    src: ThingId,
    rel: u64,
    dst: ThingId,
}

impl DebugEdge {
    pub fn new(src: ThingId, rel: u64, dst: ThingId) -> Self {
        Self { src, rel, dst }
    }
}

impl fmt::Display for DebugEdge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0u8; 512]; 
        match super::sys::describe_edge(self.src, self.rel, self.dst, &mut buf) {
            Ok(len) => {
                let s = core::str::from_utf8(&buf[..len]).unwrap_or("<invalid utf8>");
                f.write_str(s)
            },
            Err(_) => {
                write!(f, "(t{:x})--[:{:x}]->(t{:x})", self.src.0, self.rel, self.dst.0)
            }
        }
    }
}

pub fn edge(src: ThingId, rel: u64, dst: ThingId) -> DebugEdge {
    DebugEdge::new(src, rel, dst)
}
