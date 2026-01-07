# Bloom Render Doctrine Enforcement

## Overview

This document describes the enforcement mechanism for Bloom's "record then execute" rendering architecture, which prevents immediate-mode rendering from creeping back into the codebase.

## Architecture

### The Problem
Bloom uses a command-recording architecture where:
1. **Record Phase**: UI code records drawing commands into a list
2. **Execute Phase**: The executor processes the list and writes pixels to scene_buffer

The danger is that someone might accidentally write pixels directly to scene_buffer during the record phase, bypassing the executor and breaking the architecture.

### The Solution

We enforce the doctrine through three mechanisms:

#### 1. RAII Guard (src/executor.rs)
```rust
#[cfg(debug_assertions)]
pub(crate) struct ExecPhaseGuard;
```

- Automatically sets `EXECUTOR_ACTIVE` flag on creation
- Automatically clears it on drop (even on panic)
- Used in both execution paths:
  - `execute_cmds_into_scene()` - Simple batch executor
  - `ChunkedExecutor::step()` - Production executor with chunking

#### 2. Debug Tripwire (src/painter/cpu.rs)
```rust
pub fn new_for_scene(buf: &'a mut [u32], w: u32, h: u32) -> Self {
    #[cfg(debug_assertions)]
    debug_assert!(
        crate::executor::is_executing(),
        "BLOOM DOCTRINE VIOLATION: ..."
    );
    Self::new(buf, w, h)
}
```

- Separate constructor for scene buffer use
- Checks that we're in execute phase (guard is set)
- Panics in debug builds if violated
- Zero overhead in release builds

#### 3. Constructor Discipline

- `CpuPainter::new()` - For framebuffer, test buffers, etc. (no check)
- `CpuPainter::new_for_scene()` - For scene_buffer only (debug-checked)

## Usage

### ✅ Correct: Execute phase writes
```rust
// Inside execute_cmds_into_scene or ChunkedExecutor::step
let _guard = ExecPhaseGuard::enter(); // Sets EXECUTOR_ACTIVE
let mut painter = CpuPainter::new_for_scene(scene_buffer, w, h); // ✓ Guard is set
painter.fill_rect(...); // ✓ Writing to scene through executor
```

### ✅ Correct: Framebuffer writes (cursor overlay)
```rust
// In cursor_overlay.rs or compositing
let mut painter = CpuPainter::new(framebuffer, w, h); // ✓ Uses new(), not new_for_scene()
painter.blit_rgba_alpha(...); // ✓ Writing to framebuffer, not scene
```

### ❌ Violation: Direct scene writes
```rust
// Somewhere outside executor
let mut painter = CpuPainter::new_for_scene(scene_buffer, w, h); 
// ❌ PANIC: "BLOOM DOCTRINE VIOLATION: new_for_scene() called outside execute phase"
```

## Exemptions

The cursor overlay (`cursor_overlay.rs`) is the ONLY code allowed to do immediate-mode rendering:
- It writes to **framebuffer**, not scene_buffer
- It uses `CpuPainter::new()`, not `new_for_scene()`
- It's architecturally necessary for cursor responsiveness during long scene rebuilds

## Build Modes

### Debug Builds
- ✅ Full enforcement active
- ✅ Violations panic with clear error message
- ✅ Helps catch bugs during development

### Release Builds
- ✅ Zero runtime overhead (all checks compiled out)
- ✅ is_executing() inlined to `false`
- ✅ new_for_scene() identical to new()

## Testing

See `test_tripwire_design.md` for test scenarios:
1. Executor paths work correctly (guard set)
2. Framebuffer paths work correctly (no guard needed)
3. Direct misuse panics in debug
4. Release builds have no overhead

## Maintenance

When adding new executors:
1. Set the guard via `ExecPhaseGuard::enter()`
2. Use `CpuPainter::new_for_scene()` for scene_buffer
3. Use `CpuPainter::new()` for other buffers

## Files Modified

- `crates/bloom/src/executor.rs` - Added guard and is_executing() (already existed)
- `crates/bloom/src/chunked_executor.rs` - Uses guard and new_for_scene()
- `crates/bloom/src/painter/cpu.rs` - Added new_for_scene() constructor

No kernel changes required.
