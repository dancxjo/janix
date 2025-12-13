# Compositor Architecture Notes

This is an internal design sketch for `user/compositor`. It captures the high level graph of how data flows through modules (graph → layout → display list → framebuffer, plus input feedback). Nothing here is enforced yet by real Things; it is a guide for future instrumentation.

## Core Modules & Types

- `CompositorModule` (`user/compositor`) defines `CompositorRuntime` and contains:
  - `compositor.config`
  - `compositor.graph`
  - `compositor.layout`
  - `compositor.model`
  - `compositor.input`
  - `compositor.render`
  - `compositor.render.display_list`
- Types declared by modules:
  - `compositor.model` → `Compositor`, `CursorState`, `DragState`
  - `compositor.layout` → `StackedWindow` (layout-owned window geometry)
  - `compositor.render.display_list` → `DrawOp`
- Graph Thing types we read/write: `Window`, `Surface`, `Mode`, `MousePacketEvent`, `PrimaryDisplayBuffer`.

## Conceptual Flow

```
World (kernel/drivers) → Graph Things
Graph (Window/Surface/Mode/MousePacketEvent) → Layout (Vec<StackedWindow>)
Layout → DisplayList (Vec<DrawOp>) → Framebuffer (PrimaryDisplayBuffer)
Input (MousePacketEvent) → Layout hit-test → Graph prop updates
```

### Detailed links

- `Module_state` calls `Module_graph` (collect Things), `Module_layout` (build `Vec<StackedWindow>`), `Module_input` (cursor/drag with stacked windows), and `Module_render.display_list` (build/execute `DrawOp`s).
- `Type_WindowThing -[:FLOW_INTO layout]-> Type_StackedWindow`
- `Type_SurfaceThing -[:FLOW_INTO display_list]-> Type_DrawOp`
- `Type_StackedWindow -[:FLOW_INTO display_list]-> Type_DrawOp`
- `Type_DrawOp -[:FLOW_INTO rasterize]-> Resource_Framebuffer`
- Input feedback:
  - `Type_MousePacketEventThing -[:FLOW_INTO input]-> Type_CursorState`
  - `Type_CursorState -[:DRIVES]-> Type_DragState`
  - `Type_DragState -[:WRITES_PROP {keys:["x","y"]}]-> Type_WindowThing`
  - `Type_StackedWindow -[:DRIVES]-> Type_Compositor` (activation decision)
  - `Type_Compositor -[:WRITES_PROP {keys:["active","z_index"]}]-> Type_WindowThing`

## Cypher Sketch

Illustrative Cypher snippet (replace `DesignNode` with your actual schema when implementing real design graph things):

```cypher
MERGE (comp:DesignNode {kind:"Module", name:"user/compositor"});
UNWIND [
  "compositor.config",
  "compositor.graph",
  "compositor.layout",
  "compositor.model",
  "compositor.input",
  "compositor.render",
  "compositor.render.display_list"
] AS mname
MERGE (m:DesignNode {kind:"Module", name:mname})
MERGE (comp)-[:CONTAINS]->(m);

UNWIND [
  ["Compositor","State","compositor.model"],
  ["CursorState","State","compositor.model"],
  ["DragState","State","compositor.model"],
  ["StackedWindow","Layout","compositor.layout"],
  ["DrawOp","Render","compositor.render.display_list"],
  ["Window","Thing","thing_os"],
  ["Surface","Thing","thing_os"],
  ["MousePacketEvent","Thing","thing_os"],
  ["Mode","Thing","thing_os"]
] AS row
MERGE (ty:DesignNode {kind:"Type", name:row[0]})
SET ty.layer = row[1]
MERGE (mod:DesignNode {kind:"Module", name:row[2]})
MERGE (mod)-[:DECLARES]->(ty);

MATCH (win:DesignNode {name:"Window"})
MATCH (stack:DesignNode {name:"StackedWindow"})
MERGE (win)-[:FLOW_INTO {step:"layout"}]->(stack);
MATCH (stack:DesignNode {name:"StackedWindow"})
MATCH (draw:DesignNode {name:"DrawOp"})
MERGE (stack)-[:FLOW_INTO {step:"display_list"}]->(draw);
MATCH (surf:DesignNode {name:"Surface"})
MERGE (surf)-[:FLOW_INTO {step:"display_list"}]->(draw);
MATCH (mouse:DesignNode {name:"MousePacketEvent"})
MATCH (cursor:DesignNode {name:"CursorState"})
MERGE (mouse)-[:FLOW_INTO {step:"input"}]->(cursor);
MATCH (cursor:DesignNode {name:"CursorState"})
MATCH (drag:DesignNode {name:"DragState"})
MERGE (cursor)-[:DRIVES]->(drag);
MATCH (drag:DesignNode {name:"DragState"})
MATCH (win:DesignNode {name:"Window"})
MERGE (drag)-[:WRITES_PROP {keys:["x","y"]}]->(win);
```

Use this as a template when you want to ingest the design graph into the actual Thing graph.
