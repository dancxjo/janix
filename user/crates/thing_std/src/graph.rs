extern crate alloc;
// use alloc::vec::Vec; // Not used yet

pub struct GraphClient;

#[derive(Debug)]
pub enum GraphError {
    Sys(i64),
    Encode,
    Decode,
    // TooSmall, // user's snippet suggested it, might as well include if needed, but the impl below doesn't use it.
    // Actually the user snippet has `TooSmall` in the enum but doesn't throw it in the example `call`.
    // I will include it to match the snippet.
    TooSmall,
}

impl GraphClient {
    pub fn new() -> Self { Self }

    pub fn call<Req: serde::Serialize, Resp: serde::de::DeserializeOwned>(
        &self,
        query: &str,
        req: &Req,
        out: &mut [u8],
    ) -> Result<Resp, GraphError> {
        let params = postcard::to_allocvec(req).map_err(|_| GraphError::Encode)?;
        let n = crate::sys::sys_graph(query, &params, out).map_err(|e| GraphError::Sys(e.0))?;
        postcard::from_bytes(&out[..n]).map_err(|_| GraphError::Decode)
    }
}
