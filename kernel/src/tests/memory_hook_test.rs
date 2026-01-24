//! Memory hook error mapping tests

use crate::memory::MapError;
use abi::errors::Errno;

pub fn run_selftest() {
    crate::kinfo!("MEMORY HOOK TEST: Starting...");
    
    // Test error code mapping
    assert_eq!(MapError::OutOfMemory.to_errno(), Errno::ENOMEM);
    assert_eq!(MapError::PageTableFault.to_errno(), Errno::EFAULT);
    assert_eq!(MapError::AlreadyMapped.to_errno(), Errno::EEXIST);
    assert_eq!(MapError::NotMapped.to_errno(), Errno::EINVAL);
    assert_eq!(MapError::AccessDenied.to_errno(), Errno::EACCES);
    
    crate::kinfo!("MEMORY HOOK TEST: PASS");
}
