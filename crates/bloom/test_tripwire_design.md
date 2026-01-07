# Bloom Doctrine Tripwire Test Design

## Test Cases

### Test 1: Legitimate executor usage (should PASS)
```rust
#[test]
#[cfg(debug_assertions)]
fn test_executor_sets_flag_correctly() {
    use crate::executor::execute_cmds_into_scene;
    use crate::draw_cmd::DrawCmd;
    use crate::scene_cache::BytespaceMappingCache;
    use crate::assets::bitmap::BitmapStore;
    
    let mut buffer = vec![0u32; 100];
    let cmds = vec![DrawCmd::Clear { color: 0xFF000000 }];
    let mut cache = BytespaceMappingCache::new();
    let store = BitmapStore::new();
    
    // This should NOT panic because execute_cmds_into_scene sets the guard
    let _result = execute_cmds_into_scene(&cmds, &mut buffer, 10, 10, &mut cache, &store);
}
```

### Test 2: ChunkedExecutor usage (should PASS)
```rust
#[test]
#[cfg(debug_assertions)]
fn test_chunked_executor_sets_flag_correctly() {
    use crate::chunked_executor::ChunkedExecutor;
    use crate::draw_cmd::DrawCmd;
    use crate::scene_cache::BytespaceMappingCache;
    use crate::assets::bitmap::BitmapStore;
    
    let mut buffer = vec![0u32; 100];
    let cmds = vec![DrawCmd::Clear { color: 0xFF000000 }];
    let mut exec = ChunkedExecutor::new(cmds, 10, 10);
    let mut cache = BytespaceMappingCache::new();
    let store = BitmapStore::new();
    
    // This should NOT panic because ChunkedExecutor::step sets the guard
    let _done = exec.step(&mut buffer, &mut cache, &store, 100);
}
```

### Test 3: Direct CpuPainter misuse (should PANIC in debug)
```rust
#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "BLOOM DOCTRINE VIOLATION")]
fn test_scene_painter_without_guard_panics() {
    use crate::painter::CpuPainter;
    
    let mut buffer = vec![0u32; 100];
    
    // This should panic because we're calling new_for_scene without setting the guard
    let _painter = CpuPainter::new_for_scene(&mut buffer, 10, 10);
}
```

### Test 4: Framebuffer usage (should PASS - no guard required)
```rust
#[test]
fn test_framebuffer_painter_no_guard_required() {
    use crate::painter::CpuPainter;
    use crate::scene::Rect;
    
    let mut framebuffer = vec![0u32; 100];
    
    // This should work fine - framebuffer doesn't need guard
    let mut painter = CpuPainter::new(&mut framebuffer, 10, 10);
    painter.fill_rect(Rect { x: 0, y: 0, w: 5, h: 5 }, 0xFF0000FF);
}
```

### Test 5: Release build has no overhead
```rust
#[test]
#[cfg(not(debug_assertions))]
fn test_release_no_overhead() {
    use crate::executor::is_executing;
    use crate::painter::CpuPainter;
    
    // In release builds, is_executing() always returns false
    assert_eq!(is_executing(), false);
    
    // new_for_scene and new should behave identically in release
    let mut buffer1 = vec![0u32; 100];
    let mut buffer2 = vec![0u32; 100];
    
    let _painter1 = CpuPainter::new(&mut buffer1, 10, 10);
    let _painter2 = CpuPainter::new_for_scene(&mut buffer2, 10, 10); // No panic
}
```

## Expected Behavior

### Debug Builds
- ✅ Executor paths work normally (guard is set)
- ✅ Framebuffer usage works normally (uses `new()`)
- ❌ Direct `new_for_scene()` without guard panics
- ❌ Accidental scene_buffer misuse panics

### Release Builds
- ✅ All code paths work
- ✅ No runtime overhead
- ✅ is_executing() always returns false
- ✅ No panics even without guard
