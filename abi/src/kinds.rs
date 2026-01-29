pub type ThingKind = u64;

pub const KIND_BYTESPACE_BUFFER: ThingKind = 0x1000;
pub const KIND_STREAM_WATCH: ThingKind = 0x2000;
pub const KIND_TEST_NODE: ThingKind = 0x3000;

pub type RelKey = u64;

pub const REL_HAS_BUS: RelKey = 0x10;
pub const REL_HAS_DEVICE: RelKey = 0x11;
pub const REL_HAS_RESOURCE: RelKey = 0x12;
pub const REL_BINDS: RelKey = 0x13;
pub const REL_BOUND_TO: RelKey = 0x14;
pub const REL_PROVIDES: RelKey = 0x15;
pub const REL_EMITS: RelKey = 0x16;

pub const KIND_HOST: ThingKind = 0x10;
pub const KIND_KERNEL: ThingKind = 0x11;
pub const KIND_CPU: ThingKind = 0x12;
pub const KIND_MEMORY_RANGE: ThingKind = 0x13;
pub const KIND_FRAMEBUFFER: ThingKind = 0x14;
pub const KIND_CONSOLE: ThingKind = 0x15;
pub const KIND_BOOT_MODULE: ThingKind = 0x16;
pub const KIND_SERVICE: ThingKind = 0x17;
pub const KIND_ASSET: ThingKind = 0x18;
pub const KIND_ASSET_REQUEST: ThingKind = 0x19;

pub const REL_RUNS_ON: RelKey = 0x17;
pub const REL_HAS_CPU: RelKey = 0x18;
pub const REL_HAS_MEMORY_RANGE: RelKey = 0x19;
pub const REL_HAS_MODULE: RelKey = 0x1A;
pub const REL_LOGS_TO: RelKey = 0x1B;

pub type PropKey = u64;

pub const PROP_DUMMY: PropKey = 0x0;
pub const PROP_ARCH: PropKey = 0x1;
pub const PROP_BOOT_ID: PropKey = 0x2;
pub const PROP_HHDM_OFFSET: PropKey = 0x3;
pub const PROP_VERSION: PropKey = 0x4;
pub const PROP_BUILD: PropKey = 0x5;
pub const PROP_START: PropKey = 0x6;
pub const PROP_END: PropKey = 0x7;
pub const PROP_KIND: PropKey = 0x8; // e.g. Memory Kind
pub const PROP_PHYS_BASE: PropKey = 0x9;
pub const PROP_SIZE_BYTES: PropKey = 0xA;
pub const PROP_WIDTH: PropKey = 0xB;
pub const PROP_HEIGHT: PropKey = 0xC;
pub const PROP_STRIDE: PropKey = 0xD;
pub const PROP_FORMAT: PropKey = 0xE;
pub const PROP_PATH_HASH: PropKey = 0xF; // Hash of path since no string props yet
pub const PROP_INDEX: PropKey = 0x10;
pub const PROP_ID: PropKey = 0x11; // CPU ID
