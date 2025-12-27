pub const BUILTIN_SYMBOLS: &[&str] = &[
    // meta
    "Kind", "Schema", "Link", "Intent", "Observation", "Result",
    // core-ish
    "TimeNow", "Process", "Thread", "Capability", "Graph", "Mount", "GraphProvider", "Buffer", "Stream",
    "Keyboard", "KeyEvent", "Font", "BootProgram", "BootRoot",
    // predicates
    "OWNS", "HAS_CAP", "HAS_SCHEMA", "HAS_META", "MOUNTS", "BACKED_BY", "LAUNCHES",
    // common fields (for future query tools)
    "name", "version",
    "created_at_ns", "created_by", "last_modified_at_ns", "last_modified_by", "owner",
];
