extern crate alloc;

use crate::graph::store::iter_things;
use crate::graph_kinds;
use abi::{Predicate, PropKey, PropValue, ThingId, Link};
use alloc::string::String;

/// Dump the entire graph to the kernel console as a simple table of Things
/// and Links. The output uses compact single-line forms for Things and Links.
///
/// Example:
/// 1023 = (:IoPort { x: 'ad' })
/// (1023)=[:IS_A_BARF {dl: '123' }]=>(321).
pub fn dump_graph_table() {
    crate::console::print("Graph dump:\n");

    iter_things(|thing| {
        if thing.kind == graph_kinds::KIND_LINK {
            print_link(thing.id, thing.kind, &thing.props);
        } else {
            print_thing(thing.id, thing.kind, &thing.props);
        }
    });

    // Now print a concise list of links (relationships) using the link index.
    crate::console::print("Links:\n");
    crate::graph::index_links::for_each_link(|link| {
        // Try to fetch any extra props stored on the link Thing
        let mut extra = String::new();
        // Try to fetch any extra props stored on the link Thing
        crate::graph::with_thing(link.id, |thing| {
            let mut first = true;
            for prop in thing.props.iter().flatten() {
                let (k, v) = prop;
                // skip canonical link props
                if *k == graph_kinds::PROP_LINK_SRC
                    || *k == graph_kinds::PROP_LINK_DST
                    || *k == graph_kinds::PROP_LINK_PRED
                {
                    continue;
                }
                if !first {
                    extra.push_str(", ");
                }
                first = false;
                match v {
                    PropValue::U64(n) => extra.push_str(&alloc::format!("{}: {}", k, n)),
                    PropValue::I64(n) => extra.push_str(&alloc::format!("{}: {}", k, n)),
                    PropValue::Bool(b) => extra.push_str(&alloc::format!("{}: {}", k, b)),
                    PropValue::Str(s) => extra.push_str(&alloc::format!("{}: '{}'", k, s)),
                }
            }
        });

        // Print link in canonical form: (src)-[:PRED {props}]->(dst)
        let pred_sym = match link.pred.0 {
            x if x == graph_kinds::LINK_OWNS_THREAD.0 => alloc::format!("{}", "OWNS_THREAD"),
            x if x == graph_kinds::LINK_RUNS_ON.0 => alloc::format!("{}", "RUNS_ON"),
            x if x == graph_kinds::LINK_SLEEPS_UNTIL.0 => alloc::format!("{}", "SLEEPS_UNTIL"),
            x if x == graph_kinds::LINK_LAUNCHES.0 => alloc::format!("{}", "LAUNCHES"),
            x if x == graph_kinds::LINK_SPAWNED.0 => alloc::format!("{}", "SPAWNED"),
            _ => alloc::format!("0x{:x}", link.pred.0),
        };

        let s = if extra.is_empty() {
            alloc::format!("({})-[:{}]->({})\n", link.src.0, pred_sym, link.dst.0)
        } else {
            alloc::format!(
                "({})-[:{} {{ {} }}]->({})\n",
                link.src.0,
                pred_sym,
                extra,
                link.dst.0
            )
        };
        crate::console::print(&s);
    });
}

pub fn print_thing(id: ThingId, kind: &str, props: &[Option<(&str, PropValue)>]) {
    let mut s: String = alloc::format!("{}:{} {{ ", id.0, kind);

    let mut first = true;
    for prop in props.iter().flatten() {
        let (k, v) = prop;
        if !first {
            s.push_str(", ");
        }
        first = false;
        match v {
            PropValue::U64(n) => s.push_str(&alloc::format!("{}: {}", k, n)),
            PropValue::I64(n) => s.push_str(&alloc::format!("{}: {}", k, n)),
            PropValue::Bool(b) => s.push_str(&alloc::format!("{}: {}", k, b)),
            PropValue::Str(st) => s.push_str(&alloc::format!("{}: \"{}\"", k, st)),
        }
    }

    s.push_str(" }\n");
    crate::console::print(&s);
}

pub fn print_link(_id: ThingId, _kind: &str, props: &[Option<(&str, PropValue)>]) {
    // Extract canonical link parts from properties
    let mut src: Option<ThingId> = None;
    let mut dst: Option<ThingId> = None;
    let mut pred: Option<Predicate> = None;

    for prop in props.iter().flatten() {
        let (k, v) = prop;
        match *k {
            graph_kinds::PROP_LINK_SRC => {
                if let PropValue::U64(n) = v {
                    src = Some(ThingId(*n));
                }
            }
            graph_kinds::PROP_LINK_DST => {
                if let PropValue::U64(n) = v {
                    dst = Some(ThingId(*n));
                }
            }
            graph_kinds::PROP_LINK_PRED => {
                if let PropValue::U64(n) = v {
                    pred = Some(Predicate(*n));
                }
            }
            _ => {}
        }
    }

    // Fallback if some parts missing
    let src_id = src.unwrap_or_else(|| ThingId(0));
    let dst_id = dst.unwrap_or_else(|| ThingId(0));

    let src_kind = crate::graph::with_thing(src_id, |t| t.kind).unwrap_or("?");
    let dst_kind = crate::graph::with_thing(dst_id, |t| t.kind).unwrap_or("?");

    let mut s = alloc::format!("({}:{})", src_id.0, src_kind);

    if let Some(p) = pred {
        let sym_opt: Option<&str> = match p.0 {
            x if x == graph_kinds::LINK_OWNS_THREAD.0 => Some("OWNS_THREAD"),
            x if x == graph_kinds::LINK_RUNS_ON.0 => Some("RUNS_ON"),
            x if x == graph_kinds::LINK_SLEEPS_UNTIL.0 => Some("SLEEPS_UNTIL"),
            x if x == graph_kinds::LINK_LAUNCHES.0 => Some("LAUNCHES"),
            x if x == graph_kinds::LINK_SPAWNED.0 => Some("SPAWNED"),
            _ => None,
        };
        
        s.push_str(" -[:");
        if let Some(sym) = sym_opt {
            s.push_str(sym);
        } else {
            s.push_str(&alloc::format!("0x{:x}", p.0));
        }
    } else {
        s.push_str(" -[:?");
    }

    s.push_str("]-> ");
    s.push_str(&alloc::format!("({}:{})\n", dst_id.0, dst_kind));
    
    crate::console::print(&s);
}

pub fn print_thing_created(id: ThingId, kind: &str, props: &[(PropKey, PropValue)]) {
    let mut s: String = alloc::format!("{}:{} {{ ", id.0, kind);

    let mut first = true;
    for (prop_key, v) in props {
        if !first {
            s.push_str(", ");
        }
        first = false;
        match v {
            PropValue::U64(n) => s.push_str(&alloc::format!("{}: {}", prop_key, n)),
            PropValue::I64(n) => s.push_str(&alloc::format!("{}: {}", prop_key, n)),
            PropValue::Bool(b) => s.push_str(&alloc::format!("{}: {}", prop_key, b)),
            PropValue::Str(st) => s.push_str(&alloc::format!("{}: \"{}\"", prop_key, st)),
        }
    }

    s.push_str(" }\n");
    crate::console::print(&s);
}

pub fn print_link_created(link: &Link) {
    let src_kind = crate::graph::with_thing(link.src, |t| t.kind).unwrap_or("?");
    let dst_kind = crate::graph::with_thing(link.dst, |t| t.kind).unwrap_or("?");

    let mut s = alloc::format!("({}:{})", link.src.0, src_kind);

    let sym_opt: Option<&str> = match link.pred.0 {
        x if x == graph_kinds::LINK_OWNS_THREAD.0 => Some("OWNS_THREAD"),
        x if x == graph_kinds::LINK_RUNS_ON.0 => Some("RUNS_ON"),
        x if x == graph_kinds::LINK_SLEEPS_UNTIL.0 => Some("SLEEPS_UNTIL"),
        x if x == graph_kinds::LINK_LAUNCHES.0 => Some("LAUNCHES"),
        x if x == graph_kinds::LINK_SPAWNED.0 => Some("SPAWNED"),
        _ => None,
    };
    
    s.push_str(" -[:");
    if let Some(sym) = sym_opt {
        s.push_str(sym);
    } else {
        s.push_str(&alloc::format!("0x{:x}", link.pred.0));
    }

    s.push_str("]-> ");
    s.push_str(&alloc::format!("({}:{})\n", link.dst.0, dst_kind));
    
    crate::console::print(&s);
}
