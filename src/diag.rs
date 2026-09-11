// Boot-time diagnostic: blinks LD2 in a repeating count-then-pause pattern and halts
// forever. Used only for unrecoverable startup errors, before the kernel exists, so
// it must not depend on the RTOS (no system::delay_ms, no scheduler, no interrupts).
use nucleo_l152re::{GpioLevel, OutputPin, OutputStyle, UninitPin, PA5};

const BLINK_SPIN: u32 = 400_000; // on/off duration via a busy loop -> approximate, not timed
const GAP_SPIN: u32 = 1_600_000; // pause between repeats of the blink count

fn spin(n: u32) {
    for _ in 0..n {
        core::hint::spin_loop();
    }
}

/// DESCRIPTION
/// blink `count` times, pause, repeat forever -> lets a boot failure be read off LD2
/// without a debugger attached; never returns
pub fn fail_blink(count: u32) -> ! {
    let mut led = PA5.into_output(OutputStyle::PushPull);
    loop {
        for _ in 0..count {
            led.set(GpioLevel::High);
            spin(BLINK_SPIN);
            led.set(GpioLevel::Low);
            spin(BLINK_SPIN);
        }
        spin(GAP_SPIN);
    }
}
