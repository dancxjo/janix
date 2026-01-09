use crate::trap::x86_64::TrapFrame;

unsafe extern "C" {
    fn enter_user_sysret_asm(tf: *const TrapFrame) -> !;
}

pub unsafe fn enter_user_sysret(tf: &TrapFrame) -> ! {
    // ... (checks)
    let rip = tf.user_rip;
    if (rip & 1 << 47) != 0 && (rip >> 48) != 0xFFFF {
         panic!("Invalid user RIP: non-canonical");
    }
    if (rip as i64) < 0 {
         panic!("Invalid user RIP: kernel range");
    }

    unsafe { enter_user_sysret_asm(tf) }
}
