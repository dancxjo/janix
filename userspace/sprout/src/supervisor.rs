//! Supervisor service for Thing-OS (sprout)
//!
//! Orchestrates the system boot sequence:
//! 1. VFS Namespace management (via memfds and mounts).
//! 2. Sovereign registration handshake (receiving handles from drivers).
//! 3. Graphics stack bring-up (coordinating display + fonts + bloom).
//! 4. Monitoring device arrivals and spawning dependent services.

#![feature(restricted_std)]
#![no_main]

extern crate alloc;

// Modules are now declared in main.rs
use crate::ledger::DeviceLedger;
use crate::pipelines::{DisplayHandles, setup_display_pipeline, setup_graphics_stack, setup_input_broker, setup_serial_shell};
use crate::task::{ManagedTask, TaskKind};
use abi::supervisor_protocol::{self, classes, MSG_BIND_READY, MSG_BIND_ASSIGNED};
use abi::display_driver_protocol;
use alloc::format;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;
use stem::syscall::{channel_create, channel_send_all, vfs_mount, ChannelHandle};
use stem::{error, info, warn};

pub struct Supervisor {
    pub tasks: Arc<Mutex<Vec<ManagedTask>>>,
    pub ledger: Arc<Mutex<DeviceLedger>>,
    pub registry_ptr: usize,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
            ledger: Arc::new(Mutex::new(DeviceLedger::new())),
            registry_ptr,
        }
    }

    pub fn run_forever(&mut self) -> ! {
        stem::debug!("SPROUT: Supervisor session started (Sovereign mode)");

        // Stage 1: Discover boot modules
        self.discover();

        // Stage 2: Create Sovereign Registrar channel
        let (supervisor_write, supervisor_read) =
            channel_create(4096).expect("Failed to create supervisor registrar channel");

        // Stage 3: Launch Serial Shell EARLY on its own processor
        info!("SPROUT: Launching early serial shell...");
        let tasks_cloned = self.tasks.clone();
        let _ = stem::thread::spawn_task(move || {
            setup_serial_shell(tasks_cloned);
        });

        // Stage 4: Fan-out Setup Pipelines in parallel
        info!("SPROUT: Fanning out setup pipelines...");
        
        let tasks_for_display = self.tasks.clone();
        let _ = stem::thread::spawn_task(move || {
            let _ = setup_display_pipeline(tasks_for_display, 0, 1);
        });

        let tasks_for_graphics = self.tasks.clone();
        let _ = stem::thread::spawn_task(move || {
            setup_graphics_stack(tasks_for_graphics);
        });

        let tasks_for_input = self.tasks.clone();
        let _ = stem::thread::spawn_task(move || {
            setup_input_broker(tasks_for_input);
        });

        // Stage 8: Run Readiness Model Verification Test
        info!("SPROUT: Spawning poll_mux verification test...");
        let _ = stem::syscall::spawn_process("/bin/poll_mux", 0);

        // Stage 4: Busy Stage - Wait for Display Driver to register its VFS provider
        // self.wait_for_display();

        info!("SPROUT: System bring-up COMPLETE. Entering supervisor loop.");

        loop {
            // Process any new registrations (networking, sound, input, etc)
            self.process_registrations();

            // Monitor already running tasks
            self.monitor();

            // Relinquish some time
            stem::syscall::yield_now();
            stem::sleep_ms(100);
        }
    }

    fn wait_for_display(&mut self) {
        stem::debug!("SPROUT: Waiting for display driver registration...");
        let start = stem::monotonic_ns();
        let timeout = 5_000_000_000; // 5 seconds
        let mut step = 0;

        loop {
            self.process_registrations();
            self.monitor();
            
            if step % 20 == 0 {
                stem::debug!("SPROUT: Still waiting for display (step {})...", step);
            }
            if step % 100 == 0 {
                stem::debug!("SPROUT: Health check: Loop still running, tasks={}", self.tasks.lock().len());
            }
            step += 1;
            
            if let Ok(fd) = stem::syscall::vfs::vfs_open("/dev/display/card0", stem::abi::syscall::vfs_flags::O_RDONLY) {
                let _ = stem::syscall::vfs::vfs_close(fd);
                stem::info!("SPROUT: Display card0 detected. Proceeding.");
                break;
            }

            if stem::monotonic_ns() - start > timeout {
                warn!("SPROUT: Timeout waiting for display driver! UI may fail.");
                break;
            }

            stem::sleep_ms(100);
        }
    }

    fn discover(&mut self) {
        // We no longer auto-spawn everything in /bin. 
        // We only scan to keep the registry metadata if needed.
        stem::debug!("SPROUT: Discovery loop disabled in favor of devd.");
    }

    fn spawn_devd(&mut self) {
        let (write, read) = match stem::syscall::channel_create(4096) {
            Ok(h) => h,
            Err(_) => return,
        };
        // We could pass the registrar port to devd if it needs to register things,
        // but for now devd just spawns drivers.
        match stem::syscall::spawn_process("/bin/devd", 0) {
            Ok(pid) => {
                info!("SPROUT: Spawned devd (PID={})", pid);
                let mut tasks = self.tasks.lock();
                tasks.push(ManagedTask {
                    name: "devd".to_string(),
                    kind: TaskKind::Service("svc.devd".to_string()),
                    module_path: "/bin/devd".to_string(),
                    pid: Some(pid),
                    restarts: 0,
                    spawn_arg: 0,
                    bind_instance_id: 0,
                    drv_req_write: write,
                    drv_resp_read: read,
                    boot_req_read: 0,
                    boot_resp_write: 0,
                });
            }
            Err(e) => warn!("SPROUT: Failed to spawn devd: {:?}", e),
        }
    }


    fn monitor(&mut self) {
        let mut tasks = self.tasks.lock();
        for task in tasks.iter_mut() {
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
                            stem::sleep_ms(100);
                        }
                    }
                    Err(_) => {
                        task.pid = None;
                    }
                }
            }

            // Spawn or Restart
            if task.pid.is_none() {
                let handles = if task.boot_req_read != 0 && task.boot_resp_write != 0 {
                    &[task.boot_req_read as u64, task.boot_resp_write as u64] as &[u64]
                } else {
                    &[] as &[u64]
                };

                let arg_str = alloc::format!("{}", task.spawn_arg);
                let spawn_res = stem::syscall::spawn_process_ex(
                    &task.name,
                    &[task.name.as_bytes(), arg_str.as_bytes()],
                    &alloc::collections::BTreeMap::new(),
                    stem::abi::types::stdio_mode::INHERIT,
                    stem::abi::types::stdio_mode::INHERIT,
                    stem::abi::types::stdio_mode::INHERIT,
                    task.spawn_arg as u64,
                    handles,
                );

                if let Ok(resp) = spawn_res {
                    task.pid = Some(resp.child_tid);
                    if let TaskKind::Driver(_) = task.kind {
                        let _ = stem::thread::set_priority(resp.child_tid, 3);
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

        let mut tasks_vec = self.tasks.lock();
        if tasks_vec.is_empty() {
             // stem::debug!("SPROUT: No tasks to process registrations for.");
        }

        // We check EACH task's private response channel
        for task in tasks_vec.iter_mut() {
            if task.drv_resp_read == 0 || task.pid.is_none() {
                continue;
            }

            // stem::debug!("SPROUT: Polling task {} on port {}...", task.name, task.drv_resp_read);

            while let Ok(n) = stem::syscall::channel_try_recv(task.drv_resp_read, &mut buf) {
                stem::debug!("SPROUT: Received {} bytes from task {} (tid={})", n, task.name, task.pid.unwrap_or(0));
                if let Some((header, payload)) = display_driver_protocol::parse_message(&buf[..n]) {
                    stem::debug!("SPROUT: Received message type {} from {}", header.msg_type, task.name);
                    if header.msg_type == MSG_BIND_READY {
                        if let Some(ready) = supervisor_protocol::decode_bind_ready_le(payload) {
                            let task_name = task.name.clone();
                            stem::debug!("SPROUT: BIND_READY from {} (ID: {}, Classes: 0x{:x})", task_name, ready.bind_instance_id, ready.class_mask);

                            // 1. Extract provider port
                            // Drivers send the vfs handle BEFORE the BIND_READY message
                            let mut provider_port = 0;
                            match stem::syscall::channel_recv_handle(task.drv_resp_read) {
                                Ok(p) => {
                                    provider_port = p;
                                    stem::debug!("SPROUT: Received VFS provider handle {} from {}", p, task_name);
                                }
                                Err(e) => {
                                    warn!("SPROUT: Failed to receive VFS provider handle from {}: {:?}", task_name, e);
                                }
                            }

                            if provider_port != 0 {
                                // 2. Deterministic allocation
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

                                let mut ledger = self.ledger.lock();
                                let unit = ledger.get(class_name).cloned().unwrap_or(0);
                                ledger.insert(class_name.to_string(), unit + 1);
                                let path = format!("{}{}", root, unit);

                                // 3. Mount
                                match vfs_mount(provider_port, &path) {
                                    Ok(()) => {
                                        stem::debug!("SPROUT: Sovereign mount success: {} -> {}", task_name, path);
                                        
                                        // 4. Reply to driver
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
                        }
                    }
                }
            }
        }
    }
}
