#![feature(restricted_std)]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;
use stem::println;

// XorShift32 PRNG for deterministic test data
struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }

    fn next_u8(&mut self) -> u8 {
        (self.next() & 0xFF) as u8
    }
}

// Benchmark configuration
const WARMUP_ITERATIONS: usize = 3;
const BENCHMARK_ITERATIONS: usize = 100;
const SEED: u32 = 0x12345678;

#[stem::main]
fn main() -> ! {
    stem::info!("=== Masked Compositing Microbenchmarks ===");
    stem::info!("Backend detection and performance measurements");
    println!();

    // Detect and report backend
    detect_and_report_backend();
    println!();

    // Run benchmark suites
    bench_contiguous_buffers();
    bench_strided_buffers();
    bench_small_spans();
    bench_large_spans();

    stem::info!("\n=== Benchmarks Complete ===");
    loop {
        stem::yield_now();
    }
}

fn detect_and_report_backend() {
    stem::info!("Backend Detection:");

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        #[cfg(target_feature = "sse2")]
        {
            stem::info!("  Architecture: x86_64");
            stem::info!("  Backend: SSE2");
        }
        #[cfg(not(target_feature = "sse2"))]
        {
            stem::info!("  Architecture: x86_64");
            stem::info!("  Backend: Scalar (no SSE2)");
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        #[cfg(target_feature = "neon")]
        {
            stem::info!("  Architecture: aarch64");
            stem::info!("  Backend: NEON");
        }
        #[cfg(not(target_feature = "neon"))]
        {
            stem::info!("  Architecture: aarch64");
            stem::info!("  Backend: Scalar (no NEON)");
        }
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    {
        stem::info!("  Architecture: Other");
        stem::info!("  Backend: Scalar");
    }
}

fn bench_contiguous_buffers() {
    stem::info!("--- Contiguous Buffers (1024 pixels) ---");

    let width = 1024;
    let height = 1;

    let mut rng = XorShift32::new(SEED);
    let mut dst = Vec::new();
    let mut mask = Vec::new();

    for _ in 0..width {
        dst.push(rng.next() | 0xFF000000); // Opaque dst
        mask.push(rng.next_u8());
    }

    let color = 0x80FF8040; // Semi-transparent orange

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }

    // Benchmark
    let start = stem::time::monotonic_ns();
    for _ in 0..BENCHMARK_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }
    let elapsed = stem::time::monotonic_ns() - start;

    let pixels_processed = (width * height * BENCHMARK_ITERATIONS) as u64;
    let avg_ns_per_pixel = elapsed as f64 / pixels_processed as f64;
    let mpixels_per_sec =
        (pixels_processed as f64 / (elapsed as f64 / 1_000_000_000.0)) / 1_000_000.0;

    stem::info!("  Iterations: {}", BENCHMARK_ITERATIONS);
    stem::info!(
        "  Total time: {} ns ({:.2} ms)",
        elapsed,
        elapsed as f64 / 1_000_000.0
    );
    stem::info!(
        "  Avg per iteration: {:.2} ns ({:.3} μs)",
        elapsed as f64 / BENCHMARK_ITERATIONS as f64,
        elapsed as f64 / (BENCHMARK_ITERATIONS as f64 * 1000.0)
    );
    stem::info!("  Avg per pixel: {:.2} ns", avg_ns_per_pixel);
    stem::info!("  Throughput: {:.2} Mpixels/sec", mpixels_per_sec);
    println!();
}

fn bench_strided_buffers() {
    stem::info!("--- Strided Buffers (128x128, stride=256) ---");

    let width = 128;
    let height = 128;
    let stride = 256;

    let mut rng = XorShift32::new(SEED);
    let mut dst = Vec::new();
    let mut mask = Vec::new();

    for _ in 0..(stride * height) {
        dst.push(rng.next() | 0xFF000000);
        mask.push(rng.next_u8());
    }

    let color = 0x80FF8040;

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, stride, &mask, stride, width, height, color,
        );
    }

    // Benchmark
    let start = stem::time::monotonic_ns();
    for _ in 0..BENCHMARK_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, stride, &mask, stride, width, height, color,
        );
    }
    let elapsed = stem::time::monotonic_ns() - start;

    let pixels_processed = (width * height * BENCHMARK_ITERATIONS) as u64;
    let avg_ns_per_pixel = elapsed as f64 / pixels_processed as f64;
    let mpixels_per_sec =
        (pixels_processed as f64 / (elapsed as f64 / 1_000_000_000.0)) / 1_000_000.0;

    stem::info!("  Iterations: {}", BENCHMARK_ITERATIONS);
    stem::info!(
        "  Total time: {} ns ({:.2} ms)",
        elapsed,
        elapsed as f64 / 1_000_000.0
    );
    stem::info!(
        "  Avg per iteration: {:.2} ns ({:.3} μs)",
        elapsed as f64 / BENCHMARK_ITERATIONS as f64,
        elapsed as f64 / (BENCHMARK_ITERATIONS as f64 * 1000.0)
    );
    stem::info!("  Avg per pixel: {:.2} ns", avg_ns_per_pixel);
    stem::info!("  Throughput: {:.2} Mpixels/sec", mpixels_per_sec);
    println!();
}

fn bench_small_spans() {
    stem::info!("--- Small Spans (Glyph-like: 16x16) ---");

    let width = 16;
    let height = 16;

    let mut rng = XorShift32::new(SEED);
    let mut dst = Vec::new();
    let mut mask = Vec::new();

    for _ in 0..(width * height) {
        dst.push(rng.next() | 0xFF000000);
        mask.push(rng.next_u8());
    }

    let color = 0xFFFFFFFF; // White (typical text color)

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }

    // More iterations for small spans to get stable measurements
    let iterations = BENCHMARK_ITERATIONS * 10;

    // Benchmark
    let start = stem::time::monotonic_ns();
    for _ in 0..iterations {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }
    let elapsed = stem::time::monotonic_ns() - start;

    let pixels_processed = (width * height * iterations) as u64;
    let avg_ns_per_pixel = elapsed as f64 / pixels_processed as f64;
    let mpixels_per_sec =
        (pixels_processed as f64 / (elapsed as f64 / 1_000_000_000.0)) / 1_000_000.0;

    stem::info!("  Iterations: {}", iterations);
    stem::info!(
        "  Total time: {} ns ({:.2} ms)",
        elapsed,
        elapsed as f64 / 1_000_000.0
    );
    stem::info!(
        "  Avg per iteration: {:.2} ns ({:.3} μs)",
        elapsed as f64 / iterations as f64,
        elapsed as f64 / (iterations as f64 * 1000.0)
    );
    stem::info!("  Avg per pixel: {:.2} ns", avg_ns_per_pixel);
    stem::info!("  Throughput: {:.2} Mpixels/sec", mpixels_per_sec);
    println!();
}

fn bench_large_spans() {
    stem::info!("--- Large Spans (SVG fills: 512x512) ---");

    let width = 512;
    let height = 512;

    let mut rng = XorShift32::new(SEED);
    let mut dst = Vec::new();
    let mut mask = Vec::new();

    for _ in 0..(width * height) {
        dst.push(rng.next() | 0xFF000000);
        mask.push(rng.next_u8());
    }

    let color = 0x80FF4040; // Semi-transparent red

    // Warmup
    for _ in 0..WARMUP_ITERATIONS {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }

    // Fewer iterations for large spans (they're expensive)
    let iterations = 10;

    // Benchmark
    let start = stem::time::monotonic_ns();
    for _ in 0..iterations {
        stem::simd::composite_solid_masked_over(
            &mut dst, width, &mask, width, width, height, color,
        );
    }
    let elapsed = stem::time::monotonic_ns() - start;

    let pixels_processed = (width * height * iterations) as u64;
    let avg_ns_per_pixel = elapsed as f64 / pixels_processed as f64;
    let mpixels_per_sec =
        (pixels_processed as f64 / (elapsed as f64 / 1_000_000_000.0)) / 1_000_000.0;

    stem::info!("  Iterations: {}", iterations);
    stem::info!(
        "  Total time: {} ns ({:.2} ms)",
        elapsed,
        elapsed as f64 / 1_000_000.0
    );
    stem::info!(
        "  Avg per iteration: {:.2} ns ({:.3} μs)",
        elapsed as f64 / iterations as f64,
        elapsed as f64 / (iterations as f64 * 1000.0)
    );
    stem::info!("  Avg per pixel: {:.2} ns", avg_ns_per_pixel);
    stem::info!("  Throughput: {:.2} Mpixels/sec", mpixels_per_sec);
    println!();
}
