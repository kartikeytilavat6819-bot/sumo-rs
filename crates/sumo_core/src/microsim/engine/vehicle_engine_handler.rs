//! Port of microsim/engine/VehicleEngineHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::xml::generic_sax_handler::SaxHandler;

/// C++ enum `VehicleEngineHandler::EngineXMLAttr`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum VehicleEngineHandler_EngineXMLAttr {
    ENGINE_ATTR_NOTHING = 0,
    ENGINE_ATTR_ID = 1,
    ENGINE_ATTR_GEAR_N = 2,
    ENGINE_ATTR_GEAR_RATIO = 3,
    ENGINE_ATTR_MASS = 4,
    ENGINE_ATTR_MASS_FACTOR = 5,
    ENGINE_ATTR_WHEELS_DIAMETER = 6,
    ENGINE_ATTR_WHEELS_FRICTION = 7,
    ENGINE_ATTR_WHEELS_CR1 = 8,
    ENGINE_ATTR_WHEELS_CR2 = 9,
    ENGINE_ATTR_DRAG_CAIR = 10,
    ENGINE_ATTR_DRAG_SECTION = 11,
    ENGINE_ATTR_ENGINE_TYPE = 12,
    ENGINE_ATTR_ENGINE_EFFICIENCY = 13,
    ENGINE_ATTR_ENGINE_CYLINDERS = 14,
    ENGINE_ATTR_ENGINE_MINRPM = 15,
    ENGINE_ATTR_ENGINE_MAXRPM = 16,
    ENGINE_ATTR_ENGINE_TAU_EX = 17,
    ENGINE_ATTR_ENGINE_TAU_BURN = 18,
    ENGINE_ATTR_ENGINE_POWER_RPM = 19,
    ENGINE_ATTR_ENGINE_POWER_HP = 20,
    ENGINE_ATTR_ENGINE_POWER_KW = 21,
    ENGINE_ATTR_ENGINE_POWER_SLOPE = 22,
    ENGINE_ATTR_ENGINE_POWER_INTERCEPT = 23,
    ENGINE_ATTR_SHIFTING_RPM = 24,
    ENGINE_ATTR_SHIFTING_DELTARPM = 25,
    ENGINE_ATTR_BRAKES_TAU = 26,
}

/// C++ enum `VehicleEngineHandler::EngineXMLTag`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum VehicleEngineHandler_EngineXMLTag {
    ENGINE_TAG_NOTHING = 0,
    ENGINE_TAG_VEHICLES = 1,
    ENGINE_TAG_VEHICLE = 2,
    ENGINE_TAG_GEARS = 3,
    ENGINE_TAG_GEAR = 4,
    ENGINE_TAG_GEAR_DIFFERENTIAL = 5,
    ENGINE_TAG_MASS = 6,
    ENGINE_TAG_WHEELS = 7,
    ENGINE_TAG_DRAG = 8,
    ENGINE_TAG_ENGINE = 9,
    ENGINE_TAG_ENGINE_POWER = 10,
    ENGINE_TAG_SHIFTING = 11,
    ENGINE_TAG_BRAKES = 12,
}

/// SAX handler used to parse engine parameters
/// C++ class `VehicleEngineHandler` (microsim/engine/VehicleEngineHandler.h)
#[repr(C)]
pub struct VehicleEngineHandler {
    /// C++ base class `GenericSAXHandler`
    pub base: GenericSAXHandler,
    pub vehicleToLoad: String,
    pub skip: bool,
    pub currentGear: i32,
    pub engineParameters: EngineParameters,
    pub gearRatios: Vec<f64>,
}

impl std::ops::Deref for VehicleEngineHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for VehicleEngineHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

// C++ static members of VehicleEngineHandler (port as module-level statics, see AGENTS.md):
//   static SequentialStringBijection::Entry[] VehicleEngineHandler::engineTags;
//   static SequentialStringBijection::Entry[] VehicleEngineHandler::engineAttrs;

const MAX_POLY_DEGREE: i32 = 9; // EngineParameters.h #define
const ENGINE_TAG_NOTHING: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_NOTHING as i32;
const ENGINE_TAG_VEHICLES: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_VEHICLES as i32;
const ENGINE_TAG_VEHICLE: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_VEHICLE as i32;
const ENGINE_TAG_GEARS: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_GEARS as i32;
const ENGINE_TAG_GEAR: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_GEAR as i32;
const ENGINE_TAG_GEAR_DIFFERENTIAL: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_GEAR_DIFFERENTIAL as i32;
const ENGINE_TAG_MASS: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_MASS as i32;
const ENGINE_TAG_WHEELS: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_WHEELS as i32;
const ENGINE_TAG_DRAG: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_DRAG as i32;
const ENGINE_TAG_ENGINE: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_ENGINE as i32;
const ENGINE_TAG_ENGINE_POWER: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_ENGINE_POWER as i32;
const ENGINE_TAG_SHIFTING: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_SHIFTING as i32;
const ENGINE_TAG_BRAKES: i32 = VehicleEngineHandler_EngineXMLTag::ENGINE_TAG_BRAKES as i32;
const ENGINE_ATTR_NOTHING: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_NOTHING as i32;
const ENGINE_ATTR_ID: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ID as i32;
const ENGINE_ATTR_GEAR_N: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_GEAR_N as i32;
const ENGINE_ATTR_GEAR_RATIO: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_GEAR_RATIO as i32;
const ENGINE_ATTR_MASS: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_MASS as i32;
const ENGINE_ATTR_MASS_FACTOR: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_MASS_FACTOR as i32;
const ENGINE_ATTR_WHEELS_DIAMETER: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_WHEELS_DIAMETER as i32;
const ENGINE_ATTR_WHEELS_FRICTION: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_WHEELS_FRICTION as i32;
const ENGINE_ATTR_WHEELS_CR1: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_WHEELS_CR1 as i32;
const ENGINE_ATTR_WHEELS_CR2: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_WHEELS_CR2 as i32;
const ENGINE_ATTR_DRAG_CAIR: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_DRAG_CAIR as i32;
const ENGINE_ATTR_DRAG_SECTION: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_DRAG_SECTION as i32;
const ENGINE_ATTR_ENGINE_TYPE: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_TYPE as i32;
const ENGINE_ATTR_ENGINE_EFFICIENCY: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_EFFICIENCY as i32;
const ENGINE_ATTR_ENGINE_CYLINDERS: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_CYLINDERS as i32;
const ENGINE_ATTR_ENGINE_MINRPM: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_MINRPM as i32;
const ENGINE_ATTR_ENGINE_MAXRPM: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_MAXRPM as i32;
const ENGINE_ATTR_ENGINE_TAU_EX: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_TAU_EX as i32;
const ENGINE_ATTR_ENGINE_TAU_BURN: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_TAU_BURN as i32;
const ENGINE_ATTR_ENGINE_POWER_RPM: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_POWER_RPM as i32;
const ENGINE_ATTR_ENGINE_POWER_HP: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_POWER_HP as i32;
const ENGINE_ATTR_ENGINE_POWER_KW: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_POWER_KW as i32;
const ENGINE_ATTR_ENGINE_POWER_SLOPE: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_POWER_SLOPE as i32;
const ENGINE_ATTR_ENGINE_POWER_INTERCEPT: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_ENGINE_POWER_INTERCEPT as i32;
const ENGINE_ATTR_SHIFTING_RPM: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_SHIFTING_RPM as i32;
const ENGINE_ATTR_SHIFTING_DELTARPM: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_SHIFTING_DELTARPM as i32;
const ENGINE_ATTR_BRAKES_TAU: i32 = VehicleEngineHandler_EngineXMLAttr::ENGINE_ATTR_BRAKES_TAU as i32;

// SequentialStringBijection tables in the C++ array order, terminated by the empty key
static ENGINE_TAGS: &[(&str, i32)] = &[
    ("vehicles", ENGINE_TAG_VEHICLES),
    ("vehicle", ENGINE_TAG_VEHICLE),
    ("gears", ENGINE_TAG_GEARS),
    ("gear", ENGINE_TAG_GEAR),
    ("differential", ENGINE_TAG_GEAR_DIFFERENTIAL),
    ("mass", ENGINE_TAG_MASS),
    ("wheels", ENGINE_TAG_WHEELS),
    ("drag", ENGINE_TAG_DRAG),
    ("engine", ENGINE_TAG_ENGINE),
    ("power", ENGINE_TAG_ENGINE_POWER),
    ("shifting", ENGINE_TAG_SHIFTING),
    ("brakes", ENGINE_TAG_BRAKES),
    ("", ENGINE_TAG_NOTHING),
];
static ENGINE_ATTRS: &[(&str, i32)] = &[
    ("id", ENGINE_ATTR_ID),
    ("n", ENGINE_ATTR_GEAR_N),
    ("ratio", ENGINE_ATTR_GEAR_RATIO),
    ("mass", ENGINE_ATTR_MASS),
    ("massFactor", ENGINE_ATTR_MASS_FACTOR),
    ("diameter", ENGINE_ATTR_WHEELS_DIAMETER),
    ("friction", ENGINE_ATTR_WHEELS_FRICTION),
    ("cr1", ENGINE_ATTR_WHEELS_CR1),
    ("cr2", ENGINE_ATTR_WHEELS_CR2),
    ("cAir", ENGINE_ATTR_DRAG_CAIR),
    ("section", ENGINE_ATTR_DRAG_SECTION),
    ("type", ENGINE_ATTR_ENGINE_TYPE),
    ("efficiency", ENGINE_ATTR_ENGINE_EFFICIENCY),
    ("cylinders", ENGINE_ATTR_ENGINE_CYLINDERS),
    ("minRpm", ENGINE_ATTR_ENGINE_MINRPM),
    ("maxRpm", ENGINE_ATTR_ENGINE_MAXRPM),
    ("tauEx", ENGINE_ATTR_ENGINE_TAU_EX),
    ("tauBurn", ENGINE_ATTR_ENGINE_TAU_BURN),
    ("rpm", ENGINE_ATTR_ENGINE_POWER_RPM),
    ("hp", ENGINE_ATTR_ENGINE_POWER_HP),
    ("kw", ENGINE_ATTR_ENGINE_POWER_KW),
    ("slope", ENGINE_ATTR_ENGINE_POWER_SLOPE),
    ("intercept", ENGINE_ATTR_ENGINE_POWER_INTERCEPT),
    ("rpm", ENGINE_ATTR_SHIFTING_RPM),
    ("deltaRpm", ENGINE_ATTR_SHIFTING_DELTARPM),
    ("tau", ENGINE_ATTR_BRAKES_TAU),
    ("", ENGINE_ATTR_NOTHING),
];

impl VehicleEngineHandler {
    /// C++: `void VehicleEngineHandler::VehicleEngineHandler(const std::string &)`
    /// Constructor
    // VehicleEngineHandler::VehicleEngineHandler(const std::string& toLoad)
    //     : GenericSAXHandler(engineTags, ENGINE_TAG_NOTHING, engineAttrs, ENGINE_ATTR_NOTHING, "vehicles"),
    //       skip(false), currentGear(1) {
    //     vehicleToLoad = toLoad;
    // }
    #[allow(unused_variables)]
    pub fn new(toLoad: &str) -> Self {
        VehicleEngineHandler {
            base: GenericSAXHandler::new(ENGINE_TAGS, ENGINE_TAG_NOTHING, ENGINE_ATTRS, ENGINE_ATTR_NOTHING, "", "vehicles"),
            vehicleToLoad: toLoad.to_string(),
            skip: false,
            currentGear: 1,
            engineParameters: EngineParameters::new(),
            gearRatios: Vec::new(),
        }
    }

    /// C++: `void VehicleEngineHandler::~VehicleEngineHandler() noexcept` [virtual]
    /// Destructor
    // VehicleEngineHandler::~VehicleEngineHandler() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty: nothing to release.
    }

    /// C++: `const EngineParameters & VehicleEngineHandler::getEngineParameters()`
    // const EngineParameters& getEngineParameters() {
    //         return engineParameters;
    //     }
    #[allow(unused_variables)]
    pub fn get_engine_parameters(&mut self) -> &EngineParameters {
        &self.engineParameters
    }

    /// C++: `void VehicleEngineHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // VehicleEngineHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     switch (element) {
    //         case ENGINE_TAG_VEHICLE:
    //             skip = attrs.getString(ENGINE_ATTR_ID) != vehicleToLoad;
    //             if (!skip) {
    //                 engineParameters.id = vehicleToLoad;
    //             }
    //             break;
    //         case ENGINE_TAG_GEAR:
    //             if (!skip) {
    //                 int number = attrs.getInt(ENGINE_ATTR_GEAR_N);
    //                 if (number != currentGear) {
    //                     //fatal
    //                     std::stringstream ss;
    //                     ss << "Invalid gear number " << number << ". Please check that gears are inserted in order";
    //                     throw ProcessError(ss.str());
    //                 }
    //                 gearRatios.push_back(attrs.getFloat(ENGINE_ATTR_GEAR_RATIO));
    //                 currentGear++;
    //             }
    //             break;
    //         case ENGINE_TAG_GEAR_DIFFERENTIAL:
    //             if (!skip) {
    //                 // Load final drive ratio
    //                 engineParameters.differentialRatio = attrs.getFloat(ENGINE_ATTR_GEAR_RATIO);
    //             }
    //             break;
    //         case ENGINE_TAG_MASS:
    //             if (!skip) {
    //                 // Loads mass information, i.e., mass in kg and mass factor which takes into account rotational parts of the engine
    //                 engineParameters.mass_kg = attrs.getFloat(ENGINE_ATTR_MASS);
    //                 engineParameters.massFactor = attrs.getFloat(ENGINE_ATTR_MASS_FACTOR);
    //             }
    //             break;
    //         case ENGINE_TAG_DRAG:
    //             if (!skip) {
    //                 // Load air drag related data such as drag coefficient and maximum vehicle section
    //                 engineParameters.cAir = attrs.getFloat(ENGINE_ATTR_DRAG_CAIR);
    //                 engineParameters.a_m2 = attrs.getFloat(ENGINE_ATTR_DRAG_SECTION);
    //             }
    //             break;
    //         case ENGINE_TAG_WHEELS:
    //             if (!skip) {
    //                 // Load data about vehicle's wheels, such as diameter and friction coefficient
    //                 engineParameters.wheelDiameter_m = attrs.getFloat(ENGINE_ATTR_WHEELS_DIAMETER);
    //                 engineParameters.tiresFrictionCoefficient = attrs.getFloat(ENGINE_ATTR_WHEELS_FRICTION);
    //                 engineParameters.cr1 = attrs.getFloat(ENGINE_ATTR_WHEELS_CR1);
    //                 engineParameters.cr2 = attrs.getFloat(ENGINE_ATTR_WHEELS_CR2);
    //             }
    //             break;
    //         case ENGINE_TAG_ENGINE:
    //             if (!skip) {
    //                 // Load data about the engine, such as efficiency factor and cylinders
    //                 engineParameters.engineEfficiency = attrs.getFloat(ENGINE_ATTR_ENGINE_EFFICIENCY);
    //                 engineParameters.cylinders = attrs.getInt(ENGINE_ATTR_ENGINE_CYLINDERS);
    //                 engineParameters.minRpm = attrs.getInt(ENGINE_ATTR_ENGINE_MINRPM);
    //                 engineParameters.maxRpm = attrs.getInt(ENGINE_ATTR_ENGINE_MAXRPM);
    //                 engineParameters.tauEx_s = attrs.getFloat(ENGINE_ATTR_ENGINE_TAU_EX);
    //                 engineParameters.fixedTauBurn = attrs.hasAttribute(ENGINE_ATTR_ENGINE_TAU_BURN);
    //                 if (engineParameters.fixedTauBurn) {
    //                     engineParameters.tauBurn_s = attrs.getFloat(ENGINE_ATTR_ENGINE_TAU_BURN);
    //                 }
    //                 engineParameters.maxRpm = attrs.getInt(ENGINE_ATTR_ENGINE_MAXRPM);
    //                 std::string mapType = attrs.getString(ENGINE_ATTR_ENGINE_TYPE);
    //                 if (mapType != "poly") {
    //                     throw ProcessError(TL("Invalid engine map type. Only \"poly\" is supported for now"));
    //                 }
    //             }
    //             break;
    //         case ENGINE_TAG_ENGINE_POWER:
    //             if (!skip) {
    //                 //check that the degree is within the maximum supported
    //                 if (attrs.hasAttribute("x" + toString(MAX_POLY_DEGREE))) {
    //                     std::stringstream ss;
    //                     ss << "Maximum degree for the engine polynomial is " << MAX_POLY_DEGREE << ". Please check your model's data";
    //                     throw ProcessError(ss.str());
    //                 }
    //                 //parse all polynomial coefficients
    //                 for (int i = 0; i < MAX_POLY_DEGREE; i++) {
    //                     const std::string param = "x" + toString(i);
    //                     if (attrs.hasAttribute(param)) {
    //                         engineParameters.engineMapping.x[i] = attrs.getFloat(param);
    //                     } else {
    //                         //save the actual degree
    //                         engineParameters.engineMapping.degree = i;
    //                         break;
    //                     }
    //                 }
    //             }
    //             break;
    //         case ENGINE_TAG_SHIFTING:
    //             if (!skip) {
    //                 engineParameters.shiftingRule.rpm = attrs.getFloat(ENGINE_ATTR_SHIFTING_RPM);
    //                 engineParameters.shiftingRule.deltaRpm = attrs.getFloat(ENGINE_ATTR_SHIFTING_DELTARPM);
    //             }
    //             break;
    //         case ENGINE_TAG_BRAKES:
    //             if (!skip) {
    //                 engineParameters.brakesTau_s = attrs.getFloat(ENGINE_ATTR_BRAKES_TAU);
    //             }
    //             break;
    //         case ENGINE_TAG_VEHICLES:
    //         case ENGINE_TAG_GEARS:
    //             break;
    //         default:
    //             WRITE_WARNINGF(TL("Unknown tag '%' while parsing."), toString(element));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        match element {
            ENGINE_TAG_VEHICLE => {
                self.skip = attrs.get_string(ENGINE_ATTR_ID) != self.vehicleToLoad;
                if !self.skip {
                    self.engineParameters.id = self.vehicleToLoad.clone();
                }
            }
            ENGINE_TAG_GEAR => {
                if !self.skip {
                    let number = attrs.get_int(ENGINE_ATTR_GEAR_N)?;
                    if number != self.currentGear {
                        // fatal
                        return Err(ProcessError::new(format!("Invalid gear number {number}. Please check that gears are inserted in order")));
                    }
                    self.gearRatios.push(attrs.get_float(ENGINE_ATTR_GEAR_RATIO)?);
                    self.currentGear += 1;
                }
            }
            ENGINE_TAG_GEAR_DIFFERENTIAL => {
                if !self.skip {
                    // Load final drive ratio
                    self.engineParameters.differentialRatio = attrs.get_float(ENGINE_ATTR_GEAR_RATIO)?;
                }
            }
            ENGINE_TAG_MASS => {
                if !self.skip {
                    // Loads mass information, i.e., mass in kg and mass factor which takes into account rotational parts of the engine
                    self.engineParameters.mass_kg = attrs.get_float(ENGINE_ATTR_MASS)?;
                    self.engineParameters.massFactor = attrs.get_float(ENGINE_ATTR_MASS_FACTOR)?;
                }
            }
            ENGINE_TAG_DRAG => {
                if !self.skip {
                    // Load air drag related data such as drag coefficient and maximum vehicle section
                    self.engineParameters.cAir = attrs.get_float(ENGINE_ATTR_DRAG_CAIR)?;
                    self.engineParameters.a_m2 = attrs.get_float(ENGINE_ATTR_DRAG_SECTION)?;
                }
            }
            ENGINE_TAG_WHEELS => {
                if !self.skip {
                    // Load data about vehicle's wheels, such as diameter and friction coefficient
                    self.engineParameters.wheelDiameter_m = attrs.get_float(ENGINE_ATTR_WHEELS_DIAMETER)?;
                    self.engineParameters.tiresFrictionCoefficient = attrs.get_float(ENGINE_ATTR_WHEELS_FRICTION)?;
                    self.engineParameters.cr1 = attrs.get_float(ENGINE_ATTR_WHEELS_CR1)?;
                    self.engineParameters.cr2 = attrs.get_float(ENGINE_ATTR_WHEELS_CR2)?;
                }
            }
            ENGINE_TAG_ENGINE => {
                if !self.skip {
                    // Load data about the engine, such as efficiency factor and cylinders
                    self.engineParameters.engineEfficiency = attrs.get_float(ENGINE_ATTR_ENGINE_EFFICIENCY)?;
                    self.engineParameters.cylinders = attrs.get_int(ENGINE_ATTR_ENGINE_CYLINDERS)?;
                    self.engineParameters.minRpm = attrs.get_int(ENGINE_ATTR_ENGINE_MINRPM)? as f64;
                    self.engineParameters.maxRpm = attrs.get_int(ENGINE_ATTR_ENGINE_MAXRPM)? as f64;
                    self.engineParameters.tauEx_s = attrs.get_float(ENGINE_ATTR_ENGINE_TAU_EX)?;
                    self.engineParameters.fixedTauBurn = attrs.has_attribute(ENGINE_ATTR_ENGINE_TAU_BURN);
                    if self.engineParameters.fixedTauBurn {
                        self.engineParameters.tauBurn_s = attrs.get_float(ENGINE_ATTR_ENGINE_TAU_BURN)?;
                    }
                    self.engineParameters.maxRpm = attrs.get_int(ENGINE_ATTR_ENGINE_MAXRPM)? as f64;
                    let mapType = attrs.get_string(ENGINE_ATTR_ENGINE_TYPE);
                    if mapType != "poly" {
                        return Err(ProcessError::new("Invalid engine map type. Only \"poly\" is supported for now"));
                    }
                }
            }
            ENGINE_TAG_ENGINE_POWER => {
                if !self.skip {
                    //check that the degree is within the maximum supported
                    if attrs.has_attribute_name(&format!("x{MAX_POLY_DEGREE}")) {
                        return Err(ProcessError::new(format!("Maximum degree for the engine polynomial is {MAX_POLY_DEGREE}. Please check your model's data")));
                    }
                    //parse all polynomial coefficients
                    for i in 0..MAX_POLY_DEGREE {
                        let param = format!("x{i}");
                        if attrs.has_attribute_name(&param) {
                            // shortcut: EngineParameters_PolynomialEngineModelRpmToHp.x is Unmapped (double[9]) in engine_parameters.rs, so coefficients cannot be stored yet, upgrade when that field is typed as [f64; 9]
                            return Err(ProcessError::new(format!("Engine polynomial coefficient {param} cannot be loaded yet (not ported)")));
                        } else {
                            //save the actual degree
                            self.engineParameters.engineMapping.degree = i;
                            break;
                        }
                    }
                }
            }
            ENGINE_TAG_SHIFTING => {
                if !self.skip {
                    self.engineParameters.shiftingRule.rpm = attrs.get_float(ENGINE_ATTR_SHIFTING_RPM)?;
                    self.engineParameters.shiftingRule.deltaRpm = attrs.get_float(ENGINE_ATTR_SHIFTING_DELTARPM)?;
                }
            }
            ENGINE_TAG_BRAKES => {
                if !self.skip {
                    self.engineParameters.brakesTau_s = attrs.get_float(ENGINE_ATTR_BRAKES_TAU)?;
                }
            }
            ENGINE_TAG_VEHICLES | ENGINE_TAG_GEARS => {}
            _ => {
                utils::write_warningf!("Unknown tag '%' while parsing.", element);
            }
        }
        Ok(())
    }

    /// C++: `void VehicleEngineHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // VehicleEngineHandler::myEndElement(int element) {
    //     switch (element) {
    //         case ENGINE_TAG_GEARS:
    //             delete[] engineParameters.gearRatios;
    //             engineParameters.gearRatios = new double[gearRatios.size()];
    //             for (int i = 0; i < (int)gearRatios.size(); i++) {
    //                 engineParameters.gearRatios[i] = gearRatios[i];
    //             }
    //             engineParameters.nGears = (int)gearRatios.size();
    //             break;
    //         case ENGINE_TAG_VEHICLES:
    //             engineParameters.computeCoefficients();
    //             break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            ENGINE_TAG_GEARS => {
                // shortcut: the old gearRatios buffer is leaked instead of freed (delete[]), Ptr<f64> owns nothing, fine while engines are loaded once per process
                let leaked: &mut [f64] = Box::leak(self.gearRatios.clone().into_boxed_slice());
                self.engineParameters.gearRatios = Ptr::from_raw(leaked.as_mut_ptr());
                self.engineParameters.nGears = self.gearRatios.len() as i32;
            }
            ENGINE_TAG_VEHICLES => {
                self.engineParameters.compute_coefficients();
            }
            _ => {}
        }
    }

    /// C++: `void VehicleEngineHandler::loadEngineModelData(const xercesc_3_2::Attributes &)` [protected]
    /// Load the mapping between engine rpm and output power in terms of linear function, i.e., slope and intercept
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn load_engine_model_data(&mut self, attrs: &Unmapped /* xercesc_3_2::Attributes */) {
        // C++ declares loadEngineModelData but VehicleEngineHandler.cpp has no definition, so there is nothing to port.
    }

    /// C++: `void VehicleEngineHandler::VehicleEngineHandler(const VehicleEngineHandler &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &VehicleEngineHandler) -> Self {
        panic!("VehicleEngineHandler is not copyable: the copy constructor is invalidated in C++")
    }

    /// C++: `const VehicleEngineHandler & VehicleEngineHandler::operator=(const VehicleEngineHandler &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &VehicleEngineHandler) -> &VehicleEngineHandler {
        panic!("VehicleEngineHandler is not assignable: operator= is invalidated in C++")
    }

}

impl SaxHandler for VehicleEngineHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), ProcessError> {
        VehicleEngineHandler::my_start_element(self, element, attrs)
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), ProcessError> {
        VehicleEngineHandler::my_end_element(self, element);
        Ok(())
    }
}
