use crate::graph::GraphEvent;
use alloc::vec::Vec;
use core::slice;

// Fixed size circular buffer
const JOURNAL_SIZE: usize = 1024;

/// A recorded event in the journal
#[derive(Debug, Clone)]
pub struct JournalEntry {
    pub sequence: u64,
    pub event: GraphEvent,
}

pub struct EventJournal {
    buffer: [Option<JournalEntry>; JOURNAL_SIZE],
    next_seq: u64,
    head: usize,
}

impl EventJournal {
    const fn new() -> Self {
        Self {
            buffer: [const { None }; JOURNAL_SIZE],
            next_seq: 1,
            head: 0,
        }
    }

    fn push(&mut self, event: GraphEvent) {
        let entry = JournalEntry {
            sequence: self.next_seq,
            event,
        };

        self.buffer[self.head] = Some(entry);
        self.head = (self.head + 1) % JOURNAL_SIZE;
        self.next_seq += 1;
    }
}

static mut JOURNAL: EventJournal = EventJournal::new();

pub fn init() {
    // Initialized by static
}

pub fn push(event: GraphEvent) {
    unsafe {
        let ptr = &raw mut JOURNAL;
        (*ptr).push(event);
    }
}

/// Iterator over journal entries
pub struct JournalIterator {
    current_seq: u64,
    end_seq: u64,
    head_at_start: usize,
}

impl Iterator for JournalIterator {
    type Item = &'static JournalEntry;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let ptr = &raw const JOURNAL;
            let journal = &*ptr;

            if self.current_seq >= self.end_seq {
                return None;
            }

            // Find index for current_seq.
            // Since it's a ring buffer, we need to map sequence to index.
            // But sequence numbers are strictly increasing.
            // The newest item is at `head - 1` (modulo size). Its seq is `next_seq - 1`.
            // So `index = (head - (next_seq - target_seq)) % SIZE` ?

            // Simpler: We know `head` points to the *next* write slot.
            // So `head - 1` is the last written.
            // The buffer holds sequences [next_seq - size .. next_seq).

            // If requested seq is too old, we can't provide it.
            let oldest_seq = if journal.next_seq > JOURNAL_SIZE as u64 {
                journal.next_seq - JOURNAL_SIZE as u64
            } else {
                1
            };

            if self.current_seq < oldest_seq {
                // Consumer lagged too much. In a real system we'd error or skip.
                // For now, jump to oldest.
                self.current_seq = oldest_seq;
            }

            if self.current_seq >= journal.next_seq {
                return None; // Caught up
            }

            // Calculate index
            // If `next_seq` maps to `head`, then `seq` maps to:
            // `head - (next_seq - seq)`

            let offset_from_end = journal.next_seq - self.current_seq;
            let index = (journal.head + JOURNAL_SIZE - (offset_from_end as usize % JOURNAL_SIZE))
                % JOURNAL_SIZE;

            let entry = journal.buffer[index].as_ref()?;

            if entry.sequence != self.current_seq {
                // Should not happen if logic is correct and no overwrite race during read
                // (Kernel is single threaded so no race).
                return None;
            }

            self.current_seq += 1;
            Some(entry)
        }
    }
}

pub fn read_since(seq: u64) -> JournalIterator {
    unsafe {
        let ptr = &raw const JOURNAL;
        let journal = &*ptr;
        JournalIterator {
            current_seq: seq,
            end_seq: journal.next_seq,
            head_at_start: journal.head,
        }
    }
}
