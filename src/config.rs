// Combines a battery and a motor into the runtime duty targets. Neither battery.rs
// nor motor.rs knows the other exists -> this is the one place that wires "what
// voltage do we want" (motor, per wheel) to "what duty gets that voltage" (battery).
// Takes both as arguments rather than reaching for a fixed global -> main.rs is the
// only place that decides which battery/motor this build is for.
use core::sync::atomic::Ordering;

use crate::battery::BatteryProfile;
use crate::motor::{MotorProfile, Wheel};
use crate::shared::{
    LEFT_CRUISE_DUTY, LEFT_TURN_DUTY, LEFT_TURN_OUTSIDE_DUTY, RIGHT_CRUISE_DUTY, RIGHT_TURN_DUTY,
    RIGHT_TURN_OUTSIDE_DUTY,
};

/// DESCRIPTION
/// seed the runtime-tunable per-wheel duty targets from the given battery + motor;
/// call once at boot, before any task reads the *_DUTY atomics
pub fn apply_startup_config(battery: &impl BatteryProfile, motor: &impl MotorProfile) {
    LEFT_CRUISE_DUTY.store(
        battery.duty_for_target_mv(motor.target_cruise_mv(Wheel::Left)),
        Ordering::Relaxed,
    );
    RIGHT_CRUISE_DUTY.store(
        battery.duty_for_target_mv(motor.target_cruise_mv(Wheel::Right)),
        Ordering::Relaxed,
    );
    LEFT_TURN_DUTY.store(
        battery.duty_for_target_mv(motor.target_turn_mv(Wheel::Left)),
        Ordering::Relaxed,
    );
    RIGHT_TURN_DUTY.store(
        battery.duty_for_target_mv(motor.target_turn_mv(Wheel::Right)),
        Ordering::Relaxed,
    );
    LEFT_TURN_OUTSIDE_DUTY.store(
        battery.duty_for_target_mv(motor.target_turn_outside_mv(Wheel::Left)),
        Ordering::Relaxed,
    );
    RIGHT_TURN_OUTSIDE_DUTY.store(
        battery.duty_for_target_mv(motor.target_turn_outside_mv(Wheel::Right)),
        Ordering::Relaxed,
    );
}
