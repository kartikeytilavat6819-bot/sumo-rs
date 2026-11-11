//! Port of microsim/transportables/MSPerson.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// Changes the wished person speed and position
/// C++ class `MSPerson::Influencer` (microsim/transportables/MSPerson.h)
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
        todo!("port Influencer::Influencer")
    }

    /// C++: `void Influencer::~Influencer()`
    /// Destructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port Influencer::~Influencer")
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
        todo!("port Influencer::setRemoteControlled")
    }

    /// C++: `SUMOTime Influencer::getLastAccessTimeStep() const` [const]
    // SUMOTime getLastAccessTimeStep() const {
    //             return myLastRemoteAccess;
    //         }
    #[allow(unused_variables)]
    pub fn get_last_access_time_step(&self) -> SUMOTime {
        todo!("port Influencer::getLastAccessTimeStep")
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
        todo!("port Influencer::postProcessRemoteControl")
    }

    /// C++: `bool Influencer::isRemoteControlled() const` [const]
    // bool
    // MSPerson::Influencer::isRemoteControlled() const {
    //     return myLastRemoteAccess == MSNet::getInstance()->getCurrentTimeStep();
    // }
    #[allow(unused_variables)]
    pub fn is_remote_controlled(&self) -> bool {
        todo!("port Influencer::isRemoteControlled")
    }

    /// C++: `bool Influencer::isRemoteAffected(SUMOTime) const` [const]
    // bool
    // MSPerson::Influencer::isRemoteAffected(SUMOTime t) const {
    //     return myLastRemoteAccess >= t - TIME2STEPS(10);
    // }
    #[allow(unused_variables)]
    pub fn is_remote_affected(&self, t: SUMOTime) -> bool {
        todo!("port Influencer::isRemoteAffected")
    }

}

/// The class holds a simulated person together with her movement stages
/// C++ class `MSPerson` (microsim/transportables/MSPerson.h)
pub struct MSPerson {
    /// C++ base class `MSTransportable`
    pub base: MSTransportable,
    /// An instance of a speed/position influencing instance; built in "getInfluencer"
    pub myInfluencer: Ptr<MSVehicle_Influencer>,
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
        todo!("port MSPerson::MSPerson")
    }

    /// C++: `void MSPerson::~MSPerson() noexcept` [virtual]
    /// destructor
    // MSPerson::~MSPerson() {
    //     delete myInfluencer;
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSPerson::~MSPerson")
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
        todo!("port MSPerson::checkAccess")
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
        todo!("port MSPerson::getNextEdge")
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
        todo!("port MSPerson::getNextEdgePtr")
    }

    /// C++: `bool MSPerson::isSelected() const` [const, virtual]
    /// whether this person is selected in the GUI
    // virtual bool isSelected() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        todo!("port MSPerson::isSelected")
    }

    /// C++: `double MSPerson::getChosenSpeedFactor() const` [const, virtual]
    // inline double getChosenSpeedFactor() const {
    //         return myChosenSpeedFactor;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        todo!("port MSPerson::getChosenSpeedFactor")
    }

    /// C++: `void MSPerson::setChosenSpeedFactor(const double)`
    // inline void setChosenSpeedFactor(const double factor) {
    //         myChosenSpeedFactor = factor;
    //     }
    #[allow(unused_variables)]
    pub fn set_chosen_speed_factor(&mut self, factor: f64) {
        todo!("port MSPerson::setChosenSpeedFactor")
    }

    /// C++: `double MSPerson::getImpatience() const` [const]
    // double
    // MSPerson::getImpatience() const {
    //     return MAX2(0., MIN2(1., getVehicleType().getImpatience()
    //                          + STEPS2TIME((*myStep)->getWaitingTime()) / MSPModel_Striping::MAX_WAIT_TOLERANCE));
    // }
    #[allow(unused_variables)]
    pub fn get_impatience(&self) -> f64 {
        todo!("port MSPerson::getImpatience")
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
        todo!("port MSPerson::isJammed")
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
        todo!("port MSPerson::replaceWalk")
    }

    /// C++: `double MSPerson::getTimegapCrossing() const` [const]
    // inline double getTimegapCrossing() const {
    //         return myTimegapCrossing;
    //     }
    #[allow(unused_variables)]
    pub fn get_timegap_crossing(&self) -> f64 {
        todo!("port MSPerson::getTimegapCrossing")
    }

    /// C++: `Influencer & MSPerson::getInfluencer()`
    /// Returns the velocity/lane influencer
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_influencer(&mut self) -> &mut MSVehicle_Influencer {
        todo!("port MSPerson::getInfluencer")
    }

    /// C++: `const Influencer * MSPerson::getInfluencer() const` [const]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_influencer_2(&self) -> Ptr<MSVehicle_Influencer> {
        todo!("port MSPerson::getInfluencer")
    }

    /// C++: `bool MSPerson::hasInfluencer() const` [const, virtual]
    // bool hasInfluencer() const {
    //         return myInfluencer != 0;
    //     }
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        todo!("port MSPerson::hasInfluencer")
    }

    /// C++: `void MSPerson::setRemoteState(Position)`
    /// sets position outside the road network
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn set_remote_state(&mut self, xyPos: Position) {
        todo!("port MSPerson::setRemoteState")
    }

    /// C++: `void MSPerson::MSPerson(const MSPerson &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSPerson) -> Self {
        todo!("port MSPerson::MSPerson")
    }

    /// C++: `MSPerson & MSPerson::operator=(const MSPerson &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPerson) -> &mut MSPerson {
        todo!("port MSPerson::operator=")
    }

}

/// An intermediate stage performing the access from or to public transport as given by the access elements of the public transport stop. The travel time is computed by the simulation
/// C++ class `MSPerson::MSPersonStage_Access` (microsim/transportables/MSPerson.h)
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
        todo!("port MSPersonStage_Access::MSPersonStage_Access")
    }

    /// C++: `void MSPersonStage_Access::~MSPersonStage_Access() noexcept` [virtual]
    /// destructor
    // MSPerson::MSPersonStage_Access::~MSPersonStage_Access() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port MSPersonStage_Access::~MSPersonStage_Access")
    }

    /// C++: `MSStage * MSPersonStage_Access::clone() const` [const, virtual]
    // MSStage*
    // MSPerson::MSPersonStage_Access::clone() const {
    //     return new MSPersonStage_Access(myDestination, myDestinationStop, myArrivalPos, myArrivalPosLat, myDist, myAmExit, myPath.front(), myPath.back());
    // }
    #[allow(unused_variables)]
    pub fn clone(&self) -> Ptr<MSStage> {
        todo!("port MSPersonStage_Access::clone")
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
        todo!("port MSPersonStage_Access::proceed")
    }

    /// C++: `std::string MSPersonStage_Access::getStageDescription(const bool) const` [const, virtual]
    /// returns the stage description as a string
    // std::string
    // MSPerson::MSPersonStage_Access::getStageDescription(const bool /* isPerson */) const {
    //     return "access";
    // }
    #[allow(unused_variables)]
    pub fn get_stage_description(&self, isPerson: bool) -> String {
        todo!("port MSPersonStage_Access::getStageDescription")
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
        todo!("port MSPersonStage_Access::getStageSummary")
    }

    /// C++: `Position MSPersonStage_Access::getPosition(SUMOTime) const` [const, virtual]
    // Position
    // MSPerson::MSPersonStage_Access::getPosition(SUMOTime now) const {
    //     return myPath.positionAtOffset(myPath.length() * (double)(now - myDeparted) / (double)(myEstimatedArrival - myDeparted));
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        todo!("port MSPersonStage_Access::getPosition")
    }

    /// C++: `double MSPersonStage_Access::getAngle(SUMOTime) const` [const, virtual]
    // double
    // MSPerson::MSPersonStage_Access::getAngle(SUMOTime /* now */) const {
    //     return myPath.angleAt2D(0);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        todo!("port MSPersonStage_Access::getAngle")
    }

    /// C++: `double MSPersonStage_Access::getDistance() const` [const, virtual]
    /// get travel distance in this stage
    // double getDistance() const {
    //             return myDist;
    //         }
    #[allow(unused_variables)]
    pub fn get_distance(&self) -> f64 {
        todo!("port MSPersonStage_Access::getDistance")
    }

    /// C++: `double MSPersonStage_Access::getSpeed() const` [const, virtual]
    /// the speed of the person in this stage
    // double
    // MSPerson::MSPersonStage_Access::getSpeed() const {
    //     return myDist / STEPS2TIME(MAX2((SUMOTime)1, myEstimatedArrival - myDeparted));
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port MSPersonStage_Access::getSpeed")
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
        todo!("port MSPersonStage_Access::tripInfoOutput")
    }

    /// C++: `void MSPersonStage_Access::routeOutput(const bool, OutputDevice &, const bool, const MSStage *const, const bool, const bool) const` [const, virtual]
    /// Called on writing vehroute output. Currently does nothing.
    // void routeOutput(const bool /*isPerson*/, OutputDevice& /*os*/, const bool /*withRouteLength*/, const MSStage* const /*previous*/, const bool /*withTiming*/, const bool /*saveState*/ = false) const {}
    #[allow(unused_variables)]
    pub fn route_output(&self, arg0: bool, arg1: &mut OutputDevice, arg2: bool, arg3: Ptr<MSStage>, arg4: bool, arg5: bool) {
        todo!("port MSPersonStage_Access::routeOutput")
    }

    /// C++: `void MSPersonStage_Access::saveState(std::ostringstream &, MSTransportable *)` [virtual]
    /// Saves the current state into the given stream
    // void
    // MSPerson::MSPersonStage_Access::saveState(std::ostringstream& out, MSTransportable* /*transportable*/) {
    //     out << " " << myDeparted << " " << myEstimatedArrival;
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut Unmapped /* std::ostringstream */, transportable: Ptr<MSTransportable>) {
        todo!("port MSPersonStage_Access::saveState")
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
    pub fn load_state(&mut self, person: Ptr<MSTransportable>, state: &mut Unmapped /* std::istringstream */) {
        todo!("port MSPersonStage_Access::loadState")
    }

}

/// C++ class `MSPersonStage_Access::ProceedCmd` (microsim/transportables/MSPerson.h)
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
        todo!("port ProceedCmd::ProceedCmd")
    }

    /// C++: `void ProceedCmd::~ProceedCmd() noexcept` [virtual]
    // ~ProceedCmd() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port ProceedCmd::~ProceedCmd")
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
        todo!("port ProceedCmd::execute")
    }

    /// C++: `ProceedCmd & ProceedCmd::operator=(const ProceedCmd &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSPersonStage_Access_ProceedCmd) -> &mut MSPersonStage_Access_ProceedCmd {
        todo!("port ProceedCmd::operator=")
    }

}

