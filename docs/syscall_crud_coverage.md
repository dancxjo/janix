# Syscall CRUD Coverage Inventory

## Phase 0.1 Inventory

| Category | Operation | Status | Wire Request Type | Wire Response Type | Syscall / Handler | Notes |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Things** | **Create** | ❌ Missing | - | - | - | Needs `CreateThing` op via `GraphOp`. |
| | **Read** | ⚠️ Partial | `GraphOp::Log`, `GraphOp::SymbolResolve` | `GraphReply` | `sys_graph` | No generic "Get Thing by ID" or properties reader. |
| | **Update** | ❌ Missing | - | - | - | Needs `SetProp` or `UpdateThing` op. |
| | **Delete** | ❌ Missing | - | - | - | No delete operational support. |
| **Links** | **Create** | ❌ Missing | - | - | - | Modeled as Things (`Link` Kind), but no creation op for either. |
| | **Read (Out)**| ❌ Missing | - | - | - | No adjacency scan. |
| | **Read (In)** | ❌ Missing | - | - | - | No adjacency scan. |
| | **Delete** | ❌ Missing | - | - | - | No delete logic. |
| **TypeDefs** | **Register** | ✅ Exists | `[u8; len]` (raw bytes) | `SysRet` (0/err) | `sys_typedef_register` | Handled via specific syscall 200. |
| | **Get** | ✅ Exists | `TypeId` (16 bytes) | `TypeDef` (via postcard) | `sys_typedef_get` | Handled via specific syscall 201. |
| **Kind Association** | **Set** | ❌ Missing | - | - | - | No mechanism to link Kind -> TypeDef. |
| | **Get** | ❌ Missing | - | - | - | No mechanism to inspect Kind metadata. |

## Notes

*   **Links as Things**: The system uses **Model 2** (Links are Things). `crates/models/src/builtins/kinds.rs` defines `THING_LINK_KIND` using `crate::link::LinkBody`.
    *   This means "Create Link" == "Create Thing of Kind Link".
    *   BUT, the kernel needs to know it's a link to index the graph edges.
    *   Therefore, generic `CreateThing` *might* suffice if the kernel intercepts the Kind ID, OR we need a focused `AddLink` that creates the underlying Thing and updates indexes. The Plan prefers `AddLink` in wire format for clarity/indexing guarantees.
*   **TypeDefs**: Handled by separate syscalls (200/201), not `GraphOp`.
*   **Safety**: Currently relies on identity mapping or unsafe pointer access in handlers.
