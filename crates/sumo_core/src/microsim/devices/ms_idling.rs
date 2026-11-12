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
        MSIdling {}
    }

    /// C++: `void MSIdling::~MSIdling() noexcept` [virtual]
    /// Destructor
    // virtual ~MSIdling() { }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty

    }

    /// C++: `void MSIdling::idle(MSDevice_Taxi *)` [virtual, pure virtual]
    /// computes Idling and updates reservations
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn idle(&mut self, taxi: Ptr<MSDevice_Taxi>) {
        panic!("MSIdling::idle is pure virtual; subclasses implement it")
    }

}

// C++ static/defined stop flags (SUMOVehicleParameter.h), parametersSet is i64
const STOP_END_SET: i32 = 2;
const STOP_TRIGGER_SET: i32 = 2 << 4;
const STOP_PARKING_SET: i32 = 2 << 5;

/// C++ class `MSIdling_RandomCircling` (microsim/devices/MSIdling.h)
#[repr(C)]
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
        let veh: &mut SUMOVehicle = taxi.get_holder();
        let vehB: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
        let edges: ConstMSEdgeVector = veh.get_route().get_edges().clone();
        let mut newEdges: ConstMSEdgeVector = Vec::new();
        let mut remainingDist = -veh.get_position_on_lane();
        let mut remainingEdges = 0;
        let mut routePos = veh.get_route_position();
        let routeLength = edges.len() as i32;
        while routePos + 1 < routeLength && (remainingEdges < 2 || remainingDist < 200.0) {
            let edge = edges[routePos as usize];
            remainingDist += edge.get_length();
            remainingEdges += 1;
            routePos += 1;
            newEdges.push(edge);
        }
        let mut lastEdge = *edges.last().unwrap();
        newEdges.push(lastEdge);
        let mut added = 0;
        while remainingEdges < 2 || remainingDist < 200.0 {
            remainingDist += lastEdge.get_length();
            remainingEdges += 1;
            let mut successors: MSEdgeVector = lastEdge.get_successors(vehB.get_v_class()).clone();
            successors.retain(|e| e.get_function() != SumoXMLEdgeFunc::CONNECTOR);
            // shortcut: the gRoutingPreferences filter (device.taxi.idleMinPref via getFloatParam/getPreference) is skipped, it only runs when routing preferences are enabled; upgrade when they are ported
            if successors.is_empty() {
                utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ends idling in a cul-de-sac", veh.get_id()));
                break;
            } else {
                let nextIndex = vehB.get_rng().m().rand_int(successors.len() as i32) as usize;
                newEdges.push(successors[nextIndex]);
                lastEdge = *newEdges.last().unwrap();
                added += 1;
            }
        }
        if added > 0 {
            //std::cout << SIMTIME << " circleVeh=" << veh.getID() << "  newEdges=" << toString(newEdges) << "\n";
            vehB.m().replace_route_edges(&mut newEdges, -1.0, 0.0, "taxi:idling:randomCircling", false, false, false, Ptr::default());
        }
    }

}

/// C++ class `MSIdling_Stop` (microsim/devices/MSIdling.h)
#[repr(C)]
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
        let now = MSNet::get_instance().m().get_current_time_step();
        let veh: &mut SUMOVehicle = taxi.get_holder();
        let vehB: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
        if !vehB.has_stops() {
            let mut errorOut = String::new();
            let mut brakeGap = 0.0;
            let stopPos: (Ptr<MSLane>, f64);
            if use_meso_sim() {
                // stops are only checked in MESegment::receive so we need to put this onto the next segment
                let route = vehB.get_route();
                let riIdx = *vehB.get_current_route_edge();
                let mut curSeg = meso_net().m().get_segment_for_edge(&route.get_edges()[riIdx], vehB.get_position_on_lane());
                let mut stopSeg = curSeg.get_next_segment();
                if stopSeg.is_null() {
                    if riIdx + 1 < route.get_edges().len() {
                        stopSeg = meso_net().m().get_segment_for_edge(&route.get_edges()[riIdx + 1], 0.0);
                    } else {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Idle taxi '%' has no next segment to stop. time=%.", veh.get_id(), time2string(now)));
                        return;
                    }
                }
                // determine offset of stopSeg
                let mut stopOffset = 0.0;
                let stopEdge = stopSeg.get_edge();
                let mut seg = meso_net().m().get_segment_for_edge(stopEdge, 0.0);
                while seg != stopSeg {
                    stopOffset += seg.get_length();
                    seg = seg.get_next_segment();
                }
                stopPos = (stopEdge.get_lanes()[0], stopOffset);
            } else {
                // shortcut: CarFollowModel::brakeGap(speed, decel, headway) takes three arguments in C++, the ported base MSCFModel::brake_gap takes one, upgrade when MSCFModel gets the virtual 3-arg brakeGap
                panic!("MSIdling_Stop: micro brakeGap(speed, decel, headway) is not ported on MSCFModel yet");
            }
            if !stopPos.0.is_null() {
                let mut stop = SUMOVehicleParameter_Stop::default();
                if use_meso_sim() {
                    stop.edge = stopPos.0.get_edge().get_id().to_string();
                } else {
                    stop.lane = stopPos.0.get_id().to_string();
                }
                stop.startPos = f64::max(0.0, stopPos.1 - utils::geom::POSITION_EPS);
                stop.endPos = stopPos.1;
                if use_meso_sim() {
                    // meso needs the stop to be on the next segment
                    stop.startPos += utils::geom::POSITION_EPS;
                    stop.endPos += utils::geom::POSITION_EPS;
                }
                stop.actType = "idling".to_string();
                vehB.m().add_traci_stop(stop, &mut errorOut);
                if errorOut != "" {
                    utils::common::msg_handler::write_warning(&errorOut);
                }
            } else {
                utils::common::msg_handler::write_warning(&utils::tlf!("Idle taxi '%' could not stop within %m", veh.get_id(), brakeGap.to_string()));
            }
        }
        if vehB.has_stops() {
            let containerCapacity = veh.get_vehicle_type().get_container_capacity();
            let stop = vehB.m().get_next_stop_mutable();
            if containerCapacity > 0 {
                stop.containerTriggered = true;
                stop.pars.containerTriggered = true;
            } else {
                stop.triggered = true;
                stop.pars.triggered = true;
            }
            stop.pars.parametersSet |= STOP_TRIGGER_SET | STOP_PARKING_SET | STOP_END_SET;
            stop.pars.parking = ParkingType::OFFROAD;
            if use_meso_sim() {
                // MEVehicle: register triggered stop
                Ptr::from_mut(veh).cast::<MEVehicle>().m().may_proceed();
            }
        }
    }

}

/// C++ class `MSIdling_TaxiStand` (microsim/devices/MSIdling.h)
#[repr(C)]
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
        MSIdling_TaxiStand {
            base: MSIdling {},
            myRerouter: rerouter,
            myHaveWarned: false,
        }
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
        let veh: &mut SUMOVehicle = taxi.get_holder();
        let vehB: Ptr<MSBaseVehicle> = Ptr::from_mut(veh).cast::<MSBaseVehicle>();
        let now = MSNet::get_instance().m().get_current_time_step();
        let rerouteDef = self.myRerouter.m().get_current_reroute(now, veh);
        if rerouteDef.is_null() || rerouteDef.parkProbs.get_vals().len() == 0 {
            if !self.myHaveWarned {
                utils::common::msg_handler::write_warning(&utils::tlf!("Could not determine taxi stand for vehicle '%' at time=%", veh.get_id(), time2string(now)));
                self.myHaveWarned = true;
            }
            return;
        }
        let mut lastStop: Option<&mut MSStop> = None;
        if vehB.has_stops() {
            let n = vehB.get_stops().len() as i32;
            lastStop = Some(vehB.m().get_stop(n - 1));
        }
        if lastStop.as_ref().map_or(true, |s| s.parkingarea.is_null()) {
            let pa = rerouteDef.parkProbs.get_vals()[0].0.cast::<MSParkingArea>();
            let mut stop = SUMOVehicleParameter_Stop::default();
            stop.lane = pa.get_lane().get_id().to_string();
            stop.startPos = pa.get_begin_lane_position();
            stop.endPos = pa.get_end_lane_position();
            if veh.get_vehicle_type().get_container_capacity() > 0 {
                stop.containerTriggered = true;
            } else {
                stop.triggered = true;
            }
            stop.actType = "idling".to_string();
            stop.parkingarea = pa.get_id().to_string();
            stop.parking = ParkingType::OFFROAD;
            stop.index = vehB.get_stops().len() as i32;
            let mut error = String::new();
            if !vehB.m().insert_stop(stop.index, stop, "taxi:taxistand", false, &mut error) {
                utils::common::msg_handler::write_warning(&format!("Stop insertion failed for idling taxi '{}' ({}).", veh.get_id(), error));
            } else {
                //std::cout << SIMTIME << " taxistandsVeh=" << veh.getID() << "  driving to parkingArea " << pa->getID() << "\n";
                self.myRerouter.m().trigger_routing(veh, MSMoveReminder_Notification::NOTIFICATION_PARKING_REROUTE);
            }
        } else if !use_meso_sim() {
            //std::cout << SIMTIME << " taxistandsVeh=" << veh.getID() << "  already driving to parkingArea\n";
            let pa = lastStop.as_ref().unwrap().parkingarea;
            if vehB.is_stopped_triggered() && !pa.is_null() && pa.must_advance(vehB.get_v_class()) {
                let vehPos = veh.get_position_on_lane();
                let targetPos = pa.get_last_free_pos(veh, vehPos);
                if targetPos > vehPos + utils::geom::POSITION_EPS {
                    vehB.m().abort_next_stop(0);
                    self.idle(taxi);
                } else {
                    let follower = vehB.get_follower(0.0);
                    if !follower.0.is_null() && follower.0.get_waiting_time(false) > delta_t() {
                        // advance in queue to unblock follower
                        let mut stop = vehB.get_next_stop().pars.clone();
                        vehB.m().abort_next_stop(0);
                        let mut loopedRoute: ConstMSEdgeVector = Vec::new();
                        let router = MSRoutingEngine::get_router_tt(vehB.get_rng_index(), vehB.get_v_class(), &Default::default());
                        router.compute_looped(vehB.get_edge(), vehB.get_edge(), Ptr::from_mut(veh), now, &mut loopedRoute, false);
                        vehB.m().replace_route_edges(&mut loopedRoute, -1.0, 0.0, "taxi:idling_unblock", false, false, false, Ptr::default());
                        stop.index = 1;
                        let mut error = String::new();
                        vehB.m().add_stop(&stop, &mut error, 0, Ptr::default());
                    }
                }
            }
        }
    }

}

