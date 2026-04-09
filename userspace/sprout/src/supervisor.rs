use crate::registry::Registry;
use crate::task::{ManagedTask, TaskKind};
use abi::ids::HandleId;
use abi::schema::keys;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use stem::{info, warn};

pub struct Supervisor {
    tasks: Vec<ManagedTask>,
    registry: Registry,
    registry_ptr: usize,
    ledger: BTreeMap<String, u32>,
    pub supervisor_port: stem::syscall::ChannelHandle,
    pub supervisor_write: stem::syscall::ChannelHandle,
    next_bind_id: u64,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        let (supervisor_write, supervisor_read) = stem::syscall::channel_create(4096).expect("Failed to create supervisor port");
        Self {
            tasks: Vec::new(),
            registry: Registry::new(),
            registry_ptr,
            ledger: BTreeMap::new(),
            supervisor_port: supervisor_read,
            supervisor_write,
            next_bind_id: 1,
        }
    }

    pub fn run_forever(&mut self) -> ! {
        info!("SPROUT: Supervisor starting (phased mode)...");

        // --- STAGE 1: Discovery & Core Drivers ---
        info!("SPROUT: [Stage 1] Hardware Discovery and Core Drivers");
        self.discover();

        crate::pipelines::setup_rtc_pipeline(&mut self.tasks);
        crate::pipelines::setup_storage_pipeline(&mut self.tasks);
        crate::pipelines::setup_audio_driver(&mut self.tasks);
        let bind_id = self.next_bind_id;
        self.next_bind_id += 1;
        let display_handles = crate::pipelines::setup_display_pipeline(&mut self.tasks, self.supervisor_write, bind_id);
        let input_handles = crate::pipelines::setup_input_broker(&mut self.tasks);
        crate::pipelines::setup_network_stack(&mut self.tasks);

        // --- STAGE 2: Network & Core Services ---
        info!("SPROUT: [Stage 2] Starting Network Apps and Services");
        self.ensure_service("/devd", "svc.devd");
        self.ensure_service("/netd", "svc.net");

        crate::pipelines::setup_network_apps(&mut self.tasks);

        self.process_registrations();

        // info!("SPROUT: [Stage 3] -> Setting up Terminal...");
        // crate::pipelines::setup_terminal(&mut self.tasks, display_handles, input_handles);
        
        info!("SPROUT: [Stage 3] UI initialization triggered");

        // --- STAGE 4: Final Polish (Beeper) ---
        info!("SPROUT: [Stage 4] Proof of life (Beeper)");
        crate::pipelines::spawn_beeper(&mut self.tasks);

        self.process_registrations();

        // --- STAGE 5: User Apps ---
        info!("SPROUT: [Stage 5] Starting Discovered User Apps");
        self.discover();
        self.ensure_app("/sh");

        // Wait for essential drivers (Display) before launching UI apps.
        self.wait_for_display();

        crate::pipelines::setup_graphics_stack(&mut self.tasks);

        self.spawn_apps();

        // Enter monitor loop
        info!("SPROUT: Startup complete. Entering monitor loop.");
        loop {
            self.process_registrations();
            self.monitor();
            stem::yield_now();
            stem::sleep_ms(100);
        }
    }

    fn wait_for_display(&mut self) {
        info!("SPROUT: Waiting for display driver registration...");
        let start = stem::monotonic_ns();
        let timeout = 5_000_000_000; // 5 seconds

        loop {
            self.process_registrations();
            self.monitor();
            
            // Check if we have any display card in /dev/display
            if let Ok(fd) = stem::syscall::vfs::vfs_open("/dev/display/card0", stem::abi::syscall::vfs_flags::O_RDONLY) {
                let _ = stem::syscall::vfs::vfs_close(fd);
                info!("SPROUT: Display card0 detected. Proceeding.");
                break;
            }

            if stem::monotonic_ns() - start > timeout {
                warn!("SPROUT: Timeout waiting for display driver! UI may fail.");
                break;
            }

            stem::yield_now();
            stem::sleep_ms(50);
        }
    }

    fn discover(&mut self) {
        info!(
            "SPROUT: Discovering modules from registry at 0x{:x}...",
            self.registry_ptr
        );

        if self.registry_ptr == 0 {
            info!("SPROUT: No boot registry provided!");
            return;
        }

        let count = unsafe { *(self.registry_ptr as *const usize) };
        info!("SPROUT: Found {} modules natively from BootRegistry", count);

        let entries_ptr = (self.registry_ptr + core::mem::size_of::<usize>()) as *const usize;

        for i in 0..count {
            let name_ptr = unsafe { *entries_ptr.add(i * 2) };
            let name_len = unsafe { *entries_ptr.add(i * 2 + 1) };

            let name_bytes =
                unsafe { core::slice::from_raw_parts(name_ptr as *const u8, name_len) };
            let name = core::str::from_utf8(name_bytes).unwrap_or("").to_string();

            info!("SPROUT: Module[{}] = '{}'", i, name);

            if name.is_empty() {
                continue;
            }

            if name.contains("/drivers/") {
                // Handled natively by registry if needed
            } else if name.contains("/apps/")
                || name.ends_with("/idle")
                || name.ends_with("/hello_std")
                || name.ends_with("/wayland_hello")
                || (cfg!(feature = "diagnostic-apps")
                    && (name.ends_with("/threads_demo") || name.ends_with("/scheduler_verify")))
            {
                info!("SPROUT: Discovered app: {}", name);
                self.tasks.push(ManagedTask {
                    name: name.clone(),
                    kind: TaskKind::App,
                    module_path: name,
                    pid: None,
                    restarts: 0,
                    spawn_arg: 0,
                    bind_instance_id: 0,
                    drv_req_write: 0,
                    drv_resp_read: 0,
                });
            }
        }
    }

    fn spawn_apps(&mut self) {
        for task in self.tasks.iter_mut() {
            if let TaskKind::App = task.kind {
                if task.pid.is_some() {
                    continue;
                }
                info!("SPROUT: Launching app '{}'", task.name);
                match stem::syscall::spawn_process(&task.name, 0) {
                    Ok(pid) => {
                        info!("SPROUT: App launched (PID={})", pid);
                        task.pid = Some(pid);
                        let _ = stem::thread::set_priority(pid, 2);
                    }
                    Err(e) => info!("SPROUT: Failed to launch app '{}': {:?}", task.name, e),
                }
            }
        }
    }

    fn ensure_app(&mut self, name: &str) {
        if self.tasks.iter().any(|t| t.name.contains(name)) {
            return;
        }

        let full = format!("/bin{}", name);
        self.tasks.push(ManagedTask {
            name: "/bin/login".to_string(),
            kind: TaskKind::App,
            module_path: "/bin/login".to_string(),
            pid: None,
            restarts: 0,
            spawn_arg: 0,
            bind_instance_id: 0,
            drv_req_write: 0,
            drv_resp_read: 0,
        });
    }

    fn ensure_service(&mut self, name: &str, service_kind: &str) {
        if self.tasks.iter().any(|t| t.name.contains(name)) {
            return;
        }

        let full = format!("/bin{}", name);
        self.tasks.push(ManagedTask {
            name: full.clone(),
            kind: TaskKind::Service(service_kind.to_string()),
            module_path: full,
            pid: None,
            restarts: 0,
            spawn_arg: 0,
            bind_instance_id: 0,
            drv_req_write: 0,
            drv_resp_read: 0,
        });
    }

    fn spawn_discovered_apps(&mut self) {
        for task in self.tasks.iter_mut() {
            if let TaskKind::App = task.kind {
                if task.pid.is_some() {
                    continue;
                }
                match stem::syscall::spawn_process(&task.name, 0) {
                    Ok(pid) => {
                        task.pid = Some(pid);
                    }
                    Err(_) => {}
                }
            }
        }
    }

    fn monitor(&mut self) {
        for task in self.tasks.iter_mut() {
            if let Some(pid) = task.pid {
                match stem::syscall::task_poll(pid) {
                    Ok((status, code)) => {
                        if status == stem::abi::types::TaskStatus::Dead {
                            info!(
                                "SPROUT: Task '{}' (PID {}) died with code {}. Restarting...",
                                task.name, pid, code
                            );
                            task.pid = None;
                            task.restarts += 1;
                            let arg = task.spawn_arg;
                            stem::sleep_ms(100);
                            match stem::syscall::spawn_process(&task.name, arg) {
                                Ok(new_pid) => {
                                    task.pid = Some(new_pid);
                                }
                                Err(_) => {}
                            }
                        }
                    }
                    Err(_) => {
                        task.pid = None;
                    }
                }
            }
        }
    }

    fn process_registrations(&mut self) {
        use abi::supervisor_protocol::{self, classes, MSG_BIND_READY, MSG_BIND_ASSIGNED};
        use abi::display_driver_protocol;
        use stem::syscall::{vfs_mount, channel_send_all};

        let mut buf = [0u8; 1024];

        // We check EACH task's private response channel
        for task in self.tasks.iter_mut() {
            if task.drv_resp_read == 0 || task.pid.is_none() {
                continue;
            }

            while let Ok(n) = stem::syscall::channel_try_recv(task.drv_resp_read, &mut buf) {
                if let Some((header, payload)) = display_driver_protocol::parse_message(&buf[..n]) {
                    if header.msg_type == MSG_BIND_READY {
                        if let Some(ready) = supervisor_protocol::decode_bind_ready_le(payload) {
                            let task_name = task.name.clone();
                            info!("SPROUT: BIND_READY from {} (ID: {}, Classes: 0x{:x})", task_name, ready.bind_instance_id, ready.class_mask);

                            // 2. Extract provider port
                            // Since we ensure drivers send the handle BEFORE the BIND_READY byte,
                            // it should be here. We retry a few times just in case.
                            let mut provider_port = 0;
                            for _ in 0..10 {
                                if let Ok(p) = stem::syscall::channel_recv_handle(task.drv_resp_read) {
                                    provider_port = p;
                                    break;
                                }
                                stem::yield_now();
                            }

                            if provider_port != 0 {
                                // 3. Deterministic allocation
                                let (class_name, root) = if ready.class_mask & classes::DISPLAY_CARD != 0 {
                                    ("display", "/dev/display/card")
                                } else if ready.class_mask & classes::INPUT_EVENT != 0 {
                                    ("input", "/dev/input/event")
                                } else if ready.class_mask & classes::BLOCK_DEVICE != 0 {
                                    ("block", "/dev/block/sd")
                                } else if ready.class_mask & classes::NETWORK_INTERFACE != 0 {
                                    ("net", "/dev/net/virtio")
                                } else if ready.class_mask & classes::SOUND_CARD != 0 {
                                    ("sound", "/dev/sound/card")
                                } else {
                                    ("misc", "/dev/misc/device")
                                };

                                let unit = self.ledger.get(class_name).cloned().unwrap_or(0);
                                self.ledger.insert(class_name.to_string(), unit + 1);
                                let path = format!("{}{}", root, unit);

                                // 4. Mount
                                match vfs_mount(provider_port, &path) {
                                    Ok(()) => {
                                        info!("SPROUT: Sovereign mount success: {} -> {}", task_name, path);
                                        
                                        // 5. Reply to driver
                                        let mut assigned = supervisor_protocol::BindAssignedPayload {
                                            bind_instance_id: ready.bind_instance_id,
                                            status: 0,
                                            unit_number: unit,
                                            primary_path: [0u8; 64],
                                        };
                                        let path_bytes = path.as_bytes();
                                        let len = path_bytes.len().min(64);
                                        assigned.primary_path[..len].copy_from_slice(&path_bytes[..len]);

                                        let mut reply_buf = [0u8; 256];
                                        let mut payload_bytes = [0u8; supervisor_protocol::BIND_ASSIGNED_PAYLOAD_SIZE];
                                        if let Some(p_len) = supervisor_protocol::encode_bind_assigned_le(&assigned, &mut payload_bytes) {
                                            if let Some(total_len) = display_driver_protocol::encode_message(&mut reply_buf, MSG_BIND_ASSIGNED, &payload_bytes[..p_len]) {
                                                let _ = channel_send_all(task.drv_req_write, &reply_buf[..total_len]);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        warn!("SPROUT: Sovereign mount FAILED for {}: {:?}", task_name, e);
                                    }
                                }
                            }
                        } else {
                            warn!("SPROUT: BIND_READY with UNKNOWN ID");
                        }
                    }
                }
            }
        }
    }
}
