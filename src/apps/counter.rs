//! Counter Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 2, stack_size = 384, name = "counter")]
pub unsafe extern "C" fn counter() -> ! {
    // Removed rprintln! calls to prevent unprivileged interrupt disable
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