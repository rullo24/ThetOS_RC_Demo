// Motor duty targets: comms writes them, drive reads them.
// u16::MAX = full speed, 0 = stopped. Aligned 16-bit load/store is atomic on
// Cortex-M3, so one writer / one reader needs no lock.
use core::sync::atomic::AtomicU16;

pub static LEFT_DUTY: AtomicU16 = AtomicU16::new(0);
pub static RIGHT_DUTY: AtomicU16 = AtomicU16::new(0);

// per-wheel cruise/turn duty, seeded from the motor+battery runstate at boot,
// live-tunable over serial so each motor can be trimmed independently
pub static LEFT_CRUISE_DUTY: AtomicU16 = AtomicU16::new(0);
pub static RIGHT_CRUISE_DUTY: AtomicU16 = AtomicU16::new(0);
pub static LEFT_TURN_DUTY: AtomicU16 = AtomicU16::new(0);
pub static RIGHT_TURN_DUTY: AtomicU16 = AtomicU16::new(0);

// outside-wheel duty while turning -> deliberately less than straight cruise
pub static LEFT_TURN_OUTSIDE_DUTY: AtomicU16 = AtomicU16::new(0);
pub static RIGHT_TURN_OUTSIDE_DUTY: AtomicU16 = AtomicU16::new(0);
