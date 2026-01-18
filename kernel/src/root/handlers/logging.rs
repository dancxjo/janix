//! Logging event handler.

use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use crate::root::{LogProvenance, SymbolShell};
use abi::symbols::SymbolId;
use abi::wire::ThingId;

use super::graph::resolve_shell;
use super::HandlerResult;

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
        }
    }
}

pub fn handle_log_event(
    graph: &mut Graph,
    interner: &mut Interner,
    log_symbols: &LogSymbols,
    level: u8,
    event: SymbolShell,
    message: &str,
    timestamp: u64,
    provenance: &LogProvenance,
    fields: &[(SymbolShell, [u8; 16])],
    about: &[ThingId],
) -> HandlerResult {
    let entry_id = graph.alloc(log_symbols.log_entry);

    let event_id = resolve_shell(event, interner);
    let msg_id = interner.intern(message);

    if let Some(node) = graph.get_node_mut(entry_id) {
        let mut val_buf = [0u8; 16];

        val_buf[0..8].copy_from_slice(&(level as u64).to_le_bytes());
        node.props.insert(log_symbols.level, val_buf);

        val_buf[0..8].copy_from_slice(&(provenance.line as u64).to_le_bytes());
        node.props.insert(log_symbols.line, val_buf);

        val_buf[0..8].copy_from_slice(&timestamp.to_le_bytes());
        node.props.insert(log_symbols.timestamp, val_buf);

        node.props.insert(log_symbols.message, msg_id.0);
        node.props.insert(log_symbols.event, event_id.0);

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

        node.props.insert(log_symbols.file, f_id.0);
        node.props.insert(log_symbols.module, m_id.0);

        let mut val_buf = [0u8; 16];
        val_buf[0..8].copy_from_slice(&provenance.tid.to_le_bytes());
        node.props.insert(log_symbols.tid, val_buf);
    }

    (0, 0)
}
