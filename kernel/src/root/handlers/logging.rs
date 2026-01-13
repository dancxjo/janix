//! Logging event handler.

use crate::root::graph::Graph;
use crate::root::symbols::Interner;
use crate::root::{LogProvenance, SymbolShell};

use super::graph::resolve_shell;
use super::HandlerResult;

pub fn handle_log_event(
    graph: &mut Graph,
    interner: &mut Interner,
    level: u8,
    event: SymbolShell,
    message: &str,
    timestamp: u64,
    provenance: &LogProvenance,
    fields: &[(SymbolShell, u64)],
    about: &[u64],
) -> HandlerResult {
    let kind_id = interner.intern("log.Entry");
    let entry_id = graph.alloc(kind_id);

    let p_level = interner.intern("level");
    let p_line = interner.intern("line");
    let p_ts = interner.intern("timestamp");
    let p_msg = interner.intern("message");
    let p_evt = interner.intern("event");

    let event_id = resolve_shell(event, interner);
    let msg_id = interner.intern(message);

    if let Some(node) = graph.get_node_mut(entry_id) {
        node.props.insert(p_level, level as u64);
        node.props.insert(p_line, provenance.line as u64);
        node.props.insert(p_ts, timestamp);
        node.props.insert(p_msg, msg_id as u64);
        node.props.insert(p_evt, event_id as u64);

        for (key_shell, val) in fields {
            let kid = resolve_shell(key_shell.clone(), interner);
            node.props.insert(kid, *val);
        }
    }

    let r_about = interner.intern("ABOUT");
    for subject_id in about {
        graph.link(entry_id, r_about, *subject_id);
    }

    let p_file = interner.intern("file");
    let p_module = interner.intern("module");
    let p_tid = interner.intern("tid");

    if let Some(node) = graph.get_node_mut(entry_id) {
        let f_id = interner.intern(&provenance.file);
        let m_id = interner.intern(&provenance.module);
        node.props.insert(p_file, f_id as u64);
        node.props.insert(p_module, m_id as u64);
        node.props.insert(p_tid, provenance.tid);
    }

    (0, entry_id)
}
