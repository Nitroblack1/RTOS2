//! Counter Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 2, stack_size = 384, name = "counter")]
pub unsafe extern "C" fn counter() -> ! {
    // First execution log (only once)
    debug_print(2, "Counter app started");

    let mut count = 0u32;
    loop {
        count = count.wrapping_add(1);

        // Increment activity counter for performance monitoring
        increment_activity(2); // Task ID 2

        // Increment loop iterations every 50 iterations
        if count % 50 == 0 {
            increment_iterations(2);
        }

        // Continue counting (removed logging to prevent unprivileged interrupt disable)
        if count % 800 == 0 {
            // Milestone reached (no logging)
        }

        // MPU Protection Test disabled temporarily to check if this causes RTT hang
        // if count % 10000 == 0 {
        //     test_mpu_protection();
        // }
        yield_cpu();
    }
}

/// Test MPU protection by attempting illegal memory access
/// This should trigger MemoryManagement fault in unprivileged mode
unsafe fn test_mpu_protection() {
    // Test 1: Attempt to access another task's stack memory
    // LED Blinker task (Task 0) stack is around 0x20003000 range
    let foreign_stack = 0x20003000 as *mut u32;

    // This access should be BLOCKED by MPU process isolation
    // and trigger MemoryManagement fault
    let _illegal_read = core::ptr::read_volatile(foreign_stack);

    // Test 2: Attempt direct MMIO access (hardware registers)
    // GPIO Port A Base Address - should be blocked in unprivileged mode
    const GPIOA_BASE: u32 = 0x4002_0000;
    let gpio_reg = GPIOA_BASE as *mut u32;

    // This should trigger MPU fault for peripheral access
    let _illegal_mmio = core::ptr::read_volatile(gpio_reg);

    // Test 3: Attempt to write to Flash memory (read-only for user tasks)
    let flash_addr = 0x08000000 as *mut u32;

    // This should trigger MPU fault for flash write attempt
    // core::ptr::write_volatile(flash_addr, 0xDEADBEEF);
}