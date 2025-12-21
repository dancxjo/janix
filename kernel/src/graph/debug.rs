use crate::graph::store;
use crate::graph_kinds;
use crate::symbols;
use abi::{Predicate, ThingId, syscall_defs::SymbolId};
use alloc::format;
use thing_models::PropValue;

fn resolve_predicate(pred: Predicate) -> alloc::string::String {
    match pred {
        graph_kinds::LINK_OWNS_THREAD => "OWNS_THREAD".into(),
        graph_kinds::LINK_RUNS_ON => "RUNS_ON".into(),
        graph_kinds::LINK_SLEEPS_UNTIL => "SLEEPS_UNTIL".into(),
        graph_kinds::LINK_LAUNCHES => "LAUNCHES".into(),
        graph_kinds::LINK_SPAWNED => "SPAWNED".into(),
        graph_kinds::LINK_DISPLAY_SCANOUT => "DISPLAY_SCANOUT".into(),
        graph_kinds::LINK_DISPLAY_FRONT_BUFFER => "DISPLAY_FRONT_BUFFER".into(),
        graph_kinds::LINK_DISPLAY_BACK_BUFFER => "DISPLAY_BACK_BUFFER".into(),
        graph_kinds::LINK_MODE_PLACE => "MODE_PLACE".into(),
        graph_kinds::LINK_MODE_HAS_WINDOW => "MODE_HAS_WINDOW".into(),
        graph_kinds::LINK_WINDOW_SURFACE => "WINDOW_SURFACE".into(),
        graph_kinds::LINK_WINDOW_HAS_SURFACE => "WINDOW_HAS_SURFACE".into(),
        graph_kinds::LINK_PLACE_WINDOW => "PLACE_WINDOW".into(),
        graph_kinds::LINK_ACTIVE_MODE => "ACTIVE_MODE".into(),
        graph_kinds::LINK_APP_OWNS_WINDOW => "APP_OWNS_WINDOW".into(),
        graph_kinds::LINK_DISPLAY_HAS_FRONT_BUFFER => "DISPLAY_HAS_FRONT_BUFFER".into(),
        graph_kinds::LINK_DISPLAY_HAS_BACK_BUFFER => "DISPLAY_HAS_BACK_BUFFER".into(),
        graph_kinds::LINK_HAS_ACTIVE_MODE => "HAS_ACTIVE_MODE".into(),
        graph_kinds::LINK_ABOUT => "ABOUT".into(),
        graph_kinds::LINK_RUNNING => "RUNNING".into(),
        graph_kinds::LINK_RESPAWNED_FROM => "RESPAWNED_FROM".into(),
        _ => format!("PRED_{:x}", pred.0),
    }
}

fn fmt_prop_value(val: &PropValue) -> alloc::string::String {
    match val {
        PropValue::U64(v) => format!("{}", v),
        PropValue::I64(v) => format!("{}", v),
        PropValue::Bool(v) => format!("{}", v),
        PropValue::Str(s) => format!("{:?}", s),
        PropValue::Blob(b) => format!("<blob len={}>", b.len()),
        PropValue::Symbol(id) => format!("Symbol({})", id.0),
    }
}

pub fn dump_graph_table() {
    #[cfg(not(test))]
    {
        crate::log("Graph Dump:");
        crate::graph::iter_things(|node| {
            let kind_str = symbols::resolve(node.kind).unwrap_or_else(|| "???".into());
            let mut props_str = alloc::string::String::new();
            // Determine if we need comma
            let mut first = true;
            for (key_id, val) in &node.props {
                if !first {
                    props_str.push_str(", ");
                }
                let key_str = symbols::resolve(*key_id).unwrap_or_else(|| "???".into());
                let val_str = fmt_prop_value(val);
                props_str.push_str(&format!("{}: {}", key_str, val_str));
                first = false;
            }
            let msg = format!("(t{}:{} {{ {} }})", node.id.0, kind_str, props_str);
            crate::log(&msg);
        });
    }
}

pub fn print_thing_created(id: ThingId, kind: SymbolId, props: &[(SymbolId, PropValue)]) {
    let kind_str = symbols::resolve(kind).unwrap_or_else(|| "???".into());
    let mut props_str = alloc::string::String::new();
    let mut first = true;
    for (key_id, val) in props {
        if !first {
            props_str.push_str(", ");
        }
        let key_str = symbols::resolve(*key_id).unwrap_or_else(|| "???".into());
        let val_str = fmt_prop_value(val);
        props_str.push_str(&format!("{}: {}", key_str, val_str));
        first = false;
    }
    // format: (t1234:Thorton { kay: 'vel', boo: 123 })
    let msg = format!("(t{}:{} {{ {} }})", id.0, kind_str, props_str);
    crate::log(&msg);
}

pub fn print_thing_updated(id: ThingId, props: &[(SymbolId, PropValue)]) {
    let mut props_str = alloc::string::String::new();
    let mut first = true;
    for (key_id, val) in props {
        if !first {
            props_str.push_str(", ");
        }
        let key_str = symbols::resolve(*key_id).unwrap_or_else(|| "???".into());
        let val_str = fmt_prop_value(val);
        props_str.push_str(&format!("{}: {}", key_str, val_str));
        first = false;
    }
    let msg = format!("(t{}) updated {{ {} }}", id.0, props_str);
    crate::log(&msg);
}

pub fn print_link_created(src: ThingId, pred: Predicate, dst: ThingId) {
    // format: (t1234)-[:RESOLVED_NAME]->(x8549)
    let pred_str = resolve_predicate(pred);
    let msg = format!("(t{})-[:{}]->(t{})", src.0, pred_str, dst.0);
    crate::log(&msg);
}

pub fn print_link_removed(src: ThingId, pred: Predicate, dst: ThingId) {
    let pred_str = resolve_predicate(pred);
    let msg = format!("(t{})-[:{}]-/->(t{})", src.0, pred_str, dst.0);
    crate::log(&msg);
}
