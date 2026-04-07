//! Logging event handler.

use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use crate::root::{LogProvenance, SymbolShell};
use abi::symbols::SymbolId;
use alloc::collections::VecDeque;

use super::HandlerResult;
use super::graph::resolve_shell;

/// Maximum number of log entries to keep in the graph
const MAX_LOG_ENTRIES: usize = 256;

pub struct LogSymbols {
    pub log_entry: SymbolId,
    pub level: SymbolId,
    pub line: SymbolId,
    pub timestamp: SymbolId,
    pub message: SymbolId,
    pub event: SymbolId,
    pub about: SymbolId,
    pub file: SymbolId,
    pub module: SymbolId,
    pub tid: SymbolId,
    /// Ring buffer of recent log entry ThingIds for eviction
    pub recent_entries: VecDeque<u64>,
}

impl LogSymbols {
    pub fn new(interner: &mut Interner) -> Self {
        Self {
            log_entry: interner.intern("log.Entry"),
            level: interner.intern("level"),
            line: interner.intern("line"),
            timestamp: interner.intern("timestamp"),
            message: interner.intern("message"),
            event: interner.intern("event"),
            about: interner.intern("ABOUT"),
            file: interner.intern("file"),
            module: interner.intern("module"),
            tid: interner.intern("tid"),
            recent_entries: VecDeque::with_capacity(MAX_LOG_ENTRIES),
        }
    }
}

pub fn handle_log_event(
    graph: &mut Graph,
    interner: &mut Interner,
    log_symbols: &mut LogSymbols,
    level: u8,
    event: SymbolShell,
    message: &str,
    timestamp: u64,
    provenance: &LogProvenance,
    fields: &[(SymbolShell, u64)],
    about: &[u64],
) -> HandlerResult {
    // Evict oldest log entries if at capacity
    while log_symbols.recent_entries.len() >= MAX_LOG_ENTRIES {
        if let Some(old_id) = log_symbols.recent_entries.pop_front() {
            graph.remove_node(old_id);
        }
    }

    let entry_id = graph.alloc(log_symbols.log_entry);
    log_symbols.recent_entries.push_back(entry_id);

    let event_id = resolve_shell(event, interner);
    let msg_id = interner.intern(message);

    if let Some(node) = graph.get_node_mut(entry_id) {
        node.props.insert(log_symbols.level, level as u64);
        node.props.insert(log_symbols.line, provenance.line as u64);
        node.props.insert(log_symbols.timestamp, timestamp);
        node.props.insert(log_symbols.message, msg_id as u64);
        node.props.insert(log_symbols.event, event_id as u64);

        for (key_shell, val) in fields {
            let kid = resolve_shell(key_shell.clone(), interner);
            node.props.insert(kid, *val);
        }
    }

    for subject_id in about {
        graph.link(entry_id, log_symbols.about, *subject_id);
    }

    if let Some(node) = graph.get_node_mut(entry_id) {
        let f_id = interner.intern(provenance.file);
        let m_id = interner.intern(provenance.module);
        node.props.insert(log_symbols.file, f_id as u64);
        node.props.insert(log_symbols.module, m_id as u64);
        node.props.insert(log_symbols.tid, provenance.tid);
    }

    (0, entry_id)
}
