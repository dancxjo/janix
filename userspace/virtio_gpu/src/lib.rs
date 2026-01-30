//! VirtIO GPU driver library
//!
//! This module provides a reusable VirtioGpu driver that can be used by
//! display_virtio_gpu or as a standalone program.

#![no_std]

extern crate alloc;

use abi::errors::Errno;
use core::ptr::{read_volatile, write_volatile};

pub mod commands;
pub mod virtio;
pub mod virtqueue;

pub use commands::*;
pub use virtqueue::Virtqueue;

/// Rectangle for partial updates
#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Virtio GPU driver state
pub struct VirtioGpu {
    claim_handle: usize,

    // MMIO regions
    common_cfg: u64,
    notify_cfg: u64,
    notify_off_multiplier: u32,

    // Virtqueues
    controlq: Option<Virtqueue>,

    // Display info
    display_width: u32,
    display_height: u32,

    // Resource tracking
    framebuffer: u64, // Virtual address of framebuffer (for standalone mode)
    fb_phys: u64,     // Physical address
    fb_size: usize,   // Size in bytes
    fb_stride: u32,   // Stride in bytes for offset calculation

    // Command buffer for sending commands
    cmd_buf: u64,      // Virtual address
    cmd_buf_phys: u64, // Physical address

    // Track if using external bytespace (display driver mode)
    external_backing: bool,
}

impl VirtioGpu {
    /// Create a new VirtioGpu driver instance
    pub fn new(device_id: u64) -> Result<Self, Errno> {
        use abi::ids::HandleId;
        use abi::schema::keys;
        use stem::syscall::{device_alloc_dma, device_claim, device_dma_phys, device_map_mmio};
        use stem::thing::ThingId;
        use stem::thing::sys as thingsys;

        let claim_handle = device_claim(device_id)?;
        let gpu_node = ThingId::from_u64(device_id);

        // Read VirtIO capability offsets from graph properties (set by kernel PCI enumeration)
        let common_bar = thingsys::prop_get(gpu_node, keys::VIRTIO_COMMON_BAR).unwrap_or(0) as usize;
        let common_offset = thingsys::prop_get(gpu_node, keys::VIRTIO_COMMON_OFFSET).unwrap_or(0);
        let notify_bar = thingsys::prop_get(gpu_node, keys::VIRTIO_NOTIFY_BAR).unwrap_or(0) as usize;
        let notify_offset = thingsys::prop_get(gpu_node, keys::VIRTIO_NOTIFY_OFFSET).unwrap_or(0);
        let notify_multiplier = thingsys::prop_get(gpu_node, keys::VIRTIO_NOTIFY_MULTIPLIER).unwrap_or(4) as u32;

        stem::info!(
            "virtio_gpu: caps from graph - common BAR{} off=0x{:x}, notify BAR{} off=0x{:x} mult={}",
            common_bar, common_offset, notify_bar, notify_offset, notify_multiplier
        );

        // Map the BAR containing common config
        let common_bar_base = device_map_mmio(claim_handle, common_bar)?;
        let common_cfg = common_bar_base + common_offset;

        // Map notify BAR (may be same as common BAR)  
        let notify_cfg = if notify_bar == common_bar {
            common_bar_base + notify_offset
        } else {
            let notify_bar_base = device_map_mmio(claim_handle, notify_bar)?;
            notify_bar_base + notify_offset
        };

        // Allocate command buffer (1 page for commands + responses)
        let cmd_buf = device_alloc_dma(claim_handle, 1).map_err(|_| Errno::ENOMEM)?;
        let cmd_buf_phys = device_dma_phys(cmd_buf).map_err(|_| Errno::EFAULT)?;

        Ok(Self {
            claim_handle,
            common_cfg,
            notify_cfg,
            notify_off_multiplier: notify_multiplier,
            controlq: None,
            display_width: 1024,
            display_height: 768,
            framebuffer: 0,
            fb_phys: 0,
            fb_size: 0,
            fb_stride: 0,
            cmd_buf,
            cmd_buf_phys,
            external_backing: false,
        })
    }

    /// Initialize the virtio device
    pub fn init_virtio(&mut self) -> Result<(), &'static str> {
        // 1. Reset device
        self.write_common(virtio::VIRTIO_COMMON_STATUS, 0);

        // 2. Set ACKNOWLEDGE status
        self.write_common(
            virtio::VIRTIO_COMMON_STATUS,
            virtio::VIRTIO_STATUS_ACKNOWLEDGE,
        );

        // 3. Set DRIVER status
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        self.write_common(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_DRIVER,
        );

        // 4. Read and negotiate features
        let _features = self.read_common(virtio::VIRTIO_COMMON_DEVICE_FEATURE);

        // Accept basic features
        self.write_common(virtio::VIRTIO_COMMON_DRIVER_FEATURE, 0);

        // 5. Set FEATURES_OK
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        self.write_common(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_FEATURES_OK,
        );

        // 6. Verify FEATURES_OK
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        if (status & virtio::VIRTIO_STATUS_FEATURES_OK) == 0 {
            return Err("Features not accepted");
        }

        // 7. Setup virtqueues
        self.setup_controlq()?;

        // 8. Set DRIVER_OK
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        self.write_common(
            virtio::VIRTIO_COMMON_STATUS,
            status | virtio::VIRTIO_STATUS_DRIVER_OK,
        );

        Ok(())
    }

    /// Get display dimensions
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.display_width, self.display_height)
    }

    /// Set display dimensions (must be called before creating resource)
    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.display_width = width;
        self.display_height = height;
    }

    /// Create a 2D resource with current dimensions
    pub fn create_resource_2d(&mut self, resource_id: u32) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceCreate2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            format: VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM, // XRGB8888
            width: self.display_width,
            height: self.display_height,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceCreate2d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Attach backing memory to the resource
    pub fn attach_backing(
        &mut self,
        resource_id: u32,
        phys_addr: u64,
        size: usize,
        stride: u32,
    ) -> Result<(), &'static str> {
        self.fb_phys = phys_addr;
        self.fb_size = size;
        self.fb_stride = stride;
        self.external_backing = true;

        // Need to send header + 1 memory entry
        #[repr(C, packed)]
        struct AttachCmd {
            hdr: VirtioGpuCtrlHdr,
            resource_id: u32,
            nr_entries: u32,
            entry: VirtioGpuMemEntry,
        }

        let cmd = AttachCmd {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id,
            nr_entries: 1,
            entry: VirtioGpuMemEntry {
                addr: phys_addr,
                length: size as u32,
                padding: 0,
            },
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<AttachCmd>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Set the scanout to use the resource
    pub fn set_scanout(
        &mut self,
        resource_id: u32,
        width: u32,
        height: u32,
    ) -> Result<(), &'static str> {
        let cmd = VirtioGpuSetScanout {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_SET_SCANOUT,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: 0,
            r_y: 0,
            r_width: width,
            r_height: height,
            scanout_id: 0,
            resource_id,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuSetScanout>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Transfer a rectangle from backing memory to host
    pub fn transfer_to_host(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        // Calculate byte offset into backing memory for this rectangle
        // Format is BGRA32 (4 bytes per pixel)
        const BPP: u32 = 4;
        let offset = (rect.y as u64) * (self.fb_stride as u64) + (rect.x as u64) * (BPP as u64);
        
        let cmd = VirtioGpuTransferToHost2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: rect.x,
            r_y: rect.y,
            r_width: rect.w,
            r_height: rect.h,
            offset,
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuTransferToHost2d>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Flush a rectangle to display
    pub fn flush_resource(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceFlush {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_FLUSH,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: rect.x,
            r_y: rect.y,
            r_width: rect.w,
            r_height: rect.h,
            resource_id,
            padding: 0,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuResourceFlush>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    /// Transfer and flush a rectangle (convenience method)
    pub fn present_rect(&mut self, resource_id: u32, rect: Rect) -> Result<(), &'static str> {
        self.transfer_to_host(resource_id, rect)?;
        self.flush_resource(resource_id, rect)
    }

    /// Flush the union of multiple rectangles as a single operation.
    ///
    /// Computes the bounding box (union) of all provided rects, clamps it to bounds,
    /// and issues a single flush command. Returns Ok(()) even if rects is empty.
    pub fn flush_union(
        &mut self,
        resource_id: u32,
        rects: &[Rect],
        bounds: (u32, u32),
    ) -> Result<(), &'static str> {
        if rects.is_empty() {
            return Ok(());
        }

        // Compute union of all rects
        let mut union = rects[0];
        for &r in &rects[1..] {
            let x1 = union.x.min(r.x);
            let y1 = union.y.min(r.y);
            let x2 = (union.x + union.w).max(r.x + r.w);
            let y2 = (union.y + union.h).max(r.y + r.h);
            union = Rect {
                x: x1,
                y: y1,
                w: x2.saturating_sub(x1),
                h: y2.saturating_sub(y1),
            };
        }

        // Clamp to bounds
        let x = union.x.min(bounds.0);
        let y = union.y.min(bounds.1);
        let max_w = bounds.0.saturating_sub(x);
        let max_h = bounds.1.saturating_sub(y);
        let clamped = Rect {
            x,
            y,
            w: union.w.min(max_w),
            h: union.h.min(max_h),
        };

        // Skip empty rect
        if clamped.w == 0 || clamped.h == 0 {
            return Ok(());
        }

        self.flush_resource(resource_id, clamped)
    }

    // === Internal methods ===

    fn setup_controlq(&mut self) -> Result<(), &'static str> {
        use stem::syscall::{device_alloc_dma, device_dma_phys};

        let vq_virt =
            device_alloc_dma(self.claim_handle, 4).map_err(|_| "Failed to alloc virtqueue")?;
        let vq_phys = device_dma_phys(vq_virt).map_err(|_| "Failed to get vq phys")?;

        let vq = Virtqueue::new(vq_virt, vq_phys, 128);

        // Configure the queue in device
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_SELECT, 0);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_SIZE, 128);

        // Write queue addresses
        self.write_common(
            virtio::VIRTIO_COMMON_QUEUE_DESC_LO,
            (vq_phys & 0xFFFFFFFF) as u32,
        );
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_HI, (vq_phys >> 32) as u32);

        let avail_offset = 128 * 16;
        let avail_phys = vq_phys + avail_offset as u64;
        self.write_common(
            virtio::VIRTIO_COMMON_QUEUE_AVAIL_LO,
            (avail_phys & 0xFFFFFFFF) as u32,
        );
        self.write_common(
            virtio::VIRTIO_COMMON_QUEUE_AVAIL_HI,
            (avail_phys >> 32) as u32,
        );

        let used_offset = avail_offset + 6 + 128 * 2;
        let used_phys = vq_phys + used_offset as u64;
        self.write_common(
            virtio::VIRTIO_COMMON_QUEUE_USED_LO,
            (used_phys & 0xFFFFFFFF) as u32,
        );
        self.write_common(
            virtio::VIRTIO_COMMON_QUEUE_USED_HI,
            (used_phys >> 32) as u32,
        );

        // Enable the queue
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_ENABLE, 1);

        self.controlq = Some(vq);
        Ok(())
    }

    fn read_common(&self, offset: u32) -> u32 {
        unsafe { read_volatile((self.common_cfg + offset as u64) as *const u32) }
    }

    fn write_common(&self, offset: u32, value: u32) {
        unsafe { write_volatile((self.common_cfg + offset as u64) as *mut u32, value) }
    }

    fn notify_queue(&self, queue_idx: u16) {
        // Write queue index to notify register
        let notify_addr = self.notify_cfg + (queue_idx as u64 * self.notify_off_multiplier as u64);
        unsafe { write_volatile(notify_addr as *mut u16, queue_idx) }
    }

    /// Send a command and wait for response
    fn send_cmd(&mut self, cmd: &[u8], resp_size: usize) -> Result<(), &'static str> {
        // Copy command to DMA buffer
        let cmd_ptr = self.cmd_buf as *mut u8;
        unsafe {
            for (i, byte) in cmd.iter().enumerate() {
                write_volatile(cmd_ptr.add(i), *byte);
            }
        }

        // Response goes after command
        let resp_offset = ((cmd.len() + 15) / 16) * 16; // Align to 16 bytes
        let resp_phys = self.cmd_buf_phys + resp_offset as u64;

        // Get mutable ref to controlq, add buffer, then release borrow
        {
            let vq = self.controlq.as_mut().ok_or("No controlq")?;

            // Add buffer chain: command (read by device), response (written by device)
            let bufs = [
                (self.cmd_buf_phys, cmd.len() as u32, false),
                (resp_phys, resp_size as u32, true),
            ];

            vq.add_buffer(&bufs).ok_or("Queue full")?;
        }

        // Notify device (no borrow conflict now)
        self.notify_queue(0);

        // Wait for response (poll used ring)
        for _ in 0..1000 {
            let completed = {
                let vq = self.controlq.as_mut().ok_or("No controlq")?;
                vq.poll_used().is_some()
            };

            if completed {
                // Read response type safely from packed struct
                let resp_ptr = (self.cmd_buf + resp_offset as u64) as *const u8;
                let resp_type = unsafe {
                    let type_bytes: [u8; 4] = [
                        *resp_ptr,
                        *resp_ptr.add(1),
                        *resp_ptr.add(2),
                        *resp_ptr.add(3),
                    ];
                    u32::from_le_bytes(type_bytes)
                };

                if resp_type >= VIRTIO_GPU_RESP_OK_NODATA {
                    return Ok(());
                } else {
                    return Err("Command failed");
                }
            }
            core::hint::spin_loop();
        }

        Err("Command timeout")
    }

    /// Get claim handle for device operations
    pub fn claim_handle(&self) -> usize {
        self.claim_handle
    }
}
