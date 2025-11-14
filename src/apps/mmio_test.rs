//! MMIO Access Test Application - Tests Direct Hardware Access Prevention

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 11, stack_size = 512, name = "mmio_test")]
pub unsafe extern "C" fn mmio_test() -> ! {
    let task_id = 11;
    let mut counter = 0u32;

    // First demonstrate the SAFE way - through Driver Framework
    let _result = request_peripheral_access(0, task_id, false);

    loop {
        counter = counter.wrapping_add(1);

        if counter % 10000 == 0 {
            // Test 1: Safe GPIO access through Driver Framework
            let _result = driver_gpio_toggle(task_id, 0, 5);
        }

        if counter % 50000 == 0 {
            // Test 2: Attempt DANGEROUS direct MMIO access
            // This should trigger MPU fault in unprivileged mode
            test_direct_mmio_access();
        }

        yield_cpu();
    }
}

/// Test function that attempts direct MMIO access
/// This should fail and trigger MPU fault when app runs in unprivileged mode
unsafe fn test_direct_mmio_access() {
    // STM32F446 GPIO Port A registers (should be blocked by MPU)
    const GPIOA_BASE: u32 = 0x4002_0000;
    const GPIOA_BSRR: u32 = GPIOA_BASE + 0x18;  // Bit Set/Reset Register

    // Attempt to write directly to GPIO hardware (DANGEROUS!)
    // In unprivileged mode with proper MPU, this should trigger fault
    let gpio_bsrr = GPIOA_BSRR as *mut u32;

    // Try to set pin 5 (LED) - direct hardware manipulation
    // This bypasses all driver security and should be prevented
    *gpio_bsrr = 1 << 5; // Set bit 5 (should trigger MPU fault)
}