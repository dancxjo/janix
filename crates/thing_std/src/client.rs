use serde::{Serialize, Deserialize};
use crate::syscalls;

pub struct GraphClient;

impl GraphClient {
    pub fn new() -> Self {
        Self
    }

    pub fn query<S: Serialize, D: for<'de> Deserialize<'de>>(
        &self,
        method: &str,
        req: &S,
        out_buf: &mut [u8]
    ) -> Result<D, ()> {
        // Serialize request
        // We use a local buffer for request serialization?
        // Or do we assume `syscalls::graph_query` can take the struct pointer?
        // Syscall ABI is flat bytes. We must serialize.
        // We need a scratch buffer.
        // Limitations of no_std / no-alloc: where to allocate?
        // For v0, maybe stack? 256 bytes?

        let mut req_buf = [0u8; 512];
        let req_slice = postcard::to_slice(req, &mut req_buf).map_err(|_| ())?;

        // Syscall
        // We reuse out_buf for the raw response bytes?
        // `query` takes `out` buffer.
        // Loop until success or non-retryable error
        let used = loop {
            match syscalls::graph_query(method, req_slice, out_buf) {
                Ok(len) => break len,
                Err(e) if e == abi::syscall_defs::SYS_EAGAIN => {
                     // Busy wait / yield
                     // In real OS we might sched_yield, here we rely on preemption
                     core::hint::spin_loop();
                     continue;
                },
                Err(_) => return Err(()),
            }
        };

        let resp_slice = &out_buf[..used];
        let resp = postcard::from_bytes(resp_slice).map_err(|_| ())?;
        Ok(resp)
    }
    pub fn call_op(
        &self,
        op: &abi::wire::graph::GraphOp,
        out_buf: &mut [u8]
    ) -> Result<abi::wire::graph::GraphReply, ()> {
        self.query("op", op, out_buf)
    }
}
