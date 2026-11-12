//! Port of microsim/devices/MSDevice_Bluelight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Bluelight` (microsim/devices/MSDevice_Bluelight.h)
pub struct MSDevice_Bluelight {
    /// C++ base class `MSVehicleDevice`
    pub base: MSVehicleDevice,
    pub myInfluencedVehicles: BTreeSet<String>,
    pub myInfluencedTypes: utils::common::parameterised::ParameterisedMap /* Parameterised::Map */,
    /// reaction distance of other vehicle (i.e. due to different noise levels of the siren)
    pub myReactionDist: f64,
    /// min gap reduction of other vehicles
    pub myMinGapFactor: f64,
}

impl std::ops::Deref for MSDevice_Bluelight {
    type Target = MSVehicleDevice;
    fn deref(&self) -> &MSVehicleDevice {
        &self.base
    }
}
impl std::ops::DerefMut for MSDevice_Bluelight {
    fn deref_mut(&mut self) -> &mut MSVehicleDevice {
        &mut self.base
    }
}

impl MSDevice_Bluelight {
    /// C++: `void MSDevice_Bluelight::insertOptions(OptionsCont &)` [static]
    /// Inserts MSDevice_Bluelight-options
    // void
    // MSDevice_Bluelight::insertOptions(OptionsCont& oc) {
    //     oc.addOptionSubTopic("Bluelight Device");
    //     insertDefaultAssignmentOptions("bluelight", "Bluelight Device", oc);
    //
    //     oc.doRegister("device.bluelight.reactiondist", new Option_Float(25.0));
    //     oc.addDescription("device.bluelight.reactiondist", "Bluelight Device", TL("Set the distance at which other drivers react to the blue light and siren sound"));
    //     oc.doRegister("device.bluelight.mingapfactor", new Option_Float(1.));
    //     oc.addDescription("device.bluelight.mingapfactor", "Bluelight Device", TL("Reduce the minGap for reacting vehicles by the given factor"));
    // }
    #[allow(unused_variables)]
    pub fn insert_options(oc: &mut OptionsCont) {
        oc.add_option_sub_topic("Bluelight Device");
        MSDevice::insert_default_assignment_options("bluelight", "Bluelight Device", oc, false);

        oc.do_register("device.bluelight.reactiondist", SumoOption::float(25.0));
        oc.add_description("device.bluelight.reactiondist", "Bluelight Device", "Set the distance at which other drivers react to the blue light and siren sound");
        oc.do_register("device.bluelight.mingapfactor", SumoOption::float(1.));
        oc.add_description("device.bluelight.mingapfactor", "Bluelight Device", "Reduce the minGap for reacting vehicles by the given factor");
    }

    /// C++: `void MSDevice_Bluelight::buildVehicleDevices(SUMOVehicle &, std::vector<MSVehicleDevice *> &)` [static]
    /// Build devices for the given vehicle, if needed
    // void
    // MSDevice_Bluelight::buildVehicleDevices(SUMOVehicle& v, std::vector<MSVehicleDevice*>& into) {
    //     OptionsCont& oc = OptionsCont::getOptions();
    //     if (equippedByDefaultAssignmentOptions(oc, "bluelight", v, false)) {
    //         if (MSGlobals::gUseMesoSim) {
    //             WRITE_WARNINGF(TL("bluelight device is not compatible with mesosim (ignored for vehicle '%')"), v.getID());
    //         } else {
    //             MSDevice_Bluelight* device = new MSDevice_Bluelight(v, "bluelight_" + v.getID(),
    //                     v.getFloatParam("device.bluelight.reactiondist"),
    //                     v.getFloatParam("device.bluelight.mingapfactor"));
    //             into.push_back(device);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle_devices(v: &mut SUMOVehicle, into: &mut Vec<Ptr<MSVehicleDevice>>) {
        todo!("port MSDevice_Bluelight::buildVehicleDevices")
    }

    /// C++: `void MSDevice_Bluelight::~MSDevice_Bluelight() noexcept` [virtual]
    /// Destructor.
    // MSDevice_Bluelight::~MSDevice_Bluelight() {
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `bool MSDevice_Bluelight::notifyMove(SUMOTrafficObject &, double, double, double)` [virtual]
    /// Checks for waiting steps when the vehicle moves
    // bool
    // MSDevice_Bluelight::notifyMove(SUMOTrafficObject& veh, double /* oldPos */,
    //                                double /* newPos */, double newSpeed) {
    // #ifdef DEBUG_BLUELIGHT
    //     std::cout << SIMTIME  << " device '" << getID() << "' notifyMove: newSpeed=" << newSpeed << "\n";
    // #else
    //     UNUSED_PARAMETER(newSpeed);
    // #endif
    //     //violate red lights  this only need to be done once so shift it todo
    //     MSVehicle& ego = dynamic_cast<MSVehicle&>(veh);
    //     MSVehicle::Influencer& redLight = ego.getInfluencer();
    //     const double vMax = ego.getLane()->getVehicleMaxSpeed(&ego);
    //     if (ego.getSpeed() < 0.5 * vMax) {
    //         // advance as far as possible (assume vehicles will keep moving out of the way)
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_STRATEGIC_PARAM), "-1");
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD), "0");
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME), "0");
    //         try {
    //             ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_MINGAP_LAT), "0");
    //         } catch (InvalidArgument&) {
    //             // not supported by the current laneChangeModel
    //         }
    //     } else {
    //         // restore defaults
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_STRATEGIC_PARAM),
    //                                               ego.getVehicleType().getParameter().getLCParamString(SUMO_ATTR_LCA_STRATEGIC_PARAM, "1"));
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD),
    //                                               ego.getVehicleType().getParameter().getLCParamString(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD, "5"));
    //         ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME),
    //                                               ego.getVehicleType().getParameter().getLCParamString(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME, "20"));
    //         try {
    //             ego.getLaneChangeModel().setParameter(toString(SUMO_ATTR_MINGAP_LAT),
    //                                                   toString(ego.getVehicleType().getMinGapLat()));
    //         } catch (InvalidArgument&) {
    //             // not supported by the current laneChangeModel
    //         }
    //     }
    //     // build a rescue lane for all vehicles on the route of the emergency vehicle within the range of the siren
    //     MSVehicleControl& vc = MSNet::getInstance()->getVehicleControl();
    //     // use edges on the way of the emergency vehicle
    //     std::vector<const MSEdge*> upcomingEdges;
    //     std::set<MSVehicle*, ComparatorIdLess> upcomingVehicles;
    //     std::set<std::string> lastStepInfluencedVehicles = myInfluencedVehicles;
    //     std::vector<MSLink*> upcomingLinks;
    //     double affectedJunctionDist = ego.getPositionOnLane() + myReactionDist;
    //     for (const MSLane* const l : ego.getUpcomingLanesUntil(myReactionDist)) {
    //         upcomingEdges.push_back(&l->getEdge());
    //
    //         affectedJunctionDist -= l->getLength();
    //         if (affectedJunctionDist > 0 && l->isInternal()) {
    //             upcomingLinks.push_back(l->getIncomingLanes()[0].viaLink);
    //         }
    //     }
    //
    //     for (const MSEdge* const e : upcomingEdges) {
    //         //inform all vehicles on upcomingEdges
    //         for (const SUMOVehicle* v : e->getVehicles()) {
    //             upcomingVehicles.insert(dynamic_cast<MSVehicle*>(const_cast<SUMOVehicle*>(v)));
    //             if (lastStepInfluencedVehicles.count(v->getID()) > 0) {
    //                 lastStepInfluencedVehicles.erase(v->getID());
    //             }
    //         }
    //     }
    //     // reset all vehicles that were in myInfluencedVehicles in the previous step but not in the current step todo refactor
    //     for (std::string vehID : lastStepInfluencedVehicles) {
    //         myInfluencedVehicles.erase(vehID);
    //         Parameterised::Map::iterator it = myInfluencedTypes.find(vehID);
    //         MSVehicle* veh2 = dynamic_cast<MSVehicle*>(vc.getVehicle(vehID));
    //         if (veh2 != nullptr && it != myInfluencedTypes.end()) {
    //             // The vehicle gets back its old VehicleType after the emergency vehicle have passed them
    //             resetVehicle(veh2, it->second);
    //         }
    //     }
    //
    //     for (MSVehicle* veh2 : upcomingVehicles) {
    //         assert(veh2 != nullptr);
    //         if (veh2->getLane() == nullptr) {
    //             continue;
    //         }
    //         if (std::find(upcomingEdges.begin(), upcomingEdges.end(), &veh2->getLane()->getEdge()) != upcomingEdges.end()) {
    //             if (veh2->getDevice(typeid(MSDevice_Bluelight)) != nullptr) {
    //                 // emergency vehicles should not react
    //                 continue;
    //             }
    //             const int numLanes = (int)veh2->getLane()->getEdge().getNumLanes();
    //             // make sure that vehicles are still building the rescue lane as they might have moved to a new edge or changed lanes
    //             if (myInfluencedVehicles.count(veh2->getID()) > 0) {
    //                 // Vehicle gets a new Vehicletype to change the alignment and the lanechange options
    //                 MSVehicleType& t = veh2->getSingularType();
    //                 // Setting the lateral alignment to build a rescue lane
    //                 LatAlignmentDefinition align = LatAlignmentDefinition::RIGHT;
    //                 if (veh2->getLane()->getIndex() == numLanes - 1) {
    //                     align = LatAlignmentDefinition::LEFT;
    //                 }
    //                 t.setPreferredLateralAlignment(align);
    // #ifdef DEBUG_BLUELIGHT_RESCUELANE
    //                 std::cout << "Refresh alignment for vehicle: " << veh2->getID()
    //                           << " laneIndex=" << veh2->getLane()->getIndex() << " numLanes=" << numLanes
    //                           << " alignment=" << toString(align) << "\n";
    // #endif
    //             }
    //
    //             double distanceDelta = veh.getPosition().distanceTo(veh2->getPosition());
    //             //emergency vehicle has to slow down when entering the rescue lane
    //             if (distanceDelta <= 10 && veh.getID() != veh2->getID() && myInfluencedVehicles.count(veh2->getID()) > 0 && veh2->getSpeed() < 1) {
    //                 // set ev speed to 20 km/h 0 5.56 m/s
    //                 const double redSpeed = ego.getVehicleType().getParameter().getJMParam(SUMO_ATTR_JM_DRIVE_RED_SPEED, 5.56);
    //                 std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //                 speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep(), veh.getSpeed()));
    //                 speedTimeLine.push_back(std::make_pair(MSNet::getInstance()->getCurrentTimeStep() + TIME2STEPS(2), redSpeed));
    //                 redLight.setSpeedTimeLine(speedTimeLine);
    //             }
    //
    //             // the perception of the sound of the siren should be around 25 meters
    //             // todo only vehicles in front of the emergency vehicle should react
    //             if (distanceDelta <= myReactionDist && veh.getID() != veh2->getID() && myInfluencedVehicles.count(veh2->getID()) == 0) {
    //                 // only a percentage of vehicles should react to the emergency vehicle to make the behaviour more realistic
    //                 double reaction = RandHelper::rand();
    //                 MSVehicle::Influencer& lanechange = veh2->getInfluencer();
    //
    //                 //other vehicle should not use the rescue lane so they should not make any lane changes
    //                 lanechange.setLaneChangeMode(1605);//todo change lane back
    //                 // the vehicles should react according to the distance to the emergency vehicle taken from real world data
    //                 double reactionProb = (
    //                                           distanceDelta < myHolder.getFloatParam("device.bluelight.near-dist", false, 12.5)
    //                                           ? myHolder.getFloatParam("device.bluelight.reaction-prob-near", false, 0.577)
    //                                           : myHolder.getFloatParam("device.bluelight.reaction-prob-far", false, 0.189));
    //                 // todo works only for one second steps
    //                 //std::cout << SIMTIME << " veh2=" << veh2->getID() << " distanceDelta=" << distanceDelta << " reaction=" << reaction << " reactionProb=" << reactionProb << "\n";
    //                 if (veh2->isActionStep(SIMSTEP) && reaction < reactionProb * veh2->getActionStepLengthSecs()) {
    //                     myInfluencedVehicles.insert(veh2->getID());
    //                     myInfluencedTypes.insert(std::make_pair(veh2->getID(), veh2->getVehicleType().getID()));
    //                     if (myMinGapFactor != 1.) {
    //                         // TODO this is a permanent change to the vtype!
    //                         MSNet::getInstance()->getVehicleControl().getVType(veh2->getVehicleType().getID())->getCarFollowModel().setCollisionMinGapFactor(myMinGapFactor);
    //                     }
    //
    //                     // Vehicle gets a new Vehicletype to change the alignment and the lanechange options
    //                     MSVehicleType& t = veh2->getSingularType();
    //                     // Setting the lateral alignment to build a rescue lane
    //                     LatAlignmentDefinition align = LatAlignmentDefinition::RIGHT;
    //                     if (veh2->getLane()->getIndex() == numLanes - 1) {
    //                         align = LatAlignmentDefinition::LEFT;
    //                     }
    //                     t.setPreferredLateralAlignment(align);
    //                     t.setMinGap(t.getMinGap() * myMinGapFactor);
    //                     const_cast<SUMOVTypeParameter&>(t.getParameter()).jmParameter[SUMO_ATTR_JM_STOPLINE_GAP] = toString(myMinGapFactor);
    //                     // disable strategic lane-changing
    // #ifdef DEBUG_BLUELIGHT_RESCUELANE
    //                     std::cout << SIMTIME << " device=" << getID() << " formingRescueLane=" << veh2->getID()
    //                               << " laneIndex=" << veh2->getLane()->getIndex() << " numLanes=" << numLanes
    //                               << " alignment=" << toString(align) << "\n";
    // #endif
    //                     std::vector<std::string> influencedBy = StringTokenizer(veh2->getParameter().getParameter(INFLUENCED_BY, "")).getVector();
    //                     if (std::find(influencedBy.begin(), influencedBy.end(), myHolder.getID()) == influencedBy.end()) {
    //                         influencedBy.push_back(myHolder.getID());
    //                         const_cast<SUMOVehicleParameter&>(veh2->getParameter()).setParameter(INFLUENCED_BY, toString(influencedBy));
    //                     }
    //                     veh2->getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_STRATEGIC_PARAM), "-1");
    //                 }
    //             }
    //
    //         } else { //if vehicle is passed all vehicles which had to react should get their state back after they leave the communication range
    //             if (myInfluencedVehicles.count(veh2->getID()) > 0) {
    //                 double distanceDelta = veh.getPosition().distanceTo(veh2->getPosition());
    //                 if (distanceDelta > myReactionDist && veh.getID() != veh2->getID()) {
    //                     myInfluencedVehicles.erase(veh2->getID());
    //                     Parameterised::Map::iterator it = myInfluencedTypes.find(veh2->getID());
    //                     if (it != myInfluencedTypes.end()) {
    //                         // The vehicle gets back its old VehicleType after the emergency vehicle have passed them
    //                         resetVehicle(veh2, it->second);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // make upcoming junction foes slow down
    //     for (MSLink* link : upcomingLinks) {
    //         auto avi = link->getApproaching(&ego);
    //         MSLink::BlockingFoes blockingFoes;
    //         link->opened(avi.arrivalTime, avi.arrivalSpeed, avi.arrivalSpeed, ego.getLength(),
    //                      0, ego.getCarFollowModel().getMaxDecel(), ego.getWaitingTime(), ego.getLateralPositionOnLane(), &blockingFoes, true, &ego);
    //         const SUMOTime timeToArrival = avi.arrivalTime - SIMSTEP;
    //         for (const SUMOTrafficObject* foe : blockingFoes) {
    //             if (!foe->isVehicle()) {
    //                 continue;
    //             }
    //             const double dist = ego.getPosition().distanceTo2D(foe->getPosition());
    //             if (dist < myReactionDist) {
    //                 MSVehicle* microFoe = dynamic_cast<MSVehicle*>(const_cast<SUMOTrafficObject*>(foe));
    //                 if (microFoe->getDevice(typeid(MSDevice_Bluelight)) != nullptr) {
    //                     // emergency vehicles should not react
    //                     continue;
    //                 }
    //                 const double timeToBrake = foe->getSpeed() / SUMOVTypeParameter::getDefaultDecel();
    //                 if (timeToArrival < TIME2STEPS(timeToBrake + 1)) {
    //                     // trigger emergency braking
    //                     const double decel = 0.5 * foe->getSpeed() * foe->getSpeed() / avi.dist;
    //                     std::vector<std::pair<SUMOTime, double> > speedTimeLine;
    //                     speedTimeLine.push_back(std::make_pair(SIMSTEP, foe->getSpeed()));
    //                     speedTimeLine.push_back(std::make_pair(SIMSTEP + TIME2STEPS(foe->getSpeed() / decel), 0));
    //                     microFoe->getInfluencer().setSpeedTimeLine(speedTimeLine);
    //                     //std::cout << SIMTIME << " foe=" << foe->getID() << " timeToBrake=" << timeToBrake << " timeToArrival=" << STEPS2TIME(timeToArrival) << " decel=" << decel << "\n";
    //                 }
    //             }
    //         }
    //     }
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_move(&mut self, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
        todo!("port MSDevice_Bluelight::notifyMove")
    }

    /// C++: `bool MSDevice_Bluelight::notifyEnter(SUMOTrafficObject &, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves departure info on insertion
    // bool
    // MSDevice_Bluelight::notifyEnter(SUMOTrafficObject& veh, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    //     UNUSED_PARAMETER(veh);
    // #ifdef DEBUG_BLUELIGHT
    //     std::cout << SIMTIME << " device '" << getID() << "' notifyEnter: reason=" << toString(reason) << " enteredLane=" << Named::getIDSecure(enteredLane)  << "\n";
    // #else
    //     UNUSED_PARAMETER(reason);
    //     UNUSED_PARAMETER(enteredLane);
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_enter(&mut self, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        true // keep the device
    }

    /// C++: `bool MSDevice_Bluelight::notifyLeave(SUMOTrafficObject &, double, MSMoveReminder::Notification, const MSLane *)` [virtual]
    /// Saves arrival info
    // bool
    // MSDevice_Bluelight::notifyLeave(SUMOTrafficObject& veh, double /*lastPos*/, MSMoveReminder::Notification reason, const MSLane* enteredLane) {
    //     UNUSED_PARAMETER(veh);
    // #ifdef DEBUG_BLUELIGHT
    //     std::cout << SIMTIME << " device '" << getID() << "' notifyLeave: reason=" << toString(reason) << " approachedLane=" << Named::getIDSecure(enteredLane) << "\n";
    // #else
    //     UNUSED_PARAMETER(reason);
    //     UNUSED_PARAMETER(enteredLane);
    // #endif
    //     return true; // keep the device
    // }
    #[allow(unused_variables)]
    pub fn notify_leave(&mut self, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
        true // keep the device
    }

    /// C++: `const std::string MSDevice_Bluelight::deviceName() const` [const, virtual]
    /// return the name for this type of device
    // const std::string deviceName() const {
    //         return "bluelight";
    //     }
    #[allow(unused_variables)]
    pub fn device_name(&self) -> String {
        "bluelight".to_string()
    }

    /// C++: `std::string MSDevice_Bluelight::getParameter(const std::string &) const` [const, virtual]
    /// try to retrieve the given parameter from this device. Throw exception for unsupported key
    // std::string
    // MSDevice_Bluelight::getParameter(const std::string& key) const {
    //     if (key == "reactiondist") {
    //         return toString(myReactionDist);
    //     }
    //     throw InvalidArgument("Parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    // }
    #[allow(unused_variables)]
    pub fn get_parameter(&self, key: &str) -> String {
        if key == "reactiondist" {
            return utils::common::to_string::f64_to_string(self.myReactionDist);
        }
        // shortcut: C++ throws InvalidArgument; the port panics with the same text because overrides return String, upgrade when device getParameter returns Result
        panic!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())
    }

    /// C++: `void MSDevice_Bluelight::setParameter(const std::string &, const std::string &)` [virtual]
    /// try to set the given parameter for this device. Throw exception for unsupported key
    // void
    // MSDevice_Bluelight::setParameter(const std::string& key, const std::string& value) {
    //     double doubleValue;
    //     try {
    //         doubleValue = StringUtils::toDouble(value);
    //     } catch (NumberFormatException&) {
    //         throw InvalidArgument("Setting parameter '" + key + "' requires a number for device of type '" + deviceName() + "'");
    //     }
    //     if (key == "reactiondist") {
    //         myReactionDist = doubleValue;
    //     } else {
    //         throw InvalidArgument("Setting parameter '" + key + "' is not supported for device of type '" + deviceName() + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_parameter(&mut self, key: &str, value: &str) {
        let doubleValue = match utils::common::string_utils::to_double(value) {
            Ok(d) => d,
            // shortcut: C++ throws InvalidArgument; the port panics with the same text, upgrade when device setParameter returns Result
            Err(_) => panic!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()),
        };
        if key == "reactiondist" {
            self.myReactionDist = doubleValue;
        } else {
            panic!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name());
        }
    }

    /// C++: `void MSDevice_Bluelight::generateOutput(OutputDevice *) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSDevice_Bluelight::generateOutput(OutputDevice* tripinfoOut) const {
    //     if (tripinfoOut != nullptr) {
    //         tripinfoOut->openTag("bluelight");
    //         tripinfoOut->closeTag();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn generate_output(&self, tripinfoOut: Ptr<OutputDevice>) {
        if !tripinfoOut.is_null() {
            tripinfoOut.m().open_tag("bluelight");
            tripinfoOut.m().close_tag("");
        }
    }

    /// C++: `void MSDevice_Bluelight::MSDevice_Bluelight(SUMOVehicle &, const std::string &, const double, const double)` [private]
    /// Constructor
    // MSDevice_Bluelight::MSDevice_Bluelight(SUMOVehicle& holder, const std::string& id,
    //                                        const double reactionDist, const double minGapFactor) :
    //     MSVehicleDevice(holder, id),
    //     myReactionDist(reactionDist),
    //     myMinGapFactor(minGapFactor) {
    // #ifdef DEBUG_BLUELIGHT
    //     std::cout << SIMTIME << " initialized device '" << id << "' with myReactionDist=" << myReactionDist << "\n";
    // #endif
    //     // set only once to permit TraCI override
    //     MSVehicle& veh = dynamic_cast<MSVehicle&>(holder);
    //     veh.getInfluencer().setSpeedMode(7);
    // }
    #[allow(unused_variables)]
    pub fn new(holder: &mut SUMOVehicle, id: &str, reactionDist: f64, minGapFactor: f64) -> Self {
        unreachable!("MSDevice_Bluelight copy constructor is private and undefined in C++")
    }

    /// C++: `void MSDevice_Bluelight::resetVehicle(MSVehicle *, const std::string &)` [private]
    /// restore type of influenced vehicle
    // void
    // MSDevice_Bluelight::resetVehicle(MSVehicle* veh2, const std::string& targetTypeID) {
    //     MSVehicleType* targetType = MSNet::getInstance()->getVehicleControl().getVType(targetTypeID);
    //     //targetType is nullptr if the vehicle type has already changed to its old vehicleType
    //     if (targetType != nullptr) {
    // #ifdef DEBUG_BLUELIGHT_RESCUELANE
    //         std::cout << SIMTIME << " device=" << getID() << " reset " << veh2->getID() << "\n";
    // #endif
    //
    //         std::vector<std::string> influencedBy = StringTokenizer(veh2->getParameter().getParameter(INFLUENCED_BY, "")).getVector();
    //         auto it = std::find(influencedBy.begin(), influencedBy.end(), myHolder.getID());
    //         if (it != influencedBy.end()) {
    //             influencedBy.erase(it);
    //             const_cast<SUMOVehicleParameter&>(veh2->getParameter()).setParameter(INFLUENCED_BY, toString(influencedBy));
    //         }
    //         if (influencedBy.size() == 0) {
    //             veh2->replaceVehicleType(targetType);
    //             veh2->getLaneChangeModel().setParameter(toString(SUMO_ATTR_LCA_STRATEGIC_PARAM),
    //                                                     targetType->getParameter().getLCParamString(SUMO_ATTR_LCA_STRATEGIC_PARAM, "1"));
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reset_vehicle(&mut self, veh2: Ptr<MSVehicle>, targetTypeID: &str) {
        todo!("port MSDevice_Bluelight::resetVehicle")
    }

    /// C++: `void MSDevice_Bluelight::MSDevice_Bluelight(const MSDevice_Bluelight &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Bluelight) -> Self {
        todo!("port MSDevice_Bluelight::MSDevice_Bluelight")
    }

    /// C++: `MSDevice_Bluelight & MSDevice_Bluelight::operator=(const MSDevice_Bluelight &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Bluelight) -> &mut MSDevice_Bluelight {
        unreachable!("MSDevice_Bluelight assignment operator is private and undefined in C++")
    }

}

