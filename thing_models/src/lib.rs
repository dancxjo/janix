#![no_std]

extern crate alloc;

pub mod display;
pub mod input;
pub mod io;
pub mod ui;
pub mod usb;

use abi::{PropKey, PropType, PropValue, Thing, ThingId, graph_kinds};
use alloc::string::String;
use alloc::vec::Vec;
pub use display::*;
pub use input::*;
pub use io::*;
pub use ui::*;
pub use usb::*;

pub struct ThreadInfo {
    pub name: String,  // "hello", "heartbeat", "dashboard"
    pub state: String, // "NEW", "RUNNABLE", "RUNNING", "SLEEPING", "TERMINATED"
    pub last_run_ns: i64,
    pub total_run_ns: i64,
    pub process_thing_id: u64,
    pub scheduler_thing_id: u64,
}

impl Thing for ThreadInfo {
    const KIND: &'static str = "ThreadInfo";
    const DESCRIPTION: &'static str =
        "Runtime information about a thread including state, execution time, and owning process";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name", PropValue::Str(self.name.clone())));
        out.push(("state", PropValue::Str(self.state.clone())));
        out.push(("last_run_ns", PropValue::I64(self.last_run_ns)));
        out.push(("total_run_ns", PropValue::I64(self.total_run_ns)));
        out.push(("process_thing_id", PropValue::U64(self.process_thing_id)));
        out.push((
            "scheduler_thing_id",
            PropValue::U64(self.scheduler_thing_id),
        ));
    }

    fn from_props(_id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        use alloc::string::String;

        let mut name = String::new();
        let mut state = String::new();
        let mut last_run_ns = 0_i64;
        let mut total_run_ns = 0_i64;
        let mut process_thing_id = 0_u64;
        let mut scheduler_thing_id = 0_u64;

        for prop in props {
            if let Some((k, v)) = prop {
                match *k {
                    "name" => {
                        if let PropValue::Str(s) = v {
                            name = s.clone();
                        }
                    }
                    "state" => {
                        if let PropValue::Str(s) = v {
                            state = s.clone();
                        }
                    }
                    "last_run_ns" => {
                        if let PropValue::I64(val) = v {
                            last_run_ns = *val;
                        }
                    }
                    "total_run_ns" => {
                        if let PropValue::I64(val) = v {
                            total_run_ns = *val;
                        }
                    }
                    "process_thing_id" => {
                        if let PropValue::U64(val) = v {
                            process_thing_id = *val;
                        }
                    }
                    "scheduler_thing_id" => {
                        if let PropValue::U64(val) = v {
                            scheduler_thing_id = *val;
                        }
                    }
                    _ => {}
                }
            }
        }

        ThreadInfo {
            name,
            state,
            last_run_ns,
            total_run_ns,
            process_thing_id,
            scheduler_thing_id,
        }
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

pub struct BootProfile {
    pub id: ThingId,
    pub version: u64,
}

impl Thing for BootProfile {
    const KIND: &'static str = "BootProfile";
    const DESCRIPTION: &'static str =
        "The system-wide boot configuration used by init to launch all services and programs.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("version", PropValue::U64(self.version)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut version = 0;
        for prop in props.iter().flatten() {
            if prop.0 == "version" {
                if let PropValue::U64(v) = prop.1 {
                    version = v;
                }
            }
        }
        BootProfile { id, version }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[("version", PropType::U64)]
    }
}

pub struct BootProgram {
    pub id: ThingId,
    pub name: String,
    pub app_id: u64,
    pub priority: u64,
    pub binary: String,
}

impl Thing for BootProgram {
    const KIND: &'static str = "BootProgram";
    const DESCRIPTION: &'static str =
        "A program to be launched automatically by init during system boot.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("name", PropValue::Str(self.name.clone())));
        out.push(("app_id", PropValue::U64(self.app_id)));
        out.push(("priority", PropValue::U64(self.priority)));
        out.push(("binary", PropValue::Str(self.binary.clone())));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut app_id = 0;
        let mut priority = 0;
        let mut binary = String::new();
        for prop in props.iter().flatten() {
            match prop.0 {
                "name" => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                "app_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        app_id = v;
                    }
                }
                "priority" => {
                    if let PropValue::U64(v) = prop.1 {
                        priority = v;
                    }
                }
                "binary" => {
                    if let PropValue::Str(v) = &prop.1 {
                        binary = v.clone();
                    }
                }
                _ => {}
            }
        }
        BootProgram {
            id,
            name,
            app_id,
            priority,
            binary,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("name", PropType::Str),
            ("app_id", PropType::U64),
            ("priority", PropType::U64),
            ("binary", PropType::Str),
        ]
    }
}

pub struct ProgramImage {
    pub id: ThingId,
    pub identifier: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
}

impl Thing for ProgramImage {
    const KIND: &'static str = "ProgramImage";
    const DESCRIPTION: &'static str =
        "An ELF program image discovered at boot and available for loading.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            graph_kinds::PROP_IDENTIFIER,
            PropValue::Str(self.identifier.clone()),
        ));
        out.push((
            graph_kinds::PROP_MODULE_INDEX,
            PropValue::U64(self.module_index),
        ));
        out.push((graph_kinds::PROP_BASE_PHYS, PropValue::U64(self.base_phys)));
        out.push((graph_kinds::PROP_SIZE, PropValue::U64(self.size)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut identifier = String::new();
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;
        for prop in props.iter().flatten() {
            match prop.0 {
                graph_kinds::PROP_IDENTIFIER => {
                    if let PropValue::Str(v) = &prop.1 {
                        identifier = v.clone();
                    }
                }
                graph_kinds::PROP_MODULE_INDEX => {
                    if let PropValue::U64(v) = prop.1 {
                        module_index = v;
                    }
                }
                graph_kinds::PROP_BASE_PHYS => {
                    if let PropValue::U64(v) = prop.1 {
                        base_phys = v;
                    }
                }
                graph_kinds::PROP_SIZE => {
                    if let PropValue::U64(v) = prop.1 {
                        size = v;
                    }
                }
                _ => {}
            }
        }
        ProgramImage {
            id,
            identifier,
            module_index,
            base_phys,
            size,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_IDENTIFIER, PropType::Str),
            (graph_kinds::PROP_MODULE_INDEX, PropType::U64),
            (graph_kinds::PROP_BASE_PHYS, PropType::U64),
            (graph_kinds::PROP_SIZE, PropType::U64),
        ]
    }
}

pub struct FontModule {
    pub id: ThingId,
    pub name: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
}

impl Thing for FontModule {
    const KIND: &'static str = graph_kinds::KIND_FONT_MODULE;
    const DESCRIPTION: &'static str =
        "A font payload supplied as a boot module available for UI rendering.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            graph_kinds::PROP_FONT_NAME,
            PropValue::Str(self.name.clone()),
        ));
        out.push((
            graph_kinds::PROP_MODULE_INDEX,
            PropValue::U64(self.module_index),
        ));
        out.push((graph_kinds::PROP_BASE_PHYS, PropValue::U64(self.base_phys)));
        out.push((graph_kinds::PROP_SIZE, PropValue::U64(self.size)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;
        for prop in props.iter().flatten() {
            match prop.0 {
                graph_kinds::PROP_FONT_NAME => {
                    if let PropValue::Str(v) = &prop.1 {
                        name = v.clone();
                    }
                }
                graph_kinds::PROP_MODULE_INDEX => {
                    if let PropValue::U64(v) = prop.1 {
                        module_index = v;
                    }
                }
                graph_kinds::PROP_BASE_PHYS => {
                    if let PropValue::U64(v) = prop.1 {
                        base_phys = v;
                    }
                }
                graph_kinds::PROP_SIZE => {
                    if let PropValue::U64(v) = prop.1 {
                        size = v;
                    }
                }
                _ => {}
            }
        }

        FontModule {
            id,
            name,
            module_index,
            base_phys,
            size,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_FONT_NAME, PropType::Str),
            (graph_kinds::PROP_MODULE_INDEX, PropType::U64),
            (graph_kinds::PROP_BASE_PHYS, PropType::U64),
            (graph_kinds::PROP_SIZE, PropType::U64),
        ]
    }
}

pub struct TimeSource {
    pub id: ThingId,
    pub ticks_since_boot: u64,
    pub tick_hz: u32,
    pub unix_seconds: i64,
    pub unix_nanos: u32,
}

impl Thing for TimeSource {
    const KIND: &'static str = "TimeSource";
    const DESCRIPTION: &'static str =
        "Kernel-published system clock including monotonic tick counter and Unix wall time.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("ticks_since_boot", PropValue::U64(self.ticks_since_boot)));
        out.push(("tick_hz", PropValue::U64(self.tick_hz as u64)));
        out.push(("unix_seconds", PropValue::I64(self.unix_seconds)));
        out.push(("unix_nanos", PropValue::U64(self.unix_nanos as u64)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut ticks_since_boot = 0_u64;
        let mut tick_hz = 0_u32;
        let mut unix_seconds = 0_i64;
        let mut unix_nanos = 0_u32;

        for prop in props.iter().flatten() {
            match prop.0 {
                "ticks_since_boot" => {
                    if let PropValue::U64(v) = prop.1 {
                        ticks_since_boot = v;
                    }
                }
                "tick_hz" => {
                    if let PropValue::U64(v) = prop.1 {
                        tick_hz = v as u32;
                    }
                }
                "unix_seconds" => {
                    if let PropValue::I64(v) = prop.1 {
                        unix_seconds = v;
                    }
                }
                "unix_nanos" => {
                    if let PropValue::U64(v) = prop.1 {
                        unix_nanos = v as u32;
                    }
                }
                _ => {}
            }
        }

        TimeSource {
            id,
            ticks_since_boot,
            tick_hz,
            unix_seconds,
            unix_nanos,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("ticks_since_boot", PropType::U64),
            ("tick_hz", PropType::U64),
            ("unix_seconds", PropType::I64),
            ("unix_nanos", PropType::U64),
        ]
    }
}

impl TimeSource {
    pub fn create(tick_hz: u32, unix_seconds: i64, unix_nanos: u32) -> [(PropKey, PropValue); 4] {
        [
            ("tick_hz", PropValue::U64(tick_hz as u64)),
            ("ticks_since_boot", PropValue::U64(0)),
            ("unix_seconds", PropValue::I64(unix_seconds)),
            ("unix_nanos", PropValue::U64(unix_nanos as u64)),
        ]
    }

    pub fn update_from_kernel(
        ticks_since_boot: u64,
        unix_seconds: i64,
        unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        [
            ("ticks_since_boot", PropValue::U64(ticks_since_boot)),
            ("unix_seconds", PropValue::I64(unix_seconds)),
            ("unix_nanos", PropValue::U64(unix_nanos as u64)),
        ]
    }
}

pub struct AlarmRequest {
    pub id: ThingId,
    pub target_unix_seconds: i64,
    pub target_unix_nanos: u32,
    pub owner_process: ThingId,
    pub owner_thread: ThingId,
    pub state: String,
    pub target_ticks: Option<u64>,
}

impl Thing for AlarmRequest {
    const KIND: &'static str = "AlarmRequest";
    const DESCRIPTION: &'static str =
        "User-requested alarm mapped to kernel tick space and lifecycle state.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((
            "target_unix_seconds",
            PropValue::I64(self.target_unix_seconds),
        ));
        out.push((
            "target_unix_nanos",
            PropValue::U64(self.target_unix_nanos as u64),
        ));
        out.push(("owner_process", PropValue::U64(self.owner_process.0)));
        out.push(("owner_thread", PropValue::U64(self.owner_thread.0)));
        out.push(("state", PropValue::Str(self.state.clone())));
        if let Some(ticks) = self.target_ticks {
            out.push(("target_ticks", PropValue::U64(ticks)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut target_unix_seconds = 0_i64;
        let mut target_unix_nanos = 0_u32;
        let mut owner_process = ThingId(0);
        let mut owner_thread = ThingId(0);
        let mut state = String::new();
        let mut target_ticks = None;

        for prop in props.iter().flatten() {
            match prop.0 {
                "target_unix_seconds" => {
                    if let PropValue::I64(v) = prop.1 {
                        target_unix_seconds = v;
                    }
                }
                "target_unix_nanos" => {
                    if let PropValue::U64(v) = prop.1 {
                        target_unix_nanos = v as u32;
                    }
                }
                "owner_process" => {
                    if let PropValue::U64(v) = prop.1 {
                        owner_process = ThingId(v);
                    }
                }
                "owner_thread" => {
                    if let PropValue::U64(v) = prop.1 {
                        owner_thread = ThingId(v);
                    }
                }
                "state" => {
                    if let PropValue::Str(ref v) = prop.1 {
                        state = v.clone();
                    }
                }
                "target_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        target_ticks = Some(v);
                    }
                }
                _ => {}
            }
        }

        AlarmRequest {
            id,
            target_unix_seconds,
            target_unix_nanos,
            owner_process,
            owner_thread,
            state,
            target_ticks,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("target_unix_seconds", PropType::I64),
            ("target_unix_nanos", PropType::U64),
            ("owner_process", PropType::U64),
            ("owner_thread", PropType::U64),
            ("state", PropType::Str),
            ("target_ticks", PropType::U64),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use abi::{graph_kinds, PropKey, PropValue};
    use alloc::{string::String, vec::Vec};

    fn to_prop_slice(props: &[(PropKey, PropValue)]) -> Vec<Option<(PropKey, PropValue)>> {
        props.iter().map(|(k, v)| Some((*k, v.clone()))).collect()
    }

    #[test]
    fn boot_program_roundtrip_props() {
        let boot = BootProgram {
            id: ThingId(0x00_1),
            name: String::from("init"),
            app_id: 0x42,
            priority: 1,
            binary: String::from("/bin/init"),
        };

        let mut props = Vec::new();
        boot.to_props(&mut props);
        assert_eq!(
            props,
            [
                ("name", PropValue::Str(String::from("init"))),
                ("app_id", PropValue::U64(0x42)),
                ("priority", PropValue::U64(1)),
                ("binary", PropValue::Str(String::from("/bin/init"))),
            ]
        );

        let roundtrip = BootProgram::from_props(boot.id, &to_prop_slice(&props));
        assert_eq!(roundtrip.id, boot.id);
        assert_eq!(roundtrip.name, boot.name);
        assert_eq!(roundtrip.app_id, boot.app_id);
        assert_eq!(roundtrip.priority, boot.priority);
        assert_eq!(roundtrip.binary, boot.binary);
        let mut roundtrip_props = Vec::new();
        roundtrip.to_props(&mut roundtrip_props);
        assert_eq!(roundtrip_props, props);
    }

    #[test]
    fn program_image_roundtrip_props() {
        let image = ProgramImage {
            id: ThingId(0xAA),
            identifier: String::from("kernel"),
            module_index: 3,
            base_phys: 0x1000,
            size: 0x2000,
        };

        let mut props = Vec::new();
        image.to_props(&mut props);
        assert_eq!(
            props,
            [
                (graph_kinds::PROP_IDENTIFIER, PropValue::Str(String::from("kernel"))),
                (graph_kinds::PROP_MODULE_INDEX, PropValue::U64(3)),
                (graph_kinds::PROP_BASE_PHYS, PropValue::U64(0x1000)),
                (graph_kinds::PROP_SIZE, PropValue::U64(0x2000)),
            ]
        );

        let roundtrip = ProgramImage::from_props(image.id, &to_prop_slice(&props));
        assert_eq!(roundtrip.identifier, image.identifier);
        assert_eq!(roundtrip.module_index, image.module_index);
        assert_eq!(roundtrip.base_phys, image.base_phys);
        assert_eq!(roundtrip.size, image.size);
        let mut roundtrip_props = Vec::new();
        roundtrip.to_props(&mut roundtrip_props);
        assert_eq!(roundtrip_props, props);
    }

    #[test]
    fn display_present_request_roundtrip_props() {
        let request = DisplayPresentRequest {
            id: ThingId(0x1234),
            framebuffer_id: ThingId(0x99),
            frame_index: 5,
            requested_at_ns: 1_000,
            presented_at_ns: Some(2_000),
            completed: true,
        };

        let mut props = Vec::new();
        request.to_props(&mut props);
        assert!(props.contains(&(
            graph_kinds::PROP_PRESENTED_AT_NS,
            PropValue::U64(2_000)
        )));

        let roundtrip = DisplayPresentRequest::from_props(request.id, &to_prop_slice(&props));
        assert_eq!(roundtrip.framebuffer_id, request.framebuffer_id);
        assert_eq!(roundtrip.frame_index, request.frame_index);
        assert_eq!(roundtrip.requested_at_ns, request.requested_at_ns);
        assert_eq!(roundtrip.presented_at_ns, request.presented_at_ns);
        assert_eq!(roundtrip.completed, request.completed);
        let mut roundtrip_props = Vec::new();
        roundtrip.to_props(&mut roundtrip_props);
        assert_eq!(roundtrip_props, props);
    }
}

impl AlarmRequest {
    pub fn create_pending(
        target_unix_seconds: i64,
        target_unix_nanos: u32,
        owner_process: ThingId,
        owner_thread: ThingId,
    ) -> [(PropKey, PropValue); 5] {
        [
            ("target_unix_seconds", PropValue::I64(target_unix_seconds)),
            (
                "target_unix_nanos",
                PropValue::U64(target_unix_nanos as u64),
            ),
            ("owner_process", PropValue::U64(owner_process.0)),
            ("owner_thread", PropValue::U64(owner_thread.0)),
            ("state", PropValue::Str(String::from("Pending"))),
        ]
    }

    pub fn arm_props(target_ticks: u64) -> [(PropKey, PropValue); 2] {
        [
            ("state", PropValue::Str(String::from("Armed"))),
            ("target_ticks", PropValue::U64(target_ticks)),
        ]
    }

    pub fn fired_props() -> [(PropKey, PropValue); 1] {
        [("state", PropValue::Str(String::from("Fired")))]
    }

    pub fn cancel_props() -> [(PropKey, PropValue); 1] {
        [("state", PropValue::Str(String::from("Cancelled")))]
    }
}

pub struct AlarmEvent {
    pub id: ThingId,
    pub alarm_id: ThingId,
    pub fired_unix_seconds: i64,
    pub fired_unix_nanos: u32,
}

impl Thing for AlarmEvent {
    const KIND: &'static str = "AlarmEvent";
    const DESCRIPTION: &'static str = "Recorded firing of a kernel-backed alarm.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push(("alarm_id", PropValue::U64(self.alarm_id.0)));
        out.push((
            "fired_unix_seconds",
            PropValue::I64(self.fired_unix_seconds),
        ));
        out.push((
            "fired_unix_nanos",
            PropValue::U64(self.fired_unix_nanos as u64),
        ));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut alarm_id = ThingId(0);
        let mut fired_unix_seconds = 0_i64;
        let mut fired_unix_nanos = 0_u32;

        for prop in props.iter().flatten() {
            match prop.0 {
                "alarm_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        alarm_id = ThingId(v);
                    }
                }
                "fired_unix_seconds" => {
                    if let PropValue::I64(v) = prop.1 {
                        fired_unix_seconds = v;
                    }
                }
                "fired_unix_nanos" => {
                    if let PropValue::U64(v) = prop.1 {
                        fired_unix_nanos = v as u32;
                    }
                }
                _ => {}
            }
        }

        AlarmEvent {
            id,
            alarm_id,
            fired_unix_seconds,
            fired_unix_nanos,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("alarm_id", PropType::U64),
            ("fired_unix_seconds", PropType::I64),
            ("fired_unix_nanos", PropType::U64),
        ]
    }
}

impl AlarmEvent {
    pub fn from_fire(
        alarm_id: ThingId,
        fired_unix_seconds: i64,
        fired_unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        [
            ("alarm_id", PropValue::U64(alarm_id.0)),
            ("fired_unix_seconds", PropValue::I64(fired_unix_seconds)),
            ("fired_unix_nanos", PropValue::U64(fired_unix_nanos as u64)),
        ]
    }
}
