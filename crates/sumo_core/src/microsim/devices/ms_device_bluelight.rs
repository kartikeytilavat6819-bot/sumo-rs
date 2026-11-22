//! Port of microsim/devices/MSDevice_Bluelight.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::std_defs::INVALID_DOUBLE;
use utils::common::rand_helper::rand;
use utils::common::string_tokenizer::StringTokenizer;
use utils::common::to_string::f64_to_string;
use utils::options::options_cont::with_options;

/// C++ `#define INFLUENCED_BY "rescueLane"` (MSDevice_Bluelight.cpp)
const INFLUENCED_BY: &str = "rescueLane";

/// C++ `toString(SumoXMLAttr)`: the XML name of an attribute from the ATTRS table
fn sumo_attr_name(attr: SumoXMLAttr) -> &'static str {
    ATTRS.iter().find(|(_, v)| *v == attr).map(|(n, _)| *n).unwrap_or("")
}

/// A device which collects info on the vehicle trip (mainly on departure and arrival)
/// C++ class `MSDevice_Bluelight` (microsim/devices/MSDevice_Bluelight.h)
#[repr(C)]
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
        let equipped = with_options(|oc| MSDevice::equipped_by_default_assignment_options(oc, "bluelight", v.get_id(), v.get_parameter(), v.get_vehicle_type().get_parameter(), false, false));
        if equipped {
            if use_meso_sim() {
                write_warningf!("bluelight device is not compatible with mesosim (ignored for vehicle '%')", v.get_id());
            } else {
                let id = format!("bluelight_{}", v.get_id());
                let reactionDist = v.get_float_param("device.bluelight.reactiondist", false, INVALID_DOUBLE, true);
                let minGapFactor = v.get_float_param("device.bluelight.mingapfactor", false, INVALID_DOUBLE, true);
                // shortcut: the device is leaked like the C++ new, so the vehicle's device list keeps a stable pointer; upgrade when MSVehicle owns its devices as Box
                let device: &mut MSDevice_Bluelight = Box::leak(Box::new(MSDevice_Bluelight::new(v, &id, reactionDist, minGapFactor)));
                into.push(Ptr::from_mut(device).cast::<MSVehicleDevice>());
            }
        }
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
        //violate red lights  this only need to be done once so shift it todo
        let ego = Ptr::from_mut(veh).cast::<MSVehicle>();
        let egoTO = ego.cast::<SUMOTrafficObject>();
        let redLight = Ptr::from_mut(ego.m().get_influencer());
        let net = MSNet::get_instance();
        let simstep = net.get_current_time_step();
        let vMax = ego.get_lane().get_vehicle_max_speed(egoTO);
        if ego.get_speed() < 0.5 * vMax {
            // advance as far as possible (assume vehicles will keep moving out of the way)
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_STRATEGIC_PARAM), "-1");
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD), "0");
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME), "0");
            // not supported by the current laneChangeModel: set_parameter panics with InvalidArgument, which is caught here as in C++
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_MINGAP_LAT), "0")));
        } else {
            // restore defaults
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_STRATEGIC_PARAM), &ego.get_vehicle_type().get_parameter().get_lc_param_string(SUMO_ATTR_LCA_STRATEGIC_PARAM, "1"));
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD), &ego.get_vehicle_type().get_parameter().get_lc_param_string(SUMO_ATTR_LCA_SPEEDGAIN_LOOKAHEAD, "5"));
            ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME), &ego.get_vehicle_type().get_parameter().get_lc_param_string(SUMO_ATTR_LCA_SPEEDGAIN_REMAIN_TIME, "20"));
            // not supported by the current laneChangeModel: set_parameter panics with InvalidArgument, which is caught here as in C++
            let minGapLat = f64_to_string(ego.get_vehicle_type().get_min_gap_lat());
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| ego.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_MINGAP_LAT), &minGapLat)));
        }
        // build a rescue lane for all vehicles on the route of the emergency vehicle within the range of the siren
        let vc = net.m().get_vehicle_control();
        // use edges on the way of the emergency vehicle
        let mut upcomingEdges: Vec<Ptr<MSEdge>> = Vec::new();
        let mut upcomingVehicles: BTreeMap<String, Ptr<MSVehicle>> = BTreeMap::new();
        let mut lastStepInfluencedVehicles: BTreeSet<String> = self.myInfluencedVehicles.clone();
        let mut upcomingLinks: Vec<Ptr<MSLink>> = Vec::new();
        let mut affectedJunctionDist = ego.get_position_on_lane() + self.myReactionDist;
        for l in ego.get_upcoming_lanes_until(self.myReactionDist) {
            upcomingEdges.push(Ptr::from_mut(l.get_edge()));

            affectedJunctionDist -= l.get_length();
            if affectedJunctionDist > 0. && l.is_internal() {
                upcomingLinks.push(l.get_incoming_lanes()[0].viaLink);
            }
        }

        for e in upcomingEdges.iter() {
            //inform all vehicles on upcomingEdges
            for v in e.get_vehicles() {
                upcomingVehicles.entry(v.get_id().to_string()).or_insert(v.cast::<MSVehicle>());
                lastStepInfluencedVehicles.remove(v.get_id());
            }
        }
        // reset all vehicles that were in myInfluencedVehicles in the previous step but not in the current step todo refactor
        for vehID in lastStepInfluencedVehicles.iter().cloned().collect::<Vec<String>>() {
            self.myInfluencedVehicles.remove(&vehID);
            let it = self.myInfluencedTypes.get(&vehID).cloned();
            let veh2 = vc.get_vehicle(&vehID).cast::<MSVehicle>();
            if !veh2.is_null() {
                if let Some(targetType) = it {
                    // The vehicle gets back its old VehicleType after the emergency vehicle have passed them
                    self.reset_vehicle(veh2, &targetType);
                }
            }
        }

        for veh2 in upcomingVehicles.values().copied() {
            debug_assert!(!veh2.is_null());
            if veh2.get_lane().is_null() {
                continue;
            }
            if upcomingEdges.contains(&Ptr::from_mut(veh2.get_lane().get_edge())) {
                if !MSDevice::get_device_by_name(&veh2.myDevices, "bluelight").is_null() {
                    // emergency vehicles should not react
                    continue;
                }
                let numLanes = veh2.get_lane().get_edge().get_num_lanes();
                // make sure that vehicles are still building the rescue lane as they might have moved to a new edge or changed lanes
                if self.myInfluencedVehicles.contains(veh2.get_id()) {
                    // Vehicle gets a new Vehicletype to change the alignment and the lanechange options
                    let t = veh2.m().get_singular_type();
                    // Setting the lateral alignment to build a rescue lane
                    let mut align = LatAlignmentDefinition::RIGHT;
                    if veh2.get_lane().get_index() == numLanes - 1 {
                        align = LatAlignmentDefinition::LEFT;
                    }
                    t.set_preferred_lateral_alignment(&align, 0.0);
                }

                let distanceDelta = veh.get_position(0.).distance_to(&veh2.get_position(0.));
                //emergency vehicle has to slow down when entering the rescue lane
                if distanceDelta <= 10. && veh.get_id() != veh2.get_id() && self.myInfluencedVehicles.contains(veh2.get_id()) && veh2.get_speed() < 1. {
                    // set ev speed to 20 km/h 0 5.56 m/s
                    let redSpeed = ego.get_vehicle_type().get_parameter().get_jm_param(SUMO_ATTR_JM_DRIVE_RED_SPEED, 5.56);
                    let speedTimeLine: Vec<(SUMOTime, f64)> = vec![(simstep, veh.get_speed()), (simstep + time2steps(2.), redSpeed)];
                    redLight.m().set_speed_time_line(&speedTimeLine);
                }

                // the perception of the sound of the siren should be around 25 meters
                // todo only vehicles in front of the emergency vehicle should react
                if distanceDelta <= self.myReactionDist && veh.get_id() != veh2.get_id() && !self.myInfluencedVehicles.contains(veh2.get_id()) {
                    // only a percentage of vehicles should react to the emergency vehicle to make the behaviour more realistic
                    let reaction = rand();
                    let lanechange = veh2.m().get_influencer();

                    //other vehicle should not use the rescue lane so they should not make any lane changes
                    lanechange.set_lane_change_mode(1605); //todo change lane back
                    // the vehicles should react according to the distance to the emergency vehicle taken from real world data
                    let holderID = self.myHolder.get_id().to_string();
                    let reactionProb = if distanceDelta < self.myHolder.get_float_param("device.bluelight.near-dist", false, 12.5, true) {
                        self.myHolder.get_float_param("device.bluelight.reaction-prob-near", false, 0.577, true)
                    } else {
                        self.myHolder.get_float_param("device.bluelight.reaction-prob-far", false, 0.189, true)
                    };
                    // todo works only for one second steps
                    if veh2.is_action_step(simstep) && reaction < reactionProb * veh2.get_action_step_length_secs() {
                        self.myInfluencedVehicles.insert(veh2.get_id().to_string());
                        // std::map::insert keeps an existing entry
                        self.myInfluencedTypes.entry(veh2.get_id().to_string()).or_insert(veh2.get_vehicle_type().get_id().to_string());
                        if self.myMinGapFactor != 1. {
                            // TODO this is a permanent change to the vtype!
                            let vt = vc.get_v_type(veh2.get_vehicle_type().get_id(), Ptr::null(), false);
                            vt.m().get_car_follow_model_2().set_collision_min_gap_factor(self.myMinGapFactor);
                        }

                        // Vehicle gets a new Vehicletype to change the alignment and the lanechange options
                        let t = veh2.m().get_singular_type();
                        // Setting the lateral alignment to build a rescue lane
                        let mut align = LatAlignmentDefinition::RIGHT;
                        if veh2.get_lane().get_index() == numLanes - 1 {
                            align = LatAlignmentDefinition::LEFT;
                        }
                        t.set_preferred_lateral_alignment(&align, 0.0);
                        let minGap = t.get_min_gap() * self.myMinGapFactor;
                        t.set_min_gap(&minGap);
                        // C++ const_cast<SUMOVTypeParameter&>(t.getParameter()).jmParameter[SUMO_ATTR_JM_STOPLINE_GAP] = toString(myMinGapFactor)
                        t.myParameter.jmParameter.insert(SUMO_ATTR_JM_STOPLINE_GAP, f64_to_string(self.myMinGapFactor));
                        // disable strategic lane-changing
                        let mut tokenizer = StringTokenizer::new(&veh2.get_parameter().get_parameter(INFLUENCED_BY, ""));
                        let mut influencedBy: Vec<String> = tokenizer.get_vector();
                        if !influencedBy.contains(&holderID) {
                            influencedBy.push(holderID.clone());
                            veh2.m().get_parameter_mutable().set_parameter(INFLUENCED_BY, &influencedBy.join(" "));
                        }
                        veh2.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_STRATEGIC_PARAM), "-1");
                    }
                }
            } else {
                //if vehicle is passed all vehicles which had to react should get their state back after they leave the communication range
                if self.myInfluencedVehicles.contains(veh2.get_id()) {
                    let distanceDelta = veh.get_position(0.).distance_to(&veh2.get_position(0.));
                    if distanceDelta > self.myReactionDist && veh.get_id() != veh2.get_id() {
                        self.myInfluencedVehicles.remove(veh2.get_id());
                        let it = self.myInfluencedTypes.get(veh2.get_id()).cloned();
                        if let Some(targetType) = it {
                            // The vehicle gets back its old VehicleType after the emergency vehicle have passed them
                            self.reset_vehicle(veh2, &targetType);
                        }
                    }
                }
            }
        }
        // make upcoming junction foes slow down
        for link in upcomingLinks {
            let avi = link.get_approaching(ego.cast::<SUMOVehicle>());
            let mut blockingFoes: MSLink_BlockingFoes = Vec::new();
            link.opened(avi.arrivalTime, avi.arrivalSpeed, avi.arrivalSpeed, ego.get_length(), 0., ego.get_car_follow_model().get_max_decel(), ego.get_waiting_time(false), ego.get_lateral_position_on_lane(), Ptr::from_mut(&mut blockingFoes), true, egoTO, -1.);
            let timeToArrival = avi.arrivalTime - simstep;
            for foe in blockingFoes.iter().copied() {
                if !foe.is_vehicle() {
                    continue;
                }
                let dist = ego.get_position(0.).distance_to2d(&foe.get_position(0.));
                if dist < self.myReactionDist {
                    let microFoe = foe.cast::<MSVehicle>();
                    if !MSDevice::get_device_by_name(&microFoe.myDevices, "bluelight").is_null() {
                        // emergency vehicles should not react
                        continue;
                    }
                    let timeToBrake = foe.get_speed() / SUMOVTypeParameter::get_default_decel(SVC_IGNORING);
                    if timeToArrival < time2steps(timeToBrake + 1.) {
                        // trigger emergency braking
                        let decel = 0.5 * foe.get_speed() * foe.get_speed() / avi.dist;
                        let speedTimeLine: Vec<(SUMOTime, f64)> = vec![(simstep, foe.get_speed()), (simstep + time2steps(foe.get_speed() / decel), 0.)];
                        microFoe.m().get_influencer().set_speed_time_line(&speedTimeLine);
                    }
                }
            }
        }
        true // keep the device
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
    pub fn get_parameter(&self, key: &str) -> Result<String, ProcessError> {
        if key == "reactiondist" {
            return Ok(utils::common::to_string::f64_to_string(self.myReactionDist));
        }
        Err(ProcessError::InvalidArgument(format!("Parameter '{}' is not supported for device of type '{}'", key, self.device_name())))
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
    pub fn set_parameter(&mut self, key: &str, value: &str) -> Result<(), ProcessError> {
        let doubleValue = match utils::common::string_utils::to_double(value) {
            Ok(d) => d,
            Err(_) => return Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' requires a number for device of type '{}'", key, self.device_name()))),
        };
        if key == "reactiondist" {
            self.myReactionDist = doubleValue;
            Ok(())
        } else {
            Err(ProcessError::InvalidArgument(format!("Setting parameter '{}' is not supported for device of type '{}'", key, self.device_name())))
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
        fn mr_notify_enter(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Bluelight>().m().notify_enter(veh, reason, enteredLane)
        }
        fn mr_notify_move(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, oldPos: f64, newPos: f64, newSpeed: f64) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Bluelight>().m().notify_move(veh, oldPos, newPos, newSpeed)
        }
        fn mr_notify_leave(mr: &mut MSMoveReminder, veh: &mut SUMOTrafficObject, lastPos: f64, reason: MSMoveReminder_Notification, enteredLane: Ptr<MSLane>) -> bool {
            Ptr::from_mut(mr).cast::<MSDevice_Bluelight>().m().notify_leave(veh, lastPos, reason, enteredLane)
        }
        static MS_MOVE_REMINDER_VT: MSMoveReminderVt = MSMoveReminderVt {
            notify_enter: mr_notify_enter,
            notify_move: mr_notify_move,
            notify_leave: mr_notify_leave,
        };
        let mut this = Self::new_device(holder, id, reactionDist, minGapFactor);
        this.base.base.vt = &MS_MOVE_REMINDER_VT;
        this
    }

    fn new_device(holder: &mut SUMOVehicle, id: &str, reactionDist: f64, minGapFactor: f64) -> Self {
        let mut device = MSDevice_Bluelight {
            base: MSVehicleDevice::new(holder, id),
            myInfluencedVehicles: BTreeSet::new(),
            myInfluencedTypes: utils::common::parameterised::ParameterisedMap::new(),
            myReactionDist: reactionDist,
            myMinGapFactor: minGapFactor,
        };
        // set only once to permit TraCI override
        let veh = Ptr::from_mut(holder).cast::<MSVehicle>();
        veh.m().get_influencer().set_speed_mode(7);
        device
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
        let targetType = MSNet::get_instance().m().get_vehicle_control().get_v_type(targetTypeID, Ptr::null(), false);
        //targetType is nullptr if the vehicle type has already changed to its old vehicleType
        if !targetType.is_null() {
            let mut tokenizer = utils::common::string_tokenizer::StringTokenizer::new(&veh2.get_parameter().get_parameter(INFLUENCED_BY, ""));
            let mut influencedBy: Vec<String> = tokenizer.get_vector();
            let holderID = self.myHolder.get_id().to_string();
            if let Some(pos) = influencedBy.iter().position(|s| *s == holderID) {
                influencedBy.remove(pos);
                // C++ const_cast<SUMOVehicleParameter&>(veh2->getParameter()).setParameter(...)
                veh2.m().get_parameter_mutable().set_parameter(INFLUENCED_BY, &influencedBy.join(" "));
            }
            if influencedBy.is_empty() {
                veh2.m().replace_vehicle_type(targetType);
                veh2.m().get_lane_change_model().set_parameter(sumo_attr_name(SUMO_ATTR_LCA_STRATEGIC_PARAM), &targetType.get_parameter().get_lc_param_string(SUMO_ATTR_LCA_STRATEGIC_PARAM, "1"));
            }
        }
    }

    /// C++: `void MSDevice_Bluelight::MSDevice_Bluelight(const MSDevice_Bluelight &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSDevice_Bluelight) -> Self {
        panic!("MSDevice_Bluelight copy constructor is private and undefined in C++; it cannot be called")
    }

    /// C++: `MSDevice_Bluelight & MSDevice_Bluelight::operator=(const MSDevice_Bluelight &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSDevice_Bluelight) -> &mut MSDevice_Bluelight {
        unreachable!("MSDevice_Bluelight assignment operator is private and undefined in C++")
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::microsim::ms_vehicle::MSVehicle_Influencer;

    // MSDevice_Bluelight's constructor calls setSpeedMode(7) on its holder.
    // Speed mode bits (MSVehicle.cpp Influencer::setSpeedMode): 1=safe velocity,
    // 2=max acceleration, 4=max deceleration, 8=respect junction priority,
    // 16=emergency brake at red. Mode 7 must leave the last two disabled so the
    // emergency vehicle runs red lights (MSVehicle::ignoreRed / influencerPrio).
    #[test]
    fn bluelight_speed_mode_7_disables_red_and_junction_priority() {
        let mut inf = MSVehicle_Influencer::new();
        assert!(inf.get_respect_junction_priority());
        assert!(inf.get_emergency_brake_red_light());
        inf.set_speed_mode(7);
        assert!(inf.consider_safe_velocity());
        assert!(inf.myConsiderMaxAcceleration);
        assert!(inf.consider_max_deceleration());
        assert!(!inf.get_respect_junction_priority());
        assert!(!inf.get_emergency_brake_red_light());
    }

    #[test]
    fn bluelight_device_name_from_id() {
        assert_eq!(MSDevice::get_device_name("bluelight_rescue"), "bluelight");
    }

    // C++ MSDevice_Bluelight.cpp:255-259: influencedBy is a space-separated
    // StringTokenizer list of holder IDs; resetVehicle erases this holder and
    // restores the original vtype only when nothing else influenced the car.
    #[test]
    fn influenced_by_list_removal_gates_type_reset() {
        let holder = "rescue";
        // one influencer: removing it empties the list -> reset
        let mut v = StringTokenizer::new(&format!("{}", holder)).get_vector();
        assert_eq!(v, vec![holder]);
        if let Some(pos) = v.iter().position(|s| *s == holder) { v.remove(pos); }
        assert!(v.is_empty(), "single influencer must allow the vtype reset");

        // two influencers: removing one leaves another -> keep singular type
        let mut v = StringTokenizer::new(&format!("rescue_other {}", holder)).get_vector();
        assert_eq!(v, vec!["rescue_other", holder]);
        if let Some(pos) = v.iter().position(|s| *s == holder) { v.remove(pos); }
        assert_eq!(v, vec!["rescue_other"], "another influencer must keep the singular type");
    }

    // C++ MSDevice_Bluelight.cpp:225-228: reactionProb uses the near-distance
    // threshold 12.5 m; near prob 0.577, far prob 0.189, multiplied by the
    // action-step length. The observed draws (gate trace) must select these.
    #[test]
    fn reaction_probability_threshold_and_defaults() {
        let near_dist = 12.5_f64;
        let prob_near = 0.577_f64;
        let prob_far = 0.189_f64;
        let step_secs = 1.0_f64;
        let prob = |dist: f64| if dist < near_dist { prob_near } else { prob_far };
        assert_eq!(prob(7.5), prob_near);
        assert_eq!(prob(12.49), prob_near);
        assert_eq!(prob(12.5), prob_far); // strict '<', as in C++
        assert_eq!(prob(19.7), prob_far);
        // draws from the official-matching gate trace
        assert!(0.0455 < prob(19.7) * step_secs);   // straight.4 at t=17
        assert!(0.0060 < prob(7.5) * step_secs);    // left.4 at t=19
        assert!(0.4662 < prob(7.5) * step_secs);    // right.4 at t=19
        assert!(0.4113 < prob(7.5) * step_secs);    // left.1 at t=24
    }

    // C++ uses std::set<std::string> (influenced IDs) and
    // std::map<std::string, std::string> (target vtype per ID): both iterate
    // in ID order and insert keeps the first value. BTreeMap/BTreeSet match.
    #[test]
    fn influenced_containers_are_id_ordered_and_keep_first_type() {
        let mut ids = BTreeSet::new();
        for id in ["straight.4", "left.0", "right.3"] { ids.insert(id.to_string()); }
        assert_eq!(ids.iter().map(String::as_str).collect::<Vec<_>>(),
                   vec!["left.0", "right.3", "straight.4"]);

        let mut types = utils::common::parameterised::ParameterisedMap::new();
        types.entry("left.0".to_string()).or_insert("car".to_string());
        types.entry("left.0".to_string()).or_insert("car@left.0".to_string());
        assert_eq!(types["left.0"], "car");
    }

    // C++ MSBaseVehicle::getSingularType builds the id "<vtype>@<vehicle id>";
    // MSDevice_Bluelight calls it on influenced vehicles to set the rescue-lane
    // alignment, producing ids like "car@straight.4".
    #[test]
    fn singular_type_id_convention() {
        let vtype = "car";
        let veh = "straight.4";
        assert_eq!(format!("{}@{}", vtype, veh), "car@straight.4");
        // the device writes StringUtils/myMinGapFactor (default 1) to jmParameter;
        // C++ toString(double, gPrecision=2) is fixed-point "1.00" (ToString.h:49-54).
        assert_eq!(f64_to_string(1.0_f64), "1.00");
    }
}

