use crate::{graph, graph_kinds, time};
use abi::{PropValue, Thing, ThingId};
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};
use thing_models::{
    InterruptEvent, IoDirection, IoPortOp, IoPortRegion, IoStatus, IoWidth,
};

#[cfg(target_arch = "x86_64")]
use x86_64::instructions::port::Port;

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
    graph::get_thing(id)
        .map(|(_, props)| IoPortRegion::from_props(id, props))
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
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let addr = port_addr as u16;
        match (op.direction, op.width) {
            (IoDirection::Write, IoWidth::U8) => {
                Port::<u8>::new(addr).write(op.value as u8);
                Ok(None)
            }
            (IoDirection::Write, IoWidth::U16) => {
                Port::<u16>::new(addr).write(op.value as u16);
                Ok(None)
            }
            (IoDirection::Write, IoWidth::U32) => {
                Port::<u32>::new(addr).write(op.value);
                Ok(None)
            }
            (IoDirection::Read, IoWidth::U8) => {
                Ok(Some(Port::<u8>::new(addr).read() as u32))
            }
            (IoDirection::Read, IoWidth::U16) => {
                Ok(Some(Port::<u16>::new(addr).read() as u32))
            }
            (IoDirection::Read, IoWidth::U32) => Ok(Some(Port::<u32>::new(addr).read())),
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = port_addr;
        Err(IoError::Unsupported)
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
