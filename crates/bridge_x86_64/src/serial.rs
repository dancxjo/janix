use x86_64::instructions::port::Port;
use abi::{ThingId, SymbolId};
use models::core::serial::SerialPortBody;
use models::Thing;
use models::value::ThingBody;
use models::builtins::ids::{THING_SERIAL_PORT_KIND, THING_HAS_CONSOLE_KIND, THING_BOOT_ROOT, THING_LINK_KIND};
use abi::wire::typed::{TypedBytes, TypeId, CodecId};

pub const SERIAL_IO_PORT: u16 = 0x3F8;

pub unsafe fn init() {
    let bridge = crate::Bridge;
    use hw::HardwareBridge;

    bridge.log("SERIAL: Initializing COM1...\n");
    // Standard COM1 initialization
    let mut port_int = Port::<u8>::new(SERIAL_IO_PORT + 1); // Interrupt Enable
    let mut port_fifo = Port::<u8>::new(SERIAL_IO_PORT + 2); // FIFO Control
    let mut port_lcr = Port::<u8>::new(SERIAL_IO_PORT + 3); // Line Control
    let mut port_mcr = Port::<u8>::new(SERIAL_IO_PORT + 4); // Modem Control
    
    port_int.write(0x00);    // Disable interrupts
    port_lcr.write(0x80);    // Enable DLAB (set baud rate divisor)
    
    // Set 38400 baud (divisor 3)
    let mut port_dll = Port::<u8>::new(SERIAL_IO_PORT); // Divisor Latch Low
    let mut port_dlh = Port::<u8>::new(SERIAL_IO_PORT + 1); // Divisor Latch High
    port_dll.write(0x03);
    port_dlh.write(0x00);
    
    port_lcr.write(0x03);    // 8 bits, no parity, one stop bit
    port_fifo.write(0xC7);   // Enable FIFO, clear them, with 14-byte threshold
     port_mcr.write(0x0B);    // IRQs enabled, RTS/DSR set
    port_int.write(0x00); // Disable interrupts for now (we poll for logs)
    
    bridge.log("SERIAL: COM1 Ready\n");
}

pub fn publish_serial_thing(k: &mut kernel_core::Kernel<crate::Bridge>) {
    let dev_body = SerialPortBody {
        port_base: SERIAL_IO_PORT,
        irq: 4,
    };
    
    let body_bytes = postcard::to_allocvec(&dev_body).unwrap();
    let tb = ThingBody::from(&TypedBytes {
         type_id: TypeId(THING_SERIAL_PORT_KIND.0 as u128),
         codec_id: CodecId::POSTCARD,
         bytes: body_bytes,
    }).unwrap();
    
    let dev_id = k.graph.create_thing(THING_SERIAL_PORT_KIND, tb);
    
    // Link Root -> Serial (HAS_CONSOLE)
    let link = models::link::LinkBody {
         from: THING_BOOT_ROOT,
         to: dev_id,
         predicate: THING_HAS_CONSOLE_KIND,
    };
    let lb = ThingBody::from(&TypedBytes {
         type_id: TypeId(THING_LINK_KIND.0 as u128),
         codec_id: CodecId::POSTCARD,
         bytes: postcard::to_allocvec(&link).unwrap() 
    }).unwrap();
    k.graph.create_thing(THING_LINK_KIND, lb);
}
