// Combines the active battery and motor runstates into the runtime duty targets.
// Neither battery.rs nor motor.rs knows the other exists -> this is the one place
// that wires "what voltage do we want" (motor, per wheel) to "what duty gets that
// voltage" (battery).
use core::sync::atomic::Ordering;

use crate::battery::{BatteryProfile, CURRENT_BATTERY};
use crate::motor::{MotorProfile, Wheel, CURRENT_MOTOR};
use crate::shared::{LEFT_CRUISE_DUTY, LEFT_TURN_DUTY, RIGHT_CRUISE_DUTY, RIGHT_TURN_DUTY};

/// DESCRIPTION
/// seed the runtime-tunable per-wheel duty targets from the active battery + motor
/// runstates; call once at boot, before any task reads the *_DUTY atomics
pub fn apply_startup_config() {
    LEFT_CRUISE_DUTY.store(
        CURRENT_BATTERY.duty_for_target_mv(CURRENT_MOTOR.target_cruise_mv(Wheel::Left)),
        Ordering::Relaxed,
    );
    RIGHT_CRUISE_DUTY.store(
        CURRENT_BATTERY.duty_for_target_mv(CURRENT_MOTOR.target_cruise_mv(Wheel::Right)),
        Ordering::Relaxed,
    );
    LEFT_TURN_DUTY.store(
        CURRENT_BATTERY.duty_for_target_mv(CURRENT_MOTOR.target_turn_mv(Wheel::Left)),
        Ordering::Relaxed,
    );
    RIGHT_TURN_DUTY.store(
        CURRENT_BATTERY.duty_for_target_mv(CURRENT_MOTOR.target_turn_mv(Wheel::Right)),
        Ordering::Relaxed,
    );
}
