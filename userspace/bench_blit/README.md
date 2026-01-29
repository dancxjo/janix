# Masked Compositing Microbenchmarks

Performance benchmarks for SIMD-accelerated masked compositing operations in Thing-OS.

## Overview

This benchmark suite measures the performance of `composite_solid_masked_over` across different scenarios:

- **Contiguous buffers**: Simple aligned case (1024 pixels)
- **Strided buffers**: Realistic compositor use (128x128 with 256-byte stride)
- **Small spans**: Glyph-like rendering (16x16)
- **Large spans**: SVG fills (512x512)

## Running Benchmarks

### Using justfile (if available):
```bash
just bench-blit          # Run on default architecture (x86_64)
just bench-blit aarch64  # Run on aarch64
```

### Manual build:
```bash
cargo build -Z build-std=core,alloc -Z build-std-features=compiler-builtins-mem \
    --target targets/x86_64-unknown-thingos.json \
    -p bench_blit
```

## Output

The benchmark reports:
- Backend selection (SSE2/NEON/Scalar)
- Iterations and total time
- Average time per iteration
- Average time per pixel
- Throughput in megapixels/second

Example output:
```
=== Masked Compositing Microbenchmarks ===
Backend detection and performance measurements

Backend Detection:
  Architecture: x86_64
  Backend: SSE2

--- Contiguous Buffers (1024 pixels) ---
  Iterations: 100
  Total time: 12345678 ns (12.35 ms)
  Avg per iteration: 123456.78 ns (123.457 μs)
  Avg per pixel: 120.56 ns
  Throughput: 8.29 Mpixels/sec
...
```

## CI Smoke Test

A smoke test ensures the benchmark builds correctly:
```bash
./scripts/ci_bench_smoke.sh
```

## Implementation Details

- Uses fixed seeds (0x12345678) for reproducible results
- Warmup iterations to stabilize cache/branch predictor
- Iteration counts tuned per scenario size:
  - Large spans: 10 iterations (expensive)
  - Small spans: 1000 iterations (cheap)
  - Medium: 100 iterations

## See Also

- `stem/src/simd/` - SIMD implementation
- `userspace/bloom/src/perf.rs` - Runtime performance counters
