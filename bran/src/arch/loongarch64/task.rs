use core::arch::global_asm;
use kernel::boot::ArchContext;

unsafe extern "C" {
    pub fn context_switch(old_handle_ptr: *mut u64, new_handle: u64);
}

// Trampoline for new threads
unsafe extern "C" {
    fn trampoline();
}

#[derive(Default, Debug)]
pub struct Context {
    pub sp: u64,
}

impl ArchContext for Context {
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

// Based on loongarch64_switch_to from trunk, adjusted for our naming
global_asm!(r#"
.section .text
.global context_switch
context_switch:
    // a0 = old_handle_ptr (*mut u64)
    // a1 = new_handle (u64)
    
    // Save callee-saved registers
    // $r1 (ra), $r22 (fp), $r23-$r31 (s0-s8)
    // Total 11 registers. 11*8 = 88 bytes. 88 is 16-byte aligned? No.
    // 88 / 16 = 5.5.
    // Trunk used 88. But SP should be 16-byte aligned.
    // Let's use 96 to be safe? Or stick to 88 if hardware allows 8-byte alignment?
    // User manual says SP should be 16-byte aligned.
    // trunk usage: addi.d $sp, $sp, -88. This violates alignment if SP was aligned.
    // But maybe it worked by luck. I will use 96.
    
    addi.d $sp, $sp, -96
    
    // Save RA at top
    st.d $r1, $sp, 0
    st.d $r22, $sp, 8
    st.d $r23, $sp, 16
    st.d $r24, $sp, 24
    st.d $r25, $sp, 32
    st.d $r26, $sp, 40
    st.d $r27, $sp, 48
    st.d $r28, $sp, 56
    st.d $r29, $sp, 64
    st.d $r30, $sp, 72
    st.d $r31, $sp, 80
    // padding at 88..96

    st.d $sp, $a0, 0
    move $sp, $a1

    ld.d $r1, $sp, 0
    ld.d $r22, $sp, 8
    ld.d $r23, $sp, 16
    ld.d $r24, $sp, 24
    ld.d $r25, $sp, 32
    ld.d $r26, $sp, 40
    ld.d $r27, $sp, 48
    ld.d $r28, $sp, 56
    ld.d $r29, $sp, 64
    ld.d $r30, $sp, 72
    ld.d $r31, $sp, 80
    
    addi.d $sp, $sp, 96
    jirl $r0, $r1, 0

.global trampoline
trampoline:
    // We expect:
    // s0 ($r23) = arg
    // s1 ($r24) = entry
    
    move $a0, $r23
    jirl $r0, $r24, 0
    
    // Should not return
    break 0
"#);

pub fn context_init(
    kstack_top: u64,
    entry: extern "C" fn(usize) -> !,
    arg: usize,
) -> usize {
    let stack_size = 96;
    let sp = kstack_top - stack_size;
    let ptr = sp as *mut u64;

    unsafe {
        // r1 (ra) = trampoline
        ptr.offset(0).write(trampoline as usize as u64);
        
        // r22 (fp) = 0
        ptr.offset(1).write(0);
        
        // r23 (s0) = arg
        ptr.offset(2).write(arg as u64);
        
        // r24 (s1) = entry
        ptr.offset(3).write(entry as usize as u64);
        
        // r25..r31 (s2..s8) = 0 
        core::ptr::write_bytes(ptr.offset(4), 0, 7);
    }

    sp as usize
}
