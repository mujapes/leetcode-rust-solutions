include!("Q0131-Palindrome-Partitioning-slices-only.rs");

// Author: Copilot

use std::alloc::{GlobalAlloc, Layout, System};
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

struct TrackingAllocator;

static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);
static PEAK_BYTES: AtomicUsize = AtomicUsize::new(0);

#[global_allocator]
static GLOBAL_ALLOCATOR: TrackingAllocator = TrackingAllocator;

fn update_peak(current: usize) {
    let mut peak = PEAK_BYTES.load(Ordering::Relaxed);
    while current > peak {
        match PEAK_BYTES.compare_exchange_weak(peak, current, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => break,
            Err(observed) => peak = observed,
        }
    }
}

fn reset_alloc_stats() {
    ALLOCATED_BYTES.store(0, Ordering::Relaxed);
    PEAK_BYTES.store(0, Ordering::Relaxed);
}

fn peak_alloc_bytes() -> usize {
    PEAK_BYTES.load(Ordering::Relaxed)
}

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            let current = ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            update_peak(current);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            let current = ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            update_peak(current);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        ALLOCATED_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let new_ptr = unsafe { System.realloc(ptr, layout, new_size) };
        if !new_ptr.is_null() {
            if new_size >= layout.size() {
                let delta = new_size - layout.size();
                if delta > 0 {
                    let current = ALLOCATED_BYTES.fetch_add(delta, Ordering::Relaxed) + delta;
                    update_peak(current);
                }
            } else {
                ALLOCATED_BYTES.fetch_sub(layout.size() - new_size, Ordering::Relaxed);
            }
        }
        new_ptr
    }
}

struct BenchStats {
    total_us: u128,
    min_us: u128,
    max_us: u128,
    parts_count: usize,
    total_peak_bytes: u128,
    min_peak_bytes: usize,
    max_peak_bytes: usize,
}

impl BenchStats {
    fn avg_us(&self, runs: usize) -> f64 {
        self.total_us as f64 / runs as f64
    }

    fn avg_peak_bytes(&self, runs: usize) -> f64 {
        self.total_peak_bytes as f64 / runs as f64
    }
}

fn benchmark_slices(input: &str, runs: usize) -> BenchStats {
    let mut total_us: u128 = 0;
    let mut min_us: u128 = u128::MAX;
    let mut max_us: u128 = 0;
    let mut parts_count: usize = 0;
    let mut total_peak_bytes: u128 = 0;
    let mut min_peak_bytes: usize = usize::MAX;
    let mut max_peak_bytes: usize = 0;

    for _ in 0..runs {
        reset_alloc_stats();
        let started = Instant::now();
        let result = partition(input);
        let elapsed_us = started.elapsed().as_micros();
        let peak_bytes = peak_alloc_bytes();

        parts_count = result.len();
        drop(result);
        total_us += elapsed_us;
        min_us = min_us.min(elapsed_us);
        max_us = max_us.max(elapsed_us);
        total_peak_bytes += peak_bytes as u128;
        min_peak_bytes = min_peak_bytes.min(peak_bytes);
        max_peak_bytes = max_peak_bytes.max(peak_bytes);
    }

    BenchStats {
        total_us,
        min_us,
        max_us,
        parts_count,
        total_peak_bytes,
        min_peak_bytes,
        max_peak_bytes,
    }
}

fn benchmark_old(input: &str, runs: usize) -> BenchStats {
    let mut total_us: u128 = 0;
    let mut min_us: u128 = u128::MAX;
    let mut max_us: u128 = 0;
    let mut parts_count: usize = 0;
    let mut total_peak_bytes: u128 = 0;
    let mut min_peak_bytes: usize = usize::MAX;
    let mut max_peak_bytes: usize = 0;

    for _ in 0..runs {
        reset_alloc_stats();
        let started = Instant::now();
        let result = partition_old(input.to_string());
        let elapsed_us = started.elapsed().as_micros();
        let peak_bytes = peak_alloc_bytes();

        parts_count = result.len();
        drop(result);
        total_us += elapsed_us;
        min_us = min_us.min(elapsed_us);
        max_us = max_us.max(elapsed_us);
        total_peak_bytes += peak_bytes as u128;
        min_peak_bytes = min_peak_bytes.min(peak_bytes);
        max_peak_bytes = max_peak_bytes.max(peak_bytes);
    }

    BenchStats {
        total_us,
        min_us,
        max_us,
        parts_count,
        total_peak_bytes,
        min_peak_bytes,
        max_peak_bytes,
    }
}

fn benchmark_case(input: &str, runs: usize) {
    let slices = benchmark_slices(input, runs);
    let old = benchmark_old(input, runs);

    let slices_avg = slices.avg_us(runs);
    let old_avg = old.avg_us(runs);
    let slices_avg_peak = slices.avg_peak_bytes(runs);
    let old_avg_peak = old.avg_peak_bytes(runs);
    let speedup = if slices_avg > 0.0 {
        old_avg / slices_avg
    } else {
        f64::INFINITY
    };
    let memory_multiplier = if slices_avg_peak > 0.0 {
        old_avg_peak / slices_avg_peak
    } else {
        f64::INFINITY
    };

    println!("input={:?} len={} runs={}", input, input.len(), runs);
    println!(
        "  slices: parts={} avg_us={:.2} min_us={} max_us={}",
        slices.parts_count,
        slices_avg,
        slices.min_us,
        slices.max_us
    );
    println!(
        "          avg_peak_kb={:.2} min_peak_kb={:.2} max_peak_kb={:.2}",
        slices_avg_peak / 1024.0,
        slices.min_peak_bytes as f64 / 1024.0,
        slices.max_peak_bytes as f64 / 1024.0
    );
    println!(
        "  old:    parts={} avg_us={:.2} min_us={} max_us={}",
        old.parts_count,
        old_avg,
        old.min_us,
        old.max_us
    );
    println!(
        "          avg_peak_kb={:.2} min_peak_kb={:.2} max_peak_kb={:.2}",
        old_avg_peak / 1024.0,
        old.min_peak_bytes as f64 / 1024.0,
        old.max_peak_bytes as f64 / 1024.0
    );
    println!("  speedup old/slices = {:.2}x", speedup);
    println!("  memory old/slices = {:.2}x", memory_multiplier);

    if slices.parts_count != old.parts_count {
        println!(
            "  WARNING: partition count mismatch (slices={} old={})",
            slices.parts_count, old.parts_count
        );
    }
}

fn main() {
    const RUNS: usize = 20;

    let mut arg = env::args().skip(1);
    let Some(input) = arg.next() else {
        eprintln!("usage: ./run_side_quest <input>");
        eprintln!("note: benchmark uses {} runs per invocation", RUNS);
        return;
    };

    benchmark_case(&input, RUNS);
}
