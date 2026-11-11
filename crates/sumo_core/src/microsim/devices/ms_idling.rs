//! Port of microsim/devices/MSIdling.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// An algorithm that performs distpach for a taxi fleet
/// C++ class `MSIdling` (microsim/devices/MSIdling.h)
pub struct MSIdling {
}

impl MSIdling {
    /// C++: `void MSIdling::MSIdling()`
    /// Constructor;
    // MSIdling() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        todo!("port MSIdling::MSIdling")
    }

    /// C++: `void MSIdling::~MSIdling() noexcept` [virtual]
    /// Destructor
    // virtual ~MSIdling() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSIdling::~MSIdling")
    }

    /// C++: `void MSIdling::idle(MSDevice_Taxi *)` [virtual, pure virtual]
    /// computes Idling and updates reservations
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn idle(&mut self, taxi: Ptr<MSDevice_Taxi>) {
        todo!("port MSIdling::idle")
    }

}

/// C++ class `MSIdling_RandomCircling` (microsim/devices/MSIdling.h)
pub struct MSIdling_RandomCircling {
    /// C++ base class `MSIdling`
    pub base: MSIdling,
}

impl std::ops::Deref for MSIdling_RandomCircling {
    type Target = MSIdling;
    fn deref(&self) -> &MSIdling {
        &self.base
    }
}
impl std::ops::DerefMut for MSIdling_RandomCircling {
    fn deref_mut(&mut self) -> &mut MSIdling {
        &mut self.base
    }
}

impl MSIdling_RandomCircling {
    /// C++: `void MSIdling_RandomCircling::idle(MSDevice_Taxi *)` [virtual, private]
    // void
    // MSIdling_RandomCircling::idle(MSDevice_Taxi* taxi) {
    //     SUMOVehicle& veh = taxi->getHolder();
    //     ConstMSEdgeVector edges = veh.getRoute().getEdges();
    //     ConstMSEdgeVector newEdges;
    //     double remainingDist = -veh.getPositionOnLane();
    //     int remainingEdges = 0;
    //     int routePos = veh.getRoutePosition();
    //     const int routeLength = (int)edges.size();
    //     while (routePos + 1 < routeLength && (remainingEdges < 2 || remainingDist < 200)) {
    //         const MSEdge* edge = edges[routePos];
    //         remainingDist += edge->getLength();
    //         remainingEdges++;
    //         routePos++;
    //         newEdges.push_back(edge);
    //     }
    //     const MSEdge* lastEdge = edges.back();
    //     newEdges.push_back(lastEdge);
    //     int added = 0;
    //     while (remainingEdges < 2 || remainingDist < 200) {
    //         remainingDist += lastEdge->getLength();
    //         remainingEdges++;
    //         MSEdgeVector successors = lastEdge->getSuccessors(veh.getVClass());
    //         for (auto it = successors.begin(); it != successors.end();) {
    //             if ((*it)->getFunction() == SumoXMLEdgeFunc::CONNECTOR) {
    //                 it = successors.erase(it);
    //             } else {
    //                 it++;
    //             }
    //         }
    //         if (gRoutingPreferences && successors.size() > 1) {
    //             const double threshPref = veh.getFloatParam("device.taxi.idleMinPref", false, 0);
    //             double maxPref = -std::numeric_limits<double>::max();
    //             for (const MSEdge* edge : successors) {
    //                 maxPref = MAX2(maxPref, edge->getPreference(veh.getVTypeParameter()));
    //             }
    //             if (maxPref >= threshPref) {
    //                 // there is at least one favoured successor, remove low preference edges
    //                 for (auto it = successors.begin(); it != successors.end();) {
    //                     if ((*it)->getPreference(veh.getVTypeParameter()) < threshPref) {
    //                         it = successors.erase(it);
    //                     } else {
    //                         it++;
    //                     }
    //                 }
    //             }
    //         }
    //         if (successors.size() == 0) {
    //             WRITE_WARNINGF(TL("Vehicle '%' ends idling in a cul-de-sac"), veh.getID());
    //             break;
    //         } else {
    //             int nextIndex = RandHelper::rand((int)successors.size(), veh.getRNG());
    //             newEdges.push_back(successors[nextIndex]);
    //             lastEdge = newEdges.back();
    //             added++;
    //         }
    //     }
    //     if (added > 0) {
    //         //std::cout << SIMTIME << " circleVeh=" << veh.getID() << "  newEdges=" << toString(newEdges) << "\n";
    //         veh.replaceRouteEdges(newEdges, -1, 0, "taxi:idling:randomCircling", false, false, false);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn idle(&mut self, taxi: Ptr<MSDevice_Taxi>) {
        todo!("port MSIdling_RandomCircling::idle")
    }

}

/// C++ class `MSIdling_Stop` (microsim/devices/MSIdling.h)
pub struct MSIdling_Stop {
    /// C++ base class `MSIdling`
    pub base: MSIdling,
}

impl std::ops::Deref for MSIdling_Stop {
    type Target = MSIdling;
    fn deref(&self) -> &MSIdling {
        &self.base
    }
}
impl std::ops::DerefMut for MSIdling_Stop {
    fn deref_mut(&mut self) -> &mut MSIdling {
        &mut self.base
    }
}

impl MSIdling_Stop {
    /// C++: `void MSIdling_Stop::idle(MSDevice_Taxi *)` [virtual, private]
    // void
    // MSIdling_Stop::idle(MSDevice_Taxi* taxi) {
    //     if (!taxi->getHolder().hasStops()) {
    // #ifdef DEBUG_IDLING
    //         if (DEBUG_COND(taxi)) {
    //             std::cout << SIMTIME << " taxi=" << taxi->getHolder().getID() << " MSIdling_Stop add stop\n";
    //         }
    // #endif
    //         std::string errorOut;
    //         double brakeGap = 0;
    //         std::pair<const MSLane*, double> stopPos;
    //         if (MSGlobals::gUseMesoSim) {
    //             // stops are only checked in MESegment::receive so we need to put this onto the next segment
    //             MSBaseVehicle& veh = dynamic_cast<MSBaseVehicle&>(taxi->getHolder());
    //             MSRouteIterator ri = veh.getCurrentRouteEdge();
    //             MESegment* curSeg = MSGlobals::gMesoNet->getSegmentForEdge(**ri, veh.getPositionOnLane());
    //             MESegment* stopSeg = curSeg->getNextSegment();
    //             if (stopSeg == nullptr) {
    //                 if ((ri + 1) != veh.getRoute().end()) {
    //                     stopSeg = MSGlobals::gMesoNet->getSegmentForEdge(**(ri + 1), 0);
    //                 } else {
    //                     WRITE_WARNINGF(TL("Idle taxi '%' has no next segment to stop. time=%."), taxi->getHolder().getID(), time2string(SIMSTEP));
    //                     return;
    //                 }
    //             }
    //             // determine offset of stopSeg
    //             double stopOffset = 0;
    //             const MSEdge& stopEdge = stopSeg->getEdge();
    //             MESegment* seg = MSGlobals::gMesoNet->getSegmentForEdge(stopEdge);
    //             while (seg != stopSeg) {
    //                 stopOffset += seg->getLength();
    //                 seg = seg->getNextSegment();
    //             }
    //             stopPos = std::make_pair(stopEdge.getLanes()[0], stopOffset);
    //         } else {
    //             MSVehicle& veh = dynamic_cast<MSVehicle&>(taxi->getHolder());
    //             brakeGap = veh.getCarFollowModel().brakeGap(veh.getSpeed(), veh.getCarFollowModel().getMaxDecel(), 0.0);
    //             stopPos = veh.getLanePosAfterDist(brakeGap);
    //         }
    //         if (stopPos.first != nullptr) {
    //             SUMOVehicleParameter::Stop stop;
    //             if (MSGlobals::gUseMesoSim) {
    //                 stop.edge = stopPos.first->getEdge().getID();
    //             } else {
    //                 stop.lane = stopPos.first->getID();
    //             }
    //             stop.startPos = MAX2(0.0, stopPos.second - POSITION_EPS);
    //             stop.endPos = stopPos.second;
    //             if (MSGlobals::gUseMesoSim) {
    //                 // meso needs the stop to be on the next segment
    //                 stop.startPos += POSITION_EPS;
    //                 stop.endPos += POSITION_EPS;
    //             }
    //             stop.actType = "idling";
    //             taxi->getHolder().addTraciStop(stop, errorOut);
    //             if (errorOut != "") {
    //                 WRITE_WARNING(errorOut);
    //             }
    //         } else {
    //             WRITE_WARNINGF(TL("Idle taxi '%' could not stop within %m"), taxi->getHolder().getID(), toString(brakeGap));
    //         }
    //     } else {
    // #ifdef DEBUG_IDLING
    //         if (DEBUG_COND(taxi)) {
    //             std::cout << SIMTIME << " taxi=" << taxi->getHolder().getID() << " MSIdling_Stop reusing stop with duration " << time2string(taxi->getHolder().getNextStop().duration) << "\n";
    //         }
    // #endif
    //     }
    //     if (taxi->getHolder().hasStops()) {
    //         MSStop& stop = taxi->getHolder().getNextStopMutable();
    //         SUMOVehicleParameter::Stop& pars = const_cast<SUMOVehicleParameter::Stop&>(stop.pars);
    //         if (taxi->getHolder().getVehicleType().getContainerCapacity() > 0) {
    //             stop.containerTriggered = true;
    //             pars.containerTriggered = true;
    //         } else {
    //             stop.triggered = true;
    //             pars.triggered = true;
    //         }
    //         pars.parametersSet |= STOP_TRIGGER_SET | STOP_PARKING_SET | STOP_END_SET;
    //         pars.parking = ParkingType::OFFROAD;
    //         if (MSGlobals::gUseMesoSim) {
    //             MEVehicle& veh = dynamic_cast<MEVehicle&>(taxi->getHolder());
    //             // register triggered stop
    //             veh.mayProceed();
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn idle(&mut self, taxi: Ptr<MSDevice_Taxi>) {
        todo!("port MSIdling_Stop::idle")
    }

}

/// C++ class `MSIdling_TaxiStand` (microsim/devices/MSIdling.h)
pub struct MSIdling_TaxiStand {
    /// C++ base class `MSIdling`
    pub base: MSIdling,
    pub myRerouter: Ptr<MSTriggeredRerouter>,
    pub myHaveWarned: bool,
}

impl std::ops::Deref for MSIdling_TaxiStand {
    type Target = MSIdling;
    fn deref(&self) -> &MSIdling {
        &self.base
    }
}
impl std::ops::DerefMut for MSIdling_TaxiStand {
    fn deref_mut(&mut self) -> &mut MSIdling {
        &mut self.base
    }
}

impl MSIdling_TaxiStand {
    /// C++: `void MSIdling_TaxiStand::MSIdling_TaxiStand(MSTriggeredRerouter *)`
    // MSIdling_TaxiStand(MSTriggeredRerouter* rerouter) : myRerouter(rerouter) {}
    #[allow(unused_variables)]
    pub fn new(rerouter: Ptr<MSTriggeredRerouter>) -> Self {
        todo!("port MSIdling_TaxiStand::MSIdling_TaxiStand")
    }

    /// C++: `void MSIdling_TaxiStand::idle(MSDevice_Taxi *)` [virtual]
    // void
    // MSIdling_TaxiStand::idle(MSDevice_Taxi* taxi) {
    //     MSBaseVehicle& veh = dynamic_cast<MSBaseVehicle&>(taxi->getHolder());
    //
    //     const MSTriggeredRerouter::RerouteInterval* rerouteDef = myRerouter->getCurrentReroute(SIMSTEP);
    //     if (rerouteDef == nullptr || rerouteDef->parkProbs.getVals().size() == 0) {
    //         if (!myHaveWarned) {
    //             WRITE_WARNINGF(TL("Could not determine taxi stand for vehicle '%' at time=%"), veh.getID(), time2string(SIMSTEP));
    //             myHaveWarned = true;
    //         }
    //         return;
    //     }
    //     MSStop* lastStop = nullptr;
    //     if (veh.hasStops()) {
    //         lastStop = &veh.getStop((int)veh.getStops().size() - 1);
    //     }
    //     if (lastStop == nullptr || lastStop->parkingarea == nullptr) {
    //         const MSParkingArea* pa = dynamic_cast<MSParkingArea*>(rerouteDef->parkProbs.getVals().front().first);
    //         SUMOVehicleParameter::Stop stop;
    //         stop.lane = pa->getLane().getID();
    //         stop.startPos = pa->getBeginLanePosition();
    //         stop.endPos = pa->getEndLanePosition();
    //
    //         if (taxi->getHolder().getVehicleType().getContainerCapacity() > 0) {
    //             stop.containerTriggered = true;
    //         } else {
    //             stop.triggered = true;
    //         }
    //         stop.actType = "idling";
    //         stop.parkingarea = pa->getID();
    //         stop.parking = ParkingType::OFFROAD;
    //         stop.index = (int)veh.getStops().size();
    //         std::string error;
    //         if (!veh.insertStop(stop.index, stop, "taxi:taxistand", false, error)) {
    //             WRITE_WARNING("Stop insertion failed for idling taxi '" + veh.getID() + "' (" + error + ").");
    //         } else {
    //             //std::cout << SIMTIME << " taxistandsVeh=" << veh.getID() << "  driving to parkingArea " << pa->getID() << "\n";
    //             myRerouter->triggerRouting(veh, MSMoveReminder::NOTIFICATION_PARKING_REROUTE);
    //         }
    //     } else if (!MSGlobals::gUseMesoSim) {
    //         //std::cout << SIMTIME << " taxistandsVeh=" << veh.getID() << "  already driving to parkingArea\n";
    //         MSParkingArea* pa = lastStop->parkingarea;
    //         if (taxi->getHolder().isStoppedTriggered() && pa != nullptr && pa->mustAdvance(taxi->getHolder().getVClass())) {
    //             double vehPos = taxi->getHolder().getPositionOnLane();
    //             double targetPos = pa->getLastFreePos(taxi->getHolder(), vehPos);
    //             //std::cout << SIMTIME << " veh=" << taxi->getHolder().getID() << " vehPos=" << vehPos << " targetPos=" << targetPos << " cap=" << pa->getCapacity() << " occ=" << pa->getOccupancyIncludingBlocked() << "\n";
    //             if (targetPos > vehPos + POSITION_EPS) {
    //                 taxi->getHolder().abortNextStop();
    //                 idle(taxi);
    //             } else {
    //                 auto follower = veh.getFollower();
    //                 if (follower.first != nullptr && follower.first->getWaitingTime() > DELTA_T) {
    //                     // advance in queue to unblock follower
    //                     SUMOVehicleParameter::Stop stop = taxi->getHolder().getNextStop().pars;
    //                     taxi->getHolder().abortNextStop();
    //                     ConstMSEdgeVector loopedRoute;
    //                     SUMOAbstractRouter<MSEdge, SUMOVehicle>& router = MSRoutingEngine::getRouterTT(veh.getRNGIndex(), veh.getVClass());
    //                     router.computeLooped(veh.getEdge(), veh.getEdge(), &veh, SIMSTEP, loopedRoute);
    //                     veh.replaceRouteEdges(loopedRoute, -1, 0, "taxi:idling_unblock", false, false, false);
    //                     stop.index = 1;
    //                     std::string error;
    //                     veh.addStop(stop, error);
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn idle(&mut self, taxi: Ptr<MSDevice_Taxi>) {
        todo!("port MSIdling_TaxiStand::idle")
    }

}

