// LIBRARY INCLUDES
use nucleo_l152re::{system, GpioLevel, OutputPin, OutputStyle, UninitPin, PA5};

const HALF_PERIOD_MS: u32 = 500; // 1 Hz blink

// blink LD2 (PA5) at 1 Hz -> visible "scheduler is running" indicator, lowest priority
pub extern "C" fn heartbeat_task(_arg: *mut ()) -> ! {
    let mut ld2 = PA5.into_output(OutputStyle::PushPull);
    loop {
        ld2.set(GpioLevel::High);
        system::delay_ms(HALF_PERIOD_MS).unwrap();
        ld2.set(GpioLevel::Low);
        system::delay_ms(HALF_PERIOD_MS).unwrap();
    }
}
