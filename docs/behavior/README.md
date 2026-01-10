# Thing-OS BDD Test Results

Behavior-driven development test results organized by target architecture.

## Feature Definitions

Feature files are in the [`features/`](./features/) subdirectory:
- `boot.feature` - Boot screen tests
- `kernel_heap.feature` - Heap allocation tests
- `memory.feature` / `memory_real.feature` - Memory subsystem tests
- `paging.feature` - Paging subsystem tests
- `simple-boot.feature` - Basic boot verification
- `threading.feature` - Multi-threading tests
- `time.feature` - System timer tests

## Test Results

Results are stored per-scenario in the following structure:
```
docs/behavior/{arch}/{feature-slug}/{scenario-slug}/README.md
```

Each scenario README contains:
- Step-by-step results with pass/fail status
- Screenshots (before/after) for each step
- Serial log excerpts
- Register dumps (for debugging failures)

## Architectures

- [x86_64](./x86_64/)
- [aarch64](./aarch64/)
- [riscv64](./riscv64/)
- [loongarch64](./loongarch64/)

## Running Tests

```bash
# Run all BDD tests for all architectures
just behave

# Run tests for a specific architecture
just behave x86_64

# Run a specific feature
BDD_FEATURE=threading just behave x86_64

# Clear test results (preserves feature files)
just clear-behavior
```

---

*See individual scenario READMEs for detailed test results.*
