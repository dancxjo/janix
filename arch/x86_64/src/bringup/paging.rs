//! Early paging bring-up for x86_64.
//!
//! Stub on x86_64; required for API symmetry with AArch64.
//! Limine sets up identity-mapped paging, so this is not needed.

/// Initialize paging early in boot.
///
/// # Safety
/// Must be called during early boot.
pub unsafe fn init_paging_early() {
    // Limine handles early paging setup.
    // Stub on x86_64 until implemented; required for API symmetry.
}
