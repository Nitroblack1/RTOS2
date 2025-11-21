//! Counter Application

use crate::app_syscalls::*;
use app_macros::app;
use rtt_target::rprintln;

#[app(id = 2, stack_size = 384, name = "counter")]
pub unsafe extern "C" fn counter() -> ! {
    // rprintln!("[APP counter] started"); // DISABLED for benchmark focus
    let mut count = 0u32;
    loop {
        count = count.wrapping_add(1);
        // Only log every 10000 iterations (reduced for benchmark focus)
        if count % 10000 == 0 {
            rprintln!("[COUNT] {}", count);
        }
        yield_cpu();
    }
}