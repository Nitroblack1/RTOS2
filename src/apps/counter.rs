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
        // Continue counting (removed logging to prevent unprivileged interrupt disable)
        if count % 800 == 0 {
            // Milestone reached (no logging)
        }
        yield_cpu();
    }
}