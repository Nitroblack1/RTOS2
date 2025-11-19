//! Memory Violator App - Simple MPU Protection Demo
//! Proves MPU works by showing memory isolation between apps

use app_macros::app;
use crate::app_syscalls::*;
use rtt_target::rprintln;

#[app(id = 15, stack_size = 1024, name = "memory_violator")]
pub unsafe extern "C" fn memory_violator() -> ! {
    rprintln!("[MEMORY_VIOLATOR] 🎯 MPU Protection Test - Attempting Cross-App Memory Access");

    // Write a known pattern in our own memory
    let my_data = [0xCAFEBABEu32, 0xDEADBEEFu32, 0x12345678u32, 0xABCDEF00u32];
    rprintln!("[MEMORY_VIOLATOR] ✅ Our memory: 0x{:08x} = 0x{:08x}",
              my_data.as_ptr() as u32, my_data[0]);

    let mut test_count = 0u32;

    loop {
        test_count += 1;
        rprintln!("[MEMORY_VIOLATOR] 🔍 Test #{}: Attempting unauthorized memory access...", test_count);

        // Try to access Producer's memory region - THIS SHOULD BE BLOCKED BY MPU
        rprintln!("[MEMORY_VIOLATOR] 🎯 Attempting to read Producer app region (0x20010000)...");
        let producer_ptr = 0x20010000 as *const u32;

        // This should trigger MemManage fault if MPU is working
        let result = core::ptr::read_volatile(producer_ptr);
        rprintln!("[MEMORY_VIOLATOR] ❌ SECURITY BREACH! Read succeeded: 0x{:08x}", result);
        rprintln!("[MEMORY_VIOLATOR] 🚨 MPU did NOT block unauthorized access!");

        // Wait and try again
        for _ in 0..2000000 {
            cortex_m::asm::nop();
        }

        yield_cpu();

        if test_count >= 3 {
            rprintln!("[MEMORY_VIOLATOR] 🔄 Restarting tests...");
            test_count = 0;
        }
    }
}