use crate::registry::Registry;
use crate::task::{ManagedTask, TaskKind};
use abi::ids::HandleId;
use abi::schema::keys;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::info;

pub struct Supervisor {
    tasks: Vec<ManagedTask>,
    registry: Registry,
    registry_ptr: usize,
}

impl Supervisor {
    pub fn new(registry_ptr: usize) -> Self {
        Self {
            tasks: Vec::new(),
            registry: Registry::new(),
            registry_ptr,
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
        let display_handles = crate::pipelines::setup_display_pipeline(&mut self.tasks);
        let input_handles = crate::pipelines::setup_input_broker(&mut self.tasks);
        crate::pipelines::setup_network_stack(&mut self.tasks);
        crate::pipelines::setup_graphics_stack(&mut self.tasks);

        // --- STAGE 2: Network & Core Services ---
        info!("SPROUT: [Stage 2] Starting Network Apps and Services");
        self.ensure_service("/devd", "svc.devd");
        self.ensure_service("/netd", "svc.net");

        crate::pipelines::setup_network_apps(&mut self.tasks);


        // info!("SPROUT: [Stage 3] -> Setting up Terminal...");
        // crate::pipelines::setup_terminal(&mut self.tasks, display_handles, input_handles);
        
        info!("SPROUT: [Stage 3] UI initialization triggered");

        // --- STAGE 4: Final Polish (Beeper) ---
        info!("SPROUT: [Stage 4] Proof of life (Beeper)");
        crate::pipelines::spawn_beeper(&mut self.tasks);

        // --- STAGE 5: User Apps ---
        info!("SPROUT: [Stage 5] Starting Discovered User Apps");
        self.spawn_discovered_apps();

        self.ensure_app("/sh");
        self.spawn_apps();

        // Enter monitor loop
        info!("SPROUT: Startup complete. Entering monitor loop.");
        loop {
            self.monitor();
            stem::yield_now();
            stem::sleep_ms(100);
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
            name: full.clone(),
            kind: TaskKind::App,
            module_path: full,
            pid: None,
            restarts: 0,
            spawn_arg: 0,
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
}
