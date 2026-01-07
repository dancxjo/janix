//! Syscall-side validation and user-memory helpers.
//!
//! # Examples
//!
//! ```rust,ignore
//! use crate::syscall::user_mem::{copy_from_user, copy_to_user};
//! use abi::syscall::err;
//!
//! let mut buf = [0u8; 16];
//! copy_from_user(&mut buf, user_ptr, buf.len()).map_err(|_| err::EFAULT)?;
//! copy_to_user(user_ptr, &buf, buf.len()).map_err(|_| err::EFAULT)?;
//! ```

use abi::cap::{Cap, CapOp, CapScope};
use abi::ids::ThingId;
use abi::syscall::err;
use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::memory::map::MapPerms;
use crate::memory::space::UserMappings;
use crate::sched;

pub type SysError = i32;

#[derive(Clone, Copy, Debug)]
pub struct UserSlice<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> UserSlice<'a> {
    pub fn as_ptr(self) -> *const u8 {
        self.ptr
    }

    pub fn len(self) -> usize {
        self.len
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UserSliceMut<'a> {
    ptr: *mut u8,
    len: usize,
    _marker: PhantomData<&'a mut [u8]>,
}

impl<'a> UserSliceMut<'a> {
    pub fn as_mut_ptr(self) -> *mut u8 {
        self.ptr
    }

    pub fn len(self) -> usize {
        self.len
    }
}

pub trait CapSource {
    fn caps(&self) -> &[Cap];
}

impl CapSource for crate::sched::task::Task {
    fn caps(&self) -> &[Cap] {
        &self.caps
    }
}

pub fn require_cap<T: CapSource>(
    task: &T,
    op: CapOp,
    target: Option<ThingId>,
) -> Result<(), SysError> {
    for cap in task.caps() {
        if cap.op != op {
            continue;
        }
        match cap.scope {
            CapScope::Global => return Ok(()),
            CapScope::Thing(id) => {
                if target == Some(id) {
                    return Ok(());
                }
            }
        }
    }
    Err(err::EPERM)
}

pub fn require_current_cap(op: CapOp, target: Option<ThingId>) -> Result<(), SysError> {
    sched::with_current_task(|task| require_cap(task, op, target))
        .unwrap_or(Err(err::EFAULT))
}

fn validate_user_range(ptr: u64, len: usize, user_end: u64) -> Result<(u64, u64), SysError> {
    if len == 0 {
        return Ok((ptr, ptr));
    }
    if ptr == 0 {
        return Err(err::EFAULT);
    }
    let end = ptr.checked_add(len as u64).ok_or(err::EFAULT)?;
    if ptr >= user_end || end > user_end {
        return Err(err::EFAULT);
    }
    Ok((ptr, end))
}

pub fn validate_user_read_in(
    mappings: &UserMappings,
    user_end: u64,
    ptr: u64,
    len: usize,
) -> Result<UserSlice<'static>, SysError> {
    let (start, _) = validate_user_range(ptr, len, user_end)?;
    if !mappings.permits(start, len, MapPerms::READ | MapPerms::USER) {
        return Err(err::EFAULT);
    }
    Ok(UserSlice {
        ptr: start as *const u8,
        len,
        _marker: PhantomData,
    })
}

pub fn validate_user_write_in(
    mappings: &UserMappings,
    user_end: u64,
    ptr: u64,
    len: usize,
) -> Result<UserSliceMut<'static>, SysError> {
    let (start, _) = validate_user_range(ptr, len, user_end)?;
    if !mappings.permits(start, len, MapPerms::WRITE | MapPerms::USER) {
        return Err(err::EFAULT);
    }
    Ok(UserSliceMut {
        ptr: start as *mut u8,
        len,
        _marker: PhantomData,
    })
}

pub fn validate_user_read(ptr: u64, len: usize) -> Result<UserSlice<'static>, SysError> {
    sched::with_current_task(|task| {
        let user_end = task.address_space.user_range_end();
        let slice = validate_user_read_in(task.address_space.user_mappings(), user_end, ptr, len)?;
        if !task
            .address_space
            .probe_user_range(ptr, len, MapPerms::READ | MapPerms::USER)
        {
            return Err(err::EFAULT);
        }
        Ok(slice)
    })
    .unwrap_or(Err(err::EFAULT))
}

pub fn validate_user_write(ptr: u64, len: usize) -> Result<UserSliceMut<'static>, SysError> {
    sched::with_current_task(|task| {
        let user_end = task.address_space.user_range_end();
        let slice =
            validate_user_write_in(task.address_space.user_mappings(), user_end, ptr, len)?;
        if !task
            .address_space
            .probe_user_range(ptr, len, MapPerms::WRITE | MapPerms::USER)
        {
            return Err(err::EFAULT);
        }
        Ok(slice)
    })
    .unwrap_or(Err(err::EFAULT))
}

pub fn copy_from_user_in(
    mappings: &UserMappings,
    user_end: u64,
    dst: &mut [u8],
    user_ptr: u64,
    len: usize,
) -> Result<(), SysError> {
    if len > dst.len() {
        return Err(err::EINVAL);
    }
    let user_slice = validate_user_read_in(mappings, user_end, user_ptr, len)?;
    if len == 0 {
        return Ok(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(user_slice.as_ptr(), dst.as_mut_ptr(), len);
    }
    Ok(())
}

pub fn copy_to_user_in(
    mappings: &UserMappings,
    user_end: u64,
    user_ptr: u64,
    src: &[u8],
    len: usize,
) -> Result<(), SysError> {
    if len > src.len() {
        return Err(err::EINVAL);
    }
    let user_slice = validate_user_write_in(mappings, user_end, user_ptr, len)?;
    if len == 0 {
        return Ok(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), user_slice.as_mut_ptr(), len);
    }
    Ok(())
}

pub fn copy_from_user(dst: &mut [u8], user_ptr: u64, len: usize) -> Result<(), SysError> {
    let user_slice = validate_user_read(user_ptr, len)?;
    if len > dst.len() {
        return Err(err::EINVAL);
    }
    if len == 0 {
        return Ok(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(user_slice.as_ptr(), dst.as_mut_ptr(), len);
    }
    Ok(())
}

pub fn copy_to_user(user_ptr: u64, src: &[u8], len: usize) -> Result<(), SysError> {
    let user_slice = validate_user_write(user_ptr, len)?;
    if len > src.len() {
        return Err(err::EINVAL);
    }
    if len == 0 {
        return Ok(());
    }
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), user_slice.as_mut_ptr(), len);
    }
    Ok(())
}

pub fn copy_from_user_vec(user_ptr: u64, len: usize) -> Result<Vec<u8>, SysError> {
    let mut buf = alloc::vec![0u8; len];
    copy_from_user(&mut buf, user_ptr, len)?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sched::test_harness::{with_test_task, map_user_memory};
    use crate::memory::map::MapPerms;

    struct TestCaps {
        caps: Vec<Cap>,
    }

    impl CapSource for TestCaps {
        fn caps(&self) -> &[Cap] {
            &self.caps
        }
    }

    #[test]
    fn require_cap_rejects_missing() {
        let caps = TestCaps { caps: Vec::new() };
        let err = require_cap(&caps, CapOp::Log, None).unwrap_err();
        assert_eq!(err, err::EPERM);
    }

    #[test]
    fn validate_user_range_rejects_bad_pointer() {
        let mappings = UserMappings::new();
        let err = validate_user_read_in(&mappings, u64::MAX, 0, 8).unwrap_err();
        assert_eq!(err, err::EFAULT);
    }

    #[test]
    fn validate_user_range_rejects_overflow() {
        let mappings = UserMappings::new();
        let err =
            validate_user_read_in(&mappings, u64::MAX, u64::MAX - 1, 4).unwrap_err();
        assert_eq!(err, err::EFAULT);
    }

    #[test]
    fn validate_and_copy_roundtrip_manual() {
        let mappings = UserMappings::new();
        let mut backing = [0u8; 64];
        let base = backing.as_mut_ptr() as u64;
        let user_end = base + backing.len() as u64 + 1;
        mappings.record(
            base,
            backing.len(),
            MapPerms::READ | MapPerms::WRITE | MapPerms::USER,
        );

        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        let dst_len = dst.len();

        copy_to_user_in(&mappings, user_end, base, &src, src.len()).unwrap();
        copy_from_user_in(&mappings, user_end, &mut dst, base, dst_len).unwrap();

        assert_eq!(src, dst);
    }

    #[test]
    fn require_cap_scoped_allows_match() {
        let caps = TestCaps {
            caps: alloc::vec![Cap {
                op: CapOp::GraphRead,
                scope: CapScope::Thing(ThingId(42)),
            }],
        };
        require_cap(&caps, CapOp::GraphRead, Some(ThingId(42))).unwrap();
    }

    #[test]
    fn copy_from_user_integrated() {
        with_test_task(|tid| {
            let buf = [0xAAu8; 16];
            let ptr = buf.as_ptr() as u64;

            map_user_memory(tid, ptr, 16, MapPerms::READ | MapPerms::USER);

            let mut dst = [0u8; 16];
            copy_from_user(&mut dst, ptr, 16).expect("copy_from_user failed");
            assert_eq!(dst, buf);
        });
    }

    #[test]
    fn copy_to_user_integrated() {
        with_test_task(|tid| {
            let mut buf = [0u8; 16];
            let ptr = buf.as_mut_ptr() as u64;

            map_user_memory(tid, ptr, 16, MapPerms::WRITE | MapPerms::USER);

            let src = [0xBBu8; 16];
            copy_to_user(ptr, &src, 16).expect("copy_to_user failed");
            assert_eq!(buf, src);
        });
    }

    #[test]
    fn integrated_perms_check() {
        with_test_task(|tid| {
            let buf = [0u8; 16];
            let ptr = buf.as_ptr() as u64;

            map_user_memory(tid, ptr, 16, MapPerms::READ | MapPerms::USER);

            let src = [0u8; 16];
            let err = copy_to_user(ptr, &src, 16).unwrap_err();
            assert_eq!(err, err::EFAULT);
        });
    }
}
