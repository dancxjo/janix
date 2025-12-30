pub mod framebuffer;

/// Description of a framebuffer discovered by architecture-specific probing.
#[derive(Debug)]
pub struct DiscoveredFramebuffer {
    pub lfb_phys: u64,
    pub size: u64,
    /// Width/height/stride/format; addr is physical (or 0) at probe time.
    pub info: abi::wire::machine::FbGetInfoResp,
}
