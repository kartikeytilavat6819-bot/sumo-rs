//! Port of utils/emissions/HelpersPHEMlight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Helper methods for PHEMlight-based emission computation
/// C++ class `HelpersPHEMlight` (utils/emissions/HelpersPHEMlight.h)
pub struct HelpersPHEMlight {
    /// C++ base class `PollutantsInterface::Helper`
    pub base: PollutantsInterface_Helper,
    /// the index of the next class
    pub myIndex: i32,
    pub myCEPHandler: CEPHandler,
    pub myHelper: Helpers,
    pub myCEPs: BTreeMap<SUMOEmissionClass, Ptr<CEP>>,
}

impl std::ops::Deref for HelpersPHEMlight {
    type Target = PollutantsInterface_Helper;
    fn deref(&self) -> &PollutantsInterface_Helper {
        &self.base
    }
}
impl std::ops::DerefMut for HelpersPHEMlight {
    fn deref_mut(&mut self) -> &mut PollutantsInterface_Helper {
        &mut self.base
    }
}

// C++ static members of HelpersPHEMlight (port as module-level statics, see AGENTS.md):
//   static const int HelpersPHEMlight::PHEMLIGHT_BASE;

impl HelpersPHEMlight {
    /// C++: `void HelpersPHEMlight::HelpersPHEMlight()`
    /// Constructor
    // HelpersPHEMlight::HelpersPHEMlight() :
    //     PollutantsInterface::Helper("PHEMlight", PHEMLIGHT_BASE, -1),
    //     myIndex(PHEMLIGHT_BASE) {
    // }
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port HelpersPHEMlight::HelpersPHEMlight")
    }

    /// C++: `void HelpersPHEMlight::~HelpersPHEMlight() noexcept` [virtual]
    /// Destructor
    // HelpersPHEMlight::~HelpersPHEMlight() {
    //     for (const auto& cep : myCEPs) {
    //         delete cep.second;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port HelpersPHEMlight::~HelpersPHEMlight")
    }

    /// C++: `void HelpersPHEMlight::HelpersPHEMlight(std::string, const int, const int)` [protected]
    /// Constructor for subclasses
    // HelpersPHEMlight(std::string name, const int baseIndex, const int defaultClass)
    //         : PollutantsInterface::Helper(name, baseIndex, defaultClass) {}
    #[allow(unused_variables)]
    pub fn new_2(name: String, baseIndex: i32, defaultClass: i32) -> Self {
        todo!("port HelpersPHEMlight::HelpersPHEMlight")
    }

    /// C++: `SUMOEmissionClass HelpersPHEMlight::getClassByName(const std::string &, const SUMOVehicleClass)` [virtual]
    /// Checks whether the string describes a known vehicle class
    // SUMOEmissionClass
    // HelpersPHEMlight::getClassByName(const std::string& eClass, const SUMOVehicleClass vc) {
    //     if (eClass == "unknown" && !myEmissionClassStrings.hasString("unknown")) {
    //         myEmissionClassStrings.addAlias("unknown", getClassByName("PC_G_EU4", vc));
    //     }
    //     if (eClass == "default" && !myEmissionClassStrings.hasString("default")) {
    //         myEmissionClassStrings.addAlias("default", getClassByName("PC_G_EU4", vc));
    //     }
    //     if (myEmissionClassStrings.hasString(eClass)) {
    //         return myEmissionClassStrings.get(eClass);
    //     }
    //     if (eClass.size() < 6) {
    //         throw InvalidArgument("Unknown emission class '" + eClass + "'.");
    //     }
    //     int index = myIndex++;
    //     const std::string type = eClass.substr(0, 3);
    //     if (type == "HDV" || type == "LB_" || type == "RB_" || type == "LSZ" || eClass.find("LKW") != std::string::npos) {
    //         index |= PollutantsInterface::HEAVY_BIT;
    //     }
    //     myEmissionClassStrings.insert(eClass, index);
    // #ifdef INTERNAL_PHEM
    //     if (type == "HDV" || type == "LCV" || type == "PC_" || !PHEMCEPHandler::getHandlerInstance().Load(index, eClass)) {
    // #endif
    //         myVolumetricFuel = OptionsCont::getOptions().getBool("emissions.volumetric-fuel");
    //         std::vector<std::string> phemPath;
    //         phemPath.push_back(OptionsCont::getOptions().getString("phemlight-path") + "/");
    //         if (getenv("PHEMLIGHT_PATH") != nullptr) {
    //             phemPath.push_back(std::string(getenv("PHEMLIGHT_PATH")) + "/");
    //         }
    //         if (getenv("SUMO_HOME") != nullptr) {
    //             phemPath.push_back(std::string(getenv("SUMO_HOME")) + "/data/emissions/PHEMlight/");
    //         }
    //         myHelper.setCommentPrefix("c");
    //         myHelper.setPHEMDataV("V4");
    //         myHelper.setclass(eClass);
    //         if (!myCEPHandler.GetCEP(phemPath, &myHelper)) {
    //             myEmissionClassStrings.remove(eClass, index);
    //             myIndex--;
    //             throw InvalidArgument("File for PHEM emission class " + eClass + " not found.\n" + myHelper.getErrMsg());
    //         }
    //         myCEPs[index] = myCEPHandler.getCEPS().find(myHelper.getgClass())->second;
    // #ifdef INTERNAL_PHEM
    //     }
    // #endif
    //     myEmissionClassStrings.addAlias(StringUtils::to_lower_case(eClass), index);
    //     return index;
    // }
    #[allow(unused_variables)]
    pub fn get_class_by_name(&mut self, eClass: &str, vc: SUMOVehicleClass) -> SUMOEmissionClass {
        todo!("port HelpersPHEMlight::getClassByName")
    }

    /// C++: `SUMOEmissionClass HelpersPHEMlight::getClass(const SUMOEmissionClass, const std::string &, const std::string &, const std::string &, const double) const` [const, virtual]
    /// Returns the emission class described by the given parameters.
    // SUMOEmissionClass
    // HelpersPHEMlight::getClass(const SUMOEmissionClass base, const std::string& vClass, const std::string& fuel, const std::string& eClass, const double weight) const {
    //     std::string eClassOffset = "0";
    //     if (eClass.length() == 5 && eClass.substr(0, 4) == "Euro") {
    //         if (eClass[4] >= '0' && eClass[4] <= '6') {
    //             eClassOffset = eClass.substr(4, 1);
    //         }
    //     }
    //     std::string desc;
    //     if (vClass == "Passenger") {
    //         desc = "PKW_";
    //         if (fuel == "Gasoline") {
    //             desc += "G_";
    //         } else if (fuel == "Diesel") {
    //             desc += "D_";
    //         } else if (fuel == "HybridGasoline") {
    //             desc = "H_" + desc + "G_";
    //         } else if (fuel == "HybridDiesel") {
    //             desc = "H_" + desc + "G_";
    //         }
    //         desc += "EU" + eClassOffset;
    //     } else if (vClass == "Moped") {
    //         desc = "KKR_G_EU" + eClassOffset;
    //     } else if (vClass == "Motorcycle") {
    //         desc = "MR_G_EU" + eClassOffset;
    //         if (fuel == "Gasoline2S") {
    //             desc += "_2T";
    //         } else {
    //             desc += "_4T";
    //         }
    //     } else if (vClass == "Delivery") {
    //         desc = "LNF_";
    //         if (fuel == "Gasoline") {
    //             desc += "G_";
    //         } else if (fuel == "Diesel") {
    //             desc += "D_";
    //         }
    //         desc += "EU" + eClassOffset + "_I";
    //         if (weight > 1305.) {
    //             desc += "I";
    //             if (weight > 1760.) {
    //                 desc += "I";
    //             }
    //         }
    //     } else if (vClass == "UrbanBus") {
    //         desc = "LB_D_EU" + eClassOffset;
    //     } else if (vClass == "Coach") {
    //         desc = "RB_D_EU" + eClassOffset;
    //     } else if (vClass == "Truck") {
    //         desc = "Solo_LKW_D_EU" + eClassOffset + "_I";
    //         if (weight > 1305.) {
    //             desc += "I";
    //         }
    //     } else if (vClass == "Trailer") {
    //         desc = "LSZ_D_EU" + eClassOffset;
    //     }
    //     if (myEmissionClassStrings.hasString(desc)) {
    //         return myEmissionClassStrings.get(desc);
    //     }
    //     return base;
    // }
    #[allow(unused_variables)]
    pub fn get_class(&self, base: SUMOEmissionClass, vClass: &str, fuel: &str, eClass: &str, weight: f64) -> SUMOEmissionClass {
        todo!("port HelpersPHEMlight::getClass")
    }

    /// C++: `std::string HelpersPHEMlight::getAmitranVehicleClass(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the vehicle class described by this emission class as described in the Amitran interface (Passenger, ...)
    // std::string
    // HelpersPHEMlight::getAmitranVehicleClass(const SUMOEmissionClass c) const {
    //     const std::string name = myEmissionClassStrings.getString(c);
    //     if (name.find("KKR_") != std::string::npos) {
    //         return "Moped";
    //     } else if (name.find("RB_") != std::string::npos) {
    //         return "Coach";
    //     } else if (name.find("LB_") != std::string::npos) {
    //         return "UrbanBus";
    //     } else if (name.find("LNF_") != std::string::npos) {
    //         return "Delivery";
    //     } else if (name.find("LSZ_") != std::string::npos) {
    //         return "Trailer";
    //     } else if (name.find("MR_") != std::string::npos) {
    //         return "Motorcycle";
    //     } else if (name.find("LKW_") != std::string::npos) {
    //         return "Truck";
    //     }
    //     return "Passenger";
    // }
    #[allow(unused_variables)]
    pub fn get_amitran_vehicle_class(&self, c: SUMOEmissionClass) -> String {
        todo!("port HelpersPHEMlight::getAmitranVehicleClass")
    }

    /// C++: `std::string HelpersPHEMlight::getFuel(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the fuel type described by this emission class as described in the Amitran interface (Gasoline, Diesel, ...)
    // std::string
    // HelpersPHEMlight::getFuel(const SUMOEmissionClass c) const {
    //     const std::string name = myEmissionClassStrings.getString(c);
    //     std::string fuel = "Gasoline";
    //     if (name.find("_D_") != std::string::npos) {
    //         fuel = "Diesel";
    //     }
    //     if (name.find("H_") != std::string::npos) {
    //         fuel = "Hybrid" + fuel;
    //     }
    //     return fuel;
    // }
    #[allow(unused_variables)]
    pub fn get_fuel(&self, c: SUMOEmissionClass) -> String {
        todo!("port HelpersPHEMlight::getFuel")
    }

    /// C++: `int HelpersPHEMlight::getEuroClass(const SUMOEmissionClass) const` [const, virtual]
    /// Returns the Euro emission class described by this emission class as described in the Amitran interface (0, ..., 6)
    // int
    // HelpersPHEMlight::getEuroClass(const SUMOEmissionClass c) const {
    //     const std::string name = myEmissionClassStrings.getString(c);
    //     if (name.find("_EU1") != std::string::npos) {
    //         return 1;
    //     } else if (name.find("_EU2") != std::string::npos) {
    //         return 2;
    //     } else if (name.find("_EU3") != std::string::npos) {
    //         return 3;
    //     } else if (name.find("_EU4") != std::string::npos) {
    //         return 4;
    //     } else if (name.find("_EU5") != std::string::npos) {
    //         return 5;
    //     } else if (name.find("_EU6") != std::string::npos) {
    //         return 6;
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn get_euro_class(&self, c: SUMOEmissionClass) -> i32 {
        todo!("port HelpersPHEMlight::getEuroClass")
    }

    /// C++: `double HelpersPHEMlight::getWeight(const SUMOEmissionClass) const` [const, virtual]
    /// Returns a reference weight in kg described by this emission class as described in the Amitran interface This implementation returns only meaningful values for Solo_LKW (truck without trailer) and LNF (light duty vehicles).
    // double
    // HelpersPHEMlight::getWeight(const SUMOEmissionClass c) const {
    //     const std::string name = myEmissionClassStrings.getString(c);
    //     if (name.find("LNF_") != std::string::npos) {
    //         if (name.find("_III") != std::string::npos) {
    //             return 2630.;
    //         } else if (name.find("_II") != std::string::npos) {
    //             return 1532.;
    //         } else if (name.find("_I") != std::string::npos) {
    //             return 652.;
    //         }
    //     }
    //     if (name.find("Solo_LKW_") != std::string::npos) {
    //         if (name.find("_II") != std::string::npos) {
    //             return 8398.;
    //         } else if (name.find("_I") != std::string::npos) {
    //             return 18702.;
    //         }
    //     }
    //     return -1.;
    // }
    #[allow(unused_variables)]
    pub fn get_weight(&self, c: SUMOEmissionClass) -> f64 {
        todo!("port HelpersPHEMlight::getWeight")
    }

    /// C++: `double HelpersPHEMlight::compute(const SUMOEmissionClass, const PollutantsInterface::EmissionType, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the amount of emitted pollutant given the vehicle type and state (in mg/s or in ml/s for fuel)
    // double
    // HelpersPHEMlight::compute(const SUMOEmissionClass c, const PollutantsInterface::EmissionType e, const double v, const double a, const double slope, const EnergyParams* param) const {
    //     if (param != nullptr && param->isEngineOff()) {
    //         return 0.;
    //     }
    //     const double corrSpeed = MAX2(0.0, v);
    //     double power = 0.;
    // #ifdef INTERNAL_PHEM
    //     const PHEMCEP* const oldCep = PHEMCEPHandler::getHandlerInstance().GetCep(c);
    //     if (oldCep != nullptr) {
    //         if (v > IDLE_SPEED && a < oldCep->GetDecelCoast(corrSpeed, a, slope, 0)) {
    //             // coasting without power use only works if the engine runs above idle speed and
    //             // the vehicle does not accelerate beyond friction losses
    //             return 0;
    //         }
    //         power = oldCep->CalcPower(corrSpeed, a, slope);
    //     }
    // #else
    //     const PHEMCEP* const oldCep = 0;
    // #endif
    //     PHEMlightdll::CEP* currCep = myCEPs.count(c) == 0 ? 0 : myCEPs.find(c)->second;
    //     if (currCep != nullptr) {
    //         const double corrAcc = getModifiedAccel(c, corrSpeed, a, slope, param);
    //         if (currCep->getFuelType() != PHEMlightdll::Constants::strBEV &&
    //                 corrAcc < currCep->GetDecelCoast(corrSpeed, corrAcc, slope) &&
    //                 corrSpeed > PHEMlightdll::Constants::ZERO_SPEED_ACCURACY) {
    //             // the IDLE_SPEED fix above is now directly in the decel coast calculation.
    //             return 0;
    //         }
    //         power = currCep->CalcPower(corrSpeed, corrAcc, slope);
    //     }
    //     const std::string& fuelType = oldCep != nullptr ? oldCep->GetVehicleFuelType() : currCep->getFuelType();
    //     switch (e) {
    //         case PollutantsInterface::CO:
    //             return getEmission(oldCep, currCep, "CO", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::CO2:
    //             if (oldCep != nullptr) {
    //                 return getEmission(oldCep, currCep, "FC", power, corrSpeed) * 3.15 / SECONDS_PER_HOUR * 1000.;
    //             }
    //             return currCep->GetCO2Emission(getEmission(nullptr, currCep, "FC", power, corrSpeed),
    //                                            getEmission(nullptr, currCep, "CO", power, corrSpeed),
    //                                            getEmission(nullptr, currCep, "HC", power, corrSpeed), &myHelper) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::HC:
    //             return getEmission(oldCep, currCep, "HC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::NO_X:
    //             return getEmission(oldCep, currCep, "NOx", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::PM_X:
    //             return getEmission(oldCep, currCep, "PM", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
    //         case PollutantsInterface::FUEL: {
    //             if (myVolumetricFuel && fuelType == PHEMlightdll::Constants::strDiesel) { // divide by average diesel density of 836 g/l
    //                 return getEmission(oldCep, currCep, "FC", power, corrSpeed) / 836. / SECONDS_PER_HOUR * 1000.;
    //             }
    //             if (myVolumetricFuel && fuelType == PHEMlightdll::Constants::strGasoline) { // divide by average gasoline density of 742 g/l
    //                 return getEmission(oldCep, currCep, "FC", power, corrSpeed) / 742. / SECONDS_PER_HOUR * 1000.;
    //             }
    //             if (fuelType == PHEMlightdll::Constants::strBEV) {
    //                 return 0.;
    //             }
    //             return getEmission(oldCep, currCep, "FC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.; // still in mg even if myVolumetricFuel is set!
    //         }
    //         case PollutantsInterface::ELEC:
    //             if (fuelType == PHEMlightdll::Constants::strBEV) {
    //                 return getEmission(oldCep, currCep, "FC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
    //             }
    //             return 0;
    //     }
    //     // should never get here
    //     return 0.;
    // }
    #[allow(unused_variables)]
    pub fn compute(&self, c: SUMOEmissionClass, e: PollutantsInterface_EmissionType, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port HelpersPHEMlight::compute")
    }

    /// C++: `double HelpersPHEMlight::getModifiedAccel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the adapted acceleration value, useful for comparing with external PHEMlight references.
    // double
    // HelpersPHEMlight::getModifiedAccel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* /* param */) const {
    //     PHEMlightdll::CEP* currCep = myCEPs.count(c) == 0 ? 0 : myCEPs.find(c)->second;
    //     if (currCep != nullptr) {
    //         return v == 0.0 ? 0.0 : MIN2(a, currCep->GetMaxAccel(v, slope));
    //     }
    //     return a;
    // }
    #[allow(unused_variables)]
    pub fn get_modified_accel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port HelpersPHEMlight::getModifiedAccel")
    }

    /// C++: `double HelpersPHEMlight::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the maximum deceleration value (as a negative number), which can still be considered as non-braking.
    // double
    // HelpersPHEMlight::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* /* param */) const {
    //     return myCEPs.count(c) == 0 ? 0. : myCEPs.find(c)->second->GetDecelCoast(v, a, slope);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        todo!("port HelpersPHEMlight::getCoastingDecel")
    }

    /// C++: `double HelpersPHEMlight::getEmission(const PHEMCEP *, PHEMlightdll::CEP *, const std::string &, const double, const double) const` [const, private]
    /// Returns the amount of emitted pollutant given the vehicle type and state (in mg/s or in ml/s for fuel)
    // double
    // HelpersPHEMlight::getEmission(const PHEMCEP* oldCep, PHEMlightdll::CEP* currCep, const std::string& e, const double p, const double v) const {
    //     if (oldCep != nullptr) {
    //         return oldCep->GetEmission(e, p, v);
    //     }
    //     return currCep->GetEmission(e, p, v, &myHelper);
    // }
    #[allow(unused_variables)]
    pub fn get_emission(&self, oldCep: Ptr<PHEMCEP>, currCep: Ptr<CEP>, e: &str, p: f64, v: f64) -> f64 {
        todo!("port HelpersPHEMlight::getEmission")
    }

}

