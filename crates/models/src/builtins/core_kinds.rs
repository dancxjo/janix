#![allow(unused_imports)]
use crate::builtins::ids::*;
use crate::builtins::symbols::*;
use crate::builtins::symbols::{SYM_DISPLAY_FRAMEBUFFER, SYM_SCANOUT_BUFFER};
use crate::declare::type_tag;
use crate::declare::type_tag::fnv1a64;
use crate::thing_kind;
// Use the core module types
use crate::core::block::BlockDeviceBody;
use crate::core::buffer::BufferBody;
use crate::core::capability::CapabilityBody;
use crate::core::fs::{DirBody, FileBody, VolumeBody};
use crate::core::input::{KeyEventBody, KeyEventStreamBody, KeyboardBody};
use crate::core::pci::PciDeviceBody;
use crate::core::process::{ProcessBody, ThreadBody};
use crate::core::serial::{LogStreamBody, SerialPortBody};
use crate::core::time::TimeNow;
use crate::core::vgs::{GraphProviderBody, MountBody};
use crate::diag::{ErrorBody, FaultBody, LogEntryBody};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ByteSpaceRef {
    pub id: u64,
    pub len: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ByteSpaceBody {
    pub store_id: u64,
    pub len: u64,
    pub flags: u32,
    pub backing: crate::abi::SymbolId,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct StreamBody {
    pub cursor: u64,
    pub mode: u32,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BootProgramBody {
    pub name: alloc::string::String,
    pub binary: alloc::string::String,
    pub priority: u64,
    pub entry_point: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ModuleBody {
    pub path: alloc::string::String,
    pub size_bytes: u64,
    pub base_phys: u64,
    pub index: u32,
    pub role: alloc::string::String,
    pub mime: alloc::string::String,
    pub kind: alloc::string::String,
    pub sniff: u32,
    pub valid: bool,
    pub bytes: ByteSpaceRef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ProgramImageBody {
    pub format: alloc::string::String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct FontBody {
    pub name: alloc::string::String,
    pub format: alloc::string::String,
    pub glyph_width: u16,
    pub glyph_height: u16,
    pub glyph_count: u32,
    pub bytes: ByteSpaceRef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct BitmapBody {
    pub format: alloc::string::String,
    pub width: u32,
    pub height: u32,
    pub bytes: ByteSpaceRef,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct DisplayFramebufferBody {
    pub width: u64,
    pub height: u64,
    pub pitch: u64,
    pub format: u32,
    pub address: u64,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ScanoutBufferBody {
    pub address: u64,
    pub len: u64,
}

thing_kind! {
    kind TimeNow {
        id: crate::builtins::ids::THING_TIME_NOW_KIND,
        sym: SYM_TIME_NOW,
        version: 1,
        body: TimeNow,
        type_tag: "thingos.TimeNow.v1",
        schema_id: crate::builtins::ids::THING_TIME_NOW_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Process {
        id: crate::builtins::ids::THING_PROCESS_KIND,
        sym: SYM_PROCESS,
        version: 1,
        body: ProcessBody,
        type_tag: "thingos.ProcessBody.v1",
        schema_id: crate::builtins::ids::THING_PROCESS_SCHEMA,
        links {
            predicate THING_OWNS_KIND min 0 max many;
            predicate THING_HAS_CAP_KIND min 0 max many;
        }
    }
}

thing_kind! {
    kind Thread {
        id: crate::builtins::ids::THING_THREAD_KIND,
        sym: SYM_THREAD,
        version: 1,
        body: ThreadBody,
        type_tag: "thingos.ThreadBody.v1",
        schema_id: crate::builtins::ids::THING_THREAD_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Capability {
        id: crate::builtins::ids::THING_CAPABILITY_KIND,
        sym: SYM_CAPABILITY,
        version: 1,
        body: CapabilityBody,
        type_tag: "thingos.CapabilityBody.v1",
        schema_id: crate::builtins::ids::THING_CAPABILITY_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Graph {
        id: crate::builtins::ids::THING_GRAPH_KIND,
        sym: SYM_GRAPH,
        version: 1,
        body: GraphBody,
        type_tag: "thingos.GraphBody.v1",
        schema_id: crate::builtins::ids::THING_GRAPH_SCHEMA,
        links {
            predicate THING_BACKED_BY_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind Mount {
        id: crate::builtins::ids::THING_MOUNT_KIND,
        sym: SYM_MOUNT,
        version: 1,
        body: MountBody,
        type_tag: "thingos.MountBody.v1",
        schema_id: crate::builtins::ids::THING_MOUNT_SCHEMA,
        links {
            predicate THING_MOUNTS_KIND min 1 max 1;
        }
    }
}

thing_kind! {
    kind GraphProvider {
        id: crate::builtins::ids::THING_GRAPH_PROVIDER_KIND,
        sym: SYM_GRAPH_PROVIDER,
        version: 1,
        body: GraphProviderBody,
        type_tag: "thingos.GraphProviderBody.v1",
        schema_id: crate::builtins::ids::THING_GRAPH_PROVIDER_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Buffer {
        id: crate::builtins::ids::THING_BUFFER_KIND,
        sym: SYM_BUFFER,
        version: 1,
        body: BufferBody,
        type_tag: "thingos.BufferBody.v1",
        schema_id: crate::builtins::ids::THING_BUFFER_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Stream {
        id: crate::builtins::ids::THING_STREAM_KIND,
        sym: SYM_STREAM,
        version: 1,
        body: StreamBody,
        type_tag: "thingos.StreamBody.v1",
        schema_id: crate::builtins::ids::THING_STREAM_SCHEMA,
        links {
            predicate THING_BACKED_BY_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind ByteSpace {
        id: crate::builtins::ids::THING_BYTESPACE_KIND,
        sym: SYM_BYTESPACE,
        version: 1,
        body: ByteSpaceBody,
        type_tag: "thingos.ByteSpaceBody.v1",
        schema_id: crate::builtins::ids::THING_BYTESPACE_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Keyboard {
        id: crate::builtins::ids::THING_KEYBOARD_KIND,
        sym: SYM_KEYBOARD,
        version: 1,
        body: KeyboardBody,
        type_tag: "thingos.KeyboardBody.v1",
        schema_id: crate::builtins::ids::THING_KEYBOARD_SCHEMA,
        links {} // meta (0..1) implied by v0 rules
    }
}

thing_kind! {
    kind KeyEvent {
        id: crate::builtins::ids::THING_KEY_EVENT_KIND,
        sym: SYM_KEY_EVENT,
        version: 1,
        body: KeyEventBody,
        type_tag: "thingos.KeyEventBody.v1",
        schema_id: crate::builtins::ids::THING_KEY_EVENT_SCHEMA,
        links {} // meta (0..1) implied by v0 rules
    }
}

thing_kind! {
    kind BootProgram {
        id: crate::builtins::ids::THING_BOOT_PROGRAM_KIND,
        sym: SYM_BOOT_PROGRAM,
        version: 1,
        body: BootProgramBody,
        type_tag: "thingos.BootProgramBody.v1",
        schema_id: crate::builtins::ids::THING_BOOT_PROGRAM_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Module {
        id: crate::builtins::ids::THING_MODULE_KIND,
        sym: SYM_MODULE,
        version: 1,
        body: ModuleBody,
        type_tag: "thingos.ModuleBody.v1",
        schema_id: crate::builtins::ids::THING_MODULE_SCHEMA,
        links {
            predicate THING_BINARY_IMAGE_KIND min 0 max 1;
            predicate THING_ASSET_KIND min 0 max many;
            predicate THING_HAS_BYTES_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind ProgramImage {
        id: crate::builtins::ids::THING_PROGRAM_IMAGE_KIND,
        sym: SYM_PROGRAM_IMAGE,
        version: 1,
        body: ProgramImageBody,
        type_tag: "thingos.ProgramImageBody.v1",
        schema_id: crate::builtins::ids::THING_PROGRAM_IMAGE_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Bitmap {
        id: crate::builtins::ids::THING_BITMAP_KIND,
        sym: SYM_BITMAP,
        version: 1,
        body: BitmapBody,
        type_tag: "thingos.BitmapBody.v1",
        schema_id: crate::builtins::ids::THING_BITMAP_SCHEMA,
        links {
             predicate THING_HAS_BYTES_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind Font {
        id: crate::builtins::ids::THING_FONT_KIND,
        sym: SYM_FONT,
        version: 1,
        body: FontBody,
        type_tag: "thingos.FontBody.v1",
        schema_id: crate::builtins::ids::THING_FONT_SCHEMA,
        links {
             predicate THING_BACKED_BY_KIND min 0 max 1;
             predicate THING_HAS_BYTES_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind KeyEventStream {
        id: crate::builtins::ids::THING_KEY_EVENT_STREAM_KIND,
        sym: SYM_KEY_EVENT_STREAM,
        version: 1,
        body: KeyEventStreamBody,
        type_tag: "thingos.KeyEventStreamBody.v1",
        schema_id: crate::builtins::ids::THING_KEY_EVENT_STREAM_SCHEMA,
        links {}
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct FileSystemBody {
    pub name: alloc::string::String,
}

thing_kind! {
    kind LogEntry {
        id: crate::builtins::ids::THING_LOG_ENTRY_KIND,
        sym: SYM_LOG_ENTRY,
        version: 1,
        body: LogEntryBody,
        type_tag: "thingos.LogEntryBody.v1",
        schema_id: crate::builtins::ids::THING_LOG_ENTRY_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Error {
        id: crate::builtins::ids::THING_ERROR_KIND,
        sym: SYM_ERROR,
        version: 1,
        body: ErrorBody,
        type_tag: "thingos.ErrorBody.v1",
        schema_id: crate::builtins::ids::THING_ERROR_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Fault {
        id: crate::builtins::ids::THING_FAULT_KIND,
        sym: SYM_FAULT,
        version: 1,
        body: FaultBody,
        type_tag: "thingos.FaultBody.v1",
        schema_id: crate::builtins::ids::THING_FAULT_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind PciDevice {
        id: crate::builtins::ids::THING_PCI_DEVICE_KIND,
        sym: SYM_PCI_DEVICE,
        version: 1,
        body: PciDeviceBody,
        type_tag: "thingos.PciDeviceBody.v1",
        schema_id: crate::builtins::ids::THING_PCI_DEVICE_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind SerialPort {
        id: crate::builtins::ids::THING_SERIAL_PORT_KIND,
        sym: SYM_SERIAL_PORT,
        version: 1,
        body: SerialPortBody,
        type_tag: "thingos.SerialPortBody.v1",
        schema_id: crate::builtins::ids::THING_SERIAL_PORT_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind LogStream {
        id: crate::builtins::ids::THING_LOG_STREAM_KIND,
        sym: SYM_LOG_STREAM,
        version: 1,
        body: LogStreamBody,
        type_tag: "thingos.LogStreamBody.v1",
        schema_id: crate::builtins::ids::THING_LOG_STREAM_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind BlockDevice {
        id: crate::builtins::ids::THING_BLOCK_DEVICE_KIND,
        sym: SYM_BLOCK_DEVICE,
        version: 1,
        body: BlockDeviceBody,
        type_tag: "thingos.BlockDeviceBody.v1",
        schema_id: crate::builtins::ids::THING_BLOCK_DEVICE_SCHEMA,
        links {
             predicate THING_ON_VOLUME_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind FileSystem {
        id: crate::builtins::ids::THING_FILESYSTEM_KIND,
        sym: SYM_FILESYSTEM,
        version: 1,
        body: FileSystemBody,
        type_tag: "thingos.FileSystemBody.v1",
        schema_id: crate::builtins::ids::THING_FILESYSTEM_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind File {
        id: crate::builtins::ids::THING_FILE_KIND,
        sym: SYM_FILE,
        version: 1,
        body: FileBody,
        type_tag: "thingos.FileBody.v1",
        schema_id: crate::builtins::ids::THING_FILE_SCHEMA,
        links {}
    }
}

thing_kind! {
    kind Dir {
        id: crate::builtins::ids::THING_DIR_KIND,
        sym: SYM_DIR,
        version: 1,
        body: DirBody,
        type_tag: "thingos.DirBody.v1",
        schema_id: crate::builtins::ids::THING_DIR_SCHEMA,
        links {
            predicate THING_CONTAINS_FILE_KIND min 0 max many;
        }
    }
}

thing_kind! {
     kind Volume {
        id: crate::builtins::ids::THING_VOLUME_KIND,
        sym: SYM_VOLUME,
        version: 1,
        body: VolumeBody,
        type_tag: "thingos.VolumeBody.v1",
        schema_id: crate::builtins::ids::THING_VOLUME_SCHEMA,
        links {
             predicate THING_HAS_MOUNT_KIND min 0 max many;
        }
    }
}

thing_kind! {
    kind DisplayFramebuffer {
        id: crate::builtins::ids::THING_DISPLAY_FRAMEBUFFER_KIND,
        sym: SYM_DISPLAY_FRAMEBUFFER,
        version: 1,
        body: DisplayFramebufferBody,
        type_tag: "thingos.DisplayFramebufferBody.v1",
        schema_id: crate::builtins::ids::THING_DISPLAY_FRAMEBUFFER_SCHEMA,
        links {
            predicate THING_DISPLAY_SCANOUT_BUFFER_KIND min 0 max 1;
        }
    }
}

thing_kind! {
    kind ScanoutBuffer {
        id: crate::builtins::ids::THING_DISPLAY_SCANOUT_BUFFER_KIND,
        sym: SYM_SCANOUT_BUFFER,
        version: 1,
        body: ScanoutBufferBody,
        type_tag: "thingos.ScanoutBufferBody.v1",
        schema_id: crate::builtins::ids::THING_DISPLAY_SCANOUT_BUFFER_SCHEMA,
        links {}
    }
}
