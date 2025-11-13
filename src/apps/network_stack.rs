//! Network Stack Application
//! Simulates network packet processing using Tock-style syscalls

use app_macros::app;

static mut PACKET_COUNT: u32 = 0;
static mut BYTES_PROCESSED: u32 = 0;

#[app(id = 6, stack_size = 512, name = "network_stack")]
pub unsafe extern "C" fn network_stack() -> ! {
    // Removed rtt_target::rprintln! to prevent unprivileged interrupt disable

    crate::app_syscalls::debug_print(6, "Network Stack Application started - processing packets!");

    loop {
        unsafe {
            PACKET_COUNT = PACKET_COUNT.wrapping_add(1);
            BYTES_PROCESSED = BYTES_PROCESSED.wrapping_add(64); // Simulate 64-byte packets

            // Report every 1000 packets
            if PACKET_COUNT % 1000 == 0 {
                // Removed cortex_m::interrupt::disable/enable to prevent unprivileged fault
                let _packets = core::ptr::read_volatile(core::ptr::addr_of!(PACKET_COUNT));
                let _bytes = core::ptr::read_volatile(core::ptr::addr_of!(BYTES_PROCESSED));
                crate::app_syscalls::debug_print(6, "Network packet milestone reached");
                // Removed cortex_m::interrupt::enable() - not needed for simple volatile reads
            }
        }

        // Use Tock-style cooperative yielding
        crate::app_syscalls::yield_cpu();
    }
}
