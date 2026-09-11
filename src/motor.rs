// Motor/drivetrain profiles: what average voltage this specific motor + gearbox +
// chassis needs to move, independent of which battery supplies it. Split per wheel
// because two "matched" motors rarely draw perfectly evenly in practice. Which
// drivetrain is active for a build is decided in main.rs and passed in, not fixed here.
#[derive(Clone, Copy)]
pub enum Wheel {
    Left,
    Right,
}

pub trait MotorProfile {
    /// DESCRIPTION
    /// average voltage (mV) needed to drive this wheel forward without stalling
    fn target_cruise_mv(&self, wheel: Wheel) -> u32;

    /// DESCRIPTION
    /// average voltage (mV) needed on this wheel while it's the inside of a turn
    fn target_turn_mv(&self, wheel: Wheel) -> u32;

    /// DESCRIPTION
    /// average voltage (mV) for this wheel while it's the outside (driving) side of a
    /// turn -> deliberately less than straight-line cruise, since the inside wheel is
    /// now dragging rather than rolling
    fn target_turn_outside_mv(&self, wheel: Wheel) -> u32;
}

/// the two PMOS-switched brushed DC motors + gearbox + wheels currently in the car
pub struct RcCarDrivetrain;
impl MotorProfile for RcCarDrivetrain {
    fn target_cruise_mv(&self, wheel: Wheel) -> u32 {
        match wheel {
            Wheel::Left => 7_000,
            Wheel::Right => 7_000,
        }
    }

    fn target_turn_mv(&self, wheel: Wheel) -> u32 {
        // deliberate full stop, not a near-stall guess -> this drivetrain's operating
        // range is narrow enough ("moves solidly" or "doesn't move") that a "slows but
        // keeps turning" voltage isn't reliably available; skid/pivot on one wheel instead
        match wheel {
            Wheel::Left => 0,
            Wheel::Right => 0,
        }
    }

    fn target_turn_outside_mv(&self, wheel: Wheel) -> u32 {
        // ~75% of straight cruise, untested -> bench-tune: fast enough to turn cleanly,
        // slow enough not to fight the dragging inside wheel too hard
        match wheel {
            Wheel::Left => 5_250,
            Wheel::Right => 5_250,
        }
    }
}

// future drivetrains (different motors/gearing/wheels) are another zero-sized
// type + its own targets, same pattern as this one.
