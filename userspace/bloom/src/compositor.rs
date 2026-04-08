use abi::display_driver_protocol::{FbInfoPayload, FB_INFO_PAYLOAD_SIZE};
use abi::schema::{kinds};
use stem::syscall::vfs;
use stem::syscall::ChannelHandle;
use stem::syscall::{vm_map, vm_unmap};

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
            core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
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

        let fd = stem::syscall::memfd_create("bloom.bootfb", size)
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
        geometry: (u32, u32, u32, u32),
        arg_ports: (ChannelHandle, ChannelHandle),
    ) -> Result<Self, CompositorError> {
        let (width, height, stride, format) = geometry;
        
        if width != 0 && height != 0 && stride != 0 {
            return Self::build_from_config(fd, width, height, stride, format, arg_ports);
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

        // Detect backend (legacy property check removed)
        let backend = DisplayBackend::Unknown; 

        // Size resolution from stat
        let info_res = vfs::vfs_stat(fd); // Use VFS stat instead of thingsys::stat
        let size = if let Ok((_, size, _)) = info_res {
            size as usize
        } else {
            (height as usize).saturating_mul(stride as usize)
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

        // Port resolution: already provided via args or bootstrap
        let (req, resp) = arg_ports;

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

fn detect_backend(_fd: u32) -> DisplayBackend {
    DisplayBackend::Unknown
}
