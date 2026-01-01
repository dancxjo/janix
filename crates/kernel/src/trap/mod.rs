//! Unified Trap Handling
//!
//! Normalizes interrupts/exceptions into `TrapRecord` and records them valid
//! `fault` Things in the system graph.

use abi::ids::ThingId;
// use abi::types::{Thing, Value}; // types is unlikely, wire or ids.
// Looking at error, I'll remove unused imports or use specific ones.
use graph::store::{self, PlaceStore}; 
use graph::symbols::{self, sym};
use alloc::vec::Vec;
use crate::log::{self, Level};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    Aarch64,
    Riscv64,
    LoongArch64,
}

#[derive(Debug, Clone, Copy)]
pub enum FaultKind {
    Unknown,
    Syscall,
    Irq,
    Timer,
    ExternalInterrupt,
    SoftwareInterrupt,
    Breakpoint,
    IllegalInstruction,
    PageFault,
    AccessFault,
    DataAbort,
    InstructionAbort,
    GeneralProtection,
    DoubleFault,
}

impl FaultKind {
    pub fn to_symbol(&self) -> abi::ids::SymbolId {
        match self {
            FaultKind::Unknown => sym::FAULT_KIND_UNKNOWN,
            FaultKind::Syscall => sym::FAULT_KIND_SYSCALL,
            FaultKind::Irq => sym::FAULT_KIND_IRQ,
            FaultKind::Timer => sym::FAULT_KIND_TIMER,
            FaultKind::ExternalInterrupt => sym::FAULT_KIND_EXTERNAL_INTERRUPT,
            FaultKind::SoftwareInterrupt => sym::FAULT_KIND_SOFTWARE_INTERRUPT,
            FaultKind::Breakpoint => sym::FAULT_KIND_BREAKPOINT,
            FaultKind::IllegalInstruction => sym::FAULT_KIND_ILLEGAL_INSTRUCTION,
            FaultKind::PageFault => sym::FAULT_KIND_PAGE_FAULT,
            FaultKind::AccessFault => sym::FAULT_KIND_ACCESS_FAULT,
            FaultKind::DataAbort => sym::FAULT_KIND_DATA_ABORT,
            FaultKind::InstructionAbort => sym::FAULT_KIND_INSTRUCTION_ABORT,
            FaultKind::GeneralProtection => sym::FAULT_KIND_GENERAL_PROTECTION,
            FaultKind::DoubleFault => sym::FAULT_KIND_DOUBLE_FAULT,
        }
    }
}

pub struct TrapRecord {
    pub arch: Arch,
    pub kind: FaultKind,
    pub ip: u64,
    pub sp: u64,
    pub addr: Option<u64>,
    pub code: u64,
    pub vector: u32,
    pub cpu: u32,
    pub in_kernel: bool,
    pub task: Option<ThingId>,
}

pub fn record_fault(place: &mut PlaceStore, tr: &TrapRecord) -> ThingId {
    let fault_id = place.create_thing(sym::KIND_FAULT).expect("failed to create fault thing");
    
    // Resolve Place.Faults ID
    // We can't use store::find_by_name directly because we have &mut PlaceStore `place`.
    // store::find_thing_by_name locks global.
    // We should use `place.find_by_name` (which I exposed).
    
    if let Some(place_faults) = place.find_by_name(sym::PLACE_FAULTS) {
        place.create_relationship(sym::PRED_CONTAINS, place_faults, fault_id).expect("failed to link fault");
    } else {
        // Fallback or panic? For now log error but continue (orphaned fault)
        // This likely means seed_minimal wasn't called or symbols name mismatch.
    }
    
    let kind_sym = tr.kind.to_symbol();
    let kind_thing_id = place.create_thing(sym::KIND_THING).expect("failed to create kind thing");
    place.register_name(kind_thing_id, kind_sym);
    
    place.create_relationship(sym::PRED_HAS_KIND, fault_id, kind_thing_id).expect("failed to link kind");
    
    let ip_thing = place.create_thing(sym::KIND_THING).expect("IP thing");
    place.set_payload(ip_thing, &tr.ip.to_le_bytes());
    place.create_relationship(sym::PRED_AT_IP, fault_id, ip_thing).expect("link IP");
    
    let sp_thing = place.create_thing(sym::KIND_THING).expect("SP thing");
    place.set_payload(sp_thing, &tr.sp.to_le_bytes());
    place.create_relationship(sym::PRED_AT_SP, fault_id, sp_thing).expect("link SP");
    
    if let Some(addr) = tr.addr {
        let addr_thing = place.create_thing(sym::KIND_THING).expect("Addr thing");
        place.set_payload(addr_thing, &addr.to_le_bytes());
        place.create_relationship(sym::PRED_AT_ADDR, fault_id, addr_thing).expect("link Addr");
    }
    
    let mut add_field = |val: u64| {
        if let Ok(t) = place.create_thing(sym::KIND_THING) {
            place.set_payload(t, &val.to_le_bytes());
            let _ = place.create_relationship(sym::PRED_HAS_FIELD, fault_id, t);
        }
    };
    
    add_field(tr.code);
    add_field(tr.vector as u64);
    add_field(tr.cpu as u64);
    
    if let Some(task_id) = tr.task {
        let _ = place.create_relationship(sym::PRED_CAUSED_BY, fault_id, task_id);
    }

    fault_id
}

pub fn debug_dump_faults(limit: usize) {
    let place_faults = match store::find_thing_by_name(sym::PLACE_FAULTS) {
        Some(id) => id,
        None => {
             log::klog(Level::Warn, "TRAP", "place.faults not found");
             return;
        }
    };

    let faults = crate::place::contained_in(place_faults);
    log::klog(Level::Info, "TRAP", "Dumping faults..."); // Replaced macro with klog
    
    for (i, fault_id) in faults.iter().take(limit).enumerate() {
        // log::klog_info!("Fault #{}: {:?}", i, fault_id); // Replaced macro
        // formatting is annoying with klog, need format! or similar, but we are no_std.
        // crate::log::klog supports &str.
        // We can use alloc::format! if we have it? crate::log uses it.
        // Assuming alloc::format! is available in context or we can just print ID.
        // Actually log entry takes bytes.
        
        let msg = alloc::format!("Fault #{}: {:?}", i, fault_id);
        log::klog(Level::Info, "TRAP", &msg);
    }
}
