use alloc::vec::Vec;
use alloc::string::{String, ToString};
use alloc::format;
use stem::{info, error, warn};
use stem::thing::ThingId;
use stem::thing::sys as thingsys;
use crate::registry::Registry;
use crate::devtree;

#[derive(Debug, PartialEq)]
enum TaskKind {
    Driver(String), // Device Kind
    App,
}

struct ManagedTask {
    name: String,
    kind: TaskKind,
    #[allow(dead_code)]
    module_path: String,
    pid: Option<u64>,
    restarts: u32,
}

pub struct Supervisor {
    tasks: Vec<ManagedTask>,
    registry: Registry,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            registry: Registry::new(),
        }
    }

    pub fn run_forever(&mut self) -> ! {
        info!("SPROUT: Supervisor starting...");

        // 1. Discovery
        self.discover();

        // 2. Spawn Apps
        self.spawn_apps();

        // 3. Match and Spawn Drivers
        self.match_and_spawn_drivers();

        // 4. Loop
        info!("SPROUT: Entering supervisor loop.");
        loop {
            self.monitor();
            stem::yield_now();
            stem::sleep_ms(100);
            
            // Heartbeat?
            // "If there are no runnable user tasks, Idle runs and emits an occasional heartbeat (throttled)."
            // Sprout is a user task. If Sprout is sleeping 100ms, then Idle runs (if nothing else).
            // So Sprout doesn't need to print heartbeat. Idle does.
            // Sprout is supervisor.
        }
    }

    fn discover(&mut self) {
        info!("SPROUT: Discovering modules...");
        let mut modules = [ThingId(0); 32];
        let count = thingsys::find(stem::abi::schema::kinds::BOOT_MODULE, &mut modules).unwrap_or(0);
        info!("SPROUT: Found {} modules", count);
        
        for i in 0..count {
            if i >= modules.len() {
                info!("SPROUT: Module index {} out of bounds!", i);
                break;
            }
            let mod_id = modules[i];
            let name = self.get_module_name(mod_id);
            if name.is_empty() { 
                info!("SPROUT: Module {} has empty name", mod_id.0);
                continue; 
            }

            // Heuristic Partitioning
            // "Drivers live at: /boot/modules/drivers/<name>"
            // "Apps live at: /boot/modules/apps/<name>"
            // But current setup might be flat "/boot/modules/clock".
            // So I will iterate known apps if strict path not found?
            // "Implement a simple convention for now"
            
            if name.contains("/drivers/") {
                // It's a driver. Register it.
                // We use Registry's logic to parse the manifest.
                // Modifying Registry to accept external ID is hard without refactor.
                // I will just let Registry scan ALL modules internally?
                // But Registry doesn't know about Apps.
                // Let's defer to Registry scan logic for drivers.
            } else if name.contains("/apps/") || name.ends_with("/clock") || name.ends_with("/threads_demo") || name.ends_with("/idle") {
                 // Treat as App
                 info!("SPROUT: Discovered app: {}", name);
                 self.tasks.push(ManagedTask {
                     name: name.clone(),
                     kind: TaskKind::App,
                     module_path: name,
                     pid: None,
                     restarts: 0,
                 });
            } else {
                 // Unknown or Driver in flat dir?
                 // Let's assume everything else is potential driver for Registry to check.
            }
        }
        
        // Let registry scan for drivers (it iterates all modules itself currently)
        self.registry.scan();
    }
    
    fn get_module_name(&self, mod_id: ThingId) -> String {
        let mut buf = [0u8; 1024];
        if let Ok(len) = thingsys::describe_thing(mod_id, &mut buf) {
             let s = core::str::from_utf8(&buf[..len]).unwrap_or("");
             if let Some(pos) = s.find("name=\"") {
                 let rest = &s[pos + 6..];
                 if let Some(end) = rest.find('"') {
                     return rest[..end].to_string();
                 }
             }
        }
        String::new()
    }

    fn spawn_apps(&mut self) {
        for task in self.tasks.iter_mut() {
            if let TaskKind::App = task.kind {
                // name is full path. spawn_process expects name to match module name?
                // spawn_process implementation in kernel matches `if m.name.contains(name)`.
                // So passing full path is fine.
                info!("SPROUT: Launching app '{}'", task.name);
                match stem::syscall::spawn_process(&task.name, 0) {
                    Ok(pid) => {
                        info!("SPROUT: App launched (PID={})", pid);
                        task.pid = Some(pid);
                    },
                    Err(e) => info!("SPROUT: Failed to launch app '{}': {:?}", task.name, e),
                }
            }
        }
    }

    fn match_and_spawn_drivers(&mut self) {
        // Simple logic: Scan for RTC (hardcoded for now as per main.rs)
        // Ideally we traverse the graph for "REQUIRES_DRIVER" or similar.
        // But for v0, we just look for RTC.
        
        let mut buf = [ThingId(0); 1];
        if let Ok(1) = thingsys::find(stem::abi::schema::kinds::DEV_RTC_CMOS, &mut buf) {
            let rtc_id = buf[0];
            if let Some(driver_name) = self.registry.find_driver("dev.rtc.Cmos") {
                 info!("SPROUT: Found match for RTC: driver '{}'", driver_name);
                 
                 // Check if already running?
                 // Add to managed tasks
                 
                 let ctx = stem::abi::driver_ctx::DriverCtx { device_id: stem::abi::types::ThingId(rtc_id.0) };
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
                         });
                     },
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
                            info!("SPROUT: Task '{}' (PID {}) died with code {}. Restarting...", task.name, pid, code);
                            
                            // Restart logic
                            task.pid = None; // Reset
                            task.restarts += 1;
                            
                            let arg = if let TaskKind::Driver(ref dk) = task.kind {
                                // Reconstruct arg for driver?
                                // Assuming RTC for now.
                                // We lost the original ID. 
                                // Ideally ManagedTask stores the argument too.
                                // Quick hack: Re-find RTC
                                if dk == "dev.rtc.Cmos" {
                                    let mut buf = [ThingId(0); 1];
                                    if let Ok(1) = thingsys::find(stem::abi::schema::kinds::DEV_RTC_CMOS, &mut buf) {
                                         let rtc_id = buf[0];
                                         let ctx = stem::abi::driver_ctx::DriverCtx { device_id: stem::abi::types::ThingId(rtc_id.0) };
                                         ctx.to_raw()
                                    } else { 0 }
                                } else { 0 }
                            } else {
                                0
                            };
                            
                            // Exponential Backoff? "10 ticks"
                            // For now just sleep before restart? Or just yield.
                            stem::sleep_ms(100 * (task.restarts as u64 + 1)); 

                            match stem::syscall::spawn_process(&task.name, arg) {
                                Ok(new_pid) => {
                                    info!("SPROUT: Restarted '{}' (PID={})", task.name, new_pid);
                                    task.pid = Some(new_pid);
                                },
                                Err(e) => info!("SPROUT: Failed to restart '{}': {:?}", task.name, e),
                            }
                        }
                    },
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
