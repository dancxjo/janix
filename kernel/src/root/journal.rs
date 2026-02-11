use alloc::collections::VecDeque;

/// Maximum journal entries to keep in memory
const MAX_JOURNAL_ENTRIES: usize = 1024;

pub enum JournalOp {
    Init,
    CreateResult { id: u64, kind: u64 },
    UpdateProp { id: u64, key: u64, val: u64 },
}

pub struct JournalEntry {
    pub seq: u64,
    pub op: JournalOp,
}

pub struct Journal {
    pub entries: VecDeque<JournalEntry>,
    pub next_seq: u64,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::with_capacity(MAX_JOURNAL_ENTRIES),
            next_seq: 1,
        }
    }

    pub fn append(&mut self, op: JournalOp) -> u64 {
        // Evict oldest entries if at capacity
        while self.entries.len() >= MAX_JOURNAL_ENTRIES {
            self.entries.pop_front();
        }

        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.push_back(JournalEntry { seq, op }); // In-memory only
        seq
    }
}
