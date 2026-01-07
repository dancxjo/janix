#![no_std]

extern crate alloc;
extern crate abi;
extern crate thing_codec;
extern crate thing_derive;

use abi::cap::CapOp;
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
#[thing(kind = "kind.Graph")]
pub struct Graph {
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
    pub name: SymbolId,
    pub bytespace: ThingId,
    pub size: u64,
    pub caps: [CapOp; 8],
    pub cap_count: u8,
    pub deps: [SymbolId; 8],
    pub dep_count: u8,
    pub _pad: u16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Service")]
pub struct Service {
    pub name: SymbolId,
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
#[thing(kind = "kind.Canvas")]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub bytespace: ThingId,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.DrawList")]
pub struct DrawList {
    pub width: u32,
    pub height: u32,
    pub bytespace: ThingId,
    pub cmd_count: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.Frame")]
pub struct Frame {
    pub window: ThingId,
    pub seq: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.PointerEvent")]
pub struct PointerEvent {
    pub phase: abi::ui::PointerPhase,
    pub x: i32,
    pub y: i32,
    pub buttons: u32,
    pub modifiers: u32,
    pub timestamp_ns: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.WindowAction")]
pub struct WindowAction {
    pub window: ThingId,
    pub kind: abi::ui::WindowActionKind,
    pub start_x: i32,
    pub start_y: i32,
    pub dx: i32,
    pub dy: i32,
    pub edges: abi::ui::ResizeEdge,
}

// ============================================================================
// Text Rendering Service Types
// ============================================================================

/// Status of a text render request
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextRenderStatus {
    Pending = 0,
    Rendering = 1,
    Ready = 2,
    Error = 3,
}

/// A font family (e.g. "NotoSans" containing Regular, Bold, etc.)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.FontFamily")]
pub struct FontFamily {
    pub name: SymbolId,
    pub variant_count: u8,
    pub _pad: [u8; 7],
}

/// A specific font variant (e.g. NotoSans-Regular)
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.FontFace")]
pub struct FontFace {
    pub name: SymbolId,           // e.g. "font.NotoSans.Regular"
    pub family: ThingId,          // Link to FontFamily
    pub style: u8,                // 0=Regular, 1=Bold, 2=Italic, 3=BoldItalic
    pub weight: u16,              // 400=regular, 700=bold
    pub units_per_em: u16,
    pub ascender: i16,
    pub descender: i16,
    pub source_asset: ThingId,    // Original TTF asset
    pub _pad: u16,
}

/// Glyph cache for a (FontFace, px_size) pair - glyphs stored in atlas bytespace
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.GlyphCache")]
pub struct GlyphCache {
    pub font_face: ThingId,
    pub px_size: u16,
    pub format: u8,               // 0=A8
    pub glyph_count: u16,
    pub ascent: i16,
    pub descent: i16,
    pub atlas_bytespace: ThingId, // A8 glyph atlas
    pub index_bytespace: ThingId, // codepoint -> (offset, metrics)
    pub _pad: u8,
}

/// Request from Bloom to textd for composed text rendering
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.TextRenderRequest")]
pub struct TextRenderRequest {
    pub request_id: u64,
    pub text_symbol: SymbolId,    // Interned text content
    pub font_face: ThingId,       // Link to FontFace
    pub px_size: u16,
    pub max_width_px: u16,        // 0 = no wrap
    pub status: TextRenderStatus,
    pub error_code: u16,
    pub result: ThingId,          // Link to TextRenderResult when Ready
}

/// Composed text rendering result
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Thing, Serialize, Deserialize)]
#[thing(kind = "kind.TextRenderResult")]
pub struct TextRenderResult {
    pub width_px: u16,
    pub height_px: u16,
    pub baseline_y: u16,
    pub bytespace: ThingId,       // Composed A8 mask
    pub _pad: u16,
}
