use bridge_hosted::{HostedBridge, FileSymbolStore};
use kernel_core::Kernel;
use kernel_core::symbols::store::SymbolStore;
use std::path::PathBuf;
use std::env;

fn main() {
    let bridge = HostedBridge;
    
    // Determine symbol store path
    let store_path = env::var("THINGOS_SYMBOLS_PATH").map(PathBuf::from).unwrap_or_else(|_| {
        let mut p = PathBuf::from("target/thingos/symbols.tsym");
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create symbol dir");
        }
        p
    });

    // Create store
    let mut store = FileSymbolStore::new(store_path);

    let mut kernel = Kernel::new(bridge);
    
    // Smoke Test Hook
    if let Ok(action) = env::var("THINGOS_SMOKE_ACTION") {
        use abi::wire::graph::{GraphOp, GraphReply};
        use kernel_core::syscalls::graph::handle_graph_op;
        
        // Ensure symbols loaded
        if let Err(_) = kernel.symbols.load_from_store(&mut store) {
            // ignore load error
        }

        match action.as_str() {
            "intern" => {
                let val = env::var("THINGOS_SMOKE_VAL").expect("Need VAL");
                let op = GraphOp::SymbolIntern { text: &val };
                if let GraphReply::SymbolInterned { id } = handle_graph_op(&mut kernel, abi::ProcessId(0), op) {
                    println!("ID: {}", id.0);
                    // Persist
                    kernel.symbols.persist_to_store(&mut store).expect("Save failed");
                } else {
                    panic!("Intern failed");
                }
            },
            "resolve" => {
                let id_s = env::var("THINGOS_SMOKE_ID").expect("Need ID");
                let id = abi::SymbolId(id_s.parse().expect("Invalid ID"));
                let op = GraphOp::SymbolResolve { id };
                if let GraphReply::SymbolResolved { text } = handle_graph_op(&mut kernel, abi::ProcessId(0), op) {
                    println!("VAL: {}", text);
                } else {
                    panic!("Resolve failed");
                }
            },
            _ => panic!("Unknown action"),
        }
        return;
    }

    // Pass store to boot
    kernel.boot(Some(&mut store));
}
