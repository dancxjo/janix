//! Symbol table
//!
//! Provides stable symbol interning for the kernel. Symbols are used
//! for Thing kinds, predicates, and other semantic identifiers.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

use abi::ids::SymbolId;

/// Global symbol table
static SYMBOLS: Mutex<Option<SymbolTable>> = Mutex::new(None);

pub struct SymbolTable {
    /// Forward mapping: string -> id
    string_to_id: BTreeMap<Vec<u8>, SymbolId>,
    /// Reverse mapping: id -> string
    id_to_string: BTreeMap<SymbolId, Vec<u8>>,
    /// Next available ID
    next_id: u64,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            string_to_id: BTreeMap::new(),
            id_to_string: BTreeMap::new(),
            next_id: 1, // 0 is reserved for INVALID
        }
    }

    /// Pre-seed symbols deterministically.
    /// This MUST be called once at initialization time with a fixed list of strings.
    /// It assigns IDs 1..N in order.
    fn preseed(&mut self, symbols: &[&str]) {
        for s in symbols {
            let bytes = s.as_bytes().to_vec();
            if !self.string_to_id.contains_key(&bytes) {
                let id = SymbolId(self.next_id);
                self.next_id += 1;
                self.string_to_id.insert(bytes.clone(), id);
                self.id_to_string.insert(id, bytes);
            }
        }
    }

    fn intern(&mut self, s: &[u8]) -> SymbolId {
        if let Some(&id) = self.string_to_id.get(s) {
            return id;
        }

        let id = SymbolId(self.next_id);
        self.next_id += 1;

        let bytes = s.to_vec();
        self.string_to_id.insert(bytes.clone(), id);
        self.id_to_string.insert(id, bytes);

        id
    }

    fn resolve(&self, id: SymbolId) -> Option<&[u8]> {
        self.id_to_string.get(&id).map(|v| v.as_slice())
    }
}

/// Initialize the symbol table and seed core symbols
pub fn init() {
    let mut table = SymbolTable::new();

    // Deterministic pre-seeding
    // These MUST match the constants in the `sym` module below.
    table.preseed(&[
        "place.root",
        "place.kernel",
        "place.devices",
        "place.scheduler", // Added
        "place.memory",
        "place.tasks",
        "place.logs",
        "place.time",
        "kind.Place",
        "kind.Thing",
        "kind.Relationship",
        "predicate.contains",
        "predicate.owns",
        "predicate.references",
        "place.faults",
        "kind.Fault",
        "predicate.has_kind",
        "predicate.at_ip",
        "predicate.at_sp",
        "predicate.at_addr",
        "predicate.has_field",
        "predicate.caused_by",
        "predicate.value",
        "fault_kind.unknown",
        "fault_kind.syscall",
        "fault_kind.irq",
        "fault_kind.timer",
        "fault_kind.external_interrupt",
        "fault_kind.software_interrupt",
        "fault_kind.breakpoint",
        "fault_kind.illegal_instruction",
        "fault_kind.page_fault",
        "fault_kind.access_fault",
        "fault_kind.data_abort",
        "fault_kind.instruction_abort",
        "fault_kind.general_protection",
        "fault_kind.double_fault",
        // Scheduler Symbols
        "scheduler.main",
        "kind.Task",
        "kind.TaskContext",
        "kind.RunQueue",
        "kind.Cpu",
        "kind.Scheduler",
        "task_state.ready",
        "task_state.running",
        "task_state.blocked",
        "task_state.dead",
        "task_state.new",
        "predicate.on_cpu",
        "predicate.in_run_queue",
        "predicate.has_context",
        "predicate.state",
        "predicate.has_stack",
        "predicate.owned_by",
        // Memory symbols
        "kind.AddressSpace",
        "kind.Mapping",
        "kind.PageTable",
        "kind.ByteSpace",
        "kind.ValueU64",
        "kind.ValuePerms",
        "predicate.has_space",
        "predicate.maps",
        "predicate.backs",
        "predicate.range",
        "predicate.perms",
        "predicate.user",    // used as predicate or flag? "predicate.is_user"? or relationship?
        "predicate.kernel",
        "perm.read",
        "perm.write",
        "perm.exec",
        "perm.user",
        "perm.kernel",
        "perm.device",
        "perm.normal",
    ]);

    *SYMBOLS.lock() = Some(table);
}

/// Intern a symbol, returning its stable ID
pub fn intern(s: &[u8]) -> SymbolId {
    let mut guard = SYMBOLS.lock();
    match guard.as_mut() {
        Some(table) => table.intern(s),
        None => SymbolId::INVALID,
    }
}

/// Resolve a symbol ID to its string representation
pub fn resolve(id: SymbolId) -> Option<String> {
    let guard = SYMBOLS.lock();
    guard.as_ref().and_then(|table| {
        table
            .resolve(id)
            .and_then(|bytes| core::str::from_utf8(bytes).ok().map(String::from))
    })
}

/// Well-known symbol constants.
/// These MUST match the order in `init()`.
pub mod sym {
    use abi::ids::SymbolId;

    pub const PLACE_ROOT: SymbolId = SymbolId(1);
    pub const PLACE_KERNEL: SymbolId = SymbolId(2);
    pub const PLACE_DEVICES: SymbolId = SymbolId(3);
    pub const PLACE_SCHEDULER: SymbolId = SymbolId(4); // New
    pub const PLACE_MEMORY: SymbolId = SymbolId(5);
    pub const PLACE_TASKS: SymbolId = SymbolId(6);
    pub const PLACE_LOGS: SymbolId = SymbolId(7);
    pub const PLACE_TIME: SymbolId = SymbolId(8);

    pub const KIND_PLACE: SymbolId = SymbolId(9);
    pub const KIND_THING: SymbolId = SymbolId(10);
    pub const KIND_RELATIONSHIP: SymbolId = SymbolId(11);
    
    pub const PRED_CONTAINS: SymbolId = SymbolId(12);
    pub const PRED_OWNS: SymbolId = SymbolId(13);
    pub const PRED_REFERENCES: SymbolId = SymbolId(14);

    pub const PLACE_FAULTS: SymbolId = SymbolId(15);
    pub const KIND_FAULT: SymbolId = SymbolId(16);
    pub const PRED_HAS_KIND: SymbolId = SymbolId(17);
    pub const PRED_AT_IP: SymbolId = SymbolId(18);
    pub const PRED_AT_SP: SymbolId = SymbolId(19);
    pub const PRED_AT_ADDR: SymbolId = SymbolId(20);
    pub const PRED_HAS_FIELD: SymbolId = SymbolId(21);
    pub const PRED_CAUSED_BY: SymbolId = SymbolId(22);
    pub const PRED_VALUE: SymbolId = SymbolId(23); // Generic value predicate if needed, or specific for values

    // Fault Kinds
    pub const FAULT_KIND_UNKNOWN: SymbolId = SymbolId(24);
    pub const FAULT_KIND_SYSCALL: SymbolId = SymbolId(25);
    pub const FAULT_KIND_IRQ: SymbolId = SymbolId(26);
    pub const FAULT_KIND_TIMER: SymbolId = SymbolId(27);
    pub const FAULT_KIND_EXTERNAL_INTERRUPT: SymbolId = SymbolId(28);
    pub const FAULT_KIND_SOFTWARE_INTERRUPT: SymbolId = SymbolId(29);
    pub const FAULT_KIND_BREAKPOINT: SymbolId = SymbolId(30);
    pub const FAULT_KIND_ILLEGAL_INSTRUCTION: SymbolId = SymbolId(31);
    pub const FAULT_KIND_PAGE_FAULT: SymbolId = SymbolId(32);
    pub const FAULT_KIND_ACCESS_FAULT: SymbolId = SymbolId(33);
    pub const FAULT_KIND_DATA_ABORT: SymbolId = SymbolId(34);
    pub const FAULT_KIND_INSTRUCTION_ABORT: SymbolId = SymbolId(35);
    pub const FAULT_KIND_GENERAL_PROTECTION: SymbolId = SymbolId(36);
    pub const FAULT_KIND_DOUBLE_FAULT: SymbolId = SymbolId(37);

    // Scheduler
    pub const SCHEDULER_MAIN: SymbolId = SymbolId(38);
    pub const KIND_TASK: SymbolId = SymbolId(39);
    pub const KIND_TASK_CONTEXT: SymbolId = SymbolId(40);
    pub const KIND_RUN_QUEUE: SymbolId = SymbolId(41);
    pub const KIND_CPU: SymbolId = SymbolId(42);
    pub const KIND_SCHEDULER: SymbolId = SymbolId(43);
    
    pub const TASK_STATE_READY: SymbolId = SymbolId(44);
    pub const TASK_STATE_RUNNING: SymbolId = SymbolId(45);
    pub const TASK_STATE_BLOCKED: SymbolId = SymbolId(46);
    pub const TASK_STATE_DEAD: SymbolId = SymbolId(47);
    pub const TASK_STATE_NEW: SymbolId = SymbolId(48);

    pub const PRED_ON_CPU: SymbolId = SymbolId(49);
    pub const PRED_IN_RUN_QUEUE: SymbolId = SymbolId(50);
    pub const PRED_HAS_CONTEXT: SymbolId = SymbolId(51);
    pub const PRED_STATE: SymbolId = SymbolId(52);
    pub const PRED_HAS_STACK: SymbolId = SymbolId(53);
    pub const PRED_OWNED_BY: SymbolId = SymbolId(54);

    // Memory
    pub const KIND_ADDRESS_SPACE: SymbolId = SymbolId(55);
    pub const KIND_MAPPING: SymbolId = SymbolId(56);
    pub const KIND_PAGE_TABLE: SymbolId = SymbolId(57);
    pub const KIND_BYTE_SPACE: SymbolId = SymbolId(58);
    pub const KIND_VALUE_U64: SymbolId = SymbolId(59);
    pub const KIND_VALUE_PERMS: SymbolId = SymbolId(60);

    pub const PRED_HAS_SPACE: SymbolId = SymbolId(61);
    pub const PRED_MAPS: SymbolId = SymbolId(62);
    pub const PRED_BACKS: SymbolId = SymbolId(63);
    pub const PRED_RANGE: SymbolId = SymbolId(64);
    pub const PRED_PERMS: SymbolId = SymbolId(65);
    pub const PRED_USER: SymbolId = SymbolId(66);
    pub const PRED_KERNEL: SymbolId = SymbolId(67);

    pub const PERM_READ: SymbolId = SymbolId(68);
    pub const PERM_WRITE: SymbolId = SymbolId(69);
    pub const PERM_EXEC: SymbolId = SymbolId(70);
    pub const PERM_USER: SymbolId = SymbolId(71);
    pub const PERM_KERNEL: SymbolId = SymbolId(72);
    pub const PERM_DEVICE: SymbolId = SymbolId(73);
    pub const PERM_NORMAL: SymbolId = SymbolId(74);
}
