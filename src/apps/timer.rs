//! Timer Application
//! Counts seconds and tracks elapsed time using Tock-style syscalls

use app_macros::app;

static mut TIMER_TICKS: u32 = 0;
static mut TIMER_SECONDS: u32 = 0;

#[app(id = 2, stack_size = 1024, name = "timer")]
pub unsafe extern "C" fn timer() -> ! {
    // SIMPLIFIED timer for debugging - remove complex operations
    crate::app_syscalls::debug_print(3, "Simple Timer Started");

    let mut count = 0u32;
    loop {
        count = count.wrapping_add(1);

        // Only log every 10000 iterations
        if count % 10000 == 0 {
            crate::app_syscalls::debug_print(3, "Timer tick");
        }

        // Simple yield without interrupt manipulation
        crate::app_syscalls::yield_cpu();
    }
}
