//! Data Logger Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 10, stack_size = 384, name = "data_logger")]
pub unsafe extern "C" fn data_logger() -> ! {
    // Removed RTT to prevent unprivileged interrupt disable issues
    let mut data_points = 0u32;

    loop {
        data_points = data_points.wrapping_add(1);

        // Simulate data logging without RTT output to prevent crash
        // Data would be logged to kernel via syscall in real implementation

        yield_cpu();
    }
}