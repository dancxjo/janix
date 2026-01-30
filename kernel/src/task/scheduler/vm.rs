use super::SCHEDULER;
use super::types::Scheduler;
use crate::{BootRuntime, BootTasking};
use abi::errors::Errno;
use abi::vm::VmRegionInfo;
use alloc::vec::Vec;

pub fn add_user_mapping<R: BootRuntime>(region: VmRegionInfo) -> Result<(), Errno> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = (|| {
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
    })();
    rt.irq_restore(_irq);
    res
}

pub fn remove_user_mappings<R: BootRuntime>(
    addr: usize,
    len: usize,
) -> Result<Vec<(usize, usize)>, Errno> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = (|| {
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
    })();
    rt.irq_restore(_irq);
    res
}

pub fn check_user_mapping<R: BootRuntime>(addr: usize, len: usize, write: bool) -> bool {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let current_id = match sched.current {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return false;
            }
        };

        if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
            let mappings = task.mappings.lock();
            mappings.check(addr, len, write)
        } else {
            false
        }
    } else {
        false
    };
    rt.irq_restore(_irq);
    res
}

pub fn get_user_mapping_at<R: BootRuntime>(addr: usize) -> Option<VmRegionInfo> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let current_id = match sched.current {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return None;
            }
        };

        if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
            let mappings = task.mappings.lock();
            mappings.find_at(addr)
        } else {
            None
        }
    } else {
        None
    };
    rt.irq_restore(_irq);
    res
}

pub unsafe fn translate_user_page<R: BootRuntime>(addr: u64) -> Option<u64> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let current_id = match sched.current {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return None;
            }
        };

        if let Some(task) = sched.tasks.iter().find(|t| t.id == current_id) {
            rt.tasking().translate(task.aspace, addr)
        } else {
            None
        }
    } else {
        None
    };
    rt.irq_restore(_irq);
    res
}
