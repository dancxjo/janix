use abi::symbols::sym;
use abi::{SymbolId, ThingId};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub trait ThingPayload: Serialize + DeserializeOwned {
    const KIND: SymbolId;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ByteSpace {
    pub len: u64,
    pub flags: u32,        // READ/WRITE/EXEC/COW/etc
    pub backing: SymbolId, // "anonymous", "module", "bitmap", "font", "framebuffer"
}

impl ThingPayload for ByteSpace {
    const KIND: SymbolId = sym("core.bytespace");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Region {
    pub offset: u64,
    pub len: u64,
}
impl ThingPayload for Region { const KIND: SymbolId = sym("core.region"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Image2D {
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub pixel_format: SymbolId,
}
impl ThingPayload for Image2D { const KIND: SymbolId = sym("core.image2d"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Ring {
    pub capacity: u64,
    pub head_off: u64,
    pub tail_off: u64,
    pub elem_size: u32,
}
impl ThingPayload for Ring { const KIND: SymbolId = sym("core.ring"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Stream {
    pub cursor: u64,
    pub mode: u32,         // READ/WRITE/APPEND/RING
}

impl ThingPayload for Stream {
    const KIND: SymbolId = sym("core.stream");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Module {
    pub name: SymbolId,
    pub image_len: u64,
    pub image_kind: SymbolId, // "elf"
    pub entry: u64,           // optional but keep if already known at ingest time
}

impl ThingPayload for Module {
    const KIND: SymbolId = sym("core.module");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    pub format: SymbolId,
}

impl ThingPayload for Bitmap {
    const KIND: SymbolId = sym("core.bitmap");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Font {
    pub family: SymbolId,
    pub style: SymbolId,
    pub weight: u16,
}

impl ThingPayload for Font {
    const KIND: SymbolId = sym("core.font");
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dir {
    pub name: alloc::string::String,
}
impl ThingPayload for Dir { const KIND: SymbolId = sym("core.dir"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct File {
    pub name: alloc::string::String,
    pub size: u64,
}
impl ThingPayload for File { const KIND: SymbolId = sym("core.file"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Mount {
    pub path: alloc::string::String,
    pub readonly: bool,
}
impl ThingPayload for Mount { const KIND: SymbolId = sym("core.mount"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Process {
    pub pid: u64,
    pub name: SymbolId,
    pub state: u32,
}
impl ThingPayload for Process { const KIND: SymbolId = sym("core.process"); }

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BootProgram {
    pub name: alloc::string::String,
    pub entry: u64,
}
impl ThingPayload for BootProgram { const KIND: SymbolId = sym("core.boot_program"); }

// Predicates
pub const HAS_BYTES: SymbolId = sym("core.has_bytes");
pub const HAS_PIXELS: SymbolId = sym("core.has_pixels");
pub const HAS_FONT_DATA: SymbolId = sym("core.has_font_data");
pub const BACKED_BY: SymbolId = sym("core.backed_by");

pub const IN: SymbolId = sym("core.in");
pub const DATA: SymbolId = sym("core.data");
pub const HAS_VIEW: SymbolId = sym("core.has_view");

// Other Predicates (migrated from ThingId)
pub const HAS_ENTRY: SymbolId = sym("core.has_entry");
pub const MOUNTS: SymbolId = sym("core.mounts");
pub const HAS_MOUNT: SymbolId = sym("core.has_mount");
pub const SPAWNED: SymbolId = sym("core.spawned");
pub const RUNS: SymbolId = sym("core.runs");
pub const HAS_MODULE: SymbolId = sym("core.has_module");
