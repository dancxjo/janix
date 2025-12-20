use abi::ThingId;
use crate::{PropKey, PropType, PropValue, Thing};
use crate::graph_kinds;
use alloc::vec::Vec;
use alloc::string::{String, ToString};

#[derive(Clone, Debug)]
pub struct PhysFrame {
    pub id: ThingId,
    pub base: u64,
    pub size: u64,
    pub allocated: bool,
}

impl Thing for PhysFrame {
    const KIND: &'static str = "PhysFrame";
    const DESCRIPTION: &'static str = "A region of physical memory with base address, size, and allocation status";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("base".to_string(), PropValue::U64(self.base)));
        out.push(("size".to_string(), PropValue::U64(self.size)));
        out.push(("allocated".to_string(), PropValue::Bool(self.allocated)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut base = 0;
        let mut size = 0;
        let mut allocated = false;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "base" => if let PropValue::U64(v) = prop.1 { base = v; },
                "size" => if let PropValue::U64(v) = prop.1 { size = v; },
                "allocated" => if let PropValue::Bool(v) = prop.1 { allocated = v; },
                _ => {}
            }
        }
        PhysFrame { id, base, size, allocated }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("base", PropType::U64),
            ("size", PropType::U64),
            ("allocated", PropType::Bool),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct FramePool {
    pub id: ThingId,
    pub start: u64,
    pub end: u64,
    pub frame_size: u64,
}

impl Thing for FramePool {
    const KIND: &'static str = "FramePool";
    const DESCRIPTION: &'static str = "A pool of physical memory frames with defined start, end, and frame size";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("start".to_string(), PropValue::U64(self.start)));
        out.push(("end".to_string(), PropValue::U64(self.end)));
        out.push(("frame_size".to_string(), PropValue::U64(self.frame_size)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut start = 0;
        let mut end = 0;
        let mut frame_size = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "start" => if let PropValue::U64(v) = prop.1 { start = v; },
                "end" => if let PropValue::U64(v) = prop.1 { end = v; },
                "frame_size" => if let PropValue::U64(v) = prop.1 { frame_size = v; },
                _ => {}
            }
        }
        FramePool { id, start, end, frame_size }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("start", PropType::U64),
            ("end", PropType::U64),
            ("frame_size", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct AddressSpace {
    pub id: ThingId,
    pub asid: u64,
}

impl Thing for AddressSpace {
    const KIND: &'static str = "AddressSpace";
    const DESCRIPTION: &'static str = "A virtual address space identified by its address space identifier (ASID)";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("asid".to_string(), PropValue::U64(self.asid)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut asid = 0;
        for prop in props.iter().flatten() {
            if prop.0 == "asid" {
                if let PropValue::U64(v) = prop.1 { asid = v; }
            }
        }
        AddressSpace { id, asid }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("asid", PropType::U64)]
    }
}

#[derive(Clone, Debug)]
pub struct VirtRegion {
    pub id: ThingId,
    pub base: u64,
    pub len: u64,
    pub flags: u64,
}

impl Thing for VirtRegion {
    const KIND: &'static str = "VirtRegion";
    const DESCRIPTION: &'static str = "A virtual memory region with base address, length, and access flags";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("base".to_string(), PropValue::U64(self.base)));
        out.push(("len".to_string(), PropValue::U64(self.len)));
        out.push(("flags".to_string(), PropValue::U64(self.flags)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut base = 0;
        let mut len = 0;
        let mut flags = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "base" => if let PropValue::U64(v) = prop.1 { base = v; },
                "len" => if let PropValue::U64(v) = prop.1 { len = v; },
                "flags" => if let PropValue::U64(v) = prop.1 { flags = v; },
                _ => {}
            }
        }
        VirtRegion { id, base, len, flags }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("base", PropType::U64),
            ("len", PropType::U64),
            ("flags", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct Process {
    pub id: ThingId,
    pub pid: u64,
    pub name: String,
}

impl Thing for Process {
    const KIND: &'static str = graph_kinds::KIND_PROCESS;
    const DESCRIPTION: &'static str = "A process with process identifier (PID) and execution state";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("pid".to_string(), PropValue::U64(self.pid)));
        out.push(("name".to_string(), PropValue::Str(self.name.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut pid = 0;
        let mut name = String::new();

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "pid" => if let PropValue::U64(v) = prop.1 { pid = v; },
                "name" => if let PropValue::Str(v) = &prop.1 { name = v.clone(); },
                _ => {}
            }
        }
        Process { id, pid, name }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("pid", PropType::U64), ("name", PropType::Str)]
    }
}

#[derive(Clone, Debug)]
pub struct Thread {
    pub id: ThingId,
    pub tid: u64,
    pub name: String,
    pub state: String,
    pub priority: u64,
    pub runtime_ns: u64,
    pub last_started_ns: u64,
    pub sleep_until_ns: u64,
}

impl Thing for Thread {
    const KIND: &'static str = graph_kinds::KIND_THREAD;
    const DESCRIPTION: &'static str = "A thread of execution with thread identifier, state, priority, runtime tracking, and last start time";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("tid".to_string(), PropValue::U64(self.tid)));
        out.push(("name".to_string(), PropValue::Str(self.name.clone())));
        out.push(("state".to_string(), PropValue::Str(self.state.clone())));
        out.push(("priority".to_string(), PropValue::U64(self.priority)));
        out.push(("runtime_ns".to_string(), PropValue::U64(self.runtime_ns)));
        out.push(("last_started_ns".to_string(), PropValue::U64(self.last_started_ns)));
        out.push(("sleep_until_ns".to_string(), PropValue::U64(self.sleep_until_ns)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut tid = 0;
        let mut name = String::new();
        let mut state = String::new();
        let mut priority = 0;
        let mut runtime_ns = 0;
        let mut last_started_ns = 0;
        let mut sleep_until_ns = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "tid" => if let PropValue::U64(v) = prop.1 { tid = v; },
                "name" => if let PropValue::Str(v) = &prop.1 { name = v.clone(); },
                "state" => if let PropValue::Str(v) = &prop.1 { state = v.clone(); },
                "priority" => if let PropValue::U64(v) = prop.1 { priority = v; },
                "runtime_ns" => if let PropValue::U64(v) = prop.1 { runtime_ns = v; },
                "last_started_ns" => if let PropValue::U64(v) = prop.1 { last_started_ns = v; },
                "sleep_until_ns" => if let PropValue::U64(v) = prop.1 { sleep_until_ns = v; },
                _ => {}
            }
        }
        Thread { id, tid, name, state, priority, runtime_ns, last_started_ns, sleep_until_ns }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("tid", PropType::U64),
            ("name", PropType::Str),
            ("state", PropType::Str),
            ("priority", PropType::U64),
            ("runtime_ns", PropType::U64),
            ("last_started_ns", PropType::U64),
            ("sleep_until_ns", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct CpuCore {
    pub id: ThingId,
    pub index: u64,
}

impl Thing for CpuCore {
    const KIND: &'static str = graph_kinds::KIND_CPU_CORE;
    const DESCRIPTION: &'static str = "A CPU core identified by its index in the system";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("index".to_string(), PropValue::U64(self.index)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut index = 0;
        for prop in props.iter().flatten() {
            if prop.0 == "index" {
                if let PropValue::U64(v) = prop.1 { index = v; }
            }
        }
        CpuCore { id, index }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("index", PropType::U64)]
    }
}

#[derive(Clone, Debug)]
pub struct SleepEvent {
    pub id: ThingId,
    pub wake_at_ns: u64,
    pub created_at_ns: u64,
}

impl Thing for SleepEvent {
    const KIND: &'static str = graph_kinds::KIND_SLEEP_EVENT;
    const DESCRIPTION: &'static str = "A scheduled wakeup for a sleeping thread";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("wake_at_ns".to_string(), PropValue::U64(self.wake_at_ns)));
        out.push(("created_at_ns".to_string(), PropValue::U64(self.created_at_ns)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut wake_at_ns = 0;
        let mut created_at_ns = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "wake_at_ns" => if let PropValue::U64(v) = prop.1 { wake_at_ns = v; },
                "created_at_ns" => if let PropValue::U64(v) = prop.1 { created_at_ns = v; },
                _ => {}
            }
        }
        SleepEvent { id, wake_at_ns, created_at_ns }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("wake_at_ns", PropType::U64),
            ("created_at_ns", PropType::U64),
        ]
    }
}

#[derive(Clone, Debug)]
pub struct ThreadInfo {
    pub id: ThingId, // Added id field to match Thing trait requirement if it wasn't there
    pub name: String,
    pub state: String,
    pub last_run_ns: i64,
    pub total_run_ns: i64,
    pub process_thing_id: u64,
    pub scheduler_thing_id: u64,
}

impl Thing for ThreadInfo {
    const KIND: &'static str = "ThreadInfo";
    const DESCRIPTION: &'static str = "Runtime information about a thread including state, execution time, and owning process";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name".to_string(), PropValue::Str(self.name.clone())));
        out.push(("state".to_string(), PropValue::Str(self.state.clone())));
        out.push(("last_run_ns".to_string(), PropValue::I64(self.last_run_ns)));
        out.push(("total_run_ns".to_string(), PropValue::I64(self.total_run_ns)));
        out.push(("process_thing_id".to_string(), PropValue::U64(self.process_thing_id)));
        out.push(("scheduler_thing_id".to_string(), PropValue::U64(self.scheduler_thing_id)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut state = String::new();
        let mut last_run_ns = 0;
        let mut total_run_ns = 0;
        let mut process_thing_id = 0;
        let mut scheduler_thing_id = 0;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "name" => if let PropValue::Str(v) = &prop.1 { name = v.clone(); },
                "state" => if let PropValue::Str(v) = &prop.1 { state = v.clone(); },
                "last_run_ns" => if let PropValue::I64(v) = prop.1 { last_run_ns = v; },
                "total_run_ns" => if let PropValue::I64(v) = prop.1 { total_run_ns = v; },
                "process_thing_id" => if let PropValue::U64(v) = prop.1 { process_thing_id = v; },
                "scheduler_thing_id" => if let PropValue::U64(v) = prop.1 { scheduler_thing_id = v; },
                _ => {}
            }
        }
        ThreadInfo { id, name, state, last_run_ns, total_run_ns, process_thing_id, scheduler_thing_id }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::Str),
            ("state", PropType::Str),
            ("last_run_ns", PropType::I64),
            ("total_run_ns", PropType::I64),
            ("process_thing_id", PropType::U64),
            ("scheduler_thing_id", PropType::U64),
        ]
    }
}
