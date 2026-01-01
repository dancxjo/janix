use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
// use crate::log;
use crate::trap::{self, TrapRecord, FaultKind, Arch};
// use graph::store; // For PlaceStore

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[allow(static_mut_refs)]
pub unsafe fn init() {
    IDT.breakpoint.set_handler_fn(breakpoint_handler);
    IDT.double_fault.set_handler_fn(double_fault_handler)
        .set_stack_index(super::gdt::DOUBLE_FAULT_IST_INDEX);
    IDT.general_protection_fault.set_handler_fn(gp_handler);
    IDT.page_fault.set_handler_fn(page_fault_handler);

    // Timer (Vector 32)
    unsafe {
        IDT[32].set_handler_addr(x86_64::VirtAddr::new(timer_interrupt_trampoline as u64));
    }
    
    IDT.load();
}

extern "C" {
    fn timer_interrupt_trampoline();
}

unsafe fn record_x86_fault(
    kind: FaultKind,
    stack_frame: &InterruptStackFrame, 
    error_code: u64, 
    addr: Option<u64>,
    vector: u32
) {
    // We need to lock the store to record.
    // WARNING: If we fault while holding the lock, we double-fault/deadlock.
    // This is a known hazard.
    
    // In strict v0.3, we assume basic graph health.
    // We use the public `graph::store::PLACE_STORE` lock if we could, 
    // but better to use `crate::trap::record_fault` which we defined to take `&mut PlaceStore`.
    
    // We need access to the global PlaceStore.
    // Since we made methods public on PlaceStore, but we don't have a `get_global()`
    // we normally access it via `graph::store::PLACE_STORE.lock()`.
    // But `PLACE_STORE` is private in `graph::store`.
    // This is where I might have updated `graph::store` insufficiently?
    // `pub struct PlaceStore` is public.
    // But the *instance* is private.
    // I should have made `PLACE_STORE` public or added a getter.
    // I will use `graph::store::thing_create` etc which manage locks internally,
    // OR I need to update `graph/src/store.rs` to expose the lock/instance.
    
    // Wait, `trap::record_fault` takes `&mut PlaceStore`.
    // If I can't get a `&mut PlaceStore`, I can't call it.
    // I should update `graph/src/store.rs` to expose `pub fn with_store<F>(f: F)` or make `PLACE_STORE` public.
    // I'll assume I can fix that in the next step.
    // For now, let's pretend I can access it via a helper I'll add.
    
    // Let's add `graph::store::with_store` pattern.
    
    graph::store::with_store(|store| {
        let rec = TrapRecord {
            arch: Arch::X86_64,
            kind,
            ip: stack_frame.instruction_pointer.as_u64(),
            sp: stack_frame.stack_pointer.as_u64(),
            addr,
            code: error_code,
            vector,
            cpu: 0, // TODO: percpu
            in_kernel: stack_frame.code_segment == 0x08, // Kernel CS
            task: None, // TODO: current task
        };
        
        trap::record_fault(store, &rec);
    });
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    // Breakpoint is benign. Record and return.
    unsafe { 
        record_x86_fault(FaultKind::Breakpoint, &stack_frame, 0, None, 3);
    }
    // Don't panic.
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
    // We can try to record, but DF implies stack issues often.
    unsafe {
        record_x86_fault(FaultKind::DoubleFault, &stack_frame, _error_code, None, 8);
    }
    panic!("DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn gp_handler(
    stack_frame: InterruptStackFrame, error_code: u64)
{
    unsafe {
        record_x86_fault(FaultKind::GeneralProtection, &stack_frame, error_code, None, 13);
    }
    panic!("GENERAL PROTECTION FAULT: error_code={}\n{:#?}", error_code, stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode)
{
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read().as_u64();
    unsafe {
        record_x86_fault(FaultKind::PageFault, &stack_frame, error_code.bits(), Some(addr), 14);
    }
    panic!("PAGE FAULT: accessed {:x}\nerror code: {:?}\n{:#?}", addr, error_code, stack_frame);
}
