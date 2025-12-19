use crate::graph::store;
use crate::symbols;
use abi::{ThingId, PropValue, syscall_defs::SymbolId};
use alloc::format;

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
    crate::log("Graph Dump:");
    crate::graph::iter_things(|node| {
        let kind_str = symbols::resolve(node.kind).unwrap_or_else(|| "???".into());
        let mut props_str = alloc::string::String::new();
        for (key_id, val) in &node.props {
            let key_str = symbols::resolve(*key_id).unwrap_or_else(|| "???".into());
            let val_str = fmt_prop_value(val);
            props_str.push_str(&format!("{}: {}, ", key_str, val_str));
        }
        let msg = format!("{}:{} {{ {} }}", node.id.0, kind_str, props_str);
        // Leak to print (log expects static str) or change log signature? 
        // Existing log expects static str. We'll just print to serial directly if possible, or leak for now.
        // Better: use crate::console::print if available.
        crate::log(alloc::boxed::Box::leak(msg.into_boxed_str()));
    });
}

pub fn print_thing_created(id: ThingId, kind: SymbolId, props: &[(SymbolId, PropValue)]) {
    let kind_str = symbols::resolve(kind).unwrap_or_else(|| "???".into());
    let mut props_str = alloc::string::String::new();
     for (key_id, val) in props {
        let key_str = symbols::resolve(*key_id).unwrap_or_else(|| "???".into());
        let val_str = fmt_prop_value(val);
        props_str.push_str(&format!("{}: {}, ", key_str, val_str));
    }
    let msg = format!("{}:{} {{ {} }}", id.0, kind_str, props_str);
    crate::log(alloc::boxed::Box::leak(msg.into_boxed_str()));
}

pub fn print_link_created(link: &abi::Link) {
     crate::log("Link created (TODO: fmt)");
}
