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

pub const REL_RUNS_ON: RelKey = 0x17;
pub const REL_HAS_CPU: RelKey = 0x18;
pub const REL_HAS_MEMORY_RANGE: RelKey = 0x19;
pub const REL_HAS_MODULE: RelKey = 0x1A;
pub const REL_LOGS_TO: RelKey = 0x1B;
