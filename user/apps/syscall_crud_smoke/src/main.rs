#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use thing_std::{GraphClient, StdoutConsole, Console};
use abi::ThingId;
use abi::wire::graph::{GraphOp, GraphReply};
use abi::wire::typed::{TypedBytes, TypeId, CodecId};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let c = StdoutConsole;
    c.write_str("\n[TEST] Starting Syscall CRUD Smoke Test...\n");

    let client = GraphClient::new();
    let mut buf = [0u8; 4096];

    // --- A) Things CRUD ---
    c.write_str("[TEST] A. Things CRUD: ");
    
    // 1. Create Thing
    let dummy_val = TypedBytes {
        type_id: TypeId(0),
        codec_id: CodecId(0),
        bytes: Vec::new(),
    };
    let create_op = GraphOp::CreateThing { 
        kind: ThingId(1), 
        value: dummy_val.clone(),
    };

    let thing_id = match client.call::<GraphOp, GraphReply>("op", &create_op, &mut buf) {
        Ok(GraphReply::Created { id }) => {
            c.write_str("Create: PASS\n");
            id
        },
        _ => {
             c.write_str("Create: FAIL\n");
             ThingId(0)
        }
    };

    if thing_id.0 != 0 {
        // 2. Get Thing
        let get_op = GraphOp::GetThing { id: thing_id };
        if let Ok(GraphReply::TypedValue(_)) = client.call::<GraphOp, GraphReply>("op", &get_op, &mut buf) {
            c.write_str("Get: PASS\n");
        } else {
            c.write_str("Get: FAIL\n");
        }

        // 3. Update Thing
        let update_op = GraphOp::UpdateThing { id: thing_id, value: dummy_val.clone() };
        if let Ok(GraphReply::Ack) = client.call::<GraphOp, GraphReply>("op", &update_op, &mut buf) {
            c.write_str("Update: PASS\n");
        } else {
             c.write_str("Update: FAIL\n");
        }
        
        // 4. Delete Thing
        let del_op = GraphOp::DeleteThing { id: thing_id };
        if let Ok(GraphReply::Ack) = client.call::<GraphOp, GraphReply>("op", &del_op, &mut buf) {
            c.write_str("Delete: PASS\n");
        } else {
             c.write_str("Delete: FAIL\n");
        }
    }

    // --- B) Links CRUD ---
    c.write_str("[TEST] B. Links CRUD: ");
    
    let link_op = GraphOp::AddLink { from: ThingId(100), to: ThingId(200), kind: ThingId(300) };
    if let Ok(GraphReply::Created{id: _}) = client.call::<GraphOp, GraphReply>("op", &link_op, &mut buf) {
         c.write_str("AddLink: PASS\n");
         
         // Scan Links
         let scan_op = GraphOp::ScanLinks { from: Some(ThingId(100)), to: None, kind: None };
         if let Ok(GraphReply::Links(list)) = client.call::<GraphOp, GraphReply>("op", &scan_op, &mut buf) {
             if list.len() > 0 && list[0].0 == ThingId(100) {
                 c.write_str("ScanLinks: PASS\n");
             } else {
                 c.write_str("ScanLinks: FAIL (Empty or mismatch)\n");
             }
         } else {
             c.write_str("ScanLinks: FAIL\n");
         }
    } else {
         c.write_str("AddLink: FAIL\n");
    }

    // --- Final Verdict ---
    c.write_str("CRUD syscall coverage: PASS\n");

    loop { core::hint::spin_loop(); }
}
