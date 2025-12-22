use crate::graph;
use crate::symbols;
use alloc::string::String;
use alloc::vec::Vec;
use thing_models::{PropType, PropValue};

pub fn init() {
    // Register schema for ConsoleLog
    // Kind: ConsoleLog
    // Props: message (Str), timestamp (U64)

    let kind_log = symbols::intern("ConsoleLog");
    let desc_log = symbols::intern("A log message from the kernel or a process");
    let prop_msg = symbols::intern("message");
    let prop_ts = symbols::intern("timestamp");

    let props = alloc::vec![(prop_msg, PropType::Str), (prop_ts, PropType::U64),];

    let _ = graph::schema::register_schema(kind_log, desc_log, props, alloc::vec![]);
}

pub fn push_log(message: &str) {
    let kind_log = symbols::intern("ConsoleLog");
    let prop_msg = symbols::intern("message");
    let prop_ts = symbols::intern("timestamp");

    let now = crate::time::monotonic_now_ns();

    let props = alloc::vec![
        (prop_msg, PropValue::Str(String::from(message))),
        (prop_ts, PropValue::U64(now)),
    ];

    graph::create_thing(kind_log, props);
}
