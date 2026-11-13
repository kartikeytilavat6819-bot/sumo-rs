//! Port of microsim/transportables/MSPerson.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::geom::POSITION_EPS;

/// Changes the wished person speed and position
/// C++ class `MSPerson::Influencer` (microsim/transportables/MSPerson.h)
#[repr(C)]
pub struct MSPerson_Influencer {
    pub myRemoteXYPos: Position,
    pub myRemoteLane: Ptr<MSLane>,
    pub myRemotePos: f64,
    pub myRemotePosLat: f64,
    pub myRemoteAngle: f64,
    pub myRemoteEdgeOffset: i32,
    pub myRemoteRoute: ConstMSEdgeVector,
    pub myLastRemoteAccess: SUMOTime,
}

impl MSPerson_Influencer {
    /// C++: `void Influencer::Influencer()`
    /// Constructor
    // MSPerson::Influencer::Influencer() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        MSPerson_Influencer {
            myRemoteXYPos: Position::new(0., 0.),
            myRemoteLane: Ptr::null(),
            myRemotePos: 0.,
            myRemotePosLat: 0.,
            myRemoteAngle: 0.,
            myRemoteEdgeOffset: 0,
            myRemoteRoute: Vec::new(),
            myLastRemoteAccess: 0,
        }
    }

    /// C++: `void Influencer::~Influencer()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `void Influencer::setRemoteControlled(Position, MSLane *, double, double, double, int, const ConstMSEdgeVector &, SUMOTime)`
    // void
    // MSPerson::Influencer::setRemoteControlled(Position xyPos, MSLane* l, double pos, double posLat, double angle, int edgeOffset, const ConstMSEdgeVector& route, SUMOTime t) {
    //     myRemoteXYPos = xyPos;
    //     myRemoteLane = l;
    //     myRemotePos = pos;
    //     myRemotePosLat = posLat;
    //     myRemoteAngle = angle;
    //     myRemoteEdgeOffset = edgeOffset;
    //     myRemoteRoute = route;
    //     myLastRemoteAccess = t;
    // }
    #[allow(unused_variables)]
    pub fn set_remote_controlled(&mut self, xyPos: Position, l: Ptr<MSLane>, pos: f64, posLat: f64, angle: f64, edgeOffset: i32, route: &ConstMSEdgeVector, t: SUMOTime) {
        self.myRemoteXYPos = xyPos;
        self.myRemoteLane = l;
        self.myRemotePos = pos;
        self.myRemotePosLat = posLat;
        self.myRemoteAngle = angle;
        self.myRemoteEdgeOffset = edgeOffset;
        self.myRemoteRoute = route.clone();
        self.myLastRemoteAccess = t;
    }

    /// C++: `SUMOTime Influencer::getLastAccessTimeStep() const` [const]
    // SUMOTime getLastAccessTimeStep() const {
    //             return myLastRemoteAccess;
    //         }
    #[allow(unused_variables)]
    pub fn get_last_access_time_step(&self) -> SUMOTime {
        self.myLastRemoteAccess
    }

    /// C++: `void Influencer::postProcessRemoteControl(MSPerson *)`
    // void
    // MSPerson::Influencer::postProcessRemoteControl(MSPerson* p) {
    //     /*
    //     std::cout << SIMTIME << " moveToXY person=" << p->getID()
    //         << " xyPos=" << myRemoteXYPos
    //         << " lane=" << Named::getIDSecure(myRemoteLane)
    //         << " pos=" << myRemotePos
    //         << " posLat=" << myRemotePosLat
    //         << " angle=" << myRemoteAngle
    //         << " eOf=" << myRemoteEdgeOffset
    //         << " route=" << toString(myRemoteRoute)
    //         << " aTime=" << time2string(myLastRemoteAccess)
    //         << "\n";
    //         */
    //     switch (p->getStageType(0)) {
    //         case MSStageType::WALKING: {
    //             MSStageWalking* s = dynamic_cast<MSStageWalking*>(p->getCurrentStage());
    //             assert(s != nullptr);
    //             s->getPState()->moveToXY(p, myRemoteXYPos, myRemoteLane, myRemotePos, myRemotePosLat,
    //                                      myRemoteAngle, myRemoteEdgeOffset, myRemoteRoute,
    //                                      MSNet::getInstance()->getCurrentTimeStep());
    //         }
    //         break;
    //         default:
    //             break;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn post_process_remote_control(&mut self, p: Ptr<MSPerson>) {
        match p.get_stage_type(0) {
            MSStageType::WALKING => {
                let s: Ptr<MSStageWalking> = p.get_current_stage().cast::<MSStageWalking>();
                let now = MSNet::get_instance().get_current_time_step();
                s.myPState.m().move_to_xy(p, self.myRemoteXYPos, self.myRemoteLane, self.myRemotePos, self.myRemotePosLat, self.myRemoteAngle, self.myRemoteEdgeOffset, &self.myRemoteRoute, now);
            }
            _ => {}
        }
    }

    /// C++: `bool Influencer::isRemoteControlled() const` [const]
    // bool
    // MSPerson::Influencer::isRemoteControlled() const {
    //     return myLastRemoteAccess == MSNet::getInstance()->getCurrentTimeStep();
    // }
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        self.myLastRemoteAccess == MSNet::get_instance().get_current_time_step()
    }

    /// C++: `bool Influencer::isRemoteAffected(SUMOTime) const` [const]
    // bool
    // MSPerson::Influencer::isRemoteAffected(SUMOTime t) const {
    //     return myLastRemoteAccess >= t - TIME2STEPS(10);
    // }
    #[allow(unused_variables)]
    pub fn is_remote_affected(&self, t: SUMOTime) -> bool {
        self.myLastRemoteAccess >= t - time2steps(10.)
    }

}

// C++ virtual overrides of SUMOTrafficObject, dispatched through SUMOTrafficObject::vt
fn ms_person_is_vehicle(_: &SUMOTrafficObject) -> bool {
    false
}
fn ms_person_is_person(_: &SUMOTrafficObject) -> bool {
    true
}
fn ms_person_is_container(_: &SUMOTrafficObject) -> bool {
    false
}
static MS_PERSON_VT: SUMOTrafficObjectVt = SUMOTrafficObjectVt {
    is_vehicle: ms_person_is_vehicle,
    is_person: ms_person_is_person,
    is_container: ms_person_is_container,
    get_parameter: ms_person_get_parameter,
    // shortcut: MSPerson getEdge/getNumericalID are not ported yet, base pure-virtual panic kept, upgrade when persons are simulated
    get_edge: crate::utils::vehicle::sumo_traffic_object::default_get_edge,
    get_numerical_id: crate::utils::vehicle::sumo_traffic_object::default_get_numerical_id,
};

/// The class holds a simulated person together with her movement stages
/// C++ class `MSPerson` (microsim/transportables/MSPerson.h)
#[repr(C)]
pub struct MSPerson {
    /// C++ base class `MSTransportable`
    pub base: MSTransportable,
    /// An instance of a speed/position influencing instance; built in "getInfluencer"
    pub myInfluencer: Ptr<MSPerson_Influencer>,
    pub myChosenSpeedFactor: f64,
    /// value may be sampled from distribution and therefore must be cached
    pub myTimegapCrossing: f64,
}

impl std::ops::Deref for MSPerson {
    type Target = MSTransportable;
    fn deref(&self) -> &MSTransportable {
        &self.base
    }
}
impl std::ops::DerefMut for MSPerson {
    fn deref_mut(&mut self) -> &mut MSTransportable {
        &mut self.base
    }
}

impl MSPerson {
    /// C++: `void MSPerson::MSPerson(const SUMOVehicleParameter *, MSVehicleType *, MSTransportable::MSTransportablePlan *, const double)`
    /// constructor
    // MSPerson::MSPerson(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportable::MSTransportablePlan* plan, const double speedFactor) :
    //     MSTransportable(pars, vtype, plan, true),
    //     myInfluencer(nullptr),
    //     myChosenSpeedFactor(pars->speedFactor < 0 ? speedFactor : pars->speedFactor),
    //     myTimegapCrossing(getFloatParam("pedestrian.timegap-crossing"))
    // { }
    #[allow(unused_variables)]
    pub fn new(pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>, speedFactor: f64) -> Self {
        let base = MSTransportable::new(pars, vtype, plan, true);
        let timegap = base.get_float_param("pedestrian.timegap-crossing", false, utils::common::std_defs::INVALID_DOUBLE, true);
        let mut p = MSPerson {
            base,
            myInfluencer: Ptr::null(),
            myChosenSpeedFactor: if pars.speedFactor < 0. { speedFactor } else { pars.speedFactor },
            myTimegapCrossing: timegap,
        };
        p.base.base.vt = &MS_PERSON_VT;
        p
    }

    /// C++: `void MSPerson::~MSPerson() noexcept` [virtual]
    /// destructor
    // MSPerson::~MSPerson() {
    //     delete myInfluencer;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ deletes myInfluencer here; ownership of the influencer is not modelled yet
    }

    /// C++: `bool MSPerson::checkAccess(const MSStage *const, const bool)` [virtual]
    // bool
    // MSPerson::checkAccess(const MSStage* const prior, const bool waitAtStop) {
    //     MSStoppingPlace* prevStop = prior->getDestinationStop();
    //     if (!waitAtStop && prior->getStageType() == MSStageType::TRIP) {
    //         prevStop = prior->getOriginStop();
    //     }
    //     if (prevStop != nullptr) {
    //         const MSEdge* const accessEdge = waitAtStop ? prior->getDestination() : (*myStep)->getFromEdge();
    //         const MSStoppingPlace::Access* const access = prevStop->getAccess(accessEdge);
    //         if (access != nullptr) {
    //             const MSLane* const lane = accessEdge->getLanes()[0];
    //             MSStage* newStage = nullptr;
    //             if (waitAtStop) {
    //                 const MSEdge* const stopEdge = &prevStop->getLane().getEdge();
    //                 const double arrivalAtBs = (prevStop->getBeginLanePosition() + prevStop->getEndLanePosition()) / 2;
    //                 newStage = new MSPersonStage_Access(stopEdge, prevStop, arrivalAtBs, 0.0, access->length, false,
    //                                                     lane->geometryPositionAtOffset(access->endPos),
    //                                                     prevStop->getLane().geometryPositionAtOffset(arrivalAtBs));
    //             } else {
    //                 const bool useDoors = access->exit == MSStoppingPlace::AccessExit::DOORS ||
    //                                       (!MSNet::getInstance()->getPersonControl().getMovementModel()->usingDoors() && access->exit == MSStoppingPlace::AccessExit::CARRIAGE);
    //                 if (access->exit == MSStoppingPlace::AccessExit::CARRIAGE) {
    //                     const double startPos = prior->getStageType() == MSStageType::TRIP ? prior->getEdgePos(0) : prior->getArrivalPos();
    //                     const double startPosLat = prior->getStageType() == MSStageType::TRIP ? prior->getEdgePosLat(0) : prior->getArrivalPosLat();
    //                     // The start and end attributes of the access stage are equal in this case, but we need to compute the arrival position relatively
    //                     // to the current lane and not the lane of the previous stage.
    //                     const Position start = prevStop->getLane().geometryPositionAtOffset(startPos, startPosLat);
    //                     const Position end = lane->getShape().transformToVectorCoordinates(start);
    //                     newStage = new MSPersonStage_Access(accessEdge, prevStop, end.x(), -end.y(), access->length, true, start, start);
    //                 } else {
    //                     const double startPos = prior->getStageType() == MSStageType::TRIP ? prior->getEdgePos(0) : prior->getArrivalPos();
    //                     const Position& trainExit = prevStop->getLane().geometryPositionAtOffset(startPos);
    //                     const double arrivalPos = useDoors ? lane->getShape().nearest_offset_to_point2D(trainExit) : access->endPos;
    //                     Position platformEntry = lane->geometryPositionAtOffset(arrivalPos);
    //                     if (useDoors) {
    //                         // find the closer side of the platform to enter
    //                         const double halfWidth = lane->getWidth() / 2. - MAX2(getVehicleType().getLength(), getVehicleType().getWidth()) / 2. - POSITION_EPS;
    //                         platformEntry = lane->geometryPositionAtOffset(arrivalPos, halfWidth);
    //                         const Position& plat2 = lane->geometryPositionAtOffset(arrivalPos, -halfWidth);
    //                         if (trainExit.distanceSquaredTo2D(plat2) < trainExit.distanceSquaredTo2D(platformEntry)) {
    //                             platformEntry = plat2;
    //                         }
    //                     }
    //                     newStage = new MSPersonStage_Access(accessEdge, prevStop, arrivalPos, 0.0, access->length, true,
    //                                                         trainExit, platformEntry);
    //                 }
    //             }
    //             newStage->setTrip(prior->getTrip());
    //             myStep = myPlan->insert(myStep, newStage);
    //             return true;
    //         }
    //     }
    //     if (prior->getJumpDuration() > 0) {
    //         // negative distance indicates jump
    //         MSStage* newStage = new MSPersonStage_Access(getDestination(), nullptr, getArrivalPos(), 0.0, -1, true,
    //                 prior->getPosition(SIMSTEP), (*myStep)->getPosition(SIMSTEP));
    //         myStep = myPlan->insert(myStep, newStage);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn check_access(&mut self, prior: Ptr<MSStage>, waitAtStop: bool) -> bool {
        let mut prevStop = prior.get_destination_stop();
        if !waitAtStop && prior.get_stage_type() == MSStageType::TRIP {
            prevStop = prior.get_origin_stop();
        }
        if !prevStop.is_null() {
            let accessEdge = if waitAtStop { prior.get_destination() } else { self.get_current_stage().get_from_edge() };
            let access = prevStop.get_access(accessEdge);
            if !access.is_null() {
                let lane = accessEdge.get_lanes()[0];
                let st: MSPerson_MSPersonStage_Access = if waitAtStop {
                    let stopEdge = Ptr::from_mut(prevStop.get_lane().get_edge());
                    let arrivalAtBs = (prevStop.get_begin_lane_position() + prevStop.get_end_lane_position()) / 2.;
                    let startP = lane.geometry_position_at_offset(access.endPos, 0.);
                    let endP = prevStop.get_lane().geometry_position_at_offset(arrivalAtBs, 0.);
                    MSPerson_MSPersonStage_Access::new(stopEdge, prevStop, arrivalAtBs, 0.0, access.length, false, &startP, &endP)
                } else {
                    // shortcut: MSTransportableControl::getMovementModel is not ported, so usingDoors() is assumed false here
                    let usingDoors = false;
                    let useDoors = access.exit == MSStoppingPlace_AccessExit::DOORS || (!usingDoors && access.exit == MSStoppingPlace_AccessExit::CARRIAGE);
                    if access.exit == MSStoppingPlace_AccessExit::CARRIAGE {
                        let startPos = if prior.get_stage_type() == MSStageType::TRIP { prior.get_edge_pos(0) } else { prior.get_arrival_pos() };
                        let startPosLat = if prior.get_stage_type() == MSStageType::TRIP { prior.get_edge_pos_lat(0) } else { prior.get_arrival_pos_lat() };
                        // The start and end attributes of the access stage are equal in this case, but we need to compute the arrival position relatively
                        // to the current lane and not the lane of the previous stage.
                        let start = prevStop.get_lane().geometry_position_at_offset(startPos, startPosLat);
                        let end = lane.get_shape_2().transform_to_vector_coordinates(&start, false);
                        MSPerson_MSPersonStage_Access::new(accessEdge, prevStop, end.x(), -end.y(), access.length, true, &start, &start)
                    } else {
                        let startPos = if prior.get_stage_type() == MSStageType::TRIP { prior.get_edge_pos(0) } else { prior.get_arrival_pos() };
                        let trainExit = prevStop.get_lane().geometry_position_at_offset(startPos, 0.);
                        let arrivalPos = if useDoors { lane.get_shape_2().nearest_offset_to_point2d(&trainExit, true) } else { access.endPos };
                        let mut platformEntry = lane.geometry_position_at_offset(arrivalPos, 0.);
                        if useDoors {
                            // find the closer side of the platform to enter
                            let (len, wid) = (self.get_vehicle_type().get_length(), self.get_vehicle_type().get_width());
                            let maxDim = if len > wid { len } else { wid };
                            let halfWidth = lane.get_width() / 2. - maxDim / 2. - POSITION_EPS;
                            platformEntry = lane.geometry_position_at_offset(arrivalPos, halfWidth);
                            let plat2 = lane.geometry_position_at_offset(arrivalPos, -halfWidth);
                            if trainExit.distance_squared_to2d(&plat2) < trainExit.distance_squared_to2d(&platformEntry) {
                                platformEntry = plat2;
                            }
                        }
                        MSPerson_MSPersonStage_Access::new(accessEdge, prevStop, arrivalPos, 0.0, access.length, true, &trainExit, &platformEntry)
                    }
                };
                let newStage = Ptr::from_mut(Box::leak(Box::new(st))).cast::<MSStage>();
                newStage.m().set_trip(prior.get_trip());
                self.myPlan.m().insert(self.myStep, newStage);
                return true;
            }
        }
        if prior.get_jump_duration() > 0 {
            // negative distance indicates jump
            let simstep = MSNet::get_instance().get_current_time_step();
            let startP = prior.get_position(simstep);
            let endP = self.get_current_stage().get_position(simstep);
            let st = MSPerson_MSPersonStage_Access::new(self.get_destination(), Ptr::null(), self.get_arrival_pos(), 0.0, -1., true, &startP, &endP);
            let newStage = Ptr::from_mut(Box::leak(Box::new(st))).cast::<MSStage>();
            self.myPlan.m().insert(self.myStep, newStage);
            return true;
        }
        false
    }

    /// C++: `const std::string & MSPerson::getNextEdge() const` [const]
    /// return the list of internal edges if this person is walking and the pedestrian model allows it
    // const std::string&
    // MSPerson::getNextEdge() const {
    // //    if (getCurrentStageType() == WALKING) {
    // //        MSStageWalking* walkingStage =  dynamic_cast<MSStageWalking*>(*myStep);
    // //        assert(walkingStage != 0);
    // //        const MSEdge* nextEdge = walkingStage->getPedestrianState()->getNextEdge(*walkingStage);
    // //        if (nextEdge != 0) {
    // //            return nextEdge->getID();
    // //        }
    // //    }
    // //    return StringUtils::emptyString;
    //     const MSEdge* nextEdge = getNextEdgePtr();
    //     if (nextEdge != nullptr) {
    //         return nextEdge->getID();
    //     }
    //     return StringUtils::emptyString;
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge(&self) -> &str {
        let nextEdge = self.get_next_edge_ptr();
        if let Some(e) = nextEdge.get() {
            e.get_id()
        } else {
            ""
        }
    }

    /// C++: `const MSEdge * MSPerson::getNextEdgePtr() const` [const, virtual]
    /// returns the next edge ptr if this person is walking and the pedestrian model allows it
    // const MSEdge*
    // MSPerson::getNextEdgePtr() const {
    //     if (getCurrentStageType() == MSStageType::WALKING) {
    //         MSStageWalking* walkingStage =  dynamic_cast<MSStageWalking*>(*myStep);
    //         assert(walkingStage != nullptr);
    //         return walkingStage->getPState()->getNextEdge(*walkingStage);
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        if self.get_current_stage_type() == MSStageType::WALKING {
            let walkingStage = self.get_current_stage().cast::<MSStageWalking>();
            return walkingStage.myPState.get_next_edge(&walkingStage.base);
        }
        Ptr::null()
    }

    /// C++: `bool MSPerson::isSelected() const` [const, virtual]
    /// whether this person is selected in the GUI
    // virtual bool isSelected() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        false
    }

    /// C++: `double MSPerson::getChosenSpeedFactor() const` [const, virtual]
    // inline double getChosenSpeedFactor() const {
    //         return myChosenSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        self.myChosenSpeedFactor
    }

    /// C++: `void MSPerson::setChosenSpeedFactor(const double)`
    // inline void setChosenSpeedFactor(const double factor) {
    //         myChosenSpeedFactor = factor;
    //     }
    #[allow(unused_variables)]
    pub fn set_chosen_speed_factor(&mut self, factor: f64) {
        self.myChosenSpeedFactor = factor;
    }

    /// C++: `double MSPerson::getImpatience() const` [const]
    // double
    // MSPerson::getImpatience() const {
    //     return MAX2(0., MIN2(1., getVehicleType().getImpatience()
    //                          + STEPS2TIME((*myStep)->getWaitingTime()) / MSPModel_Striping::MAX_WAIT_TOLERANCE));
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        // MSPModel_Striping::MAX_WAIT_TOLERANCE = 120 s
        let x = self.get_vehicle_type().get_impatience() + steps2time(self.get_current_stage().get_waiting_time()) / 120.;
        let mn = if x < 1. { x } else { 1. };
        if 0. < mn { mn } else { 0. }
    }

    /// C++: `bool MSPerson::isJammed() const` [const, virtual]
    /// whether the person is jammed as defined by the current pedestrian model
    // bool
    // MSPerson::isJammed() const {
    //     MSStageWalking* stage = dynamic_cast<MSStageWalking*>(getCurrentStage());
    //     if (stage != nullptr) {
    //         return stage->getPState()->isJammed();
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_jammed(&self) -> bool {
        if self.get_current_stage_type() == MSStageType::WALKING {
            let stage = self.get_current_stage().cast::<MSStageWalking>();
            return stage.myPState.is_jammed();
        }
        false
    }

    /// C++: `void MSPerson::replaceWalk(const ConstMSEdgeVector &, double, int, int)`
    /// set new walk and replace the stages with relative indices in the interval [firstIndex, nextIndex[
    // void
    // MSPerson::replaceWalk(const ConstMSEdgeVector& newEdges, double departPos, int firstIndex, int nextIndex) {
    //     assert(nextIndex > firstIndex);
    //     //std::cout << SIMTIME << " reroute person " << getID()
    //     //    << "  newEdges=" << toString(newEdges)
    //     //    << " firstIndex=" << firstIndex
    //     //    << " nextIndex=" << nextIndex
    //     //    << " departPos=" << getEdgePos()
    //     //    << " arrivalPos=" <<  getNextStage(nextIndex - 1)->getArrivalPos()
    //     //    << "\n";
    //     MSStage* const toBeReplaced = getNextStage(nextIndex - 1);
    //     MSStageWalking* newStage = new MSStageWalking(getID(), newEdges,
    //             toBeReplaced->getDestinationStop(), -1,
    //             -1,
    //             departPos,
    //             toBeReplaced->getArrivalPos(),
    //             MSPModel::UNSPECIFIED_POS_LAT);
    //     appendStage(newStage, nextIndex);
    //     // remove stages in reverse order so that proceed will only be called at the last removal
    //     for (int i = nextIndex - 1; i >= firstIndex; i--) {
    //         //std::cout << " removeStage=" << i << "\n";
    //         removeStage(i);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn replace_walk(&mut self, newEdges: &ConstMSEdgeVector, departPos: f64, firstIndex: i32, nextIndex: i32) {
        assert!(nextIndex > firstIndex);
        let toBeReplaced = self.get_next_stage(nextIndex - 1);
        let newStage = MSStageWalking::new(self.get_id(), newEdges, toBeReplaced.get_destination_stop(), -1, -1., departPos, toBeReplaced.get_arrival_pos(), std::f64::MAX, -1, "");
        let newPtr = Ptr::from_mut(Box::leak(Box::new(newStage))).cast::<MSStage>();
        self.append_stage(newPtr, nextIndex);
        // remove stages in reverse order so that proceed will only be called at the last removal
        let mut i = nextIndex - 1;
        while i >= firstIndex {
            self.remove_stage(i, true);
            i -= 1;
        }
    }

    /// C++: `double MSPerson::getTimegapCrossing() const` [const]
    // inline double getTimegapCrossing() const {
    //         return myTimegapCrossing;
    //     }
    #[allow(unused_variables)]
    pub fn get_timegap_crossing(&self) -> f64 {
        self.myTimegapCrossing
    }

    /// C++: `Influencer & MSPerson::getInfluencer()`
    /// Returns the velocity/lane influencer
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_influencer(&mut self) -> &mut MSPerson_Influencer {
        if self.myInfluencer.is_null() {
            self.myInfluencer = Ptr::from_raw(Box::into_raw(Box::new(MSPerson_Influencer::new())));
        }
        self.myInfluencer.m()
    }

    /// C++: `const Influencer * MSPerson::getInfluencer() const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_influencer_2(&self) -> Ptr<MSPerson_Influencer> {
        self.myInfluencer
    }

    /// C++: `bool MSPerson::hasInfluencer() const` [const, virtual]
    // bool hasInfluencer() const {
    //         return myInfluencer != 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        !self.myInfluencer.is_null()
    }

    /// C++: `void MSPerson::setRemoteState(Position)`
    /// sets position outside the road network
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_remote_state(&mut self, xyPos: Position) {
        panic!("MSPerson::setRemoteState is declared in C++ but never defined");
    }

    /// C++: `void MSPerson::MSPerson(const MSPerson &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSPerson) -> Self {
        panic!("MSPerson copy constructor is invalidated in C++ (private)");
    }

    /// C++: `MSPerson & MSPerson::operator=(const MSPerson &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPerson) -> &mut MSPerson {
        panic!("MSPerson assignment operator is invalidated in C++ (private)");
    }

}

/// An intermediate stage performing the access from or to public transport as given by the access elements of the public transport stop. The travel time is computed by the simulation
/// C++ class `MSPerson::MSPersonStage_Access` (microsim/transportables/MSPerson.h)
#[repr(C)]
pub struct MSPerson_MSPersonStage_Access {
    /// C++ base class `MSStage`
    pub base: MSStage,
    /// the origin edge
    pub myOrigin: Ptr<MSEdge>,
    pub myDist: f64,
    pub myAmExit: bool,
    pub myEstimatedArrival: SUMOTime,
    pub myPath: PositionVector,
}

impl std::ops::Deref for MSPerson_MSPersonStage_Access {
    type Target = MSStage;
    fn deref(&self) -> &MSStage {
        &self.base
    }
}
impl std::ops::DerefMut for MSPerson_MSPersonStage_Access {
    fn deref_mut(&mut self) -> &mut MSStage {
        &mut self.base
    }
}

impl MSPerson_MSPersonStage_Access {
    /// C++: `void MSPersonStage_Access::MSPersonStage_Access(const MSEdge *, MSStoppingPlace *, const double, const double, const double, const bool, const Position &, const Position &)`
    /// constructor
    // MSPerson::MSPersonStage_Access::MSPersonStage_Access(const MSEdge* destination, MSStoppingPlace* toStop,
    //         const double arrivalPos, const double arrivalPosLat, const double dist, const bool isExit,
    //         const Position& startPos, const Position& endPos) :
    //     MSStage(MSStageType::ACCESS, destination, toStop, arrivalPos, arrivalPosLat),
    //     myDist(dist), myAmExit(isExit) {
    //     myPath.push_back(startPos);
    //     myPath.push_back(endPos);
    // }
    #[allow(unused_variables)]
    pub fn new(destination: Ptr<MSEdge>, toStop: Ptr<MSStoppingPlace>, arrivalPos: f64, arrivalPosLat: f64, dist: f64, isExit: bool, startPos: &Position, endPos: &Position) -> Self {
        let mut path = PositionVector(Vec::new());
        path.push(*startPos);
        path.push(*endPos);
        MSPerson_MSPersonStage_Access {
            base: MSStage::new(MSStageType::ACCESS, destination, toStop, arrivalPos, arrivalPosLat, ""),
            myOrigin: Ptr::null(),
            myDist: dist,
            myAmExit: isExit,
            myEstimatedArrival: 0,
            myPath: path,
        }
    }

    /// C++: `void MSPersonStage_Access::~MSPersonStage_Access() noexcept` [virtual]
    /// destructor
    // MSPerson::MSPersonStage_Access::~MSPersonStage_Access() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `MSStage * MSPersonStage_Access::clone() const` [const, virtual]
    // MSStage*
    // MSPerson::MSPersonStage_Access::clone() const {
    //     return new MSPersonStage_Access(myDestination, myDestinationStop, myArrivalPos, myArrivalPosLat, myDist, myAmExit, myPath.front(), myPath.back());
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        let st = MSPerson_MSPersonStage_Access::new(self.myDestination, self.myDestinationStop, self.myArrivalPos, self.myArrivalPosLat, self.myDist, self.myAmExit, &self.myPath[0], &self.myPath[self.myPath.len() - 1]);
        Ptr::from_mut(Box::leak(Box::new(st))).cast::<MSStage>()
    }

    /// C++: `bool MSPersonStage_Access::proceed(MSNet *, MSTransportable *, SUMOTime, MSStage *)` [virtual]
    /// proceeds to the next step
    // bool
    // MSPerson::MSPersonStage_Access::proceed(MSNet* net, MSTransportable* person, SUMOTime now, MSStage* previous) {
    //     myDeparted = now;
    //     if (myDist >= 0) {
    //         myEstimatedArrival = now + TIME2STEPS(myDist / person->getMaxSpeed());
    //     } else {
    //         myEstimatedArrival = now + previous->getJumpDuration();
    //     }
    //     // TODO myEstimatedArrival is not a multiple of DELTA_T here. This might give a problem because the destination position will not be reached precisely
    //     MSEdge* edge = myDestinationStop != nullptr ? &myDestinationStop->getLane().getEdge() : const_cast<MSEdge*>(myDestination);
    //     net->getBeginOfTimestepEvents()->addEvent(new ProceedCmd(person, edge), myEstimatedArrival);
    //     net->getPersonControl().startedAccess();
    //     edge->addTransportable(person);
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, person: Ptr<MSTransportable>, now: SUMOTime, previous: Ptr<MSStage>) -> bool {
        self.myDeparted = now;
        if self.myDist >= 0. {
            self.myEstimatedArrival = now + time2steps(self.myDist / person.get_max_speed());
        } else {
            self.myEstimatedArrival = now + previous.get_jump_duration();
        }
        // TODO myEstimatedArrival is not a multiple of DELTA_T here. This might give a problem because the destination position will not be reached precisely
        let edge = if !self.myDestinationStop.is_null() { Ptr::from_mut(self.myDestinationStop.get_lane().get_edge()) } else { self.myDestination };
        let cmd = MSPersonStage_Access_ProceedCmd::new(person, edge);
        let cmdPtr: Ptr<dyn Command> = Ptr::from_mut(Box::leak(Box::new(cmd)) as &mut dyn Command);
        net.myBeginOfTimestepEvents.m().add_event(cmdPtr, self.myEstimatedArrival);
        // shortcut: MSTransportableControl::startedAccess is not ported yet, so the access counter is not updated
        edge.add_transportable(person);
        true
    }

    /// C++: `std::string MSPersonStage_Access::getStageDescription(const bool) const` [const, virtual]
    /// returns the stage description as a string
    // std::string
    // MSPerson::MSPersonStage_Access::getStageDescription(const bool /* isPerson */) const {
    //     return "access";
    // }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        "access".to_string()
    }

    /// C++: `std::string MSPersonStage_Access::getStageSummary(const bool) const` [const, virtual]
    // std::string
    // MSPerson::MSPersonStage_Access::getStageSummary(const bool /* isPerson */) const {
    //     if (myDestination == nullptr) {
    //         return ("jump to edge '") + getDestination()->getID() + "'";
    //     } else {
    //         return (myAmExit ? "access from stop '" : "access to stop '") + getDestinationStop()->getID() + "'";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, isPerson: bool) -> String {
        if self.myDestination.is_null() {
            format!("jump to edge '{}'", self.get_destination().get_id())
        } else {
            format!("{} '{}'", if self.myAmExit { "access from stop" } else { "access to stop" }, self.get_destination_stop().get_id())
        }
    }

    /// C++: `Position MSPersonStage_Access::getPosition(SUMOTime) const` [const, virtual]
    // Position
    // MSPerson::MSPersonStage_Access::getPosition(SUMOTime now) const {
    //     return myPath.positionAtOffset(myPath.length() * (double)(now - myDeparted) / (double)(myEstimatedArrival - myDeparted));
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        self.myPath.position_at_offset(self.myPath.length() * ((now - self.myDeparted) as f64) / ((self.myEstimatedArrival - self.myDeparted) as f64), 0.)
    }

    /// C++: `double MSPersonStage_Access::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSPerson::MSPersonStage_Access::getAngle(SUMOTime /* now */) const {
    //     return myPath.angleAt2D(0);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        self.myPath.angle_at2d(0)
    }

    /// C++: `double MSPersonStage_Access::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //             return myDist;
    //         }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        self.myDist
    }

    /// C++: `double MSPersonStage_Access::getSpeed() const` [const, virtual]
    /// the speed of the person in this stage
    // double
    // MSPerson::MSPersonStage_Access::getSpeed() const {
    //     return myDist / STEPS2TIME(MAX2((SUMOTime)1, myEstimatedArrival - myDeparted));
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        let denom = if 1 > self.myEstimatedArrival - self.myDeparted { 1 } else { self.myEstimatedArrival - self.myDeparted };
        self.myDist / steps2time(denom)
    }

    /// C++: `void MSPersonStage_Access::tripInfoOutput(OutputDevice &, const MSTransportable *const) const` [const, virtual]
    /// Called on writing tripinfo output
    // void
    // MSPerson::MSPersonStage_Access::tripInfoOutput(OutputDevice& os, const MSTransportable* const) const {
    //     os.openTag("access");
    //     if (getDestinationStop() != nullptr) {
    //         os.writeAttr("stop", getDestinationStop()->getID());
    //     }
    //     os.writeAttr("depart", time2string(myDeparted));
    //     os.writeAttr("arrival", myArrived >= 0 ? time2string(myArrived) : "-1");
    //     os.writeAttr("duration", myArrived > 0 ? time2string(getDuration()) : "-1");
    //     os.writeAttr("routeLength", myDist);
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice, transportable: Ptr<MSTransportable>) {
        os.open_tag("access");
        if !self.myDestinationStop.is_null() {
            os.write_attr("stop", self.myDestinationStop.get_id());
        }
        os.write_attr("depart", &time2string(self.myDeparted));
        let arrivalStr = if self.myArrived >= 0 { time2string(self.myArrived) } else { "-1".to_string() };
        os.write_attr("arrival", &arrivalStr);
        let durationStr = if self.myArrived > 0 { time2string(self.get_duration()) } else { "-1".to_string() };
        os.write_attr("duration", &durationStr);
        os.write_attr("routeLength", &self.myDist);
        os.close_tag("");
    }

    /// C++: `void MSPersonStage_Access::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output. Currently does nothing.
    // void routeOutput(const bool /*isPerson*/, OutputDevice& /*os*/, const bool /*withRouteLength*/, const MSStage* const /*previous*/, const bool /*withTiming*/, const bool /*saveState*/ = false) const {}
    #[allow(unused_variables)]
    pub fn route_output(&self, arg0: bool, arg1: &mut OutputDevice, arg2: bool, arg3: Ptr<MSStage>, arg4: bool, arg5: bool) {
        // Called on writing vehroute output. Currently does nothing.
    }

    /// C++: `void MSPersonStage_Access::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSPerson::MSPersonStage_Access::saveState(std::ostringstream& out, MSTransportable* /*transportable*/) {
    //     out << " " << myDeparted << " " << myEstimatedArrival;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut String /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        out.push_str(&format!(" {} {}", self.myDeparted, self.myEstimatedArrival));
    }

    /// C++: `void MSPersonStage_Access::loadState(MSTransportable *, std::istringstream &)` [virtual]
    /// Reconstructs the current state
    // void
    // MSPerson::MSPersonStage_Access::loadState(MSTransportable* person, std::istringstream& state) {
    //     state >> myDeparted;
    //     state >> myEstimatedArrival;
    //     MSNet* net = MSNet::getInstance();
    //     MSEdge* edge = myDestinationStop != nullptr ? &myDestinationStop->getLane().getEdge() : const_cast<MSEdge*>(myDestination);
    //     net->getBeginOfTimestepEvents()->addEvent(new ProceedCmd(person, edge), myEstimatedArrival);
    //     net->getPersonControl().startedAccess();
    //     edge->addTransportable(person);
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, person: Ptr<MSTransportable>, state: &mut std::str::SplitWhitespace<'_> /* std::istringstream */) {
        self.myDeparted = state.next().and_then(|s| s.parse().ok()).unwrap_or_default();
        self.myEstimatedArrival = state.next().and_then(|s| s.parse().ok()).unwrap_or_default();
        let net = MSNet::get_instance();
        let edge = if !self.myDestinationStop.is_null() { Ptr::from_mut(self.myDestinationStop.get_lane().get_edge()) } else { self.myDestination };
        let cmd = MSPersonStage_Access_ProceedCmd::new(person, edge);
        let cmdPtr: Ptr<dyn Command> = Ptr::from_mut(Box::leak(Box::new(cmd)) as &mut dyn Command);
        net.myBeginOfTimestepEvents.m().add_event(cmdPtr, self.myEstimatedArrival);
        // shortcut: MSTransportableControl::startedAccess is not ported yet, so the access counter is not updated
        edge.add_transportable(person);
    }

}

/// C++ class `MSPersonStage_Access::ProceedCmd` (microsim/transportables/MSPerson.h)
#[repr(C)]
pub struct MSPersonStage_Access_ProceedCmd {
    /// C++ base class `Command`
    pub base: Unmapped /* Command */,
    pub myPerson: Ptr<MSTransportable>,
    pub myStopEdge: Ptr<MSEdge>,
}

impl std::ops::Deref for MSPersonStage_Access_ProceedCmd {
    type Target = Unmapped /* Command */;
    fn deref(&self) -> &Unmapped /* Command */ {
        &self.base
    }
}
impl std::ops::DerefMut for MSPersonStage_Access_ProceedCmd {
    fn deref_mut(&mut self) -> &mut Unmapped /* Command */ {
        &mut self.base
    }
}

impl MSPersonStage_Access_ProceedCmd {
    /// C++: `void ProceedCmd::ProceedCmd(MSTransportable *, MSEdge *)`
    // ProceedCmd(MSTransportable* person, MSEdge* edge) : myPerson(person), myStopEdge(edge) {}
    #[allow(unused_variables)]
    pub fn new(person: Ptr<MSTransportable>, edge: Ptr<MSEdge>) -> Self {
        MSPersonStage_Access_ProceedCmd { base: (), myPerson: person, myStopEdge: edge }
    }

    /// C++: `void ProceedCmd::~ProceedCmd() noexcept` [virtual]
    // ~ProceedCmd() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // empty C++ destructor body
    }

    /// C++: `SUMOTime ProceedCmd::execute(SUMOTime)` [virtual]
    // SUMOTime
    // MSPerson::MSPersonStage_Access::ProceedCmd::execute(SUMOTime currentTime) {
    //     MSNet::getInstance()->getPersonControl().endedAccess();
    //     myStopEdge->removeTransportable(myPerson);
    //     if (!myPerson->proceed(MSNet::getInstance(), currentTime)) {
    //         MSNet::getInstance()->getPersonControl().erase(myPerson);
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        // shortcut: MSTransportableControl::endedAccess is not ported yet, so the access counter is not updated
        self.myStopEdge.remove_transportable(self.myPerson);
        if !self.myPerson.m().proceed(MSNet::get_instance(), currentTime, false) {
            MSNet::get_instance().m().get_person_control().erase(self.myPerson);
        }
        0
    }

    /// C++: `ProceedCmd & ProceedCmd::operator=(const ProceedCmd &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPersonStage_Access_ProceedCmd) -> &mut MSPersonStage_Access_ProceedCmd {
        panic!("ProceedCmd assignment operator is invalidated in C++");
    }

}

impl Command for MSPersonStage_Access_ProceedCmd {
    fn execute(&mut self, currentTime: SUMOTime) -> SUMOTime {
        MSPersonStage_Access_ProceedCmd::execute(self, currentTime)
    }
}

fn ms_person_get_parameter(o: &SUMOTrafficObject) -> &SUMOVehicleParameter {
    let p: &MSPerson = Ptr::new(o).cast::<MSPerson>().get().unwrap();
    p.get_parameter()
}
