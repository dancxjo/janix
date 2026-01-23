Desired Final Form (Contract)

Everything visible is a snapshot surface (RGBA8888 bytespace) presented with `UI_PRESENT_EPOCH`.
Only Blossom presents snapshots.
Bloom composites only from window snapshot surfaces.
Apps publish only model nodes and assets.

The vibe

"I am a clock / grid / cursor. Here is my model and my assets. Go paint me."

What Bloom is allowed to do

- Discover display target and map framebuffer bytespace.
- Read window + cursor geometry, z-order, and snapshot metadata.
- Composite RGBA8888 snapshot surfaces onto the framebuffer.
- Emit loud diagnostics when a snapshot is missing.

What Blossom is responsible for

- SVG rasterization, text rasterization, and vector paint execution.
- View composition: tile -> viewport -> window snapshots.
- Cursor rendering into its own snapshot surface.
- Presenting snapshots by writing `UI_SNAPSHOT_*` + `UI_PRESENT_EPOCH`.
- `userspace/blossom/src/ui_paint.rs` is the paint boundary (Bloom never imports it).

What apps can do

- Publish UI model nodes (windows, viewports, tiles, text runs, cursor model).
- Publish asset bytespaces (SVG, fonts, images).
- Never present or render pixels; never write snapshot keys.

Snapshot contract

- Keys:
  - `UI_SNAPSHOT_BYTESPACE`, `UI_SNAPSHOT_WIDTH`, `UI_SNAPSHOT_HEIGHT`, `UI_SNAPSHOT_STRIDE`, `UI_SNAPSHOT_FORMAT`
  - `UI_PRESENT_EPOCH`
- Atomicity: `UI_PRESENT_EPOCH` is the commit stamp for the snapshot metadata.
- Bloom only uses snapshot metadata with a non-zero `UI_PRESENT_EPOCH`.

Cursor model (Model A)

- `kinds::UI_CURSOR` node with `UI_SNAPSHOT_*` + `UI_PRESENT_EPOCH`.
- Position lives in `UI_CURSOR_X` / `UI_CURSOR_Y`.

Progressive rendering rule

- Tiles can present independently for progressive fill.
- Viewports can present with partial tile availability.
- Windows present composed snapshots with whatever tiles are ready.

Forbidden dependencies (enforced)

- Bloom must not depend on SVG parser/rasterizer modules.
- Bloom must not depend on font rasterization modules.
- Bloom must not import `ui/paint.rs`, `ui/layout.rs`, `lowered.rs`, or any model interpreter.

Inventory (Phase 1)

Checklist (file -> action)

- `userspace/bloom/src/snapshot.rs`: keep (compositor snapshot reader)
- `userspace/bloom/src/compositor.rs`: keep (display target discovery)
- `userspace/bloom/src/surface.rs`: keep (framebuffer surface)
- `userspace/bloom/src/frame_loop.rs`: keep (frame pacing)
- `userspace/bloom/src/present.rs`: delete or replace (legacy frame pipeline)
- `userspace/bloom/src/asset.rs`: delete (fonts/cursors/wallpaper)
- `userspace/bloom/src/font_graph.rs`: delete (font resolution)
- `userspace/bloom/src/svg/`: move to Blossom (SVG parsing/raster)
- `userspace/bloom/src/raster.rs`: move to Blossom (vector/text raster)
- `userspace/bloom/src/drawlist.rs`: move to Blossom (paint ISA)
- `userspace/bloom/src/lowered.rs`: move to Blossom (low-level ops)
- `userspace/bloom/src/ui/`: delete (model -> paint/layout)
- `userspace/bloom/src/cursor.rs`: delete (cursor rendering)
- `userspace/bloom/src/bristle.rs`: move to Blossom (pointer input)
