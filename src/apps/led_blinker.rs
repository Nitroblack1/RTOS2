//! LED Blinker Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 0, stack_size = 256, name = "led_blinker")]
pub unsafe extern "C" fn led_blinker() -> ! {
    // Removed RTT to prevent unprivileged interrupt disable issues
    let mut counter = 0u32;
    loop {
        counter = counter.wrapping_add(1);
        // LED blinking logic without RTT output to prevent RTT crashes
        yield_cpu();
    }
}