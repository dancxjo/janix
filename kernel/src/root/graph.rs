use super::resources::ResourceHandle;
use abi::symbols::SymbolId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use alloc::collections::VecDeque;
use core::sync::atomic::AtomicU64;

pub type ThingId = u64;

pub struct Node {
    pub kind: SymbolId,
    pub props: BTreeMap<SymbolId, u64>,
    pub resource: Option<ResourceHandle>,
    pub watches: Vec<(u64, ThingId)>,
    // Edges: list of (RelKind, Target)
    pub edges: Vec<(SymbolId, ThingId)>,
    /// Owner of this thing (ThingId of the process/task that created it).
    /// None means kernel-owned or orphaned.
    pub owner: Option<ThingId>,
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
pub const COMMIT_HISTORY_MAX_BYTES: usize = 8 * 1024 * 1024; // 8 MiB - must fit within 32 MiB kernel heap

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
        while !self.ring.is_empty()
            && (self.ring.len() >= self.max_commits || self.bytes + data_len > self.max_bytes)
        {
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
    pub kind_filter: SymbolId,  // "kind == Bytespace"
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
    /// Reverse index for efficient incoming edge queries (Dst -> [(Rel, Src)])
    pub reverse_index: BTreeMap<ThingId, Vec<(SymbolId, ThingId)>>,
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
            reverse_index: BTreeMap::new(),
            global_watches: BTreeMap::new(),
            commit_history: CommitHistory::with_defaults(),
        }
    }

    pub fn alloc(&mut self, kind: SymbolId) -> ThingId {
        self.alloc_with_owner(kind, None)
    }

    pub fn alloc_with_owner(&mut self, kind: SymbolId, owner: Option<ThingId>) -> ThingId {
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
                owner,
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
        self.reverse_index.entry(dst).or_default().push((rel, src));
    }

    /// Remove a specific edge from src to dst with the given relation.
    /// Returns true if an edge was removed, false if not found.
    pub fn unlink(&mut self, src: ThingId, rel: SymbolId, dst: ThingId) -> bool {
        let mut removed = false;
        if let Some(node) = self.nodes.get_mut(&src) {
            let before = node.edges.len();
            node.edges.retain(|&(r, d)| !(r == rel && d == dst));
            removed = node.edges.len() < before;
        }
        if removed {
            if let Some(rev_list) = self.reverse_index.get_mut(&dst) {
                rev_list.retain(|&(r, s)| !(r == rel && s == src));
            }
        }
        removed
    }

    /// Remove a node from the graph, cleaning up indices.
    /// Used for evicting old log entries.
    pub fn remove_node(&mut self, id: ThingId) {
        if let Some(node) = self.nodes.remove(&id) {
            // Clean up kind_index
            if let Some(kind_list) = self.kind_index.get_mut(&node.kind) {
                kind_list.retain(|&x| x != id);
            }

            // Clean up reverse_index for outgoing edges
            for (_, dst) in &node.edges {
                if let Some(rev_list) = self.reverse_index.get_mut(dst) {
                    rev_list.retain(|(_, src)| *src != id);
                }
            }

            // Remove this node's reverse_index entry
            self.reverse_index.remove(&id);
        }
    }

    /// Get all things owned by a specific process/task.
    pub fn get_owned_things(&self, owner_id: ThingId) -> Vec<ThingId> {
        self.nodes
            .iter()
            .filter_map(|(&id, node)| {
                if node.owner == Some(owner_id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Transfer ownership of a thing to the kernel (orphan it).
    /// Returns true if the thing existed and was orphaned.
    pub fn orphan_thing(&mut self, thing_id: ThingId) -> bool {
        if let Some(node) = self.nodes.get_mut(&thing_id) {
            node.owner = None;
            true
        } else {
            false
        }
    }

    /// Set the owner of a thing.
    pub fn set_owner(&mut self, thing_id: ThingId, owner: Option<ThingId>) -> bool {
        if let Some(node) = self.nodes.get_mut(&thing_id) {
            node.owner = owner;
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_id_set() {
        let mut set = SmallIdSet::default();
        assert!(!set.overflowed);
        assert!(!set.contains(10));

        // Insert unique items
        for i in 0..MAX_SUMMARY_IDS {
            set.insert(i as u32);
        }

        assert!(!set.overflowed);
        assert!(set.contains(0));
        assert!(set.contains((MAX_SUMMARY_IDS - 1) as u32));
        assert!(!set.contains(100));

        // Duplicate insert shouldn't overflow
        set.insert(0);
        assert!(!set.overflowed);

        // Overflow
        set.insert(100);
        assert!(set.overflowed);

        // Once overflowed, contains doesn't guarantee false negatives, but implementation logic
        // says "overflowed" flag is just a flag. The set content is still valid for what it holds.
        // Wait, commit_matches checks: if !overflowed && !contains -> return false.
        // So if overflowed, it returns true (match).
        // The set itself still works as a set of what it holds.
        assert!(set.contains(0));
    }

    #[test]
    fn test_small_thing_set() {
        let mut set = SmallThingSet::default();

        for i in 0..MAX_SUMMARY_THINGS {
            set.insert(i as u64);
        }
        assert!(!set.overflowed);
        assert!(set.contains(0));

        set.insert(100);
        assert!(set.overflowed);
    }

    #[test]
    fn test_commit_history_ring() {
        // Create small history: max 3 commits, max 100 bytes
        let mut history = CommitHistory::new(3, 100);

        // Push 1
        history.push(1, vec![1, 2, 3], CommitSummary::default());
        assert_eq!(history.oldest_seq(), Some(1));
        assert_eq!(history.newest_seq(), Some(1));
        assert_eq!(history.len(), 1);
        assert!(history.contains(1));
        assert_eq!(history.get(1), Some(vec![1, 2, 3].as_slice()));

        // Push 2
        history.push(2, vec![4, 5], CommitSummary::default());
        assert_eq!(history.oldest_seq(), Some(1));
        assert_eq!(history.newest_seq(), Some(2));
        assert_eq!(history.len(), 2);

        // Push 3
        history.push(3, vec![6], CommitSummary::default());
        assert_eq!(history.oldest_seq(), Some(1));
        assert_eq!(history.newest_seq(), Some(3));
        assert_eq!(history.len(), 3);

        // Push 4 (Evicts 1)
        history.push(4, vec![7], CommitSummary::default());
        assert_eq!(history.oldest_seq(), Some(2));
        assert_eq!(history.newest_seq(), Some(4));
        assert_eq!(history.len(), 3);
        assert!(!history.contains(1));
        assert!(history.contains(2));
        assert_eq!(history.get(4), Some(vec![7].as_slice()));
    }

    #[test]
    fn test_commit_history_byte_limit() {
        // Max 10 commits, but max 10 bytes
        let mut history = CommitHistory::new(10, 10);

        // Push 5 bytes
        history.push(1, vec![1, 2, 3, 4, 5], CommitSummary::default());
        assert_eq!(history.len(), 1);

        // Push 6 bytes (Total 11 > 10, evicts first)
        history.push(2, vec![1, 2, 3, 4, 5, 6], CommitSummary::default());
        assert_eq!(history.len(), 1);
        assert_eq!(history.oldest_seq(), Some(2));

        // Push 11 bytes (Evicts everything, but stores this one? Or refuses?)
        // Code: while ... bytes + data_len > max_bytes { pop }
        // If data_len > max_bytes, it will pop everything and then push.
        // Then we have 1 item that exceeds limit?
        // Code:
        // while !empty && (len >= max || bytes + new > max) pop
        // push
        // So yes, it allows a single commit larger than max_bytes if it's the only one.
        history.push(3, vec![0; 11], CommitSummary::default());
        assert_eq!(history.len(), 1);
        assert_eq!(history.oldest_seq(), Some(3));
        assert_eq!(history.bytes, 11);
    }

    #[test]
    fn test_commit_matches_filtering() {
        let mut summary = CommitSummary::default();
        summary.kinds.insert(10);
        summary.predicates.insert(20);
        summary.subjects.insert(30);

        // 1. Match all (flags = 0)
        let filter_all = WatchFilter::default();
        assert!(commit_matches(&filter_all, &summary));

        // 2. Match Kind
        let mut filter_kind = WatchFilter::default();
        filter_kind.flags |= WATCH_F_KIND;
        filter_kind.kind_id = 10;
        assert!(commit_matches(&filter_kind, &summary));

        // Mismatch Kind
        filter_kind.kind_id = 11;
        assert!(!commit_matches(&filter_kind, &summary));

        // 3. Match Predicate
        let mut filter_pred = WatchFilter::default();
        filter_pred.flags |= WATCH_F_PREDICATE;
        filter_pred.predicate_id = 20;
        assert!(commit_matches(&filter_pred, &summary));

        // Mismatch Predicate
        filter_pred.predicate_id = 21;
        assert!(!commit_matches(&filter_pred, &summary));

        // 4. Match Subject
        let mut filter_subj = WatchFilter::default();
        filter_subj.flags |= WATCH_F_SUBJECT;
        filter_subj.subject_lo = 30;
        assert!(commit_matches(&filter_subj, &summary));

        // Mismatch Subject
        filter_subj.subject_lo = 31;
        assert!(!commit_matches(&filter_subj, &summary));

        // 5. Match AND logic (Kind AND Predicate)
        let mut filter_and = WatchFilter::default();
        filter_and.flags = WATCH_F_KIND | WATCH_F_PREDICATE;
        filter_and.kind_id = 10;
        filter_and.predicate_id = 20;
        assert!(commit_matches(&filter_and, &summary));

        // One matches, other doesn't
        filter_and.kind_id = 11; // Wrong kind
        assert!(!commit_matches(&filter_and, &summary));

        // 6. Overflow behavior
        let mut summary_overflow = CommitSummary::default();
        // Force overflow kinds
        summary_overflow.kinds.overflowed = true;

        let mut filter_overflow = WatchFilter::default();
        filter_overflow.flags = WATCH_F_KIND;
        filter_overflow.kind_id = 999; // Not in set (set is empty/irrelevant if overflowed)

        // Should match because overflowed set is conservative
        assert!(commit_matches(&filter_overflow, &summary_overflow));
    }

    #[test]
    fn test_graph_structure() {
        let mut graph = Graph::new();

        let kind_a = 1;
        let kind_b = 2;
        let rel_x = 10;

        // Alloc nodes
        let node1 = graph.alloc(kind_a);
        let node2 = graph.alloc(kind_b);

        // IDs should be distinct
        assert_ne!(node1, node2);

        // Kinds should be retrievable
        assert_eq!(graph.get_kind(node1), Some(kind_a));
        assert_eq!(graph.get_kind(node2), Some(kind_b));

        // Index check
        assert!(graph.kind_index.contains_key(&kind_a));
        assert!(graph.kind_index[&kind_a].contains(&node1));

        assert!(graph.kind_index.contains_key(&kind_b));
        assert!(graph.kind_index[&kind_b].contains(&node2));

        // Link
        graph.link(node1, rel_x, node2);

        // Verify edge
        let node1_ref = graph.nodes.get(&node1).expect("Node1 missing");
        assert_eq!(node1_ref.edges.len(), 1);
        assert_eq!(node1_ref.edges[0], (rel_x, node2));
    }

    #[test]
    fn test_commit_history_multi_eviction() {
        // Max 100 bytes, enough commits to not trigger count limit
        let mut history = CommitHistory::new(100, 100);

        // Fill with 3 commits of 30 bytes each (Total 90)
        history.push(1, vec![0; 30], CommitSummary::default());
        history.push(2, vec![0; 30], CommitSummary::default());
        history.push(3, vec![0; 30], CommitSummary::default());

        assert_eq!(history.len(), 3);
        assert_eq!(history.bytes, 90);
        assert_eq!(history.oldest_seq(), Some(1));

        // Push a larger commit (50 bytes).
        // 90 + 50 = 140 > 100.
        // Evict 1: 60 + 50 = 110 > 100.
        // Evict 2: 30 + 50 = 80 <= 100. Stop.
        // Result should be: [3, 4]
        history.push(4, vec![0; 50], CommitSummary::default());

        assert_eq!(history.len(), 2);
        assert_eq!(history.oldest_seq(), Some(3));
        assert_eq!(history.newest_seq(), Some(4));
        assert_eq!(history.bytes, 80); // 30 + 50

        assert!(!history.contains(1));
        assert!(!history.contains(2));
        assert!(history.contains(3));
        assert!(history.contains(4));
    }

    #[test]
    fn test_ownership_tracking() {
        let mut graph = Graph::new();
        
        // Create some nodes with different owners
        let owner1_id = 1000;
        let owner2_id = 2000;
        
        let thing1 = graph.alloc_with_owner(100, Some(owner1_id));
        let thing2 = graph.alloc_with_owner(101, Some(owner1_id));
        let thing3 = graph.alloc_with_owner(102, Some(owner2_id));
        let thing4 = graph.alloc_with_owner(103, None); // kernel-owned
        
        // Check ownership
        assert_eq!(graph.nodes.get(&thing1).unwrap().owner, Some(owner1_id));
        assert_eq!(graph.nodes.get(&thing2).unwrap().owner, Some(owner1_id));
        assert_eq!(graph.nodes.get(&thing3).unwrap().owner, Some(owner2_id));
        assert_eq!(graph.nodes.get(&thing4).unwrap().owner, None);
        
        // Get owned things
        let owned_by_1 = graph.get_owned_things(owner1_id);
        assert_eq!(owned_by_1.len(), 2);
        assert!(owned_by_1.contains(&thing1));
        assert!(owned_by_1.contains(&thing2));
        
        let owned_by_2 = graph.get_owned_things(owner2_id);
        assert_eq!(owned_by_2.len(), 1);
        assert!(owned_by_2.contains(&thing3));
        
        // Test orphaning
        assert!(graph.orphan_thing(thing1));
        assert_eq!(graph.nodes.get(&thing1).unwrap().owner, None);
        
        let owned_by_1_after = graph.get_owned_things(owner1_id);
        assert_eq!(owned_by_1_after.len(), 1);
        assert!(!owned_by_1_after.contains(&thing1));
        
        // Test set_owner
        assert!(graph.set_owner(thing1, Some(owner2_id)));
        assert_eq!(graph.nodes.get(&thing1).unwrap().owner, Some(owner2_id));
    }
}
