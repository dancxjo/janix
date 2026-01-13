use crate::task::{ManagedTask, TaskKind};
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
}

pub fn setup_display_pipeline(tasks: &mut Vec<ManagedTask>) -> Option<DisplayHandles> {
    info!("SPROUT: Setting up display pipeline...");

    let mut fb_buf = [ThingId(0); 1];
    let mut display_width = 0u32;
    let mut display_height = 0u32;
    let mut display_stride = 0u32;
    let mut display_format = 0u32;
    let mut driver_name: Option<&'static str> = None;
    let mut display_device: Option<ThingId> = None;

    if let Ok(count) = thingsys::find(kinds::DEV_DISPLAY_FRAMEBUFFER, &mut fb_buf) {
        if count > 0 {
            let fb = fb_buf[0];
            display_device = Some(fb);
            display_width = thingsys::prop_get(fb, keys::WIDTH).unwrap_or(0) as u32;
            display_height = thingsys::prop_get(fb, keys::HEIGHT).unwrap_or(0) as u32;
            display_stride = thingsys::prop_get(fb, keys::STRIDE).unwrap_or(0) as u32;
            display_format = thingsys::prop_get(fb, keys::FORMAT).unwrap_or(0) as u32;
            driver_name = Some("/display_bootfb");
        }
    }

    if driver_name.is_none() {
        let mut gpu_buf = [ThingId(0); 1];
        if let Ok(count) = thingsys::find(kinds::DEV_DISPLAY_GPU, &mut gpu_buf) {
            if count > 0 {
                display_device = Some(gpu_buf[0]);
                display_width = 800;
                display_height = 600;
                display_stride = display_width * 4;
                display_format = 1;
                driver_name = Some("/display_virtio_gpu");
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
    // arg0 = (kbd_read << 48) | (mouse_read << 32) | (evt_write << 16) | evt_echo_write
    // Using 16-bit handle slots
    let bristle_arg = ((kbd_raw.1 as u64) << 48)
        | ((mouse_raw.1 as u64) << 32)
        | ((evt.0 as u64) << 16)
        | (evt_echo.0 as u64);
    match stem::syscall::spawn_process("/bristle", bristle_arg as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned bristle (PID={})", pid);
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

    let (drv_req_write, drv_resp_read) = display
        .map(|d| (d.drv_req_write, d.drv_resp_read))
        .unwrap_or((0, 0));
    let bloom_arg =
        (drv_req_write as u64) | ((drv_resp_read as u64) << 16) | ((evt.1 as u64) << 32);
    info!(
        "SPROUT: Bloom handles req_w={} resp_r={} bristle_r={} arg=0x{:x}",
        drv_req_write, drv_resp_read, evt.1, bloom_arg
    );

    // Spawn bloom with packed handles
    match stem::syscall::spawn_process("/bloom", bloom_arg as usize) {
        Ok(pid) => {
            info!("SPROUT: Spawned bloom (PID={})", pid);
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
