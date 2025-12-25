use crate::limine::heap_init::HeapInitInfo;
use bridge_aarch64::Bridge;
use hw::HardwareBridge;

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
    for i in (0..16).rev() {
        let digit = (val >> (i * 4)) & 0xF;
        let c = if digit < 10 {
            b'0' + digit as u8
        } else {
            b'a' + (digit - 10) as u8
        };
        let s = unsafe { core::str::from_utf8_unchecked(core::slice::from_ref(&c)) };
        bridge.log(s);
    }
}
