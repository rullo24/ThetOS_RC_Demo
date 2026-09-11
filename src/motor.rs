// Motor/drivetrain profiles: what average voltage this specific motor + gearbox +
// chassis needs to move, independent of which battery supplies it. Split per wheel
// because two "matched" motors rarely draw perfectly evenly in practice.
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
        match wheel {
            Wheel::Left => 5_500,
            Wheel::Right => 5_500,
        }
    }
}

// future drivetrains (different motors/gearing/wheels) are another zero-sized
// type + its own targets, same pattern as ActiveBattery.
pub enum ActiveMotor {
    RcCarDrivetrain(RcCarDrivetrain),
}

impl MotorProfile for ActiveMotor {
    fn target_cruise_mv(&self, wheel: Wheel) -> u32 {
        match self {
            ActiveMotor::RcCarDrivetrain(m) => m.target_cruise_mv(wheel),
        }
    }

    fn target_turn_mv(&self, wheel: Wheel) -> u32 {
        match self {
            ActiveMotor::RcCarDrivetrain(m) => m.target_turn_mv(wheel),
        }
    }
}

/// the runstate: which drivetrain is currently in the car
pub const CURRENT_MOTOR: ActiveMotor = ActiveMotor::RcCarDrivetrain(RcCarDrivetrain);
