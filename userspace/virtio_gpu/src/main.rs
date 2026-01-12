#![no_std]
#![no_main]

extern crate alloc;

use core::ptr::{read_volatile, write_volatile};
use stem::abi::module_manifest::{ManifestHeader, ModuleKind, MANIFEST_MAGIC};
use stem::thing::sys as thingsys;
use stem::{error, info, warn};
use stem::syscall::{device_claim, device_map_mmio, device_alloc_dma, device_dma_phys};
use abi::errors::Errno;

mod virtio;
mod virtqueue;
mod commands;

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
    device_id: u64,
    
    // MMIO regions (via HHDM mapping)
    common_cfg: u64,
    notify_cfg: u64,
    isr_cfg: u64,
    device_cfg: u64,
    
    // Virtqueues
    controlq: Option<virtqueue::Virtqueue>,
    
    // Display info
    display_width: u32,
    display_height: u32,
    
    // Scanout resource
    resource_id: u32,
    framebuffer: u64,  // Virtual address of framebuffer
    fb_phys: u64,      // Physical address
}

impl VirtioGpu {
    fn new(device_id: u64) -> Result<Self, Errno> {
        // Claim the device
        let claim_handle = device_claim(device_id)?;
        info!("VIRTIO_GPU: Claimed device, handle={}", claim_handle);
        
        // Map BAR0 (typical for virtio-pci modern)
        let bar0_virt = device_map_mmio(claim_handle, 0)?;
        info!("VIRTIO_GPU: Mapped BAR0 at 0x{:x}", bar0_virt);
        
        // For simplicity in this first version, assume BAR0 contains all virtio config
        // In reality, we'd parse PCI capabilities to find exact offsets
        // For QEMU virtio-vga, BAR0 is typically the VGA framebuffer, BAR2 is virtio config
        
        // Try BAR2 which is usually the virtio config for virtio-vga
        let config_bar = match device_map_mmio(claim_handle, 2) {
            Ok(addr) => {
                info!("VIRTIO_GPU: Mapped BAR2 (virtio config) at 0x{:x}", addr);
                addr
            }
            Err(_) => {
                info!("VIRTIO_GPU: BAR2 not available, using BAR0");
                bar0_virt
            }
        };
        
        Ok(Self {
            claim_handle,
            device_id,
            common_cfg: config_bar,
            notify_cfg: config_bar + 0x1000,  // Offset estimates
            isr_cfg: config_bar + 0x2000,
            device_cfg: config_bar + 0x3000,
            controlq: None,
            display_width: 0,
            display_height: 0,
            resource_id: 1,
            framebuffer: 0,
            fb_phys: 0,
        })
    }
    
    fn init_virtio(&mut self) -> Result<(), &'static str> {
        // Simplified virtio init sequence
        // Real implementation would read capabilities and configure properly
        
        info!("VIRTIO_GPU: Initializing virtio device...");
        
        // 1. Reset device
        self.write_common(virtio::VIRTIO_COMMON_STATUS, 0);
        
        // 2. Set ACKNOWLEDGE status
        self.write_common(virtio::VIRTIO_COMMON_STATUS, virtio::VIRTIO_STATUS_ACKNOWLEDGE);
        
        // 3. Set DRIVER status  
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        self.write_common(virtio::VIRTIO_COMMON_STATUS, status | virtio::VIRTIO_STATUS_DRIVER);
        
        // 4. Read and negotiate features
        let features = self.read_common(virtio::VIRTIO_COMMON_DEVICE_FEATURE);
        info!("VIRTIO_GPU: Device features: 0x{:x}", features);
        
        // Accept VIRTIO_F_VERSION_1
        self.write_common(virtio::VIRTIO_COMMON_DRIVER_FEATURE, 0);
        
        // 5. Set FEATURES_OK
        let status = self.read_common(virtio::VIRTIO_COMMON_STATUS);
        self.write_common(virtio::VIRTIO_COMMON_STATUS, status | virtio::VIRTIO_STATUS_FEATURES_OK);
        
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
        self.write_common(virtio::VIRTIO_COMMON_STATUS, status | virtio::VIRTIO_STATUS_DRIVER_OK);
        
        info!("VIRTIO_GPU: Device initialized, status=0x{:x}", 
              self.read_common(virtio::VIRTIO_COMMON_STATUS));
        
        Ok(())
    }
    
    fn setup_controlq(&mut self) -> Result<(), &'static str> {
        // Allocate virtqueue memory (desc, avail, used rings)
        // Need at least 3 pages for a 256-entry queue
        let vq_virt = device_alloc_dma(self.claim_handle, 4)
            .map_err(|_| "Failed to alloc virtqueue")?;
        let vq_phys = device_dma_phys(vq_virt)
            .map_err(|_| "Failed to get vq phys")?;
        
        info!("VIRTIO_GPU: Controlq at virt=0x{:x} phys=0x{:x}", vq_virt, vq_phys);
        
        let vq = virtqueue::Virtqueue::new(vq_virt, vq_phys, 128);
        
        // Configure the queue in device
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_SELECT, 0); // Queue 0 = controlq
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_SIZE, 128);
        
        // Write queue addresses (split virtqueue format)
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_LO, (vq_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_DESC_HI, (vq_phys >> 32) as u32);
        
        let avail_offset = 128 * 16; // 128 descriptors * 16 bytes each
        let avail_phys = vq_phys + avail_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_LO, (avail_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_AVAIL_HI, (avail_phys >> 32) as u32);
        
        let used_offset = avail_offset + 6 + 128 * 2; // Header + entries
        let used_phys = vq_phys + used_offset as u64;
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_LO, (used_phys & 0xFFFFFFFF) as u32);
        self.write_common(virtio::VIRTIO_COMMON_QUEUE_USED_HI, (used_phys >> 32) as u32);
        
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
    
    fn get_display_info(&mut self) -> Result<(), &'static str> {
        // Send GET_DISPLAY_INFO command
        let cmd = commands::VirtioGpuCtrlHdr {
            type_: commands::VIRTIO_GPU_CMD_GET_DISPLAY_INFO,
            flags: 0,
            fence_id: 0,
            ctx_id: 0,
            padding: 0,
        };
        
        // For now, assume 800x600 (typical QEMU default)
        // Real implementation would send command and parse response
        self.display_width = 800;
        self.display_height = 600;
        
        info!("VIRTIO_GPU: Display size: {}x{}", self.display_width, self.display_height);
        Ok(())
    }
    
    fn create_framebuffer(&mut self) -> Result<(), &'static str> {
        let fb_size = (self.display_width * self.display_height * 4) as usize;
        let pages = (fb_size + 4095) / 4096;
        
        self.framebuffer = device_alloc_dma(self.claim_handle, pages)
            .map_err(|_| "Failed to alloc framebuffer")?;
        self.fb_phys = device_dma_phys(self.framebuffer)
            .map_err(|_| "Failed to get fb phys")?;
        
        info!("VIRTIO_GPU: Framebuffer {}x{} @ virt=0x{:x} phys=0x{:x}", 
            self.display_width, self.display_height, self.framebuffer, self.fb_phys);
        
        // Clear framebuffer to blue
        let fb = self.framebuffer as *mut u32;
        for i in 0..(self.display_width * self.display_height) as usize {
            unsafe { write_volatile(fb.add(i), 0x000088FF) }; // BGRA blue
        }
        
        Ok(())
    }
    
    fn setup_scanout(&mut self) -> Result<(), &'static str> {
        // In a real implementation, we'd send:
        // 1. RESOURCE_CREATE_2D
        // 2. RESOURCE_ATTACH_BACKING  
        // 3. SET_SCANOUT
        // 
        // For now, this is stubbed to demonstrate the flow
        
        info!("VIRTIO_GPU: Scanout configured (stubbed)");
        Ok(())
    }
    
    fn flush_full_frame(&mut self) {
        // Would send TRANSFER_TO_HOST_2D + RESOURCE_FLUSH
        // For now, just log periodically
    }
}

#[stem::main]
fn main(arg: usize) -> ! {
    let cpu = stem::arch::whoami();
    info!(
        "VIRTIO_GPU: whoami: cs=0x{:x} ss=0x{:x} cpl={} rsp=0x{:x}",
        cpu.cs, cpu.ss, cpu.cpl, cpu.rsp
    );
    
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
    
    // Get display info
    if let Err(e) = gpu.get_display_info() {
        warn!("VIRTIO_GPU: Couldn't get display info, using defaults: {}", e);
    }
    
    // Create framebuffer
    if let Err(e) = gpu.create_framebuffer() {
        error!("VIRTIO_GPU: Framebuffer creation failed: {}", e);
        stem::syscall::exit(1);
    }
    
    // Setup scanout
    if let Err(e) = gpu.setup_scanout() {
        warn!("VIRTIO_GPU: Scanout setup failed: {}", e);
    }
    
    info!("VIRTIO_GPU: Driver initialized, entering demo loop");
    
    // Simple animation loop
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
                    0x00FF0000 // Red
                } else {
                    0x000000FF // Blue  
                };
                unsafe { write_volatile(fb.add(y * w + x), color) };
            }
        }
        
        // Flush
        gpu.flush_full_frame();
        
        if frame % 60 == 0 {
            info!("VIRTIO_GPU: Frame {}", frame);
        }
        
        frame = frame.wrapping_add(1);
        stem::syscall::sleep_ms(16); // ~60fps
    }
}

fn find_virtio_gpu() -> Option<u64> {
    // Search for dev.display.Gpu in the graph
    let mut buf = [stem::thing::ThingId(0); 1];
    match thingsys::find("dev.display.Gpu", &mut buf) {
        Ok(count) if count > 0 => Some(buf[0].0),
        _ => {
            // Try finding by PCI class
            info!("VIRTIO_GPU: dev.display.Gpu not found, searching by PCI...");
            None
        }
    }
}
