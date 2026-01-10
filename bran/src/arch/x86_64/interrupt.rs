use super::idt::{Idt, lidt};
use super::gdt::KERNEL_CODE_SEL;
use super::timer;


static mut IDT: Idt = Idt::new();

unsafe extern "C" {
    fn timer_trampoline();
    fn yield_handler();
    fn exception_0();
    fn exception_3();
    fn exception_6();
    fn exception_8();
    fn exception_13();
    fn exception_14();
}

pub unsafe fn init() {
    unsafe {
        kernel::kinfo!("idt: initializing...");
        let idt = &mut *core::ptr::addr_of_mut!(IDT);
        
        // Timer
        idt.entries[32].set_handler(timer_trampoline as u64, KERNEL_CODE_SEL, 0x8E00); // Int Gate, DPL=0

        // Exceptions
        idt.entries[0].set_handler(exception_0 as u64, KERNEL_CODE_SEL, 0x8E00);
        idt.entries[3].set_handler(exception_3 as u64, KERNEL_CODE_SEL, 0x8E00);
        idt.entries[6].set_handler(exception_6 as u64, KERNEL_CODE_SEL, 0x8E00);
        
        // Double Fault (IST 1)
        // 0x8E00 | IST=1 -> 0x8E01
        idt.entries[8].set_handler(exception_8 as u64, KERNEL_CODE_SEL, 0x8E01);
        
        idt.entries[13].set_handler(exception_13 as u64, KERNEL_CODE_SEL, 0x8E00);
        idt.entries[14].set_handler(exception_14 as u64, KERNEL_CODE_SEL, 0x8E00);

        // Yield (0x81)
        idt.entries[0x81].set_handler(yield_handler as u64, KERNEL_CODE_SEL, 0x8E00);

        lidt(idt);
        kernel::kinfo!("idt: loaded");
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn timer_handler() {
    unsafe {
        // Ack LAPIC
        timer::ack();

        // Trigger rescheduling request
        kernel::task::set_need_resched(true);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn check_preemption_shim() {
    kernel::task::check_preemption();
}
