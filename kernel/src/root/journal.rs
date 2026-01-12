use alloc::vec::Vec;

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
    pub entries: Vec<JournalEntry>,
    pub next_seq: u64,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_seq: 1,
        }
    }

    pub fn append(&mut self, op: JournalOp) -> u64 {
        let seq = self.next_seq;
        self.next_seq += 1;
        self.entries.push(JournalEntry { seq, op }); // In-memory only
        seq
    }
}
