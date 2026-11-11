//! Port of utils/vehicle/SUMORouteHandler.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
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
pub struct SUMORouteHandler {
    /// C++ base class `SUMOSAXHandler`
    pub base: Unmapped /* SUMOSAXHandler */,
    /// flag to enable or disable hard fails
    pub myHardFail: bool,
    /// Parameter of the current vehicle, trip, person, container or flow
    pub myVehicleParameter: Ptr<SUMOVehicleParameter>,
    /// The stack of currently parsed parameterised objects
    pub myParamStack: i32,
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
    pub myElementStack: i32,
    /// whether references to internal routes are allowed in this context
    pub myAllowInternalRoutes: bool,
    /// IDs of skipped vehicles to suppress errors for the triggered transportables within
    pub mySkippedVehicles: i32,
}

impl std::ops::Deref for SUMORouteHandler {
    type Target = Unmapped /* SUMOSAXHandler */;
    fn deref(&self) -> &Unmapped /* SUMOSAXHandler */ {
        &self.base
    }
}
impl std::ops::DerefMut for SUMORouteHandler {
    fn deref_mut(&mut self) -> &mut Unmapped /* SUMOSAXHandler */ {
        &mut self.base
    }
}

// C++ static members of SUMORouteHandler (port as module-level statics, see AGENTS.md):
//   static SumoRNG SUMORouteHandler::myParsingRNG;

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
        todo!("port SUMORouteHandler::SUMORouteHandler")
    }

    /// C++: `void SUMORouteHandler::~SUMORouteHandler()` [virtual]
    /// standard destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SUMORouteHandler::~SUMORouteHandler")
    }

    /// C++: `StopPos SUMORouteHandler::checkStopPos(double &, double &, const double, const double, const bool)` [static]
    /// return
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn check_stop_pos(startPos: &mut f64, endPos: &mut f64, laneLength: f64, minLength: f64, friendlyPos: bool) -> SUMORouteHandler_StopPos {
        todo!("port SUMORouteHandler::checkStopPos")
    }

    /// C++: `SUMOTime SUMORouteHandler::getFirstDepart() const` [const]
    /// returns the first departure time that was ever read
    // SUMOTime
    // SUMORouteHandler::getFirstDepart() const {
    //     return myFirstDepart;
    // }
    #[allow(unused_variables)]
    pub fn get_first_depart(&self) -> SUMOTime {
        todo!("port SUMORouteHandler::getFirstDepart")
    }

    /// C++: `SUMOTime SUMORouteHandler::getLastDepart() const` [const]
    /// Returns the last loaded depart time
    // SUMOTime
    // SUMORouteHandler::getLastDepart() const {
    //     return myLastDepart;
    // }
    #[allow(unused_variables)]
    pub fn get_last_depart(&self) -> SUMOTime {
        todo!("port SUMORouteHandler::getLastDepart")
    }

    /// C++: `SumoRNG * SUMORouteHandler::getParsingRNG()` [static]
    /// get parsing RNG
    // static SumoRNG* getParsingRNG() {
    //         return &myParsingRNG;
    //     }
    #[allow(unused_variables)]
    pub fn get_parsing_rng() -> Ptr<SumoRNG> {
        todo!("port SUMORouteHandler::getParsingRNG")
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
        todo!("port SUMORouteHandler::myStartElement")
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
        todo!("port SUMORouteHandler::myEndElement")
    }

    /// C++: `void SUMORouteHandler::openVehicleTypeDistribution(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a type distribution for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_vehicle_type_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openVehicleTypeDistribution")
    }

    /// C++: `void SUMORouteHandler::closeVehicleTypeDistribution()` [virtual, pure virtual, protected]
    /// closes (ends) the building of a distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_vehicle_type_distribution(&mut self) {
        todo!("port SUMORouteHandler::closeVehicleTypeDistribution")
    }

    /// C++: `void SUMORouteHandler::openRoute(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openRoute")
    }

    /// C++: `void SUMORouteHandler::openFlow(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a flow for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openFlow")
    }

    /// C++: `void SUMORouteHandler::openRouteFlow(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route flow for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route_flow(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openRouteFlow")
    }

    /// C++: `void SUMORouteHandler::openTrip(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a trip for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_trip(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openTrip")
    }

    /// C++: `void SUMORouteHandler::closeRoute(const bool)` [virtual, pure virtual, protected]
    /// closes (ends) the building of a route. Afterwards no edges may be added to it; this method may throw exceptions when a) the route is empty or b) another route with the same id already exists
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_route(&mut self, mayBeDisconnected: bool) {
        todo!("port SUMORouteHandler::closeRoute")
    }

    /// C++: `void SUMORouteHandler::openRouteDistribution(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// opens a route distribution for reading
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn open_route_distribution(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::openRouteDistribution")
    }

    /// C++: `void SUMORouteHandler::closeRouteDistribution()` [virtual, pure virtual, protected]
    /// closes (ends) the building of a distribution
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_route_distribution(&mut self) {
        todo!("port SUMORouteHandler::closeRouteDistribution")
    }

    /// C++: `void SUMORouteHandler::closeVehicle()` [virtual, pure virtual, protected]
    /// Ends the processing of a vehicle
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_vehicle(&mut self) {
        todo!("port SUMORouteHandler::closeVehicle")
    }

    /// C++: `void SUMORouteHandler::closeVType()` [virtual, pure virtual, protected]
    /// Ends the processing of a vehicle type
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_v_type(&mut self) {
        todo!("port SUMORouteHandler::closeVType")
    }

    /// C++: `void SUMORouteHandler::closePerson()` [virtual, pure virtual, protected]
    /// Ends the processing of a person
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_person(&mut self) {
        todo!("port SUMORouteHandler::closePerson")
    }

    /// C++: `void SUMORouteHandler::closePersonFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a person flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_person_flow(&mut self) {
        todo!("port SUMORouteHandler::closePersonFlow")
    }

    /// C++: `void SUMORouteHandler::closeContainer()` [virtual, pure virtual, protected]
    /// Ends the processing of a container
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_container(&mut self) {
        todo!("port SUMORouteHandler::closeContainer")
    }

    /// C++: `void SUMORouteHandler::closeContainerFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a container flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_container_flow(&mut self) {
        todo!("port SUMORouteHandler::closeContainerFlow")
    }

    /// C++: `void SUMORouteHandler::closeFlow()` [virtual, pure virtual, protected]
    /// Ends the processing of a flow
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_flow(&mut self) {
        todo!("port SUMORouteHandler::closeFlow")
    }

    /// C++: `void SUMORouteHandler::closeTrip()` [virtual, pure virtual, protected]
    /// Ends the processing of a trip
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn close_trip(&mut self) {
        todo!("port SUMORouteHandler::closeTrip")
    }

    /// C++: `void SUMORouteHandler::addTransportable(const SUMOSAXAttributes &, const bool)` [virtual, protected]
    /// Processing of a person or container
    // virtual void addTransportable(const SUMOSAXAttributes& attrs, const bool isPerson) {
    //         UNUSED_PARAMETER(attrs);
    //         UNUSED_PARAMETER(isPerson);
    //     }
    #[allow(unused_variables)]
    pub fn add_transportable(&mut self, attrs: &SUMOSAXAttributes, isPerson: bool) {
        todo!("port SUMORouteHandler::addTransportable")
    }

    /// C++: `Parameterised * SUMORouteHandler::addStop(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_stop(&mut self, attrs: &SUMOSAXAttributes) -> Ptr<Parameterised> {
        todo!("port SUMORouteHandler::addStop")
    }

    /// C++: `void SUMORouteHandler::addPersonTrip(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// add a routing request for a walking or intermodal person
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_person_trip(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::addPersonTrip")
    }

    /// C++: `void SUMORouteHandler::addWalk(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// add a fully specified walk
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_walk(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::addWalk")
    }

    /// C++: `void SUMORouteHandler::addRide(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a ride
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_ride(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::addRide")
    }

    /// C++: `void SUMORouteHandler::addTransport(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a transport
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_transport(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::addTransport")
    }

    /// C++: `void SUMORouteHandler::addTranship(const SUMOSAXAttributes &)` [virtual, pure virtual, protected]
    /// Processing of a tranship
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn add_tranship(&mut self, attrs: &SUMOSAXAttributes) {
        todo!("port SUMORouteHandler::addTranship")
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
        todo!("port SUMORouteHandler::checkLastDepart")
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
        todo!("port SUMORouteHandler::registerLastDepart")
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
        todo!("port SUMORouteHandler::addParam")
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
        todo!("port SUMORouteHandler::parseStop")
    }

    /// C++: `const SUMOVTypeParameter * SUMORouteHandler::getVTypeParameter(const std::string &)` [virtual, pure virtual, protected]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&mut self, refid: &str) -> Ptr<SUMOVTypeParameter> {
        todo!("port SUMORouteHandler::getVTypeParameter")
    }

    /// C++: `void SUMORouteHandler::SUMORouteHandler(const SUMORouteHandler &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &SUMORouteHandler) -> Self {
        todo!("port SUMORouteHandler::SUMORouteHandler")
    }

    /// C++: `SUMORouteHandler & SUMORouteHandler::operator=(const SUMORouteHandler &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &SUMORouteHandler) -> &mut SUMORouteHandler {
        todo!("port SUMORouteHandler::operator=")
    }

}

