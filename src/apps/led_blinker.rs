//! LED Blinker Application

use crate::app_syscalls::*;
use app_macros::app;

#[app(id = 0, stack_size = 256, name = "led_blinker")]
pub unsafe extern "C" fn led_blinker() -> ! {
    // Demonstrate new Driver Framework usage
    let task_id = 0; // This task's ID
    let mut counter = 0u32;

    // Request access to GPIO peripheral (index 0 = GPIO)
    let _result = request_peripheral_access(0, task_id, false);

    loop {
        counter = counter.wrapping_add(1);

        // Use new Driver Framework for secure GPIO access
        // Port A (0), Pin 5 (built-in LED on many STM32 boards)
        if counter % 2000 == 0 {
            // Toggle LED every 2000 iterations using Driver Framework
            let _result = driver_gpio_toggle(task_id, 0, 5); // Port A, Pin 5
        }

        yield_cpu();
    }
}