//! Port of marouter/ROMARouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::prelude::*;

/// Parser and container for routes during their loading
/// C++ class `ROMARouteHandler` (marouter/ROMARouteHandler.h)
#[repr(C)]
pub struct ROMARouteHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// The matrix to fill
    pub myMatrix: Ptr<ODMatrix>,
    /// The keys for reading taz
    pub myTazParamKeys: Vec<String>,
    /// The current vehicle parameters
    pub myVehicleParameter: Ptr<SUMOVehicleParameter>,
    /// whether to ignore attributes fromTaz, toTaz
    pub myIgnoreTaz: bool,
    /// scale for loading vehicles
    pub myScale: f64,
    /// number of parsed vehicles
    pub myNumLoaded: i64,
}

impl std::ops::Deref for ROMARouteHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for ROMARouteHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

impl ROMARouteHandler {
    /// C++: `void ROMARouteHandler::ROMARouteHandler(ODMatrix &)`
    /// standard constructor
    // ROMARouteHandler::ROMARouteHandler(ODMatrix& matrix) :
    //     SUMOSAXHandler(""), myMatrix(matrix),
    //     myIgnoreTaz(OptionsCont::getOptions().getBool("ignore-taz")),
    //     myScale(OptionsCont::getOptions().getFloat("scale")),
    //     myNumLoaded(0) {
    //     if (OptionsCont::getOptions().isSet("taz-param")) {
    //         myTazParamKeys = OptionsCont::getOptions().getStringVector("taz-param");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(matrix: &mut ODMatrix) -> Self {
        let mut handler = ROMARouteHandler {
            base: utils::xml::sumo_sax_handler::new("", ""),
            myMatrix: Ptr::from_mut(matrix),
            myTazParamKeys: Vec::new(),
            myVehicleParameter: Ptr::null(),
            myIgnoreTaz: utils::options::options_cont::get_bool("ignore-taz"),
            myScale: utils::options::options_cont::get_float("scale"),
            myNumLoaded: 0,
        };
        if utils::options::options_cont::is_set("taz-param") {
            handler.myTazParamKeys = utils::options::options_cont::get_string_vector("taz-param");
        }
        handler
    }

    /// C++: `void ROMARouteHandler::~ROMARouteHandler() noexcept` [virtual]
    /// standard destructor
    // ROMARouteHandler::~ROMARouteHandler() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty; members are destroyed implicitly
    }

    /// C++: `void ROMARouteHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // ROMARouteHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     if (element == SUMO_TAG_TRIP || element == SUMO_TAG_VEHICLE || element == SUMO_TAG_FLOW) {
    //         myVehicleParameter = (element == SUMO_TAG_FLOW
    //                               ? SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_FLOW, attrs, true, true, 0, TIME2STEPS(3600))
    //                               : SUMOVehicleParserHelper::parseVehicleAttributes(element, attrs, true));
    //         if (!myVehicleParameter->wasSet(VEHPARS_FROM_TAZ_SET) || myIgnoreTaz) {
    //             if (attrs.hasAttribute(SUMO_ATTR_FROM)) {
    //                 myVehicleParameter->fromTaz = attrs.getString(SUMO_ATTR_FROM);
    //             } else if (attrs.hasAttribute(SUMO_ATTR_FROM_JUNCTION)) {
    //                 myVehicleParameter->fromTaz = attrs.getString(SUMO_ATTR_FROM_JUNCTION) + "-source";
    //             }
    //
    //         }
    //         if (!myVehicleParameter->wasSet(VEHPARS_TO_TAZ_SET) || myIgnoreTaz) {
    //             if (attrs.hasAttribute(SUMO_ATTR_TO)) {
    //                 myVehicleParameter->toTaz = attrs.getString(SUMO_ATTR_TO);
    //             } else if (attrs.hasAttribute(SUMO_ATTR_TO_JUNCTION)) {
    //                 myVehicleParameter->toTaz = attrs.getString(SUMO_ATTR_TO_JUNCTION) + "-sink";
    //             }
    //         }
    //     } else if (element == SUMO_TAG_PARAM && !myTazParamKeys.empty()) {
    //         if (attrs.getString(SUMO_ATTR_KEY) == myTazParamKeys[0]) {
    //             myVehicleParameter->fromTaz = attrs.getString(SUMO_ATTR_VALUE);
    //             myVehicleParameter->parametersSet |= VEHPARS_FROM_TAZ_SET;
    //         }
    //         if (myTazParamKeys.size() > 1 && attrs.getString(SUMO_ATTR_KEY) == myTazParamKeys[1]) {
    //             myVehicleParameter->toTaz = attrs.getString(SUMO_ATTR_VALUE);
    //             myVehicleParameter->parametersSet |= VEHPARS_TO_TAZ_SET;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        // mirrors the private constants in utils/vehicle/sumo_vehicle_parameter.rs
        const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
        const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
        if element == SUMO_TAG_TRIP || element == SUMO_TAG_VEHICLE || element == SUMO_TAG_FLOW {
            self.myVehicleParameter = if element == SUMO_TAG_FLOW {
                crate::utils::vehicle::sumo_vehicle_parser_helper::SUMOVehicleParserHelper::parse_flow_attributes(SUMO_TAG_FLOW, attrs, true, true, 0, utils::common::time::time2steps(3600.0), false)
            } else {
                crate::utils::vehicle::sumo_vehicle_parser_helper::SUMOVehicleParserHelper::parse_vehicle_attributes(element, attrs, true, false, false, false)
            };
            let vp = self.myVehicleParameter;
            if !vp.was_set(VEHPARS_FROM_TAZ_SET) || self.myIgnoreTaz {
                if attrs.has_attribute(SUMO_ATTR_FROM) {
                    vp.m().fromTaz = attrs.get_string(SUMO_ATTR_FROM);
                } else if attrs.has_attribute(SUMO_ATTR_FROM_JUNCTION) {
                    vp.m().fromTaz = attrs.get_string(SUMO_ATTR_FROM_JUNCTION) + "-source";
                }
            }
            if !vp.was_set(VEHPARS_TO_TAZ_SET) || self.myIgnoreTaz {
                if attrs.has_attribute(SUMO_ATTR_TO) {
                    vp.m().toTaz = attrs.get_string(SUMO_ATTR_TO);
                } else if attrs.has_attribute(SUMO_ATTR_TO_JUNCTION) {
                    vp.m().toTaz = attrs.get_string(SUMO_ATTR_TO_JUNCTION) + "-sink";
                }
            }
        } else if element == SUMO_TAG_PARAM && !self.myTazParamKeys.is_empty() {
            let vp = self.myVehicleParameter;
            if attrs.get_string(SUMO_ATTR_KEY) == self.myTazParamKeys[0] {
                vp.m().fromTaz = attrs.get_string(SUMO_ATTR_VALUE);
                vp.m().parametersSet |= VEHPARS_FROM_TAZ_SET;
            }
            if self.myTazParamKeys.len() > 1 && attrs.get_string(SUMO_ATTR_KEY) == self.myTazParamKeys[1] {
                vp.m().toTaz = attrs.get_string(SUMO_ATTR_VALUE);
                vp.m().parametersSet |= VEHPARS_TO_TAZ_SET;
            }
        }
    }

    /// C++: `void ROMARouteHandler::myEndElement(int)` [virtual, protected]
    // void
    // ROMARouteHandler::myEndElement(int element) {
    //     if (element == SUMO_TAG_TRIP || element == SUMO_TAG_VEHICLE || element == SUMO_TAG_FLOW) {
    //         if (myVehicleParameter->fromTaz == "" || myVehicleParameter->toTaz == "") {
    //             WRITE_WARNINGF(TL("No origin or no destination given, ignoring '%'!"), myVehicleParameter->id);
    //         } else {
    //             int quota = 1;
    //             SUMOTime departOffset = 0;
    //             if (element == SUMO_TAG_FLOW) {
    //                 long long int flowSize = 1;
    //                 double flowDur = STEPS2TIME(myVehicleParameter->repetitionEnd - myVehicleParameter->depart);
    //                 if (myVehicleParameter->repetitionNumber != std::numeric_limits<long long int>::max()) {
    //                     flowSize = myVehicleParameter->repetitionNumber;
    //                 } else if (myVehicleParameter->poissonRate > 0) {
    //                     flowSize = (long long int)(flowDur * myVehicleParameter->poissonRate);
    //                 } else if (myVehicleParameter->repetitionProbability > 0) {
    //                     flowSize = (long long int)(flowDur * myVehicleParameter->repetitionProbability);
    //                 }
    //                 quota = (int)((double)flowSize * myScale + 0.5);
    //                 myNumLoaded += flowSize;
    //                 departOffset = TIME2STEPS(flowDur) / quota;
    //             } else {
    //                 quota = getScalingQuota(myScale, myNumLoaded);
    //                 myNumLoaded += 1;
    //             }
    //             SUMOTime depart = myVehicleParameter->depart;
    //             for (int i = 0; i < quota; i++) {
    //                 SUMOVehicleParameter veh = *myVehicleParameter;
    //                 veh.id = i == 0 ? myVehicleParameter->id : myVehicleParameter->id + "." + toString(i);
    //                 veh.depart = depart;
    //                 myMatrix.add(veh,
    //                              !myVehicleParameter->wasSet(VEHPARS_FROM_TAZ_SET) || myIgnoreTaz,
    //                              !myVehicleParameter->wasSet(VEHPARS_TO_TAZ_SET) || myIgnoreTaz);
    //                 depart += departOffset;
    //             }
    //         }
    //         delete myVehicleParameter;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        const VEHPARS_FROM_TAZ_SET: i64 = 2 << 15;
        const VEHPARS_TO_TAZ_SET: i64 = 2 << 16;
        if element == SUMO_TAG_TRIP || element == SUMO_TAG_VEHICLE || element == SUMO_TAG_FLOW {
            let vp = self.myVehicleParameter;
            if vp.fromTaz == "" || vp.toTaz == "" {
                utils::write_warningf!("No origin or no destination given, ignoring '%'!", vp.id);
            } else {
                let mut quota: i32 = 1;
                let mut departOffset: SUMOTime = 0;
                if element == SUMO_TAG_FLOW {
                    let mut flowSize: i64 = 1;
                    let flowDur = utils::common::time::steps2time(vp.repetitionEnd - vp.depart);
                    if vp.repetitionNumber != i64::MAX {
                        flowSize = vp.repetitionNumber;
                    } else if vp.poissonRate > 0. {
                        flowSize = (flowDur * vp.poissonRate) as i64;
                    } else if vp.repetitionProbability > 0. {
                        flowSize = (flowDur * vp.repetitionProbability) as i64;
                    }
                    quota = ((flowSize as f64) * self.myScale + 0.5) as i32;
                    self.myNumLoaded += flowSize;
                    departOffset = utils::common::time::time2steps(flowDur) / quota as i64;
                } else {
                    quota = utils::common::std_defs::get_scaling_quota(self.myScale, self.myNumLoaded);
                    self.myNumLoaded += 1;
                }
                let mut depart: SUMOTime = vp.depart;
                for i in 0..quota {
                    let mut veh: SUMOVehicleParameter = (*vp).clone();
                    veh.id = if i == 0 { vp.id.clone() } else { format!("{}.{}", vp.id, i) };
                    veh.depart = depart;
                    self.myMatrix.m().add_2(&veh, !vp.was_set(VEHPARS_FROM_TAZ_SET) || self.myIgnoreTaz, !vp.was_set(VEHPARS_TO_TAZ_SET) || self.myIgnoreTaz);
                    depart += departOffset;
                }
            }
            drop(unsafe { Box::from_raw(self.myVehicleParameter.as_raw()) });
        }
    }

    /// C++: `void ROMARouteHandler::ROMARouteHandler(const ROMARouteHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &ROMARouteHandler) -> Self {
        // C++ note: private copy constructor is declared in ROMARouteHandler.h but never defined; calling it is a link error in C++
        panic!("ROMARouteHandler copy constructor is declared but not defined");
    }

    /// C++: `ROMARouteHandler & ROMARouteHandler::operator=(const ROMARouteHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &ROMARouteHandler) -> &mut ROMARouteHandler {
        // C++ declares `ROMARouteHandler& operator=(const ROMARouteHandler&)` as private and undefined, so it can never be called
        panic!("ROMARouteHandler::operator= is declared but not defined");
    }

}


impl utils::xml::generic_sax_handler::SaxHandler for ROMARouteHandler {
    fn handler(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        ROMARouteHandler::my_start_element(self, element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), utils::common::util_exceptions::ProcessError> {
        ROMARouteHandler::my_end_element(self, element);
        Ok(())
    }
}
