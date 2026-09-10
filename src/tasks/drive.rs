// LIBRARY INCLUDES
use core::sync::atomic::Ordering;
use nucleo_l152re::{system, Pwm, PwmChannel, PwmConfig, Tim3, UninitPwm, PA6, PA7};

// USER INCLUDES
use crate::shared::{LEFT_DUTY, RIGHT_DUTY};

const DRIVE_PERIOD_MS: u32 = 20; // how often the shared targets are pushed to the timer

// push the shared duty targets onto the two motor PWM channels (TIM3: PA6 left, PA7 right)
pub extern "C" fn drive_task(_arg: *mut ()) -> ! {
    let pwm = Pwm::<Tim3, _>::new().into_active(PwmConfig::default()); // 10 kHz
    let mut left = pwm.channel1(PA6);
    let mut right = pwm.channel2(PA7);
    left.enable();
    right.enable();

    loop {
        left.set_duty(LEFT_DUTY.load(Ordering::Relaxed));
        right.set_duty(RIGHT_DUTY.load(Ordering::Relaxed));
        system::delay_ms(DRIVE_PERIOD_MS).unwrap();
    }
}
