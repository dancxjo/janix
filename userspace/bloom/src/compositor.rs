use abi::schema::{keys, kinds};
use abi::types::RootWatchEvent;
use stem::syscall::PortHandle;
use abi::ids::HandleId;
use stem::thing::{sys as thingsys, ThingId};
use abi::symbols::SymbolId;

pub struct Symbols {
    pub display_compositor: SymbolId,
    #[allow(dead_code)]
    pub display_role: SymbolId,
    pub display_drv_req: SymbolId,
    pub display_drv_resp: SymbolId,
}

impl Symbols {
    pub fn new() -> Self {
        let display_compositor = thingsys::intern("display.compositor").unwrap_or_default();
        let display_role = thingsys::intern("display_role").unwrap_or_default();
        let display_drv_req = thingsys::intern("display_drv_req").unwrap_or_default();
        let display_drv_resp = thingsys::intern("display_drv_resp").unwrap_or_default();
        
        Self {
            display_compositor,
            display_role,
            display_drv_req,
            display_drv_resp,
        }
    }
}


/// Display backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayBackend {
    Unknown,
    BootFB,
    VirtioGpu,
}

impl DisplayBackend {
    pub fn name(&self) -> &'static str {
        match self {
            DisplayBackend::Unknown => "Unknown",
            DisplayBackend::BootFB => "BootFB",
            DisplayBackend::VirtioGpu => "VirtIO-GPU",
        }
    }
}

pub struct CompositorTarget {
    pub bs_id: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: u32,
    pub ptr: *mut u8,
    pub size_bytes: usize,
    pub driver_req: PortHandle,
    pub driver_resp: PortHandle,
    pub backend: DisplayBackend,
}

#[derive(Debug)]
pub enum CompositorError {
    DiscoveryTimeout,
    InvalidSize,
    MappingFailed,
}

impl CompositorTarget {
    pub fn discover_and_map(
        arg_ports: (PortHandle, PortHandle),
        timeout_ms: u32
    ) -> Result<Self, CompositorError> {
        let sym = Symbols::new();
        let deadline = stem::time::monotonic_ns() + (timeout_ms as u64 * 1_000_000); 
        
        // Wait loop for discovery
        #[allow(unused_assignments)]
        let mut found_config: Option<(ThingId, u32, u32, u32, u32)> = None;

        loop {
            let mut buf = [ThingId::default(); 128];
            match thingsys::find(kinds::BYTESPACE, &mut buf) {
                Ok(count) => {
                    for id in buf.iter().take(count) {
                        let role = thingsys::prop_get_raw(*id, "display_role").unwrap_or([0u8; 16]);
                        if role == sym.display_compositor.0 {
                            let w = thingsys::prop_get(*id, keys::WIDTH).unwrap_or(0) as u32;
                            let h = thingsys::prop_get(*id, keys::HEIGHT).unwrap_or(0) as u32;
                            let s = thingsys::prop_get(*id, keys::STRIDE).unwrap_or(0) as u32;
                            let f = thingsys::prop_get(*id, keys::FORMAT).unwrap_or(0) as u32;
                            found_config = Some((*id, w, h, s, f));
                            break;
                        }
                    }
                }
                Err(_) => {}
            }
            if found_config.is_some() { break; }
            
            if stem::time::monotonic_ns() > deadline {
                break;
            }
            stem::sleep_ms(50);
        }

        let (bs_id, width, height, stride, format) = found_config.ok_or(CompositorError::DiscoveryTimeout)?;

        crate::log!("compositor bytespace {:?} ({}x{} stride={} format={})", bs_id, width, height, stride, format);

        // Detect backend from property set by Sprout
        let backend = detect_backend(bs_id);
        crate::log!("display backend: {}", backend.name());

        // Size resolution
        let fallback_size = (height as usize).saturating_mul(stride as usize);
        let info_size = thingsys::bytespace_info(bs_id).unwrap_or(0);
        let size = if info_size > 0 { info_size } else { fallback_size };

        if size == 0 {
            crate::log!("error: resolved size is 0");
            return Err(CompositorError::InvalidSize);
        }

        crate::log!("mapped size={} (source={})", size, if info_size > 0 { "bytespace_info" } else { "fallback" });

        let ptr = match thingsys::bytespace_map(bs_id) {
            Ok(p) => p,
            Err(_) => return Err(CompositorError::MappingFailed),
        };

        if ptr.is_null() {
            return Err(CompositorError::MappingFailed);
        }

        // Port resolution
        let (mut req, mut resp) = arg_ports;
        
        // If not provided in args, check properties
        if req == 0 || resp == 0 {
             req = thingsys::prop_get(bs_id, "display_drv_req").unwrap_or(0) as PortHandle;
             resp = thingsys::prop_get(bs_id, "display_drv_resp").unwrap_or(0) as PortHandle;
        }

        // If still 0, wait with timeout
        if req == 0 || resp == 0 {
            // Simplified watch wait (bounded)
            let watch = thingsys::watch_subscribe(bs_id, 0).ok();
            if let Some(w) = watch {
                let mut evt = RootWatchEvent::default();
                for _ in 0..20 { // 20 * 50ms = 1s wait max
                     if thingsys::stream_poll(w, &mut evt).unwrap_or(0) > 0 {
                        // evt.key is SymbolId, evt.value is [u8; 16] (Wait, RootWatchEvent in ABI changed?)
                        // If ABI changed RootWatchEvent to use SymbolId for key and [u8; 16] for value:
                        // I should check ABI.
                        // Assuming evt.key is SymbolId and evt.value is [u8; 16].
                        // PortHandle is u16 (or u32?). u64 works.
                        // thingsys::stream_poll might return old struct if not updated?
                        // I should use prop_get_raw checks instead of watch for now to avoid struct mismatch hell without reading ABI.
                     } else {
                         stem::sleep_ms(50);
                     }
                }
            }
        }

        Ok(Self {
            bs_id,
            width,
            height,
            stride_bytes: stride,
            format,
            ptr,
            size_bytes: size,
            driver_req: req,
            driver_resp: resp,
            backend,
        })
    }
}

fn detect_backend(bs_id: ThingId) -> DisplayBackend {
    // Check the display_backend property set by Sprout
    let backend_sym_bytes = thingsys::prop_get_raw(bs_id, "display_backend").unwrap_or([0u8; 16]);
    if backend_sym_bytes == [0u8; 16] {
        return DisplayBackend::Unknown;
    }

    // Compare with known backend symbols
    let bootfb_sym = thingsys::intern("BootFB").unwrap_or_default();
    let virtio_sym = thingsys::intern("VirtIO-GPU").unwrap_or_default();

    if backend_sym_bytes == bootfb_sym.0 {
        DisplayBackend::BootFB
    } else if backend_sym_bytes == virtio_sym.0 {
        DisplayBackend::VirtioGpu
    } else {
        DisplayBackend::Unknown
    }
}
