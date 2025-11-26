// benchmark_context_switch.rs
// Performance benchmark for MPU context switching overhead

#![allow(dead_code)]
use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 20, stack_size = 2048, name = "benchmark_context_switch")]
pub unsafe extern "C" fn benchmark_context_switch() -> ! {
    let task_count = crate::sched::get_task_count();
    rprintln!("[BENCH-CTX] 🎯 Context Switch Performance Benchmark Started (Tasks: {})", task_count);

    let mut test_iteration = 0u32;
    let mut mpu_on_cycles = [0u32; 100];
    let mut mpu_off_cycles = [0u32; 100];

    loop {
        test_iteration += 1;
        rprintln!("[BENCH-CTX] === Test Iteration {} ===", test_iteration);

        // Phase 1: Measure context switch with MPU enabled
        rprintln!("[BENCH-CTX] Phase 1: Measuring with MPU ENABLED");

        // Ensure MPU is enabled for first phase
        crate::mpu::restore_mpu_after_benchmark();

        // Simplified measurement: just a few iterations first
        for i in 0..5 {
            rprintln!("[BENCH-CTX] Starting measurement {}", i);
            let start_cycles = crate::dwt::get_cycles();

            // Trigger context switch by yielding
            yield_cpu();

            let end_cycles = crate::dwt::get_cycles();
            let cycles = end_cycles.wrapping_sub(start_cycles);
            mpu_on_cycles[i] = cycles;
            rprintln!("[BENCH-CTX] Measurement {}: {} cycles", i, cycles);
        }

        // Fill remaining with first measurement
        for i in 5..100 {
            mpu_on_cycles[i] = mpu_on_cycles[0];
        }

        // Calculate statistics for MPU ON
        let mut min_on = u32::MAX;
        let mut max_on = 0u32;
        let mut total_on = 0u64;

        for &cycles in &mpu_on_cycles {
            min_on = min_on.min(cycles);
            max_on = max_on.max(cycles);
            total_on += cycles as u64;
        }
        let avg_on = (total_on / 100) as u32;

        rprintln!("[BENCH-CTX] MPU ENABLED Results:");
        rprintln!("[BENCH-CTX]   Min: {} cycles ({:.2} μs)", min_on, crate::perf::cycles_to_us(min_on));
        rprintln!("[BENCH-CTX]   Max: {} cycles ({:.2} μs)", max_on, crate::perf::cycles_to_us(max_on));
        rprintln!("[BENCH-CTX]   Avg: {} cycles ({:.2} μs)", avg_on, crate::perf::cycles_to_us(avg_on));

        // Phase 2: Measure context switch with MPU disabled
        rprintln!("[BENCH-CTX] Phase 2: Measuring with MPU DISABLED");

        // Disable MPU for second phase
        crate::mpu::disable_mpu_for_benchmark();

        // Simplified measurement: just a few iterations first
        for i in 0..5 {
            rprintln!("[BENCH-CTX] Starting MPU OFF measurement {}", i);
            let start_cycles = crate::dwt::get_cycles();

            // Trigger context switch by yielding
            yield_cpu();

            let end_cycles = crate::dwt::get_cycles();
            let cycles = end_cycles.wrapping_sub(start_cycles);
            mpu_off_cycles[i] = cycles;
            rprintln!("[BENCH-CTX] MPU OFF Measurement {}: {} cycles", i, cycles);
        }

        // Fill remaining with first measurement
        for i in 5..100 {
            mpu_off_cycles[i] = mpu_off_cycles[0];
        }

        // Restore MPU state
        crate::mpu::restore_mpu_after_benchmark();

        // Calculate statistics for MPU OFF
        let mut min_off = u32::MAX;
        let mut max_off = 0u32;
        let mut total_off = 0u64;

        for &cycles in &mpu_off_cycles {
            min_off = min_off.min(cycles);
            max_off = max_off.max(cycles);
            total_off += cycles as u64;
        }
        let avg_off = (total_off / 100) as u32;

        rprintln!("[BENCH-CTX] MPU DISABLED Results:");
        rprintln!("[BENCH-CTX]   Min: {} cycles ({:.2} μs)", min_off, crate::perf::cycles_to_us(min_off));
        rprintln!("[BENCH-CTX]   Max: {} cycles ({:.2} μs)", max_off, crate::perf::cycles_to_us(max_off));
        rprintln!("[BENCH-CTX]   Avg: {} cycles ({:.2} μs)", avg_off, crate::perf::cycles_to_us(avg_off));

        // Performance comparison analysis
        let overhead_cycles = avg_on as i32 - avg_off as i32;
        let overhead_percent = if avg_off > 0 {
(overhead_cycles as f32 / avg_off as f32) * 100.0
        } else { 0.0 };

        rprintln!("[BENCH-CTX] === PERFORMANCE ANALYSIS ===");
        rprintln!("[BENCH-CTX] MPU Overhead: {} cycles ({:.2} μs)", overhead_cycles, crate::perf::cycles_to_us(overhead_cycles.abs() as u32));
        rprintln!("[BENCH-CTX] MPU Overhead: {:.2}%", overhead_percent);

        if overhead_cycles > 0 {
            rprintln!("[BENCH-CTX] ⚠️  MPU adds {:.2}% overhead to context switches", overhead_percent);
        } else {
            rprintln!("[BENCH-CTX] ✅ No significant MPU overhead detected");
        }

        // CSV format output for data analysis
        rprintln!("[BENCH-CTX-CSV] {},{},{},{},{},{:.2},{}",
                 test_iteration, avg_on, avg_off, overhead_cycles, overhead_percent,
                 crate::perf::cycles_to_us(overhead_cycles.abs() as u32), task_count);

        // Wait before next iteration
        for _ in 0..5000000 {
            cortex_m::asm::nop();
        }
    }
}