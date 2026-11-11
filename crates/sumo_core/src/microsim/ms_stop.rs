//! Port of microsim/MSStop.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Definition of vehicle stop (position and duration)
/// C++ class `MSStop` (microsim/MSStop.h)
pub struct MSStop {
    /// The edge in the route to stop at
    pub edge: MSRouteIterator,
    /// The lane to stop at (microsim only)
    pub lane: Ptr<MSLane>,
    /// The segment to stop at (mesosim only)
    pub segment: Ptr<MESegment>,
    /// (Optional) bus stop if one is assigned to the stop
    pub busstop: Ptr<MSStoppingPlace>,
    /// (Optional) container stop if one is assigned to the stop
    pub containerstop: Ptr<MSStoppingPlace>,
    /// (Optional) parkingArea if one is assigned to the stop
    pub parkingarea: Ptr<MSParkingArea>,
    /// (Optional) charging station if one is assigned to the stop
    pub chargingStation: Ptr<MSStoppingPlace>,
    /// (Optional) overhead wire segment if one is assigned to the stop
    pub overheadWireSegment: Ptr<MSStoppingPlace>,
    /// The stop parameter
    pub pars: SUMOVehicleParameter_Stop,
    /// The stopping duration
    pub duration: SUMOTime,
    /// whether an arriving person lets the vehicle continue
    pub triggered: bool,
    /// whether an arriving container lets the vehicle continue
    pub containerTriggered: bool,
    /// whether coupling another vehicle (train) the vehicle continue
    pub joinTriggered: bool,
    /// Information whether the stop has been reached
    pub reached: bool,
    /// The number of still expected persons
    pub numExpectedPerson: i32,
    /// The number of still expected containers
    pub numExpectedContainer: i32,
    /// The time at which the vehicle is able to board another person
    pub timeToBoardNextPerson: SUMOTime,
    /// The time at which the vehicle is able to load another container
    pub timeToLoadNextContainer: SUMOTime,
    /// the maximum time at which persons may board this vehicle
    pub endBoarding: SUMOTime,
    /// whether this an opposite-direction stop
    pub isOpposite: bool,
    /// whether the decision to skip this stop has been made
    pub skipOnDemand: bool,
    /// whether the 'started' value was loaded from simulaton state
    pub startedFromState: bool,
    /// whehther the vehicle stopped despite having a waypoing
    pub waypointWithStop: bool,
    /// route indices on looped route that were skipped before this stop (after pior stop or depart edge)
    pub skips: Vec<i32>,
    /// the exact position when entering the stop (for state saving)
    pub entryPos: f64,
}

impl MSStop {
    /// C++: `void MSStop::MSStop(const SUMOVehicleParameter::Stop &)`
    // MSStop(const SUMOVehicleParameter::Stop& par) : pars(par) {}
    #[allow(unused_variables)]
    pub fn new(par: &SUMOVehicleParameter_Stop) -> Self {
        todo!("port MSStop::MSStop")
    }

    /// C++: `void MSStop::write(OutputDevice &) const` [const]
    /// Write the current stop configuration (used for state saving)
    // void
    // MSStop::write(OutputDevice& dev) const {
    //     SUMOVehicleParameter::Stop tmp = pars;
    //     tmp.duration = duration;
    //     if (!triggered && !containerTriggered) {
    //         // we are writing in the context of saveState. All required
    //         // transportables have entered and we must prevent the trigger condition
    //         // to be renewed on loading
    //         tmp.triggered = false;
    //         tmp.containerTriggered = false;
    //     }
    //     tmp.write(dev, false);
    //     // if the stop has already started but hasn't ended yet we are writing it in
    //     // the context of saveState (but we do not want to write the attribute twice
    //     if (pars.started >= 0 && (pars.parametersSet & STOP_STARTED_SET) == 0) {
    //         dev.writeAttr(SUMO_ATTR_STARTED, time2string(pars.started));
    //     }
    //     pars.writeParams(dev);
    //     dev.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn write(&self, dev: &mut OutputDevice) {
        todo!("port MSStop::write")
    }

    /// C++: `double MSStop::getEndPos(const SUMOVehicle &) const` [const]
    /// return halting position for upcoming stop;
    // double
    // MSStop::getEndPos(const SUMOVehicle& veh) const {
    //     const double brakePos = veh.getEdge() == getEdge() ? veh.getPositionOnLane() + veh.getBrakeGap() : 0;
    //     if ((pars.parametersSet & STOP_END_SET) != 0) {
    //         return pars.endPos;
    //     } else if (busstop != nullptr) {
    //         return busstop->getLastFreePos(veh, brakePos);
    //     } else if (containerstop != nullptr) {
    //         return containerstop->getLastFreePos(veh, brakePos);
    //     } else if (parkingarea != nullptr) {
    //         return parkingarea->getLastFreePos(veh, brakePos);
    //     } else if (chargingStation != nullptr) {
    //         return chargingStation->getLastFreePos(veh);
    //     } else if (overheadWireSegment != nullptr) {
    //         return overheadWireSegment->getLastFreePos(veh);
    //     }
    //     return pars.endPos;
    // }
    #[allow(unused_variables)]
    pub fn get_end_pos(&self, veh: &SUMOVehicle) -> f64 {
        todo!("port MSStop::getEndPos")
    }

    /// C++: `double MSStop::getReachedThreshold() const` [const]
    /// return startPos taking into account opposite stopping
    // double
    // MSStop::getReachedThreshold() const {
    //     return isOpposite ? lane->getOppositePos(pars.endPos) - (pars.endPos - pars.startPos) : pars.startPos;
    // }
    #[allow(unused_variables)]
    pub fn get_reached_threshold(&self) -> f64 {
        todo!("port MSStop::getReachedThreshold")
    }

    /// C++: `std::string MSStop::getDescription(bool) const` [const]
    /// get a short description for showing in the gui
    // std::string
    // MSStop::getDescription(bool nameOnly) const {
    //     std::string result;
    //     if (parkingarea != nullptr) {
    //         if (nameOnly) {
    //             return parkingarea->getID();
    //         }
    //         result = "parkingArea:" + parkingarea->getID();
    //     } else if (containerstop != nullptr) {
    //         if (nameOnly) {
    //             return containerstop->getID();
    //         }
    //         result = "containerStop:" + containerstop->getID();
    //     } else if (busstop != nullptr) {
    //         if (nameOnly) {
    //             return busstop->getID();
    //         }
    //         result = "busStop:" + busstop->getID();
    //     } else if (chargingStation != nullptr) {
    //         if (nameOnly) {
    //             return chargingStation->getID();
    //         }
    //         result = "chargingStation:" + chargingStation->getID();
    //     } else if (overheadWireSegment != nullptr) {
    //         if (nameOnly) {
    //             return overheadWireSegment->getID();
    //         }
    //         result = "overheadWireSegment:" + overheadWireSegment->getID();
    //     } else {
    //         if (nameOnly) {
    //             return "";
    //         }
    //         result = "lane:" + lane->getID() + " pos:" + toString(pars.endPos);
    //     }
    //     if (pars.actType != "") {
    //         result += " actType:" + pars.actType;
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_description(&self, nameOnly: bool) -> String {
        todo!("port MSStop::getDescription")
    }

    /// C++: `std::pair<std::string, SumoXMLTag> MSStop::getStoppingPlaceName() const` [const]
    /// return the name of the stopping place or an empty string
    // std::pair<std::string, SumoXMLTag>
    // MSStop::getStoppingPlaceName() const {
    //     if (busstop != nullptr && !busstop->getMyName().empty()) {
    //         return std::make_pair(busstop->getMyName(), SUMO_TAG_BUS_STOP);
    //     } else if (containerstop != nullptr && !containerstop->getMyName().empty()) {
    //         return std::make_pair(containerstop->getMyName(), SUMO_TAG_CONTAINER_STOP);
    //     } else if (parkingarea != nullptr && !parkingarea->getMyName().empty()) {
    //         return std::make_pair(parkingarea->getMyName(), SUMO_TAG_PARKING_AREA);
    //     } else if (chargingStation != nullptr && !chargingStation->getMyName().empty()) {
    //         return std::make_pair(chargingStation->getMyName(), SUMO_TAG_CHARGING_STATION);
    //     } else if (overheadWireSegment != nullptr && !overheadWireSegment->getMyName().empty()) {
    //         return std::make_pair(overheadWireSegment->getMyName(), SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //     }
    //     return std::make_pair("", SUMO_TAG_NOTHING);
    // }
    #[allow(unused_variables)]
    pub fn get_stopping_place_name(&self) -> (String, SumoXMLTag) {
        todo!("port MSStop::getStoppingPlaceName")
    }

    /// C++: `void MSStop::initPars(const SUMOVehicleParameter::Stop &)`
    /// initialize attributes from the given stop parameters
    // void
    // MSStop::initPars(const SUMOVehicleParameter::Stop& stopPar) {
    //     busstop = MSNet::getInstance()->getStoppingPlace(stopPar.busstop, SUMO_TAG_BUS_STOP);
    //     containerstop = MSNet::getInstance()->getStoppingPlace(stopPar.containerstop, SUMO_TAG_CONTAINER_STOP);
    //     parkingarea = static_cast<MSParkingArea*>(MSNet::getInstance()->getStoppingPlace(stopPar.parkingarea, SUMO_TAG_PARKING_AREA));
    //     chargingStation = MSNet::getInstance()->getStoppingPlace(stopPar.chargingStation, SUMO_TAG_CHARGING_STATION);
    //     overheadWireSegment = MSNet::getInstance()->getStoppingPlace(stopPar.overheadWireSegment, SUMO_TAG_OVERHEAD_WIRE_SEGMENT);
    //     duration = stopPar.durationDist.isVariable() ? TIME2STEPS(stopPar.durationDist.sample(MSRouteHandler::getParsingRNG())) : stopPar.duration;
    //     triggered = stopPar.triggered;
    //     containerTriggered = stopPar.containerTriggered;
    //     joinTriggered = stopPar.joinTriggered || stopPar.join != "";
    //     numExpectedPerson = (int)stopPar.awaitedPersons.size();
    //     numExpectedContainer = (int)stopPar.awaitedContainers.size();
    // }
    #[allow(unused_variables)]
    pub fn init_pars(&mut self, stopPar: &SUMOVehicleParameter_Stop) {
        todo!("port MSStop::initPars")
    }

    /// C++: `const MSEdge * MSStop::getEdge() const` [const]
    // const MSEdge*
    // MSStop::getEdge() const {
    //     if (lane != nullptr) {
    //         return &lane->getEdge();
    //     } else if (segment != nullptr) {
    //         return &segment->getEdge();
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        todo!("port MSStop::getEdge")
    }

    /// C++: `int MSStop::getStateFlagsOld() const` [const]
    /// return flags as used by Vehicle::getStopState
    // int
    // MSStop::getStateFlagsOld() const {
    //     return ((reached ? 1 : 0) + 2 * pars.getFlags());
    // }
    #[allow(unused_variables)]
    pub fn get_state_flags_old(&self) -> i32 {
        todo!("port MSStop::getStateFlagsOld")
    }

    /// C++: `SUMOTime MSStop::getMinDuration(SUMOTime) const` [const]
    /// return minimum stop duration when starting stop at time
    // SUMOTime
    // MSStop::getMinDuration(SUMOTime time) const {
    //     if (MSGlobals::gUseStopEnded && pars.ended >= 0) {
    //         return pars.ended - time;
    //     }
    //     if (pars.until >= 0) {
    //         if (duration == -1) {
    //             return pars.until - time;
    //         } else {
    //             return MAX2(duration, pars.until - time);
    //         }
    //     } else {
    //         return duration;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_min_duration(&self, time: SUMOTime) -> SUMOTime {
        todo!("port MSStop::getMinDuration")
    }

    /// C++: `SUMOTime MSStop::getUntil() const` [const]
    /// return until / ended time
    // SUMOTime
    // MSStop::getUntil() const {
    //     return MSGlobals::gUseStopEnded && pars.ended >= 0 ? pars.ended : pars.until;
    // }
    #[allow(unused_variables)]
    pub fn get_until(&self) -> SUMOTime {
        todo!("port MSStop::getUntil")
    }

    /// C++: `SUMOTime MSStop::getArrival() const` [const]
    /// return arrival / started time
    // SUMOTime
    // MSStop::getArrival() const {
    //     return MSGlobals::gUseStopStarted && pars.started >= 0 ? pars.started : pars.arrival;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival(&self) -> SUMOTime {
        todo!("port MSStop::getArrival")
    }

    /// C++: `SUMOTime MSStop::getArrivalFallback() const` [const]
    /// return arrival / started time or estimated arrival from until/duration
    // SUMOTime
    // MSStop::getArrivalFallback() const {
    //     SUMOTime result = getArrival();
    //     if (result < 0) {
    //         result = getUntil();
    //         if (result >= 0 && pars.duration >= 0) {
    //             result -= pars.duration;
    //         }
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_arrival_fallback(&self) -> SUMOTime {
        todo!("port MSStop::getArrivalFallback")
    }

    /// C++: `double MSStop::getSpeed() const` [const]
    /// return speed for passing waypoint / skipping on-demand stop
    // double
    // MSStop::getSpeed() const {
    //     return skipOnDemand ? std::numeric_limits<double>::max() : pars.speed;
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port MSStop::getSpeed")
    }

    /// C++: `bool MSStop::isInRange(const double, const double) const` [const]
    /// whether the stop is in range of the given position
    // bool
    // MSStop::isInRange(const double pos, const double tolerance) const {
    //     return pars.startPos - tolerance <= pos && pars.endPos + tolerance >= pos;
    // }
    #[allow(unused_variables)]
    pub fn is_in_range(&self, pos: f64, tolerance: f64) -> bool {
        todo!("port MSStop::isInRange")
    }

    /// C++: `std::vector<MSStoppingPlace *> MSStop::getPlaces() const` [const]
    /// return all stoppingPlaces associated with this stop
    // std::vector<MSStoppingPlace*>
    // MSStop::getPlaces() const {
    //     std::vector<MSStoppingPlace*> result;
    //     if (busstop != nullptr) {
    //         result.push_back(busstop);
    //     }
    //     if (containerstop != nullptr) {
    //         result.push_back(containerstop);
    //     }
    //     if (parkingarea != nullptr) {
    //         result.push_back(parkingarea);
    //     }
    //     if (chargingStation != nullptr) {
    //         result.push_back(chargingStation);
    //     }
    //     if (overheadWireSegment != nullptr) {
    //         result.push_back(overheadWireSegment);
    //     }
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_places(&self) -> Vec<Ptr<MSStoppingPlace>> {
        todo!("port MSStop::getPlaces")
    }

    /// C++: `void MSStop::replaceStoppingPlace(MSStoppingPlace *)`
    /// modify all properties so the stop happens at sp instead
    // void
    // MSStop::replaceStoppingPlace(MSStoppingPlace* sp) {
    //     // @note: assume iterator edge is handled elsewhere
    //     assert(*edge == &sp->getLane().getEdge());
    //     lane = &sp->getLane();
    //     SUMOVehicleParameter::Stop& ncPars = const_cast<SUMOVehicleParameter::Stop&>(pars);
    //     ncPars.edge = lane->getEdge().getID();
    //     ncPars.lane = lane->getID();
    //     ncPars.startPos = sp->getBeginLanePosition();
    //     ncPars.endPos = sp->getEndLanePosition();
    //     if (MSGlobals::gUseMesoSim) {
    //         segment = MSGlobals::gMesoNet->getSegmentForEdge(lane->getEdge(), sp->getEndLanePosition());
    //     }
    //     switch (sp->getElement()) {
    //         case SUMO_TAG_BUS_STOP:
    //         case SUMO_TAG_TRAIN_STOP:
    //             busstop = sp;
    //             ncPars.busstop = sp->getID();
    //             break;
    //         case SUMO_TAG_CONTAINER_STOP:
    //             containerstop = sp;
    //             ncPars.containerstop = sp->getID();
    //             break;
    //         case SUMO_TAG_PARKING_AREA:
    //             parkingarea = dynamic_cast<MSParkingArea*>(sp);
    //             ncPars.parkingarea = sp->getID();
    //             break;
    //         case SUMO_TAG_CHARGING_STATION:
    //             chargingStation = sp;
    //             ncPars.chargingStation = sp->getID();
    //             break;
    //         case SUMO_TAG_OVERHEAD_WIRE_SEGMENT:
    //             overheadWireSegment = sp;
    //             ncPars.overheadWireSegment = sp->getID();
    //             break;
    //         default:
    //             // should not happen
    //             assert(false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_stopping_place(&mut self, sp: Ptr<MSStoppingPlace>) {
        todo!("port MSStop::replaceStoppingPlace")
    }

}

