//! Port of utils/emissions/HelpersPHEMlight5.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use foreign::phemlight::v5::cep::CEP as V5CEP;
use foreign::phemlight::v5::cep_handler::CEPHandler as V5CEPHandler;
use foreign::phemlight::v5::correction::Correction as V5Correction;
use foreign::phemlight::v5::helpers::Helpers as V5Helpers;
use foreign::phemlight::v5::constants as pc5;
use foreign::phemlight::v5::constants::{STR_BEV, STR_DIESEL, STR_GASOLINE, STR_HYBRID};

/// C++ `HelpersPHEMlight5::PHEMLIGHT5_BASE` (6 << 16)
pub const PHEMLIGHT5_BASE: i32 = 6 << 16;
/// C++ `SECONDS_PER_HOUR` (utils/emissions/PHEMConstants.h)
pub const SECONDS_PER_HOUR: f64 = 3600.;

/// Helper methods for PHEMlight-based emission computation
/// C++ class `HelpersPHEMlight5` (utils/emissions/HelpersPHEMlight5.h)
#[repr(C)]
pub struct HelpersPHEMlight5 {
    /// C++ base class `HelpersPHEMlight`
    pub base: HelpersPHEMlight,
    /// the index of the next class
    pub myIndex: i32,
    pub myCEPHandler: V5CEPHandler,
    pub myCorrection: Ptr<V5Correction>,
    pub myHelper: V5Helpers,
    pub myCEPs: BTreeMap<SUMOEmissionClass, Ptr<V5CEP>>,
}

impl std::ops::Deref for HelpersPHEMlight5 {
    type Target = HelpersPHEMlight;
    fn deref(&self) -> &HelpersPHEMlight {
        &self.base
    }
}
impl std::ops::DerefMut for HelpersPHEMlight5 {
    fn deref_mut(&mut self) -> &mut HelpersPHEMlight {
        &mut self.base
    }
}

// C++ static members of HelpersPHEMlight5 (port as module-level statics, see AGENTS.md):
//   static const int HelpersPHEMlight5::PHEMLIGHT5_BASE;

impl HelpersPHEMlight5 {
    /// C++: `void HelpersPHEMlight5::HelpersPHEMlight5()`
    /// Constructor
    // HelpersPHEMlight5::HelpersPHEMlight5() :
    //     HelpersPHEMlight("PHEMlight5", PHEMLIGHT5_BASE, -1),
    //     myIndex(PHEMLIGHT5_BASE) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        Self {
            base: HelpersPHEMlight::new_2("PHEMlight5".to_string(), PHEMLIGHT5_BASE, -1),
            myIndex: PHEMLIGHT5_BASE,
            myCEPHandler: V5CEPHandler::new(),
            myCorrection: Ptr::null(),
            myHelper: V5Helpers::default(),
            myCEPs: BTreeMap::new(),
        }
    }

    /// C++: `void HelpersPHEMlight5::~HelpersPHEMlight5() noexcept` [virtual]
    /// Destructor
    // HelpersPHEMlight5::~HelpersPHEMlight5() {
    //     for (const auto& cep : myCEPs) {
    //         delete cep.second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: the CEP objects are owned by myCEPHandler in this port, so the C++ delete loop only clears the index map, upgrade if CEPs move out of the handler
        self.myCEPs.clear();
    }

    /// C++: `SUMOEmissionClass HelpersPHEMlight5::getClassByName(const std::string &, const SUMOVehicleClass)` [virtual]
    /// Checks whether the string describes a known vehicle class
    // SUMOEmissionClass
    // HelpersPHEMlight5::getClassByName(const std::string& eClass, const SUMOVehicleClass vc) {
    //     if (eClass == "unknown" && !myEmissionClassStrings.hasString("unknown")) {
    //         myEmissionClassStrings.addAlias("unknown", getClassByName("PC_EU4_G", vc));
    //     }
    //     if (eClass == "default" && !myEmissionClassStrings.hasString("default")) {
    //         myEmissionClassStrings.addAlias("default", getClassByName("PC_EU4_G", vc));
    //     }
    //     if (myEmissionClassStrings.hasString(eClass)) {
    //         return myEmissionClassStrings.get(eClass);
    //     }
    //     if (eClass.size() < 6) {
    //         throw InvalidArgument("Unknown emission class '" + eClass + "'.");
    //     }
    //     const OptionsCont& oc = OptionsCont::getOptions();
    //     myVolumetricFuel = oc.getBool("emissions.volumetric-fuel");
    //     std::vector<std::string> phemPath;
    //     phemPath.push_back(oc.getString("phemlight-path") + "/");
    //     if (getenv("PHEMLIGHT_PATH") != nullptr) {
    //         phemPath.push_back(std::string(getenv("PHEMLIGHT_PATH")) + "/");
    //     }
    //     if (getenv("SUMO_HOME") != nullptr) {
    //         phemPath.push_back(std::string(getenv("SUMO_HOME")) + "/data/emissions/PHEMlight5/");
    //     }
    //     if (myCorrection == nullptr && (!oc.isDefault("phemlight-year") || !oc.isDefault("phemlight-temperature"))) {
    //         myCorrection = new PHEMlightdllV5::Correction(phemPath);
    //         if (!oc.isDefault("phemlight-year")) {
    //             myCorrection->setYear(oc.getInt("phemlight-year"));
    //             std::string err;
    //             if (!myCorrection->ReadDet(err)) {
    //                 throw InvalidArgument("Error reading PHEMlight5 deterioration data.\n" + err);
    //             }
    //             myCorrection->setUseDet(true);
    //         }
    //         if (!oc.isDefault("phemlight-temperature")) {
    //             myCorrection->setAmbTemp(oc.getFloat("phemlight-temperature"));
    //             std::string err;
    //             if (!myCorrection->ReadTNOx(err)) {
    //                 throw InvalidArgument("Error reading PHEMlight5 deterioration data.\n" + err);
    //             }
    //             myCorrection->setUseTNOx(true);
    //         }
    //     }
    //     myHelper.setCommentPrefix("c");
    //     myHelper.setPHEMDataV("V5");
    //     myHelper.setclass(eClass);
    //     if (!myCEPHandler.GetCEP(phemPath, &myHelper, myCorrection)) {
    //         throw InvalidArgument("File for PHEMlight5 emission class " + eClass + " not found.\n" + myHelper.getErrMsg());
    //     }
    //     PHEMlightdllV5::CEP* const currCep = myCEPHandler.getCEPS().find(myHelper.getgClass())->second;
    //     int index = myIndex++;
    //     if (currCep->getHeavyVehicle()) {
    //         index |= PollutantsInterface::HEAVY_BIT;
    //     }
    //     myEmissionClassStrings.insert(eClass, index);
    //     myCEPs[index] = currCep;
    //     myEmissionClassStrings.addAlias(StringUtils::to_lower_case(eClass), index);
    //     return index;
    // }
    #[allow(unused_variables)]
    pub fn get_class_by_name(&mut self, eClass: &str, vc: SUMOVehicleClass) -> SUMOEmissionClass {
        if eClass == "unknown" && !self.myEmissionClassStrings.has_string("unknown") {
            let alias = self.get_class_by_name("PC_EU4_G", vc);
            self.myEmissionClassStrings.add_alias("unknown", alias);
        }
        if eClass == "default" && !self.myEmissionClassStrings.has_string("default") {
            let alias = self.get_class_by_name("PC_EU4_G", vc);
            self.myEmissionClassStrings.add_alias("default", alias);
        }
        if self.myEmissionClassStrings.has_string(eClass) {
            return self.myEmissionClassStrings.get(eClass).unwrap();
        }
        if eClass.len() < 6 {
            panic!("Unknown emission class '{}'.", eClass);
        }
        self.myVolumetricFuel = utils::options::options_cont::get_bool("emissions.volumetric-fuel");
        let mut phemPath: Vec<String> = Vec::new();
        phemPath.push(format!("{}/", utils::options::options_cont::get_string("phemlight-path")));
        if let Ok(p) = std::env::var("PHEMLIGHT_PATH") {
            phemPath.push(format!("{}/", p));
        }
        if let Ok(s) = std::env::var("SUMO_HOME") {
            phemPath.push(format!("{}/data/emissions/PHEMlight5/", s));
        }
        if self.myCorrection.is_null() && (!utils::options::options_cont::is_default("phemlight-year") || !utils::options::options_cont::is_default("phemlight-temperature")) {
            self.myCorrection = Ptr::from_mut(Box::leak(Box::new(V5Correction::new(&phemPath))));
            if !utils::options::options_cont::is_default("phemlight-year") {
                self.myCorrection.m().set_year(utils::options::options_cont::get_int("phemlight-year"));
                let mut err = String::new();
                if !self.myCorrection.m().read_det(&mut err) {
                    panic!("Error reading PHEMlight5 deterioration data.\n{}", err);
                }
                self.myCorrection.m().set_use_det(true);
            }
            if !utils::options::options_cont::is_default("phemlight-temperature") {
                self.myCorrection.m().set_amb_temp(utils::options::options_cont::get_float("phemlight-temperature"));
                let mut err = String::new();
                if !self.myCorrection.m().read_tnox(&mut err) {
                    panic!("Error reading PHEMlight5 deterioration data.\n{}", err);
                }
                self.myCorrection.m().set_use_tnox(true);
            }
        }
        self.myHelper.set_comment_prefix("c");
        self.myHelper.set_phem_data_v("V5");
        self.myHelper.setclass(eClass);
        let correction = if self.myCorrection.is_null() { None } else { Some(self.myCorrection.m()) };
        let found = self.myCEPHandler.get_cep(&phemPath, &mut self.myHelper, correction);
        if !matches!(found, Ok(true)) {
            panic!("File for PHEMlight5 emission class {} not found.\n{}", eClass, self.myHelper.get_err_msg());
        }
        let g_class = self.myHelper.get_g_class().to_string();
        let curr_cep = Ptr::new(self.myCEPHandler.get_ceps().get(&g_class).unwrap());
        let mut index = self.myIndex;
        self.myIndex += 1;
        if curr_cep.m().get_heavy_vehicle() {
            index |= HEAVY_BIT;
        }
        self.myEmissionClassStrings.insert(eClass, index, true).unwrap();
        self.myCEPs.insert(index, curr_cep);
        self.myEmissionClassStrings.add_alias(&utils::common::string_utils::to_lower_case(eClass), index);
        index
    }

    /// C++: `std::string HelpersPHEMlight5::getFuel(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the fuel type described by this emission class as described in the Amitran interface (Gasoline, Diesel, ...)
    // std::string
    // HelpersPHEMlight5::getFuel(const SUMOEmissionClass c) const {
    //     const std::string name = myEmissionClassStrings.getString(c);
    //     std::string fuel = "Gasoline";
    //     if (name.find("_D_") != std::string::npos) {
    //         fuel = "Diesel";
    //     }
    //     if (name.find("_BEV_") != std::string::npos) {
    //         fuel = "Electricity";
    //     }
    //     if (name.find("_HEV") != std::string::npos) {
    //         fuel = "Hybrid" + fuel;
    //     }
    //     return fuel;
    // }
    #[allow(unused_variables)]
    pub fn get_fuel(&self, c: SUMOEmissionClass) -> String {
        let name = self.myEmissionClassStrings.get_string(c).unwrap();
        let mut fuel = "Gasoline".to_string();
        if name.contains("_D_") {
            fuel = "Diesel".to_string();
        }
        if name.contains("_BEV_") {
            fuel = "Electricity".to_string();
        }
        if name.contains("_HEV") {
            fuel = format!("Hybrid{}", fuel);
        }
        fuel
    }

    /// C++: `double HelpersPHEMlight5::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class This implementation returns the value from the corresponding veh description file.
    // double
    // HelpersPHEMlight5::getWeight(const SUMOEmissionClass c) const {
    //     return myCEPs.find(c)->second->getVehicleMass();
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, c: SUMOEmissionClass) -> f64 {
        self.myCEPs.get(&c).unwrap().m().get_vehicle_mass()
    }

    /// C++: `double HelpersPHEMlight5::compute(const SUMOEmissionClass, const PollutantsInterface::EmissionType, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the amount of emitted pollutant given the vehicle type and state (in mg/s or in ml/s for fuel)
    // double
    // HelpersPHEMlight5::compute(const SUMOEmissionClass c, const PollutantsInterface::EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     if (param != nullptr && param->isEngineOff()) {
    //         return 0.;
    //     }
    //     const double corrSpeed = MAX2(0.0, v);
    //     assert(myCEPs.count(c) == 1);
    //     PHEMlightdllV5::CEP* const currCep = myCEPs.find(c)->second;
    //     const double corrAcc = getModifiedAccel(c, corrSpeed, a, slope, param);
    //     const bool isBEV = currCep->getFuelType() == PHEMlightdllV5::Constants::strBEV;
    //     const bool isHybrid = currCep->getCalcType() == PHEMlightdllV5::Constants::strHybrid;
    //     const double power_raw = calcPower(currCep, corrSpeed, corrAcc, slope, param);
    //     const double ratedPower = param->getDoubleOptional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep->getRatedPower()) / 1000.;
    //     const double power = isHybrid ? calcWheelPower(currCep, corrSpeed, corrAcc, slope, param) : currCep->CalcEngPower(power_raw, ratedPower);
    //
    //     if (!isBEV && corrAcc < getCoastingDecel(c, corrSpeed, corrAcc, slope, param) &&
    //             corrSpeed > PHEMlightdllV5::Constants::ZERO_SPEED_ACCURACY) {
    //         return 0.;
    //     }
    //     // TODO: this is probably only needed for non-heavy vehicles, so if execution speed becomes an issue this could be optimized out
    //     const double drivingPower = calcPower(currCep, PHEMlightdllV5::Constants::NORMALIZING_SPEED, PHEMlightdllV5::Constants::NORMALIZING_ACCELARATION, 0, param);
    //     switch (e) {
    //         case PollutantsInterface::CO:
    //             return getEmission(currCep, "CO", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::CO2:
    //             return currCep->GetCO2Emission(getEmission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower),
    //                                            getEmission(currCep, "CO", power, corrSpeed, drivingPower, ratedPower),
    //                                            getEmission(currCep, "HC", power, corrSpeed, drivingPower, ratedPower), &myHelper) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::HC:
    //             return getEmission(currCep, "HC", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::NO_X:
    //             return getEmission(currCep, "NOx", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::PM_X:
    //             return getEmission(currCep, "PM", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::FUEL: {
    //             if (myVolumetricFuel && currCep->getFuelType() == PHEMlightdllV5::Constants::strDiesel) { // divide by average diesel density of 836 g/l
    //                 return getEmission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / 836. / SECONDS_PER_HOUR * 1000.;
    //             }
    //             if (myVolumetricFuel && currCep->getFuelType() == PHEMlightdllV5::Constants::strGasoline) { // divide by average gasoline density of 742 g/l
    //                 return getEmission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / 742. / SECONDS_PER_HOUR * 1000.;
    //             }
    //             if (isBEV) {
    //                 return 0.;
    //             }
    //             return getEmission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.; // still in mg even if myVolumetricFuel is set!
    //         }
    //         case PollutantsInterface::ELEC:
    //             if (isBEV) {
    //                 const double auxPower = param->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE, currCep->getAuxPower() * 1000.) / 1000.;
    //                 return (getEmission(currCep, "FC_el", power, corrSpeed, drivingPower, ratedPower) + auxPower) / SECONDS_PER_HOUR * 1000.;
    //             }
    //             return 0;
    //     }
    //     // should never get here
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn compute(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        if !param.is_null() && param.m().is_engine_off() {
            return 0.;
        }
        let corrSpeed = if 0.0 > v { 0.0 } else { v }; // MAX2(0.0, v)
        debug_assert!(self.myCEPs.contains_key(&c));
        let currCep = *self.myCEPs.get(&c).unwrap();
        let corrAcc = self.get_modified_accel(c, corrSpeed, a, slope, param);
        let isBEV = currCep.m().get_fuel_type() == STR_BEV;
        let isHybrid = currCep.m().get_calc_type() == STR_HYBRID;
        let power_raw = self.calc_power(currCep, corrSpeed, corrAcc, slope, param);
        let ratedPower = param.m().get_double_optional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep.m().get_rated_power()) / 1000.;
        let power = if isHybrid {
            self.calc_wheel_power(currCep, corrSpeed, corrAcc, slope, param)
        } else {
            currCep.m().calc_eng_power(power_raw, ratedPower)
        };
        if !isBEV && corrAcc < self.get_coasting_decel(c, corrSpeed, corrAcc, slope, param) && corrSpeed > pc5::ZERO_SPEED_ACCURACY {
            return 0.;
        }
        // TODO: this is probably only needed for non-heavy vehicles, so if execution speed becomes an issue this could be optimized out
        let drivingPower = self.calc_power(currCep, pc5::NORMALIZING_SPEED, pc5::NORMALIZING_ACCELARATION, 0., param);
        match e {
            PollutantsInterface_EmissionType::CO => {
                self.get_emission(currCep, "CO", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::CO2 => {
                let fc = self.get_emission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower);
                let co = self.get_emission(currCep, "CO", power, corrSpeed, drivingPower, ratedPower);
                let hc = self.get_emission(currCep, "HC", power, corrSpeed, drivingPower, ratedPower);
                currCep.m().get_co2_emission(fc, co, hc, Ptr::new(&self.myHelper).m()) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::HC => {
                self.get_emission(currCep, "HC", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::NO_X => {
                self.get_emission(currCep, "NOx", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::PM_X => {
                self.get_emission(currCep, "PM", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::FUEL => {
                if self.myVolumetricFuel && currCep.m().get_fuel_type() == STR_DIESEL {
                    // divide by average diesel density of 836 g/l
                    return self.get_emission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / 836. / SECONDS_PER_HOUR * 1000.;
                }
                if self.myVolumetricFuel && currCep.m().get_fuel_type() == STR_GASOLINE {
                    // divide by average gasoline density of 742 g/l
                    return self.get_emission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / 742. / SECONDS_PER_HOUR * 1000.;
                }
                if isBEV {
                    return 0.;
                }
                // still in mg even if myVolumetricFuel is set!
                self.get_emission(currCep, "FC", power, corrSpeed, drivingPower, ratedPower) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::ELEC => {
                if isBEV {
                    let auxPower = param.m().get_double_optional(SUMO_ATTR_CONSTANTPOWERINTAKE, currCep.m().get_aux_power() * 1000.) / 1000.;
                    return (self.get_emission(currCep, "FC_el", power, corrSpeed, drivingPower, ratedPower) + auxPower) / SECONDS_PER_HOUR * 1000.;
                }
                0.
            }
        }
    }

    /// C++: `double HelpersPHEMlight5::getModifiedAccel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the adapted acceleration value, useful for comparing with external PHEMlight references.
    // double
    // HelpersPHEMlight5::getModifiedAccel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     PHEMlightdllV5::CEP* currCep = myCEPs.count(c) == 0 ? nullptr : myCEPs.find(c)->second;
    //     if (currCep != nullptr) {
    //         if (v == 0.) {
    //             return 0.;
    //         }
    //         // this is a copy of CEP::GetMaxAccel
    //         const double rotFactor = currCep->GetRotationalCoeffecient(v);
    //         const double mass = param->getDoubleOptional(SUMO_ATTR_MASS, currCep->getVehicleMass());
    //         const double massRot = param->getDoubleOptional(SUMO_ATTR_ROTATINGMASS, currCep->getVehicleMassRot());
    //         const double load = param->getDoubleOptional(SUMO_ATTR_LOADING, currCep->getVehicleLoading()) + param->getTransportableMass();
    //         const double ratedPower = param->getDoubleOptional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep->getRatedPower()) / 1000.;
    //         const double pMaxForAcc = currCep->GetPMaxNorm(v) * ratedPower - calcPower(currCep, v, 0, slope, param);
    //         const double maxAcc = (pMaxForAcc * 1000) / ((mass * rotFactor + massRot + load) * v);
    //         return MIN2(a, maxAcc);
    //     }
    //     return a;
    // }
    #[allow(unused_variables)]
    pub fn get_modified_accel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        if let Some(currCep) = self.myCEPs.get(&c).copied() {
            if v == 0. {
                return 0.;
            }
            // this is a copy of CEP::GetMaxAccel
            let rotFactor = currCep.m().get_rotational_coeffecient(v);
            let mass = param.m().get_double_optional(SUMO_ATTR_MASS, currCep.m().get_vehicle_mass());
            let massRot = param.m().get_double_optional(SUMO_ATTR_ROTATINGMASS, currCep.m().get_vehicle_mass_rot());
            let load = param.m().get_double_optional(SUMO_ATTR_LOADING, currCep.m().get_vehicle_loading()) + param.m().get_transportable_mass();
            let ratedPower = param.m().get_double_optional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep.m().get_rated_power()) / 1000.;
            let pMaxForAcc = currCep.m().get_p_max_norm(v) * ratedPower - self.calc_power(currCep, v, 0., slope, param);
            let maxAcc = (pMaxForAcc * 1000.) / ((mass * rotFactor + massRot + load) * v);
            return if a < maxAcc { a } else { maxAcc }; // MIN2(a, maxAcc)
        }
        a
    }

    /// C++: `double HelpersPHEMlight5::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the maximum deceleration value (as a negative number), which can still be considered as non-braking.
    // double
    // HelpersPHEMlight5::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     PHEMlightdllV5::CEP* const currCep = myCEPs.find(c)->second;
    //     // this is a copy of CEP::GetDecelCoast with an adapted slope force calculation
    //     if (v < PHEMlightdllV5::Constants::SPEED_DCEL_MIN) {
    //         return v / PHEMlightdllV5::Constants::SPEED_DCEL_MIN * getCoastingDecel(c, PHEMlightdllV5::Constants::SPEED_DCEL_MIN, a, slope, param);
    //     }
    //     const double rotFactor = currCep->GetRotationalCoeffecient(v);
    //     const double mass = param->getDoubleOptional(SUMO_ATTR_MASS, currCep->getVehicleMass());
    //     const double load = param->getDoubleOptional(SUMO_ATTR_LOADING, currCep->getVehicleLoading()) + param->getTransportableMass();
    //     const double cw = param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA, currCep->getCrossSectionalArea()) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT, currCep->getCWValue());
    //     const double ratedPower = param->getDoubleOptional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep->getRatedPower()) / 1000.;
    //     const double wheelRadius = param->getDoubleOptional(SUMO_ATTR_WHEELRADIUS, currCep->getWheelRadius());
    //     const double rf0 = param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, currCep->getResistanceF0());
    //
    //     const double fRoll = currCep->getResistance(v, rf0) * (mass + load) * PHEMlightdllV5::Constants::GRAVITY_CONST;
    //     const double fAir = cw * PHEMlightdllV5::Constants::AIR_DENSITY_CONST * 0.5 * std::pow(v, 2);
    //     const double fGrad = (mass + load) * PHEMlightdllV5::Constants::GRAVITY_CONST * sin(DEG2RAD(slope));
    //
    //     return -(currCep->getFMot(v, ratedPower, wheelRadius) + fRoll + fAir + fGrad) / ((mass + load) * rotFactor);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        let currCep = *self.myCEPs.get(&c).unwrap();
        // this is a copy of CEP::GetDecelCoast with an adapted slope force calculation
        if v < pc5::SPEED_DCEL_MIN {
            return v / pc5::SPEED_DCEL_MIN * self.get_coasting_decel(c, pc5::SPEED_DCEL_MIN, a, slope, param);
        }
        let rotFactor = currCep.m().get_rotational_coeffecient(v);
        let mass = param.m().get_double_optional(SUMO_ATTR_MASS, currCep.m().get_vehicle_mass());
        let load = param.m().get_double_optional(SUMO_ATTR_LOADING, currCep.m().get_vehicle_loading()) + param.m().get_transportable_mass();
        let cw = param.m().get_double_optional(SUMO_ATTR_FRONTSURFACEAREA, currCep.m().get_cross_sectional_area()) * param.m().get_double_optional(SUMO_ATTR_AIRDRAGCOEFFICIENT, currCep.m().get_cw_value());
        let ratedPower = param.m().get_double_optional(SUMO_ATTR_MAXIMUMPOWER, 1000. * currCep.m().get_rated_power()) / 1000.;
        let wheelRadius = param.m().get_double_optional(SUMO_ATTR_WHEELRADIUS, currCep.m().get_wheel_radius());
        let rf0 = param.m().get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, currCep.m().get_resistance_f0());
        let fRoll = currCep.m().get_resistance(v, rf0) * (mass + load) * pc5::GRAVITY_CONST;
        let fAir = cw * pc5::AIR_DENSITY_CONST * 0.5 * v.powf(2.0);
        let fGrad = (mass + load) * pc5::GRAVITY_CONST * utils::geom::geom_helper::deg2rad(slope).sin();
        -(currCep.m().get_f_mot(v, ratedPower, wheelRadius) + fRoll + fAir + fGrad) / ((mass + load) * rotFactor)
    }

    /// C++: `double HelpersPHEMlight5::getEmission(PHEMlightdllV5::CEP *, const std::string &, const double, const double, const double, const double) const` [const, private]
    /// Returns the amount of emitted pollutant given the vehicle type and state (in mg/s or in ml/s for fuel)
    // double
    // HelpersPHEMlight5::getEmission(PHEMlightdllV5::CEP* currCep, const std::string& e, const double p, const double v, const double drivingPower, const double ratedPower) const {
    //     return currCep->GetEmission(e, p, v, &myHelper, drivingPower, ratedPower);
    // }
    #[allow(unused_variables)]
    pub fn get_emission(&self, currCep: Ptr<V5CEP>, e: &str, p: f64, v: f64, drivingPower: f64, ratedPower: f64) -> f64 {
        currCep.m().get_emission(e, p, v, Ptr::new(&self.myHelper).m(), drivingPower, ratedPower)
    }

    /// C++: `double HelpersPHEMlight5::calcPower(PHEMlightdllV5::CEP *, const double, const double, const double, const EnergyParams *) const` [const, private]
    /// Returns the total power needed.
    // double
    // HelpersPHEMlight5::calcPower(PHEMlightdllV5::CEP* currCep, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     // copy of CEP::CalcPower
    //     const double power = calcWheelPower(currCep, v, a, slope, param) / PHEMlightdllV5::Constants::_DRIVE_TRAIN_EFFICIENCY;
    //     if (!(currCep->getCalcType() == "HEV" || currCep->getCalcType() == "BEV")) {
    //         return power + param->getDoubleOptional(SUMO_ATTR_CONSTANTPOWERINTAKE, currCep->getAuxPower() * 1000.) / 1000.;
    //     }
    //     return power;
    // }
    #[allow(unused_variables)]
    pub fn calc_power(&self, currCep: Ptr<V5CEP>, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        // copy of CEP::CalcPower
        let power = self.calc_wheel_power(currCep, v, a, slope, param) / foreign::phemlight::v5::constants::get_drive_train_efficiency();
        let calcType = currCep.m().get_calc_type();
        if !(calcType == STR_HYBRID || calcType == STR_BEV) {
            return power + param.m().get_double_optional(SUMO_ATTR_CONSTANTPOWERINTAKE, currCep.m().get_aux_power() * 1000.) / 1000.;
        }
        power
    }

    /// C++: `double HelpersPHEMlight5::calcWheelPower(PHEMlightdllV5::CEP *, const double, const double, const double, const EnergyParams *) const` [const, private]
    /// Returns the power without auxiliaries.
    // double
    // HelpersPHEMlight5::calcWheelPower(PHEMlightdllV5::CEP* currCep, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     // copy of CEP::CalcWheelPower
    //     const double rotFactor = currCep->GetRotationalCoeffecient(v);
    //     const double mass = param->getDoubleOptional(SUMO_ATTR_MASS, currCep->getVehicleMass());
    //     const double massRot = param->getDoubleOptional(SUMO_ATTR_ROTATINGMASS, currCep->getVehicleMassRot());
    //     const double load = param->getDoubleOptional(SUMO_ATTR_LOADING, currCep->getVehicleLoading()) + param->getTransportableMass();
    //     const double cw = param->getDoubleOptional(SUMO_ATTR_FRONTSURFACEAREA, currCep->getCrossSectionalArea()) * param->getDoubleOptional(SUMO_ATTR_AIRDRAGCOEFFICIENT, currCep->getCWValue());
    //     const double rf0 = param->getDoubleOptional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, currCep->getResistanceF0());
    //
    //     double power = (mass + load) * PHEMlightdllV5::Constants::GRAVITY_CONST * currCep->getResistance(v, rf0) * v;
    //     power += (cw * PHEMlightdllV5::Constants::AIR_DENSITY_CONST / 2) * std::pow(v, 3);
    //     power += (mass * rotFactor + massRot + load) * a * v;
    //     power += (mass + load) * PHEMlightdllV5::Constants::GRAVITY_CONST * slope * 0.01 * v;
    //     return power / 1000.;
    // }
    #[allow(unused_variables)]
    pub fn calc_wheel_power(&self, currCep: Ptr<V5CEP>, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        // copy of CEP::CalcWheelPower
        let rotFactor = currCep.m().get_rotational_coeffecient(v);
        let mass = param.m().get_double_optional(SUMO_ATTR_MASS, currCep.m().get_vehicle_mass());
        let massRot = param.m().get_double_optional(SUMO_ATTR_ROTATINGMASS, currCep.m().get_vehicle_mass_rot());
        let load = param.m().get_double_optional(SUMO_ATTR_LOADING, currCep.m().get_vehicle_loading()) + param.m().get_transportable_mass();
        let cw = param.m().get_double_optional(SUMO_ATTR_FRONTSURFACEAREA, currCep.m().get_cross_sectional_area()) * param.m().get_double_optional(SUMO_ATTR_AIRDRAGCOEFFICIENT, currCep.m().get_cw_value());
        let rf0 = param.m().get_double_optional(SUMO_ATTR_ROLLDRAGCOEFFICIENT, currCep.m().get_resistance_f0());
        let mut power = (mass + load) * pc5::GRAVITY_CONST * currCep.m().get_resistance(v, rf0) * v;
        power += (cw * pc5::AIR_DENSITY_CONST / 2.) * v.powf(3.0);
        power += (mass * rotFactor + massRot + load) * a * v;
        power += (mass + load) * pc5::GRAVITY_CONST * slope * 0.01 * v;
        power / 1000.
    }

}

