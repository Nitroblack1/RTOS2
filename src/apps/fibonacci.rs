//! Fibonacci Calculator Application
//! Computes fibonacci numbers and displays milestones using Tock-style syscalls

use app_macros::app;
static mut APP1_COUNTER: u32 = 0;
static mut APP1_FIB_A: u32 = 0;
static mut APP1_FIB_B: u32 = 1;
static mut APP1_FIB_COUNT: u32 = 0;

#[app(id = 1, stack_size = 512, name = "fibonacci")]
pub unsafe extern "C" fn fibonacci() -> ! {
    // First execution log (only once)
    crate::app_syscalls::debug_print(1, "Fibonacci app started");

    use crate::app_syscalls::yield_cpu;
    let mut fib_a = 0u32;
    let mut fib_b = 1u32;
    let mut iterations = 0u32;

    loop {
        iterations = iterations.wrapping_add(1);

        // Increment activity counter for performance monitoring
        crate::app_syscalls::increment_activity(1); // Task ID 1

        // Increment loop iterations every 200 iterations
        if iterations % 200 == 0 {
            crate::app_syscalls::increment_iterations(1);
        }

        // Compute next fibonacci number
        let next = fib_a.wrapping_add(fib_b);
        fib_a = fib_b;
        fib_b = next;

        // Continue computing (removed logging to prevent unprivileged interrupt disable)
        if iterations % 1000 == 0 {
            // Milestone reached (no logging)
        }

        yield_cpu();
    }
}