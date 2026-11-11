//! Port of foreign/PHEMlight/cpp/Constants.{h,cpp}

use std::sync::atomic::{AtomicU64, Ordering};

// Calculation constant
pub const GRAVITY_CONST: f64 = 9.81;
pub const AIR_DENSITY_CONST: f64 = 1.182;
pub const NORMALIZING_SPEED: f64 = 19.444;
pub const NORMALIZING_ACCELARATION: f64 = 0.45;
pub const SPEED_DCEL_MIN: f64 = 10.0 / 3.6;
pub const ZERO_SPEED_ACCURACY: f64 = 0.5;
pub const DRIVE_TRAIN_EFFICIENCY_ALL: f64 = 0.9;
pub const DRIVE_TRAIN_EFFICIENCY_CB: f64 = 0.8;

// Strings
pub const HEAVY_VEHICLE: &str = "HV";

// Vehiclestrings
pub const STR_PKW: &str = "PC";
pub const STR_LNF: &str = "LCV";
pub const STR_LKW: &str = "HDV_RT";
pub const STR_LSZ: &str = "HDV_TT";
pub const STR_RB: &str = "HDV_CO";
pub const STR_LB: &str = "HDV_CB";
pub const STR_MR2: &str = "MC_2S";
pub const STR_MR4: &str = "MC_4S";
pub const STR_KKR: &str = "MOP";

// Fuelstrings
pub const STR_GASOLINE: &str = "G";
pub const STR_DIESEL: &str = "D";
pub const STR_CNG: &str = "CNG";
pub const STR_LPG: &str = "LPG";
pub const STR_HYBRID: &str = "HEV";
pub const STR_BEV: &str = "BEV";

// Euroclasses
pub const STR_EU: &str = "EU";

// Sizeclasse
pub const STR_SI: &str = "I";
pub const STR_SII: &str = "II";
pub const STR_SIII: &str = "III";

/// Global drive train efficiency (C++ static `myDRIVE_TRAIN_EFFICIENCY`), stored as f64 bits.
/// Initial value 0.0 matches the C++ static initializer.
static DRIVE_TRAIN_EFFICIENCY: AtomicU64 = AtomicU64::new(0);

/// get drive train efficiency
pub fn get_drive_train_efficiency() -> f64 {
    f64::from_bits(DRIVE_TRAIN_EFFICIENCY.load(Ordering::Relaxed))
}

/// set drive train efficiency
pub fn set_drive_train_efficiency(value: f64) {
    DRIVE_TRAIN_EFFICIENCY.store(value.to_bits(), Ordering::Relaxed);
}

/// Serialises unit tests that touch the global drive train efficiency.
#[cfg(test)]
pub(crate) static EFFICIENCY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_train_efficiency_roundtrip() {
        let _guard = EFFICIENCY_TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_drive_train_efficiency(DRIVE_TRAIN_EFFICIENCY_CB);
        assert_eq!(get_drive_train_efficiency(), 0.8);
        set_drive_train_efficiency(DRIVE_TRAIN_EFFICIENCY_ALL);
        assert_eq!(get_drive_train_efficiency(), 0.9);
    }
}
