use spin::Mutex;
use alloc::collections::VecDeque;
use abi::syscall_defs::{DeviceHandle, DeviceKind, SysError, SysRet};
use x86_64::instructions::interrupts;

pub const KEYBOARD_BUFFER_SIZE: usize = 256;
pub const MOUSE_BUFFER_SIZE: usize = 256;

// Use Mutex<VecDeque> for simplicity and safety via without_interrupts
pub static PS2_KEYBOARD_BUFFER: Mutex<Option<VecDeque<u8>>> = Mutex::new(None);
pub static PS2_MOUSE_BUFFER: Mutex<Option<VecDeque<u8>>> = Mutex::new(None);

pub fn init() {
    // Initialize buffers
    interrupts::without_interrupts(|| {
        *PS2_KEYBOARD_BUFFER.lock() = Some(VecDeque::with_capacity(KEYBOARD_BUFFER_SIZE));
        *PS2_MOUSE_BUFFER.lock() = Some(VecDeque::with_capacity(MOUSE_BUFFER_SIZE));
    });
}

pub fn push_keyboard_byte(byte: u8) {
    // ISR runs with interrupts disabled, so safe to lock directly IF thread also disables interrupts
    // We assume thread uses without_interrupts when locking.
    if let Some(queue) = &mut *PS2_KEYBOARD_BUFFER.lock() {
        if queue.len() >= KEYBOARD_BUFFER_SIZE {
            queue.pop_front(); // Drop oldest
        }
        queue.push_back(byte);
    }
}

pub fn push_mouse_byte(byte: u8) {
    if let Some(queue) = &mut *PS2_MOUSE_BUFFER.lock() {
        if queue.len() >= MOUSE_BUFFER_SIZE {
            queue.pop_front();
        }
        queue.push_back(byte);
    }
}

pub fn dev_open(kind: u32, index: u32) -> Result<DeviceHandle, SysError> {
    if index != 0 {
        return Err(SysError { code: SysError::NOT_FOUND, detail: 0 });
    }
    match kind {
        1 => Ok(DeviceHandle { raw: 1 }), // Keyboard
        2 => Ok(DeviceHandle { raw: 2 }), // Mouse
        _ => Err(SysError { code: SysError::NOT_FOUND, detail: 0 }),
    }
}

pub fn dev_read(handle: DeviceHandle, out: &mut [u8]) -> Result<usize, SysError> {
    // MUST disable interrupts to avoid deadlock with ISR
    interrupts::without_interrupts(|| {
        let mut guard = match handle.raw {
            1 => PS2_KEYBOARD_BUFFER.lock(),
            2 => PS2_MOUSE_BUFFER.lock(),
             _ => return Err(SysError { code: SysError::BAD_HANDLE, detail: 0 }),
        };

        if let Some(queue) = &mut *guard {
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
             Err(SysError { code: SysError::INTERNAL, detail: 0 })
        }
    })
}
