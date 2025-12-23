#[cfg(target_arch = "x86_64")]
mod inner {
    use abi::ThreadId;
    use abi::syscall_defs::{DeviceHandle, DeviceKind, SysError, SysRet};
    use alloc::collections::VecDeque;
    use spin::Mutex;
    use x86_64::instructions::interrupts;

    pub const KEYBOARD_BUFFER_SIZE: usize = 256;
    pub const MOUSE_BUFFER_SIZE: usize = 256;

    pub static PS2_KEYBOARD_BUFFER: Mutex<Option<(VecDeque<u8>, Option<ThreadId>)>> =
        Mutex::new(None);
    pub static PS2_MOUSE_BUFFER: Mutex<Option<(VecDeque<u8>, Option<ThreadId>)>> = Mutex::new(None);

    pub fn init() {
        // Initialize buffers
        interrupts::without_interrupts(|| {
            *PS2_KEYBOARD_BUFFER.lock() =
                Some((VecDeque::with_capacity(KEYBOARD_BUFFER_SIZE), None));
            *PS2_MOUSE_BUFFER.lock() = Some((VecDeque::with_capacity(MOUSE_BUFFER_SIZE), None));
        });
    }

    pub fn push_keyboard_byte(byte: u8) {
        // ISR runs with interrupts disabled, so safe to lock directly IF thread also disables interrupts
        // We assume thread uses without_interrupts when locking.
        if let Some((queue, waiter)) = &mut *PS2_KEYBOARD_BUFFER.lock() {
            if queue.len() >= KEYBOARD_BUFFER_SIZE {
                queue.pop_front(); // Drop oldest
            }
            queue.push_back(byte);
            if let Some(tid) = waiter.take() {
                // Wake the waiting thread
                use crate::sched;
                sched::SCHEDULER.lock().wake_thread(tid);
            }
        }
    }

    pub fn push_mouse_byte(byte: u8) {
        if let Some((queue, waiter)) = &mut *PS2_MOUSE_BUFFER.lock() {
            if queue.len() >= MOUSE_BUFFER_SIZE {
                queue.pop_front();
            }
            queue.push_back(byte);
            if let Some(tid) = waiter.take() {
                // Wake the waiting thread
                use crate::sched;
                sched::SCHEDULER.lock().wake_thread(tid);
            }
        }
    }

    pub fn dev_open(kind: u32, index: u32) -> Result<DeviceHandle, SysError> {
        if index != 0 {
            return Err(SysError {
                code: SysError::NOT_FOUND,
                detail: 0,
            });
        }
        match kind {
            1 => Ok(DeviceHandle { raw: 1 }), // Keyboard
            2 => Ok(DeviceHandle { raw: 2 }), // Mouse
            _ => Err(SysError {
                code: SysError::NOT_FOUND,
                detail: 0,
            }),
        }
    }

    pub fn dev_read(handle: DeviceHandle, out: &mut [u8]) -> Result<usize, SysError> {
        // MUST disable interrupts to avoid deadlock with ISR
        interrupts::without_interrupts(|| {
            let mut guard = match handle.raw {
                1 => PS2_KEYBOARD_BUFFER.lock(),
                2 => PS2_MOUSE_BUFFER.lock(),
                _ => {
                    return Err(SysError {
                        code: SysError::BAD_HANDLE,
                        detail: 0,
                    });
                }
            };

            if let Some((queue, waiter)) = &mut *guard {
                if queue.is_empty() {
                    // Register waiter
                    use crate::sched;
                    if let Some(tid) = sched::SCHEDULER.lock().current_id() {
                        *waiter = Some(tid);
                        return Err(SysError {
                            code: SysError::WOULD_BLOCK,
                            detail: 0,
                        });
                    } else {
                        // Should be impossible if called from syscall context
                        return Err(SysError {
                            code: SysError::INTERNAL,
                            detail: 0,
                        });
                    }
                }

                let mut count = 0;
                for slot in out.iter_mut() {
                    if let Some(byte) = queue.pop_front() {
                        *slot = byte;
                        count += 1;
                    } else {
                        break;
                    }
                }
                Ok(count)
            } else {
                Err(SysError {
                    code: SysError::INTERNAL,
                    detail: 0,
                })
            }
        })
    }

    // Actualizer helpers
    pub fn pop_keyboard_byte() -> Option<u8> {
        interrupts::without_interrupts(|| {
            if let Some((queue, _)) = &mut *PS2_KEYBOARD_BUFFER.lock() {
                queue.pop_front()
            } else {
                None
            }
        })
    }

    pub fn set_keyboard_waiter(tid: ThreadId) {
        interrupts::without_interrupts(|| {
             if let Some((_, waiter)) = &mut *PS2_KEYBOARD_BUFFER.lock() {
                 *waiter = Some(tid);
             }
        })
    }

    pub fn pop_mouse_byte() -> Option<u8> {
        interrupts::without_interrupts(|| {
            if let Some((queue, _)) = &mut *PS2_MOUSE_BUFFER.lock() {
                queue.pop_front()
            } else {
                None
            }
        })
    }

    pub fn set_mouse_waiter(tid: ThreadId) {
        interrupts::without_interrupts(|| {
             if let Some((_, waiter)) = &mut *PS2_MOUSE_BUFFER.lock() {
                 *waiter = Some(tid);
             }
        })
    }}

#[cfg(target_arch = "x86_64")]
pub use inner::*;

#[cfg(not(target_arch = "x86_64"))]
pub fn init() {}

#[cfg(not(target_arch = "x86_64"))]
pub fn push_keyboard_byte(_byte: u8) {}

#[cfg(not(target_arch = "x86_64"))]
pub fn push_mouse_byte(_byte: u8) {}

#[cfg(not(target_arch = "x86_64"))]
use abi::syscall_defs::{DeviceHandle, SysError};

#[cfg(not(target_arch = "x86_64"))]
pub fn dev_open(_kind: u32, _index: u32) -> Result<DeviceHandle, SysError> {
    Err(SysError {
        code: SysError::NOT_FOUND,
        detail: 0,
    })
}

#[cfg(not(target_arch = "x86_64"))]
pub fn dev_read(_handle: DeviceHandle, _out: &mut [u8]) -> Result<usize, SysError> {
    Err(SysError {
        code: SysError::NOT_FOUND,
        detail: 0,
    })
}
