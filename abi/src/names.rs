use super::kinds::*;

pub fn kind_name(k: ThingKind) -> &'static str {
    match k {
        KIND_BYTESPACE_BUFFER => "bytespace",
        KIND_STREAM_WATCH => "stream.watch",
        KIND_TEST_NODE => "test.node",
        KIND_HOST => "dev.host",
        KIND_KERNEL => "proc.kernel",
        KIND_CPU => "dev.cpu",
        KIND_MEMORY_RANGE => "mem.range",
        KIND_FRAMEBUFFER => "dev.display.framebuffer",
        KIND_CONSOLE => "dev.console",
        KIND_BOOT_MODULE => "boot.module",
        KIND_SERVICE => "svc",
        _ => "unknown",
    }
}

pub fn rel_name(r: RelKey) -> &'static str {
    match r {
        REL_HAS_BUS => "HAS_BUS",
        REL_HAS_DEVICE => "HAS_DEVICE",
        REL_HAS_RESOURCE => "HAS_RESOURCE",
        REL_BINDS => "BINDS",
        REL_BOUND_TO => "BOUND_TO",
        REL_PROVIDES => "PROVIDES",
        REL_EMITS => "EMITS",
        REL_RUNS_ON => "RUNS_ON",
        REL_HAS_CPU => "HAS_CPU",
        REL_HAS_MEMORY_RANGE => "HAS_MEMORY_RANGE",
        REL_HAS_MODULE => "HAS_MODULE",
        REL_LOGS_TO => "LOGS_TO",
        _ => "REL_UNKNOWN",
    }
}

pub fn prop_name(p: PropKey) -> &'static str {
    match p {
        PROP_ARCH => "arch",
        PROP_BOOT_ID => "boot_id",
        PROP_HHDM_OFFSET => "hhdm_offset",
        PROP_VERSION => "version",
        PROP_BUILD => "build",
        PROP_START => "start",
        PROP_END => "end",
        PROP_KIND => "kind",
        PROP_PHYS_BASE => "phys_base",
        PROP_SIZE_BYTES => "size_bytes",
        PROP_WIDTH => "width",
        PROP_HEIGHT => "height",
        PROP_STRIDE => "stride",
        PROP_FORMAT => "format",
        PROP_PATH_HASH => "path_hash",
        PROP_INDEX => "index",
        PROP_ID => "id",
        _ => "p",
    }
}
