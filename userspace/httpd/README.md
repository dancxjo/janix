# httpd - Thing-OS HTTP Server

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
  -p httpd
```

## Testing

Unit tests are included in each module:
```bash
# Note: Tests run on host, not target
cargo test -p httpd
```

## Implementation Notes

- Uses `#![no_std]` for bare metal compatibility
- Depends on `stem` runtime and `abi` for syscalls
- JSON formatting is manual to avoid dependencies
- All allocations go through the Thing-OS allocator
