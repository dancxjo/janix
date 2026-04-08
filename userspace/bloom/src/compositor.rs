use abi::display_driver_protocol::{FbInfoPayload, FB_INFO_PAYLOAD_SIZE};
use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use abi::types::RootWatchEvent;
use stem::syscall::vfs;
use stem::syscall::ChannelHandle;
use stem::syscall::{vm_map, vm_unmap};
use stem::thing::{sys as thingsys, ThingId};

pub struct Symbols {
    pub display_compositor: u64,
    #[allow(dead_code)]
    pub display_role: u64,
    pub display_drv_req: u64,
    pub display_drv_resp: u64,
}

impl Symbols {
    pub fn new() -> Self {
        let display_compositor = thingsys::intern("display.compositor").unwrap_or(0) as u64;
        let display_role = thingsys::intern("display_role").unwrap_or(0) as u64;
        let display_drv_req = thingsys::intern("display_drv_req").unwrap_or(0) as u64;
        let display_drv_resp = thingsys::intern("display_drv_resp").unwrap_or(0) as u64;

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
    pub fd: u32,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: u32,
    pub ptr: *mut u8,
    pub size_bytes: usize,
    pub driver_req: ChannelHandle,
    pub driver_resp: ChannelHandle,
    pub backend: DisplayBackend,
}

#[derive(Debug)]
pub enum CompositorError {
    DiscoveryTimeout,
    InvalidSize,
    MappingFailed,
}

impl CompositorTarget {
    fn probe_bootfb() -> Option<(u32, u32, u32, u32)> {
        let fd = vfs::vfs_open("/dev/fb0", abi::syscall::vfs_flags::O_RDONLY).ok()?;
        let mut payload = FbInfoPayload {
            graph_id: 0,
            width: 0,
            height: 0,
            stride: 0,
            bpp: 0,
            format: 0,
        };
        let buf = unsafe {
            core::slice::from_raw_parts_mut(
                &mut payload as *mut _ as *mut u8,
                FB_INFO_PAYLOAD_SIZE,
            )
        };
        let n = match vfs::vfs_read(fd, buf) {
            Ok(n) => n,
            Err(_) => {
                let _ = vfs::vfs_close(fd);
                return None;
            }
        };
        let _ = vfs::vfs_close(fd);
        if n < FB_INFO_PAYLOAD_SIZE
            || payload.width == 0
            || payload.height == 0
            || payload.stride == 0
        {
            return None;
        }
        Some((
            payload.width,
            payload.height,
            payload.stride,
            payload.format,
        ))
    }

    fn build_bootfb_staging_target(
        width: u32,
        height: u32,
        stride: u32,
        format: u32,
    ) -> Result<Self, CompositorError> {
        use abi::vm::{VmBacking, VmMapReq, VmProt};

        let size = (height as usize).saturating_mul(stride as usize);
        if size == 0 {
            return Err(CompositorError::InvalidSize);
        }

        let fd = thingsys::memfd_create("bloom.bootfb", size)
            .map_err(|_| CompositorError::MappingFailed)?;
        let req = VmMapReq {
            addr_hint: 0,
            len: size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: abi::vm::VmMapFlags::empty(),
            backing: VmBacking::File { fd, offset: 0 },
        };
        let ptr = vm_map(&req)
            .map(|resp| resp.addr as *mut u8)
            .map_err(|_| CompositorError::MappingFailed)?;
        if ptr.is_null() {
            return Err(CompositorError::MappingFailed);
        }

        Ok(Self {
            fd,
            width,
            height,
            stride_bytes: stride,
            format,
            ptr,
            size_bytes: size,
            driver_req: 0,
            driver_resp: 0,
            backend: DisplayBackend::BootFB,
        })
    }

    pub fn map_from_fd(
        fd: u32,
        arg_ports: (ChannelHandle, ChannelHandle),
    ) -> Result<Self, CompositorError> {
        stem::info!("bloom: map_from_fd start fd={}", fd);
        let sym = Symbols::new();
        stem::info!("bloom: symbols loaded");
        let deadline = stem::time::now() + stem::time::Duration::from_millis(2000);

        let mut iters = 0;
        loop {
            iters += 1;
            stem::info!("bloom: loop iter {}", iters);

            let width = thingsys::prop_get_fd(fd, keys::WIDTH).unwrap_or(0) as u32;
            stem::info!("bloom: got width {}", width);

            let height = thingsys::prop_get_fd(fd, keys::HEIGHT).unwrap_or(0) as u32;
            let stride = thingsys::prop_get_fd(fd, keys::STRIDE).unwrap_or(0) as u32;
            let format = thingsys::prop_get_fd(fd, keys::FORMAT).unwrap_or(0) as u32;
            stem::info!(
                "bloom: got properties w={} h={} s={} f={}",
                width,
                height,
                stride,
                format
            );

            if width != 0 && height != 0 && stride != 0 {
                stem::info!("bloom: properties OK, building from config...");
                return Self::build_from_config(&sym, fd, width, height, stride, format, arg_ports);
            }

            if stem::time::now() > deadline {
                stem::info!("bloom: map_from_bytespace loop deadline expired!");
                break;
            }

            stem::info!("bloom: sleeping 50ms");
            stem::sleep_ms(50);
            stem::info!("bloom: woke up from sleep");
        }

        Err(CompositorError::InvalidSize)
    }

    pub fn discover_and_map(
        arg_ports: (ChannelHandle, ChannelHandle),
        timeout_ms: u32,
    ) -> Result<Self, CompositorError> {
        let _ = arg_ports;
        let deadline = stem::time::now() + stem::time::Duration::from_millis(timeout_ms as u64);

        loop {
            if let Some((width, height, stride, format)) = Self::probe_bootfb() {
                return Self::build_bootfb_staging_target(width, height, stride, format);
            }

            if stem::time::now() > deadline {
                crate::log!("bloom: discover_and_map timed out!");
                break;
            }
            stem::sleep_ms(50);
        }

        Err(CompositorError::DiscoveryTimeout)
    }

    fn build_from_config(
        sym: &Symbols,
        fd: u32,
        width: u32,
        height: u32,
        stride: u32,
        format: u32,
        arg_ports: (ChannelHandle, ChannelHandle),
    ) -> Result<Self, CompositorError> {
        crate::log!(
            "compositor fd {} ({}x{} stride={} format={})",
            fd,
            width,
            height,
            stride,
            format
        );

        // Detect backend from property set by Sprout
        let backend = detect_backend(fd);
        crate::log!("display backend: {}", backend.name());

        // Size resolution
        let fallback_size = (height as usize).saturating_mul(stride as usize);
        let info_res = thingsys::stat(fd);
        let size = if let Ok((_, s, _)) = info_res {
            s as usize
        } else {
            fallback_size
        };

        if size == 0 {
            crate::log!("error: resolved size is 0");
            return Err(CompositorError::InvalidSize);
        }

        crate::log!(
            "mapped size={} (source={})",
            size,
            if info_res.is_ok() { "stat" } else { "fallback" }
        );

        // Mapping (updated to use MemFD/vm_map)
        use abi::vm::{VmBacking, VmMapReq, VmProt};
        let req_map = VmMapReq {
            addr_hint: 0,
            len: size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: abi::vm::VmMapFlags::empty(),
            backing: VmBacking::File { fd, offset: 0 },
        };
        let ptr = match vm_map(&req_map) {
            Ok(resp) => resp.addr as *mut u8,
            Err(_) => return Err(CompositorError::MappingFailed),
        };

        if ptr.is_null() {
            return Err(CompositorError::MappingFailed);
        }

        // Port resolution
        let (mut req, mut resp) = arg_ports;

        // If not provided in args, check properties
        if req == 0 || resp == 0 {
            req = thingsys::prop_get_fd(fd, "display_drv_req").unwrap_or(0) as ChannelHandle;
            resp = thingsys::prop_get_fd(fd, "display_drv_resp").unwrap_or(0) as ChannelHandle;
        }

        // If still 0, wait with timeout
        if req == 0 || resp == 0 {
            // Simplified watch wait (bounded)
            let watch = thingsys::watch_subscribe(fd, 0).ok();
            if let Some(w) = watch {
                let mut evt = RootWatchEvent::default();
                for _ in 0..20 {
                    // 20 * 50ms = 1s wait max
                    if thingsys::root_stream_poll(w, &mut evt).is_ok() {
                        if evt.key == sym.display_drv_req {
                            req = evt.value as ChannelHandle;
                        }
                        if evt.key == sym.display_drv_resp {
                            resp = evt.value as ChannelHandle;
                        }
                        if req != 0 && resp != 0 {
                            break;
                        }
                    } else {
                        stem::sleep_ms(50);
                    }
                }
            }
        }

        Ok(Self {
            fd,
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

fn detect_backend(fd: u32) -> DisplayBackend {
    // Check the display_backend property set by Sprout
    let backend_sym = thingsys::prop_get_fd(fd, "display_backend").unwrap_or(0);
    if backend_sym == 0 {
        return DisplayBackend::Unknown;
    }

    // Compare with known backend symbols
    let bootfb_sym = thingsys::intern("BootFB").unwrap_or(0) as u64;
    let virtio_sym = thingsys::intern("VirtIO-GPU").unwrap_or(0) as u64;

    if backend_sym == bootfb_sym {
        DisplayBackend::BootFB
    } else if backend_sym == virtio_sym {
        DisplayBackend::VirtioGpu
    } else {
        DisplayBackend::Unknown
    }
}
