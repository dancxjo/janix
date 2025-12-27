// Page Table Entry Flags (AArch64 VMSA) - Re-exported for convenience
pub const PTE_VALID: u64 = 1 << 0;
pub const PTE_TABLE: u64 = 1 << 1; 
pub const PTE_PAGE: u64 = 1 << 1; 
pub const PTE_ATTR_NORMAL: u64 = 0 << 2;
pub const PTE_ATTR_DEVICE: u64 = 1 << 2;
pub const PTE_NS: u64 = 1 << 5; 
pub const PTE_AP_RW_EL1: u64 = 0 << 6;
pub const PTE_AP_RW_EL0: u64 = 1 << 6; 
pub const PTE_SH_INNER: u64 = 3 << 8; 
pub const PTE_AF: u64 = 1 << 10; 
pub const PTE_PXN: u64 = 1 << 53; 
pub const PTE_UXN: u64 = 1 << 54; 

pub const DESC_AP_EL0: u64 = PTE_AP_RW_EL0;
pub const DESC_UXN: u64 = PTE_UXN;
pub const ATTR_NORMAL: u64 = PTE_ATTR_NORMAL;

pub static mut UPDATE_FLAGS_FN: Option<fn(u64, u64, u64)> = None;

pub fn update_page_flags(virt: u64, set: u64, clear: u64) {
    unsafe {
        if let Some(f) = UPDATE_FLAGS_FN {
            f(virt, set, clear);
        }
    }
}

// Stub map_device_region if needed by someone else in bridge? 
// No one else calls it in bridge.

