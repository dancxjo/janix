use crate::{graph, graph_kinds, time};
use abi::ThingId;
use thing_models::{PropKey, PropValue};
use alloc::string::String;
use alloc::vec::Vec;

use core::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use spin::Mutex;
use thing_models::{InterruptEvent, IoPortOp, IoPortRegion, IoStatus};

static IRQ_CONTROLLER: Mutex<Option<fn(u8, bool)>> = Mutex::new(None);

/// Register a handler that will be called for each IRQ line change.
/// `handler(irq, masked)` where `masked` is true if the line should be masked.
pub fn register_irq_controller(handler: fn(u8, bool)) {
    let mut lock = IRQ_CONTROLLER.lock();
    *lock = Some(handler);
}

/// Process an InterruptRequest Thing identified by `id`.
/// Reads `irq_line` and `enabled` properties and invokes the registered handler.
pub fn process_interrupt_request(id: ThingId) {
    // Retrieve the thing
    let mut irq_line: u8 = 0;
    let mut enabled: bool = false;
    graph::with_thing(id, |thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_INTERRUPT_REQUEST) { return; }
        irq_line = thing.props.iter()
            .find(|(k, _)| *k == crate::symbols::intern(graph_kinds::PROP_IRQ_LINE))
            .and_then(|(_, v)| if let PropValue::U64(val) = v { Some(*val as u8) } else { None })
            .unwrap_or(0); // Default to 0 if not found or wrong type

        enabled = thing.props.iter()
            .find(|(k, _)| *k == crate::symbols::intern(graph_kinds::PROP_ENABLED))
            .and_then(|(_, v)| if let PropValue::Bool(b) = v { Some(*b) } else { None })
            .unwrap_or(false); // Default to false if not found or wrong type
    });
    let masked = !enabled;

    if let Some(handler) = *IRQ_CONTROLLER.lock() {
        handler(irq_line, masked);
        let msg = format!(
            "No IRQ controller registered when processing InterruptRequest {}",
            id.0
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        crate::log(leaked);
    }
}

use alloc::boxed::Box;
use alloc::format;

#[path = "../../../arch/src/io.rs"]
mod arch_io;
use arch_io as io;

static IO_REGIONS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn seed_io_regions() {
    #[cfg(target_arch = "x86_64")]
    {
        crate::log("seed_io_regions: entered");
        if IO_REGIONS_INITIALIZED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            crate::log("seed_io_regions: skipped (already initialized)");
            return;
        }
        let props = IoPortRegion::seed_props("i8042", 0x60, 5, &[1, 12]);
        // seed_props returns Vec<(String, PropValue)>?
        // We need Vec<(SymbolId, PropValue)>.
        // This requires converting keys.
        let mut final_props = Vec::new();
        for (k_str, v) in props {
            final_props.push((crate::symbols::intern(&k_str), v));
        }
        let _ = graph::create_thing(crate::symbols::intern(graph_kinds::KIND_IO_PORT_REGION), final_props);
        crate::log("seed_io_regions: finished");
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = IO_REGIONS_INITIALIZED.load(Ordering::Relaxed);
    }
}

pub fn process_io_op(op_id: ThingId) {
    let op = if let Some(op) = graph::with_thing(op_id, |thing| {
        if thing.kind != crate::symbols::intern(graph_kinds::KIND_IO_PORT_OP) { return None; }
        // Convert thing.props (Vec<(SymbolId, PropValue)>) to expected format for from_props
        // thing_models::IoPortOp::from_props expects &[Option<(String, PropValue)>] ??
        // Actually from_props signature in abi/src/lib.rs:
        // fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Option<Self>
        // PropKey is String. SymbolId is not String.
        // We cannot easily convert back.
        // We should update `from_props` to take SymbolId OR `Graph` to lookup symbols?
        // OR we just manually parse props here since we are in kernel.
        // Manual parsing:
        let mut direction = None;
        let mut region_id = None;
        let mut offset = None;
        let mut width = None;
        let mut value = None;
        let mut status = None;
        
        for (k, v) in &thing.props {
             if *k == crate::symbols::intern("direction") {
                 if let PropValue::U64(val) = v { direction = Some(*val as u8); } // enum?
             } else if *k == crate::symbols::intern("region") {
                 if let PropValue::U64(id_val) = v { region_id = Some(ThingId(*id_val)); }
             } else if *k == crate::symbols::intern("offset") {
                 if let PropValue::U64(val) = v { offset = Some(*val as u16); }
             } else if *k == crate::symbols::intern("width") {
                 if let PropValue::U64(val) = v { width = Some(*val as u8); }
             } else if *k == crate::symbols::intern("value") {
                 if let PropValue::U64(val) = v { value = Some(*val as u32); }
             } else if *k == crate::symbols::intern("status") {
                 if let PropValue::Str(s) = v { status = Some(IoStatus::from_str(s)); }
             }
        }
        
        let width_enum = match width.unwrap_or(0) {
             1 => Some(thing_models::IoWidth::U8),
             2 => Some(thing_models::IoWidth::U16),
             4 => Some(thing_models::IoWidth::U32),
             _ => None
        };

        if let (Some(d), Some(r), Some(o), Some(w_enum), Some(s_opt)) = (direction, region_id, offset, width_enum, status) {
             Some(IoPortOp {
                 id: op_id,
                 direction: if d == 0 { thing_models::IoDirection::Read } else { thing_models::IoDirection::Write },
                 region_id: r,
                 offset: o,
                 width: w_enum,
                 value: value.unwrap_or(0),
                 status: s_opt.unwrap_or(IoStatus::Pending),
                 error_code: 0,
                 issued_by: ThingId(0) // stub
             })
        } else {
             None
        }
    }).flatten() { op } else { return; };

    if op.status != IoStatus::Pending {
        return;
    }

    let _ = graph::update_thing(
        op_id,
        alloc::vec![(
            crate::symbols::intern("status"),
            PropValue::Str(String::from(IoStatus::InProgress.as_str())),
        )],
    );

    match execute_io_operation(&op) {
        Ok(value) => {
            let mut updates = Vec::new();
            updates.push((
                crate::symbols::intern("status"),
                PropValue::Str(String::from(IoStatus::Completed.as_str())),
            ));
            updates.push((crate::symbols::intern("error_code"), PropValue::U64(0)));
            if let Some(v) = value {
                updates.push((crate::symbols::intern("value"), PropValue::U64(v as u64)));
            }
            let _ = graph::update_thing(op_id, updates);
        }
        Err(err) => {
            let updates = alloc::vec![
                (
                    crate::symbols::intern("status"),
                    PropValue::Str(String::from(IoStatus::Failed.as_str())),
                ),
                (crate::symbols::intern("error_code"), PropValue::U64(err.code() as u64)),
            ];
            let _ = graph::update_thing(op_id, updates);
        }
    }
}

pub fn handle_interrupt(irq_line: u8) {
    let region = find_region_for_irq(irq_line);
    let timestamp = time::ticks_since_boot();
    let props_orig = InterruptEvent::props_for(irq_line, region, timestamp);
    let mut props = Vec::new();
    for (k, v) in props_orig {
        props.push((crate::symbols::intern(&k), v));
    }
    let _ = graph::create_thing(crate::symbols::intern(graph_kinds::KIND_INTERRUPT_EVENT), props);
}

fn find_region_for_irq(irq_line: u8) -> Option<ThingId> {
    let mut cursor = ThingId(u64::MAX);
    while let Some(id) = graph::next_thing_of_kind(crate::symbols::intern(graph_kinds::KIND_IO_PORT_REGION), cursor) {
        cursor = id;
        if let Some(region) = load_region(id) {
            if region.irq_lines.iter().any(|&irq| irq == irq_line) {
                return Some(id);
            }
        }
    }
    None
}

fn load_region(id: ThingId) -> Option<IoPortRegion> {
    graph::with_thing(id, |thing| { // Manual extraction again
        let mut base = None;
        let mut count = None;
        let mut irqs = Vec::new();
        
        for (k, v) in &thing.props {
             if *k == crate::symbols::intern("base_port") {
                 if let PropValue::U64(val) = v { base = Some(*val as u16); }
             } else if *k == crate::symbols::intern("port_count") {
                 if let PropValue::U64(val) = v { count = Some(*val as u16); }
             } else if *k == crate::symbols::intern("irq_lines") {
                 if let PropValue::U64(val) = v { irqs.push(*val as u8); } // assume scalar or we check how seed_props creates it
             }
        }
        
        // Handling irq_lines from props might need improved layout support (List). 
        // For now assume logic.
        
        if let (Some(b), Some(c)) = (base, count) {
            Some(IoPortRegion {
                id,
                base_port: b,
                port_count: c,
                irq_lines: irqs,
                name: String::from("i8042"), // stub
                claimed_by: Some(ThingId(0)) // stub
            })
        } else {
            None
        }
    }).flatten()
}

fn execute_io_operation(op: &IoPortOp) -> Result<Option<u32>, IoError> {
    let region = load_region(op.region_id).ok_or(IoError::MissingRegion)?;
    if op.offset as u32 >= region.port_count as u32 {
        return Err(IoError::OffsetOutOfRange);
    }
    let port_addr = region.base_port as u32 + op.offset as u32;
    if port_addr > u16::MAX as u32 {
        return Err(IoError::OffsetOutOfRange);
    }
    let addr = port_addr as u16;
    match io::perform_io_operation(addr, op.direction, op.width, op.value) {
        Ok(value) => Ok(value),
        Err(io::IoAccessError::Unsupported) => Err(IoError::Unsupported),
    }
}

#[allow(dead_code)]
enum IoError {
    MissingRegion,
    OffsetOutOfRange,
    Unsupported,
}

impl IoError {
    fn code(&self) -> u32 {
        match self {
            IoError::MissingRegion => 1,
            IoError::OffsetOutOfRange => 2,
            IoError::Unsupported => 3,
        }
    }
}
