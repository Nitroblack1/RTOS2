//! Shared Counter Application - Tests race conditions and memory synchronization
//! Multiple instances of this app will increment a shared counter

use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 12, stack_size = 4096, name = "shared_counter")]
pub unsafe extern "C" fn shared_counter() -> ! {
    rprintln!("[SHARED_COUNTER] Started - testing race conditions");

    // Try to get or create shared counter region
    let mut shared_ptr: *mut u32;

    rprintln!("[SHARED_COUNTER] Attempting to map existing counter...");

    // First try to map existing shared counter (region ID 1, same as producer)
    match map_shared_memory(1, 12) {
        Ok(ptr) => {
            shared_ptr = ptr as *mut u32;
            rprintln!("[SHARED_COUNTER] Mapped existing counter");
        },
        Err(_) => {
            rprintln!("[SHARED_COUNTER] Region 1 not found, trying region 2...");

            // Try region 2 as backup
            match map_shared_memory(2, 12) {
                Ok(ptr) => {
                    shared_ptr = ptr as *mut u32;
                    rprintln!("[SHARED_COUNTER] Mapped counter from region 2");
                },
                Err(_) => {
                    rprintln!("[SHARED_COUNTER] Creating new counter...");

                    // If mapping fails, try to create new shared counter
                    match request_shared_memory(64) {
                        Ok(region_id) => {
                            match map_shared_memory(region_id, 12) {
                                Ok(ptr) => {
                                    shared_ptr = ptr as *mut u32;
                                    // Initialize counter to 0
                                    unsafe {
                                        core::ptr::write_volatile(shared_ptr, 0u32);
                                        core::ptr::write_volatile(shared_ptr.add(1), 0u32); // Lock flag
                                    }
                                    rprintln!("[SHARED_COUNTER] Created new counter (region {})", region_id);
                                },
                                Err(_e) => {
                                    rprintln!("[SHARED_COUNTER] Failed to map own shared memory: {}", _e);
                                    loop { yield_cpu(); }
                                }
                            }
                        },
                        Err(_e) => {
                            rprintln!("[SHARED_COUNTER] Failed to create shared memory: {}", _e);
                            loop { yield_cpu(); }
                        }
                    }
                }
            }
        }
    }

    let mut local_increments = 0u32;

    loop {
        if !shared_ptr.is_null() {
            // Simple spinlock implementation for synchronization
            let lock_ptr = shared_ptr.add(1); // Lock flag is at offset 1

            // Try to acquire lock (simple spinlock)
            while unsafe { core::ptr::read_volatile(lock_ptr) } != 0 {
                // Busy wait - yield occasionally to prevent starvation
                for _ in 0..100 {
                    cortex_m::asm::nop();
                }
            }

            // Set lock
            unsafe {
                core::ptr::write_volatile(lock_ptr, 1);
            }

            // Critical section - increment shared counter
            unsafe {
                let current = core::ptr::read_volatile(shared_ptr);
                // Small delay to increase chance of race conditions for testing
                for _ in 0..50 {
                    cortex_m::asm::nop();
                }
                core::ptr::write_volatile(shared_ptr, current + 1);
                local_increments += 1;

                // Log occasionally
                if local_increments % 500 == 0 {
                    rprintln!("[SHARED_COUNTER] Local: {}", local_increments);
                }
            }

            // Release lock
            unsafe {
                core::ptr::write_volatile(lock_ptr, 0);
            }
        }

        // Send status message to other apps occasionally
        if local_increments % 500 == 0 && local_increments > 0 {
            let payload = [
                (local_increments & 0xFF) as u8,
                ((local_increments >> 8) & 0xFF) as u8,
                ((local_increments >> 16) & 0xFF) as u8,
                ((local_increments >> 24) & 0xFF) as u8,
            ];

            // Notify producer about our status
            let _ = send_message(5, 3, &payload); // Send to producer (app ID 5)
        }

        yield_cpu();

        // Longer delay to reduce contention and slow down logs
        for _ in 0..50000 {
            cortex_m::asm::nop();
        }
    }
}