// benchmark_ipc_latency.rs
// Performance benchmark for IPC message latency and throughput

#![allow(dead_code)]
use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 21, stack_size = 2048, name = "benchmark_ipc_latency")]
pub unsafe extern "C" fn benchmark_ipc_latency() -> ! {
    rprintln!("[BENCH-IPC] 🎯 IPC Latency & Throughput Benchmark Started");

    let mut test_iteration = 0u32;
    let mut send_latencies = [0u32; 50];
    let _receive_latencies = [0u32; 50]; // Reserved for future use

    loop {
        test_iteration += 1;
        rprintln!("[BENCH-IPC] === Test Iteration {} ===", test_iteration);

        // Phase 1: Message Send Latency Benchmark
        rprintln!("[BENCH-IPC] Phase 1: Measuring message SEND latency");

        for i in 0..50 {
            let start_cycles = crate::dwt::get_cycles();

            // Send IPC message to next app (producer->consumer pattern)
            let message_data = [0xCAFEBABE, 0xDEADBEEF, i as u32, test_iteration];
            let message_bytes: &[u8] = unsafe {
                core::slice::from_raw_parts(
                    message_data.as_ptr() as *const u8,
                    message_data.len() * 4
                )
            };
            match send_message(6, 1, message_bytes) { // Send to consumer (app id 6)
                Ok(_) => {
                    let end_cycles = crate::dwt::get_cycles();
                    send_latencies[i] = end_cycles.wrapping_sub(start_cycles);
                },
                Err(_) => {
                    send_latencies[i] = 0; // Mark failed sends
                }
            }

            // Small delay between sends
            for _ in 0..1000 {
                cortex_m::asm::nop();
            }
        }

        // Calculate send latency statistics
        let mut min_send = u32::MAX;
        let mut max_send = 0u32;
        let mut total_send = 0u64;
        let mut valid_sends = 0u32;

        for &cycles in &send_latencies {
            if cycles > 0 {
                min_send = min_send.min(cycles);
                max_send = max_send.max(cycles);
                total_send += cycles as u64;
                valid_sends += 1;
            }
        }

        let avg_send = if valid_sends > 0 { (total_send / valid_sends as u64) as u32 } else { 0 };

        rprintln!("[BENCH-IPC] Message SEND Results ({} valid):", valid_sends);
        rprintln!("[BENCH-IPC]   Min: {} cycles ({:.2} μs)", min_send, crate::perf::cycles_to_us(min_send));
        rprintln!("[BENCH-IPC]   Max: {} cycles ({:.2} μs)", max_send, crate::perf::cycles_to_us(max_send));
        rprintln!("[BENCH-IPC]   Avg: {} cycles ({:.2} μs)", avg_send, crate::perf::cycles_to_us(avg_send));

        // Phase 2: Shared Memory Allocation Latency
        rprintln!("[BENCH-IPC] Phase 2: Measuring shared memory allocation latency");

        let mut alloc_latencies = [0u32; 20];

        for i in 0..20 {
            let start_cycles = crate::dwt::get_cycles();

            match request_shared_memory(256) { // Allocate 256 bytes
                Ok(_region_id) => {
                    let end_cycles = crate::dwt::get_cycles();
                    alloc_latencies[i] = end_cycles.wrapping_sub(start_cycles);
                },
                Err(_) => {
                    alloc_latencies[i] = 0; // Mark failed allocations
                }
            }

            // Delay between allocations
            for _ in 0..10000 {
                cortex_m::asm::nop();
            }
        }

        // Calculate allocation latency statistics
        let mut min_alloc = u32::MAX;
        let mut max_alloc = 0u32;
        let mut total_alloc = 0u64;
        let mut valid_allocs = 0u32;

        for &cycles in &alloc_latencies {
            if cycles > 0 {
                min_alloc = min_alloc.min(cycles);
                max_alloc = max_alloc.max(cycles);
                total_alloc += cycles as u64;
                valid_allocs += 1;
            }
        }

        let avg_alloc = if valid_allocs > 0 { (total_alloc / valid_allocs as u64) as u32 } else { 0 };

        rprintln!("[BENCH-IPC] Shared Memory ALLOCATION Results ({} valid):", valid_allocs);
        rprintln!("[BENCH-IPC]   Min: {} cycles ({:.2} μs)", min_alloc, crate::perf::cycles_to_us(min_alloc));
        rprintln!("[BENCH-IPC]   Max: {} cycles ({:.2} μs)", max_alloc, crate::perf::cycles_to_us(max_alloc));
        rprintln!("[BENCH-IPC]   Avg: {} cycles ({:.2} μs)", avg_alloc, crate::perf::cycles_to_us(avg_alloc));

        // Phase 3: Memory Access Pattern Performance
        rprintln!("[BENCH-IPC] Phase 3: Measuring memory access patterns");

        // Test different memory access patterns
        let test_data = [0x12345678u32; 64]; // 256 bytes test array
        let mut access_times = [0u32; 3];

        // Sequential access
        let start = crate::dwt::get_cycles();
        let mut sum = 0u32;
        for &value in &test_data {
            sum = sum.wrapping_add(value);
        }
        let end = crate::dwt::get_cycles();
        access_times[0] = end.wrapping_sub(start);

        // Random access pattern
        let start = crate::dwt::get_cycles();
        let mut sum2 = 0u32;
        for i in 0..64 {
            let idx = (i * 7) % 64; // Pseudo-random pattern
            sum2 = sum2.wrapping_add(test_data[idx]);
        }
        let end = crate::dwt::get_cycles();
        access_times[1] = end.wrapping_sub(start);

        // Strided access pattern
        let start = crate::dwt::get_cycles();
        let mut sum3 = 0u32;
        for i in (0..64).step_by(4) {
            sum3 = sum3.wrapping_add(test_data[i]);
        }
        let end = crate::dwt::get_cycles();
        access_times[2] = end.wrapping_sub(start);

        rprintln!("[BENCH-IPC] Memory Access Pattern Results:");
        rprintln!("[BENCH-IPC]   Sequential: {} cycles ({:.2} μs)", access_times[0], crate::perf::cycles_to_us(access_times[0]));
        rprintln!("[BENCH-IPC]   Random: {} cycles ({:.2} μs)", access_times[1], crate::perf::cycles_to_us(access_times[1]));
        rprintln!("[BENCH-IPC]   Strided: {} cycles ({:.2} μs)", access_times[2], crate::perf::cycles_to_us(access_times[2]));

        // Prevent optimization
        if sum.wrapping_add(sum2).wrapping_add(sum3) == 0 {
            rprintln!("[BENCH-IPC] Unexpected zero sum");
        }

        // Throughput calculation
        let send_throughput = if avg_send > 0 {
            180_000_000.0 / avg_send as f32 // Messages per second at 180MHz
        } else { 0.0 };

        rprintln!("[BENCH-IPC] === THROUGHPUT ANALYSIS ===");
        rprintln!("[BENCH-IPC] Send Throughput: {:.0} messages/second", send_throughput);
        rprintln!("[BENCH-IPC] Allocation Throughput: {:.0} allocs/second",
                 if avg_alloc > 0 { 180_000_000.0 / avg_alloc as f32 } else { 0.0 });

        // CSV format output for data analysis
        rprintln!("[BENCH-IPC-CSV] {},{},{},{},{},{:.2},{:.0}",
                 test_iteration, avg_send, avg_alloc, access_times[0],
                 access_times[1], crate::perf::cycles_to_us(avg_send), send_throughput);

        // Wait before next iteration
        for _ in 0..8000000 {
            cortex_m::asm::nop();
        }
    }
}