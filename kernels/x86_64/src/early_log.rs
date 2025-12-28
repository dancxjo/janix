use bridge_x86_64::Bridge;
use kernel_core::bridge::HardwareBridge;

#[derive(Clone, Copy)]
pub struct HeapInitInfo {
    pub phys_start: u64,
    pub virt_start: u64,
    pub size: u64,
}

pub fn log_heap_init(info: HeapInitInfo) {
    let bridge = Bridge;
    bridge.log("Heap Init:\n");
    bridge.log("  Phys: ");
    print_hex(&bridge, info.phys_start);
    bridge.log("\n  Virt: ");
    print_hex(&bridge, info.virt_start);
    bridge.log("\n  Size: ");
    print_hex(&bridge, info.size);
    bridge.log("\n");
}

fn print_hex(bridge: &Bridge, val: u64) {
    bridge.log("0x");
    let mut printed = false;
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        if digit != 0 || printed || i == 0 {
            let c = if digit < 10 {
                b'0' + digit as u8
            } else {
                b'a' + (digit - 10) as u8
            };
            let s = unsafe { core::str::from_utf8_unchecked(core::slice::from_ref(&c)) };
            bridge.log(s);
            printed = true;
        }
    }
}
