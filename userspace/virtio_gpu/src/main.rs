#![no_std]
#![no_main]

extern crate alloc;

use abi::device::PCI_IRQ_MODE_MSIX;
use abi::errors::Errno;
use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicUsize, Ordering};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::device::device_enable_msi;
use stem::syscall::{
    device_alloc_dma, device_claim, device_dma_phys, device_irq_subscribe, device_irq_wait,
    device_map_mmio,
};
use stem::thing::sys as thingsys;
use stem::thread;
use stem::{error, info, warn};

mod commands;
mod virtio;
mod virtqueue;

use commands::*;
use virtqueue::Virtqueue;

static IRQ_HANDLE: AtomicUsize = AtomicUsize::new(0);

#[unsafe(link_section = ".thing_manifest")]
#[unsafe(no_mangle)]
#[used]
pub static MANIFEST: ManifestHeader = ManifestHeader {
    magic: MANIFEST_MAGIC,
    kind: ModuleKind::Driver,
    device_kind: *b"dev.display.Gpu\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    version: 1,
    _reserved: 0,
};

/// Virtio GPU driver state
struct VirtioGpu {
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
    resource_id: u32,
    framebuffer: u64, // Virtual address of framebuffer
    fb_phys: u64,     // Physical address

    // Command buffer for sending commands
    cmd_buf: u64,      // Virtual address
    cmd_buf_phys: u64, // Physical address
}

impl VirtioGpu {
    fn new(device_id: u64) -> Result<Self, Errno> {
        let claim_handle = device_claim(device_id)?;
        info!("VIRTIO_GPU: Claimed device, handle={}", claim_handle);

        // Try BAR2 first (virtio-vga config), fall back to BAR0
        let config_bar = match device_map_mmio(claim_handle, 2) {
            Ok(addr) => {
                info!("VIRTIO_GPU: Mapped BAR2 (virtio config) at 0x{:x}", addr);
                addr
            }
            Err(_) => {
                let bar0 = device_map_mmio(claim_handle, 0)?;
                info!("VIRTIO_GPU: Mapped BAR0 at 0x{:x}", bar0);
                bar0
            }
        };

        // Allocate command buffer (1 page for commands + responses)
        let cmd_buf = device_alloc_dma(claim_handle, 1).map_err(|_| Errno::ENOMEM)?;
        let cmd_buf_phys = device_dma_phys(cmd_buf).map_err(|_| Errno::EFAULT)?;

        Ok(Self {
            claim_handle,
            common_cfg: config_bar,
            notify_cfg: config_bar + 0x1000,
            notify_off_multiplier: 4,
            controlq: None,
            display_width: 1024,
            display_height: 768,
            resource_id: 1,
            framebuffer: 0,
            fb_phys: 0,
            cmd_buf,
            cmd_buf_phys,
        })
    }

    fn init_virtio(&mut self) -> Result<(), &'static str> {
        info!("VIRTIO_GPU: Initializing virtio device...");

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
        let features = self.read_common(virtio::VIRTIO_COMMON_DEVICE_FEATURE);
        info!("VIRTIO_GPU: Device features: 0x{:x}", features);

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
            error!("VIRTIO_GPU: Device did not accept features");
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

        info!(
            "VIRTIO_GPU: Device initialized, status=0x{:x}",
            self.read_common(virtio::VIRTIO_COMMON_STATUS)
        );

        Ok(())
    }

    fn setup_controlq(&mut self) -> Result<(), &'static str> {
        let vq_virt =
            device_alloc_dma(self.claim_handle, 4).map_err(|_| "Failed to alloc virtqueue")?;
        let vq_phys = device_dma_phys(vq_virt).map_err(|_| "Failed to get vq phys")?;

        info!(
            "VIRTIO_GPU: Controlq at virt=0x{:x} phys=0x{:x}",
            vq_virt, vq_phys
        );

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
                    warn!("VIRTIO_GPU: Command failed, type=0x{:x}", resp_type);
                    return Err("Command failed");
                }
            }
            core::hint::spin_loop();
        }

        Err("Command timeout")
    }

    fn create_resource_2d(&mut self) -> Result<(), &'static str> {
        info!(
            "VIRTIO_GPU: Creating 2D resource {}x{}",
            self.display_width, self.display_height
        );

        let cmd = VirtioGpuResourceCreate2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id: self.resource_id,
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

    fn attach_backing(&mut self) -> Result<(), &'static str> {
        info!(
            "VIRTIO_GPU: Attaching backing memory @ phys=0x{:x}",
            self.fb_phys
        );

        // Need to send header + 1 memory entry
        #[repr(C, packed)]
        struct AttachCmd {
            hdr: VirtioGpuCtrlHdr,
            resource_id: u32,
            nr_entries: u32,
            entry: VirtioGpuMemEntry,
        }

        let fb_size = (self.display_width * self.display_height * 4) as u32;
        let cmd = AttachCmd {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id: self.resource_id,
            nr_entries: 1,
            entry: VirtioGpuMemEntry {
                addr: self.fb_phys,
                length: fb_size,
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

    fn set_scanout(&mut self) -> Result<(), &'static str> {
        info!(
            "VIRTIO_GPU: Setting scanout 0 to resource {}",
            self.resource_id
        );

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
            r_width: self.display_width,
            r_height: self.display_height,
            scanout_id: 0,
            resource_id: self.resource_id,
        };

        let cmd_bytes = unsafe {
            core::slice::from_raw_parts(
                &cmd as *const _ as *const u8,
                core::mem::size_of::<VirtioGpuSetScanout>(),
            )
        };

        self.send_cmd(cmd_bytes, core::mem::size_of::<VirtioGpuCtrlHdr>())
    }

    fn transfer_to_host(&mut self) -> Result<(), &'static str> {
        let cmd = VirtioGpuTransferToHost2d {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: 0,
            r_y: 0,
            r_width: self.display_width,
            r_height: self.display_height,
            offset: 0,
            resource_id: self.resource_id,
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

    fn flush_resource(&mut self) -> Result<(), &'static str> {
        let cmd = VirtioGpuResourceFlush {
            hdr: VirtioGpuCtrlHdr {
                type_: VIRTIO_GPU_CMD_RESOURCE_FLUSH,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r_x: 0,
            r_y: 0,
            r_width: self.display_width,
            r_height: self.display_height,
            resource_id: self.resource_id,
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

    fn create_framebuffer(&mut self) -> Result<(), &'static str> {
        let fb_size = (self.display_width * self.display_height * 4) as usize;
        let pages = (fb_size + 4095) / 4096;

        self.framebuffer = device_alloc_dma(self.claim_handle, pages)
            .map_err(|_| "Failed to alloc framebuffer")?;
        self.fb_phys = device_dma_phys(self.framebuffer).map_err(|_| "Failed to get fb phys")?;

        info!(
            "VIRTIO_GPU: Framebuffer {}x{} @ virt=0x{:x} phys=0x{:x}",
            self.display_width, self.display_height, self.framebuffer, self.fb_phys
        );

        // Clear framebuffer to a visible color (bright green)
        let fb = self.framebuffer as *mut u32;
        for i in 0..(self.display_width * self.display_height) as usize {
            unsafe { write_volatile(fb.add(i), 0x0000FF00) }; // Green in BGRA
        }

        Ok(())
    }

    fn setup_display(&mut self) -> Result<(), &'static str> {
        // Create framebuffer memory
        self.create_framebuffer()?;

        // Create GPU resource
        self.create_resource_2d()?;

        // Attach framebuffer memory to resource
        self.attach_backing()?;

        // Set this resource as scanout 0
        self.set_scanout()?;

        // Initial transfer + flush
        self.transfer_to_host()?;
        self.flush_resource()?;

        info!("VIRTIO_GPU: Display pipeline ready!");
        Ok(())
    }

    fn flush(&mut self) {
        // Ignore errors during animation loop
        let _ = self.transfer_to_host();
        let _ = self.flush_resource();
    }

    fn configure_msix(&mut self) {
        self.write_common(virtio::VIRTIO_COMMON_MSIX_CONFIG, 0);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_SELECT, 0);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_MSIX_VECTOR, 0);
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    info!("VIRTIO_GPU: Starting driver, arg=0x{:x}", arg);

    // Find the virtio GPU device in the graph
    let device_id = match find_virtio_gpu() {
        Some(id) => id,
        None => {
            error!("VIRTIO_GPU: Device not found in graph");
            stem::syscall::exit(1);
        }
    };

    info!("VIRTIO_GPU: Found device at graph_id={}", device_id);

    // Initialize driver
    let mut gpu = match VirtioGpu::new(device_id) {
        Ok(g) => g,
        Err(e) => {
            error!("VIRTIO_GPU: Failed to init driver: {:?}", e);
            stem::syscall::exit(1);
        }
    };

    // Initialize virtio
    if let Err(e) = gpu.init_virtio() {
        error!("VIRTIO_GPU: Virtio init failed: {}", e);
        stem::syscall::exit(1);
    }

    // Enable MSI-X if available
    match device_enable_msi(gpu.claim_handle, true) {
        Ok(resp) => {
            info!(
                "VIRTIO_GPU: IRQ mode {} vector=0x{:02x}",
                resp.irq_mode, resp.vector
            );
            if resp.irq_mode == PCI_IRQ_MODE_MSIX {
                gpu.configure_msix();
            }
            if let Err(e) = device_irq_subscribe(gpu.claim_handle, 0) {
                warn!("VIRTIO_GPU: device IRQ subscribe failed: {:?}", e);
            } else {
                IRQ_HANDLE.store(gpu.claim_handle, Ordering::Release);
                let _ = thread::spawn(irq_thread);
            }
        }
        Err(e) => warn!("VIRTIO_GPU: MSI enable failed: {:?}", e),
    }

    // Setup the display pipeline
    if let Err(e) = gpu.setup_display() {
        error!("VIRTIO_GPU: Display setup failed: {}", e);
        stem::syscall::exit(1);
    }

    info!("VIRTIO_GPU: Driver initialized, entering demo loop");

    // Simple animation loop to prove it works
    let mut frame = 0u32;
    loop {
        // Draw animated pattern
        let fb = gpu.framebuffer as *mut u32;
        let w = gpu.display_width as usize;
        let h = gpu.display_height as usize;

        for y in 0..h {
            for x in 0..w {
                let offset = ((x + frame as usize) % 100) * 2;
                let color = if (y + offset) % 40 < 20 {
                    0x00FF0000 // Red in BGRA
                } else {
                    0x000000FF // Blue in BGRA  
                };
                unsafe { write_volatile(fb.add(y * w + x), color) };
            }
        }

        // Flush to display
        gpu.flush();

        if frame % 60 == 0 {
            info!("VIRTIO_GPU: Frame {}", frame);
        }

        frame = frame.wrapping_add(1);
        stem::syscall::sleep_ms(16); // ~60fps
    }
}

extern "C" fn irq_thread() -> ! {
    let claim_handle = IRQ_HANDLE.load(Ordering::Acquire);
    loop {
        match device_irq_wait(claim_handle, 0) {
            Ok(count) => info!("VIRTIO_GPU: IRQ fired ({})", count),
            Err(e) => {
                warn!("VIRTIO_GPU: IRQ wait error {:?}", e);
                stem::yield_now();
            }
        }
    }
}

fn find_virtio_gpu() -> Option<u64> {
    let mut buf = [stem::thing::ThingId(0); 1];
    match thingsys::find("dev.display.Gpu", &mut buf) {
        Ok(count) if count > 0 => Some(buf[0].0),
        _ => {
            info!("VIRTIO_GPU: dev.display.Gpu not found");
            None
        }
    }
}
