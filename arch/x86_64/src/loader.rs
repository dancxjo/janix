use abi::ThingId;

use crate::bridge::Bridge;
use crate::user_space::X64UserSpace;
use kernel::arch::user_space::UserSpace;
use kernel::bridge::CpuBridge;
use kernel::sched::spawn::{spawn_user_elf, FramebufferMap};
use kernel::Kernel;

pub fn spawn_elf(
    k: &mut Kernel<Bridge>,
    _parent_dir_id: Option<ThingId>,
    name: &str,
    data: &[u8],
    _idx: usize,
    spawn_override: Option<bool>,
) {
    let should_spawn = spawn_override.unwrap_or(true);
    if !should_spawn {
        return;
    }

    let fb = unsafe { crate::entry::FRAMEBUFFER_INFO }.map(|(phys, size)| FramebufferMap {
        phys,
        size: size as u64,
    });

    let mut us = X64UserSpace;
    if let Some(_res) = spawn_user_elf(k, &mut us, name, data, fb) {
        // Logging is already handled in spawn_user_elf
    } else {
        k.bridge.log("loader: spawn_user_elf failed\n");
    }
}
