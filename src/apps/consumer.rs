//! Consumer Application - Tests IPC and Shared Memory
//! Consumes data from producer via shared memory and IPC

use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 11, stack_size = 2048, name = "consumer")]
pub unsafe extern "C" fn consumer() -> ! {
    rprintln!("[CONSUMER] Started - waiting for producer messages");

    let mut processed_count = 0u32;

    // Wait for producer to create shared memory and then map it
    let mut shared_ptr: *mut u32 = core::ptr::null_mut();
    let mut mapped = false;

    loop {
        // Check for messages from producer
        if has_messages() {
            match receive_message() {
                Ok(msg) => {
                    // Extract iteration number from payload
                    let iteration = u32::from_le_bytes([
                        msg.payload[0], msg.payload[1], msg.payload[2], msg.payload[3]
                    ]);

                    // Map shared memory if not done yet
                    if !mapped {
                        match map_shared_memory(1, 5) { // Try to map region 1 from producer (app ID 5)
                            Ok(ptr) => {
                                shared_ptr = ptr as *mut u32;
                                mapped = true;
                                rprintln!("[CONSUMER] Mapped shared memory at: 0x{:08x}", ptr as u32);
                            },
                            Err(e) => {
                                rprintln!("[CONSUMER] Failed to map shared memory: {}", e);
                                continue;
                            }
                        }
                    }

                    // Read data from shared memory
                    if mapped {
                        unsafe {
                            let value1 = core::ptr::read_volatile(shared_ptr);
                            let value2 = core::ptr::read_volatile(shared_ptr.add(1));
                            let value3 = core::ptr::read_volatile(shared_ptr.add(2));

                            processed_count = processed_count.wrapping_add(1);

                            // Debug log for first few messages
                            if processed_count <= 3 {
                                rprintln!("[CONSUMER] Read data: {},{},{}", value1, value2, value3);
                            }

                            // Verify data integrity
                            if value1 == iteration && value2 == iteration * 2 && value3 == iteration * 3 {
                                if processed_count <= 3 || processed_count % 25 == 0 {
                                    rprintln!("[CONSUMER] ✓ Message #{} processed", iteration);
                                }
                            } else {
                                rprintln!("[CONSUMER] ✗ Data integrity error! Msg #{}", iteration);
                            }
                        }
                    }

                    // Send acknowledgment back to producer occasionally
                    if processed_count % 25 == 0 {
                        let ack_payload = [(processed_count & 0xFF) as u8];
                        match send_message(5, 2, &ack_payload) { // Send ACK to producer (app ID 5)
                            Ok(_) => rprintln!("[CONSUMER] Sent ACK for {} processed messages", processed_count),
                            Err(e) => rprintln!("[CONSUMER] Failed to send ACK: {}", e),
                        }
                    }
                },
                Err(e) => {
                    rprintln!("[CONSUMER] Failed to receive message: {}", e);
                }
            }
        }

        // Test stack boundary protection occasionally
        if processed_count % 150 == 0 && processed_count > 0 {
            rprintln!("[CONSUMER] Testing stack boundary protection...");
            test_stack_overflow_protection();
        }

        // Yield to allow other apps to run
        yield_cpu();
    }
}