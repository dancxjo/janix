# Bloom Compositor

Bloom is the reference compositor for Thing-OS, implementing a damage-aware, transactional rendering pipeline.

## Startup Contract

- Bloom is VFS-directed, not graph-directed.
- First paint must depend only on display files such as `/dev/fb0`, driver channels, and session/runtime files.
- Do not gate compositor startup on legacy graph APIs like `stem::thing`, `ThingId`, `UI_CROWN`, or root graph discovery.
- If graph-backed UI metadata exists in userspace experiments, it must be optional and must not block boot or painting.

## Desktop Namespace

- The desktop background is configured through `/session/desktop/`.
- `wallpaper`: absolute VFS path to the wallpaper asset.
- `mode`: one of `stretch`, `center`, `tile`, `fit`, or `fill`.
- `background_color`: fallback color in `#RRGGBB`.
- Bloom watches `/session/desktop` and repaints when those files change.

## Mouse Acceleration (Tuning Notes)
- Current curve: logarithmic gain `1 + a * ln(1 + speed / s)` with `a=0.8`, `s=500`.
- `max_gain=4.0` is a safety cap; raise for faster flicks on 4K+.
- If slow moves feel too fast, lower `a`; if fast flicks feel too weak, lower `s`.

## Portable Render ISA (v0)

Bloom strictly separates high-level drawing commands (`DrawCmd`) from low-level execution primitives (`LowLevelOp`). The `LowLevelOp` instruction set serves as a portable contract that any backend (CPU rasterizer, VirtIO-GPU, accelerated hardware) must implement.

### 1. Coordinate System
- **Origin**: Top-left (0, 0).
- **Axes**: X extends right, Y extends down.
- **Units**: Logical pixels (integers).
- **Bounds**: Operations outside the surface bounds are clipped safely.

### 2. State Management
The renderer maintains two strict state stacks:
- **Clip Stack**: 
  - `PushClip(rect)`: Intersects the new rect with the current clip. If the result is empty, all subsequent ops are effectively no-ops until popped.
  - `PopClip`: Restores the previous clip state.
  - *Invariant*: Stack depth is limited (e.g., 64). Underflow/Overflow is a checked error or saturating no-op.
- **Transform Stack**:
  - `PushTransform(t)`: Combines `t` with the current transform.
  - `PopTransform`: Restores previous transform.
  - *v0 Limitation*: Only 2D translations (`dx`, `dy`) are supported. Scaling/Rotation is reserved for v1.

### 3. Execution Semantics
The `LowLevelOp` enum defines the atomic operations:

#### Control
- `Clear { color }`: Clears the current clip region to a solid color.

#### Geometry
- `FillRect`, `StrokeRect`, `FillCircle`, `Line`: Renders primitives using the current transform and color.
- **Clipping**: All primitives are strictly clipped to the current clip rect.

#### Images
- **`BlitOpaque`**: 
  - Source pixels replace destination pixels.
  - **Scaling**: If `src` size != `dst` size, strict **Nearest Neighbor** scaling is applied.
  - Alpha channel in source is ignored (treated as opaque).
- **`BlitAlpha`**:
  - **Blending**: Strictly `SrcOver` (standard alpha blending).
  - **Scaling**: Nearest Neighbor.
  - **Const Alpha**: Optional `u8` factor. If present, it modulates the source alpha: `final_a = (src_a * const_alpha) / 255`.
  - Used for shadows, cursor transparency, and general UI compositing.

### 4. Text
- `TextSpan`: Renders a run of text at a position.
- Uses the system font cache (glyph caching).
- Clipped to current clip.

### 5. Rasterizer Compliance
The CPU rasterizer in `raster.rs` is the reference implementation. Any future backend (e.g., `virtio-gpu`) must produce pixel-identical results for `BlitOpaque` and topologically equivalent results for geometry.
