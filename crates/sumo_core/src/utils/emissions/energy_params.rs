//! Port of utils/emissions/EnergyParams.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An upper class for objects with additional parameters
/// C++ class `EnergyParams` (utils/emissions/EnergyParams.h)
pub struct EnergyParams {
    /// The key->value maps
    pub myMap: BTreeMap<SumoXMLAttr, f64>,
    pub myCharacteristicMapMap: BTreeMap<SumoXMLAttr, CharacteristicMap>,
    pub mySecondaryParams: Ptr<EnergyParams>,
    pub myHaveDefaultMass: bool,
    pub myHaveDefaultFrontSurfaceArea: bool,
    pub myStopDurationSeconds: f64,
    pub myAmParking: bool,
    pub myWaitingTimeSeconds: f64,
    pub myAngleDiff: f64,
    pub myTransportableMass: f64,
}

// C++ static members of EnergyParams (port as module-level statics, see AGENTS.md):
//   static const EnergyParams * EnergyParams::myDefault;
//   static const std::vector<SumoXMLAttr> EnergyParams::myParamAttrs;

impl EnergyParams {
    /// C++: `void EnergyParams::EnergyParams(const SUMOVTypeParameter *)`
    /// Constructor
    // EnergyParams::EnergyParams(const SUMOVTypeParameter* typeParams) {
    //     myCharacteristicMapMap.insert(std::pair<SumoXMLAttr, CharacteristicMap>(SUMO_ATTR_POWERLOSSMAP, CharacteristicMap("2,1|-1e9,1e9;-1e9,1e9|0,0,0,0")));  // P_loss_EM = 0 W for all operating points in the default EV power loss map
    //
    //     if (typeParams == nullptr) {
    //         myMap[SUMO_ATTR_MASS] = DEFAULT_VEH_MASS;
    //         myHaveDefaultMass = true;
    //         myMap[SUMO_ATTR_FRONTSURFACEAREA] = DEFAULT_VEH_WIDTH * DEFAULT_VEH_HEIGHT * M_PI / 4.;
    //         myHaveDefaultFrontSurfaceArea = true;
    //     } else {
    //         for (SumoXMLAttr attr : myParamAttrs) {
    //             if (typeParams->hasParameter(toString(attr))) {
    //                 myMap[attr] = typeParams->getDouble(toString(attr), INVALID_DOUBLE);
    //             }
    //         }
    //         for (auto item : myCharacteristicMapMap) {
    //             std::string characteristicMapString = typeParams->getParameter(toString(item.first), "");
    //             if (characteristicMapString != "") {
    //                 myCharacteristicMapMap.at(item.first) = CharacteristicMap(typeParams->getParameter(toString(item.first)));
    //             }
    //         }
    //         myMap[SUMO_ATTR_MASS] = typeParams->mass;
    //         myHaveDefaultMass = !typeParams->wasSet(VTYPEPARS_MASS_SET);
    //         if (myHaveDefaultMass) {
    //             const double ecMass = PollutantsInterface::getWeight(typeParams->emissionClass);
    //             if (ecMass != -1.) {
    //                 myMap[SUMO_ATTR_MASS] = ecMass;
    //             }
    //         }
    //         if (myMap.count(SUMO_ATTR_FRONTSURFACEAREA) == 0) {
    //             myHaveDefaultFrontSurfaceArea = true;
    //             myMap[SUMO_ATTR_FRONTSURFACEAREA] = typeParams->width * typeParams->height * M_PI / 4.;
    //         }
    //         const std::string& ecName = PollutantsInterface::getName(typeParams->emissionClass);
    //         if (typeParams->vehicleClass != SVC_IGNORING && (typeParams->vehicleClass & (SVC_PRIVATE | SVC_VIP | SVC_PASSENGER | SVC_HOV | SVC_TAXI | SVC_E_VEHICLE | SVC_CUSTOM1 | SVC_CUSTOM2)) == 0 && myHaveDefaultFrontSurfaceArea) {
    //             if (StringUtils::startsWith(ecName, "MMPEVEM") || StringUtils::startsWith(ecName, "Energy")) {
    //                 WRITE_WARNINGF(TL("Vehicle type '%' uses the emission class '%' which does not have proper defaults for its vehicle class. "
    //                                   "Please use a different emission model or complete the vType definition with further parameters."), typeParams->id, ecName);
    //                 if (!typeParams->wasSet(VTYPEPARS_MASS_SET)) {
    //                     WRITE_WARNING(TL(" And also set a vehicle mass!"));
    //                 }
    //             }
    //         }
    //         if (!StringUtils::startsWith(ecName, "MMPEVEM")) {
    //             if (myMap.count(SUMO_ATTR_INTERNALMOMENTOFINERTIA) > 0) {
    //                 WRITE_WARNINGF(TL("Vehicle type '%' uses the Energy model parameter 'internalMomentOfInertia' which is deprecated. Use 'rotatingMass' instead."), typeParams->id);
    //                 if (!typeParams->hasParameter(toString(SUMO_ATTR_ROTATINGMASS))) {
    //                     myMap[SUMO_ATTR_ROTATINGMASS] = myMap[SUMO_ATTR_INTERNALMOMENTOFINERTIA];
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(typeParams: Ptr<SUMOVTypeParameter>) -> Self {
        todo!("port EnergyParams::EnergyParams")
    }

    /// C++: `void EnergyParams::EnergyParams(const EnergyParams *)`
    /// Constructor
    // EnergyParams(const EnergyParams* secondaryParams) : mySecondaryParams(secondaryParams) {}
    #[allow(unused_variables)]
    pub fn new_2(secondaryParams: Ptr<EnergyParams>) -> Self {
        todo!("port EnergyParams::EnergyParams")
    }

    /// C++: `void EnergyParams::~EnergyParams()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port EnergyParams::~EnergyParams")
    }

    /// C++: `void EnergyParams::setSecondary(const EnergyParams *)`
    /// Set secondary params
    // void setSecondary(const EnergyParams* secondaryParams) {
    //         mySecondaryParams = secondaryParams;
    //     }
    #[allow(unused_variables)]
    pub fn set_secondary(&mut self, secondaryParams: Ptr<EnergyParams>) {
        todo!("port EnergyParams::setSecondary")
    }

    /// C++: `void EnergyParams::setDynamicValues(const SUMOTime, const bool, const SUMOTime, const double)`
    /// Sets the values which change possibly in every simulation step and are relevant for emsssion calculation
    // void
    // EnergyParams::setDynamicValues(const SUMOTime stopDuration, const bool parking, const SUMOTime waitingTime, const double angleDiff) {
    //     if ((stopDuration >= 0 && myStopDurationSeconds < 0.) || (stopDuration < 0 && myStopDurationSeconds >= 0.)) {
    //         myStopDurationSeconds = STEPS2TIME(stopDuration);
    //         myAmParking = parking;
    //     }
    //     myWaitingTimeSeconds = STEPS2TIME(waitingTime);
    //     myAngleDiff = angleDiff;
    // }
    #[allow(unused_variables)]
    pub fn set_dynamic_values(&mut self, stopDuration: SUMOTime, parking: bool, waitingTime: SUMOTime, angleDiff: f64) {
        todo!("port EnergyParams::setDynamicValues")
    }

    /// C++: `void EnergyParams::setMass(const double)`
    /// Sets the empty mass of the vehicle (type) This is to be used by traci/libsumo
    // void
    // EnergyParams::setMass(const double mass) {
    //     myMap[SUMO_ATTR_MASS] = mass;
    //     myHaveDefaultMass = false;
    // }
    #[allow(unused_variables)]
    pub fn set_mass(&mut self, mass: f64) {
        todo!("port EnergyParams::setMass")
    }

    /// C++: `double EnergyParams::getTransportableMass() const` [const]
    /// Returns the mass of all transportables in the vehicle
    // double getTransportableMass() const {
    //         return myTransportableMass;
    //     }
    #[allow(unused_variables)]
    pub fn get_transportable_mass(&self) -> f64 {
        todo!("port EnergyParams::getTransportableMass")
    }

    /// C++: `void EnergyParams::setTransportableMass(const double)`
    /// Sets the mass of all transportables in the vehicle
    // void
    // EnergyParams::setTransportableMass(const double mass) {
    //     myTransportableMass = mass;
    //     assert(myTransportableMass >= 0);
    // }
    #[allow(unused_variables)]
    pub fn set_transportable_mass(&mut self, mass: f64) {
        todo!("port EnergyParams::setTransportableMass")
    }

    /// C++: `double EnergyParams::getTotalMass(const double, const double) const` [const]
    /// Returns the sum of the empty mass (SUMO_ATTR_MASS), tthe loading (SUMO_ATTR_LOADING) and the mass of all transportables in the vehicle
    // double
    // EnergyParams::getTotalMass(const double defaultEmptyMass, const double defaultLoading) const {
    //     return getDoubleOptional(SUMO_ATTR_MASS, defaultEmptyMass) + getDoubleOptional(SUMO_ATTR_LOADING, defaultLoading) + getTransportableMass();
    // }
    #[allow(unused_variables)]
    pub fn get_total_mass(&self, defaultEmptyMass: f64, defaultLoading: f64) -> f64 {
        todo!("port EnergyParams::getTotalMass")
    }

    /// C++: `double EnergyParams::getAngleDiff() const` [const]
    /// Returns the angle difference between the last two calls of setDynamicValues (usually the last two time steps)
    // double
    // EnergyParams::getAngleDiff() const {
    //     return myAngleDiff;
    // }
    #[allow(unused_variables)]
    pub fn get_angle_diff(&self) -> f64 {
        todo!("port EnergyParams::getAngleDiff")
    }

    /// C++: `double EnergyParams::getDouble(SumoXMLAttr) const` [const]
    /// Returns the value for a given key
    // double
    // EnergyParams::getDouble(SumoXMLAttr attr) const {
    //     auto it = myMap.find(attr);
    //     if (it != myMap.end()) {
    //         return it->second;
    //     }
    //     if (mySecondaryParams != nullptr) {
    //         return mySecondaryParams->getDouble(attr);
    //     }
    //     throw UnknownElement("Unknown emission model parameter: " + toString(attr));
    // }
    #[allow(unused_variables)]
    pub fn get_double(&self, attr: SumoXMLAttr) -> f64 {
        todo!("port EnergyParams::getDouble")
    }

    /// C++: `double EnergyParams::getDoubleOptional(SumoXMLAttr, const double) const` [const]
    /// Returns the value for a given key with an optional default. SUMO_ATTR_MASS and SUMO_ATTR_FRONTSURFACEAREA get a special treatment and return the given def value also if the map has a value which is flagged as default.
    // double
    // EnergyParams::getDoubleOptional(SumoXMLAttr attr, const double def) const {
    //     auto it = myMap.find(attr);
    //     if (it != myMap.end() && it->second != INVALID_DOUBLE) {
    //         if (attr == SUMO_ATTR_MASS) {
    //             if (!myHaveDefaultMass) {
    //                 return it->second;
    //             }
    //         } else if (attr == SUMO_ATTR_FRONTSURFACEAREA) {
    //             if (!myHaveDefaultFrontSurfaceArea) {
    //                 return it->second;
    //             }
    //         } else {
    //             return it->second;
    //         }
    //     }
    //     if (mySecondaryParams != nullptr) {
    //         return mySecondaryParams->getDoubleOptional(attr, def);
    //     }
    //     return def;
    // }
    #[allow(unused_variables)]
    pub fn get_double_optional(&self, attr: SumoXMLAttr, def: f64) -> f64 {
        todo!("port EnergyParams::getDoubleOptional")
    }

    /// C++: `const CharacteristicMap & EnergyParams::getCharacteristicMap(SumoXMLAttr) const` [const]
    /// Return the CharacteristicMap that belongs to a given attribute.
    // const CharacteristicMap&
    // EnergyParams::getCharacteristicMap(SumoXMLAttr attr) const {
    //     auto it = myCharacteristicMapMap.find(attr);
    //     if (it != myCharacteristicMapMap.end()) {
    //         return it->second;
    //     }
    //     if (mySecondaryParams != nullptr) {
    //         return mySecondaryParams->getCharacteristicMap(attr);
    //     }
    //     throw UnknownElement("Unknown emission model parameter: " + toString(attr));
    // }
    #[allow(unused_variables)]
    pub fn get_characteristic_map(&self, attr: SumoXMLAttr) -> &CharacteristicMap {
        todo!("port EnergyParams::getCharacteristicMap")
    }

    /// C++: `const std::string EnergyParams::dump() const` [const]
    /// Returns a complete inner description
    // const std::string dump() const {
    //         return joinToString(myMap, ", ", ":") + (mySecondaryParams ? mySecondaryParams->dump() : "");
    //     }
    #[allow(unused_variables)]
    pub fn dump(&self) -> String {
        todo!("port EnergyParams::dump")
    }

    /// C++: `bool EnergyParams::isEngineOff() const` [const]
    /// Returns the state of the engine when the vehicle is not moving
    // bool
    // EnergyParams::isEngineOff() const {
    //     return myStopDurationSeconds > getDoubleOptional(SUMO_ATTR_SHUT_OFF_STOP, DEFAULT_VEH_SHUT_OFF_STOP) ||
    //            myWaitingTimeSeconds > getDoubleOptional(SUMO_ATTR_SHUT_OFF_AUTO, std::numeric_limits<double>::max());
    // }
    #[allow(unused_variables)]
    pub fn is_engine_off(&self) -> bool {
        todo!("port EnergyParams::isEngineOff")
    }

    /// C++: `bool EnergyParams::isOff() const` [const]
    /// Returns whether the vehicle is currently consuming any energy derived from the parking state
    // bool
    // EnergyParams::isOff() const {
    //     return myStopDurationSeconds > getDoubleOptional(SUMO_ATTR_SHUT_OFF_STOP, DEFAULT_VEH_SHUT_OFF_STOP);
    // }
    #[allow(unused_variables)]
    pub fn is_off(&self) -> bool {
        todo!("port EnergyParams::isOff")
    }

    /// C++: `const EnergyParams * EnergyParams::getDefault()` [static]
    // static const EnergyParams* getDefault() {
    //         if (myDefault == nullptr) {
    //             myDefault = new EnergyParams();
    //         }
    //         return myDefault;
    //     }
    #[allow(unused_variables)]
    pub fn get_default() -> Ptr<EnergyParams> {
        todo!("port EnergyParams::getDefault")
    }

    /// C++: `void EnergyParams::EnergyParams(const EnergyParams &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &EnergyParams) -> Self {
        todo!("port EnergyParams::EnergyParams")
    }

    /// C++: `EnergyParams & EnergyParams::operator=(const EnergyParams &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &EnergyParams) -> &mut EnergyParams {
        todo!("port EnergyParams::operator=")
    }

}

