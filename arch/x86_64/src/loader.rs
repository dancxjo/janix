use abi::ThingId;

use crate::bridge::Bridge;
use kernel::bridge::CpuBridge;
use kernel::sched::spawn_elf::{spawn_user_elf, FramebufferSpec, SpawnSpec, USER_FB_BASE};
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

    let fb = unsafe { crate::entry::FRAMEBUFFER_INFO }
        .map(|(phys, size)| FramebufferSpec {
            phys,
            size: size as u64,
            user_virt: USER_FB_BASE,
        });

    let spec = SpawnSpec {
        name,
        elf: data,
        spawn: true,
        fb,
    };

    if let Some((entry, layout)) = spawn_user_elf(k, spec) {
        k.bridge.log(&alloc::format!(
            "loader: spawn_user_elf entry={:#x} stack_top={:#x} heap=[{:#x},{:#x})\n",
            entry,
            layout.stack_top,
            layout.heap_start,
            layout.heap_end
        ));
    } else {
        k.bridge.log("loader: spawn_user_elf failed\n");
    }
}
