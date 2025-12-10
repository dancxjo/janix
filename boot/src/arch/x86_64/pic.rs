use spin::Mutex;
use x86_64::instructions::port::Port;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;
const PIC_EOI: u8 = 0x20;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

struct Pics;

static PIC_LOCK: Mutex<()> = Mutex::new(());

pub fn init() {
    let _lock = PIC_LOCK.lock();
    unsafe {
        let mut pic1_cmd = Port::<u8>::new(PIC1_COMMAND);
        let mut pic1_data = Port::<u8>::new(PIC1_DATA);
        let mut pic2_cmd = Port::<u8>::new(PIC2_COMMAND);
        let mut pic2_data = Port::<u8>::new(PIC2_DATA);

        pic1_cmd.write(ICW1_INIT | ICW1_ICW4);
        pic2_cmd.write(ICW1_INIT | ICW1_ICW4);
        pic1_data.write(PIC_1_OFFSET);
        pic2_data.write(PIC_2_OFFSET);
        pic1_data.write(4);
        pic2_data.write(2);
        pic1_data.write(ICW4_8086);
        pic2_data.write(ICW4_8086);

        // Unmask only IRQ1 (keyboard) on master PIC; mask all on slave.
        pic1_data.write(0b1111_1101);
        pic2_data.write(0xFF);
    }
}

pub fn notify_end_of_interrupt(irq: u8) {
    let _lock = PIC_LOCK.lock();
    unsafe {
        if irq >= 8 {
            let mut pic2_cmd = Port::<u8>::new(PIC2_COMMAND);
            pic2_cmd.write(PIC_EOI);
        }
        let mut pic1_cmd = Port::<u8>::new(PIC1_COMMAND);
        pic1_cmd.write(PIC_EOI);
    }
}
