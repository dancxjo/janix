use crate::task::{ManagedTask, TaskKind};
use abi::schema::{keys, kinds};
use alloc::string::ToString;
use alloc::vec::Vec;
use stem::abi::driver_ctx::DriverCtx;
use stem::syscall::{port_create, PortHandle};
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use stem::{info, warn};

pub struct DisplayHandles {
    pub drv_req_write: PortHandle,
    pub drv_resp_read: PortHandle,
    pub bs_id: ThingId,
    /// Which display backend was selected
    pub backend_name: &'static str,
}

fn has_kind(kind: &str) -> bool {
    let mut buf = [ThingId::default(); 1];
    matches!(thingsys::find(kind, &mut buf), Ok(count) if count > 0)
}

fn has_ahci_controller() -> bool {
    let mut funcs = [ThingId::default(); 64];
    let count = thingsys::find(kinds::DEV_PCI_FUNCTION, &mut funcs).unwrap_or(0);
    for f in funcs.iter().take(count) {
        let class = thingsys::prop_get(*f, keys::CLASS_CODE).unwrap_or(0);
        let sub = thingsys::prop_get(*f, keys::SUBCLASS_CODE).unwrap_or(0);
        let prog = thingsys::prop_get(*f, keys::PROG_IF).unwrap_or(0);
        if class == 0x01 && sub == 0x06 && prog == 0x01 {
            return true;
        }
    }
    false
}

pub fn setup_pci_stub_pipeline(tasks: &mut Vec<ManagedTask>) {
    let needs_stubd = has_kind(kinds::DEV_PCI_FUNCTION);

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
    let mut rtcs = [ThingId::default(); 1];
    let count = thingsys::find(kinds::DEV_RTC_CMOS, &mut rtcs).unwrap_or(0);
    if count == 0 {
        info!("SPROUT: No RTC CMOS device found, skipping rtc_cmos");
        return;
    }

    let rtc = rtcs[0];
    let arg = DriverCtx { device_id: rtc }.to_raw();
    match stem::syscall::spawn_process("/rtc_cmos", arg) {
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
        return;
    }

    if has_kind(kinds::DEV_BUS_LEGACY_IO) {
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

    // Check for VirtIO GPU first (preferred for accelerated display)
    let mut gpu_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_DISPLAY_GPU, &mut gpu_buf) {
        if count > 0 {
            display_device = Some(gpu_buf[0]);
            
            // Read native resolution from boot framebuffer if available
            let mut fb_buf = [ThingId::default(); 1];
            if let Ok(fb_count) = thingsys::find(kinds::DEV_DISPLAY_FRAMEBUFFER, &mut fb_buf) {
                if fb_count > 0 {
                    let fb = fb_buf[0];
                    display_width = thingsys::prop_get(fb, keys::WIDTH).unwrap_or(1024) as u32;
                    display_height = thingsys::prop_get(fb, keys::HEIGHT).unwrap_or(768) as u32;
                }
            }
            
            // Fall back to reasonable default if no bootfb
            if display_width == 0 || display_height == 0 {
                display_width = 1024;
                display_height = 768;
            }
            
            display_stride = display_width * 4;
            display_format = 1;
            driver_name = Some("/display_virtio_gpu");
            backend_name = "VirtIO-GPU";
            info!("SPROUT: Using VirtIO GPU at {}x{}", display_width, display_height);
        }
    }

    // Fallback to BootFB if no VirtIO GPU found
    if driver_name.is_none() {
        let mut fb_buf = [ThingId::default(); 1];
        if let Ok(count) = thingsys::find(kinds::DEV_DISPLAY_FRAMEBUFFER, &mut fb_buf) {
            if count > 0 {
                let fb = fb_buf[0];
                display_device = Some(fb);
                display_width = thingsys::prop_get(fb, keys::WIDTH).unwrap_or(0) as u32;
                display_height = thingsys::prop_get(fb, keys::HEIGHT).unwrap_or(0) as u32;
                display_stride = thingsys::prop_get(fb, keys::STRIDE).unwrap_or(0) as u32;
                display_format = thingsys::prop_get(fb, keys::FORMAT).unwrap_or(0) as u32;
                driver_name = Some("/display_bootfb");
                backend_name = "BootFB";
                info!("SPROUT: Using boot framebuffer (fallback)");
            }
        }
    }

    let driver_name = match driver_name {
        Some(name) => name,
        None => {
            warn!("SPROUT: No display device found, skipping display pipeline");
            return None;
        }
    };

    if display_width == 0 || display_height == 0 || display_stride == 0 {
        warn!("SPROUT: Invalid display geometry, skipping display pipeline");
        return None;
    }

    info!(
        "SPROUT: Display backend: {} ({}x{} stride={})",
        backend_name, display_width, display_height, display_stride
    );

    let size = (display_height as usize) * (display_stride as usize);
    let bs_id = match thingsys::bytespace_create(size, 0, display_format as u64) {
        Ok(id) => id,
        Err(e) => {
            warn!("SPROUT: bytespace_create failed: {:?}", e);
            return None;
        }
    };

    let role_sym = match thingsys::intern("display.compositor") {
        Ok(sym) => sym,
        Err(_) => 0,
    };
    let _ = thingsys::prop_set(bs_id, "display_role", role_sym as u64);
    let _ = thingsys::prop_set(bs_id, keys::WIDTH, display_width as u64);
    let _ = thingsys::prop_set(bs_id, keys::HEIGHT, display_height as u64);
    let _ = thingsys::prop_set(bs_id, keys::STRIDE, display_stride as u64);
    let _ = thingsys::prop_set(bs_id, keys::FORMAT, display_format as u64);

    // Store backend name as a property so Bloom can query it
    if let Ok(backend_sym) = thingsys::intern(backend_name) {
        let _ = thingsys::prop_set(bs_id, "display_backend", backend_sym as u64);
    }

    let drv_req = match port_create(4096) {
        Ok(handles) => handles,
        Err(e) => {
            warn!("SPROUT: drv_req port_create failed: {:?}", e);
            return None;
        }
    };
    let drv_resp = match port_create(4096) {
        Ok(handles) => handles,
        Err(e) => {
            warn!("SPROUT: drv_resp port_create failed: {:?}", e);
            return None;
        }
    };

    let _ = thingsys::prop_set(bs_id, "display_drv_req", drv_req.0 as u64);
    let _ = thingsys::prop_set(bs_id, "display_drv_resp", drv_resp.1 as u64);

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
            let _ = thingsys::link(drv_node, "CONSUMES", bs_id);
            if let Some(dev) = display_device {
                let _ = thingsys::link(drv_node, "PRESENTS_TO", dev);
            }
        }
    }

    Some(DisplayHandles {
        drv_req_write: drv_req.0,
        drv_resp_read: drv_resp.1,
        bs_id,
        backend_name,
    })
}

pub fn setup_input_pipeline(tasks: &mut Vec<ManagedTask>, display: Option<DisplayHandles>) {
    info!("SPROUT: Setting up input pipeline (keyboard + mouse)...");

    // Create kbd_raw port (ps2_kbd -> bristle)
    let kbd_raw = match stem::syscall::port_create(4096) {
        Ok((write_h, read_h)) => {
            info!("SPROUT: Created kbd_raw port (w={}, r={})", write_h, read_h);
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create kbd_raw port: {:?}", e);
            return;
        }
    };

    // Create mouse_raw port (ps2_mouse -> bristle)
    let mouse_raw = match stem::syscall::port_create(4096) {
        Ok((write_h, read_h)) => {
            info!(
                "SPROUT: Created mouse_raw port (w={}, r={})",
                write_h, read_h
            );
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create mouse_raw port: {:?}", e);
            return;
        }
    };

    // Note: evt port no longer needed - bloom and echo self-register as input subscribers

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
                    spawn_arg: 0,
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
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn ps2_mouse: {:?}", e);
        }
    }

    // Event ports (legacy fan-out)
    let evt = stem::syscall::port_create(8192).unwrap_or((0, 0));
    let evt_echo = stem::syscall::port_create(8192).unwrap_or((0, 0));

    // Spawn bristle with packed handles:
    // Layout: kbd_raw_read[63:48] | mouse_raw_read[47:32] | evt_write[31:16] | evt_echo_write[15:0]
    let bristle_arg = ((kbd_raw.1 as u64) << 48)
        | ((mouse_raw.1 as u64) << 32)
        | ((evt.0 as u64) << 16)
        | (evt_echo.0 as u64);

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
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bristle: {:?}", e);
        }
    }

    // Font handling is now integrated into Bloom. No standalone fontd service.

    // Extract display handles early for bloom compositor
    let (drv_req_write, drv_resp_read, display_bs_id) = display
        .as_ref()
        .map(|d| (d.drv_req_write, d.drv_resp_read, d.bs_id))
        .unwrap_or((0, 0, ThingId::default()));

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
    let boot_bs = thingsys::bytespace_create(boot_size, 0, 0).unwrap_or(ThingId::default());

    if boot_bs.to_u64_lossy() != 0 {
        use stem::thing::sys::{bytespace_map, bytespace_unmap};
        if let Ok(ptr) = bytespace_map(boot_bs) {
            let slice = unsafe { core::slice::from_raw_parts_mut(ptr as *mut u32, boot_size / 4) };
            slice[0] = 0xB100AA01; // Magic
            slice[1] = drv_req_write as u32;
            slice[2] = drv_resp_read as u32;
            slice[3] = evt.1 as u32; // Pass legacy event handle
            info!("SPROUT: Writing bloom BS: drv_req={}, drv_resp={}, bristle_evt={}", drv_req_write, drv_resp_read, evt.1);

            // Display bytespace id (u64 split into two u32s)
            let bs = display_bs_id.to_u64_lossy();
            slice[4] = bs as u32;
            slice[5] = (bs >> 32) as u32;

            let _ = bytespace_unmap(boot_bs, ptr);
        }
    }

    let bloom_arg = boot_bs.to_u64_lossy() as usize;

    let backend_info = display.as_ref().map(|d| d.backend_name).unwrap_or("none");
    info!(
        "SPROUT: Bloom handles via BS={} backend={}",
        boot_bs.to_u64_lossy(),
        backend_info
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

    // Spawn echo with legacy port handle
    match stem::syscall::spawn_process("/echo", evt_echo.1 as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned echo (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/echo".to_string(),
                kind: TaskKind::App,
                module_path: "/echo".to_string(),
                pid: Some(pid),
                restarts: 0,
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn echo: {:?}", e);
        }
    }

    info!("SPROUT: Input pipeline ready (keyboard + mouse)");
}

/// Set up network pipeline - spawn virtio_netd (driver) then netd (stack)
pub fn setup_network_pipeline(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up network pipeline...");

    // Prefer native RTL8168 driver if present.
    let mut rtl_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_NET_PCI_STUB, &mut rtl_buf) {
        if count > 0 {
            let rtl_dev = rtl_buf[0];
            let vendor = thingsys::prop_get(rtl_dev, keys::VENDOR_ID).unwrap_or(0) as u16;
            let device = thingsys::prop_get(rtl_dev, keys::DEVICE_ID).unwrap_or(0) as u16;

            if vendor == 0x10ec && device == 0x8168 {
                info!("SPROUT: Found RTL8168 NIC {:?}, spawning rtl8168d", rtl_dev);

                match stem::syscall::spawn_process("/rtl8168d", 0) {
                    Ok(pid) => {
                        info!("SPROUT: Spawned rtl8168d (PID={})", pid);
                        let _ = stem::thread::set_priority(pid, 2);
                        tasks.push(ManagedTask {
                            name: "/rtl8168d".to_string(),
                            kind: TaskKind::Driver("dev.net.rtl8168".to_string()),
                            module_path: "/rtl8168d".to_string(),
                            pid: Some(pid),
                            restarts: 0,
                    spawn_arg: 0,
                        });
                    }
                    Err(e) => {
                        warn!("SPROUT: Failed to spawn rtl8168d: {:?}", e);
                        return;
                    }
                }

                spawn_net_stack_services(tasks);

                return;
            }
        }
    }

    // Fall back to VirtIO NIC device
    let mut nic_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_NET_NIC, &mut nic_buf) {
        if count > 0 {
            let nic = nic_buf[0];
            info!("SPROUT: Found NIC device {:?}", nic);

            // Spawn virtio_netd first - the hardware driver that owns the NIC
            match stem::syscall::spawn_process("/virtio_netd", nic.to_u64_lossy() as usize) {
                Ok(pid) => {
                    info!("SPROUT: Spawned virtio_netd (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2); // Normal priority
                    tasks.push(ManagedTask {
                        name: "/virtio_netd".to_string(),
                        kind: TaskKind::Driver("dev.net.virtio".to_string()),
                        module_path: "/virtio_netd".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                    spawn_arg: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn virtio_netd: {:?}", e);
                }
            }

            spawn_net_stack_services(tasks);
        } else {
            info!("SPROUT: No NIC device found, starting net services without NIC driver");
            spawn_net_stack_services(tasks);
        }
    } else {
        info!("SPROUT: No NIC device found, starting net services without NIC driver");
        spawn_net_stack_services(tasks);
    }
}

fn spawn_net_stack_services(tasks: &mut Vec<ManagedTask>) {
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

    match stem::syscall::spawn_process("/anther", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned anther (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/anther".to_string(),
                kind: TaskKind::Service("svc.http".to_string()),
                module_path: "/anther".to_string(),
                pid: Some(pid),
                restarts: 0,
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn anther: {:?}", e);
        }
    }
}

pub fn setup_clock_service(tasks: &mut Vec<ManagedTask>) {
    match stem::syscall::spawn_process("/clock", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned clock (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 2);
            tasks.push(ManagedTask {
                name: "/clock".to_string(),
                kind: TaskKind::Service("svc.clock".to_string()),
                module_path: "/clock".to_string(),
                pid: Some(pid),
                restarts: 0,
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn clock: {:?}", e);
        }
    }
}

pub fn setup_taskman_service(tasks: &mut Vec<ManagedTask>) {
    match stem::syscall::spawn_process("/taskman", 0) {
        Ok(pid) => {
            info!("SPROUT: Spawned taskman (PID={})", pid);
            let _ = stem::thread::set_priority(pid, 1); // Low priority — background UI
            tasks.push(ManagedTask {
                name: "/taskman".to_string(),
                kind: TaskKind::Service("svc.taskman".to_string()),
                module_path: "/taskman".to_string(),
                pid: Some(pid),
                restarts: 0,
                    spawn_arg: 0,
            });
        }
        Err(e) => {
            warn!("SPROUT: Failed to spawn taskman: {:?}", e);
        }
    }
}

pub fn setup_ui_services(tasks: &mut Vec<ManagedTask>) {
    spawn_ui_service(tasks, "/flytrap", "svc.flytrap", 2);
    spawn_ui_service(tasks, "/fontd", "svc.fontd", 2);
    spawn_ui_service(tasks, "/blossom", "svc.blossom", 2);
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



/// Set up audio pipeline - spawn virtio_sound and beeper
pub fn setup_audio_pipeline(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up audio pipeline...");

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

            return;
        }
    }

    // Check for VirtIO Sound device
    let mut snd_buf = [ThingId::default(); 1];
    if let Ok(count) = thingsys::find(kinds::DEV_SOUND, &mut snd_buf) {
        if count > 0 {
            let snd = snd_buf[0];
            info!("SPROUT: Found Sound device {:?}", snd);

            // Spawn virtio_sound driver
            match stem::syscall::spawn_process("/virtio_sound", snd.to_u64_lossy() as usize) {
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

            // Spawn beeper demo
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
        } else {
            info!("SPROUT: No Sound device found");
        }
    } else {
        info!("SPROUT: No Sound device found");
    }
}
