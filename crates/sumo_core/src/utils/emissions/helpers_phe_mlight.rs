//! Port of utils/emissions/HelpersPHEMlight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use foreign::phemlight::constants as pc4;

/// C++ `HelpersPHEMlight::PHEMLIGHT_BASE` (3 << 16)
pub const PHEMLIGHT_BASE: i32 = 3 << 16;
/// C++ `SECONDS_PER_HOUR` (utils/emissions/PHEMConstants.h)
pub const SECONDS_PER_HOUR: f64 = 3600.;

/// Helper methods for PHEMlight-based emission computation
/// C++ class `HelpersPHEMlight` (utils/emissions/HelpersPHEMlight.h)
#[repr(C)]
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
        Self {
            base: PollutantsInterface_Helper::new("PHEMlight".to_string(), PHEMLIGHT_BASE, -1),
            myIndex: PHEMLIGHT_BASE,
            myCEPHandler: CEPHandler::new(),
            myHelper: Helpers::default(),
            myCEPs: BTreeMap::new(),
        }
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
        // shortcut: the CEP objects are owned by myCEPHandler in this port, so the C++ delete loop only clears the index map, upgrade if CEPs move out of the handler
        self.myCEPs.clear();
    }

    /// C++: `void HelpersPHEMlight::HelpersPHEMlight(std::string, const int, const int)` [protected]
    /// Constructor for subclasses
    // HelpersPHEMlight(std::string name, const int baseIndex, const int defaultClass)
    //         : PollutantsInterface::Helper(name, baseIndex, defaultClass) {}
    #[allow(unused_variables)]
    pub fn new_2(name: String, baseIndex: i32, defaultClass: i32) -> Self {
        Self {
            base: PollutantsInterface_Helper::new(name, baseIndex, defaultClass),
            // C++ leaves myIndex uninitialized in this constructor; subclasses set it
            myIndex: baseIndex,
            myCEPHandler: CEPHandler::new(),
            myHelper: Helpers::default(),
            myCEPs: BTreeMap::new(),
        }
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
        if eClass == "unknown" && !self.myEmissionClassStrings.has_string("unknown") {
            let alias = self.get_class_by_name("PC_G_EU4", vc);
            self.myEmissionClassStrings.add_alias("unknown", alias);
        }
        if eClass == "default" && !self.myEmissionClassStrings.has_string("default") {
            let alias = self.get_class_by_name("PC_G_EU4", vc);
            self.myEmissionClassStrings.add_alias("default", alias);
        }
        if self.myEmissionClassStrings.has_string(eClass) {
            return self.myEmissionClassStrings.get(eClass).unwrap();
        }
        if eClass.len() < 6 {
            panic!("Unknown emission class '{}'.", eClass);
        }
        let mut index = self.myIndex;
        self.myIndex += 1;
        let typ = &eClass.as_bytes()[..3];
        if matches!(typ, b"HDV" | b"LB_" | b"RB_" | b"LSZ") || eClass.contains("LKW") {
            index |= HEAVY_BIT;
        }
        self.myEmissionClassStrings.insert(eClass, index, true).unwrap();
        // the INTERNAL_PHEM handler lookup is not compiled in regular builds, so the PHEMlight CEP is always loaded here
        self.myVolumetricFuel = utils::options::options_cont::get_bool("emissions.volumetric-fuel");
        let mut phemPath: Vec<String> = Vec::new();
        phemPath.push(format!("{}/", utils::options::options_cont::get_string("phemlight-path")));
        if let Ok(p) = std::env::var("PHEMLIGHT_PATH") {
            phemPath.push(format!("{}/", p));
        }
        if let Ok(s) = std::env::var("SUMO_HOME") {
            phemPath.push(format!("{}/data/emissions/PHEMlight/", s));
        }
        self.myHelper.set_comment_prefix("c");
        self.myHelper.set_phem_data_v("V4");
        self.myHelper.setclass(eClass);
        if !self.myCEPHandler.get_cep(&phemPath, &mut self.myHelper) {
            self.myEmissionClassStrings.remove(eClass, index);
            self.myIndex -= 1;
            panic!("File for PHEM emission class {} not found.\n{}", eClass, self.myHelper.get_err_msg());
        }
        let gClass = self.myHelper.get_g_class().to_string();
        let cep = Ptr::new(self.myCEPHandler.get_ceps().get(&gClass).unwrap());
        self.myCEPs.insert(index, cep);
        self.myEmissionClassStrings.add_alias(&utils::common::string_utils::to_lower_case(eClass), index);
        index
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
        let mut eClassOffset = "0".to_string();
        if eClass.len() == 5 && eClass.starts_with("Euro") {
            if (b'0'..=b'6').contains(&eClass.as_bytes()[4]) {
                eClassOffset = eClass[4..5].to_string();
            }
        }
        let mut desc = String::new();
        if vClass == "Passenger" {
            desc = "PKW_".to_string();
            if fuel == "Gasoline" {
                desc += "G_";
            } else if fuel == "Diesel" {
                desc += "D_";
            } else if fuel == "HybridGasoline" {
                desc = format!("H_{}G_", desc);
            } else if fuel == "HybridDiesel" {
                desc = format!("H_{}G_", desc);
            }
            desc += &format!("EU{}", eClassOffset);
        } else if vClass == "Moped" {
            desc = format!("KKR_G_EU{}", eClassOffset);
        } else if vClass == "Motorcycle" {
            desc = format!("MR_G_EU{}", eClassOffset);
            if fuel == "Gasoline2S" {
                desc += "_2T";
            } else {
                desc += "_4T";
            }
        } else if vClass == "Delivery" {
            desc = "LNF_".to_string();
            if fuel == "Gasoline" {
                desc += "G_";
            } else if fuel == "Diesel" {
                desc += "D_";
            }
            desc += &format!("EU{}_I", eClassOffset);
            if weight > 1305. {
                desc += "I";
                if weight > 1760. {
                    desc += "I";
                }
            }
        } else if vClass == "UrbanBus" {
            desc = format!("LB_D_EU{}", eClassOffset);
        } else if vClass == "Coach" {
            desc = format!("RB_D_EU{}", eClassOffset);
        } else if vClass == "Truck" {
            desc = format!("Solo_LKW_D_EU{}_I", eClassOffset);
            if weight > 1305. {
                desc += "I";
            }
        } else if vClass == "Trailer" {
            desc = format!("LSZ_D_EU{}", eClassOffset);
        }
        if self.myEmissionClassStrings.has_string(&desc) {
            return self.myEmissionClassStrings.get(&desc).unwrap();
        }
        base
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
        let name = self.myEmissionClassStrings.get_string(c).unwrap();
        if name.contains("KKR_") {
            "Moped".to_string()
        } else if name.contains("RB_") {
            "Coach".to_string()
        } else if name.contains("LB_") {
            "UrbanBus".to_string()
        } else if name.contains("LNF_") {
            "Delivery".to_string()
        } else if name.contains("LSZ_") {
            "Trailer".to_string()
        } else if name.contains("MR_") {
            "Motorcycle".to_string()
        } else if name.contains("LKW_") {
            "Truck".to_string()
        } else {
            "Passenger".to_string()
        }
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
        let name = self.myEmissionClassStrings.get_string(c).unwrap();
        let mut fuel = "Gasoline".to_string();
        if name.contains("_D_") {
            fuel = "Diesel".to_string();
        }
        if name.contains("H_") {
            fuel = format!("Hybrid{}", fuel);
        }
        fuel
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
        let name = self.myEmissionClassStrings.get_string(c).unwrap();
        if name.contains("_EU1") {
            1
        } else if name.contains("_EU2") {
            2
        } else if name.contains("_EU3") {
            3
        } else if name.contains("_EU4") {
            4
        } else if name.contains("_EU5") {
            5
        } else if name.contains("_EU6") {
            6
        } else {
            0
        }
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
        let name = self.myEmissionClassStrings.get_string(c).unwrap();
        if name.contains("LNF_") {
            if name.contains("_III") {
                return 2630.;
            } else if name.contains("_II") {
                return 1532.;
            } else if name.contains("_I") {
                return 652.;
            }
        }
        if name.contains("Solo_LKW_") {
            if name.contains("_II") {
                return 8398.;
            } else if name.contains("_I") {
                return 18702.;
            }
        }
        -1.
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
        if !param.is_null() && param.m().is_engine_off() {
            return 0.;
        }
        let corrSpeed = if 0.0 > v { 0.0 } else { v }; // MAX2(0.0, v)
        let mut power = 0.;
        // the INTERNAL_PHEM PHEMCEP lookup is not compiled in regular builds, so oldCep is always null here
        let oldCep: Ptr<PHEMCEP> = Ptr::null();
        let currCep: Ptr<CEP> = self.myCEPs.get(&c).copied().unwrap_or(Ptr::null());
        if !currCep.is_null() {
            let corrAcc = self.get_modified_accel(c, corrSpeed, a, slope, param);
            if currCep.m().get_fuel_type() != pc4::STR_BEV && corrAcc < currCep.m().get_decel_coast(corrSpeed, corrAcc, slope) && corrSpeed > pc4::ZERO_SPEED_ACCURACY {
                // the IDLE_SPEED fix above is now directly in the decel coast calculation.
                return 0.;
            }
            power = currCep.m().calc_power(corrSpeed, corrAcc, slope);
        }
        let fuelType: String = if !oldCep.is_null() {
            oldCep.m().get_vehicle_fuel_type().to_string()
        } else {
            currCep.m().get_fuel_type().to_string()
        };
        match e {
            PollutantsInterface_EmissionType::CO => {
                self.get_emission(oldCep, currCep, "CO", power, corrSpeed) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::CO2 => {
                if !oldCep.is_null() {
                    return self.get_emission(oldCep, currCep, "FC", power, corrSpeed) * 3.15 / SECONDS_PER_HOUR * 1000.;
                }
                let fc = self.get_emission(Ptr::null(), currCep, "FC", power, corrSpeed);
                let co = self.get_emission(Ptr::null(), currCep, "CO", power, corrSpeed);
                let hc = self.get_emission(Ptr::null(), currCep, "HC", power, corrSpeed);
                currCep.m().get_co2_emission(fc, co, hc, Ptr::new(&self.myHelper).m()) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::HC => {
                self.get_emission(oldCep, currCep, "HC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::NO_X => {
                self.get_emission(oldCep, currCep, "NOx", power, corrSpeed) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::PM_X => {
                self.get_emission(oldCep, currCep, "PM", power, corrSpeed) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::FUEL => {
                if self.myVolumetricFuel && fuelType == pc4::STR_DIESEL {
                    // divide by average diesel density of 836 g/l
                    return self.get_emission(oldCep, currCep, "FC", power, corrSpeed) / 836. / SECONDS_PER_HOUR * 1000.;
                }
                if self.myVolumetricFuel && fuelType == pc4::STR_GASOLINE {
                    // divide by average gasoline density of 742 g/l
                    return self.get_emission(oldCep, currCep, "FC", power, corrSpeed) / 742. / SECONDS_PER_HOUR * 1000.;
                }
                if fuelType == pc4::STR_BEV {
                    return 0.;
                }
                // still in mg even if myVolumetricFuel is set!
                self.get_emission(oldCep, currCep, "FC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.
            }
            PollutantsInterface_EmissionType::ELEC => {
                if fuelType == pc4::STR_BEV {
                    return self.get_emission(oldCep, currCep, "FC", power, corrSpeed) / SECONDS_PER_HOUR * 1000.;
                }
                0.
            }
        }
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
        if let Some(currCep) = self.myCEPs.get(&c).copied() {
            if v == 0.0 {
                return 0.0;
            }
            let m = currCep.m().get_max_accel(v, slope);
            return if a < m { a } else { m }; // MIN2(a, currCep->GetMaxAccel(v, slope))
        }
        a
    }

    /// C++: `double HelpersPHEMlight::getCoastingDecel(const SUMOEmissionClass, const double, const double, const double, const EnergyParams *) const` [const, virtual]
    /// Returns the maximum deceleration value (as a negative number), which can still be considered as non-braking.
    // double
    // HelpersPHEMlight::getCoastingDecel(const SUMOEmissionClass c, const double v, const double a, const double slope, const EnergyParams* /* param */) const {
    //     return myCEPs.count(c) == 0 ? 0. : myCEPs.find(c)->second->GetDecelCoast(v, a, slope);
    // }
    #[allow(unused_variables)]
    pub fn get_coasting_decel(&self, c: SUMOEmissionClass, v: f64, a: f64, slope: f64, param: Ptr<EnergyParams>) -> f64 {
        match self.myCEPs.get(&c) {
            None => 0.,
            Some(cep) => cep.m().get_decel_coast(v, a, slope),
        }
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
        if !oldCep.is_null() {
            return oldCep.m().get_emission(e, p, v, false);
        }
        currCep.m().get_emission(e, p, v, Ptr::new(&self.myHelper).m())
    }

}

