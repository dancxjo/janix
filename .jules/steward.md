# The Steward’s Journal

## 2025-02-12 – Allocation Removal in ScanLinks
Learning: Hot-path syscalls like `ScanLinks` were performing heap allocations (`Box<dyn Iterator>`) solely for implementation convenience (dispatching between two iterator types).
Guardrail: When refactoring syscall handlers, prefer generic helper functions or enums over `Box<dyn Trait>` to avoid hidden allocator pressure in the kernel.
