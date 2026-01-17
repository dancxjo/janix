//! Virtio-GPU command structures
#![allow(dead_code)]

// Command types
pub const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: u32 = 0x0100;
pub const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
pub const VIRTIO_GPU_CMD_RESOURCE_UNREF: u32 = 0x0102;
pub const VIRTIO_GPU_CMD_SET_SCANOUT: u32 = 0x0103;
pub const VIRTIO_GPU_CMD_RESOURCE_FLUSH: u32 = 0x0104;
pub const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: u32 = 0x0105;
pub const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
pub const VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING: u32 = 0x0107;

// Response types
pub const VIRTIO_GPU_RESP_OK_NODATA: u32 = 0x1100;
pub const VIRTIO_GPU_RESP_OK_DISPLAY_INFO: u32 = 0x1101;

// Formats
pub const VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM: u32 = 1;
pub const VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM: u32 = 2;
pub const VIRTIO_GPU_FORMAT_R8G8B8A8_UNORM: u32 = 67;
pub const VIRTIO_GPU_FORMAT_R8G8B8X8_UNORM: u32 = 68;

/// Control header for all virtio-gpu commands
#[repr(C, packed)]
pub struct VirtioGpuCtrlHdr {
    pub type_: u32,
    pub flags: u32,
    pub fence_id: u64,
    pub ctx_id: u32,
    pub padding: u32,
}

/// Display info for one scanout
#[repr(C, packed)] 
pub struct VirtioGpuDisplayOne {
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub enabled: u32,
    pub flags: u32,
}

/// Response to GET_DISPLAY_INFO
#[repr(C, packed)]
pub struct VirtioGpuRespDisplayInfo {
    pub hdr: VirtioGpuCtrlHdr,
    pub pmodes: [VirtioGpuDisplayOne; 16],
}

/// Create 2D resource request
#[repr(C, packed)]
pub struct VirtioGpuResourceCreate2d {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub format: u32,
    pub width: u32,
    pub height: u32,
}

/// Attach backing memory entry
#[repr(C, packed)]
pub struct VirtioGpuMemEntry {
    pub addr: u64,
    pub length: u32,
    pub padding: u32,
}

/// Attach backing request
#[repr(C, packed)]
pub struct VirtioGpuResourceAttachBacking {
    pub hdr: VirtioGpuCtrlHdr,
    pub resource_id: u32,
    pub nr_entries: u32,
    // Followed by VirtioGpuMemEntry array
}

/// Set scanout request
#[repr(C, packed)]
pub struct VirtioGpuSetScanout {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub scanout_id: u32,
    pub resource_id: u32,
}

/// Transfer to host request
#[repr(C, packed)]
pub struct VirtioGpuTransferToHost2d {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub offset: u64,
    pub resource_id: u32,
    pub padding: u32,
}

/// Flush resource request
#[repr(C, packed)]
pub struct VirtioGpuResourceFlush {
    pub hdr: VirtioGpuCtrlHdr,
    pub r_x: u32,
    pub r_y: u32,
    pub r_width: u32,
    pub r_height: u32,
    pub resource_id: u32,
    pub padding: u32,
}
