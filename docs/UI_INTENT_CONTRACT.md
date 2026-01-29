# UI Intent Contract

## Purpose

This document defines the architectural boundary between applications, the Petals UI library, and the Blossom layout/paint service. The goal is to enforce a clean separation where:

- **Apps declare UI intent** (what they want to show)
- **Blossom performs layout and paint** (how it gets rendered)
- **Apps do not perform layout, geometry calculations, or painting**

## The Three-Layer Model

```
┌─────────────────────────────────────────────────┐
│ Applications (Font Explorer, etc.)              │
│ - Publish UI intent via Petals builder API      │
│ - Handle user input events                      │
│ - Update application state                      │
│ - NEVER: layout, geometry, paint, raster        │
└─────────────────────────────────────────────────┘
                      │
                      │ UI Intent (Scene graph)
                      ▼
┌─────────────────────────────────────────────────┐
│ Graph (Thing-OS graph database)                 │
│ - Stores UI intent as immutable scenes          │
│ - Provides watches for changes                  │
│ - No semantic understanding of UI               │
└─────────────────────────────────────────────────┘
                      │
                      │ Watch events
                      ▼
┌─────────────────────────────────────────────────┐
│ Blossom (Layout & Paint Service)                │
│ - Reads UI intent from graph                    │
│ - Computes layout (rectangles, positions)       │
│ - Generates paint commands (drawlists)          │
│ - Manages text measurement, font rendering      │
└─────────────────────────────────────────────────┘
                      │
                      │ Paint bytespace
                      ▼
┌─────────────────────────────────────────────────┐
│ Bloom (Compositor)                              │
│ - Rasterizes paint commands to pixels           │
│ - Manages window surfaces and damage             │
│ - Composites final display output                │
└─────────────────────────────────────────────────┘
```

## What Apps MAY Do

Applications are **PERMITTED** to:

1. **Build UI intent trees** using the Petals builder API:
   ```rust
   use stem::petals::{Scene, Window, Flex, Text, Scroll, ...};
   
   let scene = Scene::new().window(
       Window::new(window_id)
           .title("My App")
           .root(Flex::column()
               .push(Text::new("Hello"))
               .push(Text::new("World")))
   );
   ```

2. **Publish scenes** to the graph:
   ```rust
   stem::petals::publish_window(&scene)?;
   ```

3. **Handle user input events** from the graph (clicks, key presses, etc.)

4. **Query application state** from the graph (font lists, data, etc.)

5. **Use declarative styling**:
   - Set widths, heights, padding, margins
   - Set colors, fonts, text properties
   - These are **intent properties**, not computed geometry

## What Apps MUST NOT Do

Applications are **FORBIDDEN** from:

1. **Computing layout geometry**:
   - ❌ No rectangle calculations (x, y, width, height positions)
   - ❌ No text measurement or glyph metrics
   - ❌ No scroll offset calculations
   - ❌ No dirty rectangle tracking

2. **Performing paint operations**:
   - ❌ No direct access to `PaintBuilder` or `emit_paint` functions
   - ❌ No rasterization or pixel manipulation
   - ❌ No drawing primitives (lines, shapes, fills)

3. **Storing derived UI state**:
   - ❌ No caching of computed positions
   - ❌ No tracking of previous render state
   - ❌ No damage tracking

4. **Importing Blossom internals**:
   - ❌ Cannot import from `blossom::layout`
   - ❌ Cannot import from `blossom::emit_paint`
   - ❌ Cannot import from `blossom::scene` (internal representation)

## What Petals Provides

The `stem::petals` module provides the **builder API** for constructing UI intent:

### Core Types

- `Scene`: Root container for UI intent
- `Window`: Window-level intent with title, size constraints
- `Flex`: Flexbox layout container (row/column)
- `Text`: Text display with font, size, color, wrapping
- `Rect`: Colored rectangle
- `Image`: Image display with fit mode
- `Icon`: Icon display
- `Scroll`: Scrollable container
- `Separator`: Visual separator
- `Checkbox`: Checkbox widget
- `Spacer`: Fixed-height spacing

### Styling Properties

- `Size`: `Auto`, `Px(i32)`, `Pct(u8)`
- `Color`: ARGB color values
- `FontKey`: Font selection (by name or ThingId)
- `EdgeInsets`: Padding/margin specification
- Layout properties: `flex_grow`, `flex_shrink`, `flex_basis`, `gap`, `align_items`, `justify_content`

### Publishing API

```rust
pub fn publish_window(scene: &Scene) -> Result<()>
pub fn publish_desktop(scene: &Scene) -> Result<()>
```

These functions serialize the scene to the graph and increment the generation counter.

## What Blossom Produces

The Blossom service is responsible for:

1. **Reading UI intent** from graph nodes via watches on:
   - `ui.scene_bytespace`: Serialized scene data
   - `ui.scene_gen`: Scene generation number
   - Window properties (width, height, background, focus)

2. **Computing layout**:
   - Calculating final rectangles for all nodes
   - Text measurement and line breaking
   - Scroll calculations and clipping
   - Flexbox layout algorithm

3. **Generating paint commands**:
   - Creating drawlists from the laid-out scene
   - Emitting fill, stroke, text, image commands
   - Writing to `ui.paint_bytespace`
   - Incrementing `ui.paint_gen`

4. **Optional: Writing back computed bounds** (for debugging):
   - `ui.x`, `ui.y`, `ui.width`, `ui.height` on nodes
   - These are informational only; apps must not depend on them

## Graph Schema

### UI Intent Nodes (Written by Apps via Petals)

```
ui.window
  ├─ ui.scene_bytespace → serialized Scene
  ├─ ui.scene_gen → generation counter (incremented on publish)
  ├─ ui.bg_color → window background color
  ├─ ui.title → window title (optional)
  ├─ ui.width → window width (set by WM or app)
  └─ ui.height → window height (set by WM or app)
```

The `scene_bytespace` contains a serialized representation of the `Scene` tree built by the app.

### Paint Output (Written by Blossom)

```
ui.window
  ├─ ui.paint_bytespace → serialized paint commands
  └─ ui.paint_gen → generation counter (incremented by Blossom)
```

## Enforcement Mechanisms

### 1. Module Privacy

The `blossom` crate is structured to prevent external access to layout/paint internals:

```rust
// blossom/src/lib.rs
pub mod graph_ui;     // Public: for internal graph UI reading
mod layout;           // Private: layout algorithms
mod emit_paint;       // Private: paint command generation
mod scene;            // Private: internal scene representation
```

User applications link only to `stem`, which re-exports `petals` but **not** `blossom` internals.

### 2. Crate Boundaries

- **stem**: Library crate providing `petals` builder API to apps
- **blossom**: Binary crate (service) that apps cannot link against
- **bloom**: Binary crate (compositor) that apps cannot link against

Apps depend on `stem`, which provides no access to layout or paint internals.

### 3. Compilation Tests

A test ensures that apps cannot compile if they try to import Blossom internals:

```rust
// In tests/ui_boundary_test.rs or similar
#[test]
fn app_cannot_import_blossom_internals() {
    // This test uses trybuild to ensure code that imports
    // blossom::layout or blossom::emit_paint fails to compile
}
```

## Migration Checklist for Apps

When migrating an app to this contract:

- [ ] Remove all geometry calculations (x, y, w, h arithmetic)
- [ ] Remove all `PaintBuilder` or direct paint calls
- [ ] Remove dirty tracking, damage computation
- [ ] Replace with Petals builder API (Scene, Window, Flex, Text, etc.)
- [ ] Use only declarative styling (Size::Px, padding, Color, etc.)
- [ ] Publish scenes via `stem::petals::publish_window()`
- [ ] Verify no imports from `blossom::` or `bloom::`

## Example: Compliant App Structure

```rust
#![no_std]
#![no_main]

extern crate alloc;
use stem::petals::{Scene, Window, Flex, Text, FontKey, Color, Size};

#[stem::main]
fn main() -> ! {
    let window_id = /* find or create window */;
    
    loop {
        // 1. Read application state
        let items = fetch_items_from_graph();
        
        // 2. Build UI intent
        let mut container = Flex::column().gap(8).padding(16);
        for item in items {
            container = container.push(
                Text::new(&item.label)
                    .font(FontKey::new("NotoSans-Regular").size(14))
                    .color(Color::rgb(0, 0, 0))
            );
        }
        
        let scene = Scene::new().window(
            Window::new(window_id)
                .title("My App")
                .root(container)
        );
        
        // 3. Publish to graph
        let _ = stem::petals::publish_window(&scene);
        
        // 4. Sleep and repeat
        stem::sleep_ms(33);
    }
}
```

## Litmus Test

**If you freeze the graph at time T and restart Blossom, does the screen eventually look identical?**

- ✅ **YES**: The app is correctly publishing intent only
- ❌ **NO**: The app is doing work that belongs to Blossom

## References

- `docs/ui_architecture.md`: High-level UI architecture
- `stem/src/petals/builder.rs`: Petals builder API implementation
- `userspace/blossom/src/main.rs`: Blossom service main loop
- `userspace/font_explorer/src/main.rs`: Example application
