use crate::task::{ManagedTask, TaskKind};
use abi::ids::HandleId;
use abi::schema::{keys, kinds};
use alloc::string::ToString;
use alloc::vec::Vec;
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

    // Create evt port (bristle -> bloom)
    let evt = match stem::syscall::port_create(8192) {
        Ok((write_h, read_h)) => {
            info!("SPROUT: Created evt port (w={}, r={})", write_h, read_h);
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create evt port: {:?}", e);
            return;
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
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn ps2_mouse: {:?}", e);
        }
    }

    // Create evt_echo port (bristle -> echo)
    let evt_echo = match stem::syscall::port_create(8192) {
        Ok((write_h, read_h)) => {
            info!(
                "SPROUT: Created evt_echo port (w={}, r={})",
                write_h, read_h
            );
            (write_h, read_h)
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to create evt_echo port: {:?}", e);
            return;
        }
    };

    // Spawn bristle with packed handles:
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
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bristle: {:?}", e);
        }
    }

    // Font handling is now integrated into Bloom. No standalone fontd service.

    let (drv_req_write, drv_resp_read, display_bs_id) = display
        .as_ref()
        .map(|d| (d.drv_req_write, d.drv_resp_read, d.bs_id))
        .unwrap_or((0, 0, ThingId::default()));

    // Bloom Bootstrap
    // Create bytespace to hold args
    // Layout:
    // 0: magic (0xBl00mArg)
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
            slice[3] = evt.1 as u32; // bristle read

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
            });
        }
        Err(e) => {
            stem::error!("SPROUT: Failed to spawn bloom: {:?}", e);
        }
    }

    // Spawn echo with evt_echo read handle
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

    // Check for VirtIO NIC device
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
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn virtio_netd: {:?}", e);
                    return; // Can't run netd without the driver
                }
            }

            // Spawn netd - the network stack that talks to virtio_netd via IPC
            match stem::syscall::spawn_process("/netd", 0) {
                Ok(pid) => {
                    info!("SPROUT: Spawned netd (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2); // Normal priority
                    tasks.push(ManagedTask {
                        name: "/netd".to_string(),
                        kind: TaskKind::Service("svc.net".to_string()),
                        module_path: "/netd".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn netd: {:?}", e);
                }
            }

            // Spawn fetchd - IP address display UI
            match stem::syscall::spawn_process("/fetchd", 0) {
                Ok(pid) => {
                    info!("SPROUT: Spawned fetchd (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2); // Normal priority
                    tasks.push(ManagedTask {
                        name: "/fetchd".to_string(),
                        kind: TaskKind::App,
                        module_path: "/fetchd".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn fetchd: {:?}", e);
                }
            }

            // Spawn anther - HTTP server
            match stem::syscall::spawn_process("/anther", 0) {
                Ok(pid) => {
                    info!("SPROUT: Spawned anther (PID={})", pid);
                    let _ = stem::thread::set_priority(pid, 2); // Normal priority
                    tasks.push(ManagedTask {
                        name: "/anther".to_string(),
                        kind: TaskKind::Service("svc.http".to_string()),
                        module_path: "/anther".to_string(),
                        pid: Some(pid),
                        restarts: 0,
                    });
                }
                Err(e) => {
                    warn!("SPROUT: Failed to spawn anther: {:?}", e);
                }
            }
        } else {
            info!("SPROUT: No NIC device found, skipping network pipeline");
        }
    } else {
        info!("SPROUT: No NIC device found, skipping network pipeline");
    }
}

/// Set up audio pipeline - spawn virtio_sound and beeper
pub fn setup_audio_pipeline(tasks: &mut Vec<ManagedTask>) {
    info!("SPROUT: Setting up audio pipeline...");

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
