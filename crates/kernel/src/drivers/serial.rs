use crate::bridge::HardwareBridge;
use abi::wire::typed::{CodecId, TypeId, TypedBytes};
use abi::{SymbolId, ThingId};
use thing_models::builtins::ids::{
    THING_BOOT_ROOT, THING_HAS_DEVICE_KIND, THING_LINK_KIND, THING_SERIAL_PORT_KIND,
};
use thing_models::core::serial::SerialPortBody;
use thing_models::value::ThingBody;
use thing_models::Thing;

pub const SERIAL_IO_PORT: u16 = 0x3F8;

#[cfg(target_arch = "x86_64")]
pub fn init(bridge: &impl HardwareBridge) {
    bridge.log("SERIAL: Initializing COM1...\n");
    // Standard COM1 initialization
    // Interrupt Enable (Base + 1)
    bridge.port_outb(SERIAL_IO_PORT + 1, 0x00); // Disable interrupts

    // Line Control (Base + 3)
    bridge.port_outb(SERIAL_IO_PORT + 3, 0x80); // Enable DLAB (set baud rate divisor)

    // Set 38400 baud (divisor 3)
    // Divisor Latch Low (Base + 0)
    bridge.port_outb(SERIAL_IO_PORT, 0x03);
    // Divisor Latch High (Base + 1)
    bridge.port_outb(SERIAL_IO_PORT + 1, 0x00);

    // Line Control (Base + 3)
    bridge.port_outb(SERIAL_IO_PORT + 3, 0x03); // 8 bits, no parity, one stop bit hiding DLAB

    // FIFO Control (Base + 2)
    bridge.port_outb(SERIAL_IO_PORT + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold

    // Modem Control (Base + 4)
    bridge.port_outb(SERIAL_IO_PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set

    // Interrupt Enable again (Base + 1)
    bridge.port_outb(SERIAL_IO_PORT + 1, 0x00); // Disable interrupts for now (we poll for logs)

    bridge.log("SERIAL: COM1 Ready\n");
}

#[cfg(target_arch = "aarch64")]
const PL011_BASE: u64 = 0x0900_0000;

#[cfg(target_arch = "aarch64")]
pub fn init(bridge: &impl HardwareBridge) {
    bridge.log("SERIAL: Initializing PL011...\n");

    let base = (PL011_BASE + bridge.hhdm_offset()) as *mut u32;

    unsafe {
        // Disable UART
        core::ptr::write_volatile(base.add(0x30 / 4), 0);
        // Clear interrupts
        core::ptr::write_volatile(base.add(0x44 / 4), 0x7FF);
        // Set baud to ~115200 for 24MHz clock: IBRD=13, FBRD=2
        core::ptr::write_volatile(base.add(0x24 / 4), 13);
        core::ptr::write_volatile(base.add(0x28 / 4), 2);
        // 8 bits, FIFO enable, no parity
        core::ptr::write_volatile(base.add(0x2C / 4), 0x70);
        // Enable UART, TX, RX
        core::ptr::write_volatile(base.add(0x30 / 4), 0x301);
    }

    bridge.log("SERIAL: PL011 Ready\n");
}

pub fn publish_serial_thing<B: HardwareBridge>(k: &mut crate::Kernel<B>) {
    let dev_body = SerialPortBody {
        port_base: SERIAL_IO_PORT,
        irq: 4,
    };

    let body_bytes = postcard::to_allocvec(&dev_body).unwrap();
    let tb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_SERIAL_PORT_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: body_bytes,
    })
    .unwrap();

    let dev_id = k.graph.create_thing(THING_SERIAL_PORT_KIND, tb);

    // Link Root -> Serial (HAS_CONSOLE)
    let link = thing_models::link::LinkBody {
        from: THING_BOOT_ROOT,
        to: dev_id,
        predicate: THING_HAS_DEVICE_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
        type_id: TypeId(THING_LINK_KIND.0 as u128),
        codec_id: CodecId::POSTCARD,
        bytes: postcard::to_allocvec(&link).unwrap(),
    })
    .unwrap();
    k.graph.create_thing(THING_LINK_KIND, lb);
}
