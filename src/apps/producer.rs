//! Producer Application - Tests IPC and Shared Memory
//! Produces data and sends it to consumer via shared memory and IPC

use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 10, stack_size = 4096, name = "producer")]
pub unsafe extern "C" fn producer() -> ! {
    // rprintln!("[PRODUCER] Started - testing IPC and shared memory");

    let mut iteration = 0u32;

    // Request shared memory region (256 bytes)
    let shared_region = match request_shared_memory(256) {
        Ok(region_id) => {
            // rprintln!("[PRODUCER] Allocated shared memory region: {}", region_id);
            region_id
        },
        Err(e) => {
            // rprintln!("[PRODUCER] Failed to allocate shared memory: {}", e);
            loop { yield_cpu(); }
        }
    };

    // Get pointer to shared memory (자신이 생성한 공유 메모리는 바로 매핑 가능)
    let shared_ptr = match map_shared_memory(shared_region, 10) { // Map from our own app (ID 10)
        Ok(ptr) => {
            // rprintln!("[PRODUCER] Mapped shared memory at: 0x{:08x}", ptr as u32);
            ptr as *mut u32
        },
        Err(e) => {
            // rprintln!("[PRODUCER] Failed to map shared memory: {}", e);
            loop { yield_cpu(); }
        }
    };

    loop {
        iteration = iteration.wrapping_add(1);

        // Write data to shared memory
        unsafe {
            core::ptr::write_volatile(shared_ptr, iteration);
            core::ptr::write_volatile(shared_ptr.add(1), iteration * 2);
            core::ptr::write_volatile(shared_ptr.add(2), iteration * 3);

            // Debug log for first few iterations
            if iteration == 1 {
                rprintln!("[PRODUCER] OK");
            }
        }

        // Send notification message to consumer (app ID 11)
        let payload = iteration.to_le_bytes();

        match send_message(6, 1, &payload) { // Send to consumer (actual app ID 6)
            Ok(_) => {
                if iteration % 100 == 0 {
                    rprintln!("[PRODUCER] Sent message #{}", iteration);
                }
            },
            Err(e) => {
                rprintln!("[PRODUCER] Failed to send message: {}", e);
            }
        }

        // Test MPU protection occasionally
        if iteration % 100 == 0 {
            rprintln!("[PRODUCER] Testing MPU protection...");
            test_memory_violation();
        }

        // Yield to allow other apps to run
        yield_cpu();

        // Longer delay for slower logging
        for _ in 0..100000 {
            cortex_m::asm::nop();
        }
    }
}