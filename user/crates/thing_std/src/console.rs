pub trait Console {
    fn write_str(&self, s: &str);
    fn write_u64(&self, n: u64);
    fn write_2d(&self, n: u64);
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

    fn write_u64(&self, mut n: u64) {
        if n == 0 {
            self.write_str("0");
            return;
        }
        let mut buffer = [0u8; 20];
        let mut i = 20;
        while n > 0 {
            i -= 1;
            buffer[i] = (n % 10) as u8 + b'0';
            n /= 10;
        }
        self.write_str(core::str::from_utf8(&buffer[i..]).unwrap_or("?"));
    }

    fn write_2d(&self, n: u64) {
        let val = n % 100;
        let tens = val / 10;
        let ones = val % 10;
        let buf = [
            tens as u8 + b'0',
            ones as u8 + b'0',
        ];
        self.write_str(core::str::from_utf8(&buf).unwrap_or("??"));
    }
}
