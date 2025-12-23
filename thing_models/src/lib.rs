#![no_std]

extern crate alloc;
extern crate self as thing_models;

pub mod display;
pub mod graph_kinds;
pub mod input;
pub mod io;
pub mod kernel;
pub mod props;
pub mod ui;
pub mod usb;

pub use crate::props::{PropKey, PropType, PropValue};
use abi::{ThingId, syscall_defs::SymbolId};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use thing_macros::Thing;

// Re-export ABI types to avoid duplication
pub use abi::{SchedThreadInfo, SchemaRegistryOutcome};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SchemaId(pub u64);

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
pub use kernel::*;
pub use ui::*;
pub use usb::*;

/// Returns the complete list of core schemas that the kernel MUST register at boot.
///
/// This list is the "Single Source of Truth" for system core types.
/// The kernel should iterate this list and register each schema.
pub fn kernel_core_schemas() -> Vec<(
    &'static str,
    &'static str,
    &'static [(&'static str, PropType)],
)> {
    let mut schemas = Vec::new();

    // 1. Kernel Internals
    schemas.push((PhysFrame::KIND, PhysFrame::DESCRIPTION, PhysFrame::schema()));
    schemas.push((FramePool::KIND, FramePool::DESCRIPTION, FramePool::schema()));
    schemas.push((
        AddressSpace::KIND,
        AddressSpace::DESCRIPTION,
        AddressSpace::schema(),
    ));
    schemas.push((
        VirtRegion::KIND,
        VirtRegion::DESCRIPTION,
        VirtRegion::schema(),
    ));
    schemas.push((Process::KIND, Process::DESCRIPTION, Process::schema()));
    schemas.push((Thread::KIND, Thread::DESCRIPTION, Thread::schema()));
    schemas.push((
        ThreadInfo::KIND,
        ThreadInfo::DESCRIPTION,
        ThreadInfo::schema(),
    ));
    schemas.push((CpuCore::KIND, CpuCore::DESCRIPTION, CpuCore::schema()));
    schemas.push((
        SleepEvent::KIND,
        SleepEvent::DESCRIPTION,
        SleepEvent::schema(),
    ));

    // 2. Boot & System
    schemas.push((
        BootProfile::KIND,
        BootProfile::DESCRIPTION,
        BootProfile::schema(),
    ));
    schemas.push((
        BootProgram::KIND,
        BootProgram::DESCRIPTION,
        BootProgram::schema(),
    ));
    schemas.push((
        ProgramImage::KIND,
        ProgramImage::DESCRIPTION,
        ProgramImage::schema(),
    ));
    schemas.push((
        FontModule::KIND,
        FontModule::DESCRIPTION,
        FontModule::schema(),
    ));
    schemas.push((
        TimeSource::KIND,
        TimeSource::DESCRIPTION,
        TimeSource::schema(),
    ));
    schemas.push((
        UnixTime::KIND,
        UnixTime::DESCRIPTION,
        UnixTime::schema(),
    ));
    schemas.push((
        ClockTime::KIND,
        ClockTime::DESCRIPTION,
        ClockTime::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_IO_PORT_REGION,
        IoPortRegion::DESCRIPTION,
        IoPortRegion::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_IO_PORT_OP,
        IoPortOp::DESCRIPTION,
        IoPortOp::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_INTERRUPT_EVENT,
        InterruptEvent::DESCRIPTION,
        InterruptEvent::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_INTERRUPT_REQUEST,
        InterruptRequest::DESCRIPTION,
        InterruptRequest::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_ALARM_REQUEST,
        AlarmRequest::DESCRIPTION,
        AlarmRequest::schema(),
    ));
    schemas.push((
        graph_kinds::KIND_ALARM_EVENT,
        AlarmEvent::DESCRIPTION,
        AlarmEvent::schema(),
    ));
    schemas.push((
        BlockDevice::KIND,
        BlockDevice::DESCRIPTION,
        BlockDevice::schema(),
    ));

    // 3. Display Subsystem
    schemas.push((Display::KIND, Display::DESCRIPTION, Display::schema()));
    schemas.push((
        SharedBuffer::KIND,
        SharedBuffer::DESCRIPTION,
        SharedBuffer::schema(),
    ));
    schemas.push((
        DisplayFrame::KIND,
        DisplayFrame::DESCRIPTION,
        DisplayFrame::schema(),
    ));
    schemas.push((
        DisplayFramebuffer::KIND,
        DisplayFramebuffer::DESCRIPTION,
        DisplayFramebuffer::schema(),
    ));
    schemas.push((
        DisplayPresentRequest::KIND,
        DisplayPresentRequest::DESCRIPTION,
        DisplayPresentRequest::schema(),
    ));

    // 4. Shared UI Contract (Windowing)
    schemas.push((Mode::KIND, Mode::DESCRIPTION, Mode::schema()));
    schemas.push((
        ModeSwitchEvent::KIND,
        ModeSwitchEvent::DESCRIPTION,
        ModeSwitchEvent::schema(),
    ));
    schemas.push((Place::KIND, Place::DESCRIPTION, Place::schema()));
    schemas.push((Window::KIND, Window::DESCRIPTION, Window::schema()));
    schemas.push((Surface::KIND, Surface::DESCRIPTION, Surface::schema()));

    schemas
}

#[derive(Thing)]
#[thing(
    description = "The system-wide boot configuration used by init to launch all services and programs."
)]
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

#[derive(Thing)]
#[thing(description = "Raw data module loaded at boot")]
pub struct RawModule {
    pub id: ThingId,
    pub identifier: String,
    pub raw_kind: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
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

#[derive(Thing)]
#[thing(description = "A font payload supplied as a boot module available for UI rendering.")]
pub struct FontModule {
    pub id: ThingId,
    #[thing(rename = "font_name")]
    pub name: String,
    pub module_index: u64,
    pub base_phys: u64,
    pub size: u64,
}

#[derive(Thing)]
#[thing(
    description = "Kernel-published system clock including monotonic tick counter and Unix wall time."
)]
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
            (
                "ticks_since_boot".to_string(),
                PropValue::U64(ticks_since_boot),
            ),
            ("unix_seconds".to_string(), PropValue::I64(unix_seconds)),
            ("unix_nanos".to_string(), PropValue::U64(unix_nanos as u64)),
        ]
    }
}

#[derive(Thing)]
#[thing(
    kind = "Witness.UnixTime",
    description = "Kernel-authored Unix wall clock correlated to kernel tick space."
)]
pub struct UnixTime {
    pub id: ThingId,
    #[thing(rename = graph_kinds::PROP_TICKS_SINCE_BOOT)]
    pub ticks_since_boot: u64,
    #[thing(rename = graph_kinds::PROP_UNIX_SECONDS)]
    pub unix_seconds: i64,
    #[thing(rename = graph_kinds::PROP_UNIX_NANOS)]
    pub unix_nanos: u32,
}

impl UnixTime {
    pub fn create(
        ticks_since_boot: u64,
        unix_seconds: i64,
        unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        [
            (
                graph_kinds::PROP_TICKS_SINCE_BOOT.to_string(),
                PropValue::U64(ticks_since_boot),
            ),
            (
                graph_kinds::PROP_UNIX_SECONDS.to_string(),
                PropValue::I64(unix_seconds),
            ),
            (
                graph_kinds::PROP_UNIX_NANOS.to_string(),
                PropValue::U64(unix_nanos as u64),
            ),
        ]
    }

    pub fn update_from_kernel(
        ticks_since_boot: u64,
        unix_seconds: i64,
        unix_nanos: u32,
    ) -> [(PropKey, PropValue); 3] {
        Self::create(ticks_since_boot, unix_seconds, unix_nanos)
    }
}

#[derive(Thing)]
#[thing(
    kind = "Witness.ClockTime",
    description = "Kernel-authored UTC clock wall time for UI consumption."
)]
pub struct ClockTime {
    pub id: ThingId,
    #[thing(rename = graph_kinds::PROP_TICKS_SINCE_BOOT)]
    pub ticks_since_boot: u64,
    #[thing(rename = graph_kinds::PROP_HOURS)]
    pub hours: u8,
    #[thing(rename = graph_kinds::PROP_MINUTES)]
    pub minutes: u8,
    #[thing(rename = graph_kinds::PROP_SECONDS)]
    pub seconds: u8,
}

impl ClockTime {
    pub fn create(
        ticks_since_boot: u64,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> [(PropKey, PropValue); 4] {
        [
            (
                graph_kinds::PROP_TICKS_SINCE_BOOT.to_string(),
                PropValue::U64(ticks_since_boot),
            ),
            (
                graph_kinds::PROP_HOURS.to_string(),
                PropValue::U64(hours as u64),
            ),
            (
                graph_kinds::PROP_MINUTES.to_string(),
                PropValue::U64(minutes as u64),
            ),
            (
                graph_kinds::PROP_SECONDS.to_string(),
                PropValue::U64(seconds as u64),
            ),
        ]
    }

    pub fn update_from_kernel(
        ticks_since_boot: u64,
        hours: u8,
        minutes: u8,
        seconds: u8,
    ) -> [(PropKey, PropValue); 4] {
        Self::create(ticks_since_boot, hours, minutes, seconds)
    }
}

#[derive(Thing)]
#[thing(description = "User-requested alarm mapped to kernel tick space and lifecycle state.")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph_kinds;
    use alloc::{string::String, vec::Vec};

    fn to_prop_slice(props: &[(PropKey, PropValue)]) -> Vec<Option<(PropKey, PropValue)>> {
        props
            .iter()
            .map(|(k, v)| Some((k.clone(), v.clone())))
            .collect()
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
                (
                    "binary".to_string(),
                    PropValue::Str(String::from("/bin/init"))
                ),
                (
                    graph_kinds::PROP_RESPAWN_POLICY.to_string(),
                    PropValue::Str(String::from("Always"))
                ),
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
                (
                    graph_kinds::PROP_MODULE_INDEX.to_string(),
                    PropValue::U64(3)
                ),
                (
                    graph_kinds::PROP_BASE_PHYS.to_string(),
                    PropValue::U64(0x1000)
                ),
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
}

impl AlarmRequest {
    pub fn create_pending(
        target_unix_seconds: i64,
        target_unix_nanos: u32,
        owner_process: ThingId,
        owner_thread: ThingId,
    ) -> [(PropKey, PropValue); 6] {
        [
            (
                "target_unix_seconds".to_string(),
                PropValue::I64(target_unix_seconds),
            ),
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
            (
                "fired_unix_seconds".to_string(),
                PropValue::I64(fired_unix_seconds),
            ),
            (
                "fired_unix_nanos".to_string(),
                PropValue::U64(fired_unix_nanos as u64),
            ),
        ]
    }
}
