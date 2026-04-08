use crate::registry::Registry;
use crate::task::{ManagedTask, TaskKind};
use abi::schema::keys;
use abi::schema::kinds as abi_kinds;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use stem::info;
use stem::thing::sys as thingsys;
use stem::thing::ThingId;
use abi::ids::HandleId;

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

        // Settle hardware phase
        stem::sleep_ms(100);

        // --- STAGE 2: Network & Core Services ---
        info!("SPROUT: [Stage 2] Starting Network Apps and Services");
        self.ensure_service("/devd", "svc.devd");
        self.ensure_service("/netd", "svc.net");
        self.ensure_service("/fontd", "svc.FontD");

        crate::pipelines::setup_network_apps(&mut self.tasks);

        stem::sleep_ms(100);

        // --- STAGE 3: Compositor ---
        info!("SPROUT: [Stage 3] Starting Compositor");
        crate::pipelines::setup_compositor(&mut self.tasks, display_handles, input_handles);

        stem::sleep_ms(200);

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

    #[allow(dead_code)]
    fn run_forever_full(&mut self) -> ! {
        info!("SPROUT: Supervisor starting...");

        self.discover();
        crate::pipelines::setup_pci_stub_pipeline(&mut self.tasks);
        crate::pipelines::setup_rtc_pipeline(&mut self.tasks);
        crate::pipelines::setup_storage_pipeline(&mut self.tasks);
        crate::pipelines::setup_audio_driver(&mut self.tasks);

        self.spawn_apps();

        let display_handles = crate::pipelines::setup_display_pipeline(&mut self.tasks);
        self.match_and_spawn_drivers();
        let input_handles = crate::pipelines::setup_input_broker(&mut self.tasks);
        crate::pipelines::setup_compositor(&mut self.tasks, display_handles, input_handles);

        crate::pipelines::setup_network_stack(&mut self.tasks);
        crate::pipelines::setup_network_apps(&mut self.tasks);
        crate::pipelines::setup_taskman_service(&mut self.tasks);

        crate::pipelines::spawn_beeper(&mut self.tasks);

        info!("SPROUT: Entering supervisor loop.");

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

        // Registry holds: [count: usize] followed by array of [ptr: usize, len: usize]
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
                // Handled natively by registry if needed, though they aren't parsed dynamically in phase 1.
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

        // We comment out self.registry.scan() since it uses sys_root_find.
        // self.registry.scan();
    }

    // `get_module_name` and `debug_module` are no longer practically used (but kept to avoid unused warnings below).
    fn get_module_name(&self, mod_id: ThingId) -> String {
        let mut buf = [0u8; 1024];
        if let Ok(len) = thingsys::describe_thing(mod_id, &mut buf) {
            let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
            // handle both historical `name="..."` and newer `name: "..."` renderings
            if let Some(rest) = s
                .split_once("name: \"")
                .map(|(_, r)| r)
                .or_else(|| s.split_once("name=\"").map(|(_, r)| r))
            {
                if let Some(end) = rest.find('"') {
                    return rest[..end].to_string();
                }
            }
        }
        String::new()
    }

    fn debug_module(&self, mod_id: ThingId) {
        let mut buf = [0u8; 1024];

        if let Ok(len) = thingsys::describe_thing(mod_id, &mut buf) {
            let desc = core::str::from_utf8(&buf[..len]).unwrap_or("<invalid utf8>");
            info!("SPROUT:   describe: {}", desc);
        } else {
            info!("SPROUT:   describe: <failed>");
        }
    }

    fn spawn_apps(&mut self) {
        info!("SPROUT: spawn_apps start. tasks len={}", self.tasks.len());

        self.ensure_app("/sh");

        for task in self.tasks.iter_mut() {
            if let TaskKind::App = task.kind {
                if task.pid.is_some() {
                    continue;
                }
                // name is full path. spawn_process expects name to match module name?
                // spawn_process implementation in kernel matches `if m.name.contains(name)`.
                // So passing full path is fine.
                info!("SPROUT: Launching app '{}'", task.name);
                match stem::syscall::spawn_process(&task.name, 0) {
                    Ok(pid) => {
                        info!("SPROUT: App launched (PID={})", pid);
                        task.pid = Some(pid);

                        // If it's bloom, seed initial requests immediately after launch
                        if task.name.contains("bloom") {
                            seed_asset_requests();
                        }

                        // Set priority based on app name
                        let priority = if task.name.contains("scheduler_verify")
                            || task.name.contains("threads")
                        {
                            1 // Low - background tasks
                        } else if task.name.contains("bloom") || task.name.contains("bristle") {
                            3 // High - interactive UI only
                        } else {
                            2 // Normal - clock, flytrap, bindd, other apps
                        };

                        if let Err(e) = stem::thread::set_priority(pid, priority) {
                            info!(
                                "SPROUT: Failed to set priority for '{}': {:?}",
                                task.name, e
                            );
                        }
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

        let full = format!("/boot{}", name);
        info!("SPROUT: Adding fallback app '{}'", full);
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

        let full = format!("/boot{}", name);
        info!("SPROUT: Adding managed service '{}'", full);
        self.tasks.push(ManagedTask {
            name: full.clone(),
            kind: TaskKind::Service(service_kind.to_string()),
            module_path: full,
            pid: None,
            restarts: 0,
            spawn_arg: 0,
        });
    }

    /// Spawn any apps discovered during `discover()` that haven't been started by a pipeline.
    fn spawn_discovered_apps(&mut self) {
        for task in self.tasks.iter_mut() {
            if let TaskKind::App = task.kind {
                if task.pid.is_some() {
                    continue;
                }
                info!("SPROUT: Launching discovered app '{}'", task.name);
                match stem::syscall::spawn_process(&task.name, 0) {
                    Ok(pid) => {
                        info!("SPROUT: App launched (PID={})", pid);
                        task.pid = Some(pid);
                    }
                    Err(e) => info!("SPROUT: Failed to launch app '{}': {:?}", task.name, e),
                }
            }
        }
    }

    fn match_and_spawn_drivers(&mut self) {
        // Simple logic: Scan for RTC (hardcoded for now as per main.rs)
        // Ideally we traverse the graph for "REQUIRES_DRIVER" or similar.
        // But for v0, we just look for RTC.

        let mut buf = [ThingId::default(); 1];
        if let Ok(1) = thingsys::find(stem::abi::schema::kinds::DEV_RTC_CMOS, &mut buf) {
            let rtc_id = buf[0];
            if let Some(driver_name) = self.registry.find_driver("dev.rtc.Cmos") {
                info!("SPROUT: Found match for RTC: driver '{}'", driver_name);

                // Check if already running?
                // Add to managed tasks

                let ctx = stem::abi::driver_ctx::DriverCtx { device_id: rtc_id };
                let arg = ctx.to_raw();

                match stem::syscall::spawn_process(driver_name, arg) {
                    Ok(pid) => {
                        info!("SPROUT: Driver launched (PID={})", pid);
                        self.tasks.push(ManagedTask {
                            name: driver_name.to_string(),
                            kind: TaskKind::Driver("dev.rtc.Cmos".to_string()),
                            module_path: driver_name.to_string(), // approximation
                            pid: Some(pid),
                            restarts: 0,
                            spawn_arg: 0,
                        });

                        // Set driver priority to High (3)
                        if let Err(e) = stem::thread::set_priority(pid, 3) {
                            info!(
                                "SPROUT: Failed to set priority for driver '{}': {:?}",
                                driver_name, e
                            );
                        }
                    }
                    Err(e) => info!("SPROUT: Failed to launch driver: {:?}", e),
                }
            }
        }
    }

    fn monitor(&mut self) {
        for task in self.tasks.iter_mut() {
            if let Some(pid) = task.pid {
                // Poll status
                match stem::syscall::task_poll(pid) {
                    Ok((status, code)) => {
                        if status == stem::abi::types::TaskStatus::Dead {
                            info!(
                                "SPROUT: Task '{}' (PID {}) died with code {}. Restarting...",
                                task.name, pid, code
                            );

                            // Restart logic
                            task.pid = None; // Reset
                            task.restarts += 1;

                            let arg = task.spawn_arg;

                            // Brief backoff before restart
                            stem::sleep_ms(100 * (task.restarts as u64 + 1));

                            match stem::syscall::spawn_process(&task.name, arg) {
                                Ok(new_pid) => {
                                    info!("SPROUT: Restarted '{}' (PID={})", task.name, new_pid);
                                    task.pid = Some(new_pid);
                                }
                                Err(e) => {
                                    info!("SPROUT: Failed to restart '{}': {:?}", task.name, e)
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // ESRCH? maybe invalid pid?
                        // Assume dead.
                        info!("SPROUT: Task '{}' (PID {}) vanished?", task.name, pid);
                        task.pid = None;
                    }
                }
            }
        }
    }
}

fn seed_asset_requests() {
    info!("SPROUT: Seeding initial asset requests...");

    let requests = [
        ("NotoSans-Regular.ttf", "font"),
        ("NotoSansSymbol2-Regular.ttf", "font"),
        ("clouds.bmp", "image"),
        ("default.svg", "cursor"),
    ];

    for (name, kind) in requests {
        if let Ok(req_id) = thingsys::create_node(abi_kinds::ASSET_REQUEST) {
            if let Ok(name_sym) = thingsys::intern(name) {
                let _ = thingsys::prop_set(req_id, keys::ASSET_NAME, name_sym as u64);
            }
            if let Ok(kind_sym) = thingsys::intern(kind) {
                let _ = thingsys::prop_set(req_id, keys::ASSET_KIND, kind_sym as u64);
            }
            let _ = thingsys::prop_set(req_id, keys::ASSET_SOURCE, 0); // optional source hint
        }
    }
}

/// Spawn bloom compositor with display handles (minimal - no input events)
fn spawn_bloom(tasks: &mut Vec<ManagedTask>, dh: &crate::pipelines::DisplayHandles) {
    use abi::vm::{VmBacking, VmMapReq, VmProt};

    let boot_size = 4096;
    let boot_fd = thingsys::memfd_create("bloom.boot", boot_size).unwrap_or(0);

    if boot_fd != 0 {
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
            slice[1] = dh.drv_req_write as u32;
            slice[2] = dh.drv_resp_read as u32;
            slice[3] = 0; // No bristle event handle (input disabled)

            // Display bytespace id (128-bit)
            let bs_thing = ThingId::from_u64(dh.bs_id as u64);
            let bs_bytes = bs_thing.0;
            slice[4] = u32::from_le_bytes(bs_bytes[0..4].try_into().unwrap());
            slice[5] = u32::from_le_bytes(bs_bytes[4..8].try_into().unwrap());
            slice[6] = u32::from_le_bytes(bs_bytes[8..12].try_into().unwrap());
            slice[7] = u32::from_le_bytes(bs_bytes[12..16].try_into().unwrap());

            // stem::thing::sys::vm_unmap(&resp).ok();
        }
    }

    let bloom_arg = boot_fd as usize;
    info!(
        "SPROUT: Bloom handles via FD={} backend={}",
        boot_fd,
        dh.backend_name
    );

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
}
