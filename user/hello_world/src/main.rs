#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // 1. Fill registers with known pattern
    // RBX = 0xBBBB...
    // R12 = 0x1212...
    // R13 = 0x1313...
    // R14 = 0x1414...
    // R15 = 0x1515...
    // RBP = 0xBABA...
    
    // We will verify they are preserved after syscall.
    
    unsafe {
        let mut rbx_val: u64 = 0xBBBB_BBBB_BBBB_BBBB;
        let mut r12_val: u64 = 0x1212_1212_1212_1212;
        let mut r13_val: u64 = 0x1313_1313_1313_1313;
        let mut r14_val: u64 = 0x1414_1414_1414_1414;
        let mut r15_val: u64 = 0x1515_1515_1515_1515;
        let mut rbp_val: u64 = 0xBABA_BABA_BABA_BABA;

        // Perform Syscall 1 (LOG)
        // Args: RDI, RSI.
        // We use inline asm to set regs and perform syscall.
        
        let msg = "Hello Syscall Test\n";
        let ptr = msg.as_ptr() as u64;
        let len = msg.len() as u64;
        
        let mut ret: u64;
        let r12_out: u64;
        let r13_out: u64;
        let r14_out: u64;
        let r15_out: u64;

        core::arch::asm!(
            "syscall",
            inlateout("rax") 1u64 => ret, // SYSCALL_LOG = 1
            in("rdi") ptr,
            in("rsi") len,
            // Use inout to check preservation
            inout("r12") r12_val => r12_out,
            inout("r13") r13_val => r13_out,
            inout("r14") r14_val => r14_out,
            inout("r15") r15_val => r15_out,
            
            // Volatile clobbers
            lateout("rcx") _,
            lateout("r11") _,
            lateout("rdx") _, 
            lateout("r8") _,
            lateout("r9") _,
            lateout("r10") _,
        );
        
        // Helper to log a string
        let log_str = |s: &str| {
            let p = s.as_ptr() as u64;
            let l = s.len() as u64;
            let mut ret2: u64;
            core::arch::asm!(
                "syscall",
                inlateout("rax") 1u64 => ret2,
                in("rdi") p,
                in("rsi") l,
                lateout("rcx") _,
                lateout("r11") _,
                lateout("rdx") _,
            );
        };

        if r12_out != 0x1212_1212_1212_1212 { log_str("R12 corrupted\n"); loop {} }
        if r13_out != 0x1313_1313_1313_1313 { log_str("R13 corrupted\n"); loop {} }
        if r14_out != 0x1414_1414_1414_1414 { log_str("R14 corrupted\n"); loop {} }
        if r15_out != 0x1515_1515_1515_1515 { log_str("R15 corrupted\n"); loop {} }
        
        log_str("SUCCESS: Registers Preserved\n");
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
