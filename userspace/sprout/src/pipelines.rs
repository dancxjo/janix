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
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn};

pub struct DisplayHandles {
    pub drv_req_write: ChannelHandle,
    pub drv_resp_read: ChannelHandle,
    pub bs_id: u32,
    /// Which display backend was selected
    pub backend_name: &'static str,
}

fn has_sys_device(class_prefix: &str) -> bool {
    let fd = match vfs_open("/sys/devices", O_RDONLY) {
        Ok(fd) => fd,
        Err(_) => return false,
    };

    let mut buf = [0u8; 4096];
    let n = match stem::syscall::vfs::vfs_readdir(fd, &mut buf) {
        Ok(n) => n,
        Err(_) => {
            let _ = vfs_close(fd);
            return false;
        }
    };
    let _ = vfs_close(fd);

    let mut offset = 0usize;
    while offset < n {
        let mut end = offset;
        while end < n && buf[end] != 0 {
            end += 1;
        }
        if end > offset {
            if let Ok(name) = core::str::from_utf8(&buf[offset..end]) {
                if name.starts_with("pci-") {
                    let class_path = alloc::format!("/sys/devices/{}/class", name);
                    if let Ok(class_fd) = vfs_open(&class_path, O_RDONLY) {
                        let mut class_buf = [0u8; 16];
                        if let Ok(cn) = vfs_read(class_fd, &mut class_buf) {
                            let class_str = core::str::from_utf8(&class_buf[..cn]).unwrap_or("");
                            if class_str.trim().starts_with(class_prefix) {
                                let _ = vfs_close(class_fd);
                                return true;
                            }
                        }
                        let _ = vfs_close(class_fd);
                    }
                }
            }
        }
        offset = end.saturating_add(1);
    }
    false
}

fn has_ahci_controller() -> bool {
    has_sys_device("0x010601")
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
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn rtc_cmos: {:?}", e);
        }
    }
}

pub fn setup_storage_pipeline(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up storage pipeline...");
    if has_ahci_controller() {
        match stem::syscall::spawn_process("/ahci_disk", 0) {
            Ok(pid) => {
                info!("SPROUT: Spawned ahci_disk (PID={})", pid);
                let _ = stem::thread::set_priority(pid, 2);
                tasks.push(ManagedTask {
                    name: "/ahci_disk".to_string(),
                    kind: TaskKind::Driver("dev.storage.Ahci".to_string()),
                    module_path: "/ahci_disk".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: 0,
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn ahci_disk: {:?}", e);
            }
        }
    }

    // Check for legacy IDE/ATA via PCI class 01 01
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
                });
            }
            Err(e) => {
                warn!("SPROUT: Failed to spawn ata_disk: {:?}", e);
            }
        }
    } else {
        info!("SPROUT: No AHCI or legacy ATA hardware detected, skipping storage drivers");
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
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn iso9660d: {:?}", e);
        }
    }
}

pub fn setup_display_pipeline(tasks: &mut Vec<ManagedTask>) -> Option<DisplayHandles> {
    info!("SPROUT: Setting up display pipeline...");

    let mut display_width = 0u32;
    let mut display_height = 0u32;
    let mut display_stride = 0u32;
    let mut display_format = 0u32;
    let mut driver_name: Option<&'static str> = None;
    let mut display_device: Option<ThingId> = None;
    let mut backend_name: &'static str = "unknown";

    // Janix-style BootFB probe: if /dev/fb0 exists, trust that as the canonical display.
    if let Some((w, h, stride, format)) = probe_bootfb_vfs() {
        display_width = w;
        display_height = h;
        display_stride = stride;
        display_format = format;
        driver_name = None;
        backend_name = "BootFB";
        info!(
            "SPROUT: Using /dev/fb0 boot framebuffer ({}x{} stride={})",
            display_width, display_height, display_stride
        );
    }

    // Check for VirtIO GPU first via PCI class 03 00 00 and vendor 0x1af4 device 0x1010/0x1050
    if driver_name.is_none() && backend_name != "BootFB" {
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
        "SPROUT: Display backend: {} ({}x{} stride={})",
        backend_name, display_width, display_height, display_stride
    );

    let size = (display_height as usize) * (display_stride as usize);
    let bs_id = match thingsys::memfd_create("display.buffer", size) {
        Ok(fd) => fd,
        Err(e) => {
            warn!("SPROUT: memfd_create failed: {:?}", e);
            return None;
        }
    };

    let role_sym = match thingsys::intern("display.compositor") {
        Ok(sym) => sym,
        Err(_) => 0,
    };
    let bs_id_thing = ThingId::from_u64(bs_id as u64);
    let _ = thingsys::prop_set(bs_id_thing, "display_role", role_sym as u64);
    let _ = thingsys::prop_set(bs_id_thing, keys::WIDTH, display_width as u64);
    let _ = thingsys::prop_set(bs_id_thing, keys::HEIGHT, display_height as u64);
    let _ = thingsys::prop_set(bs_id_thing, keys::STRIDE, display_stride as u64);
    let _ = thingsys::prop_set(bs_id_thing, keys::FORMAT, display_format as u64);

    // Store backend name as a property so Bloom can query it
    if let Ok(backend_sym) = thingsys::intern(backend_name) {
        let _ = thingsys::prop_set(bs_id_thing, "display_backend", backend_sym as u64);
    }

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
        let _ = thingsys::prop_set(bs_id_thing, "display_drv_req", drv_req_write as u64);
        let _ = thingsys::prop_set(bs_id_thing, "display_drv_resp", drv_resp_read as u64);

        let driver_arg = (drv_req.1 as u64) | ((drv_resp.0 as u64) << 16);

        if let Ok(pid) = stem::syscall::spawn_process(driver_name, driver_arg as usize) {
            info!(
                "SPROUT: Spawned display driver '{}' (PID={})",
                driver_name, pid
            );
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: driver_name.to_string(),
                kind: TaskKind::Driver("dev.display".to_string()),
                module_path: driver_name.to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: driver_arg as usize,
            });
        }

        if let Ok(svc_display) = thingsys::create_node("svc.Display") {
            let drv_kind = match driver_name {
                "/display_bootfb" => "drv.DisplayBootFB",
                "/display_virtio_gpu" => "drv.DisplayVirtioGPU",
                _ => "drv.Display",
            };

            if let Ok(drv_node) = thingsys::create_node(drv_kind) {
                let _ = thingsys::link(svc_display, "USES_DRIVER", drv_node);
                let _ = thingsys::link(drv_node, "CONSUMES", bs_id_thing);
                if let Some(dev) = display_device {
                    let _ = thingsys::link(drv_node, "PRESENTS_TO", dev);
                }
            }
        }
    }

    Some(DisplayHandles {
        drv_req_write,
        drv_resp_read,
        bs_id,
        backend_name,
    })
}

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
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn ps2_mouse: {:?}", e);
        }
    }

    let bloom_evt = stem::syscall::channel_create(8192).unwrap_or((0, 0));
    let evt_input_echo = stem::syscall::channel_create(8192).unwrap_or((0, 0));

    // Spawn bristle with packed handles:
    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | bloom_evt_write[31:16] | evt_input_echo_write[15:0]
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
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bristle: {:?}", e);
        }
    }

    // Font handling is now integrated into Bloom. No standalone fontd service.

    info!("SPROUT: Input broker ready (keyboard + mouse)");
    InputHandles {
        bloom_evt_read: bloom_evt.1,
        evt_input_echo_read: evt_input_echo.1,
    }
}

pub fn setup_compositor(
    tasks: &mut Vec<ManagedTask>,
    display: Option<DisplayHandles>,
    input: InputHandles,
) {
    // Extract display handles early for bloom compositor
    let (drv_req_write, drv_resp_read, display_bs_id) = display
        .as_ref()
        .map(|d| (d.drv_req_write, d.drv_resp_read, d.bs_id))
        .unwrap_or((0, 0, 0));

    // Bloom Bootstrap
    // Create bytespace to hold args
    // Layout:
    // 0: magic (0xBl00mArg)e
    // 8: drv_req
    // 12: drv_resp
    // 16: evt
    // 20: font_req (write) -> font_req.0
    // 24: font_resp (read) -> font_resp.1

    let boot_size = 4096;
    let boot_fd = thingsys::memfd_create("bloom.boot", boot_size).unwrap_or(0);

    if boot_fd != 0 {
        use abi::vm::{VmBacking, VmMapReq, VmProt};
        let req = VmMapReq {
            addr_hint: 0,
            len: boot_size,
            prot: VmProt::READ | VmProt::WRITE | VmProt::USER,
            flags: abi::vm::VmMapFlags::empty(),
            backing: VmBacking::File {
                fd: boot_fd,
                offset: 0,
            },
        };
        if let Ok(resp) = thingsys::vm_map(&req) {
            let ptr = resp.addr;
            let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, boot_size / 4) };
            slice[0] = 0xB100AA01; // Magic
            slice[1] = drv_req_write as u32;
            slice[2] = drv_resp_read as u32;
            slice[3] = input.bloom_evt_read as u32;
            info!(
                "SPROUT: Writing bloom BS: drv_req={}, drv_resp={}, bristle_evt={}",
                drv_req_write, drv_resp_read, input.bloom_evt_read
            );

            // Display bytespace id (128-bit)
            let bs_thing = ThingId::from_u64(display_bs_id as u64);
            let bs_bytes = bs_thing.0;
            slice[4] = u32::from_le_bytes(bs_bytes[0..4].try_into().unwrap());
            slice[5] = u32::from_le_bytes(bs_bytes[4..8].try_into().unwrap());
            slice[6] = u32::from_le_bytes(bs_bytes[8..12].try_into().unwrap());
            slice[7] = u32::from_le_bytes(bs_bytes[12..16].try_into().unwrap());

            // We don't have vm_unmap yet, or it's fine to leave it mapped for now.
        }
    }

    let bloom_arg = boot_fd as usize;

    let backend_info = display.as_ref().map(|d| d.backend_name).unwrap_or("none");
    info!(
        "SPROUT: Bloom handles via FD={} backend={}",
        boot_fd, backend_info
    );

    // Spawn bloom
    match stem::syscall::spawn_process("/bloom", bloom_arg) {
        Ok(pid) => {
            info!("SPROUT: Spawned bloom (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/bloom".to_string(),
                kind: TaskKind::App,
                module_path: "/bloom".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: bloom_arg,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bloom: {:?}", e);
        }
    }

    // Spawn input_echo with wired Bristle event handle.
    match stem::syscall::spawn_process("/input_echo", input.evt_input_echo_read as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned input_echo (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/input_echo".to_string(),
                kind: TaskKind::App,
                module_path: "/input_echo".to_string(),
                pid: Some(pid),
                restarts: 0,
                spawn_arg: input.evt_input_echo_read as usize,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn input_echo: {:?}", e);
        }
    }

    info!("SPROUT: Compositor ready");
}

/// Set up network pipeline - spawn virtio_netd (driver) then netd (stack)
pub fn setup_network_stack(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up network stack...");
    info!("SPROUT: Network drivers are launched by devd");
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

    // Prefer native HDA path if present.
    let mut hda_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_SOUND_HDA_PCI_STUB, &mut hda_buf) {
        if count > 0 {
            let hda = hda_buf[0];
            info!("SPROUT: Found HDA sound device {:?}", hda);

            match stem::syscall::spawn_process("/hdaudio", 0) {
                Ok(pid) => {
                    info!("SPROUT: Spawned hdaudio (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2);
                    tasks.push(ManagedTask {
                        name: "/hdaudio".to_string(),
                        kind: TaskKind::Driver("dev.sound.hda".to_string()),
                        module_path: "/hdaudio".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                        spawn_arg: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn hdaudio: {:?}", e);
                    return;
                }
            }

            return;
        }
    }

            // Spawn virtio_sound driver if VirtIO Sound PCI device (0x040100)
            match stem::syscall::spawn_process("/virtio_sound", 0) {
                Ok(pid) => {
                    info!("SPROUT: Spawned virtio_sound (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2);
                    tasks.push(ManagedTask {
                        name: "/virtio_sound".to_string(),
                        kind: TaskKind::Driver("dev.sound.virtio".to_string()),
                        module_path: "/virtio_sound".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                        spawn_arg: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn virtio_sound: {:?}", e);
                    return;
                }
            }
        } else {
            info!("SPROUT: No Sound device found");
        }
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
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn beeper: {:?}", e);
        }
    }
}
