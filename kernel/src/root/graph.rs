use super::resources::ResourceHandle;
use abi::symbols::SymbolId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use core::sync::atomic::AtomicU64;
use alloc::collections::VecDeque;

pub type ThingId = u64;

pub struct Node {
    pub kind: SymbolId,
    pub props: BTreeMap<SymbolId, u64>,
    pub resource: Option<ResourceHandle>,
    pub watches: Vec<(u64, ThingId)>,
    // Edges: list of (RelKind, Target)
    pub edges: Vec<(SymbolId, ThingId)>,
}

// ============================================================================
// Shared Commit History (Ring Buffer)
// ============================================================================

/// A single committed batch stored in history
pub struct CommitRecord {
    pub seq: u64,
    pub data: Vec<u8>,
    /// Compact summary for O(1) filter matching
    pub summary: CommitSummary,
}

// ============================================================================
// Commit Summary (O(1) Watch Filtering)
// ============================================================================

/// Maximum IDs to track in a summary (predicates/kinds)
pub const MAX_SUMMARY_IDS: usize = 16;
/// Maximum ThingIds to track in a summary (subjects)
pub const MAX_SUMMARY_THINGS: usize = 8;

/// Small fixed-capacity set for SymbolIds
/// 
/// Tracks unique IDs without allocation. If capacity is exceeded,
/// `overflowed` is set and the set cannot disprove membership.
#[derive(Debug, Clone)]
pub struct SmallIdSet {
    pub overflowed: bool,
    len: u8,
    ids: [u32; MAX_SUMMARY_IDS],
}

impl Default for SmallIdSet {
    fn default() -> Self {
        Self {
            overflowed: false,
            len: 0,
            ids: [0; MAX_SUMMARY_IDS],
        }
    }
}

impl SmallIdSet {
    /// Insert a unique ID. If at capacity, marks as overflowed.
    pub fn insert(&mut self, id: u32) {
        if self.overflowed {
            return;
        }
        // Check if already present
        for i in 0..self.len as usize {
            if self.ids[i] == id {
                return;
            }
        }
        // Add if room
        if (self.len as usize) < MAX_SUMMARY_IDS {
            self.ids[self.len as usize] = id;
            self.len += 1;
        } else {
            self.overflowed = true;
        }
    }
    
    /// Check if ID is in the set
    pub fn contains(&self, id: u32) -> bool {
        for i in 0..self.len as usize {
            if self.ids[i] == id {
                return true;
            }
        }
        false
    }
}

/// Small fixed-capacity set for ThingIds (subjects)
#[derive(Debug, Clone)]
pub struct SmallThingSet {
    pub overflowed: bool,
    len: u8,
    things: [u64; MAX_SUMMARY_THINGS],
}

impl Default for SmallThingSet {
    fn default() -> Self {
        Self {
            overflowed: false,
            len: 0,
            things: [0; MAX_SUMMARY_THINGS],
        }
    }
}

impl SmallThingSet {
    /// Insert a unique ThingId. If at capacity, marks as overflowed.
    pub fn insert(&mut self, id: u64) {
        if self.overflowed {
            return;
        }
        // Check if already present
        for i in 0..self.len as usize {
            if self.things[i] == id {
                return;
            }
        }
        // Add if room
        if (self.len as usize) < MAX_SUMMARY_THINGS {
            self.things[self.len as usize] = id;
            self.len += 1;
        } else {
            self.overflowed = true;
        }
    }
    
    /// Check if ThingId is in the set
    pub fn contains(&self, id: u64) -> bool {
        for i in 0..self.len as usize {
            if self.things[i] == id {
                return true;
            }
        }
        false
    }
}

/// Compact summary of what's in a commit (for O(1) filter matching)
/// 
/// Computed once at commit time from validated ops.
/// Filter matching checks these sets rather than re-parsing batch bytes.
#[derive(Debug, Clone, Default)]
pub struct CommitSummary {
    /// Predicate IDs used in PUT_EDGE ops (interned SymbolIds)
    pub predicates: SmallIdSet,
    /// Subject ThingIds used in PUT_EDGE and SET_PROP ops
    pub subjects: SmallThingSet,
    /// Kind IDs from CREATE_NODE ops (interned SymbolIds)
    pub kinds: SmallIdSet,
}

/// Hard limits for commit history
pub const COMMIT_HISTORY_MAX_COMMITS: usize = 1024;
pub const COMMIT_HISTORY_MAX_BYTES: usize = 32 * 1024 * 1024; // 32 MiB

/// Shared ring buffer of recent commits
/// 
/// Accessed by all watches via cursor. Protected by the same lock
/// used for graph writes (v0 simplicity).
pub struct CommitHistory {
    /// Monotonically increasing next sequence number
    /// (The next commit pushed will have this seq)
    pub next_seq: u64,
    
    /// Ring content in seq order
    ring: VecDeque<CommitRecord>,
    
    /// Total bytes currently in ring
    bytes: usize,
    
    /// Maximum number of commits to retain
    max_commits: usize,
    
    /// Maximum bytes of commit data to retain
    max_bytes: usize,
}

impl CommitHistory {
    /// Create a new commit history with the given limits
    pub fn new(max_commits: usize, max_bytes: usize) -> Self {
        Self {
            next_seq: 1, // First commit will be seq 1
            ring: VecDeque::new(),
            bytes: 0,
            max_commits,
            max_bytes,
        }
    }
    
    /// Create with default limits
    pub fn with_defaults() -> Self {
        Self::new(COMMIT_HISTORY_MAX_COMMITS, COMMIT_HISTORY_MAX_BYTES)
    }
    
    /// Push a new commit, evicting oldest until within limits
    /// 
    /// The `seq` must equal `self.next_seq` - this is enforced for contiguity.
    pub fn push(&mut self, seq: u64, data: Vec<u8>, summary: CommitSummary) {
        debug_assert_eq!(seq, self.next_seq, "CommitHistory: seq must be contiguous");
        
        let data_len = data.len();
        
        // Evict from front until we have room for this commit
        while !self.ring.is_empty() && 
              (self.ring.len() >= self.max_commits || 
               self.bytes + data_len > self.max_bytes) {
            if let Some(evicted) = self.ring.pop_front() {
                self.bytes -= evicted.data.len();
            }
        }
        
        // Push the new commit
        self.ring.push_back(CommitRecord { seq, data, summary });
        self.bytes += data_len;
        self.next_seq = seq + 1;
    }
    
    /// Get commit data by sequence number
    /// 
    /// Returns None if seq is not in the ring (evicted or not yet committed).
    pub fn get(&self, seq: u64) -> Option<&[u8]> {
        // The ring is contiguous: if we have oldest..newest, 
        // the index is (seq - oldest)
        let oldest = self.oldest_seq()?;
        if seq < oldest {
            return None;
        }
        let idx = (seq - oldest) as usize;
        self.ring.get(idx).map(|r| r.data.as_slice())
    }
    
    /// Oldest available sequence, if any
    pub fn oldest_seq(&self) -> Option<u64> {
        self.ring.front().map(|r| r.seq)
    }
    
    /// Newest available sequence, if any  
    pub fn newest_seq(&self) -> Option<u64> {
        self.ring.back().map(|r| r.seq)
    }
    
    /// Check if a sequence is available in the history
    pub fn contains(&self, seq: u64) -> bool {
        if let (Some(oldest), Some(newest)) = (self.oldest_seq(), self.newest_seq()) {
            seq >= oldest && seq <= newest
        } else {
            false
        }
    }
    
    /// Number of commits currently in history
    pub fn len(&self) -> usize {
        self.ring.len()
    }
    
    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.ring.is_empty()
    }
    
    /// Get full commit record by sequence number (for summary access)
    pub fn get_record(&self, seq: u64) -> Option<&CommitRecord> {
        let oldest = self.oldest_seq()?;
        if seq < oldest {
            return None;
        }
        let idx = (seq - oldest) as usize;
        self.ring.get(idx)
    }
}

// ============================================================================
// Watch Filter Matching (O(1) via Summary)
// ============================================================================

use abi::root::{WATCH_F_KIND, WATCH_F_PREDICATE, WATCH_F_SUBJECT};

/// Check if a commit matches a filter using its summary (O(1))
/// 
/// Matching rules:
/// - If filter flags == 0: match all
/// - If a flag bit is set, the corresponding field must match
/// - Overflowed sets conservatively match (no false negatives)
pub fn commit_matches(filter: &WatchFilter, summary: &CommitSummary) -> bool {
    // flags=0 means match all commits
    if filter.matches_all() {
        return true;
    }
    
    // Check PREDICATE filter
    if (filter.flags & WATCH_F_PREDICATE) != 0 {
        if !summary.predicates.overflowed && !summary.predicates.contains(filter.predicate_id) {
            return false;
        }
    }
    
    // Check SUBJECT filter  
    if (filter.flags & WATCH_F_SUBJECT) != 0 {
        if !summary.subjects.overflowed && !summary.subjects.contains(filter.subject_lo) {
            return false;
        }
    }
    
    // Check KIND filter
    if (filter.flags & WATCH_F_KIND) != 0 {
        if !summary.kinds.overflowed && !summary.kinds.contains(filter.kind_id) {
            return false;
        }
    }
    
    true
}

// ============================================================================
// Global Watch (Cursor-Only)
// ============================================================================

/// Maximum commits to scan per WATCH_NEXT call (prevents unbounded work)
pub const WATCH_SCAN_LIMIT: usize = 64;

/// Watch filter for graph mutations (copied from ABI)
#[derive(Debug, Clone, Copy, Default)]
pub struct WatchFilter {
    pub flags: u32,
    pub kind_id: u32,
    pub predicate_id: u32,
    pub subject_lo: u64,
}

impl WatchFilter {
    /// Check if filter matches all commits (no filtering)
    pub fn matches_all(&self) -> bool {
        self.flags == 0
    }
}

pub struct GlobalWatch {
    pub id: u64,
    pub spec_ptr: u64, // We store the pointer to user query for now
    pub stream_handle: ResourceHandle, 
    pub kind_filter: SymbolId, // "kind == Bytespace"
    pub missing_fact: SymbolId, // "missing fact(detector=...)"
    
    // Cursor-based tracking (references shared CommitHistory)
    /// Next sequence number this watch expects to read
    pub cursor_seq: u64,
    /// Sticky overflow flag - set when watch misses commits, cleared on -EOVERFLOW return
    pub overflowed: bool,
    /// Watch filter (flags=0 means match all)
    pub filter: WatchFilter,
}

// ============================================================================
// Graph
// ============================================================================

pub struct Graph {
    pub nodes: BTreeMap<ThingId, Node>,
    pub next_id: ThingId,
    pub root_seq: AtomicU64,
    pub kind_index: BTreeMap<SymbolId, Vec<ThingId>>,
    pub global_watches: BTreeMap<u64, GlobalWatch>,
    /// Shared commit history ring buffer
    pub commit_history: CommitHistory,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: BTreeMap::new(),
            next_id: 1,
            root_seq: AtomicU64::new(0),
            kind_index: BTreeMap::new(),
            global_watches: BTreeMap::new(),
            commit_history: CommitHistory::with_defaults(),
        }
    }

    pub fn alloc(&mut self, kind: SymbolId) -> ThingId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.insert(
            id,
            Node {
                kind,
                props: BTreeMap::new(),
                resource: None,
                watches: Vec::new(),
                edges: Vec::new(),
            },
        );

        self.kind_index.entry(kind).or_default().push(id);

        id
    }

    pub fn get_kind(&self, id: ThingId) -> Option<SymbolId> {
        self.nodes.get(&id).map(|n| n.kind)
    }

    pub fn get_resource(&self, id: ThingId) -> Option<ResourceHandle> {
        self.nodes.get(&id).and_then(|n| n.resource.clone())
    }

    pub fn get_node_mut(&mut self, id: ThingId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    pub fn link(&mut self, src: ThingId, rel: SymbolId, dst: ThingId) {
        if let Some(node) = self.nodes.get_mut(&src) {
            node.edges.push((rel, dst));
        }
    }
}
