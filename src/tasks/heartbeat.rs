// LIBRARY INCLUDES
use nucleo_l152re::{system, GpioLevel, OutputPin, OutputStyle, UninitPin, PA5};

const HALF_PERIOD_MS: u32 = 500; // 1 Hz blink

// blink LD2 (PA5) at 1 Hz -> visible "scheduler is running" indicator, lowest priority
pub extern "C" fn heartbeat_task(_arg: *mut ()) -> ! {
    let mut ld2 = PA5.into_output(OutputStyle::PushPull);
    loop {
        ld2.set(GpioLevel::High);
        // delay_ms can only fail from a kernel-level fault; degrade to a faster blink
        // rather than halting the whole system, since the LED itself is the liveness signal
        let _ = system::delay_ms(HALF_PERIOD_MS);
        ld2.set(GpioLevel::Low);
        let _ = system::delay_ms(HALF_PERIOD_MS);
    }
}
