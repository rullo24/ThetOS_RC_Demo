// Battery profiles: each pack knows only its own voltage; the duty needed to hit
// a target average voltage is one shared calculation, so swapping packs never
// requires re-tuning by hand -- only the new pack's nominal_mv().
pub trait BatteryProfile {
    /// DESCRIPTION
    /// nominal pack voltage in millivolts
    fn nominal_mv(&self) -> u32;

    /// DESCRIPTION
    /// PWM duty (u16::MAX = 100%) that delivers `target_mv` average voltage from this
    /// pack; saturates at 100% if the pack can't reach target_mv on its own
    fn duty_for_target_mv(&self, target_mv: u32) -> u16 {
        let duty = (target_mv as u64 * u16::MAX as u64) / self.nominal_mv() as u64;
        duty.min(u16::MAX as u64) as u16
    }
}

/// 2S LiPo, ~7.4V nominal -> the pack currently wired into the demo
pub struct TwoCellLipo;
impl BatteryProfile for TwoCellLipo {
    fn nominal_mv(&self) -> u32 {
        7_400
    }
}

// future packs are just another zero-sized type + nominal_mv, e.g.:
// pub struct ThreeCellLipo;
// impl BatteryProfile for ThreeCellLipo {
//     fn nominal_mv(&self) -> u32 { 11_100 }
// }

/// the battery wired in for this run -> add variants as more packs are bench-tuned
pub enum ActiveBattery {
    TwoCellLipo(TwoCellLipo),
    // ThreeCellLipo(ThreeCellLipo),
}

impl BatteryProfile for ActiveBattery {
    fn nominal_mv(&self) -> u32 {
        match self {
            ActiveBattery::TwoCellLipo(b) => b.nominal_mv(),
        }
    }
}

/// the runstate: which pack is currently in the car
pub const CURRENT_BATTERY: ActiveBattery = ActiveBattery::TwoCellLipo(TwoCellLipo);
