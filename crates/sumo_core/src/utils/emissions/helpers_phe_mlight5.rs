//! Port of utils/emissions/HelpersPHEMlight5.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Helper methods for PHEMlight-based emission computation
/// C++ class `HelpersPHEMlight5` (utils/emissions/HelpersPHEMlight5.h)
pub struct HelpersPHEMlight5 {
    /// C++ base class `HelpersPHEMlight`
    pub base: HelpersPHEMlight,
    /// the index of the next class
    pub myIndex: i32,
    pub myCEPHandler: CEPHandler,
    pub myCorrection: Ptr<Correction>,
    pub myHelper: Helpers,
    pub myCEPs: BTreeMap<SUMOEmissionClass, Ptr<CEP>>,
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
        todo!("port HelpersPHEMlight5::HelpersPHEMlight5")
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
        todo!("port HelpersPHEMlight5::~HelpersPHEMlight5")
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
        todo!("port HelpersPHEMlight5::getClassByName")
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
        todo!("port HelpersPHEMlight5::getFuel")
    }

    /// C++: `double HelpersPHEMlight5::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class This implementation returns the value from the corresponding veh description file.
    // double
    // HelpersPHEMlight5::getWeight(const SUMOEmissionClass c) const {
    //     return myCEPs.find(c)->second->getVehicleMass();
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, c: SUMOEmissionClass) -> f64 {
        todo!("port HelpersPHEMlight5::getWeight")
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
        todo!("port HelpersPHEMlight5::compute")
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
        todo!("port HelpersPHEMlight5::getModifiedAccel")
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
        todo!("port HelpersPHEMlight5::getCoastingDecel")
    }

    /// C++: `double HelpersPHEMlight5::getEmission(PHEMlightdllV5::CEP *, const std::string &, const double, const double, const double, const double) const` [const, private]
    /// Returns the amount of emitted pollutant given the vehicle type and state (in mg/s or in ml/s for fuel)
    // double
    // HelpersPHEMlight5::getEmission(PHEMlightdllV5::CEP* currCep, const std::string& e, const double p, const double v, const double drivingPower, const double ratedPower) const {
    //     return currCep->GetEmission(e, p, v, &myHelper, drivingPower, ratedPower);
    // }
    #[allow(unused_variables)]
    pub fn get_emission(&self, currCep: Ptr<CEP>, e: &str, p: f64, v: f64, drivingPower: f64, ratedPower: f64) -> f64 {
        todo!("port HelpersPHEMlight5::getEmission")
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
    pub fn calc_power(&self, currCep: Ptr<CEP>, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port HelpersPHEMlight5::calcPower")
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
    pub fn calc_wheel_power(&self, currCep: Ptr<CEP>, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port HelpersPHEMlight5::calcWheelPower")
    }

}

