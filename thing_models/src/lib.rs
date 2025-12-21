#![no_std]

extern crate alloc;
extern crate self as thing_models;

pub mod display;
pub mod input;
pub mod io;
pub mod ui;
pub mod usb;
pub mod kernel;
pub mod graph_kinds;
pub mod props;

pub use crate::props::{PropKey, PropType, PropValue};
use thing_macros::Thing;
use abi::{ThingId, syscall_defs::SymbolId};
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchedThreadInfo {
    pub tid: u64,
    pub state: u64,
    pub priority: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaRegistryOutcome {
    Created,
    AlreadyRegisteredSame,
    Conflict,
}

pub trait Thing: Sized {
    const KIND: &'static str; // High level string, wrapper must intern
    const DESCRIPTION: &'static str;

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>);
    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self;

    /// Static schema for this Thing, used for registration.
    fn schema() -> &'static [(&'static str, PropType)];

    /// Get the description for this Thing instance, falling back to the type description.
    /// This can be overridden to check for an instance-specific "description" property.
    fn get_description(&self) -> &'static str {
        Self::DESCRIPTION
    }
}
pub use display::*;
pub use input::*;
pub use io::*;
pub use ui::*;
pub use usb::*;
pub use kernel::*;

/// Returns the complete list of core schemas that the kernel MUST register at boot.
/// 
/// This list is the "Single Source of Truth" for system core types.
/// The kernel should iterate this list and register each schema.
pub fn kernel_core_schemas() -> Vec<(&'static str, &'static str, &'static [(&'static str, PropType)])> {
    let mut schemas = Vec::new();

    // 1. Kernel Internals
    schemas.push((PhysFrame::KIND, PhysFrame::DESCRIPTION, PhysFrame::schema()));
    schemas.push((FramePool::KIND, FramePool::DESCRIPTION, FramePool::schema()));
    schemas.push((AddressSpace::KIND, AddressSpace::DESCRIPTION, AddressSpace::schema()));
    schemas.push((VirtRegion::KIND, VirtRegion::DESCRIPTION, VirtRegion::schema()));
    schemas.push((Process::KIND, Process::DESCRIPTION, Process::schema()));
    schemas.push((Thread::KIND, Thread::DESCRIPTION, Thread::schema()));
    schemas.push((ThreadInfo::KIND, ThreadInfo::DESCRIPTION, ThreadInfo::schema()));
    schemas.push((CpuCore::KIND, CpuCore::DESCRIPTION, CpuCore::schema()));
    schemas.push((SleepEvent::KIND, SleepEvent::DESCRIPTION, SleepEvent::schema()));

    // 2. Boot & System
    schemas.push((BootProfile::KIND, BootProfile::DESCRIPTION, BootProfile::schema()));
    schemas.push((BootProgram::KIND, BootProgram::DESCRIPTION, BootProgram::schema()));
    schemas.push((ProgramImage::KIND, ProgramImage::DESCRIPTION, ProgramImage::schema()));
    schemas.push((FontModule::KIND, FontModule::DESCRIPTION, FontModule::schema()));
    schemas.push((TimeSource::KIND, TimeSource::DESCRIPTION, TimeSource::schema()));
    schemas.push((graph_kinds::KIND_IO_PORT_REGION, IoPortRegion::DESCRIPTION, IoPortRegion::schema()));
    schemas.push((graph_kinds::KIND_IO_PORT_OP, IoPortOp::DESCRIPTION, IoPortOp::schema()));
    schemas.push((graph_kinds::KIND_INTERRUPT_EVENT, InterruptEvent::DESCRIPTION, InterruptEvent::schema()));
    schemas.push((graph_kinds::KIND_INTERRUPT_REQUEST, InterruptRequest::DESCRIPTION, InterruptRequest::schema()));
    schemas.push((graph_kinds::KIND_ALARM_REQUEST, AlarmRequest::DESCRIPTION, AlarmRequest::schema()));
    schemas.push((graph_kinds::KIND_ALARM_EVENT, AlarmEvent::DESCRIPTION, AlarmEvent::schema()));
    schemas.push((BlockDevice::KIND, BlockDevice::DESCRIPTION, BlockDevice::schema()));

    // 3. Display Subsystem
    schemas.push((Display::KIND, Display::DESCRIPTION, Display::schema()));
    schemas.push((SharedBuffer::KIND, SharedBuffer::DESCRIPTION, SharedBuffer::schema()));
    schemas.push((DisplayFramebuffer::KIND, DisplayFramebuffer::DESCRIPTION, DisplayFramebuffer::schema()));
    schemas.push((DisplayFrame::KIND, DisplayFrame::DESCRIPTION, DisplayFrame::schema()));
    schemas.push((DisplayPresentRequest::KIND, DisplayPresentRequest::DESCRIPTION, DisplayPresentRequest::schema()));

    // 4. Shared UI Contract (Windowing)
    schemas.push((Mode::KIND, Mode::DESCRIPTION, Mode::schema()));
    schemas.push((ModeSwitchEvent::KIND, ModeSwitchEvent::DESCRIPTION, ModeSwitchEvent::schema()));
    schemas.push((Place::KIND, Place::DESCRIPTION, Place::schema()));
    schemas.push((Window::KIND, Window::DESCRIPTION, Window::schema()));
    schemas.push((Surface::KIND, Surface::DESCRIPTION, Surface::schema()));

    schemas
}

#[derive(Thing)]
#[thing(description = "The system-wide boot configuration used by init to launch all services and programs.")]
pub struct BootProfile {
    pub id: ThingId,
    pub version: u64,
}

#[derive(Thing)]
#[thing(description = "A program to be launched automatically by init during system boot.")]
pub struct BootProgram {
    pub id: ThingId,
    pub name: String,
    pub app_id: u64,
    pub priority: u64,
    pub binary: String,
    pub respawn_policy: String,
}

pub struct RawModule {
    pub id: ThingId,
    pub identifier: String,
    pub raw_kind: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
    pub framebuffer_id: Option<ThingId>,
}

impl Thing for RawModule {
    const KIND: &'static str = graph_kinds::KIND_RAW_MODULE;
    const DESCRIPTION: &'static str = "Raw data module loaded at boot";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        out.push((graph_kinds::PROP_IDENTIFIER.to_string(),
            PropValue::Str(self.identifier.clone()),
        ));
        out.push((graph_kinds::PROP_RAW_KIND.to_string(),
            PropValue::Str(self.raw_kind.clone()),
        ));
        out.push((graph_kinds::PROP_MODULE_INDEX.to_string(),
            PropValue::U64(self.module_index),
        ));
        out.push((graph_kinds::PROP_BASE_PHYS.to_string(), PropValue::U64(self.base_phys)));
        out.push((graph_kinds::PROP_SIZE.to_string(), PropValue::U64(self.size)));
        if let Some(fid) = self.framebuffer_id {
            out.push((graph_kinds::PROP_FRAMEBUFFER_ID.to_string(), PropValue::U64(fid.0)));
        }
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut identifier = String::new();
        let mut raw_kind = String::new();
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;
        let mut framebuffer_id = None;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                graph_kinds::PROP_IDENTIFIER => {
                    if let PropValue::Str(v) = &prop.1 {
                        identifier = v.clone();
                    }
                }
                graph_kinds::PROP_RAW_KIND => {
                    if let PropValue::Str(v) = &prop.1 {
                        raw_kind = v.clone();
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
                graph_kinds::PROP_FRAMEBUFFER_ID => {
                    if let PropValue::U64(v) = prop.1 {
                        framebuffer_id = Some(ThingId(v));
                    }
                }
                _ => {}
            }
        }

        RawModule {
            id,
            identifier,
            raw_kind,
            module_index,
            base_phys,
            size,
            framebuffer_id,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            (graph_kinds::PROP_IDENTIFIER, PropType::Str),
            (graph_kinds::PROP_RAW_KIND, PropType::Str),
            (graph_kinds::PROP_MODULE_INDEX, PropType::U64),
            (graph_kinds::PROP_BASE_PHYS, PropType::U64),
            (graph_kinds::PROP_SIZE, PropType::U64),
            (graph_kinds::PROP_FRAMEBUFFER_ID, PropType::U64),
        ]
    }
}

#[derive(Thing)]
#[thing(description = "An ELF program image discovered at boot and available for loading.")]
pub struct ProgramImage {
    pub id: ThingId,
    pub identifier: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
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
        out.push((graph_kinds::PROP_FONT_NAME.to_string(),
            PropValue::Str(self.name.clone()),
        ));
        out.push((graph_kinds::PROP_MODULE_INDEX.to_string(),
            PropValue::U64(self.module_index),
        ));
        out.push((graph_kinds::PROP_BASE_PHYS.to_string(), PropValue::U64(self.base_phys)));
        out.push((graph_kinds::PROP_SIZE.to_string(), PropValue::U64(self.size)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut name = String::new();
        let mut module_index = 0;
        let mut base_phys = 0;
        let mut size = 0;
        for prop in props.iter().flatten() {
            match prop.0.as_str() {
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

#[derive(Thing)]
#[thing(description = "Kernel-published system clock including monotonic tick counter and Unix wall time.")]
pub struct TimeSource {
    pub id: ThingId,
    pub ticks_since_boot: u64,
    pub tick_hz: u32,
    pub unix_seconds: i64,
    pub unix_nanos: u32,
}

impl TimeSource {
    pub fn create(tick_hz: u32, unix_seconds: i64, unix_nanos: u32) -> [(PropKey, PropValue); 4] {
        [
            ("tick_hz".to_string(), PropValue::U64(tick_hz as u64)),
            ("ticks_since_boot".to_string(), PropValue::U64(0)),
            ("unix_seconds".to_string(), PropValue::I64(unix_seconds)),
            ("unix_nanos".to_string(), PropValue::U64(unix_nanos as u64)),
        ]
    }

    pub fn update_from_kernel(
        ticks_since_boot: u64,
        unix_seconds: i64,
        unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        [
            ("ticks_since_boot".to_string(), PropValue::U64(ticks_since_boot)),
            ("unix_seconds".to_string(), PropValue::I64(unix_seconds)),
            ("unix_nanos".to_string(), PropValue::U64(unix_nanos as u64)),
        ]
    }
}

pub struct AlarmRequest {
    pub id: ThingId,
    pub time_source_id: Option<ThingId>,
    pub target_unix_seconds: i64,
    pub target_unix_nanos: u32,
    pub target_ticks: Option<u64>,
    pub period_ticks: Option<u64>,
    pub owner_process: ThingId,
    pub owner_thread: ThingId,
    pub armed: bool,
    pub fired: bool,
}

impl Thing for AlarmRequest {
    const KIND: &'static str = "AlarmRequest";
    const DESCRIPTION: &'static str =
        "User-requested alarm mapped to kernel tick space and lifecycle state.";

    fn to_props(&self, out: &mut Vec<(PropKey, PropValue)>) {
        if let Some(ts_id) = self.time_source_id {
            out.push(("time_source_id".to_string(), PropValue::U64(ts_id.0)));
        }
        out.push(("target_unix_seconds".to_string(),
            PropValue::I64(self.target_unix_seconds),
        ));
        out.push(("target_unix_nanos".to_string(),
            PropValue::U64(self.target_unix_nanos as u64),
        ));
        if let Some(ticks) = self.target_ticks {
            out.push(("target_ticks".to_string(), PropValue::U64(ticks)));
        }
        if let Some(period) = self.period_ticks {
            out.push(("period_ticks".to_string(), PropValue::U64(period)));
        }
        out.push(("owner_process".to_string(), PropValue::U64(self.owner_process.0)));
        out.push(("owner_thread".to_string(), PropValue::U64(self.owner_thread.0)));
        out.push(("armed".to_string(), PropValue::Bool(self.armed)));
        out.push(("fired".to_string(), PropValue::Bool(self.fired)));
    }

    fn from_props(id: ThingId, props: &[Option<(PropKey, PropValue)>]) -> Self {
        let mut time_source_id = None;
        let mut target_unix_seconds = 0_i64;
        let mut target_unix_nanos = 0_u32;
        let mut target_ticks = None;
        let mut period_ticks = None;
        let mut owner_process = ThingId(0);
        let mut owner_thread = ThingId(0);
        let mut armed = false;
        let mut fired = false;

        for prop in props.iter().flatten() {
            match prop.0.as_str() {
                "time_source_id" => {
                    if let PropValue::U64(v) = prop.1 {
                        time_source_id = Some(ThingId(v));
                    }
                }
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
                "target_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        target_ticks = Some(v);
                    }
                }
                "period_ticks" => {
                    if let PropValue::U64(v) = prop.1 {
                        period_ticks = Some(v);
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
                "armed" => {
                    if let PropValue::Bool(v) = prop.1 {
                        armed = v;
                    }
                }
                "fired" => {
                    if let PropValue::Bool(v) = prop.1 {
                        fired = v;
                    }
                }
                _ => {}
            }
        }

        AlarmRequest {
            id,
            time_source_id,
            target_unix_seconds,
            target_unix_nanos,
            target_ticks,
            period_ticks,
            owner_process,
            owner_thread,
            armed,
            fired,
        }
    }

    fn schema() -> &'static [(&'static str, PropType)] {
        &[
            ("time_source_id", PropType::U64),
            ("target_unix_seconds", PropType::I64),
            ("target_unix_nanos", PropType::U64),
            ("target_ticks", PropType::U64),
            ("period_ticks", PropType::U64),
            ("owner_process", PropType::U64),
            ("owner_thread", PropType::U64),
            ("armed", PropType::Bool),
            ("fired", PropType::Bool),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_kinds;
    use alloc::{string::String, vec::Vec};

    fn to_prop_slice(props: &[(PropKey, PropValue)]) -> Vec<Option<(PropKey, PropValue)>> {
        props.iter().map(|(k, v)| Some((k.clone(), v.clone()))).collect()
    }

    #[test]
    fn boot_program_roundtrip_props() {
        let boot = BootProgram {
            id: ThingId(0x00_1),
            name: String::from("init"),
            app_id: 0x42,
            priority: 1,
            binary: String::from("/bin/init"),
            respawn_policy: String::from("Always"),
        };

        let mut props = Vec::new();
        boot.to_props(&mut props);
        assert_eq!(
            props,
            [
                ("name".to_string(), PropValue::Str(String::from("init"))),
                ("app_id".to_string(), PropValue::U64(0x42)),
                ("priority".to_string(), PropValue::U64(1)),
                ("binary".to_string(), PropValue::Str(String::from("/bin/init"))),
                (graph_kinds::PROP_RESPAWN_POLICY.to_string(), PropValue::Str(String::from("Always"))),
            ]
        );

        let roundtrip = BootProgram::from_props(boot.id, &to_prop_slice(&props));
        assert_eq!(roundtrip.id, boot.id);
        assert_eq!(roundtrip.name, boot.name);
        assert_eq!(roundtrip.app_id, boot.app_id);
        assert_eq!(roundtrip.priority, boot.priority);
        assert_eq!(roundtrip.binary, boot.binary);
        assert_eq!(roundtrip.respawn_policy, boot.respawn_policy);
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
                (
                    graph_kinds::PROP_IDENTIFIER.to_string(),
                    PropValue::Str(String::from("kernel"))
                ),
                (graph_kinds::PROP_MODULE_INDEX.to_string(), PropValue::U64(3)),
                (graph_kinds::PROP_BASE_PHYS.to_string(), PropValue::U64(0x1000)),
                (graph_kinds::PROP_SIZE.to_string(), PropValue::U64(0x2000)),
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
        assert!(props.contains(&(graph_kinds::PROP_PRESENTED_AT_NS.to_string(), PropValue::U64(2_000))));

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
    ) -> [(PropKey, PropValue); 6] {
        [
            ("target_unix_seconds".to_string(), PropValue::I64(target_unix_seconds)),
            (
                "target_unix_nanos".to_string(),
                PropValue::U64(target_unix_nanos as u64),
            ),
            ("owner_process".to_string(), PropValue::U64(owner_process.0)),
            ("owner_thread".to_string(), PropValue::U64(owner_thread.0)),
            ("armed".to_string(), PropValue::Bool(false)),
            ("fired".to_string(), PropValue::Bool(false)),
        ]
    }

    pub fn arm_props(target_ticks: u64) -> [(PropKey, PropValue); 2] {
        [
            ("armed".to_string(), PropValue::Bool(true)),
            ("target_ticks".to_string(), PropValue::U64(target_ticks)),
        ]
    }

    pub fn fired_props() -> [(PropKey, PropValue); 2] {
        [
            ("armed".to_string(), PropValue::Bool(false)),
            ("fired".to_string(), PropValue::Bool(true)),
        ]
    }

    pub fn cancel_props() -> [(PropKey, PropValue); 2] {
        [
            ("armed".to_string(), PropValue::Bool(false)),
            ("fired".to_string(), PropValue::Bool(false)),
        ]
    }
}

#[derive(Thing)]
#[thing(description = "Recorded firing of a kernel-backed alarm.")]
pub struct AlarmEvent {
    pub id: ThingId,
    pub alarm_id: ThingId,
    pub fired_unix_seconds: i64,
    pub fired_unix_nanos: u32,
}

impl AlarmEvent {
    pub fn from_fire(
        alarm_id: ThingId,
        fired_unix_seconds: i64,
        fired_unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        [
            ("alarm_id".to_string(), PropValue::U64(alarm_id.0)),
            ("fired_unix_seconds".to_string(), PropValue::I64(fired_unix_seconds)),
            ("fired_unix_nanos".to_string(), PropValue::U64(fired_unix_nanos as u64)),
        ]
    }
}
