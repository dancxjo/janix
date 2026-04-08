use abi::types::TaskStatus;
use alloc::string::String;
use stem::syscall::{spawn_process, task_poll, vfs_umount};
use stem::time::monotonic_ns;
use stem::{info, warn};

use crate::binding::Binding;
use crate::sysfs::{device_present, SysDevice};

const INITIAL_BACKOFF_MS: u64 = 100;
const MAX_BACKOFF_MS: u64 = 5_000;

pub struct ManagedDriver {
    pub slot: String,
    pub graph_id: u64,
    pub driver: &'static str,
    pub mount_path: Option<String>,
    pub pid: Option<u64>,
    restarts: u32,
    restart_after_ns: u64,
}

impl ManagedDriver {
    pub fn new(device: &SysDevice, binding: Binding, mount_path: Option<String>) -> Self {
        Self {
            slot: device.slot.clone(),
            graph_id: device.graph_id,
            driver: binding.driver,
            mount_path,
            pid: None,
            restarts: 0,
            restart_after_ns: 0,
        }
    }

    pub fn ensure_running(&mut self) {
        if self.pid.is_some() {
            return;
        }
        if !device_present(&self.slot) {
            return;
        }
        if monotonic_ns() < self.restart_after_ns {
            return;
        }

        match spawn_process(self.driver, self.graph_id as usize) {
            Ok(pid) => {
                info!(
                    "DEVD: launched driver {} for {} (graph_id={}, pid={})",
                    self.driver, self.slot, self.graph_id, pid
                );
                self.pid = Some(pid);
            }
            Err(err) => {
                warn!(
                    "DEVD: failed to launch {} for {}: {:?}",
                    self.driver, self.slot, err
                );
                self.schedule_restart();
            }
        }
    }

    pub fn monitor(&mut self) {
        let Some(pid) = self.pid else {
            self.ensure_running();
            return;
        };

        match task_poll(pid) {
            Ok((TaskStatus::Dead, code)) => {
                warn!(
                    "DEVD: driver {} for {} exited with code {}",
                    self.driver, self.slot, code
                );
                self.pid = None;
                self.cleanup_mount();
                if device_present(&self.slot) {
                    self.schedule_restart();
                    self.ensure_running();
                }
            }
            Ok(_) => {}
            Err(err) => {
                warn!(
                    "DEVD: lost pid {} for {} ({}): {:?}",
                    pid, self.slot, self.driver, err
                );
                self.pid = None;
                self.cleanup_mount();
                if device_present(&self.slot) {
                    self.schedule_restart();
                    self.ensure_running();
                }
            }
        }
    }

    pub fn mark_removed(&mut self) {
        self.pid = None;
        self.restart_after_ns = 0;
        self.cleanup_mount();
    }

    fn schedule_restart(&mut self) {
        self.restarts = self.restarts.saturating_add(1);
        let shift = self.restarts.saturating_sub(1).min(5);
        let delay_ms = (INITIAL_BACKOFF_MS << shift).min(MAX_BACKOFF_MS);
        self.restart_after_ns = monotonic_ns() + delay_ms * 1_000_000;
    }

    fn cleanup_mount(&self) {
        if let Some(path) = self.mount_path.as_deref() {
            let _ = vfs_umount(path);
        }
    }
}
