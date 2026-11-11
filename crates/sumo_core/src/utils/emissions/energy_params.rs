//! Port of utils/emissions/EnergyParams.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use std::cell::Cell;
use std::f64::consts::PI;
use utils::common::msg_handler::write_warning;
use utils::common::sumo_vehicle_class::{DEFAULT_VEH_HEIGHT, DEFAULT_VEH_MASS, DEFAULT_VEH_SHUT_OFF_STOP, DEFAULT_VEH_WIDTH, SVC_CUSTOM1, SVC_CUSTOM2, SVC_E_VEHICLE, SVC_HOV, SVC_IGNORING, SVC_PASSENGER, SVC_PRIVATE, SVC_TAXI, SVC_VIP};
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::string_utils::starts_with;
use utils::common::time::steps2time;
use utils::common::to_string::join_map_to_string;
use utils::xml::sumo_xml_definitions::*;

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
static MY_PARAM_ATTRS: &[SumoXMLAttr] = &[
    SUMO_ATTR_SHUT_OFF_STOP, SUMO_ATTR_SHUT_OFF_AUTO,
    SUMO_ATTR_LOADING, SUMO_ATTR_FRONTSURFACEAREA, SUMO_ATTR_AIRDRAGCOEFFICIENT,
    SUMO_ATTR_CONSTANTPOWERINTAKE, SUMO_ATTR_WHEELRADIUS, SUMO_ATTR_ROLLDRAGCOEFFICIENT, SUMO_ATTR_ROTATINGMASS,
    SUMO_ATTR_RADIALDRAGCOEFFICIENT, SUMO_ATTR_PROPULSIONEFFICIENCY, SUMO_ATTR_RECUPERATIONEFFICIENCY,
    SUMO_ATTR_RECUPERATIONEFFICIENCY_BY_DECELERATION,
    SUMO_ATTR_MAXIMUMTORQUE, SUMO_ATTR_MAXIMUMPOWER, SUMO_ATTR_GEAREFFICIENCY, SUMO_ATTR_GEARRATIO,
    SUMO_ATTR_MAXIMUMRECUPERATIONTORQUE, SUMO_ATTR_MAXIMUMRECUPERATIONPOWER,
    SUMO_ATTR_INTERNALBATTERYRESISTANCE, SUMO_ATTR_NOMINALBATTERYVOLTAGE, SUMO_ATTR_INTERNALMOMENTOFINERTIA,
];

thread_local! {
    // C++ static member EnergyParams::myDefault
    static MY_DEFAULT: Cell<Ptr<EnergyParams>> = const { Cell::new(Ptr::null()) };
}

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
        // shortcut: C++ throws UnknownElement and emits MsgHandler warnings; this port panics on the throws until the emission API returns Result, and the VTYPEPARS_MASS_SET bit is kept here until SUMOVTypeParameter.h constants are ported
        const VTYPEPARS_MASS_SET: i64 = 1 << 32;
        let mut myMap: BTreeMap<SumoXMLAttr, f64> = BTreeMap::new();
        let mut myCharacteristicMapMap: BTreeMap<SumoXMLAttr, CharacteristicMap> = BTreeMap::new();
        let mut myHaveDefaultMass = false;
        let mut myHaveDefaultFrontSurfaceArea = false;
        myCharacteristicMapMap.insert(SUMO_ATTR_POWERLOSSMAP, CharacteristicMap::new_2("2,1|-1e9,1e9;-1e9,1e9|0,0,0,0")); // P_loss_EM = 0 W for all operating points in the default EV power loss map

        if typeParams.is_null() {
            myMap.insert(SUMO_ATTR_MASS, DEFAULT_VEH_MASS);
            myHaveDefaultMass = true;
            myMap.insert(SUMO_ATTR_FRONTSURFACEAREA, DEFAULT_VEH_WIDTH * DEFAULT_VEH_HEIGHT * PI / 4.);
            myHaveDefaultFrontSurfaceArea = true;
        } else {
            for &attr in MY_PARAM_ATTRS {
                if typeParams.has_parameter(Attrs.get_string(attr)) {
                    myMap.insert(attr, typeParams.get_double(Attrs.get_string(attr), INVALID_DOUBLE));
                }
            }
            let keys: Vec<SumoXMLAttr> = myCharacteristicMapMap.keys().copied().collect();
            for key in keys {
                let characteristicMapString = typeParams.get_parameter(Attrs.get_string(key), "");
                if characteristicMapString != "" {
                    myCharacteristicMapMap.insert(key, CharacteristicMap::new_2(&characteristicMapString));
                }
            }
            myMap.insert(SUMO_ATTR_MASS, typeParams.mass);
            myHaveDefaultMass = !typeParams.was_set(VTYPEPARS_MASS_SET);
            if myHaveDefaultMass {
                let ecMass = PollutantsInterface::get_weight(typeParams.emissionClass);
                if ecMass != -1. {
                    myMap.insert(SUMO_ATTR_MASS, ecMass);
                }
            }
            if !myMap.contains_key(&SUMO_ATTR_FRONTSURFACEAREA) {
                myHaveDefaultFrontSurfaceArea = true;
                myMap.insert(SUMO_ATTR_FRONTSURFACEAREA, typeParams.width * typeParams.height * PI / 4.);
            }
            let ecName = PollutantsInterface::get_name(typeParams.emissionClass);
            if typeParams.vehicleClass != SVC_IGNORING && (typeParams.vehicleClass & (SVC_PRIVATE | SVC_VIP | SVC_PASSENGER | SVC_HOV | SVC_TAXI | SVC_E_VEHICLE | SVC_CUSTOM1 | SVC_CUSTOM2)) == 0 && myHaveDefaultFrontSurfaceArea {
                if starts_with(&ecName, "MMPEVEM") || starts_with(&ecName, "Energy") {
                    utils::write_warningf!("Vehicle type '%' uses the emission class '%' which does not have proper defaults for its vehicle class. Please use a different emission model or complete the vType definition with further parameters.", typeParams.id, ecName);
                    if !typeParams.was_set(VTYPEPARS_MASS_SET) {
                        write_warning(" And also set a vehicle mass!");
                    }
                }
            }
            if !starts_with(&ecName, "MMPEVEM") {
                if myMap.contains_key(&SUMO_ATTR_INTERNALMOMENTOFINERTIA) {
                    utils::write_warningf!("Vehicle type '%' uses the Energy model parameter 'internalMomentOfInertia' which is deprecated. Use 'rotatingMass' instead.", typeParams.id);
                    if !typeParams.has_parameter(Attrs.get_string(SUMO_ATTR_ROTATINGMASS)) {
                        let internalMomentOfInertia = myMap[&SUMO_ATTR_INTERNALMOMENTOFINERTIA];
                        myMap.insert(SUMO_ATTR_ROTATINGMASS, internalMomentOfInertia);
                    }
                }
            }
        }

        EnergyParams {
            myMap,
            myCharacteristicMapMap,
            mySecondaryParams: Ptr::null(),
            myHaveDefaultMass,
            myHaveDefaultFrontSurfaceArea,
            myStopDurationSeconds: -1.,
            myAmParking: false,
            myWaitingTimeSeconds: -1.,
            myAngleDiff: 0.,
            myTransportableMass: 0.,
        }
    }

    /// C++: `void EnergyParams::EnergyParams(const EnergyParams *)`
    /// Constructor
    // EnergyParams(const EnergyParams* secondaryParams) : mySecondaryParams(secondaryParams) {}
    #[allow(unused_variables)]
    pub fn new_2(secondaryParams: Ptr<EnergyParams>) -> Self {
        EnergyParams {
            myMap: BTreeMap::new(),
            myCharacteristicMapMap: BTreeMap::new(),
            mySecondaryParams: secondaryParams,
            myHaveDefaultMass: false,
            myHaveDefaultFrontSurfaceArea: false,
            myStopDurationSeconds: -1.,
            myAmParking: false,
            myWaitingTimeSeconds: -1.,
            myAngleDiff: 0.,
            myTransportableMass: 0.,
        }
    }

    /// C++: `void EnergyParams::~EnergyParams()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // Implicit C++ destructor: the owned maps are released automatically
    }

    /// C++: `void EnergyParams::setSecondary(const EnergyParams *)`
    /// Set secondary params
    // void setSecondary(const EnergyParams* secondaryParams) {
    //         mySecondaryParams = secondaryParams;
    //     }
    #[allow(unused_variables)]
    pub fn set_secondary(&mut self, secondaryParams: Ptr<EnergyParams>) {
        self.mySecondaryParams = secondaryParams;
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
        if (stopDuration >= 0 && self.myStopDurationSeconds < 0.) || (stopDuration < 0 && self.myStopDurationSeconds >= 0.) {
            self.myStopDurationSeconds = steps2time(stopDuration);
            self.myAmParking = parking;
        }
        self.myWaitingTimeSeconds = steps2time(waitingTime);
        self.myAngleDiff = angleDiff;
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
        self.myMap.insert(SUMO_ATTR_MASS, mass);
        self.myHaveDefaultMass = false;
    }

    /// C++: `double EnergyParams::getTransportableMass() const` [const]
    /// Returns the mass of all transportables in the vehicle
    // double getTransportableMass() const {
    //         return myTransportableMass;
    //     }
    #[allow(unused_variables)]
    pub fn get_transportable_mass(&self) -> f64 {
        self.myTransportableMass
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
        self.myTransportableMass = mass;
        assert!(self.myTransportableMass >= 0.);
    }

    /// C++: `double EnergyParams::getTotalMass(const double, const double) const` [const]
    /// Returns the sum of the empty mass (SUMO_ATTR_MASS), tthe loading (SUMO_ATTR_LOADING) and the mass of all transportables in the vehicle
    // double
    // EnergyParams::getTotalMass(const double defaultEmptyMass, const double defaultLoading) const {
    //     return getDoubleOptional(SUMO_ATTR_MASS, defaultEmptyMass) + getDoubleOptional(SUMO_ATTR_LOADING, defaultLoading) + getTransportableMass();
    // }
    #[allow(unused_variables)]
    pub fn get_total_mass(&self, defaultEmptyMass: f64, defaultLoading: f64) -> f64 {
        self.get_double_optional(SUMO_ATTR_MASS, defaultEmptyMass) + self.get_double_optional(SUMO_ATTR_LOADING, defaultLoading) + self.get_transportable_mass()
    }

    /// C++: `double EnergyParams::getAngleDiff() const` [const]
    /// Returns the angle difference between the last two calls of setDynamicValues (usually the last two time steps)
    // double
    // EnergyParams::getAngleDiff() const {
    //     return myAngleDiff;
    // }
    #[allow(unused_variables)]
    pub fn get_angle_diff(&self) -> f64 {
        self.myAngleDiff
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
        if let Some(&v) = self.myMap.get(&attr) {
            return v;
        }
        if !self.mySecondaryParams.is_null() {
            return self.mySecondaryParams.get_double(attr);
        }
        panic!("Unknown emission model parameter: {}", Attrs.get_string(attr));
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
        if let Some(&v) = self.myMap.get(&attr) {
            if v != INVALID_DOUBLE {
                if attr == SUMO_ATTR_MASS {
                    if !self.myHaveDefaultMass {
                        return v;
                    }
                } else if attr == SUMO_ATTR_FRONTSURFACEAREA {
                    if !self.myHaveDefaultFrontSurfaceArea {
                        return v;
                    }
                } else {
                    return v;
                }
            }
        }
        if !self.mySecondaryParams.is_null() {
            return self.mySecondaryParams.get_double_optional(attr, def);
        }
        def
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
        if let Some(m) = self.myCharacteristicMapMap.get(&attr) {
            return m;
        }
        if !self.mySecondaryParams.is_null() {
            return self.mySecondaryParams.get_characteristic_map(attr);
        }
        panic!("Unknown emission model parameter: {}", Attrs.get_string(attr));
    }

    /// C++: `const std::string EnergyParams::dump() const` [const]
    /// Returns a complete inner description
    // const std::string dump() const {
    //         return joinToString(myMap, ", ", ":") + (mySecondaryParams ? mySecondaryParams->dump() : "");
    //     }
    #[allow(unused_variables)]
    pub fn dump(&self) -> String {
        let mut s = join_map_to_string(&self.myMap, ", ", ":");
        if !self.mySecondaryParams.is_null() {
            s += &self.mySecondaryParams.dump();
        }
        s
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
        self.myStopDurationSeconds > self.get_double_optional(SUMO_ATTR_SHUT_OFF_STOP, DEFAULT_VEH_SHUT_OFF_STOP)
            || self.myWaitingTimeSeconds > self.get_double_optional(SUMO_ATTR_SHUT_OFF_AUTO, f64::MAX)
    }

    /// C++: `bool EnergyParams::isOff() const` [const]
    /// Returns whether the vehicle is currently consuming any energy derived from the parking state
    // bool
    // EnergyParams::isOff() const {
    //     return myStopDurationSeconds > getDoubleOptional(SUMO_ATTR_SHUT_OFF_STOP, DEFAULT_VEH_SHUT_OFF_STOP);
    // }
    #[allow(unused_variables)]
    pub fn is_off(&self) -> bool {
        self.myStopDurationSeconds > self.get_double_optional(SUMO_ATTR_SHUT_OFF_STOP, DEFAULT_VEH_SHUT_OFF_STOP)
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
        MY_DEFAULT.with(|d| {
            if d.get().is_null() {
                d.set(Ptr::from_mut(Box::leak(Box::new(EnergyParams::new(Ptr::null())))));
            }
            d.get()
        })
    }

    /// C++: `void EnergyParams::EnergyParams(const EnergyParams &)` [private]
    /// invalidate copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_3(s: &EnergyParams) -> Self {
        panic!("EnergyParams copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `EnergyParams & EnergyParams::operator=(const EnergyParams &)` [private]
    /// invalidate assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &EnergyParams) -> &mut EnergyParams {
        panic!("EnergyParams assignment operator is private and undefined in C++; it cannot be called")
    }

}

