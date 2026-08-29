// LIBRARY INCLUDES
use nucleo_l152re::{system, GpioLevel, OutputPin, OutputStyle, UninitPin, PA5};

pub extern "C" fn blink_task(_arg: *mut ()) -> ! {
    let mut ld2 = PA5.into_output(OutputStyle::PushPull);
    loop {
        ld2.set(GpioLevel::High);
        system::delay_ms(500).unwrap();
        ld2.set(GpioLevel::Low);
        system::delay_ms(500).unwrap();
    }
}
