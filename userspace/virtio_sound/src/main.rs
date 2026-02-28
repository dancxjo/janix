#![no_std]
#![no_main]

extern crate alloc;

mod spec;

use abi::ids::HandleId;
use abi::schema::kinds;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::size_of;
use spec::*;
use stem::syscall::{device_alloc_dma, device_dma_phys};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{error, info, warn};
use virtio::device::VirtioDevice;

// Device driver for VirtIO Sound
//
// Queue Layout:
// 0: Control Queue - Device commands (INFO, PARAM, START, STOP)
// 1: Event Queue - Device notifications (JACK, PCM PERIOD, XRUN)
// 2: TX Queue - PCM playback data
// 3: RX Queue - PCM capture data (unused)

const QUEUE_SIZE: u16 = 64;

#[stem::main]
fn main(_arg: usize) -> ! {
    info!("SND: Starting VirtIO Sound Driver...");

    // 1. Find the device
    let mut dev_buf = [ThingId::default(); 1];
    let count = match thingsys::find(kinds::DEV_SOUND, &mut dev_buf) {
        Ok(c) => c,
        Err(e) => {
            error!("SND: Failed to find device: {:?}", e);
            loop {
                stem::yield_now();
            }
        }
    };

    if count == 0 {
        error!("SND: No VirtIO sound device found");
        loop {
            stem::yield_now();
        }
    }

    let device_id = dev_buf[0].to_u64_lossy();
    info!("SND: Found device at ID {}", device_id);

    // 2. Initialize Hardware
    let mut driver = match VirtioDevice::new(device_id) {
        Ok(d) => d,
        Err(e) => {
            error!("SND: Failed to claim device: {:?}", e);
            loop {
                stem::yield_now();
            }
        }
    };

    // Negotiate features
    if let Err(e) = driver.init(VIRTIO_SND_F_CTLS) {
        error!("SND: Failed to init device: {}", e);
        loop {
            stem::yield_now();
        }
    }

    // Setup queues
    for q in 0..4 {
        if let Err(e) = driver.setup_queue(q, QUEUE_SIZE) {
            error!("SND: Failed to setup queue {}: {}", q, e);
            loop {
                stem::yield_now();
            }
        }
    }

    driver.driver_ok();
    info!("SND: Device initialized and active");

    // 3. Populate Event Queue (Queue 1)
    // We need to provide buffers for the device to write events into.
    populate_event_queue(&mut driver);

    // 4. Query PCM Info (Control Queue 0)
    let stream_id = match find_output_stream(&mut driver) {
        Some(id) => id,
        None => {
            error!("SND: No output stream found");
            loop {
                stem::yield_now();
            }
        }
    };
    info!("SND: Using Stream ID {}", stream_id);

    // 5. Configure Stream
    configure_stream(&mut driver, stream_id);

    // 6. Start Playback
    send_pcm_command(&mut driver, VIRTIO_SND_R_PCM_START, stream_id);
    info!("SND: Playback started");

    // 7. Receive Loop (Port + TX Queue)
    // Create a port to receive PCM data from userspace apps
    // 64KB buffer (~700ms of audio) to prevent underruns
    let (write_handle, read_handle) =
        stem::syscall::port::port_create(65536).expect("Failed to create port");

    // Publish port handle (quick hack: print it for now, usually publish to graph)
    info!(
        "SND: Listening for audio on port handles: W={} R={}",
        write_handle, read_handle
    );
    // Publish to the device node so 'beeper' can find it
    thingsys::prop_set(
        ThingId::from_u64(device_id),
        abi::schema::keys::WRITE_PORT_HANDLE,
        write_handle as u64,
    )
    .ok();

    let mut buf = [0u8; 4096]; // Max packet size (matched to beeper)
    let dma_dev_handle = driver.claim_handle(); // Pre-fetch handle

    let mut underruns_total: u64 = 0;
    let mut last_status_update = 0;

    loop {
        // 1. Process Events
        if process_event_queue(&mut driver) {
            underruns_total += 1;
            warn!("SND: PCM Underrun detected by device!");
        }

        // 2. Recycle TX Descriptors (CRITICAL: Free up space in ring!)
        process_tx_queue(&mut driver);

        // 3. Update Status Properties (every ~100ms)
        let now = stem::time::monotonic_ns();
        if now - last_status_update > 100_000_000 {
            use abi::schema::keys::*;

            let port_len = stem::syscall::port::port_len(read_handle).unwrap_or(0);
            let port_cap = stem::syscall::port::port_capacity(read_handle).unwrap_or(1);

            thingsys::prop_set(
                ThingId::from_u64(device_id),
                SOUND_BUFFERED_FRAMES,
                (port_len / 4) as u64,
            )
            .ok();
            thingsys::prop_set(
                ThingId::from_u64(device_id),
                SOUND_FREE_FRAMES,
                ((port_cap - port_len) / 4) as u64,
            )
            .ok();
            thingsys::prop_set(
                ThingId::from_u64(device_id),
                SOUND_UNDERRUNS,
                underruns_total,
            )
            .ok();
            last_status_update = now;
        }

        // 4. Process Audio Data
        match stem::syscall::port::port_recv(read_handle, &mut buf) {
            Ok(len) if len > 0 => {
                match device_alloc_dma(dma_dev_handle, 2) {
                    // 8KB pages (Need >4KB for 4K data + header)
                    Ok(dma_addr) => {
                        let phys = device_dma_phys(dma_addr).unwrap();
                        let ptr = dma_addr as *mut u8;

                        // Prepare Header
                        let hdr = VirtioSndPcmXfer { stream_id };
                        let hdr_size = size_of::<VirtioSndPcmXfer>();

                        unsafe {
                            *(ptr as *mut VirtioSndPcmXfer) = hdr;
                            let data_ptr = ptr.add(hdr_size);
                            core::ptr::copy_nonoverlapping(buf.as_ptr(), data_ptr, len);
                        }

                        loop {
                            let added = {
                                let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
                                q.add_buffer_single(phys, (hdr_size + len) as u32, false)
                                    .is_some()
                            };

                            if added {
                                driver.notify_queue(VIRTIO_SND_VQ_TX);
                                stem::yield_now(); // Let lower-priority tasks run
                                break;
                            } else {
                                // Queue full. Poll for completions and yield.
                                process_tx_queue(&mut driver);
                                stem::yield_now();
                            }
                        }
                    }
                    Err(_) => {
                        warn!("SND: DMA alloc failed, dropping frame");
                    }
                }
            }
            _ => {
                stem::yield_now();
            }
        }
    }
}

fn process_tx_queue(driver: &mut VirtioDevice) {
    let q = driver.queue_mut(VIRTIO_SND_VQ_TX).unwrap();
    while let Some((_desc_id, _len)) = q.poll_used() {
        // Recycle buffer?
        // In v0 we are leaking DMA, so we just acknowledge the descriptor recycle.
        // Virtqueue::poll_used automatically frees the descriptor chain back to the available pool.
        // info!("SND: Recycled TX desc {}", _desc_id); // Uncomment for verbose debug
    }
}

fn populate_event_queue(driver: &mut VirtioDevice) {
    let dma_dev = driver.claim_handle();
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        // Populate with 8 buffers
        for _ in 0..8 {
            let size = size_of::<VirtioSndEvent>();
            let dma = device_alloc_dma(dma_dev, 1).unwrap(); // 4KB
            let phys = device_dma_phys(dma).unwrap();
            // Device writes to this buffer, so write=true
            q.add_buffer_single(phys, size as u32, true);
        }
    }
    driver.notify_queue(VIRTIO_SND_VQ_EVENT);
}

fn process_event_queue(driver: &mut VirtioDevice) -> bool {
    let dma_dev = driver.claim_handle();
    let mut needs_notify = false;
    let mut underrun_seen = false;
    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_EVENT).unwrap();
        while let Some((desc_id, _len)) = q.poll_used() {
            // The buffer contains a VirtioSndEvent
            // For now, we don't bother reading the DMA buffer because any event
            // on the PCM stream is likely an xrun in this simple driver.
            // But let's be technically correct if possible.
            underrun_seen = true;

            // Recycle buffer
            let size = core::mem::size_of::<VirtioSndEvent>();
            let dma = device_alloc_dma(dma_dev, 1).unwrap();
            let phys = device_dma_phys(dma).unwrap();
            q.add_buffer_single(phys, size as u32, true);
            needs_notify = true;
        }
    }
    if needs_notify {
        driver.notify_queue(VIRTIO_SND_VQ_EVENT);
    }
    underrun_seen
}

fn send_pcm_command(driver: &mut VirtioDevice, cmd: u32, stream_id: u32) {
    // We need two buffers: Request (Read) and Response (Write)
    let dma_req = device_alloc_dma(driver.claim_handle(), 1).unwrap();
    let phys_req = device_dma_phys(dma_req).unwrap();

    let dma_resp = device_alloc_dma(driver.claim_handle(), 1).unwrap();
    let phys_resp = device_dma_phys(dma_resp).unwrap();

    unsafe {
        *(dma_req as *mut VirtioSndPcmHdr) = VirtioSndPcmHdr {
            hdr: VirtioSndHdr { code: cmd },
            stream_id,
        };
        *(dma_resp as *mut VirtioSndHdr) = VirtioSndHdr { code: 0 };
    }

    let bufs = [
        (phys_req, size_of::<VirtioSndPcmHdr>() as u32, false), // Device Read
        (phys_resp, size_of::<VirtioSndHdr>() as u32, true),    // Device Write
    ];

    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        q.add_buffer(&bufs);
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);

    // Simplistic polling for completion
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            break;
        }
        stem::yield_now();
    }
}

fn configure_stream(driver: &mut VirtioDevice, stream_id: u32) {
    // 1. Set Params
    let dma_req = device_alloc_dma(driver.claim_handle(), 1).unwrap();
    let phys_req = device_dma_phys(dma_req).unwrap();

    let dma_resp = device_alloc_dma(driver.claim_handle(), 1).unwrap();
    let phys_resp = device_dma_phys(dma_resp).unwrap();

    unsafe {
        *(dma_req as *mut VirtioSndPcmSetParams) = VirtioSndPcmSetParams {
            hdr: VirtioSndHdr {
                code: VIRTIO_SND_R_PCM_SET_PARAMS,
            },
            buffer_bytes: 65536, // Increased to 64KB
            period_bytes: 4096,
            features: 0,
            channels: 2, // Stereo
            format: VIRTIO_SND_PCM_FMT_S16,
            rate: VIRTIO_SND_PCM_RATE_44100,
            padding: 0,
        };
    }

    let bufs = [
        (phys_req, size_of::<VirtioSndPcmSetParams>() as u32, false),
        (phys_resp, size_of::<VirtioSndHdr>() as u32, true),
    ];

    {
        let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
        q.add_buffer(&bufs);
    }
    driver.notify_queue(VIRTIO_SND_VQ_CONTROL);

    // Wait for completion
    loop {
        let done = {
            let q = driver.queue_mut(VIRTIO_SND_VQ_CONTROL).unwrap();
            q.poll_used().is_some()
        };
        if done {
            break;
        }
        stem::yield_now();
    }

    // 2. Prepare
    send_pcm_command(driver, VIRTIO_SND_R_PCM_PREPARE, stream_id);
}

fn find_output_stream(_driver: &mut VirtioDevice) -> Option<u32> {
    // We should QUERY the device info.
    // For V0, we blind guess Stream 0 is output.
    Some(0)
}
