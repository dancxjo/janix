use core::arch::asm;

pub fn save(area: &mut [u8; 512]) {
    unsafe {
        let ptr = area.as_mut_ptr();
        asm!(
            "stp q0, q1, [{0}, #0]",
            "stp q2, q3, [{0}, #32]",
            "stp q4, q5, [{0}, #64]",
            "stp q6, q7, [{0}, #96]",
            "stp q8, q9, [{0}, #128]",
            "stp q10, q11, [{0}, #160]",
            "stp q12, q13, [{0}, #192]",
            "stp q14, q15, [{0}, #224]",
            "stp q16, q17, [{0}, #256]",
            "stp q18, q19, [{0}, #288]",
            "stp q20, q21, [{0}, #320]",
            "stp q22, q23, [{0}, #352]",
            "stp q24, q25, [{0}, #384]",
            "stp q26, q27, [{0}, #416]",
            "stp q28, q29, [{0}, #448]",
            "stp q30, q31, [{0}, #480]",
            in(reg) ptr,
        );
    }
}

pub fn restore(area: &[u8; 512]) {
    unsafe {
        let ptr = area.as_ptr();
        asm!(
            "ldp q0, q1, [{0}, #0]",
            "ldp q2, q3, [{0}, #32]",
            "ldp q4, q5, [{0}, #64]",
            "ldp q6, q7, [{0}, #96]",
            "ldp q8, q9, [{0}, #128]",
            "ldp q10, q11, [{0}, #160]",
            "ldp q12, q13, [{0}, #192]",
            "ldp q14, q15, [{0}, #224]",
            "ldp q16, q17, [{0}, #256]",
            "ldp q18, q19, [{0}, #288]",
            "ldp q20, q21, [{0}, #320]",
            "ldp q22, q23, [{0}, #352]",
            "ldp q24, q25, [{0}, #384]",
            "ldp q26, q27, [{0}, #416]",
            "ldp q28, q29, [{0}, #448]",
            "ldp q30, q31, [{0}, #480]",
            in(reg) ptr,
        );
    }
}
