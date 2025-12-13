use crate::{graph, graph_kinds, time};
use abi::{PropKey, PropValue, ThingId};
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
    let Some((kind, props)) = graph::get_thing(id) else { return; };
    if kind != abi::graph_kinds::KIND_INTERRUPT_REQUEST { return; }
    let mut irq_line: u8 = 0;
    let mut enabled: bool = false;
    for (k, v) in props.iter().flatten() {
        match *k {
            abi::graph_kinds::PROP_IRQ_LINE => if let PropValue::U64(val) = v { irq_line = *val as u8; },
            abi::graph_kinds::PROP_ENABLED => if let PropValue::Bool(b) = v { enabled = *b; },
            _ => {}
        }
    }
    let masked = !enabled;
    if let Some(handler) = *IRQ_CONTROLLER.lock() {
        handler(irq_line, masked);
        let msg = format!(
            "Processed InterruptRequest {}: line={}, enabled={}, masked={}",
            id.0, irq_line, enabled, masked
        );
        let leaked: &'static str = Box::leak(msg.into_boxed_str());
        crate::log(leaked);
    } else {
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
use abi::Thing;

#[path = "../../../arch/src/io.rs"]
mod arch_io;
use arch_io as io;

static IO_REGIONS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn seed_io_regions() {
    #[cfg(target_arch = "x86_64")]
    {
        if IO_REGIONS_INITIALIZED
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }
        let props = IoPortRegion::seed_props("i8042", 0x60, 5, &[1, 12]);
        let _ = graph::create_thing(graph_kinds::KIND_IO_PORT_REGION, props.as_slice());
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = IO_REGIONS_INITIALIZED.load(Ordering::Relaxed);
    }
}

pub fn process_io_op(op_id: ThingId) {
    let Some((kind, props)) = graph::get_thing(op_id) else {
        return;
    };
    if kind != graph_kinds::KIND_IO_PORT_OP {
        return;
    }
    let op = IoPortOp::from_props(op_id, props);
    if op.status != IoStatus::Pending {
        return;
    }

    let _ = graph::update_thing(
        op_id,
        &[(
            "status",
            PropValue::Str(String::from(IoStatus::InProgress.as_str())),
        )],
    );

    match execute_io_operation(&op) {
        Ok(value) => {
            let mut updates = Vec::new();
            updates.push((
                "status",
                PropValue::Str(String::from(IoStatus::Completed.as_str())),
            ));
            updates.push(("error_code", PropValue::U64(0)));
            if let Some(v) = value {
                updates.push(("value", PropValue::U64(v as u64)));
            }
            let _ = graph::update_thing(op_id, &updates);
        }
        Err(err) => {
            let updates = [
                (
                    "status",
                    PropValue::Str(String::from(IoStatus::Failed.as_str())),
                ),
                ("error_code", PropValue::U64(err.code() as u64)),
            ];
            let _ = graph::update_thing(op_id, &updates);
        }
    }
}

pub fn handle_interrupt(irq_line: u8) {
    let region = find_region_for_irq(irq_line);
    let timestamp = time::ticks_since_boot();
    let props = InterruptEvent::props_for(irq_line, region, timestamp);
    let _ = graph::create_thing(graph_kinds::KIND_INTERRUPT_EVENT, props.as_slice());
}

fn find_region_for_irq(irq_line: u8) -> Option<ThingId> {
    let mut cursor = ThingId(u64::MAX);
    while let Some(id) = graph::next_thing_of_kind(graph_kinds::KIND_IO_PORT_REGION, cursor) {
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
    graph::get_thing(id).map(|(_, props)| IoPortRegion::from_props(id, props))
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
