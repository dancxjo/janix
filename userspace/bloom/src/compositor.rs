use abi::schema::{keys, kinds};
use abi::types::RootWatchEvent;
use stem::syscall::PortHandle;
use stem::thing::{sys as thingsys, ThingId};

pub struct Symbols {
    pub display_compositor: u64,
    pub display_role: u64,
    pub display_drv_req: u64,
    pub display_drv_resp: u64,
}

impl Symbols {
    pub fn new() -> Self {
        Self {
            display_compositor: thingsys::intern("display.compositor").unwrap_or(0) as u64,
            display_role: thingsys::intern("display_role").unwrap_or(0) as u64,
            display_drv_req: thingsys::intern("display_drv_req").unwrap_or(0) as u64,
            display_drv_resp: thingsys::intern("display_drv_resp").unwrap_or(0) as u64,
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
        let mut found_config: Option<(ThingId, u32, u32, u32, u32)> = None;

        loop {
            let mut buf = [ThingId(0); 128];
            match thingsys::find(kinds::BYTESPACE, &mut buf) {
                Ok(count) => {
                    for id in buf.iter().take(count) {
                        let role = thingsys::prop_get(*id, "display_role").unwrap_or(0);
                        if role == sym.display_compositor {
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

        crate::log!("compositor bytespace {} ({}x{} stride={} format={})", bs_id.0, width, height, stride, format);

        // Size resolution
        let fallback_size = (height as usize).saturating_mul(stride as usize);
        let info_size = thingsys::bytespace_info(bs_id).unwrap_or(0);
        let size = if info_size > 0 { info_size } else { fallback_size };

        if size == 0 {
            crate::log!("error: resolved size is 0");
            return Err(CompositorError::InvalidSize);
        }

        crate::log!("mapped size={} (source={})", size, if info_size > 0 { "bytespace_info" } else { "fallback" });

        // Mapping (updated to use new kernel allocator via syscall)
        // Wait, main.rs calls bytespace_map logic manually in the old code.
        // `thingsys::bytespace_map(bs_id)` calls the syscall SYS_BYTESPACE_MAP.
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
                        if evt.key == sym.display_drv_req { req = evt.value as PortHandle; }
                        if evt.key == sym.display_drv_resp { resp = evt.value as PortHandle; }
                        if req != 0 && resp != 0 { break; }
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
        })
    }
}
