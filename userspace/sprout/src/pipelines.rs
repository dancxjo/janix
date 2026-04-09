use crate::task::{ManagedTask, TaskKind};
use abi::display_driver_protocol::{FbInfoPayload, FB_INFO_PAYLOAD_SIZE};
use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use abi::syscall::vfs_flags::O_RDONLY;
use alloc::string::ToString;
use alloc::vec::Vec;
use stem::abi::driver_ctx::DriverCtx;
use stem::syscall::vfs::{vfs_close, vfs_open, vfs_read};
use stem::syscall::{channel_create, ChannelHandle};
use stem::{info, warn};

fn ensure_session_roots() {
    use stem::syscall::vfs::vfs_mkdir;
    let _ = vfs_mkdir("/session");
    let _ = vfs_mkdir("/session/seat0");
    let _ = vfs_mkdir("/session/seat0/keyboard");
    let _ = vfs_mkdir("/session/seat0/pointer");
    let _ = vfs_mkdir("/session/display");
}

#[derive(Clone, Copy, Debug)]
pub struct DisplayHandles {
    pub drv_req_write: ChannelHandle,
    pub drv_resp_read: ChannelHandle,
    pub bs_id: u32,
    /// Which display backend was selected
    pub backend_name: &'static str,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}

fn find_sys_device(class_prefix: &str) -> Option<alloc::string::String> {
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return None,
    };

    let mut buf = [0u8; 4096];
    let n = match stem::syscall::vfs::vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(e) => {
            warn!("SPROUT: readdir(/sys/devices) failed: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };
    let _ = vfs_close(fd);

    warn!("SPROUT: readdir found {} bytes", n);

    let mut offset = 0usize;
    while offset < n {
        if buf[offset] == 0 {
            offset += 1;
            continue;
        }

        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }

        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
            warn!("SPROUT:   Checking entry at {}: '{}'", offset, name);
            if name.starts_with("pci-") {
                let class_path = alloc::format!("/sys/devices/{}/class", name);
                if let Ok(class_fd) = vfs_open(&class_path, O_RDONLY) {
                    let mut class_buf = [0u8; 16];
                    if let Ok(cn) = vfs_read(class_fd, &mut class_buf) {
                        let class_str = core::str::from_utf8(&class_buf[..cn]).unwrap_or("");
                        info!("SPROUT: Checked device {} class='{}'", name, class_str.trim());
                        if class_str.trim().starts_with(class_prefix) {
                            let _ = vfs_close(class_fd);
                            return Some(alloc::format!("/sys/devices/{}", name));
                        }
                    } else {
                        warn!("SPROUT: Failed to read {}", class_path);
                    }
                    let _ = vfs_close(class_fd);
                } else {
                    warn!("SPROUT: Failed to open {}", class_path);
                }
            }
        }
        offset = end.saturating_add(1);
    }
    None
}

fn find_sys_device_with_vendor(
    class_prefix: &str,
    vendor_prefix: &str,
) -> Option<alloc::string::String> {
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return None,
    };

    let mut buf = [0u8; 4096];
    let n = match stem::syscall::vfs::vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(e) => {
            warn!("SPROUT: readdir(/sys/devices) failed: {:?}", e);
            let _ = vfs_close(fd);
            return None;
        }
    };
    let _ = vfs_close(fd);

    let mut offset = 0usize;
    while offset < n {
        if buf[offset] == 0 {
            offset += 1;
            continue;
        }

        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }

        if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
            if name.starts_with("pci-") {
                let class_path = alloc::format!("/sys/devices/{}/class", name);
                let vendor_path = alloc::format!("/sys/devices/{}/vendor", name);

                let class_matches = if let Ok(class_fd) = vfs_open(&class_path, O_RDONLY) {
                    let mut class_buf = [0u8; 16];
                    let matched = if let Ok(cn) = vfs_read(class_fd, &mut class_buf) {
                        let class_str = core::str::from_utf8(&class_buf[..cn]).unwrap_or("");
                        class_str.trim().starts_with(class_prefix)
                    } else {
                        false
                    };
                    let _ = vfs_close(class_fd);
                    matched
                } else {
                    false
                };

                if !class_matches {
                    offset = end.saturating_add(1);
                    continue;
                }

                let vendor_matches = if let Ok(vendor_fd) = vfs_open(&vendor_path, O_RDONLY) {
                    let mut vendor_buf = [0u8; 16];
                    let matched = if let Ok(vn) = vfs_read(vendor_fd, &mut vendor_buf) {
                        let vendor_str = core::str::from_utf8(&vendor_buf[..vn]).unwrap_or("");
                        vendor_str.trim().starts_with(vendor_prefix)
                    } else {
                        false
                    };
                    let _ = vfs_close(vendor_fd);
                    matched
                } else {
                    false
                };

                if vendor_matches {
                    return Some(alloc::format!("/sys/devices/{}", name));
                }
            }
        }

        offset = end.saturating_add(1);
    }

    None
}

fn has_sys_device(class_prefix: &str) -> bool {
    find_sys_device(class_prefix).is_some()
}

fn probe_bootfb_vfs() -> Option<(u32, u32, u32, u32)> {
    let fd = match vfs_open("/dev/fb0", O_RDONLY) {
        Ok(fd) => fd,
        Err(e) => {
            warn!("SPROUT: open(/dev/fb0) failed: {:?}", e);
            return None;
        }
    };
    let mut payload = FbInfoPayload {
        graph_id: 0,
        width: 0,
        height: 0,
        stride: 0,
        bpp: 0,
        format: 0,
    };
    let slice = unsafe {
        core::slice::from_raw_parts_mut(&mut payload as *mut _ as *mut u8, FB_INFO_PAYLOAD_SIZE)
    };
    let n = match vfs_read(fd, slice) {
        Ok(n) => n,
        Err(e) => {
            let _ = vfs_close(fd);
            warn!("SPROUT: read(/dev/fb0) failed: {:?}", e);
            return None;
        }
    };
    let _ = vfs_close(fd);
    if n < FB_INFO_PAYLOAD_SIZE || payload.width == 0 || payload.height == 0 || payload.stride == 0
    {
        warn!(
            "SPROUT: /dev/fb0 payload invalid: n={} width={} height={} stride={} format={}",
            n, payload.width, payload.height, payload.stride, payload.format
        );
        return None;
    }
    Some((
        payload.width,
        payload.height,
        payload.stride,
        payload.format,
    ))
}

pub fn setup_pci_stub_pipeline(tasks: &mut Vec<ManagedTask>) {
    let needs_stubd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => {
            let _ = vfs_close(fd);
            true
        }
        Err(_) => false,
    };

    if !needs_stubd {
        return;
    }

    match stem::syscall::spawn_process("/pci_stubd", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned pci_stubd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/pci_stubd".to_string(),
                kind: TaskKind::Driver("dev.pci.stub".to_string()),
                module_path: "/pci_stubd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn pci_stubd: {:?}", e);
        }
    }
}

pub fn setup_rtc_pipeline(tasks: &mut Vec<ManagedTask>) {
    let fd = match vfs_open("/dev/rtc", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => {
            info!("SPROUT: No /dev/rtc found, skipping rtc_cmos");
            return;
        }
    };
    let _ = vfs_close(fd);

    // Note: rtc_cmos driver will now just open /dev/rtc itself or use sys_time_now.
    // For legacy arg passing, we can still use a fake device ID or just pass 0.
    match stem::syscall::spawn_process("/rtc_cmos", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned rtc_cmos (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/rtc_cmos".to_string(),
                kind: TaskKind::Driver(kinds::DEV_RTC_CMOS.to_string()),
                module_path: "/rtc_cmos".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn rtc_cmos: {:?}", e);
        }
    }
}
pub fn setup_storage_pipeline(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up storage pipeline...");

    // 1. Probe for AHCI (0x0106)
    if let Some(path) = find_sys_device("0x0106") {
        info!("SPROUT: Found AHCI controller at {}", path);

        // Pass path via bootstrap memfd
        let boot_size = 256;
        let boot_fd = stem::syscall::memfd_create("ahci.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
            let req = VmMapReq {
                addr_hint: 0,
                len: boot_size,
                prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
                flags: VmMapFlags::empty(),
                backing: VmBacking::File { fd: boot_fd, offset: 0 },
            };
            if let Ok(resp) = stem::syscall::vm_map(&req) {
                let ptr = resp.addr as *mut u8;
                unsafe {
                    core::ptr::copy_nonoverlapping(path.as_ptr(), ptr, path.len());
                    *ptr.add(path.len()) = 0;
                }
            }
        }

        match stem::syscall::spawn_process("/ahci_disk", boot_fd as usize) {
            Ok(pid) => {
                info!("SPROUT: Spawned ahci_disk (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/ahci_disk".to_string(),
                    kind: TaskKind::Driver("dev.storage.ahci".to_string()),
                    module_path: "/ahci_disk".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: boot_fd as usize,
                    bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn ahci_disk: {:?}", e);
            }
        }
    }

    // 2. Probe for legacy IDE (0x0101)
    if has_sys_device("0x0101") {
        match stem::syscall::spawn_process("/ata_disk", 0) {
            Ok(pid) => {
                info!("SPROUT: Spawned ata_disk (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/ata_disk".to_string(),
                    kind: TaskKind::Driver("dev.storage.ata".to_string()),
                    module_path: "/ata_disk".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: 0,
                    bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn ata_disk: {:?}", e);
            }
        }
    }

    match stem::syscall::spawn_process("/iso9660d", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned iso9660d (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/iso9660d".to_string(),
                kind: TaskKind::Service("svc.iso9660.Mount".to_string()),
                module_path: "/iso9660d".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn iso9660d: {:?}", e);
        }
    }
}

pub fn setup_display_pipeline(
    tasks: &mut Vec<ManagedTask>,
    supervisor_port: stem::syscall::ChannelHandle,
    bind_instance_id: u64,
) -> Option<DisplayHandles> {
    info!("SPROUT: setup_display_pipeline start (bind_id={})", bind_instance_id);

    let mut display_width = 0u32;
    let mut display_height = 0u32;
    let mut display_stride = 0u32;
    let mut display_format = 0u32;
    let mut driver_name: Option<&'static str> = None;
    let mut backend_name: &'static str = "unknown";

    // Janix-style BootFB probe: if /dev/fb0 exists, trust that as the canonical display.
    if let Some((w, h, stride, format)) = probe_bootfb_vfs() {
        display_width = w;
        display_height = h;
        display_stride = stride;
        display_format = format;
        driver_name = Some("/display_bootfb");
        backend_name = "BootFB";
        info!(
            "SPROUT: Using /dev/fb0 boot framebuffer ({}x{} stride={})",
            display_width, display_height, display_stride
        );
    }
        // virtio-gpu: class 0x030000, vendor 0x1af4
        if has_sys_device("0x0300") {
            display_stride = display_width * 4;
            display_format = 1;
            driver_name = Some("/display_virtio_gpu");
            backend_name = "VirtIO-GPU";
            info!(
                "SPROUT: Using VirtIO GPU at {}x{}",
                display_width, display_height
            );
        }

    // Fallback to BootFB already handled by probe_bootfb_vfs() at start of function

    // Fallback to display_fake if no other display found (ensures Bloom launches)
    if driver_name.is_none() && backend_name != "BootFB" {
        warn!("SPROUT: No display device found! Using display_fake (headless mode)");
        display_width = 1024;
        display_height = 768;
        display_stride = 1024 * 4;
        display_format = 1; // BGRA8888
        driver_name = Some("/display_fake");
        backend_name = "Fake";
    }

    if display_width == 0 || display_height == 0 || display_stride == 0 {
        warn!("SPROUT: Invalid display geometry, skipping display pipeline");
        return None;
    }

    info!(
        "SPROUT: Display backend selected: {} (Driver: {:?}, Geometry: {}x{} stride={})",
        backend_name, driver_name, display_width, display_height, display_stride
    );

    let size = (display_height as usize) * (display_stride as usize);
    let bs_id = match stem::syscall::memfd_create("display.buffer", size) {
        Ok(fd) => fd,
        Err(e) => {
            warn!("SPROUT: memfd_create failed: {:?}", e);
            return None;
        }
    };

    let _ = bs_id; // Metadata is now passed via bloom bootstrap bytespace

    let mut drv_req_write = 0;
    let mut drv_resp_read = 0;

    if let Some(driver_name) = driver_name {
        let drv_req = match channel_create(4096) {
            Ok(handles) => handles,
            Err(e) => {
                warn!("SPROUT: drv_req channel_create failed: {:?}", e);
                return None;
            }
        };
        let drv_resp = match channel_create(4096) {
            Ok(handles) => handles,
            Err(e) => {
                warn!("SPROUT: drv_resp channel_create failed: {:?}", e);
                return None;
            }
        };

        drv_req_write = drv_req.0;
        drv_resp_read = drv_resp.1;

        let boot_size = 4096;
        let boot_fd = stem::syscall::memfd_create("driver.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
            let req = VmMapReq {
                addr_hint: 0,
                len: boot_size,
                prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
                flags: VmMapFlags::empty(),
                backing: VmBacking::File {
                    fd: boot_fd,
                    offset: 0,
                },
            };
            if let Ok(resp) = stem::syscall::vm_map(&req) {
                let ptr = resp.addr;
                let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, boot_size / 4) };
                
                slice[0] = drv_req.1 as u32;  // Read end of req channel
                slice[1] = drv_resp.0 as u32; // Write end of resp channel
                slice[2] = drv_resp.0 as u32; // supervisor_port (PRIVATE!)
                
                let id_low = (bind_instance_id & 0xFFFF_FFFF) as u32;
                let id_high = (bind_instance_id >> 32) as u32;
                slice[3] = id_low;
                slice[4] = id_high;

                info!("SPROUT: Bootstrapping driver {} via memfd {}: req_r={}, resp_w={}, svc={}, id={}", 
                    driver_name, boot_fd, slice[0], slice[1], slice[2], bind_instance_id);
            }
        }

        if let Ok(pid) = stem::syscall::spawn_process(driver_name, boot_fd as usize) {
            info!(
                "SPROUT: Spawned display driver '{}' (PID={})",
                driver_name, pid
            );
            let _ = stem::thread::set_priority(pid, 3);
            tasks.push(ManagedTask {
                name: driver_name.to_string(),
                kind: TaskKind::Driver("dev.display".to_string()),
                module_path: driver_name.to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: boot_fd as usize,
                bind_instance_id: bind_instance_id,
                drv_req_write: drv_req.0,
                drv_resp_read: drv_resp.1,
            });
        }

        // Legacy graph nodes removed. display-os protocol will carry backend specifics.
    }

    Some(DisplayHandles {
        drv_req_write,
        drv_resp_read,
        bs_id,
        backend_name,
        width: display_width,
        height: display_height,
        stride: display_stride,
        format: display_format,
    })
}

pub fn setup_terminal(
    tasks: &mut Vec<ManagedTask>,
    display: Option<DisplayHandles>,
    _input: InputHandles,
) {
    info!("SPROUT: Setting up Terminal...");
    ensure_session_roots();

    let Some(display) = display else {
        warn!("SPROUT: Cannot setup terminal without display!");
        return;
    };

    let boot_size = 4096;
    let boot_fd = stem::syscall::memfd_create("terminal.boot", boot_size).unwrap_or(0);

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
        let req = VmMapReq {
            addr_hint: 0,
            len: boot_size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: VmMapFlags::empty(),
            backing: VmBacking::File {
                fd: boot_fd,
                offset: 0,
            },
        };
        if let Ok(resp) = stem::syscall::vm_map(&req) {
            let ptr = resp.addr;
            let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, boot_size / 4) };
            slice[0] = 0xB100AA01; // Magic
            slice[1] = display.drv_req_write as u32;
            slice[2] = display.drv_resp_read as u32;
            slice[4] = display.bs_id;
            info!("SPROUT: Bootstrapping terminal via memfd {}: req_w={}, resp_r={}, bs_id={}", boot_fd, slice[1], slice[2], slice[4]);
        }
    }

    let term_arg = boot_fd as u32;

    match stem::syscall::spawn_process("/terminal", term_arg as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned terminal (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/terminal".to_string(),
                kind: TaskKind::App,
                module_path: "/terminal".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: term_arg as usize,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn terminal: {:?}", e);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct InputHandles {
    pub bloom_evt_read: ChannelHandle,
    pub evt_input_echo_read: ChannelHandle,
}

pub fn setup_input_broker(tasks: &mut Vec<ManagedTask>) -> InputHandles {
    info!("SPROUT: Setting up input pipeline (keyboard + mouse)...");

    // Create kbd_raw port (ps2_kbd -> bristle)
    let kbd_raw = match stem::syscall::channel_create(4096) {
        Ok((write_h, read_h)) => {
            info!("SPROUT: Created kbd_raw port (w={}, r={})", write_h, read_h);
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create kbd_raw port: {:?}", e);
            return InputHandles {
                bloom_evt_read: 0,
                evt_input_echo_read: 0,
            };
        }
    };

    // Create mouse_raw port (ps2_mouse -> bristle)
    let mouse_raw = match stem::syscall::channel_create(4096) {
        Ok((write_h, read_h)) => {
            info!(
                "SPROUT: Created mouse_raw port (w={}, r={})",
                write_h, read_h
            );
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create mouse_raw port: {:?}", e);
            return InputHandles {
                bloom_evt_read: 0,
                evt_input_echo_read: 0,
            };
        }
    };

    // Dedicated Bristle -> Bloom input channel plus optional input echo tap.
    let bloom_evt = match stem::syscall::channel_create(4096) {
        Ok(h) => h,
        Err(e) => {
            stem::error!("SPROUT: Failed to create bloom_evt: {:?}", e);
            return InputHandles {
                bloom_evt_read: 0,
                evt_input_echo_read: 0,
            };
        }
    };

    let evt_input_echo = match stem::syscall::channel_create(4096) {
        Ok(h) => h,
        Err(e) => {
            stem::error!("SPROUT: Failed to create evt_input_echo: {:?}", e);
            return InputHandles {
                bloom_evt_read: 0,
                evt_input_echo_read: 0,
            };
        }
    };

    // Spawn ps2_kbd with raw write handle
    match stem::syscall::spawn_process("/ps2_kbd", kbd_raw.0 as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned ps2_kbd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/ps2_kbd".to_string(),
                kind: TaskKind::Driver("dev.input.ps2.kbd".to_string()),
                module_path: "/ps2_kbd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: kbd_raw.0 as usize,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn ps2_kbd: {:?}", e);
        }
    }

    // Spawn ps2_mouse with raw write handle
    match stem::syscall::spawn_process("/ps2_mouse", mouse_raw.0 as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned ps2_mouse (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/ps2_mouse".to_string(),
                kind: TaskKind::Driver("dev.input.ps2.mouse".to_string()),
                module_path: "/ps2_mouse".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: mouse_raw.0 as usize,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn ps2_mouse: {:?}", e);
        }
    }

    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | evt_input_echo_write[15:0]
    let bristle_arg = ((kbd_raw.1 as u64) << 48)
        | ((mouse_raw.1 as u64) << 32)
        | ((bloom_evt.0 as u64) << 16)
        | (evt_input_echo.0 as u64);

    match stem::syscall::spawn_process("/bristle", bristle_arg as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned bristle (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/bristle".to_string(),
                kind: TaskKind::App,
                module_path: "/bristle".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: bristle_arg as usize,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bristle: {:?}", e);
        }
    }

    info!("SPROUT: Input broker ready (keyboard + mouse)");
    InputHandles {
        bloom_evt_read: bloom_evt.1,
        evt_input_echo_read: evt_input_echo.1,
    }
}


/// Set up network pipeline - spawn virtio_netd (driver) then netd (stack)
pub fn setup_network_stack(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up network stack...");

    // Prefer VirtIO NICs in QEMU/VM flows.
    if let Some(path) = find_sys_device_with_vendor("0x0200", "0x1af4") {
        // 1. Setup virtio_link bootstrapping
        let drv_req = stem::syscall::channel_create(4096).unwrap();
        let drv_resp = stem::syscall::channel_create(4096).unwrap();
        let supervisor_port = 5; // Fixed for now, should be passed from Supervisor struct
        let bind_id = 0xDE00_0000_2000_0000u64;

        let boot_size = 4096;
        let boot_fd = stem::syscall::memfd_create("virtio-net.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use stem::syscall::vfs::{vfs_seek, vfs_write};
            // Map it to write the bootstrap header
            use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
            let req = VmMapReq {
                addr_hint: 0,
                len: boot_size,
                prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
                flags: VmMapFlags::empty(),
                backing: VmBacking::File { fd: boot_fd as u32, offset: 0 },
            };
            if let Ok(resp) = stem::syscall::vm_map(&req) {
                let slice = unsafe { core::slice::from_raw_parts_mut(resp.addr as *mut u32, 1024) };
                
                slice[0] = drv_req.1 as u32;  // Read end of req channel
                slice[1] = drv_resp.0 as u32; // Write end of resp channel
                slice[2] = drv_resp.0 as u32; // supervisor_port (PRIVATE!)
                
                let id_low = (bind_id & 0xFFFF_FFFF) as u32;
                let id_high = (bind_id >> 32) as u32;
                slice[3] = id_low;
                slice[4] = id_high;
            }
            let _ = vfs_seek(boot_fd as u32, 512, 0); // Move path to offset 512
            let _ = vfs_write(boot_fd as u32, path.as_bytes()); // Keep path for driver claim
            let _ = vfs_seek(boot_fd as u32, 0, 0);
        }

        tasks.push(ManagedTask {
            name: "/bin/virtio_netd".to_string(),
            kind: TaskKind::Driver("dev.net.virtio".to_string()),
            module_path: "/bin/virtio_netd".to_string(),
            pid: None,
            restarts: 0,
            spawn_arg: boot_fd as usize,
            bind_instance_id: bind_id,
            drv_req_write: drv_req.0,
            drv_resp_read: drv_resp.1,
        });
    } else if let Some(path) = find_sys_device("0x0200") {
        info!("SPROUT: Found RTL8168 at {}", path);

        // Pass path via bootstrap memfd
        let boot_size = 256;
        let boot_fd = stem::syscall::memfd_create("net.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use abi::vm::{VmBacking, VmMapReq, VmProt, VmMapFlags};
            let req = VmMapReq {
                addr_hint: 0,
                len: boot_size,
                prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
                flags: VmMapFlags::empty(),
                backing: VmBacking::File { fd: boot_fd, offset: 0 },
            };
            if let Ok(resp) = stem::syscall::vm_map(&req) {
                let ptr = resp.addr as *mut u8;
                unsafe {
                    core::ptr::copy_nonoverlapping(path.as_ptr(), ptr, path.len());
                    *ptr.add(path.len()) = 0;
                }
            }
        }
 
        match stem::syscall::spawn_process("/rtl8168d", boot_fd as usize) {
            Ok(pid) => {
                info!("SPROUT: Spawned rtl8168d (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/rtl8168d".to_string(),
                    kind: TaskKind::Driver("dev.net.rtl8168".to_string()),
                    module_path: "/rtl8168d".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: boot_fd as usize,
                    bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn rtl8168d: {:?}", e);
            }
        }
    }
 
    spawn_netd(tasks);
}

fn spawn_netd(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: spawn_netd start");
    match stem::syscall::spawn_process("/netd", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned netd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/netd".to_string(),
                kind: TaskKind::Service("svc.net".to_string()),
                module_path: "/netd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn netd: {:?}", e);
        }
    }
}

pub fn setup_network_apps(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up network apps...");

    match stem::syscall::spawn_process("/nectar", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned nectar (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/nectar".to_string(),
                kind: TaskKind::Service("svc.nectar".to_string()),
                module_path: "/nectar".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn nectar: {:?}", e);
        }
    }

    match stem::syscall::spawn_process("/fetchd", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned fetchd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/fetchd".to_string(),
                kind: TaskKind::App,
                module_path: "/fetchd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn fetchd: {:?}", e);
        }
    }
}

pub fn setup_taskman_service(_tasks: &mut Vec<ManagedTask>) {
    // Taskman removed
}

pub fn setup_ui_services(tasks: &mut Vec<ManagedTask>) {
    spawn_ui_service(tasks, "/flytrap", "svc.flytrap", 2);
    spawn_ui_service(tasks, "/blossom", "svc.blossom", 2);
}

pub fn setup_blossom_service(tasks: &mut Vec<ManagedTask>) {
    spawn_ui_service(tasks, "/blossom", "svc.blossom", 2);
}

pub fn setup_font_service(_tasks: &mut Vec<ManagedTask>) {
    // Font handling is integrated into Bloom directly
}

pub fn setup_flytrap_service(tasks: &mut Vec<ManagedTask>) {
    spawn_ui_service(tasks, "/flytrap", "svc.flytrap", 2);
}

fn spawn_ui_service(tasks: &mut Vec<ManagedTask>, name: &str, service: &str, priority: usize) {
    if tasks.iter().any(|t| t.name == name && t.pid.is_some()) {
        return;
    }

    match stem::syscall::spawn_process(name, 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned {} (PID={})", &name[1..], pid);
            let _ = stem::thread::set_priority(pid, priority);
            tasks.push(ManagedTask {
                name: name.to_string(),
                kind: TaskKind::Service(service.to_string()),
                module_path: name.to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn {}: {:?}", &name[1..], e);
        }
    }
}

/// Set up audio driver - spawn virtio_sound or hdaudio
pub fn setup_audio_driver(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up audio driver...");

    // Check for HDA (PCI Class 0403)
    if let Some(path) = find_sys_device("0x0403") {
        info!("SPROUT: Found HDA sound device at {}", path);

        // Pass the VFS path via bootstrap bytespace
        let boot_size = 256;
        let boot_fd = stem::syscall::memfd_create("hda.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use stem::syscall::vfs::{vfs_write, vfs_seek};
            let _ = vfs_write(boot_fd as u32, path.as_bytes());
            let _ = vfs_write(boot_fd as u32, &[0]); // Null terminator
            let _ = vfs_seek(boot_fd as u32, 0, 0); // Reset cursor for the driver
        }

        match stem::syscall::spawn_process("/hdaudio", boot_fd as usize) {
            Ok(pid) => {
                info!("SPROUT: Spawned hdaudio (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/hdaudio".to_string(),
                    kind: TaskKind::Driver("dev.sound.hda".to_string()),
                    module_path: "/hdaudio".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: boot_fd as usize,
                    bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn hdaudio: {:?}", e);
            }
        }
    }

    // Check for VirtIO Sound (PCI Class 0401)
    if let Some(path) = find_sys_device("0x0401") {
        info!("SPROUT: Found VirtIO sound device at {}", path);
        
        // Pass the VFS path via bootstrap bytespace (using memfd for string storage)
        let boot_size = 256;
        let boot_fd = stem::syscall::memfd_create("snd.boot", boot_size).unwrap_or(0);
        if boot_fd != 0 {
            use stem::syscall::vfs::{vfs_write, vfs_seek};
            let _ = vfs_write(boot_fd as u32, path.as_bytes());
            let _ = vfs_write(boot_fd as u32, &[0]); // Null terminator
            let _ = vfs_seek(boot_fd as u32, 0, 0); // Reset cursor for the driver
        }

        match stem::syscall::spawn_process("/virtio_sound", boot_fd as usize) {
            Ok(pid) => {
                info!("SPROUT: Spawned virtio_sound (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/virtio_sound".to_string(),
                    kind: TaskKind::Driver("dev.sound.virtio".to_string()),
                    module_path: "/virtio_sound".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: boot_fd as usize,
                    bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn virtio_sound: {:?}", e);
            }
        }
    } else {
        info!("SPROUT: No VirtIO Sound device found");
    }
}

pub fn spawn_beeper(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Spawning beeper...");
    match stem::syscall::spawn_process("/beeper", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned beeper (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/beeper".to_string(),
                kind: TaskKind::App,
                module_path: "/beeper".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn beeper: {:?}", e);
        }
    }
}

pub fn setup_graphics_stack(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up Graphics Stack (Bloom + fontd)...");

    // 1. Setup fontd
    let font_chan = channel_create(4096).expect("Failed to create fontd channel");
    // font_chan.1 is the read end for fontd, font_chan.0 is the write end for clients
    
    match stem::syscall::spawn_process("/fontd", font_chan.1 as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned fontd (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/fontd".to_string(),
                kind: TaskKind::Service("svc.font".to_string()),
                module_path: "/fontd".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: font_chan.1 as usize,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn fontd: {:?}", e);
        }
    }

    // 2. Setup Bloom (Compositor)
    match stem::syscall::spawn_process("/bloom", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned bloom (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 3); // High priority for compositor
            tasks.push(ManagedTask {
                name: "/bloom".to_string(),
                kind: TaskKind::App,
                module_path: "/bloom".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: 0,
                bind_instance_id: 0, drv_req_write: 0, drv_resp_read: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn bloom: {:?}", e);
        }
    }
}
