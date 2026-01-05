#![no_std]

extern crate alloc;
extern crate abi;
extern crate thing_codec;
extern crate thing_derive;

use serde::{Serialize, Deserialize};
use thing_derive::Thing;
pub use abi::ids::{SymbolId, ThingId}; 
pub use thing_codec::Thing;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.DisplayDevice")]
pub struct DisplayDevice {
    pub framebuffer: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub refresh_hz: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Framebuffer")]
pub struct Framebuffer {
    pub bytespace: ThingId,
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Place")]
pub struct Place {
    pub name: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Relationship")]
pub struct Relationship {
    pub from: ThingId,
    pub to: ThingId,
    pub predicate: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Surface")]
pub struct Surface {
    pub width: u32,
    pub height: u32,
    pub stride_bytes: u32,
    pub format: SymbolId,
    pub bytespace: ThingId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WindowStyle {
    pub bg_rgba: u32,
    pub radius: u16,
    pub shadow: u8, // 0=none, 1=drop
    pub elevation: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Window")]
pub struct Window {
    pub title: SymbolId,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub style: WindowStyle,
    pub content_root: ThingId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.PointerState")]
pub struct Pointer {
    pub stream: ThingId,
    pub x: i32,
    pub y: i32,
    pub buttons: u32,
    pub updated_at_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.MouseStream")]
pub struct MouseStream {
    pub bytespace: ThingId,
    pub capacity: u32,
    pub sample_size: u32,
    pub write_index: u32,
    pub dropped: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.EventStream")]
pub struct EventStream {
    pub bytespace: ThingId,
    pub capacity_bytes: u32,
    pub max_record_bytes: u32,
    pub flags: u64,
    pub name: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Module")]
pub struct Module {
    pub bytespace: ThingId,
    pub size: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Service")]
pub struct Service {
    pub pid: u64,
    pub state: u32, // 0: Init, 1: Ready, 2: Failed
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Bytespace")]
pub struct Bytespace {
    pub len: u64,
    pub flags: u32, 
    pub _pad: u32,
    pub phys_base: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.RtcDevice")]
pub struct RtcDevice {
    pub source: SymbolId,
    pub accuracy_ns: u64,
    pub base_seconds: u64,
    pub base_mono_ns: u64,
    pub flags: u64,
    pub _pad: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.MonotonicClock")]
pub struct MonotonicClock {
    pub now_ns: u64,
    pub resolution_ns: u64,
    pub source: SymbolId,
    pub last_update_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.SystemClock")]
pub struct SystemClock {
    pub unix_epoch_ns: i64,
    pub status: u64,
    pub _pad: u64,
    pub last_set_mono_ns: u64,
    pub accuracy_ns: u64,
    pub source: SymbolId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.TimeOffset")]
pub struct TimeOffset {
    pub offset_ns: i64,
    pub rate_ppb: i64,
    pub updated_mono_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.PciFunction")]
pub struct PciFunctionV1 {
      pub seg: u16,
      pub bus: u8,
      pub dev: u8,
      pub fun: u8,
      pub vendor_id: u16,
      pub device_id: u16,
      pub class_code: u8,
      pub subclass: u8,
      pub prog_if: u8,
      pub revision_id: u8,
      pub header_type: u8,
      pub bars: [PciBar; 6],
      pub irq_pin: u8,
      pub irq_line: u8,
      pub caps_found: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PciBar {
      Unused,
      Io { addr: u32 },
      Mmio32 { addr: u32, prefetch: bool },
      Mmio64 { addr: u64, prefetch: bool },
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PciCapability {
    pub id: u8,
    pub offset: u8,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.PciEcam")]
pub struct PciEcamV1 {
      pub seg: u16,
      pub bus_start: u8,
      pub bus_end: u8,
      pub _pad: u8,
      pub phys_base: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.HardwareResource")]
pub struct HardwareInfo {
    pub name: SymbolId,
    pub resource_type: SymbolId,
    pub start: u64,
    pub end: u64,
    pub irq: u32,
    pub _pad: u32,
}

pub type DisplayInfo = DisplayDevice;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutKind {
    Column,
    Row,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Layout")]
pub struct Layout {
    pub kind: LayoutKind,
    pub padding: u16,
    pub gap: u16,
    pub align: u8, // 0=Start, 1=Center, 2=End
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextStyle {
    pub size: u16,
    pub color_rgba: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Label")]
pub struct Label {
    pub text: SymbolId,
    pub style: TextStyle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonStyle {
    pub bg_rgba: u32,
    pub radius: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Button")]
pub struct Button {
    pub text: SymbolId,
    pub style: ButtonStyle,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Frame")]
pub struct Frame {
    pub window: ThingId,
    pub seq: u64,
}
