// Motor duty targets: comms writes them, drive reads them.
// u16::MAX = full speed, 0 = stopped. Aligned 16-bit load/store is atomic on
// Cortex-M3, so one writer / one reader needs no lock.
use core::sync::atomic::AtomicU16;

pub static LEFT_DUTY: AtomicU16 = AtomicU16::new(0);
pub static RIGHT_DUTY: AtomicU16 = AtomicU16::new(0);
