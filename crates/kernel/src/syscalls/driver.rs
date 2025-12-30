use crate::bridge::HardwareBridge;
use crate::Kernel;
use abi::wire::driver::{DriverEvent, DriverPublish};
use abi::{SysRet, SYSCALL_DRIVER_PUBLISH, SYSCALL_DRIVER_WAIT};
use postcard::from_bytes;

pub fn sys_driver_wait<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    out_ptr: *mut u8,
    out_len: usize,
) -> SysRet {
    // V0: Non-blocking poll.
    // If we loop here, we hold the KERNEL lock (BKL) from syscall_hook,
    // preventing the scheduler from ticking (which also needs BKL).
    // So we must return to user mode if no event, letting user spin.

    if let Some(event) = crate::input::try_pop_event(&kernel.bridge) {
        // Serialize
        let slice = unsafe { core::slice::from_raw_parts_mut(out_ptr, out_len) };
        match postcard::to_slice(&event, slice) {
            Ok(used) => return used.len() as SysRet,
            Err(_) => return -1, // Enobufs
        }
    }

    // No event: Return error so user loops.
    // Use a distinguishable error? For V0, -1 is fine (User ignores Err).
    -1
}

pub fn sys_driver_publish<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    ptr: *const u8,
    len: usize,
) -> SysRet {
    // Read buffer
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    let publish: DriverPublish = match from_bytes(slice) {
        Ok(p) => p,
        Err(_) => return -2, // Ebadmsg
    };

    match publish {
        DriverPublish::Observation { thing_bytes } => {
            match postcard::from_bytes::<thing_models::Thing>(&thing_bytes) {
                Ok(thing) => {
                    kernel.bridge.log(alloc::format!("Kernel: SysDriverPublish Received Kind {}", thing.kind.0).as_str());
                    let id = kernel.graph.create_thing(thing.kind, thing.body);
                    id.0 as SysRet
                }
                Err(_) => -2,
            }
        }
    }
}

pub fn sys_mmio_map<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    phys_addr: u64,
    len: u64,
) -> SysRet {
    // Identity map MMIO into user space (Low memory)
    // TODO: Use VMA allocator to avoid conflicts.
    // MMIO is usually < 4GB. Code is at 256GB (0x40...).
    // So identity mapping is safe for now.
    
    let start_page = phys_addr & !0xFFF;
    let end_addr = phys_addr + len;
    let end_page_align = (end_addr + 0xFFF) & !0xFFF;
    
    let mut curr = start_page;
    while curr < end_page_align {
        match kernel.bridge.map_user_mmio(curr, curr, 0) { // Flags handled by bridge default (NO_CACHE etc)
            Ok(_) => {},
            Err(_) => return -1, // ENOMEM
        }
        curr += 4096;
    }
    
    phys_addr as SysRet // Return virt address (identity)
}

pub fn sys_irq_register<B: HardwareBridge>(
    _kernel: &mut Kernel<B>,
    _irq: usize,
) -> SysRet {
    // TODO: Register IRQ owner in interrupt router.
    // For now, standard PC interrupts are broadcast or we assume single driver.
    0
}

pub fn sys_port_io<B: HardwareBridge>(
    kernel: &mut Kernel<B>,
    port: u16,
    val: u32,
    width: u8,
    write: bool,
) -> SysRet {
    if write {
        match width {
            1 => kernel.bridge.port_outb(port, val as u8),
            2 => kernel.bridge.port_outw(port, val as u16),
            4 => kernel.bridge.port_outd(port, val),
            _ => return -1,
        }
        0
    } else {
        match width {
            1 => kernel.bridge.port_inb(port) as SysRet,
            2 => kernel.bridge.port_inw(port) as SysRet,
            4 => kernel.bridge.port_ind(port) as SysRet,
            _ => -1,
        }
    }
}
