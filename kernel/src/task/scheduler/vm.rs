use crate::BootRuntime;
use abi::errors::Errno;
use abi::vm::VmRegionInfo;
use alloc::vec::Vec;
use super::SCHEDULER;
use super::types::Scheduler;

pub fn add_user_mapping<R: BootRuntime>(region: VmRegionInfo) -> Result<(), Errno> {
    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(ptr) => ptr,
        None => return Err(Errno::ENOSYS),
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let current_id = match sched.current {
        Some(id) => id,
        None => return Err(Errno::ESRCH),
    };

    if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
        let mut mappings = task.mappings.lock();
        mappings.insert(region);
        Ok(())
    } else {
        Err(Errno::ESRCH)
    }
}

pub fn remove_user_mappings<R: BootRuntime>(addr: usize, len: usize) -> Result<Vec<(usize, usize)>, Errno> {
    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(ptr) => ptr,
        None => return Err(Errno::ENOSYS),
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let current_id = match sched.current {
        Some(id) => id,
        None => return Err(Errno::ESRCH),
    };

    if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
        let mut mappings = task.mappings.lock();
        Ok(mappings.remove(addr, len))
    } else {
        Err(Errno::ESRCH)
    }
}

pub fn check_user_mapping<R: BootRuntime>(addr: usize, len: usize, write: bool) -> bool {
    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(ptr) => ptr,
        None => return false,
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let current_id = match sched.current {
        Some(id) => id,
        None => return false,
    };

    if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
        let mappings = task.mappings.lock();
        mappings.check(addr, len, write)
    } else {
        false
    }
}

pub fn get_user_mapping_at<R: BootRuntime>(addr: usize) -> Option<VmRegionInfo> {
    let lock = SCHEDULER.lock();
    let ptr = match *lock {
        Some(ptr) => ptr,
        None => return None,
    };
    let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
    let current_id = match sched.current {
        Some(id) => id,
        None => return None,
    };

    if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
        let mappings = task.mappings.lock();
        mappings.find_at(addr)
    } else {
        None
    }
}
