#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    RtcCmos = 1,
    Keyboard = 2,
    Mouse = 3,
    Framebuffer = 4,
    Pci = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceHandle(pub u32);

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RootCaps;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DeviceCall {
    pub kind: DeviceKind,
    pub op: u32,
    pub in_ptr: u64,
    pub in_len: u32,
    pub out_ptr: u64,
    pub out_len: u32,
}

// RTC OPs
pub const RTC_OP_READ_TIME: u32 = 1;

// PCI DeviceCall OPs
pub const PCI_OP_ENABLE_MSI: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PciEnableMsiRequest {
    pub claim_handle: u32,
    pub requested_vectors: u16,
    pub prefer_msix: u8,
    pub _reserved: u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PciEnableMsiResponse {
    pub vector: u8,
    pub irq_mode: u8, // See PCI_IRQ_MODE_* constants
    pub _reserved: [u8; 2],
}

pub const PCI_IRQ_MODE_MSI: u8 = 1;
pub const PCI_IRQ_MODE_MSIX: u8 = 2;

pub const DEVICE_IRQ_SUBSCRIBE_VECTOR: u8 = 0;
pub const DEVICE_IRQ_SUBSCRIBE_DEVICE: u8 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RtcTime {
    pub year: u16,   // e.g. 2026
    pub month: u8,   // 1-12
    pub day: u8,     // 1-31
    pub hour: u8,    // 0-23
    pub minute: u8,  // 0-59
    pub second: u8,  // 0-59
    pub weekday: u8, // 0-6
    pub flags: u8,   // Status flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_call_layout() {
        assert_eq!(
            core::mem::size_of::<DeviceCall>(),
            40,
            "DeviceCall size must be exactly 40 bytes to match ABI"
        );
        assert_eq!(
            core::mem::align_of::<DeviceCall>(),
            8,
            "DeviceCall alignment must be exactly 8 bytes"
        );
    }

    #[test]
    fn test_pci_enable_msi_request_layout() {
        assert_eq!(
            core::mem::size_of::<PciEnableMsiRequest>(),
            8,
            "PciEnableMsiRequest size must be exactly 8 bytes"
        );
        assert_eq!(
            core::mem::align_of::<PciEnableMsiRequest>(),
            4,
            "PciEnableMsiRequest alignment must be exactly 4 bytes"
        );
    }

    #[test]
    fn test_pci_enable_msi_response_layout() {
        assert_eq!(
            core::mem::size_of::<PciEnableMsiResponse>(),
            4,
            "PciEnableMsiResponse size must be exactly 4 bytes"
        );
        assert_eq!(
            core::mem::align_of::<PciEnableMsiResponse>(),
            1,
            "PciEnableMsiResponse alignment must be exactly 1 byte"
        );
    }

    #[test]
    fn test_rtc_time_layout() {
        assert_eq!(
            core::mem::size_of::<RtcTime>(),
            10,
            "RtcTime size must be exactly 10 bytes"
        );
        assert_eq!(
            core::mem::align_of::<RtcTime>(),
            2,
            "RtcTime alignment must be exactly 2 bytes"
        );
    }
}
