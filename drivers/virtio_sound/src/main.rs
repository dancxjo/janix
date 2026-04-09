#![feature(restricted_std)]
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

fn find_virtio_sound_device() -> Option<String> {
    use stem::syscall::vfs::{vfs_open, vfs_readdir, vfs_close, vfs_read};
    use abi::syscall::vfs_flags;

    let fd = match vfs_open("/sys/devices", vfs_flags::O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return None,
    };

    let mut buf = [0u8; 4096];
    let n = match vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(_) => {
            let _ = vfs_close(fd);
            return None;
        }
    };
    let _ = vfs_close(fd);

    let mut pos = 0;
    while pos < n {
        let entry_buf = &buf[pos..n];
        let name = core::str::from_utf8(entry_buf).unwrap_or("").split('\0').next().unwrap_or("");
        if name.is_empty() { break; }
        
        if name.starts_with("pci-") {
            let path = alloc::format!("/sys/devices/{}/class", name);
            if let Ok(id_fd) = vfs_open(&path, vfs_flags::O_RDONLY) {
                let mut id_buf = [0u8; 64];
                if let Ok(id_len) = vfs_read(id_fd, &mut id_buf) {
                    let id_str = core::str::from_utf8(&id_buf[..id_len]).unwrap_or("");
                    // Check for PCI Class 0401 (Audio Controller)
                    if id_str.trim().starts_with("0x0401") {
                        let _ = vfs_close(id_fd);
                        return Some(alloc::format!("/sys/devices/{}", name));
                    }
                }
                let _ = vfs_close(id_fd);
            }
        }
        pos += name.len() + 1;
    }

    None
}

#[stem::main]
fn main(boot_fd: usize) -> ! {
    info!("SND: Starting VirtIO Sound Driver (boot_fd={})...", boot_fd);

    let mut boot_fd = boot_fd;

    if boot_fd == 0 {
        let mut buf = [0u8; 1024];
        if let Ok(needed) = stem::syscall::argv_get(&mut buf) {
            if needed >= 4 {
                let count = u32::from_le_bytes(buf[0..4].try_into().unwrap());
                if count >= 2 {
                    let mut offset = 4;
                    // Skip argv[0]
                    let arg0_len = u32::from_le_bytes(buf[offset..offset+4].try_into().unwrap()) as usize;
                    offset += 4 + arg0_len;
                    // argv[1]
                    if offset + 4 <= buf.len() {
                        let arg1_len = u32::from_le_bytes(buf[offset..offset+4].try_into().unwrap()) as usize;
                        offset += 4;
                        if offset + arg1_len <= buf.len() {
                            if let Ok(s) = core::str::from_utf8(&buf[offset..offset + arg1_len]) {
                                if let Ok(val) = s.parse::<usize>() {
                                    boot_fd = val;
                                    info!("SND: Recovered boot_fd {} from argv[1]", boot_fd);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut path_buf = [0u8; 128];
    let path_len = if boot_fd != 0 {
        use stem::syscall::vfs::vfs_read;
        vfs_read(boot_fd as u32, &mut path_buf).unwrap_or(0)
    } else {
        0
    };
    
    let mut path_str = if path_len > 0 {
        core::str::from_utf8(&path_buf[..path_len]).unwrap_or("").trim_matches(char::from(0)).to_string()
    } else {
        String::new()
    };

    if path_str.is_empty() {
        if let Some(found) = find_virtio_sound_device() {
            info!("SND: Discovered device at {}", found);
            path_str = found;
        } else {
            path_str = "/sys/devices/pci-00:01.0".to_string(); // Ultimate fallback
        }
    }

    // 2. Initialize Hardware
    let mut driver = match VirtioDevice::new(&path_str) {
        Ok(d) => d,
        Err(e) => {
            error!("SND: Failed to claim device at {}: {:?}", path_str, e);
            loop { stem::time::sleep_ms(1000); }
        }
    };

    // Negotiate features
    if let Err(e) = driver.init(VIRTIO_SND_F_CTLS) {
        error!("SND: Failed to init device: {}", e);
        loop {
            stem::time::sleep_ms(1);
        }
    }

    // Setup queues
    for q in 0..4 {
        if let Err(e) = driver.setup_queue(q, QUEUE_SIZE) {
            error!("SND: Failed to setup queue {}: {}", q, e);
            loop {
                stem::time::sleep_ms(1);
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
                stem::time::sleep_ms(1);
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
        stem::syscall::channel_create(65536).expect("Failed to create channel");

    // Publish port handle (quick hack: print it for now, usually publish to graph)
    info!(
        "SND: Listening for audio on port handles: W={} R={}",
        write_handle, read_handle
    );
    // Publish to VFS
    use stem::syscall::vfs::{vfs_mkdir, vfs_open, vfs_write, vfs_close};
    use abi::sound::AudioInfoPayload;
    
    let payload = AudioInfoPayload {
        magic: AudioInfoPayload::MAGIC,
        write_handle: write_handle as u32,
        read_handle: read_handle as u32,
        sample_rate: 44100,
        channels: 2,
        bits_per_sample: 16,
    };

    let _ = vfs_mkdir("/services/sound");
    if let Ok(fd) = vfs_open("/services/sound/main", abi::syscall::vfs_flags::O_CREAT | abi::syscall::vfs_flags::O_RDWR) {
        let slice = unsafe {
            core::slice::from_raw_parts(&payload as *const _ as *const u8, abi::sound::AUDIO_INFO_PAYLOAD_SIZE)
        };
        let _ = vfs_write(fd, slice);
        let _ = vfs_close(fd);
        info!("SND: published binary PCM1 info to /services/sound/main");
    }

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

        // 3. Update Status Properties (every ~100ms) - Graph updates removed in VFS-native move
        let now = stem::time::monotonic_ns();
        if now - last_status_update > 100_000_000 {
            // let port_len = stem::syscall::channel_len(read_handle).unwrap_or(0);
            // let port_cap = stem::syscall::channel_capacity(read_handle).unwrap_or(1);
            last_status_update = now;
        }

        // 4. Process Audio Data
        match stem::syscall::channel_recv(read_handle, &mut buf) {
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
                                stem::syscall::yield_now(); // Yield immediately without locking timer
                                break;
                            } else {
                                // Queue full. Poll for completions and yield.
                                process_tx_queue(&mut driver);
                                stem::syscall::yield_now();
                            }
                        }
                    }
                    Err(_) => {
                        warn!("SND: DMA alloc failed, dropping frame");
                    }
                }
            }
            _ => {
                let mut ws = stem::wait_set::WaitSet::new();
                if let Ok(_) = ws.add_port_readable(read_handle as u64) {
                    let _ = ws.wait(Some(stem::time::Duration::from_millis(50)));
                } else {
                    stem::time::sleep_ms(10);
                }
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
        stem::time::sleep_ms(1);
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
        stem::time::sleep_ms(1);
    }

    // 2. Prepare
    send_pcm_command(driver, VIRTIO_SND_R_PCM_PREPARE, stream_id);
}

fn find_output_stream(_driver: &mut VirtioDevice) -> Option<u32> {
    // We should QUERY the device info.
    // For V0, we blind guess Stream 0 is output.
    Some(0)
}
