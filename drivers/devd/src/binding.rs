use alloc::string::String;

use crate::sysfs::SysDevice;

#[derive(Clone, Copy)]
pub struct Binding {
    pub vendor_id: u16,
    pub device_id: u16,
    pub driver: &'static str,
}

const BUILTIN_BINDINGS: &[Binding] = &[
    Binding {
        vendor_id: 0x1af4,
        device_id: 0x1000,
        driver: "/virtio_netd",
    },
    Binding {
        vendor_id: 0x1af4,
        device_id: 0x1050,
        driver: "/virtio_gpu",
    },
];

pub fn match_binding(device: &SysDevice) -> Option<Binding> {
    BUILTIN_BINDINGS.iter().copied().find(|binding| {
        binding.vendor_id == device.vendor_id && binding.device_id == device.device_id
    })
}

pub fn mount_hint(binding: Binding, device: &SysDevice) -> Option<String> {
    if binding.driver == "/virtio_netd" && device.class_code == 0x02 {
        return Some(String::from("/dev/net/virtio0"));
    }
    None
}
