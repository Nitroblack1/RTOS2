//! Counter Application

use crate::app_syscalls::*;
use app_macros::app;
use rtt_target::rprintln;

#[app(id = 2, stack_size = 384, name = "counter")]
pub unsafe extern "C" fn counter() -> ! {
    // Check Thread mode execution state immediately on app entry
    let mut control: u32;
    let mut psp: u32;
    let mut msp: u32;
    unsafe {
        core::arch::asm!("mrs {}, CONTROL", out(reg) control, options(nomem, nostack));
        core::arch::asm!("mrs {}, PSP", out(reg) psp, options(nomem, nostack));
        core::arch::asm!("mrs {}, MSP", out(reg) msp, options(nomem, nostack));
    }

    let is_privileged = (control & 0x01) == 0;
    let uses_psp = (control & 0x02) != 0;
    rprintln!("[APP counter] ENTRY STATE: CONTROL=0x{:08x} ({}|{}), PSP=0x{:08x}, MSP=0x{:08x}",
             control,
             if is_privileged { "PRIV" } else { "UNPRIV" },
             if uses_psp { "PSP" } else { "MSP" },
             psp, msp);

    if uses_psp && !is_privileged {
        rprintln!("[APP counter] ✅ SUCCESS: Running in Thread mode with PSP + Unprivileged!");
    } else {
        rprintln!("[APP counter] ❌ ERROR: Thread mode transition failed!");
    }

    rprintln!("[APP counter] started");
    let mut count = 0u32;
    loop {
        count = count.wrapping_add(1);
        // Only log every 800 iterations for testing
        if count % 800 == 0 {
            rprintln!("[COUNT] {}", count);
        }
        yield_cpu();
    }
}