use abi::root::{JournalIndex, WatchEvent};
use alloc::collections::BTreeMap;
use crate::root::journal::Journal;

pub struct Watch {
    id: u64,
    next_index: JournalIndex,
}

pub struct Watches {
    next_id: u64,
    items: BTreeMap<u64, Watch>,
}

impl Watches {
    pub const fn new() -> Self {
        Self {
            next_id: 1,
            items: BTreeMap::new(),
        }
    }

    pub fn create(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.items.insert(id, Watch {
            id,
            next_index: 0, // Watches start from the beginning of time
        });
        id
    }

    // Success: WatchEvent
    // Error: -EAGAIN if no new events, or other error
    pub fn next(&mut self, id: u64, journal: &Journal) -> Result<WatchEvent, i64> {
        let watch = self.items.get_mut(&id).ok_or(-1)?; // -1 for Invalid Argument / Access Denied?
        
        if let Some(op) = journal.get(watch.next_index) {
            let event = WatchEvent {
                index: watch.next_index,
                op,
            };
            watch.next_index += 1;
            Ok(event)
        } else {
            Err(-11) // EAGAIN
        }
    }
}
