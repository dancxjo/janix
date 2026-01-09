pub mod journal;
pub mod graph;
pub mod watch;

use spin::Mutex;
use abi::root::JournalOp;
use self::journal::Journal;
use self::graph::Graph;
use self::watch::Watches;

pub struct Root {
    // "Journal protected by a single lock"
    // We'll wrap the whole state in a lock for v0 simplicity or just the journal?
    // The prompt says "Journal protected by a single lock", "Append + index increment is atomic".
    // For v0, a single big lock over (Journal + Graph + Watches) is the most "boring" and correct way to ensure consistency.
    // But the prompt separates them somewhat.
    // "Journal protected by a single lock"
    // "Graph state is derived"
    // "Watches ... separate?" 
    
    // Let's protect them all together for now to guarantee no torn reads/writes between journal and graph.
    // Or, we can follow the implied structure:
    // Root { journal: Mutex<Journal>, graph: Mutex<Graph>, ... }
    
    // Actually, when we append, we append to journal AND apply to graph AND notify watches.
    // So a single lock `Mutex<Inner>` seems best.
    
    inner: Mutex<Inner>,
}

struct Inner {
    journal: Journal,
    graph: Graph,
    watches: Watches,
}


impl Root {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                journal: Journal::new(),
                graph: Graph::new(),
                watches: Watches::new(),
            }),
        }
    }

    pub fn append(&self, op: JournalOp) -> abi::root::JournalIndex {
        let mut inner = self.inner.lock();
        let idx = inner.journal.append(op);
        inner.graph.apply(&op);
        idx
    }

    pub fn watch_create(&self) -> u64 {
        let mut inner = self.inner.lock();
        inner.watches.create()
    }

    pub fn watch_next(&self, id: u64) -> Result<abi::root::WatchEvent, i64> {
        let mut inner = self.inner.lock();
        // Split borrow to satisfy borrow checker
        let Inner { ref journal, ref mut watches, .. } = *inner;
        watches.next(id, journal)
    }
}
