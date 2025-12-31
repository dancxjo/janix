use abi::ThingId;
use crate::bridge::Bridge;
use crate::user_space::Aarch64UserSpace;
use kernel::arch::user_space::UserSpace;
use kernel::sched::spawn::{spawn_user_elf, FramebufferMap};
use kernel::Kernel;
use kernel::bridge::CpuBridge;

pub fn spawn_elf(
    k: &mut Kernel<Bridge>,
    _parent_dir_id: Option<ThingId>,
    name: &str,
    data: &[u8],
    _idx: usize,
    spawn_override: Option<bool>,
    fb_phys: u64,
    fb_size: usize,
) {
    let should_spawn = spawn_override.unwrap_or(true);
    if !should_spawn {
        return;
    }

    let fb = if fb_size > 0 {
        Some(FramebufferMap {
            phys: fb_phys,
            size: fb_size as u64,
        })
    } else {
        None
    };

    let mut us = Aarch64UserSpace;
    if let Some(_res) = spawn_user_elf(k, &mut us, name, data, fb) {
       // Logging handled in spawn_user_elf
    } else {
        k.bridge.log("loader: spawn_user_elf failed\n");
    }
}
