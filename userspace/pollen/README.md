# pollen - Thing-OS HTTP Server

A minimal HTTP/1.1 server for Thing-OS that provides graph-backed endpoints.

## Features

- HTTP/1.1 request parsing (GET and HEAD methods)
- Static response serving
- Graph introspection endpoints
- Bytespace streaming with size limits
- Defensive header size limits

## Endpoints

### GET /health
Returns a simple health check response.
```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 2

ok
```

### GET /
Returns an HTML index page with links to available endpoints.

### GET /graph
Returns a JSON index of graph endpoints.

### GET /graph/<thing_id>
Returns JSON representation of a Thing by ID.

Response format:
```json
{
  "thing_id": 12345,
  "kind_id": 67890,
  "props": {},
  "links": []
}
```

### GET /graph/<thing_id>/bytespace/<bytespace_id>
Streams bytespace contents with a 1MB size cap.

## Modes

### stdio Mode (Default)
For testing without network stack. Reads request from stdin, writes response to stdout.

### Server Mode (Future)
Will listen on a TCP port when network stack is available.

## Graph Viewer (ELK Layout)

The interactive graph viewer at `/graph.html` provides visual exploration of the system graph using **ELK.js** (Eclipse Layout Kernel) for automatic node layout.

### Features

- **Automatic Layout**: ELK's layered algorithm places nodes to minimize edge crossings and ensure readable labels
- **Web Worker**: Layout computation runs in a background worker to prevent UI freezes
- **Label-Based Sizing**: Node dimensions are computed from label text to avoid overlap
- **Pan/Zoom**: Mouse wheel zoom and drag panning
- **Click Navigation**: Double-click a node to navigate to its detail page
- **Layout Persistence**: Save layout positions back to the graph for sharing with Photosynthesis

### URL Parameters

- `?root=<ThingId>` - Focus on a specific root node (empty = auto-discover)
- `?depth=<N>` - Traversal depth (default: 3, max: 10)
- `?spacing=<px>` - Node-to-node spacing (default: 40)
- `?layer_spacing=<px>` - Layer-to-layer spacing (default: 60)

### UI Controls

| Button | Action |
|--------|--------|
| Load | Fetch graph data and compute layout |
| Re-layout | Re-run ELK layout without refetching |
| Fit | Fit graph to viewport |
| Save Layout | Persist node positions to graph |

### Architecture

```
┌─────────────────┐     ┌─────────────────┐
│   graph.js      │────▶│  elk-worker.js  │
│ (Main Thread)   │◀────│ (Web Worker)    │
└────────┬────────┘     └─────────────────┘
         │                      │
         ▼                      ▼
┌─────────────────┐     ┌─────────────────┐
│ Cytoscape.js    │     │    ELK.js       │
│  (Rendering)    │     │   (Layout)      │
└─────────────────┘     └─────────────────┘
```

The ELK worker handles layout computation, returning node positions that Cytoscape renders. This separation enables future renderer swaps (e.g., WebGL/Three.js for 3D).

## Security Constraints

- Maximum bytespace response size: 1MB
- Path validation prevents ".." traversal
- Header size limits: 8KB per line, max 64 headers
- Request line limit: 8KB

## TODO

- [ ] Authentication token/capability check
- [ ] Per-route permission gating
- [ ] Size limit query parameters for bytespaces
- [ ] Network stack integration for server mode
- [ ] Property enumeration in graph responses
- [ ] Link enumeration in graph responses

## Building

```bash
cargo +nightly build -Z build-std=core,alloc \
  -Z build-std-features=compiler-builtins-mem \
  --target targets/x86_64-unknown-thingos.json \
  -p pollen
```

## Testing

Unit tests are included in each module:
```bash
# Note: Tests run on host, not target
cargo test -p pollen
```

## Implementation Notes

- Uses `#![no_std]` for bare metal compatibility
- Depends on `stem` runtime and `abi` for syscalls
- JSON formatting is manual to avoid dependencies
- All allocations go through the Thing-OS allocator
