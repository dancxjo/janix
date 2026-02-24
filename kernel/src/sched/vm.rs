use super::SCHEDULER;
use super::types::Scheduler;
use crate::{BootRuntime, BootTasking};
use abi::errors::Errno;
use abi::vm::VmRegionInfo;
use alloc::vec::Vec;
use alloc::sync::Arc;
use spin::Mutex;
use crate::memory::mappings::MappingList;

#[allow(clippy::declare_interior_mutable_const)]
const EMPTY_MAPPING: Mutex<Option<Arc<Mutex<MappingList>>>> = Mutex::new(None);
pub static CURRENT_MAPPINGS: [Mutex<Option<Arc<Mutex<MappingList>>>>; 32] = [EMPTY_MAPPING; 32];

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
        let cpu = super::current_cpu_index::<R>();
        let current_id = match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => return Err(Errno::ESRCH),
        };

        if let Some(task) = crate::task::registry::get_task::<R>(current_id) {
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
        let cpu = super::current_cpu_index::<R>();
        let current_id = match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => return Err(Errno::ESRCH),
        };

        if let Some(task) = crate::task::registry::get_task::<R>(current_id) {
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
    let cpu = super::current_cpu_index::<R>();
    let mappings = CURRENT_MAPPINGS[cpu].lock().clone();
    if let Some(m) = mappings {
        m.lock().check(addr, len, write)
    } else {
        false
    }
}

pub fn get_user_mapping_at<R: BootRuntime>(addr: usize) -> Option<VmRegionInfo> {
    let cpu = super::current_cpu_index::<R>();
    let mappings = CURRENT_MAPPINGS[cpu].lock().clone();
    if let Some(m) = mappings {
        m.lock().find_at(addr)
    } else {
        None
    }
}

pub unsafe fn translate_user_page<R: BootRuntime>(addr: u64) -> Option<u64> {
    let rt = crate::runtime::<R>();
    let _irq = rt.irq_disable();
    let lock = SCHEDULER.lock();
    let res = if let Some(ptr) = *lock {
        let sched = unsafe { &mut *(ptr as *mut Scheduler<R>) };
        let cpu = super::current_cpu_index::<R>();
        let current_id = match sched.state.per_cpu.get(cpu).and_then(|pc| pc.current) {
            Some(id) => id,
            None => {
                rt.irq_restore(_irq);
                return None;
            }
        };

        if let Some(task) = crate::task::registry::get_task::<R>(current_id) {
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
