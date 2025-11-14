//! Math Calculator Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 5, stack_size = 416, name = "math_calculator")]
pub unsafe extern "C" fn math_calculator() -> ! {
    // Removed RTT to prevent unprivileged interrupt disable issues
    let mut operations = 0u32;
    let mut result = 1u32;

    loop {
        operations = operations.wrapping_add(1);

        // Simple math operation
        result = result.wrapping_mul(2).wrapping_add(1) % 1000;

        // Math operations continue without RTT output to prevent crashes

        yield_cpu();
    }
}