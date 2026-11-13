//! Port of utils/vehicle/SUMORouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use utils::xml::generic_sax_handler::GenericSAXHandler;
use crate::utils::vehicle::sumo_vehicle_parser_helper::SUMOVehicleParserHelper;
use crate::microsim::ms_base_vehicle::VEHPARS_FORCE_REROUTE;
use utils::common::util_exceptions::ProcessError;
use utils::common::msg_handler::write_error;
use utils::common::std_defs::INVALID_DOUBLE;
use crate::prelude::*;

/// C++ enum `SUMORouteHandler::StopPos`
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i64)]
pub enum SUMORouteHandler_StopPos {
    STOPPOS_VALID = 0,
    STOPPOS_INVALID_STARTPOS = 1,
    STOPPOS_INVALID_ENDPOS = 2,
    STOPPOS_INVALID_LANELENGTH = 3,
}

/// Parser for routes during their loading
/// C++ class `SUMORouteHandler` (utils/vehicle/SUMORouteHandler.h)
#[repr(C)]
pub struct SUMORouteHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: GenericSAXHandler,
    /// flag to enable or disable hard fails
    pub myHardFail: bool,
    /// Parameter of the current vehicle, trip, person, container or flow
    pub myVehicleParameter: Ptr<SUMOVehicleParameter>,
    /// The stack of currently parsed parameterised objects
    pub myParamStack: Vec<Ptr<Parameterised>>,
    /// The insertion time of the vehicle read last
    pub myLastDepart: SUMOTime,
    /// The id of the current route
    pub myActiveRouteID: String,
    /// The id of the route the current route references to
    pub myActiveRouteRefID: String,
    /// The probability of the current route
    pub myActiveRouteProbability: f64,
    /// The currently parsed route's color
    pub myActiveRouteColor: Ptr<RGBColor>,
    /// The currently parsed route costs
    pub myCurrentCosts: f64,
    /// List of the stops on the parsed route
    pub myActiveRouteStops: StopParVector,
    /// The currently parsed vehicle type
    pub myCurrentVType: Ptr<SUMOVTypeParameter>,
    /// if the currently parsed vType is only a reference to an existing type, store the refId
    pub myCurrentVTypeRef: String,
    /// if the currently parsed vType is only a reference, store the probability here
    pub myCurrentVTypeProbability: f64,
    /// The default value for flow begins
    pub myBeginDefault: SUMOTime,
    /// The default value for flow ends
    pub myEndDefault: SUMOTime,
    /// the first read departure time
    pub myFirstDepart: SUMOTime,
    /// where stop edges can be inserted into the current route (-1 means no insertion)
    pub myInsertStopEdgesAt: i32,
    /// hierarchy of elements being parsed
    pub myElementStack: Vec<i32>,
    /// whether references to internal routes are allowed in this context
    pub myAllowInternalRoutes: bool,
    /// IDs of skipped vehicles to suppress errors for the triggered transportables within
    pub mySkippedVehicles: BTreeSet<String>,
}

impl std::ops::Deref for SUMORouteHandler {
    type Target = GenericSAXHandler;
    fn deref(&self) -> &GenericSAXHandler {
        &self.base
    }
}
impl std::ops::DerefMut for SUMORouteHandler {
    fn deref_mut(&mut self) -> &mut GenericSAXHandler {
        &mut self.base
    }
}

// C++ static members of SUMORouteHandler (port as module-level statics, see AGENTS.md):
//   static SumoRNG SUMORouteHandler::myParsingRNG;
// C++ `static SumoRNG myParsingRNG("routehandler")`: one leaked box per thread so the Ptr stays valid
thread_local! {
    static MY_PARSING_RNG: Ptr<SumoRNG> = Ptr::from_raw(Box::into_raw(Box::new(SumoRNG::new("routehandler"))));
}

// C++ SUMOVehicleParameter.h STOP_* flags (parametersSet of a stop)
const STOP_START_SET: i32 = 1;
const STOP_END_SET: i32 = 2;
const STOP_DURATION_SET: i32 = 2 << 1;
const STOP_UNTIL_SET: i32 = 2 << 2;
const STOP_EXTENSION_SET: i32 = 2 << 3;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;
const STOP_EXPECTED_SET: i32 = 2 << 6;
const STOP_EXPECTED_CONTAINERS_SET: i32 = 2 << 8;
const STOP_TRIP_ID_SET: i32 = 2 << 9;
const STOP_LINE_SET: i32 = 2 << 10;
const STOP_SPEED_SET: i32 = 2 << 11;
const STOP_SPLIT_SET: i32 = 2 << 12;
const STOP_JOIN_SET: i32 = 2 << 13;
const STOP_ARRIVAL_SET: i32 = 2 << 14;
const STOP_PERMITTED_SET: i32 = 2 << 15;
const STOP_ENDED_SET: i32 = 2 << 16;
const STOP_STARTED_SET: i32 = 2 << 17;
const STOP_POSLAT_SET: i32 = 2 << 18;
const STOP_ONDEMAND_SET: i32 = 2 << 19;
const STOP_JUMP_SET: i32 = 2 << 20;
const STOP_JUMP_UNTIL_SET: i32 = 2 << 21;
const STOP_PRIORITY_SET: i32 = 2 << 22;
const STOP_INDEX_END: i32 = -1;
const STOP_INDEX_FIT: i32 = -2;

/// C++ `toString(SumoXMLTag)`: the canonical name of a tag
fn tag_to_string(tag: SumoXMLTag) -> String {
    utils::xml::sumo_xml_definitions::TAGS.iter().find(|(_, v)| *v == tag).map(|(s, _)| s.to_string()).unwrap_or_default()
}

impl SUMORouteHandler {
    /// C++: `void SUMORouteHandler::SUMORouteHandler(const std::string &, const std::string &, const bool)`
    /// standard constructor
    // SUMORouteHandler::SUMORouteHandler(const std::string& file, const std::string& expectedRoot, const bool hardFail) :
    //     SUMOSAXHandler(file, expectedRoot),
    //     myHardFail(hardFail),
    //     myVehicleParameter(nullptr),
    //     myLastDepart(-1),
    //     myActiveRouteColor(nullptr),
    //     myCurrentCosts(0.),
    //     myCurrentVType(nullptr),
    //     myCurrentVTypeProbability(-1),
    //     myBeginDefault(OptionsCont::getOptions().exists("begin") ? string2time(OptionsCont::getOptions().getString("begin")) : 0),
    //     myEndDefault(OptionsCont::getOptions().exists("end") ? string2time(OptionsCont::getOptions().getString("end")) : -1),
    //     myFirstDepart(-1),
    //     myInsertStopEdgesAt(-1),
    //     myAllowInternalRoutes(false) {
    // }
    #[allow(unused_variables)]
    pub fn new(file: &str, expectedRoot: &str, hardFail: bool) -> Self {
        let begin_default = if utils::options::options_cont::exists("begin") { utils::common::time::string2time(&utils::options::options_cont::get_string("begin")).unwrap_or_else(|e| panic!("{}", e)) } else { 0 };
        let end_default = if utils::options::options_cont::exists("end") { utils::common::time::string2time(&utils::options::options_cont::get_string("end")).unwrap_or_else(|e| panic!("{}", e)) } else { -1 };
        SUMORouteHandler {
            base: utils::xml::sumo_sax_handler::new(file, expectedRoot),
            myHardFail: hardFail,
            myVehicleParameter: Ptr::null(),
            myParamStack: Vec::new(),
            myLastDepart: -1,
            myActiveRouteID: String::new(),
            myActiveRouteRefID: String::new(),
            myActiveRouteProbability: 0.0,
            myActiveRouteColor: Ptr::null(),
            myCurrentCosts: 0.0,
            myActiveRouteStops: Vec::new(),
            myCurrentVType: Ptr::null(),
            myCurrentVTypeRef: String::new(),
            myCurrentVTypeProbability: -1.0,
            myBeginDefault: begin_default,
            myEndDefault: end_default,
            myFirstDepart: -1,
            myInsertStopEdgesAt: -1,
            myElementStack: Vec::new(),
            myAllowInternalRoutes: false,
            mySkippedVehicles: BTreeSet::new(),
        }
    }

    /// C++: `void SUMORouteHandler::~SUMORouteHandler()` [virtual]
    /// standard destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: parser-owned allocations are not freed here (Ptr carries no ownership), so the C++ deletes only drop the pointers
        self.myVehicleParameter = Ptr::null();
        self.myCurrentVType = Ptr::null();
    }

    /// C++: `StopPos SUMORouteHandler::checkStopPos(double &, double &, const double, const double, const bool)` [static]
    /// return
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_stop_pos(startPos: &mut f64, endPos: &mut f64, laneLength: f64, minLength: f64, friendlyPos: bool) -> SUMORouteHandler_StopPos {
        if minLength > laneLength {
            return SUMORouteHandler_StopPos::STOPPOS_INVALID_LANELENGTH;
        }
        if *startPos < 0. {
            *startPos += laneLength;
        }
        if *endPos < 0. {
            *endPos += laneLength;
        }
        if (*endPos < minLength) || (*endPos > laneLength) {
            if !friendlyPos {
                return SUMORouteHandler_StopPos::STOPPOS_INVALID_ENDPOS;
            }
            if *endPos < minLength {
                *endPos = minLength;
            }
            if *endPos > laneLength {
                *endPos = laneLength;
            }
        }
        if (*startPos < 0.) || (*startPos > (*endPos - minLength)) {
            if !friendlyPos {
                return SUMORouteHandler_StopPos::STOPPOS_INVALID_STARTPOS;
            }
            if *startPos < 0. {
                *startPos = 0.;
            }
            if *startPos > (*endPos - minLength) {
                *startPos = *endPos - minLength;
            }
        }
        SUMORouteHandler_StopPos::STOPPOS_VALID
    }

    /// C++: `SUMOTime SUMORouteHandler::getFirstDepart() const` [const]
    /// returns the first departure time that was ever read
    // SUMOTime
    // SUMORouteHandler::getFirstDepart() const {
    //     return myFirstDepart;
    // }
    #[allow(unused_variables)]
    pub fn get_first_depart(&self) -> SUMOTime {
        self.myFirstDepart
    }

    /// C++: `SUMOTime SUMORouteHandler::getLastDepart() const` [const]
    /// Returns the last loaded depart time
    // SUMOTime
    // SUMORouteHandler::getLastDepart() const {
    //     return myLastDepart;
    // }
    #[allow(unused_variables)]
    pub fn get_last_depart(&self) -> SUMOTime {
        self.myLastDepart
    }

    /// C++: `SumoRNG * SUMORouteHandler::getParsingRNG()` [static]
    /// get parsing RNG
    // static SumoRNG* getParsingRNG() {
    //         return &myParsingRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_parsing_rng() -> Ptr<SumoRNG> {
        MY_PARSING_RNG.with(|r| *r)
    }

    /// C++: `void SUMORouteHandler::myStartElement(int, const SUMOSAXAttributes &)` [virtual, protected]
    /// Called on the opening of a tag;
    // void
    // SUMORouteHandler::myStartElement(int element, const SUMOSAXAttributes& attrs) {
    //     myElementStack.push_back(element);
    //     switch (element) {
    //         case SUMO_TAG_VEHICLE:
    //         case SUMO_TAG_PERSON:
    //         case SUMO_TAG_CONTAINER:
    //             // if myVehicleParameter is nullptr this will do nothing
    //             delete myVehicleParameter;
    //             // we set to nullptr to have a consistent state if the parsing fails
    //             myVehicleParameter = nullptr;
    //             myVehicleParameter = SUMOVehicleParserHelper::parseVehicleAttributes(element, attrs, myHardFail, false, false, myAllowInternalRoutes);
    //             myParamStack.push_back(myVehicleParameter);
    //             if (element != SUMO_TAG_VEHICLE) {
    //                 addTransportable(attrs, element == SUMO_TAG_PERSON);
    //             }
    //             break;
    //         case SUMO_TAG_FLOW:
    //             // delete if myVehicleParameter isn't null
    //             if (myVehicleParameter) {
    //                 delete myVehicleParameter;
    //                 myVehicleParameter = nullptr;
    //             }
    //             // parse vehicle parameters
    //             // might be called to parse vehicles from additional file in the
    //             // context of quickReload. In this case, rerouter flows must be ignored
    //             if (myElementStack.size() == 1 || myElementStack[myElementStack.size() - 2] != SUMO_TAG_CALIBRATOR) {
    //                 myVehicleParameter = SUMOVehicleParserHelper::parseFlowAttributes(SUMO_TAG_FLOW, attrs, myHardFail, true, myBeginDefault, myEndDefault, myAllowInternalRoutes);
    //             }
    //             // check if myVehicleParameter was successfully created
    //             if (myVehicleParameter) {
    //                 // check tag
    //                 if (myVehicleParameter->routeid.empty()) {
    //                     // open a route flow (It could be a flow with embedded route)
    //                     openFlow(attrs);
    //                 } else {
    //                     // open a route flow
    //                     openRouteFlow(attrs);
    //                 }
    //                 myParamStack.push_back(myVehicleParameter);
    //             }
    //             break;
    //         case SUMO_TAG_PERSONFLOW:
    //         case SUMO_TAG_CONTAINERFLOW:
    //             // delete if myVehicleParameter isn't null
    //             if (myVehicleParameter) {
    //                 delete myVehicleParameter;
    //                 myVehicleParameter = nullptr;
    //             }
    //             // create a new flow
    //             myVehicleParameter = SUMOVehicleParserHelper::parseFlowAttributes((SumoXMLTag)element, attrs, myHardFail, true, myBeginDefault, myEndDefault, myAllowInternalRoutes);
    //             myParamStack.push_back(myVehicleParameter);
    //             break;
    //         case SUMO_TAG_VTYPE:
    //             // delete if myCurrentVType isn't null
    //             if (myCurrentVType != nullptr) {
    //                 delete myCurrentVType;
    //                 myCurrentVType = nullptr;
    //             }
    //             // create a new vType
    //             if (attrs.hasAttribute(SUMO_ATTR_REFID) && !attrs.hasAttribute(SUMO_ATTR_ID)) {
    //                 bool ok = true;
    //                 myCurrentVTypeRef = attrs.get<std::string>(SUMO_ATTR_REFID, nullptr, ok);
    //                 myCurrentVTypeProbability = attrs.getOpt<double>(SUMO_ATTR_PROB, myCurrentVTypeRef.c_str(), ok, -1);
    //             } else {
    //                 myCurrentVType = SUMOVehicleParserHelper::beginVTypeParsing(attrs, myHardFail, getFileName(), getVTypeParameter(attrs.getStringSecure(SUMO_ATTR_REFID, "")));
    //             }
    //             myParamStack.push_back(myCurrentVType);
    //             break;
    //         case SUMO_TAG_VTYPE_DISTRIBUTION:
    //             openVehicleTypeDistribution(attrs);
    //             break;
    //         case SUMO_TAG_ROUTE:
    //             openRoute(attrs);
    //             break;
    //         case SUMO_TAG_ROUTE_DISTRIBUTION:
    //             openRouteDistribution(attrs);
    //             break;
    //         case SUMO_TAG_STOP:
    //             myParamStack.push_back(addStop(attrs));
    //             break;
    //         case SUMO_TAG_TRIP: {
    //             // delete if myVehicleParameter isn't null
    //             if (myVehicleParameter) {
    //                 delete myVehicleParameter;
    //                 myVehicleParameter = nullptr;
    //             }
    //             // parse vehicle parameters
    //             myVehicleParameter = SUMOVehicleParserHelper::parseVehicleAttributes(element, attrs, myHardFail, false, false, myAllowInternalRoutes);
    //             // check if myVehicleParameter was successfully created
    //             if (myVehicleParameter) {
    //                 myVehicleParameter->parametersSet |= VEHPARS_FORCE_REROUTE;
    //                 myActiveRouteID = "!" + myVehicleParameter->id;
    //                 // open trip
    //                 openTrip(attrs);
    //                 myParamStack.push_back(myVehicleParameter);
    //             }
    //             break;
    //         }
    //         case SUMO_TAG_PERSONTRIP:
    //             addPersonTrip(attrs);
    //             break;
    //         case SUMO_TAG_WALK:
    //             addWalk(attrs);
    //             break;
    //         case SUMO_TAG_INTERVAL: {
    //             bool ok;
    //             myBeginDefault = attrs.getSUMOTimeReporting(SUMO_ATTR_BEGIN, nullptr, ok);
    //             myEndDefault = attrs.getSUMOTimeReporting(SUMO_ATTR_END, nullptr, ok);
    //             break;
    //         }
    //         case SUMO_TAG_RIDE:
    //             addRide(attrs);
    //             break;
    //         case SUMO_TAG_TRANSPORT:
    //             addTransport(attrs);
    //             break;
    //         case SUMO_TAG_TRANSHIP:
    //             addTranship(attrs);
    //             break;
    //         case SUMO_TAG_PARAM:
    //             addParam(attrs);
    //             break;
    //         default:
    //             // parse embedded car following model information
    //             if (myCurrentVType != nullptr) {
    //                 WRITE_WARNINGF(TL("Defining car-following parameters in a nested element is deprecated in vType '%', use attributes instead!"), myCurrentVType->id);
    //                 if (!SUMOVehicleParserHelper::parseCFMParams(myCurrentVType, (SumoXMLTag)element, attrs, true)) {
    //                     if (myHardFail) {
    //                         throw ProcessError(TL("Invalid parsing embedded VType"));
    //                     } else {
    //                         WRITE_ERROR(TL("Invalid parsing embedded VType"));
    //                     }
    //                 }
    //             }
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) {
        self.myElementStack.push(element);
        match element {
            SUMO_TAG_VEHICLE | SUMO_TAG_PERSON | SUMO_TAG_CONTAINER => {
                // if myVehicleParameter is nullptr this will do nothing
                // delete myVehicleParameter (C++): dropped, see drop_
                // we set to nullptr to have a consistent state if the parsing fails
                self.myVehicleParameter = Ptr::null();
                self.myVehicleParameter = SUMOVehicleParserHelper::parse_vehicle_attributes(element, attrs, self.myHardFail, false, false, self.myAllowInternalRoutes);
                self.myParamStack.push(self.myVehicleParameter.cast::<Parameterised>());
                if element != SUMO_TAG_VEHICLE {
                    self.add_transportable(attrs, element == SUMO_TAG_PERSON);
                }
            }
            SUMO_TAG_FLOW => {
                // delete if myVehicleParameter isn't null
                if !self.myVehicleParameter.is_null() {
                    self.myVehicleParameter = Ptr::null();
                }
                // parse vehicle parameters
                // might be called to parse vehicles from additional file in the
                // context of quickReload. In this case, rerouter flows must be ignored
                if self.myElementStack.len() == 1 || self.myElementStack[self.myElementStack.len() - 2] != SUMO_TAG_CALIBRATOR {
                    self.myVehicleParameter = SUMOVehicleParserHelper::parse_flow_attributes(SUMO_TAG_FLOW, attrs, self.myHardFail, true, self.myBeginDefault, self.myEndDefault, self.myAllowInternalRoutes);
                }
                // check if myVehicleParameter was successfully created
                if !self.myVehicleParameter.is_null() {
                    // check tag
                    if self.myVehicleParameter.m().routeid.is_empty() {
                        // open a route flow (It could be a flow with embedded route)
                        self.open_flow(attrs);
                    } else {
                        // open a route flow
                        self.open_route_flow(attrs);
                    }
                    self.myParamStack.push(self.myVehicleParameter.cast::<Parameterised>());
                }
            }
            SUMO_TAG_PERSONFLOW | SUMO_TAG_CONTAINERFLOW => {
                // delete if myVehicleParameter isn't null
                if !self.myVehicleParameter.is_null() {
                    self.myVehicleParameter = Ptr::null();
                }
                // create a new flow
                self.myVehicleParameter = SUMOVehicleParserHelper::parse_flow_attributes(element, attrs, self.myHardFail, true, self.myBeginDefault, self.myEndDefault, self.myAllowInternalRoutes);
                self.myParamStack.push(self.myVehicleParameter.cast::<Parameterised>());
            }
            SUMO_TAG_VTYPE => {
                // delete if myCurrentVType isn't null
                if !self.myCurrentVType.is_null() {
                    self.myCurrentVType = Ptr::null();
                }
                // create a new vType
                if attrs.has_attribute(SUMO_ATTR_REFID) && !attrs.has_attribute(SUMO_ATTR_ID) {
                    let mut ok = true;
                    self.myCurrentVTypeRef = attrs.get::<String>(SUMO_ATTR_REFID, "", &mut ok, true);
                    let vtype_ref = self.myCurrentVTypeRef.clone();
                    self.myCurrentVTypeProbability = attrs.get_opt::<f64>(SUMO_ATTR_PROB, &vtype_ref, &mut ok, -1.0, true);
                } else {
                    let base = self.get_v_type_parameter(&attrs.get_string_secure(SUMO_ATTR_REFID, ""));
                    self.myCurrentVType = SUMOVehicleParserHelper::begin_v_type_parsing(attrs, self.myHardFail, self.base.get_file_name(), base);
                }
                self.myParamStack.push(self.myCurrentVType.cast::<Parameterised>());
            }
            SUMO_TAG_VTYPE_DISTRIBUTION => self.open_vehicle_type_distribution(attrs),
            SUMO_TAG_ROUTE => self.open_route(attrs),
            SUMO_TAG_ROUTE_DISTRIBUTION => self.open_route_distribution(attrs),
            SUMO_TAG_STOP => {
                let stop = self.add_stop(attrs);
                self.myParamStack.push(stop);
            }
            SUMO_TAG_TRIP => {
                // delete if myVehicleParameter isn't null
                if !self.myVehicleParameter.is_null() {
                    self.myVehicleParameter = Ptr::null();
                }
                // parse vehicle parameters
                self.myVehicleParameter = SUMOVehicleParserHelper::parse_vehicle_attributes(element, attrs, self.myHardFail, false, false, self.myAllowInternalRoutes);
                // check if myVehicleParameter was successfully created
                if !self.myVehicleParameter.is_null() {
                    let vp = self.myVehicleParameter;
                    vp.m().parametersSet |= VEHPARS_FORCE_REROUTE;
                    self.myActiveRouteID = format!("!{}", vp.m().id);
                    // open trip
                    self.open_trip(attrs);
                    self.myParamStack.push(vp.cast::<Parameterised>());
                }
            }
            SUMO_TAG_PERSONTRIP => self.add_person_trip(attrs),
            SUMO_TAG_WALK => self.add_walk(attrs),
            SUMO_TAG_INTERVAL => {
                let mut ok = true;
                self.myBeginDefault = attrs.get_sumo_time_reporting(SUMO_ATTR_BEGIN, "", &mut ok, true);
                self.myEndDefault = attrs.get_sumo_time_reporting(SUMO_ATTR_END, "", &mut ok, true);
            }
            SUMO_TAG_RIDE => self.add_ride(attrs),
            SUMO_TAG_TRANSPORT => self.add_transport(attrs),
            SUMO_TAG_TRANSHIP => self.add_tranship(attrs),
            SUMO_TAG_PARAM => self.add_param(attrs),
            _ => {
                // parse embedded car following model information
                if !self.myCurrentVType.is_null() {
                    let id = self.myCurrentVType.id.clone();
                    utils::write_warningf!("Defining car-following parameters in a nested element is deprecated in vType '%', use attributes instead!", id.as_str());
                    if !SUMOVehicleParserHelper::parse_cfm_params(self.myCurrentVType, element, attrs, true) {
                        if self.myHardFail {
                            panic!("{}", ProcessError::Process("Invalid parsing embedded VType".to_string()));
                        } else {
                            write_error("Invalid parsing embedded VType");
                        }
                    }
                }
            }
        }
    }

    /// C++: `void SUMORouteHandler::myEndElement(int)` [virtual, protected]
    /// Called when a closing tag occurs
    // void
    // SUMORouteHandler::myEndElement(int element) {
    //     switch (element) {
    //         case SUMO_TAG_STOP:
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_ROUTE:
    //             closeRoute();
    //             break;
    //         case SUMO_TAG_VTYPE:
    //             closeVType();
    //             if (myCurrentVTypeRef.empty()) {
    //                 delete myCurrentVType;
    //             }
    //             myCurrentVType = nullptr;
    //             myCurrentVTypeRef = "";
    //             myCurrentVTypeProbability = -1;
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_PERSON:
    //             closePerson();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_PERSONFLOW:
    //             closePersonFlow();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_CONTAINER:
    //             closeContainer();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_CONTAINERFLOW:
    //             closeContainerFlow();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             myParamStack.pop_back();
    //             break;
    //         case SUMO_TAG_VEHICLE:
    //             if (myVehicleParameter == nullptr) {
    //                 break;
    //             }
    //             if (myVehicleParameter->repetitionNumber <= 0) {
    //                 closeVehicle();
    //                 delete myVehicleParameter;
    //                 myVehicleParameter = nullptr;
    //                 myParamStack.pop_back();
    //                 break;
    //             }
    //             myVehicleParameter->repetitionNumber++; // for backwards compatibility
    //             // it is a flow, thus no break here
    //             FALLTHROUGH;
    //         case SUMO_TAG_FLOW:
    //             if (myVehicleParameter) {
    //                 closeFlow();
    //                 delete myVehicleParameter;
    //                 myParamStack.pop_back();
    //             }
    //             myVehicleParameter = nullptr;
    //             myInsertStopEdgesAt = -1;
    //             break;
    //         case SUMO_TAG_TRIP:
    //             closeTrip();
    //             delete myVehicleParameter;
    //             myVehicleParameter = nullptr;
    //             myParamStack.pop_back();
    //             myInsertStopEdgesAt = -1;
    //             break;
    //         case SUMO_TAG_VTYPE_DISTRIBUTION:
    //             closeVehicleTypeDistribution();
    //             break;
    //         case SUMO_TAG_ROUTE_DISTRIBUTION:
    //             closeRouteDistribution();
    //             break;
    //         case SUMO_TAG_PERSONTRIP:
    //         case SUMO_TAG_RIDE:
    //         case SUMO_TAG_TRANSPORT:
    //         case SUMO_TAG_TRANSHIP:
    //         case SUMO_TAG_WALK:
    //             if (myParamStack.size() == 2) {
    //                 myParamStack.pop_back();
    //             }
    //             break;
    //         case SUMO_TAG_INTERVAL:
    //             myBeginDefault = string2time(OptionsCont::getOptions().getString("begin"));
    //             myEndDefault = string2time(OptionsCont::getOptions().getString("end"));
    //             break;
    //         default:
    //             break;
    //     }
    //     myElementStack.pop_back();
    // }
    #[allow(unused_variables)]
    pub fn my_end_element(&mut self, element: i32) {
        match element {
            SUMO_TAG_STOP => {
                self.myParamStack.pop();
            }
            SUMO_TAG_ROUTE => {
                self.close_route(false);
            }
            SUMO_TAG_VTYPE => {
                self.close_v_type();
                // delete myCurrentVType only when it is not a reference to an existing type (both paths null the pointer)
                self.myCurrentVType = Ptr::null();
                self.myCurrentVTypeRef = String::new();
                self.myCurrentVTypeProbability = -1.0;
                self.myParamStack.pop();
            }
            SUMO_TAG_PERSON => {
                self.close_person();
                self.myVehicleParameter = Ptr::null();
                self.myParamStack.pop();
            }
            SUMO_TAG_PERSONFLOW => {
                self.close_person_flow();
                self.myVehicleParameter = Ptr::null();
                self.myParamStack.pop();
            }
            SUMO_TAG_CONTAINER => {
                self.close_container();
                self.myVehicleParameter = Ptr::null();
                self.myParamStack.pop();
            }
            SUMO_TAG_CONTAINERFLOW => {
                self.close_container_flow();
                self.myVehicleParameter = Ptr::null();
                self.myParamStack.pop();
            }
            SUMO_TAG_VEHICLE => {
                if self.myVehicleParameter.is_null() {
                    // break
                } else if self.myVehicleParameter.m().repetitionNumber <= 0 {
                    self.close_vehicle();
                    self.myVehicleParameter = Ptr::null();
                    self.myParamStack.pop();
                } else {
                    self.myVehicleParameter.m().repetitionNumber += 1; // for backwards compatibility
                    // it is a flow, thus fall through to the flow ending (C++ FALLTHROUGH)
                    if !self.myVehicleParameter.is_null() {
                        self.close_flow();
                        self.myVehicleParameter = Ptr::null();
                        self.myParamStack.pop();
                    }
                    self.myVehicleParameter = Ptr::null();
                    self.myInsertStopEdgesAt = -1;
                }
            }
            SUMO_TAG_FLOW => {
                if !self.myVehicleParameter.is_null() {
                    self.close_flow();
                    self.myVehicleParameter = Ptr::null();
                    self.myParamStack.pop();
                }
                self.myVehicleParameter = Ptr::null();
                self.myInsertStopEdgesAt = -1;
            }
            SUMO_TAG_TRIP => {
                self.close_trip();
                self.myVehicleParameter = Ptr::null();
                self.myParamStack.pop();
                self.myInsertStopEdgesAt = -1;
            }
            SUMO_TAG_VTYPE_DISTRIBUTION => {
                self.close_vehicle_type_distribution();
            }
            SUMO_TAG_ROUTE_DISTRIBUTION => {
                self.close_route_distribution();
            }
            SUMO_TAG_PERSONTRIP | SUMO_TAG_RIDE | SUMO_TAG_TRANSPORT | SUMO_TAG_TRANSHIP | SUMO_TAG_WALK => {
                if self.myParamStack.len() == 2 {
                    self.myParamStack.pop();
                }
            }
            SUMO_TAG_INTERVAL => {
                self.myBeginDefault = utils::common::time::string2time(&utils::options::options_cont::get_string("begin")).unwrap_or_else(|e| panic!("{}", e));
                self.myEndDefault = utils::common::time::string2time(&utils::options::options_cont::get_string("end")).unwrap_or_else(|e| panic!("{}", e));
            }
            _ => {}
        }
        self.myElementStack.pop();
    }

    /// C++: `void SUMORouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a type distribution for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_vehicle_type_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openVehicleTypeDistribution is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeVehicleTypeDistribution()` [virtual, pure virtual, protected]
    /// closes (ends) the building of a distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_vehicle_type_distribution(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeVehicleTypeDistribution is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::openRoute(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openRoute is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::openFlow(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a flow for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openFlow is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::openRouteFlow(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route flow for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route_flow(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openRouteFlow is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::openTrip(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a trip for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_trip(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openTrip is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeRoute(const bool)` [virtual, pure virtual, protected]
    /// closes (ends) the building of a route. Afterwards no edges may be added to it; this method may throw exceptions when a) the route is empty or b) another route with the same id already exists
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_route(&mut self, mayBeDisconnected: bool) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeRoute is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::openRouteDistribution(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route distribution for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::openRouteDistribution is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeRouteDistribution()` [virtual, pure virtual, protected]
    /// closes (ends) the building of a distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_route_distribution(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeRouteDistribution is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeVehicle()` [virtual, pure virtual, protected]
    /// Ends the processing of a vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_vehicle(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeVehicle is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeVType()` [virtual, pure virtual, protected]
    /// Ends the processing of a vehicle type
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_v_type(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeVType is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closePerson()` [virtual, pure virtual, protected]
    /// Ends the processing of a person
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_person(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closePerson is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closePersonFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a person flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_person_flow(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closePersonFlow is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeContainer()` [virtual, pure virtual, protected]
    /// Ends the processing of a container
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_container(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeContainer is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeContainerFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a container flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_container_flow(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeContainerFlow is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_flow(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeFlow is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::closeTrip()` [virtual, pure virtual, protected]
    /// Ends the processing of a trip
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_trip(&mut self) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::closeTrip is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addTransportable(const SUMOSAXAttributes &, const bool)` [virtual, protected]
    /// Processing of a person or container
    // virtual void addTransportable(const SUMOSAXAttributes& attrs, const bool isPerson) {
    //         UNUSED_PARAMETER(attrs);
    //         UNUSED_PARAMETER(isPerson);
    //     }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, attrs: &SUMOSAXAttributes, isPerson: bool) {
        // the C++ base implementation does nothing (both parameters are unused)
        let _ = (attrs, isPerson);
    }

    /// C++: `Parameterised * SUMORouteHandler::addStop(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, attrs: &SUMOSAXAttributes) -> Ptr<Parameterised> {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addStop is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addPersonTrip(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// add a routing request for a walking or intermodal person
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_person_trip(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addPersonTrip is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addWalk(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// add a fully specified walk
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_walk(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addWalk is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addRide(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a ride
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_ride(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addRide is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addTransport(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a transport
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_transport(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addTransport is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::addTranship(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a tranship
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_tranship(&mut self, attrs: &SUMOSAXAttributes) {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::addTranship is pure virtual; a subclass must override it")
    }

    /// C++: `bool SUMORouteHandler::checkLastDepart()` [virtual, protected]
    /// Checks whether the route file is sorted by departure time if needed
    // bool
    // SUMORouteHandler::checkLastDepart() {
    //     if (myVehicleParameter->departProcedure == DepartDefinition::GIVEN) {
    //         if (myVehicleParameter->depart < myLastDepart) {
    //             WRITE_WARNINGF(TL("Route file should be sorted by departure time, ignoring '%'!"), myVehicleParameter->id);
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn check_last_depart(&mut self) -> bool {
        let vp = self.myVehicleParameter;
        if vp.m().departProcedure == DepartDefinition::GIVEN {
            if vp.m().depart < self.myLastDepart {
                let id = vp.m().id.clone();
                utils::write_warningf!("Route file should be sorted by departure time, ignoring '%'!", id.as_str());
                return false;
            }
        }
        true
    }

    /// C++: `void SUMORouteHandler::registerLastDepart()` [protected]
    /// save last depart (only to be used if vehicle is not discarded)
    // void
    // SUMORouteHandler::registerLastDepart() {
    //     // register only non public transport to parse all public transport lines in advance
    //     if (myVehicleParameter && myVehicleParameter->line == "" && myVehicleParameter->departProcedure == DepartDefinition::GIVEN) {
    //         myLastDepart = myVehicleParameter->depart;
    //         if (myFirstDepart == -1) {
    //             myFirstDepart = myLastDepart;
    //         }
    //     }
    //     // else: we don't know when this vehicle will depart. keep the previous known depart time
    // }
    #[allow(unused_variables)]
    pub fn register_last_depart(&mut self) {
        // register only non public transport to parse all public transport lines in advance
        let vp = self.myVehicleParameter;
        if !vp.is_null() && vp.m().line == "" && vp.m().departProcedure == DepartDefinition::GIVEN {
            self.myLastDepart = vp.m().depart;
            if self.myFirstDepart == -1 {
                self.myFirstDepart = self.myLastDepart;
            }
        }
        // else: we don't know when this vehicle will depart. keep the previous known depart time
    }

    /// C++: `void SUMORouteHandler::addParam(const SUMOSAXAttributes &)` [protected]
    /// assign arbitrary vehicle parameters
    // void
    // SUMORouteHandler::addParam(const SUMOSAXAttributes& attrs) {
    //     bool ok = true;
    //     const std::string key = attrs.get<std::string>(SUMO_ATTR_KEY, nullptr, ok);
    //     // only continue if key isn't empty
    //     if (ok && (key.size() > 0)) {
    //         // circumventing empty string test
    //         std::string val = attrs.hasAttribute(SUMO_ATTR_VALUE) ? attrs.getString(SUMO_ATTR_VALUE) : "";
    //         // check special params that must be interpreted as relative paths
    //         if ((myVehicleParameter != nullptr || myCurrentVType != nullptr)
    //                 && (key == "device.toc.file" || key == "device.ssm.file")) {
    //             val = FileHelpers::checkForRelativity(val, getFileName());
    //         }
    //         // add parameter in current created element
    //         if (!myParamStack.empty()) {
    //             if (!myCurrentVTypeRef.empty() && myParamStack.back() == myCurrentVType) {
    //                 WRITE_WARNINGF(TL("Ignoring param for referenced vType %"), myCurrentVType->id);
    //             } else {
    //                 myParamStack.back()->setParameter(key, val);
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn add_param(&mut self, attrs: &SUMOSAXAttributes) {
        let mut ok = true;
        let key = attrs.get::<String>(SUMO_ATTR_KEY, "", &mut ok, true);
        // only continue if key isn't empty
        if ok && !key.is_empty() {
            // circumventing empty string test
            let mut val = if attrs.has_attribute(SUMO_ATTR_VALUE) { attrs.get_string(SUMO_ATTR_VALUE) } else { String::new() };
            // check special params that must be interpreted as relative paths
            if (!self.myVehicleParameter.is_null() || !self.myCurrentVType.is_null()) && (key == "device.toc.file" || key == "device.ssm.file") {
                val = utils::common::file_helpers::check_for_relativity(&val, self.base.get_file_name());
            }
            // add parameter in current created element
            if !self.myParamStack.is_empty() {
                let back = *self.myParamStack.last().unwrap();
                if !self.myCurrentVTypeRef.is_empty() && back.as_raw() == self.myCurrentVType.cast::<Parameterised>().as_raw() {
                    let id = self.myCurrentVType.id.clone();
                    utils::write_warningf!("Ignoring param for referenced vType %", id.as_str());
                } else {
                    back.m().set_parameter(&key, &val);
                }
            }
        }
    }

    /// C++: `bool SUMORouteHandler::parseStop(SUMOVehicleParameter::Stop &, const SUMOSAXAttributes &, std::string, MsgHandler *const)` [protected]
    /// parses attributes common to all stops
    // bool
    // SUMORouteHandler::parseStop(SUMOVehicleParameter::Stop& stop, const SUMOSAXAttributes& attrs, std::string errorSuffix, MsgHandler* const errorOutput) {
    //     stop.parametersSet = 0;
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVAL)) {
    //         stop.parametersSet |= STOP_ARRIVAL_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_DURATION)) {
    //         stop.parametersSet |= STOP_DURATION_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_UNTIL)) {
    //         stop.parametersSet |= STOP_UNTIL_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_STARTED)) {
    //         stop.parametersSet |= STOP_STARTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDED)) {
    //         stop.parametersSet |= STOP_ENDED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXTENSION)) {
    //         stop.parametersSet |= STOP_EXTENSION_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ENDPOS)) {
    //         stop.parametersSet |= STOP_END_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_STARTPOS)) {
    //         stop.parametersSet |= STOP_START_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_POSITION_LAT)) {
    //         stop.parametersSet |= STOP_POSLAT_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TRIGGERED)) {
    //         stop.parametersSet |= STOP_TRIGGER_SET;
    //     }
    //     // legacy attribute
    //     if (attrs.hasAttribute(SUMO_ATTR_CONTAINER_TRIGGERED)) {
    //         stop.parametersSet |= STOP_TRIGGER_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PARKING)) {
    //         stop.parametersSet |= STOP_PARKING_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXPECTED)) {
    //         stop.parametersSet |= STOP_EXPECTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PERMITTED)) {
    //         stop.parametersSet |= STOP_PERMITTED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_EXPECTED_CONTAINERS)) {
    //         stop.parametersSet |= STOP_EXPECTED_CONTAINERS_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_TRIP_ID)) {
    //         stop.parametersSet |= STOP_TRIP_ID_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_SPLIT)) {
    //         stop.parametersSet |= STOP_SPLIT_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JOIN)) {
    //         stop.parametersSet |= STOP_JOIN_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_LINE)) {
    //         stop.parametersSet |= STOP_LINE_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_SPEED)) {
    //         stop.parametersSet |= STOP_SPEED_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_ONDEMAND)) {
    //         stop.parametersSet |= STOP_ONDEMAND_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_PRIORITY)) {
    //         stop.parametersSet |= STOP_PRIORITY_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JUMP)) {
    //         stop.parametersSet |= STOP_JUMP_SET;
    //     }
    //     if (attrs.hasAttribute(SUMO_ATTR_JUMP_UNTIL)) {
    //         stop.parametersSet |= STOP_JUMP_UNTIL_SET;
    //     }
    //     bool ok = true;
    //     stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_BUS_STOP, nullptr, ok, "");
    //     stop.busstop = attrs.getOpt<std::string>(SUMO_ATTR_TRAIN_STOP, nullptr, ok, stop.busstop);
    //     stop.chargingStation = attrs.getOpt<std::string>(SUMO_ATTR_CHARGING_STATION, nullptr, ok, "");
    //     stop.overheadWireSegment = attrs.getOpt<std::string>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, nullptr, ok, "");
    //     stop.containerstop = attrs.getOpt<std::string>(SUMO_ATTR_CONTAINER_STOP, nullptr, ok, "");
    //     stop.parkingarea = attrs.getOpt<std::string>(SUMO_ATTR_PARKING_AREA, nullptr, ok, "");
    //     if (stop.busstop != "") {
    //         errorSuffix = " at '" + stop.busstop + "'" + errorSuffix;
    //     } else if (stop.chargingStation != "") {
    //         errorSuffix = " at '" + stop.chargingStation + "'" + errorSuffix;
    //     } else if (stop.overheadWireSegment != "") {
    //         errorSuffix = " at '" + stop.overheadWireSegment + "'" + errorSuffix;
    //     } else if (stop.containerstop != "") {
    //         errorSuffix = " at '" + stop.containerstop + "'" + errorSuffix;
    //     } else if (stop.parkingarea != "") {
    //         errorSuffix = " at '" + stop.parkingarea + "'" + errorSuffix;
    //     } else if (attrs.hasAttribute(SUMO_ATTR_LANE)) {
    //         errorSuffix = " on lane '" + attrs.get<std::string>(SUMO_ATTR_LANE, nullptr, ok) + "'" + errorSuffix;
    //     } else if (attrs.hasAttribute(SUMO_ATTR_EDGE)) {
    //         errorSuffix = " on edge '" + attrs.get<std::string>(SUMO_ATTR_EDGE, nullptr, ok) + "'" + errorSuffix;
    //     } else {
    //         errorSuffix = " at undefined location" + errorSuffix;
    //     }
    //     // speed for counting as stopped
    //     stop.speed = attrs.getOpt<double>(SUMO_ATTR_SPEED, nullptr, ok, 0);
    //     if (stop.speed < 0) {
    //         errorOutput->inform(TLF("Speed cannot be negative for stop%.", errorSuffix));
    //         return false;
    //     }
    //
    //     // get the standing duration
    //     bool expectTrigger = !attrs.hasAttribute(SUMO_ATTR_DURATION) && !attrs.hasAttribute(SUMO_ATTR_UNTIL) && !attrs.hasAttribute(SUMO_ATTR_SPEED);
    //     std::vector<std::string> triggers = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_TRIGGERED, nullptr, ok);
    //     // legacy
    //     if (attrs.getOpt<bool>(SUMO_ATTR_CONTAINER_TRIGGERED, nullptr, ok, false)) {
    //         triggers.push_back(toString(SUMO_TAG_CONTAINER));
    //     }
    //     SUMOVehicleParameter::parseStopTriggers(triggers, expectTrigger, stop);
    //     stop.arrival = attrs.getOptSUMOTimeReporting(SUMO_ATTR_ARRIVAL, nullptr, ok, -1);
    //     if (attrs.hasAttribute(SUMO_ATTR_DURATION)) {
    //         const std::string durationStr = attrs.get<std::string>(SUMO_ATTR_DURATION, nullptr, ok);
    //         std::string error;
    //         if (!Distribution_Parameterized::isValidDescription(durationStr, error)) {
    //             errorOutput->inform(TLF("Invalid duration for a stop% (%).", errorSuffix, error));
    //             return false;
    //         }
    //         stop.durationDist = Distribution_Parameterized(durationStr);
    //         stop.duration = TIME2STEPS(stop.durationDist.sample(&myParsingRNG));
    //     }
    //     stop.until = attrs.getOptSUMOTimeReporting(SUMO_ATTR_UNTIL, nullptr, ok, -1);
    //     if (!expectTrigger && (!ok || (stop.duration < 0 && stop.until < 0 && stop.speed == 0))) {
    //         errorOutput->inform(TLF("Invalid duration or end time is given for a stop%.", errorSuffix));
    //         return false;
    //     }
    //     if (triggers.size() > 0 && stop.speed > 0) {
    //         errorOutput->inform(TLF("Triggers and waypoints cannot be combined%.", errorSuffix));
    //         return false;
    //     }
    //     stop.extension = attrs.getOptSUMOTimeReporting(SUMO_ATTR_EXTENSION, nullptr, ok, -1);
    //     const bool defaultParking = (stop.triggered || stop.containerTriggered || stop.parkingarea != "");
    //     stop.parking = attrs.getOpt<ParkingType>(SUMO_ATTR_PARKING, nullptr, ok, defaultParking ? ParkingType::OFFROAD : ParkingType::ONROAD);
    //     if ((stop.parkingarea != "") && (stop.parking == ParkingType::ONROAD)) {
    //         WRITE_WARNINGF(TL("Stop at parkingArea overrides attribute 'parking' for stop%."), errorSuffix);
    //         stop.parking = ParkingType::OFFROAD;
    //     }
    //
    //     // expected persons
    //     const std::vector<std::string>& expected = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EXPECTED, nullptr, ok);
    //     stop.awaitedPersons.insert(expected.begin(), expected.end());
    //     if (stop.awaitedPersons.size() > 0) {
    //         stop.triggered = true;
    //         if ((stop.parametersSet & STOP_PARKING_SET) == 0) {
    //             stop.parking = ParkingType::OFFROAD;
    //         }
    //     }
    //
    //     // permitted transportables
    //     const std::vector<std::string>& permitted = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_PERMITTED, nullptr, ok);
    //     stop.permitted.insert(permitted.begin(), permitted.end());
    //
    //     // expected containers
    //     const std::vector<std::string>& expectedContainers = attrs.getOpt<std::vector<std::string> >(SUMO_ATTR_EXPECTED_CONTAINERS, nullptr, ok);
    //     stop.awaitedContainers.insert(expectedContainers.begin(), expectedContainers.end());
    //     if (stop.awaitedContainers.size() > 0) {
    //         stop.containerTriggered = true;
    //         if ((stop.parametersSet & STOP_PARKING_SET) == 0) {
    //             stop.parking = ParkingType::OFFROAD;
    //         }
    //     }
    //     // public transport trip id
    //     stop.tripId = attrs.getOpt<std::string>(SUMO_ATTR_TRIP_ID, nullptr, ok, "");
    //     stop.split = attrs.getOpt<std::string>(SUMO_ATTR_SPLIT, nullptr, ok, "");
    //     stop.join = attrs.getOpt<std::string>(SUMO_ATTR_JOIN, nullptr, ok, "");
    //     stop.line = attrs.getOpt<std::string>(SUMO_ATTR_LINE, nullptr, ok, "");
    //
    //     const std::string idx = attrs.getOpt<std::string>(SUMO_ATTR_INDEX, nullptr, ok, "end");
    //     if (idx == "end") {
    //         stop.index = STOP_INDEX_END;
    //     } else if (idx == "fit") {
    //         stop.index = STOP_INDEX_FIT;
    //     } else {
    //         stop.index = attrs.get<int>(SUMO_ATTR_INDEX, nullptr, ok);
    //         if (!ok || stop.index < 0) {
    //             errorOutput->inform(TLF("Invalid 'index' for stop%.", errorSuffix));
    //             return false;
    //         }
    //     }
    //     stop.started = attrs.getOptSUMOTimeReporting(SUMO_ATTR_STARTED, nullptr, ok, -1);
    //     stop.ended = attrs.getOptSUMOTimeReporting(SUMO_ATTR_ENDED, nullptr, ok, -1);
    //     stop.posLat = attrs.getOpt<double>(SUMO_ATTR_POSITION_LAT, nullptr, ok, INVALID_DOUBLE);
    //     stop.actType = attrs.getOpt<std::string>(SUMO_ATTR_ACTTYPE, nullptr, ok, "");
    //     stop.onDemand = attrs.getOpt<bool>(SUMO_ATTR_ONDEMAND, nullptr, ok, false);
    //     stop.priority = attrs.getOpt<double>(SUMO_ATTR_PRIORITY, nullptr, ok, -1);
    //     stop.jump = attrs.getOptSUMOTimeReporting(SUMO_ATTR_JUMP, nullptr, ok, -1);
    //     stop.jumpUntil = attrs.getOptSUMOTimeReporting(SUMO_ATTR_JUMP_UNTIL, nullptr, ok, -1);
    //     stop.collision = attrs.getOpt<bool>(SUMO_ATTR_COLLISION, nullptr, ok, false);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn parse_stop(&mut self, stop: &mut SUMOVehicleParameter_Stop, attrs: &SUMOSAXAttributes, errorSuffix: String, errorOutput: Ptr<MsgHandler>) -> bool {
        let mut errorSuffix = errorSuffix;
        stop.parametersSet = 0;
        if attrs.has_attribute(SUMO_ATTR_ARRIVAL) {
            stop.parametersSet |= STOP_ARRIVAL_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_DURATION) {
            stop.parametersSet |= STOP_DURATION_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_UNTIL) {
            stop.parametersSet |= STOP_UNTIL_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_STARTED) {
            stop.parametersSet |= STOP_STARTED_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_ENDED) {
            stop.parametersSet |= STOP_ENDED_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_EXTENSION) {
            stop.parametersSet |= STOP_EXTENSION_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_ENDPOS) {
            stop.parametersSet |= STOP_END_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_STARTPOS) {
            stop.parametersSet |= STOP_START_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_POSITION_LAT) {
            stop.parametersSet |= STOP_POSLAT_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_TRIGGERED) {
            stop.parametersSet |= STOP_TRIGGER_SET;
        }
        // legacy attribute
        if attrs.has_attribute(SUMO_ATTR_CONTAINER_TRIGGERED) {
            stop.parametersSet |= STOP_TRIGGER_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_PARKING) {
            stop.parametersSet |= STOP_PARKING_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_EXPECTED) {
            stop.parametersSet |= STOP_EXPECTED_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_PERMITTED) {
            stop.parametersSet |= STOP_PERMITTED_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_EXPECTED_CONTAINERS) {
            stop.parametersSet |= STOP_EXPECTED_CONTAINERS_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_TRIP_ID) {
            stop.parametersSet |= STOP_TRIP_ID_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_SPLIT) {
            stop.parametersSet |= STOP_SPLIT_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_JOIN) {
            stop.parametersSet |= STOP_JOIN_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_LINE) {
            stop.parametersSet |= STOP_LINE_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_SPEED) {
            stop.parametersSet |= STOP_SPEED_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_ONDEMAND) {
            stop.parametersSet |= STOP_ONDEMAND_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_PRIORITY) {
            stop.parametersSet |= STOP_PRIORITY_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_JUMP) {
            stop.parametersSet |= STOP_JUMP_SET;
        }
        if attrs.has_attribute(SUMO_ATTR_JUMP_UNTIL) {
            stop.parametersSet |= STOP_JUMP_UNTIL_SET;
        }
        let mut ok = true;
        stop.busstop = attrs.get_opt::<String>(SUMO_ATTR_BUS_STOP, "", &mut ok, String::new(), true);
        stop.busstop = attrs.get_opt::<String>(SUMO_ATTR_TRAIN_STOP, "", &mut ok, stop.busstop.clone(), true);
        stop.chargingStation = attrs.get_opt::<String>(SUMO_ATTR_CHARGING_STATION, "", &mut ok, String::new(), true);
        stop.overheadWireSegment = attrs.get_opt::<String>(SUMO_ATTR_OVERHEAD_WIRE_SEGMENT, "", &mut ok, String::new(), true);
        stop.containerstop = attrs.get_opt::<String>(SUMO_ATTR_CONTAINER_STOP, "", &mut ok, String::new(), true);
        stop.parkingarea = attrs.get_opt::<String>(SUMO_ATTR_PARKING_AREA, "", &mut ok, String::new(), true);
        if !stop.busstop.is_empty() {
            errorSuffix = format!(" at '{}'{}", stop.busstop, errorSuffix);
        } else if !stop.chargingStation.is_empty() {
            errorSuffix = format!(" at '{}'{}", stop.chargingStation, errorSuffix);
        } else if !stop.overheadWireSegment.is_empty() {
            errorSuffix = format!(" at '{}'{}", stop.overheadWireSegment, errorSuffix);
        } else if !stop.containerstop.is_empty() {
            errorSuffix = format!(" at '{}'{}", stop.containerstop, errorSuffix);
        } else if !stop.parkingarea.is_empty() {
            errorSuffix = format!(" at '{}'{}", stop.parkingarea, errorSuffix);
        } else if attrs.has_attribute(SUMO_ATTR_LANE) {
            errorSuffix = format!(" on lane '{}'{}", attrs.get::<String>(SUMO_ATTR_LANE, "", &mut ok, true), errorSuffix);
        } else if attrs.has_attribute(SUMO_ATTR_EDGE) {
            errorSuffix = format!(" on edge '{}'{}", attrs.get::<String>(SUMO_ATTR_EDGE, "", &mut ok, true), errorSuffix);
        } else {
            errorSuffix = format!(" at undefined location{}", errorSuffix);
        }
        // speed for counting as stopped
        stop.speed = attrs.get_opt::<f64>(SUMO_ATTR_SPEED, "", &mut ok, 0.0, true);
        if stop.speed < 0.0 {
            errorOutput.m().inform(&utils::tlf!("Speed cannot be negative for stop%.", errorSuffix.clone()), true);
            return false;
        }

        // get the standing duration
        let expectTrigger = !attrs.has_attribute(SUMO_ATTR_DURATION) && !attrs.has_attribute(SUMO_ATTR_UNTIL) && !attrs.has_attribute(SUMO_ATTR_SPEED);
        let mut triggers: Vec<String> = attrs.get_opt::<Vec<String>>(SUMO_ATTR_TRIGGERED, "", &mut ok, Vec::new(), true);
        // legacy
        if attrs.get_opt::<bool>(SUMO_ATTR_CONTAINER_TRIGGERED, "", &mut ok, false, true) {
            triggers.push(tag_to_string(SUMO_TAG_CONTAINER));
        }
        SUMOVehicleParameter::parse_stop_triggers(&triggers, expectTrigger, stop);
        stop.arrival = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_ARRIVAL, "", &mut ok, -1, true);
        if attrs.has_attribute(SUMO_ATTR_DURATION) {
            let durationStr = attrs.get::<String>(SUMO_ATTR_DURATION, "", &mut ok, true);
            let mut error = String::new();
            if !utils::distribution::distribution_parameterized::DistributionParameterized::is_valid_description(&durationStr, &mut error) {
                errorOutput.m().inform(&utils::tlf!("Invalid duration for a stop% (%).", errorSuffix.clone(), error.clone()), true);
                return false;
            }
            // shortcut: SUMOVehicleParameter_Stop::durationDist is Unmapped, so the parsed distribution is not stored here, upgrade when that field is typed as DistributionParameterized
            let durationDist = utils::distribution::distribution_parameterized::DistributionParameterized::from_description(&durationStr).unwrap_or_else(|e| panic!("{}", e));
            stop.duration = utils::common::time::time2steps(durationDist.sample(Some(Self::get_parsing_rng().m())).unwrap_or_else(|e| panic!("{}", e)));
        }
        stop.until = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_UNTIL, "", &mut ok, -1, true);
        if !expectTrigger && (!ok || (stop.duration < 0 && stop.until < 0 && stop.speed == 0.0)) {
            errorOutput.m().inform(&utils::tlf!("Invalid duration or end time is given for a stop%.", errorSuffix.clone()), true);
            return false;
        }
        if triggers.len() > 0 && stop.speed > 0.0 {
            errorOutput.m().inform(&utils::tlf!("Triggers and waypoints cannot be combined%.", errorSuffix.clone()), true);
            return false;
        }
        stop.extension = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_EXTENSION, "", &mut ok, -1, true);
        let defaultParking = stop.triggered || stop.containerTriggered || !stop.parkingarea.is_empty();
        stop.parking = attrs.get_opt::<ParkingType>(SUMO_ATTR_PARKING, "", &mut ok, if defaultParking { ParkingType::OFFROAD } else { ParkingType::ONROAD }, true);
        if (!stop.parkingarea.is_empty()) && (stop.parking == ParkingType::ONROAD) {
            utils::write_warningf!("Stop at parkingArea overrides attribute 'parking' for stop%.", errorSuffix.as_str());
            stop.parking = ParkingType::OFFROAD;
        }

        // expected persons
        let expected = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EXPECTED, "", &mut ok, Vec::new(), true);
        stop.awaitedPersons.extend(expected);
        if stop.awaitedPersons.len() > 0 {
            stop.triggered = true;
            if (stop.parametersSet & STOP_PARKING_SET) == 0 {
                stop.parking = ParkingType::OFFROAD;
            }
        }

        // permitted transportables
        let permitted = attrs.get_opt::<Vec<String>>(SUMO_ATTR_PERMITTED, "", &mut ok, Vec::new(), true);
        stop.permitted.extend(permitted);

        // expected containers
        let expectedContainers = attrs.get_opt::<Vec<String>>(SUMO_ATTR_EXPECTED_CONTAINERS, "", &mut ok, Vec::new(), true);
        stop.awaitedContainers.extend(expectedContainers);
        if stop.awaitedContainers.len() > 0 {
            stop.containerTriggered = true;
            if (stop.parametersSet & STOP_PARKING_SET) == 0 {
                stop.parking = ParkingType::OFFROAD;
            }
        }
        // public transport trip id
        stop.tripId = attrs.get_opt::<String>(SUMO_ATTR_TRIP_ID, "", &mut ok, String::new(), true);
        stop.split = attrs.get_opt::<String>(SUMO_ATTR_SPLIT, "", &mut ok, String::new(), true);
        stop.join = attrs.get_opt::<String>(SUMO_ATTR_JOIN, "", &mut ok, String::new(), true);
        stop.line = attrs.get_opt::<String>(SUMO_ATTR_LINE, "", &mut ok, String::new(), true);

        let idx = attrs.get_opt::<String>(SUMO_ATTR_INDEX, "", &mut ok, "end".to_string(), true);
        if idx == "end" {
            stop.index = STOP_INDEX_END;
        } else if idx == "fit" {
            stop.index = STOP_INDEX_FIT;
        } else {
            stop.index = attrs.get::<i32>(SUMO_ATTR_INDEX, "", &mut ok, true);
            if !ok || stop.index < 0 {
                errorOutput.m().inform(&utils::tlf!("Invalid 'index' for stop%.", errorSuffix.clone()), true);
                return false;
            }
        }
        stop.started = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_STARTED, "", &mut ok, -1, true);
        stop.ended = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_ENDED, "", &mut ok, -1, true);
        stop.posLat = attrs.get_opt::<f64>(SUMO_ATTR_POSITION_LAT, "", &mut ok, INVALID_DOUBLE, true);
        stop.actType = attrs.get_opt::<String>(SUMO_ATTR_ACTTYPE, "", &mut ok, String::new(), true);
        stop.onDemand = attrs.get_opt::<bool>(SUMO_ATTR_ONDEMAND, "", &mut ok, false, true);
        stop.priority = attrs.get_opt::<f64>(SUMO_ATTR_PRIORITY, "", &mut ok, -1.0, true);
        stop.jump = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_JUMP, "", &mut ok, -1, true);
        stop.jumpUntil = attrs.get_opt_sumo_time_reporting(SUMO_ATTR_JUMP_UNTIL, "", &mut ok, -1, true);
        stop.collision = attrs.get_opt::<bool>(SUMO_ATTR_COLLISION, "", &mut ok, false, true);
        true
    }

    /// C++: `const SUMOVTypeParameter * SUMORouteHandler::getVTypeParameter(const std::string &)` [virtual, pure virtual, protected]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&mut self, refid: &str) -> Ptr<SUMOVTypeParameter> {
        // shortcut: pure virtual in C++ (no base body); the Rust base struct has no virtual dispatch, so subclasses must provide their own inherent method
        panic!("SUMORouteHandler::getVTypeParameter is pure virtual; a subclass must override it")
    }

    /// C++: `void SUMORouteHandler::SUMORouteHandler(const SUMORouteHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &SUMORouteHandler) -> Self {
        panic!("SUMORouteHandler copy constructor is private and undefined in C++ (invalidated)")
    }

    /// C++: `SUMORouteHandler & SUMORouteHandler::operator=(const SUMORouteHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &SUMORouteHandler) -> &mut SUMORouteHandler {
        panic!("SUMORouteHandler assignment operator is private and undefined in C++ (invalidated)")
    }

}


impl utils::xml::generic_sax_handler::SaxHandler for SUMORouteHandler {
    fn handler(&mut self) -> &mut utils::xml::generic_sax_handler::GenericSAXHandler { &mut self.base }
    fn my_start_element(&mut self, element: i32, attrs: &SUMOSAXAttributes) -> Result<(), utils::common::util_exceptions::ProcessError> {
        self.my_start_element(element, attrs);
        Ok(())
    }
    fn my_end_element(&mut self, element: i32) -> Result<(), utils::common::util_exceptions::ProcessError> {
        self.my_end_element(element);
        Ok(())
    }
}
