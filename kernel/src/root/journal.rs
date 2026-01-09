use abi::root::{JournalIndex, JournalOp};
use core::mem::MaybeUninit;

// "Large enough and never wraps" for v0.
pub const CAPACITY: usize = 65536;

pub struct Journal {
    ops: [MaybeUninit<JournalOp>; CAPACITY],
    head: usize,
}

impl Journal {
    pub const fn new() -> Self {
        Self {
            ops: [MaybeUninit::uninit(); CAPACITY],
            head: 0,
        }
    }

    pub fn append(&mut self, op: JournalOp) -> JournalIndex {
        if self.head >= CAPACITY {
            // Panic as per v0 "boring" spec; ring buffers are distinct from this linear journal for now.
            panic!("Root journal exhausted");
        }
        
        let idx = self.head;
        self.ops[idx] = MaybeUninit::new(op);
        self.head += 1;
        
        idx as JournalIndex
    }

    pub fn get(&self, index: JournalIndex) -> Option<JournalOp> {
        let idx = index as usize;
        if idx >= self.head {
            return None;
        }

        // Safety: We only advance head after initializing the element.
        // We verified idx < head, so it must be initialized.
        unsafe { Some(self.ops[idx].assume_init()) }
    }
}
