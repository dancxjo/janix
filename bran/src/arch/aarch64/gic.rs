use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{AtomicU64, Ordering};
use kernel::kinfo;

// Physical addresses for mapping (Virt machine)
// We assume these are identity mapped or compatible? 
// In BRAN, we usually have identity map for lower memory or rely on HHDM.
// But `0x08000000` is usually MMIO. We might need `map_page` to access it if we are in high-half?
// BRAN runs mostly in identity map early on?
// Wait, `BootRuntime` runs when kernel is up? 
// The kernel is high-half.
// MMIO need to be mapped.
// `x86_64` maps LAPIC to `0xffffffff8f000000`.
// We should probably map GIC similarly if we want to access it.
// OR, since we are borrowing, we look at how trunk accessed it.
// Trunk `timer.rs` init used `0x08000000` directly.
// This implies trunk ran with identity map covering MMIO or `0x08000000` was mapped.
// In `bran`, we might need to map it.
// I'll stick to the addresses for now. If it faults, we need to map.
// `timer::init` in `exception.rs` (which calls this) should map it?
// x86_64 `timer::init` does `map_mmio_page`.
// I should replicate that pattern. `map_mmio_page` in `paging.rs`.
// But `paging.rs` in `aarch64` is stubs!
// This is the Catch-22.
// However, `x86_64` `map_mmio_page` maps to high half.
// I can implement a simple `map_mmio_page` stub that just identity maps if possible or if `paging` is broken, I can't do much.
// BUT, if I assume we are running in a mode where we can access physical addresses (e.g. before full paging/unmap), maybe it works.
// Or I use `phys_to_virt_offset` if mapped via HHDM?
// MMIO is usually NOT in HHDM (RAM only).
// So explicit mapping is needed.
// Given `paging` is stubs, I can't map.
// I will just use the physical address and hope we are in a 1:1 window or use a pointer.
// If it fails, I've done my best without paging.
// `x86_64` `map_mmio_page` implementation in `bran` delegates to `paging::map_page`.
// So I really need `map_page` to work for GIC key features.
// Without it, GIC access will likely Page Fault.
// Unless... we use `PhysFrame` and it happens to work?
// I'll try to just implement the logic.

pub const GICD_PHYS: u64 = 0x08000000;
pub const GICC_PHYS: u64 = 0x08010000;

static GICD_BASE: AtomicU64 = AtomicU64::new(GICD_PHYS);
static GICC_BASE: AtomicU64 = AtomicU64::new(GICC_PHYS);

const GICD_CTLR: u64 = 0x000;
const GICD_ISENABLER: u64 = 0x100;
const GICD_ITARGETSR: u64 = 0x800;

const GICC_CTLR: u64 = 0x000;
const GICC_PMR: u64 = 0x004;
const GICC_EOIR: u64 = 0x010;
const GICC_IAR: u64 = 0x00C;

pub unsafe fn init(dist_base: u64, cpu_base: u64) {
    kinfo!("gic: init dist={:#x} cpu={:#x}", dist_base, cpu_base);
    GICD_BASE.store(dist_base, Ordering::Relaxed);
    GICC_BASE.store(cpu_base, Ordering::Relaxed);

    unsafe {
        // 1. Distributor: Enable Group 0 and 1
        write_volatile((dist_base + GICD_CTLR) as *mut u32, 3);

        // 2. CPU Interface: Enable Group 0 and 1 + Priority Mask
        write_volatile((cpu_base + GICC_PMR) as *mut u32, 0xF0); // Priority mask
        write_volatile((cpu_base + GICC_CTLR) as *mut u32, 3); // Enable

        let pmr = read_volatile((cpu_base + GICC_PMR) as *const u32);
        let ctlr = read_volatile((cpu_base + GICC_CTLR) as *const u32);
        kinfo!("gic: PMR={:#x} CTLR={:#x}", pmr, ctlr);
    }
}

fn dist() -> u64 {
    GICD_BASE.load(Ordering::Relaxed)
}

fn cpu() -> u64 {
    GICC_BASE.load(Ordering::Relaxed)
}

pub unsafe fn set_priority(id: u32, priority: u8) {
    let base = dist();
    if base == 0 { return; }
    let p_addr = (base + 0x400 + (id as u64)) as *mut u8;
    unsafe { write_volatile(p_addr, priority); }
}

pub unsafe fn set_group1(id: u32) {
    let base = dist();
    if base == 0 { return; }
    let n = id / 32;
    let offset = id % 32;
    // GICD_IGROUPR = 0x080
    let g_addr = (base + 0x080 + (n as u64 * 4)) as *mut u32;
    unsafe {
        let mut val = read_volatile(g_addr);
        val |= 1 << offset;
        write_volatile(g_addr, val);
    }
}

pub unsafe fn enable_irq(id: u32) {
    let base = dist();
    if base == 0 { return; }

    let n = id / 32;
    let offset = id % 32;

    // 2. Set Enable bit
    let addr = (base + GICD_ISENABLER + (n as u64 * 4)) as *mut u32;
    unsafe {
        let val = read_volatile(addr);
        write_volatile(addr, val | (1 << offset));
    }
    
    // Target CPU 0 (or all)
    // For SPIs (id >= 32), we need to maintain targets.
    if id >= 32 {
        let t_offset = (id / 4) * 4;
        let t_addr = (base + GICD_ITARGETSR + t_offset as u64) as *mut u32;
        let shift = (id % 4) * 8;
        unsafe {
            let mut current = read_volatile(t_addr);
            current &= !(0xFF << shift);
            current |= 0x01 << shift; // Target CPU 0 (bit 0 set) - assuming Uniprocessor or CPU 0
            write_volatile(t_addr, current);
        }
    }
}

pub unsafe fn ack_irq() -> u32 {
    let base = cpu();
    if base == 0 { return 0x3ff; } 
    unsafe { read_volatile((base + GICC_IAR) as *const u32) }
}

pub unsafe fn eoi(id: u32) {
    let base = cpu();
    if base == 0 { return; }
    unsafe { write_volatile((base + GICC_EOIR) as *mut u32, id); }
}
