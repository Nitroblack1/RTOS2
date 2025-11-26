use rtt_target::rprintln;

// DWT register addresses for Cortex-M4
const DWT_CTRL: *mut u32 = 0xE0001000 as *mut u32;
const DWT_CYCCNT: *mut u32 = 0xE0001004 as *mut u32;
const DWT_CTRL_CYCCNTENA: u32 = 1 << 0;

pub struct CycleCounter;

impl CycleCounter {
    pub fn init() {
        unsafe {
            // Check if DWT exists (read should not fault)
            let dwt_ctrl = core::ptr::read_volatile(DWT_CTRL);

            // Enable DWT cycle counter only if not already enabled
            if (dwt_ctrl & DWT_CTRL_CYCCNTENA) == 0 {
                core::ptr::write_volatile(DWT_CTRL, dwt_ctrl | DWT_CTRL_CYCCNTENA);
            }

            // Reset cycle counter to 0
            core::ptr::write_volatile(DWT_CYCCNT, 0);
        }
        rprintln!("[BENCHMARK] DWT cycle counter initialized");
    }

    #[inline(always)]
    pub fn get_cycles() -> u32 {
        unsafe { core::ptr::read_volatile(DWT_CYCCNT) }
    }

    pub fn cycles_to_us(cycles: u32) -> f32 {
        // STM32F446 runs at 180MHz
        const CLOCK_FREQ_MHZ: f32 = 180.0;
        cycles as f32 / CLOCK_FREQ_MHZ
    }
}

// Simple global counters for performance tracking
pub static mut MPU_REGION_CYCLES: u32 = 0;
pub static mut CONTEXT_SWITCH_CYCLES: u32 = 0;

pub fn log_mpu_region_time(cycles: u32) {
    unsafe {
        MPU_REGION_CYCLES += cycles;
    }
    rprintln!("[BENCHMARK] MPU region: {} cycles ({:.3} μs)", cycles, CycleCounter::cycles_to_us(cycles));
}

pub fn log_context_switch_time(cycles: u32) {
    unsafe {
        CONTEXT_SWITCH_CYCLES += cycles;
    }
    rprintln!("[BENCHMARK] Context switch: {} cycles ({:.3} μs)", cycles, CycleCounter::cycles_to_us(cycles));
}

pub fn print_final_results() {
    unsafe {
        if MPU_REGION_CYCLES > 0 {
            rprintln!("[BENCHMARK] Total MPU regions: {} cycles ({:.3} μs)",
                     MPU_REGION_CYCLES, CycleCounter::cycles_to_us(MPU_REGION_CYCLES));
        }
        if CONTEXT_SWITCH_CYCLES > 0 {
            rprintln!("[BENCHMARK] Total context switches: {} cycles ({:.3} μs)",
                     CONTEXT_SWITCH_CYCLES, CycleCounter::cycles_to_us(CONTEXT_SWITCH_CYCLES));
        }
    }
}