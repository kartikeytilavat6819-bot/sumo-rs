//! Port of the `VEHPHEMLightJSON` model declared in foreign/PHEMlight/V5/cpp/CEPHandler.h.
//! The C++ getter/setter pairs are plain public fields here. Value-initialised C++ members are zero.

use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct VehicleData {
    pub mass_type: String,
    pub fuel_type: String,
    pub calc_type: String,
    pub mass: f64,
    pub loading: f64,
    pub red_mass_wheel: f64,
    pub wheel_diameter: f64,
    pub cw: f64,
    pub a: f64,
    pub mileage: f64,
}

#[derive(Debug, Default, Clone)]
pub struct RollresData {
    pub fr0: f64,
    pub fr1: f64,
    pub fr2: f64,
    pub fr3: f64,
    pub fr4: f64,
}

#[derive(Debug, Default, Clone)]
pub struct IceData {
    pub prated: f64,
    pub nrated: f64,
    pub idling: f64,
}

#[derive(Debug, Default, Clone)]
pub struct EmData {
    pub prated: f64,
    pub nrated: f64,
}

#[derive(Debug, Default, Clone)]
pub struct EngineData {
    pub ice: IceData,
    pub em: EmData,
}

#[derive(Debug, Default, Clone)]
pub struct AuxData {
    pub pauxnorm: f64,
}

#[derive(Debug, Default, Clone)]
pub struct FullLoadDragData {
    pub p_n_max_v0: f64,
    pub p_n_max_p0: f64,
    pub p_n_max_v1: f64,
    pub p_n_max_p1: f64,
    pub drag_curve: BTreeMap<String, Vec<f64>>,
}

#[derive(Debug, Default, Clone)]
pub struct TransmissionData {
    pub axel_ratio: f64,
    pub transm: BTreeMap<String, Vec<f64>>,
}

/// Root object (`VEHPHEMLightJSON::VEH`).
#[derive(Debug, Default, Clone)]
pub struct Veh {
    pub vtype: String,
    pub version: String,
    pub vehicle_data: VehicleData,
    pub auxiliaries_data: AuxData,
    pub engine_data: EngineData,
    pub rolling_res_data: RollresData,
    pub fld_data: FullLoadDragData,
    pub transmission_data: TransmissionData,
}
