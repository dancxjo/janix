#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "thingos", feature(restricted_std))]
#![no_main]

extern crate alloc;

use abi::device::PCI_IRQ_MODE_MSIX;
use core::ptr::write_volatile;
use core::sync::atomic::{AtomicUsize, Ordering};
use stem::abi::module_manifest::{MANIFEST_MAGIC, ManifestHeader, ModuleKind};
use stem::device::device_enable_msi;
use stem::syscall::{device_alloc_dma, device_dma_phys, device_irq_subscribe, device_irq_wait};
use stem::thing::sys as thingsys;
use stem::thread;
use stem::{error, info, warn};

use virtio_gpu::{Rect, VirtioGpu};

static IRQ_HANDLE: AtomicUsize = AtomicUsize::new(0);
const DEMO_RESOURCE_ID: u32 = 1;

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
    match device_enable_msi(gpu.claim_handle(), true) {
        Ok(resp) => {
            info!(
                "VIRTIO_GPU: IRQ mode {} vector=0x{:02x}",
                resp.irq_mode, resp.vector
            );
            if resp.irq_mode == PCI_IRQ_MODE_MSIX {
                configure_msix(&gpu);
            }
            if let Err(e) = device_irq_subscribe(gpu.claim_handle(), 0) {
                warn!("VIRTIO_GPU: device IRQ subscribe failed: {:?}", e);
            } else {
                IRQ_HANDLE.store(gpu.claim_handle(), Ordering::Release);
                let _ = thread::spawn(irq_thread);
            }
        }
        Err(e) => warn!("VIRTIO_GPU: MSI enable failed: {:?}", e),
    }

    // Setup the display pipeline
    if let Err(e) = setup_display(&mut gpu) {
        error!("VIRTIO_GPU: Display setup failed: {}", e);
        stem::syscall::exit(1);
    }

    info!("VIRTIO_GPU: Driver initialized, entering demo loop");

    // Get framebuffer for demo
    let framebuffer = match create_demo_framebuffer(&mut gpu) {
        Ok(fb) => fb,
        Err(e) => {
            error!("VIRTIO_GPU: Failed to create demo framebuffer: {}", e);
            stem::syscall::exit(1);
        }
    };

    // Simple animation loop to prove it works
    let mut frame = 0u32;
    let (w, h) = gpu.get_dimensions();
    loop {
        // Draw animated pattern
        let fb = framebuffer as *mut u32;

        for y in 0..h as usize {
            for x in 0..w as usize {
                let offset = ((x + frame as usize) % 100) * 2;
                let color = if (y + offset) % 40 < 20 {
                    0x00FF0000 // Red in BGRA
                } else {
                    0x000000FF // Blue in BGRA
                };
                unsafe { write_volatile(fb.add(y * w as usize + x), color) };
            }
        }

        // Flush to display
        let full_rect = Rect { x: 0, y: 0, w, h };
        let _ = gpu.present_rect(DEMO_RESOURCE_ID, full_rect);

        if frame % 60 == 0 {
            info!("VIRTIO_GPU: Frame {}", frame);
        }

        frame = frame.wrapping_add(1);
        stem::syscall::sleep_ms(16); // ~60fps
    }
}

fn setup_display(gpu: &mut VirtioGpu) -> Result<(), &'static str> {
    // Create GPU resource
    gpu.create_resource_2d(DEMO_RESOURCE_ID)?;

    info!("VIRTIO_GPU: Display pipeline ready!");
    Ok(())
}

fn create_demo_framebuffer(gpu: &mut VirtioGpu) -> Result<u64, &'static str> {
    let (width, height) = gpu.get_dimensions();
    let fb_size = (width * height * 4) as usize;
    let pages = (fb_size + 4095) / 4096;

    let framebuffer =
        device_alloc_dma(gpu.claim_handle(), pages).map_err(|_| "Failed to alloc framebuffer")?;
    let fb_phys = device_dma_phys(framebuffer).map_err(|_| "Failed to get fb phys")?;

    info!(
        "VIRTIO_GPU: Framebuffer {}x{} @ virt=0x{:x} phys=0x{:x}",
        width, height, framebuffer, fb_phys
    );

    // Clear framebuffer to a visible color (bright green)
    let fb = framebuffer as *mut u32;
    for i in 0..(width * height) as usize {
        unsafe { write_volatile(fb.add(i), 0x0000FF00) }; // Green in BGRA
    }

    // Attach framebuffer memory to resource
    gpu.attach_backing(DEMO_RESOURCE_ID, fb_phys, fb_size, width * 4)?;

    // Set this resource as scanout 0
    gpu.set_scanout(DEMO_RESOURCE_ID, width, height)?;

    // Initial transfer + flush
    let full_rect = Rect {
        x: 0,
        y: 0,
        w: width,
        h: height,
    };
    gpu.present_rect(DEMO_RESOURCE_ID, full_rect)?;

    Ok(framebuffer)
}

fn configure_msix(gpu: &VirtioGpu) {
    // MSI-X configuration is handled internally by the virtio device
    // The GPU struct doesn't expose the raw MMIO regions for security
    // This function is kept as a placeholder for future enhancements
    let _ = gpu;
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
    let mut buf = [stem::thing::ThingId::default(); 1];
    match thingsys::find("dev.display.Gpu", &mut buf) {
        Ok(count) if count > 0 => Some(buf[0].to_u64_lossy()),
        _ => {
            info!("VIRTIO_GPU: dev.display.Gpu not found");
            None
        }
    }
}
