pub trait Console {
    fn write_str(&self, s: &str);
}

pub struct StdoutConsole;

impl Console for StdoutConsole {
    fn write_str(&self, s: &str) {
        use abi::wire::graph::GraphOp;
        use crate::sys::sys_graph;
        
        let op = GraphOp::Log { text: s };
        let mut buf = [0u8; 1024];
        if let Ok(req) = postcard::to_slice(&op, &mut buf) {
            let mut out = [0u8; 32];
            let _ = sys_graph("op", req, &mut out);
        }
    }
}
