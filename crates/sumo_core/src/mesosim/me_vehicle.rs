//! Port of mesosim/MEVehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::time::{SUMOTIME_MAX, SUMOTIME_MIN, steps2time, delta_t};

// C++ `static const int MESegment::PARKING_QUEUE = -1` (used by MEVehicle)
const PARKING_QUEUE: i32 = -1;

// C++ SUMOVehicleParameter::Stop parametersSet flags (utils/vehicle/SUMOVehicleParameter.h)
const STOP_ENDED_SET: i32 = 2 << 16;
const STOP_STARTED_SET: i32 = 2 << 17;

// Reads the next whitespace-separated token like C++ `istringstream >> x` (default on failure)
fn next_token<'a, T: std::str::FromStr + Default>(it: &mut impl Iterator<Item = &'a str>) -> T {
    it.next().and_then(|s| s.parse().ok()).unwrap_or_default()
}

// C++ MIN2 on doubles (same comparison order as the C++ macro)
fn min2(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

// C++ MAX2 on doubles (a > b ? a : b)
fn max2(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}

/// A vehicle from the mesoscopic point of view
/// C++ class `MEVehicle` (mesosim/MEVehicle.h)
#[repr(C)]
pub struct MEVehicle {
    /// C++ base class `MSBaseVehicle`
    pub base: MSBaseVehicle,
    /// The segment the vehicle is at
    pub mySegment: Ptr<MESegment>,
    /// Index of the que the vehicle is in (important for multiqueue extension)
    pub myQueIndex: i32,
    /// The (planned) time of leaving the segment (cell)
    pub myEventTime: SUMOTime,
    /// The index of the vehicle's event (orders events with the same time)
    pub myEventIndex: i64,
    /// The time the vehicle entered its current segment
    pub myLastEntryTime: SUMOTime,
    /// The time at which the vehicle was blocked on its current segment
    pub myBlockTime: SUMOTime,
    /// The expected time of leaving the segment when following the leader at free-flow headway
    pub myUnqueuedEventTime: SUMOTime,
    /// Whether this vehicle was jammed on it's previous normal segment
    pub myWasJammed: bool,
    /// An instance of a velocity/lane influencing instance; built in "getInfluencer"
    pub myInfluencer: Ptr<MSBaseVehicle_BaseInfluencer>,
}

impl std::ops::Deref for MEVehicle {
    type Target = MSBaseVehicle;
    fn deref(&self) -> &MSBaseVehicle {
        &self.base
    }
}
impl std::ops::DerefMut for MEVehicle {
    fn deref_mut(&mut self) -> &mut MSBaseVehicle {
        &mut self.base
    }
}

impl MEVehicle {
    /// C++: `void MEVehicle::MEVehicle(SUMOVehicleParameter *, ConstMSRoutePtr, MSVehicleType *, const double)`
    /// Constructor
    // MEVehicle::MEVehicle(SUMOVehicleParameter* pars, ConstMSRoutePtr route,
    //                      MSVehicleType* type, const double speedFactor) :
    //     MSBaseVehicle(pars, route, type, speedFactor),
    //     mySegment(nullptr),
    //     myQueIndex(0),
    //     myEventTime(SUMOTime_MIN),
    //     myEventIndex(-1),
    //     myLastEntryTime(SUMOTime_MIN),
    //     myBlockTime(SUMOTime_MAX),
    //     myUnqueuedEventTime(SUMOTime_MIN),
    //     myWasJammed(false),
    //     myInfluencer(nullptr) {
    // }
    #[allow(unused_variables)]
    pub fn new(pars: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, speedFactor: f64) -> Self {
        MEVehicle {
            base: MSBaseVehicle::new(pars, route, type_, speedFactor),
            mySegment: Ptr::null(),
            myQueIndex: 0,
            myEventTime: SUMOTIME_MIN,
            myEventIndex: -1,
            myLastEntryTime: SUMOTIME_MIN,
            myBlockTime: SUMOTIME_MAX,
            myUnqueuedEventTime: SUMOTIME_MIN,
            myWasJammed: false,
            myInfluencer: Ptr::null(),
        }
    }

    /// C++: `double MEVehicle::getPositionOnLane() const` [const, virtual]
    /// Get the vehicle's position along the lane
    // double
    // MEVehicle::getPositionOnLane() const {
    //     if (MSGlobals::gMesoInterpolatePos) {
    //         // interpolation may cause problems with arrivals and calibrators
    //         const auto& mesoPos = getEdge()->getMesoPositions();
    //         const auto& posIt = mesoPos.find(this);
    //         if (posIt != mesoPos.end()) {
    //             return posIt->second.first;
    //         }
    //     }
    //     return mySegment == nullptr ? 0. : (double)mySegment->getIndex() * mySegment->getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_position_on_lane(&self) -> f64 {
        if MSGlobals::meso_interpolate_pos() {
            // interpolation may cause problems with arrivals and calibrators
            let edge = self.get_edge();
            let mesoPos = edge.get_meso_positions();
            if let Some(p) = mesoPos.get(&Ptr::new(self)) {
                return p.0;
            }
        }
        if self.mySegment.is_null() { 0. } else { (self.mySegment.get_index() as f64) * self.mySegment.get_length() }
    }

    /// C++: `double MEVehicle::getBackPositionOnLane(const MSLane *) const` [const, virtual]
    /// Get the vehicle's position relative to the given lane
    // double
    // MEVehicle::getBackPositionOnLane(const MSLane* /* lane */) const {
    //     return getPositionOnLane() - getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_back_position_on_lane(&self, lane: Ptr<MSLane>) -> f64 {
        self.get_position_on_lane() - self.get_vehicle_type().get_length()
    }

    /// C++: `double MEVehicle::getAngle() const` [const, virtual]
    /// Returns the vehicle's direction in degrees
    // double
    // MEVehicle::getAngle() const {
    //     const MSLane* const lane = getEdge()->getLanes()[MAX2(0, getQueIndex())];
    //     return lane->getShape().rotationAtOffset(lane->interpolateLanePosToGeometryPos(getPositionOnLane()));
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        let lane = self.get_edge().get_lanes()[std::cmp::max(0, self.get_que_index()) as usize];
        lane.get_shape(false).rotation_at_offset(lane.interpolate_lane_pos_to_geometry_pos(self.get_position_on_lane()))
    }

    /// C++: `double MEVehicle::getSlope() const` [const, virtual]
    /// Returns the slope of the road at vehicle's position in degrees
    // double
    // MEVehicle::getSlope() const {
    //     const MSLane* const lane = getEdge()->getLanes()[MAX2(0, getQueIndex())];
    //     return lane->getShape().slopeDegreeAtOffset(lane->interpolateLanePosToGeometryPos(getPositionOnLane()
    //                 - (MSGlobals::gSlopeCentered ? getLength() / 2 : 0)));
    // }
    #[allow(unused_variables)]
    pub fn get_slope(&self) -> f64 {
        let lane = self.get_edge().get_lanes()[std::cmp::max(0, self.get_que_index()) as usize];
        lane.get_shape(false).slope_degree_at_offset(lane.interpolate_lane_pos_to_geometry_pos(
            self.get_position_on_lane() - (if MSGlobals::slope_centered() { self.get_length() / 2. } else { 0. })))
    }

    /// C++: `const MSEdge * MEVehicle::getCurrentEdge() const` [const, virtual]
    /// Returns the edge the vehicle is currently at (possibly an internal edge)
    // const MSEdge*
    // MEVehicle::getCurrentEdge() const {
    //     return mySegment != nullptr ? &mySegment->getEdge() : getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        if !self.mySegment.is_null() { Ptr::new(self.mySegment.get_edge()) } else { self.get_edge() }
    }

    /// C++: `const MSLane * MEVehicle::getLane() const` [const, virtual]
    /// Returns the lane the vehicle is on
    // const MSLane* getLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        Ptr::null()
    }

    /// C++: `const MSLane * MEVehicle::getBackLane() const` [const, virtual]
    // const MSLane* getBackLane() const {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_back_lane(&self) -> Ptr<MSLane> {
        Ptr::null()
    }

    /// C++: `Position MEVehicle::getPosition(const double) const` [const, virtual]
    /// Return current position (x/y, cartesian)
    // Position
    // MEVehicle::getPosition(const double offset) const {
    //     const MSLane* const lane = getEdge()->getLanes()[MAX2(0, getQueIndex())];
    //     return lane->geometryPositionAtOffset(getPositionOnLane() + offset);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, offset: f64) -> Position {
        let lane = self.get_edge().get_lanes()[std::cmp::max(0, self.get_que_index()) as usize];
        lane.geometry_position_at_offset(self.get_position_on_lane() + offset, 0.)
    }

    /// C++: `PositionVector MEVehicle::getBoundingBox(double) const` [const, virtual]
    /// get bounding rectangle
    // PositionVector
    // MEVehicle::getBoundingBox(double offset) const {
    //     double a = getAngle() + M_PI; // angle pointing backwards
    //     double l = getLength();
    //     Position pos = getPosition();
    //     Position backPos = pos + Position(l * cos(a), l * sin(a));
    //     PositionVector centerLine;
    //     centerLine.push_back(pos);
    //     centerLine.push_back(backPos);
    //     if (offset != 0) {
    //         centerLine.extrapolate2D(offset);
    //     }
    //     PositionVector result = centerLine;
    //     result.move2side(MAX2(0.0, 0.5 * myType->getWidth() + offset));
    //     centerLine.move2side(MIN2(0.0, -0.5 * myType->getWidth() - offset));
    //     result.append(centerLine.reverse(), POSITION_EPS);
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self, offset: f64) -> PositionVector {
        let a = self.get_angle() + std::f64::consts::PI; // angle pointing backwards
        let l = self.get_length();
        let pos = self.get_position(0.);
        let back_pos = pos + Position::new(l * a.cos(), l * a.sin());
        let mut center_line = PositionVector(vec![pos, back_pos]);
        if offset != 0. {
            center_line.extrapolate2d(offset, false);
        }
        let mut result = center_line.clone();
        result.move2side(max2(0.0, 0.5 * self.get_vehicle_type().get_width() + offset), 100.);
        center_line.move2side(min2(0.0, -0.5 * self.get_vehicle_type().get_width() - offset), 100.);
        result.append(&center_line.reverse(), utils::geom::POSITION_EPS);
        result
    }

    /// C++: `double MEVehicle::getSpeed() const` [const, virtual]
    /// Returns the vehicle's estimated speed assuming no delays
    // double
    // MEVehicle::getSpeed() const {
    //     if (getWaitingTime() > 0 || isStopped()) {
    //         return 0;
    //     } else {
    //         return getAverageSpeed();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        if self.get_waiting_time(false) > 0 || self.is_stopped() {
            0.
        } else {
            self.get_average_speed()
        }
    }

    /// C++: `double MEVehicle::getAverageSpeed() const` [const]
    /// Returns the vehicle's estimated average speed on the segment assuming no further delays
    // double
    // MEVehicle::getAverageSpeed() const {
    //     // cache for thread safety
    //     MESegment* s = mySegment;
    //     if (s == nullptr || myQueIndex == MESegment::PARKING_QUEUE) {
    //         return 0;
    //     } else {
    //         return MIN2(s->getLength() / STEPS2TIME(myEventTime - myLastEntryTime),
    //                     getEdge()->getLanes()[myQueIndex]->getVehicleMaxSpeed(this));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_average_speed(&self) -> f64 {
        // cache for thread safety
        let s = self.mySegment;
        if s.is_null() || self.myQueIndex == PARKING_QUEUE {
            0.
        } else {
            min2(s.get_length() / steps2time(self.myEventTime - self.myLastEntryTime),
                 self.get_edge().get_lanes()[self.myQueIndex as usize].get_vehicle_max_speed(Ptr::new(self).cast::<SUMOTrafficObject>()))
        }
    }

    /// C++: `double MEVehicle::estimateLeaveSpeed(const MSLink *) const` [const]
    /// Returns the vehicle's estimated speed after driving across the link
    // double
    // MEVehicle::estimateLeaveSpeed(const MSLink* link) const {
    //     /// @see MSVehicle.cpp::estimateLeaveSpeed
    //     const double v = getSpeed();
    //     return MIN2(link->getViaLaneOrLane()->getVehicleMaxSpeed(this),
    //                 (double)sqrt(2 * link->getLength() * getVehicleType().getCarFollowModel().getMaxAccel() + v * v));
    // }
    #[allow(unused_variables)]
    pub fn estimate_leave_speed(&self, link: Ptr<MSLink>) -> f64 {
        /// @see MSVehicle.cpp::estimateLeaveSpeed
        let v = self.get_speed();
        min2(link.get_via_lane_or_lane().get_vehicle_max_speed(Ptr::new(self).cast::<SUMOTrafficObject>()),
             (2. * link.get_length() * self.get_vehicle_type().get_car_follow_model().get_max_accel() + v * v).sqrt())
    }

    /// C++: `double MEVehicle::getConservativeSpeed(SUMOTime &) const` [const]
    /// Returns the vehicle's estimated speed taking into account delays
    // double
    // MEVehicle::getConservativeSpeed(SUMOTime& earliestArrival) const {
    //     earliestArrival = MAX2(myEventTime, earliestArrival - DELTA_T); // event times have subsecond resolution
    //     return mySegment->getLength() / STEPS2TIME(earliestArrival - myLastEntryTime);
    // }
    #[allow(unused_variables)]
    pub fn get_conservative_speed(&self, earliestArrival: &mut SUMOTime) -> f64 {
        *earliestArrival = std::cmp::max(self.myEventTime, *earliestArrival - delta_t()); // event times have subsecond resolution
        self.mySegment.get_length() / steps2time(*earliestArrival - self.myLastEntryTime)
    }

    /// C++: `void MEVehicle::onRemovalFromNet(const MSMoveReminder::Notification)` [virtual]
    /// Called when the vehicle is removed from the network.
    // void
    // MEVehicle::onRemovalFromNet(const MSMoveReminder::Notification reason) {
    //     MSGlobals::gMesoNet->removeLeaderCar(this);
    //     MSGlobals::gMesoNet->changeSegment(this, MSNet::getInstance()->getCurrentTimeStep(), nullptr, reason);
    // }
    #[allow(unused_variables)]
    pub fn on_removal_from_net(&mut self, reason: MSMoveReminder_Notification) {
        meso_net().m().remove_leader_car(Ptr::from_mut(self));
        let now = MSNet::get_instance().get_current_time_step();
        meso_net().m().change_segment(Ptr::from_mut(self), now, Ptr::null(), reason, false);
    }

    /// C++: `bool MEVehicle::moveRoutePointer()`
    /// Update when the vehicle enters a new edge in the move step.
    // bool
    // MEVehicle::moveRoutePointer() {
    //     // vehicle has just entered a new edge. Position is 0
    //     if (myCurrEdge == myRoute->end() - 1 || (myParameter->arrivalEdge >= 0 && getRoutePosition() >= myParameter->arrivalEdge)) { // may happen during teleport
    //         return true;
    //     }
    //     ++myCurrEdge;
    //     if ((*myCurrEdge)->isVaporizing()) {
    //         return true;
    //     }
    //     // update via
    //     if (myParameter->via.size() > 0 && (*myCurrEdge)->getID() == myParameter->via.front()) {
    //         myParameter->via.erase(myParameter->via.begin());
    //     }
    //     return hasArrived();
    // }
    #[allow(unused_variables)]
    pub fn move_route_pointer(&mut self) -> bool {
        // vehicle has just entered a new edge. Position is 0
        if self.myCurrEdge == (self.myRoute.size() - 1) as usize || (self.myParameter.arrivalEdge >= 0 && self.get_route_position() >= self.myParameter.arrivalEdge) {
            // may happen during teleport
            return true;
        }
        self.myCurrEdge += 1;
        if self.myRoute.index_op(self.myCurrEdge as i32).is_vaporizing() {
            return true;
        }
        // update via
        if self.myParameter.via.len() > 0 && self.myRoute.index_op(self.myCurrEdge as i32).get_id() == self.myParameter.via[0] {
            self.myParameter.m().via.remove(0);
        }
        self.has_arrived()
    }

    /// C++: `bool MEVehicle::hasArrived() const` [const, virtual]
    /// Returns whether this vehicle has already arrived (reached the arrivalPosition on its final edge)
    // bool
    // MEVehicle::hasArrived() const {
    //     // mySegment may be 0 due to teleporting or arrival
    //     return (myCurrEdge == myRoute->end() - 1 || (myParameter->arrivalEdge >= 0 && getRoutePosition() >= myParameter->arrivalEdge)) && (
    //                (mySegment == nullptr)
    //                || myEventTime == SUMOTime_MIN
    //                || getPositionOnLane() > myArrivalPos - POSITION_EPS);
    // }
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        // mySegment may be 0 due to teleporting or arrival
        (self.myCurrEdge == (self.myRoute.size() - 1) as usize || (self.myParameter.arrivalEdge >= 0 && self.get_route_position() >= self.myParameter.arrivalEdge)) && (
                   (self.mySegment.is_null())
                   || self.myEventTime == SUMOTIME_MIN
                   || self.get_position_on_lane() > self.myArrivalPos - utils::geom::POSITION_EPS)
    }

    /// C++: `bool MEVehicle::isOnRoad() const` [const, virtual]
    /// Returns the information whether the vehicle is on a road (is simulated)
    // bool
    // MEVehicle::isOnRoad() const {
    //     return getSegment() != nullptr;
    // }
    #[allow(unused_variables)]
    pub fn is_on_road(&self) -> bool {
        !self.mySegment.is_null()
    }

    /// C++: `bool MEVehicle::isIdling() const` [const, virtual]
    /// Returns whether the vehicle is trying to re-enter the net
    // bool
    // MEVehicle::isIdling() const {
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn is_idling(&self) -> bool {
        false
    }

    /// C++: `void MEVehicle::setApproaching(MSLink *)`
    /// registers vehicle with the given link
    // void
    // MEVehicle::setApproaching(MSLink* link) {
    //     if (link != nullptr) {
    //         const double speed = getSpeed();
    //         link->setApproaching(this, getEventTime() + (link->getState() == LINKSTATE_ALLWAY_STOP ?
    //                              (SUMOTime)RandHelper::rand((int)2) : 0), // tie braker
    //                              speed, link->getViaLaneOrLane()->getVehicleMaxSpeed(this), true,
    //                              speed, getWaitingTime(),
    //                              // @note: dist is not used by meso (getZipperSpeed is never called)
    //                              getSegment()->getLength(), 0);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_approaching(&mut self, link: Ptr<MSLink>) {
        if !link.is_null() {
            let me = Ptr::from_mut(self);
            let speed = self.get_speed();
            // tie breaker
            let tie: SUMOTime = if link.get_state() == LINKSTATE_ALLWAY_STOP {
                utils::common::rand_helper::with_default(|r| r.rand_int(2)) as SUMOTime
            } else {
                0
            };
            let arrival = self.get_event_time() + tie;
            let max_speed = link.get_via_lane_or_lane().get_vehicle_max_speed(me.cast::<SUMOTrafficObject>());
            // @note: dist is not used by meso (getZipperSpeed is never called)
            let seg_len = self.get_segment().get_length();
            let waiting = self.get_waiting_time(false);
            link.m().set_approaching(me.cast::<SUMOVehicle>(), arrival, speed, max_speed, true, speed, waiting, seg_len, 0.);
        }
    }

    /// C++: `SUMOTime MEVehicle::remainingStopDuration() const` [const, virtual]
    /// Returns the remaining stop duration for a stopped vehicle or 0
    // SUMOTime remainingStopDuration() const {
    //         return 0;
    //     }
    #[allow(unused_variables)]
    pub fn remaining_stop_duration(&self) -> SUMOTime {
        0
    }

    /// C++: `bool MEVehicle::instantStopping() const` [const, virtual]
    // bool instantStopping() const {
    //         return true;
    //     }
    #[allow(unused_variables)]
    pub fn instant_stopping(&self) -> bool {
        true
    }

    /// C++: `bool MEVehicle::endTriggeredStop()`
    /// process triggered stops and return true if all stops on the current segment are completed
    // bool
    // MEVehicle::endTriggeredStop() {
    //     MSNet* const net = MSNet::getInstance();
    //     SUMOTime dummy = -1; // boarding- and loading-time are not considered
    //     for (MSStop& stop : myStops) {
    //         if (!stop.reached) {
    //             break;
    //         }
    //         if (net->getCurrentTimeStep() > stop.endBoarding) {
    //             if (stop.triggered || stop.containerTriggered) {
    //                 MSDevice_Taxi* taxiDevice = static_cast<MSDevice_Taxi*>(getDevice(typeid(MSDevice_Taxi)));
    //                 if (taxiDevice != nullptr) {
    //                     taxiDevice->cancelCurrentCustomers();
    //                 }
    //                 stop.triggered = false;
    //                 stop.containerTriggered = false;
    //             }
    //             if (myAmRegisteredAsWaiting) {
    //                 net->getVehicleControl().unregisterOneWaiting();
    //                 myAmRegisteredAsWaiting = false;
    //             }
    //         }
    //         if (stop.triggered) {
    //             if (getVehicleType().getPersonCapacity() == getPersonNumber()) {
    //                 // we could not check this on entering the segment because there may be persons who still want to leave
    //                 WRITE_WARNINGF(TL("Vehicle '%' ignores triggered stop on lane '%' due to capacity constraints."), getID(), stop.lane->getID());
    //                 stop.triggered = false;
    //                 if (myAmRegisteredAsWaiting) {
    //                     net->getVehicleControl().unregisterOneWaiting();
    //                     myAmRegisteredAsWaiting = false;
    //                 }
    //             } else if (!net->hasPersons() || !net->getPersonControl().loadAnyWaiting(&mySegment->getEdge(), this, dummy, dummy)) {
    //                 if (!myAmRegisteredAsWaiting) {
    //                     MSNet::getInstance()->getVehicleControl().registerOneWaiting();
    //                     myAmRegisteredAsWaiting = true;
    //                 }
    //                 return false;
    //             }
    //         }
    //         if (stop.containerTriggered) {
    //             if (getVehicleType().getContainerCapacity() == getContainerNumber()) {
    //                 // we could not check this on entering the segment because there may be containers who still want to leave
    //                 WRITE_WARNINGF(TL("Vehicle '%' ignores container triggered stop on lane '%' due to capacity constraints."), getID(), stop.lane->getID());
    //                 stop.containerTriggered = false;
    //                 if (myAmRegisteredAsWaiting) {
    //                     net->getVehicleControl().unregisterOneWaiting();
    //                     myAmRegisteredAsWaiting = false;
    //                 }
    //             } else if (!net->hasContainers() || !net->getContainerControl().loadAnyWaiting(&mySegment->getEdge(), this, dummy, dummy)) {
    //                 if (!myAmRegisteredAsWaiting) {
    //                     MSNet::getInstance()->getVehicleControl().registerOneWaiting();
    //                     myAmRegisteredAsWaiting = true;
    //                 }
    //                 return false;
    //             }
    //         }
    //         if (stop.joinTriggered) {
    //             // TODO do something useful here
    //             return false;
    //         }
    //     }
    //     return true;
    // }
    #[allow(unused_variables)]
    pub fn end_triggered_stop(&mut self) -> bool {
        let net = MSNet::get_instance();
        let me = Ptr::from_mut(self);
        for i in 0..self.myStops.len() {
            if !self.myStops[i].reached {
                break;
            }
            if net.get_current_time_step() > self.myStops[i].endBoarding {
                if self.myStops[i].triggered || self.myStops[i].containerTriggered {
                    // shortcut: getDevice(typeid(MSDevice_Taxi)) is null in MSBaseVehicle, so cancelCurrentCustomers() is skipped; upgrade when device lookup by type is ported
                    self.myStops[i].triggered = false;
                    self.myStops[i].containerTriggered = false;
                }
                if self.myAmRegisteredAsWaiting {
                    net.m().get_vehicle_control().unregister_one_waiting();
                    self.myAmRegisteredAsWaiting = false;
                }
            }
            if self.myStops[i].triggered {
                if self.get_vehicle_type().get_person_capacity() == self.get_person_number() {
                    // we could not check this on entering the segment because there may be persons who still want to leave
                    utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ignores triggered stop on lane '%' due to capacity constraints.", self.get_id(), self.myStops[i].lane.get_id()));
                    self.myStops[i].triggered = false;
                    if self.myAmRegisteredAsWaiting {
                        net.m().get_vehicle_control().unregister_one_waiting();
                        self.myAmRegisteredAsWaiting = false;
                    }
                } else {
                    let mut dummy_load: SUMOTime = -1;
                    let mut dummy_stop: SUMOTime = -1;
                    if !net.has_persons() || !net.m().get_person_control().load_any_waiting(Ptr::new(self.mySegment.get_edge()), me.cast::<SUMOVehicle>(), &mut dummy_load, &mut dummy_stop, Ptr::null()) {
                        if !self.myAmRegisteredAsWaiting {
                            MSNet::get_instance().m().get_vehicle_control().register_one_waiting();
                            self.myAmRegisteredAsWaiting = true;
                        }
                        return false;
                    }
                }
            }
            if self.myStops[i].containerTriggered {
                if self.get_vehicle_type().get_container_capacity() == self.get_container_number() {
                    // we could not check this on entering the segment because there may be containers who still want to leave
                    utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' ignores container triggered stop on lane '%' due to capacity constraints.", self.get_id(), self.myStops[i].lane.get_id()));
                    self.myStops[i].containerTriggered = false;
                    if self.myAmRegisteredAsWaiting {
                        net.m().get_vehicle_control().unregister_one_waiting();
                        self.myAmRegisteredAsWaiting = false;
                    }
                } else {
                    let mut dummy_load: SUMOTime = -1;
                    let mut dummy_stop: SUMOTime = -1;
                    if !net.has_containers() || !net.m().get_container_control().load_any_waiting(Ptr::new(self.mySegment.get_edge()), me.cast::<SUMOVehicle>(), &mut dummy_load, &mut dummy_stop, Ptr::null()) {
                        if !self.myAmRegisteredAsWaiting {
                            MSNet::get_instance().m().get_vehicle_control().register_one_waiting();
                            self.myAmRegisteredAsWaiting = true;
                        }
                        return false;
                    }
                }
            }
            if self.myStops[i].joinTriggered {
                // TODO do something useful here
                return false;
            }
        }
        true
    }

    /// C++: `void MEVehicle::processStop()`
    /// ends the current stop and performs loading/unloading
    // void
    // MEVehicle::processStop() {
    //     assert(isStopped());
    //     double lastPos = -1;
    //     bool hadStop = false;
    //     while (!myStops.empty()) {
    //         MSStop& stop = myStops.front();
    //         if (stop.edge != myCurrEdge || stop.segment != mySegment || stop.pars.endPos <= lastPos) {
    //             break;
    //         }
    //         lastPos = stop.pars.endPos;
    //         MSNet* const net = MSNet::getInstance();
    //         SUMOTime dummy = -1; // boarding- and loading-time are not considered
    //         if (hadStop && MSStopOut::active()) {
    //             stop.reached = true;
    //             MSStopOut::getInstance()->stopStarted(this, getPersonNumber(), getContainerNumber(), myLastEntryTime);
    //         }
    //         if (net->hasPersons()) {
    //             net->getPersonControl().loadAnyWaiting(&mySegment->getEdge(), this, dummy, dummy);
    //         }
    //         if (net->hasContainers()) {
    //             net->getContainerControl().loadAnyWaiting(&mySegment->getEdge(), this, dummy, dummy);
    //         }
    //         resumeFromStopping();
    //         hadStop = true;
    //     }
    //     if (getWaitingTime() > 0) {
    //         // entry back onto the road was blocked for some time
    //         MSDevice_Tripinfo* tripinfoDevice = static_cast<MSDevice_Tripinfo*>(getDevice(typeid(MSDevice_Tripinfo)));
    //         if (tripinfoDevice != nullptr) {
    //             tripinfoDevice->recordMesoParkingTimeLoss(getWaitingTime());
    //         }
    //     }
    //     mySegment->getEdge().removeWaiting(this);
    // }
    #[allow(unused_variables)]
    pub fn process_stop(&mut self) {
        assert!(self.is_stopped());
        let mut lastPos: f64 = -1.;
        let mut hadStop = false;
        let me = Ptr::from_mut(self);
        while !self.myStops.is_empty() {
            // shortcut: MSStop.edge is an Unmapped route iterator; compare the stop's edge with the route edge at myCurrEdge
            let curEdge = self.myRoute.index_op(self.myCurrEdge as i32);
            let endPos = self.myStops[0].pars.endPos;
            if self.myStops[0].get_edge() != curEdge || self.myStops[0].segment != self.mySegment || endPos <= lastPos {
                break;
            }
            lastPos = endPos;
            let net = MSNet::get_instance();
            let mut dummy_load: SUMOTime = -1; // boarding- and loading-time are not considered
            let mut dummy_stop: SUMOTime = -1;
            if hadStop && MSStopOut::active() {
                self.myStops[0].reached = true;
                MSStopOut::get_instance().m().stop_started(me.cast::<SUMOVehicle>(), self.get_person_number(), self.get_container_number(), self.myLastEntryTime);
            }
            if net.has_persons() {
                net.m().get_person_control().load_any_waiting(Ptr::new(self.mySegment.get_edge()), me.cast::<SUMOVehicle>(), &mut dummy_load, &mut dummy_stop, Ptr::null());
            }
            if net.has_containers() {
                net.m().get_container_control().load_any_waiting(Ptr::new(self.mySegment.get_edge()), me.cast::<SUMOVehicle>(), &mut dummy_load, &mut dummy_stop, Ptr::null());
            }
            self.resume_from_stopping();
            hadStop = true;
        }
        if self.get_waiting_time(false) > 0 {
            // entry back onto the road was blocked for some time
            // shortcut: getDevice(typeid(MSDevice_Tripinfo)) is null in MSBaseVehicle, so recordMesoParkingTimeLoss() is skipped; upgrade when device lookup by type is ported
        }
        self.mySegment.get_edge().remove_waiting(me.cast::<SUMOVehicle>());
    }

    /// C++: `SUMOTime MEVehicle::checkStop(SUMOTime)`
    /// Returns until when to stop at the current segment and sets the information that the stop has been reached
    // SUMOTime
    // MEVehicle::checkStop(SUMOTime time) {
    //     const SUMOTime initialTime = time;
    //     bool hadStop = false;
    //     bool reachedStop = false;
    //     for (MSStop& stop : myStops) {
    //         if (stop.joinTriggered) {
    //             WRITE_WARNINGF(TL("Join stops are not available in meso yet (vehicle '%', segment '%')."),
    //                            getID(), mySegment->getID());
    //             continue;
    //         }
    //         if (stop.edge != myCurrEdge || stop.segment != mySegment) {
    //             break;
    //         }
    //         const SUMOTime cur = time;
    //         if (stop.duration > 0) { // it might be a triggered stop with duration -1
    //             if (stop.duration >= SUMOTime_MAX / 2) {
    //                 // avoid overflow!
    //                 time = SUMOTime_MAX / 2;
    //             } else {
    //                 time += stop.duration;
    //             }
    //         }
    //         if (stop.pars.until > time) {
    //             // @note: this assumes the stop is reached at time. With the way this is called in MESegment (time == entryTime),
    //             // travel time is overestimated of the stop is not at the start of the segment
    //             time = stop.pars.until;
    //         }
    //         if (MSGlobals::gUseStopEnded && stop.pars.ended >= 0) {
    //             time = MAX2(cur, stop.pars.ended);
    //         }
    //         if (!stop.reached) {
    //             reachedStop = true;
    //             stop.reached = true;
    //             stop.pars.started = myLastEntryTime;
    //             stop.endBoarding = stop.pars.extension >= 0 ? time + stop.pars.extension : SUMOTime_MAX;
    //             if (MSStopOut::active()) {
    //                 if (!hadStop) {
    //                     MSStopOut::getInstance()->stopStarted(this, getPersonNumber(), getContainerNumber(), myLastEntryTime);
    //                 } else {
    //                     WRITE_WARNINGF(TL("Vehicle '%' has multiple stops on segment '%', time=% (stop-output will be merged)."),
    //                                    getID(), mySegment->getID(), time2string(time));
    //                 }
    //             }
    //         }
    //         if (stop.triggered || stop.containerTriggered || stop.joinTriggered) {
    //             time = MAX2(time, cur + DELTA_T);
    //         }
    //         hadStop = true;
    //     }
    //     if (reachedStop) {
    //         MSDevice_Taxi* taxi = static_cast<MSDevice_Taxi*>(getDevice(typeid(MSDevice_Taxi)));
    //         if (taxi != nullptr) {
    //             taxi->notifyMove(*this, 0, 0, 0);
    //         }
    //     }
    //     MSDevice_Tripinfo* tripinfo = static_cast<MSDevice_Tripinfo*>(getDevice(typeid(MSDevice_Tripinfo)));
    //     if (tripinfo != nullptr) {
    //         tripinfo->updateStopTime(time - initialTime);
    //     }
    //     return time;
    // }
    #[allow(unused_variables)]
    pub fn check_stop(&mut self, time: SUMOTime) -> SUMOTime {
        let initialTime = time;
        let mut time = time;
        let mut hadStop = false;
        let mut reachedStop = false;
        let me = Ptr::from_mut(self);
        for i in 0..self.myStops.len() {
            if self.myStops[i].joinTriggered {
                utils::common::msg_handler::write_warning(&utils::tlf!("Join stops are not available in meso yet (vehicle '%', segment '%').", self.get_id(), self.mySegment.get_id()));
                continue;
            }
            // shortcut: MSStop.edge is an Unmapped route iterator; compare the stop's edge with the route edge at myCurrEdge
            if self.myStops[i].get_edge() != self.myRoute.index_op(self.myCurrEdge as i32) || self.myStops[i].segment != self.mySegment {
                break;
            }
            let cur = time;
            if self.myStops[i].duration > 0 { // it might be a triggered stop with duration -1
                if self.myStops[i].duration >= SUMOTIME_MAX / 2 {
                    // avoid overflow!
                    time = SUMOTIME_MAX / 2;
                } else {
                    time += self.myStops[i].duration;
                }
            }
            if self.myStops[i].pars.until > time {
                // @note: this assumes the stop is reached at time. With the way this is called in MESegment (time == entryTime),
                // travel time is overestimated of the stop is not at the start of the segment
                time = self.myStops[i].pars.until;
            }
            if MSGlobals::use_stop_ended() && self.myStops[i].pars.ended >= 0 {
                time = std::cmp::max(cur, self.myStops[i].pars.ended);
            }
            if !self.myStops[i].reached {
                reachedStop = true;
                self.myStops[i].reached = true;
                self.myStops[i].pars.started = self.myLastEntryTime;
                self.myStops[i].endBoarding = if self.myStops[i].pars.extension >= 0 { time + self.myStops[i].pars.extension } else { SUMOTIME_MAX };
                if MSStopOut::active() {
                    if !hadStop {
                        MSStopOut::get_instance().m().stop_started(me.cast::<SUMOVehicle>(), self.get_person_number(), self.get_container_number(), self.myLastEntryTime);
                    } else {
                        utils::common::msg_handler::write_warning(&utils::tlf!("Vehicle '%' has multiple stops on segment '%', time=% (stop-output will be merged).", self.get_id(), self.mySegment.get_id(), utils::common::time::time2string(time)));
                    }
                }
            }
            if self.myStops[i].triggered || self.myStops[i].containerTriggered || self.myStops[i].joinTriggered {
                time = std::cmp::max(time, cur + delta_t());
            }
            hadStop = true;
        }
        if reachedStop {
            // shortcut: getDevice(typeid(MSDevice_Taxi)) is null in MSBaseVehicle, so taxi->notifyMove(*this, 0, 0, 0) is skipped
        }
        // shortcut: getDevice(typeid(MSDevice_Tripinfo)) is null in MSBaseVehicle, so tripinfo->updateStopTime(time - initialTime) is skipped
        let _ = initialTime;
        time
    }

    /// C++: `bool MEVehicle::resumeFromStopping()` [virtual]
    /// resumes a vehicle from stopping
    // bool
    // MEVehicle::resumeFromStopping() {
    //     if (isStopped()) {
    //         const SUMOTime now = SIMSTEP;
    //         MSStop& stop = myStops.front();
    //         stop.pars.ended = now;
    //         for (const auto& rem : myMoveReminders) {
    //             rem.first->notifyStopEnded();
    //         }
    //         if (MSStopOut::active()) {
    //             MSStopOut::getInstance()->stopEnded(this, stop);
    //         }
    //         myPastStops.push_back(stop.pars);
    //         myPastStops.back().routeIndex = (int)(stop.edge - myRoute->begin());
    //         if (myAmRegisteredAsWaiting && (stop.triggered || stop.containerTriggered || stop.joinTriggered)) {
    //             MSNet::getInstance()->getVehicleControl().unregisterOneWaiting();
    //             myAmRegisteredAsWaiting = false;
    //         }
    //         myStops.pop_front();
    //         if (myEventTime > now) {
    //             // if this is an aborted stop we need to change the event time of the vehicle
    //             const bool isLeader = mySegment->getQueue(myQueIndex).back() == this;
    //             if (isLeader) {
    //                 MSGlobals::gMesoNet->removeLeaderCar(this);
    //                 myEventTime = now + 1;
    //                 MSGlobals::gMesoNet->addLeaderCar(this, nullptr);
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn resume_from_stopping(&mut self) -> bool {
        if self.is_stopped() {
            let now = MSNet::get_instance().get_current_time_step();
            let me = Ptr::from_mut(self);
            self.myStops.front_mut().unwrap().pars.ended = now;
            for rem in self.myMoveReminders.iter() {
                rem.0.m().notify_stop_ended();
            }
            if MSStopOut::active() {
                MSStopOut::get_instance().m().stop_ended(me.cast::<SUMOVehicle>(), self.myStops.front().unwrap(), false);
            }
            let pars = self.myStops.front().unwrap().pars.clone();
            self.myPastStops.push(pars);
            // shortcut: MSStop.edge is an Unmapped route iterator; routeIndex uses the current route position, which equals stop.edge for stops handled in meso
            self.myPastStops.last_mut().unwrap().routeIndex = self.myCurrEdge as i32;
            let (triggered, containerTriggered, joinTriggered) = {
                let s = self.myStops.front().unwrap();
                (s.triggered, s.containerTriggered, s.joinTriggered)
            };
            if self.myAmRegisteredAsWaiting && (triggered || containerTriggered || joinTriggered) {
                MSNet::get_instance().m().get_vehicle_control().unregister_one_waiting();
                self.myAmRegisteredAsWaiting = false;
            }
            self.myStops.pop_front();
            if self.myEventTime > now {
                // if this is an aborted stop we need to change the event time of the vehicle
                let isLeader = self.mySegment.get_queue(self.myQueIndex).last().copied() == Some(me);
                if isLeader {
                    let meso = meso_net();
                    meso.m().remove_leader_car(me);
                    self.myEventTime = now + 1;
                    meso.m().add_leader_car(me, Ptr::null(), false);
                }
            }
            return true;
        }
        false
    }

    /// C++: `double MEVehicle::getBrakeGap(bool) const` [const, virtual]
    /// get distance for coming to a stop (used for rerouting checks)
    // double getBrakeGap(bool delayed = false) const {
    //         UNUSED_PARAMETER(delayed);
    //         return mySegment == nullptr || myQueIndex == MESegment::PARKING_QUEUE ? 0 : mySegment->getLength();
    //     }
    #[allow(unused_variables)]
    pub fn get_brake_gap(&self, delayed: bool) -> f64 {
        // UNUSED_PARAMETER(delayed);
        if self.mySegment.is_null() || self.myQueIndex == PARKING_QUEUE { 0. } else { self.mySegment.get_length() }
    }

    /// C++: `void MEVehicle::setEventTime(SUMOTime, bool)`
    /// Sets the (planned) time at which the vehicle leaves its current segment
    // inline void setEventTime(SUMOTime t, bool hasDelay = true) {
    //         assert(t > myLastEntryTime);
    //         if (hasDelay && mySegment != nullptr) {
    //             mySegment->getEdge().markDelayed();
    //         }
    //         myEventTime = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_event_time(&mut self, t: SUMOTime, hasDelay: bool) {
        assert!(t > self.myLastEntryTime);
        if hasDelay && !self.mySegment.is_null() {
            self.mySegment.get_edge().mark_delayed();
        }
        self.myEventTime = t;
    }

    /// C++: `SUMOTime MEVehicle::getEventTime() const` [const]
    /// Returns the (planned) time at which the vehicle leaves its current segment
    // inline SUMOTime getEventTime() const {
    //         return myEventTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_event_time(&self) -> SUMOTime {
        self.myEventTime
    }

    /// C++: `void MEVehicle::setEventIndex(long long)`
    /// Sets the index of the vehicle's event (orders events with the same time)
    // inline void setEventIndex(long long int index) {
    //         myEventIndex = index;
    //     }
    #[allow(unused_variables)]
    pub fn set_event_index(&mut self, index: i64) {
        self.myEventIndex = index;
    }

    /// C++: `long long MEVehicle::getEventIndex() const` [const]
    /// Returns the index of the vehicle's event
    // inline long long int getEventIndex() const {
    //         return myEventIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_event_index(&self) -> i64 {
        self.myEventIndex
    }

    /// C++: `void MEVehicle::setUnqueuedEventTime(SUMOTime)`
    /// Sets the minimum time at which the vehicle leaves its current segment (this includes planned stops)
    // inline void setUnqueuedEventTime(SUMOTime t) {
    //         myUnqueuedEventTime = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_unqueued_event_time(&mut self, t: SUMOTime) {
        self.myUnqueuedEventTime = t;
    }

    /// C++: `SUMOTime MEVehicle::getUnqueuedEventTime() const` [const]
    /// Returns the minimum time at which the vehicle leaves its current segment
    // inline SUMOTime getUnqueuedEventTime() const {
    //         return myUnqueuedEventTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_unqueued_event_time(&self) -> SUMOTime {
        self.myUnqueuedEventTime
    }

    /// C++: `void MEVehicle::setSegment(MESegment *, int)` [virtual]
    /// Sets the current segment the vehicle is at together with its que
    // inline virtual void setSegment(MESegment* s, int idx = 0) {
    //         mySegment = s;
    //         myQueIndex = idx;
    //     }
    #[allow(unused_variables)]
    pub fn set_segment(&mut self, s: Ptr<MESegment>, idx: i32) {
        self.mySegment = s;
        self.myQueIndex = idx;
    }

    /// C++: `MESegment * MEVehicle::getSegment() const` [const]
    /// Returns the current segment the vehicle is on
    // inline MESegment* getSegment() const {
    //         return mySegment;
    //     }
    #[allow(unused_variables)]
    pub fn get_segment(&self) -> Ptr<MESegment> {
        self.mySegment
    }

    /// C++: `int MEVehicle::getSegmentIndex() const` [const]
    // int
    // MEVehicle::getSegmentIndex() const {
    //     return getSegment() != nullptr ? getSegment()->getIndex() : -1;
    // }
    #[allow(unused_variables)]
    pub fn get_segment_index(&self) -> i32 {
        if !self.mySegment.is_null() { self.mySegment.get_index() } else { -1 }
    }

    /// C++: `int MEVehicle::getQueIndex() const` [const]
    /// Returns the index of the que the vehicle is in
    // inline int getQueIndex() const {
    //         return myQueIndex;
    //     }
    #[allow(unused_variables)]
    pub fn get_que_index(&self) -> i32 {
        self.myQueIndex
    }

    /// C++: `double MEVehicle::getRightSideOnEdge(const MSLane *) const` [const, virtual]
    /// Get the vehicle's lateral position on the edge of the given lane (or its current edge if lane == 0)
    // double
    // MEVehicle::getRightSideOnEdge(const MSLane* /*lane*/) const {
    //     if (mySegment == nullptr || mySegment->getIndex() >= getEdge()->getNumLanes()) {
    //         return 0;
    //     }
    //     const MSLane* lane = getEdge()->getLanes()[mySegment->getIndex()];
    //     return lane->getRightSideOnEdge() + lane->getWidth() * 0.5 - 0.5 * getVehicleType().getWidth();
    //
    // }
    #[allow(unused_variables)]
    pub fn get_right_side_on_edge(&self, arg0: Ptr<MSLane>) -> f64 {
        if self.mySegment.is_null() || self.mySegment.get_index() >= self.get_edge().get_num_lanes() {
            return 0.;
        }
        let lane = self.get_edge().get_lanes()[self.mySegment.get_index() as usize];
        lane.get_right_side_on_edge() + lane.get_width() * 0.5 - 0.5 * self.get_vehicle_type().get_width()
    }

    /// C++: `void MEVehicle::setLastEntryTime(SUMOTime)`
    /// Sets the entry time for the current segment
    // inline void setLastEntryTime(SUMOTime t) {
    //         myLastEntryTime = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_last_entry_time(&mut self, t: SUMOTime) {
        self.myLastEntryTime = t;
    }

    /// C++: `SUMOTime MEVehicle::getLastEntryTime() const` [const]
    /// Returns the time the vehicle entered the current segment
    // SUMOTime getLastEntryTime() const {
    //         return myLastEntryTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_entry_time(&self) -> SUMOTime {
        self.myLastEntryTime
    }

    /// C++: `SUMOTime MEVehicle::getLastActionTime() const` [const, virtual]
    /// Returns the time of the vehicle's last action point.
    // SUMOTime getLastActionTime() const {
    //         return SIMSTEP;
    //     }
    #[allow(unused_variables)]
    pub fn get_last_action_time(&self) -> SUMOTime {
        MSNet::get_instance().get_current_time_step()
    }

    /// C++: `void MEVehicle::setBlockTime(const SUMOTime)`
    /// Sets the time at which the vehicle was blocked
    // inline void setBlockTime(const SUMOTime t) {
    //         assert(t > myLastEntryTime);
    //         myBlockTime = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_block_time(&mut self, t: SUMOTime) {
        assert!(t > self.myLastEntryTime);
        self.myBlockTime = t;
    }

    /// C++: `SUMOTime MEVehicle::getBlockTime() const` [const]
    /// Returns the time at which the vehicle was blocked
    // inline SUMOTime getBlockTime() const {
    //         return myBlockTime;
    //     }
    #[allow(unused_variables)]
    pub fn get_block_time(&self) -> SUMOTime {
        self.myBlockTime
    }

    /// C++: `SUMOTime MEVehicle::getWaitingTime(const bool) const` [const, virtual]
    /// Returns the duration for which the vehicle was blocked
    // inline SUMOTime getWaitingTime(const bool accumulated = false) const {
    //         UNUSED_PARAMETER(accumulated);
    //         return MAX2(SUMOTime(0), myEventTime - myBlockTime);
    //     }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, accumulated: bool) -> SUMOTime {
        // UNUSED_PARAMETER(accumulated);
        std::cmp::max(0, self.myEventTime - self.myBlockTime)
    }

    /// C++: `SUMOTime MEVehicle::getQueuingTimeLoss() const` [const]
    /// Returns the time lost compare to free flow
    // inline SUMOTime getQueuingTimeLoss() const {
    //         return MAX2(SUMOTime(0), myEventTime - myUnqueuedEventTime);
    //     }
    #[allow(unused_variables)]
    pub fn get_queuing_time_loss(&self) -> SUMOTime {
        std::cmp::max(0, self.myEventTime - self.myUnqueuedEventTime)
    }

    /// C++: `SUMOTime MEVehicle::getTimeLoss() const` [const, virtual]
    // inline SUMOTime getTimeLoss() const {
    //         // slow-downs while driving are not modelled
    //         return getWaitingTime();
    //     }
    #[allow(unused_variables)]
    pub fn get_time_loss(&self) -> SUMOTime {
        // slow-downs while driving are not modelled
        self.get_waiting_time(false)
    }

    /// C++: `double MEVehicle::getEventTimeSeconds() const` [const]
    /// Returns the earliest leave time for the current segment
    // double getEventTimeSeconds() const {
    //         return STEPS2TIME(getEventTime());
    //     }
    #[allow(unused_variables)]
    pub fn get_event_time_seconds(&self) -> f64 {
        steps2time(self.get_event_time())
    }

    /// C++: `double MEVehicle::getLastEntryTimeSeconds() const` [const]
    /// Returns the entry time for the current segment
    // double getLastEntryTimeSeconds() const {
    //         return STEPS2TIME(getLastEntryTime());
    //     }
    #[allow(unused_variables)]
    pub fn get_last_entry_time_seconds(&self) -> f64 {
        steps2time(self.get_last_entry_time())
    }

    /// C++: `double MEVehicle::getBlockTimeSeconds() const` [const]
    /// Returns the time at which the vehicle was blocked on the current segment
    // double getBlockTimeSeconds() const {
    //         return STEPS2TIME(getBlockTime());
    //     }
    #[allow(unused_variables)]
    pub fn get_block_time_seconds(&self) -> f64 {
        steps2time(self.get_block_time())
    }

    /// C++: `double MEVehicle::getCurrentLinkPenaltySeconds() const` [const]
    /// Returns the delay that is accrued due to option --meso-tls-penalty or --meso-minor-penalty
    // double
    // MEVehicle::getCurrentLinkPenaltySeconds() const {
    //     if (mySegment == nullptr) {
    //         return 0;
    //     } else {
    //         return STEPS2TIME(mySegment->getLinkPenalty(this));
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_current_link_penalty_seconds(&self) -> f64 {
        if self.mySegment.is_null() {
            0.
        } else {
            steps2time(self.mySegment.get_link_penalty(Ptr::new(self)))
        }
    }

    /// C++: `double MEVehicle::getCurrentStoppingTimeSeconds() const` [const]
    /// Returns the delay that is accrued due to option --meso-tls-penalty or --meso-minor-penalty
    // double
    // MEVehicle::getCurrentStoppingTimeSeconds() const {
    //     SUMOTime time = myLastEntryTime;
    //     for (const MSStop& stop : myStops) {
    //         if (stop.reached) {
    //             time += stop.duration;
    //             if (stop.pars.until > time) {
    //                 // @note: this assumes the stop is reached at time. With the way this is called in MESegment (time == entryTime),
    //                 // travel time is overestimated of the stop is not at the start of the segment
    //                 time = stop.pars.until;
    //             }
    //         } else {
    //             break;
    //         }
    //     }
    //     return STEPS2TIME(time - myLastEntryTime);
    // }
    #[allow(unused_variables)]
    pub fn get_current_stopping_time_seconds(&self) -> f64 {
        let mut time = self.myLastEntryTime;
        for stop in self.myStops.iter() {
            if stop.reached {
                time += stop.duration;
                if stop.pars.until > time {
                    // @note: this assumes the stop is reached at time. With the way this is called in MESegment (time == entryTime),
                    // travel time is overestimated of the stop is not at the start of the segment
                    time = stop.pars.until;
                }
            } else {
                break;
            }
        }
        steps2time(time - self.myLastEntryTime)
    }

    /// C++: `bool MEVehicle::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual]
    /// Replaces the current route by the given one
    // bool
    // MEVehicle::replaceRoute(ConstMSRoutePtr newRoute, const std::string& info,  bool onInit, int offset, bool addRouteStops, bool removeStops, std::string* msgReturn) {
    //     MSLink* const oldLink = mySegment != nullptr ? mySegment->getLink(this) : nullptr;
    //     if (MSBaseVehicle::replaceRoute(newRoute, info, onInit, offset, addRouteStops, removeStops, msgReturn)) {
    //         if (mySegment != nullptr) {
    //             MSLink* const newLink = mySegment->getLink(this);
    //             // update approaching vehicle information
    //             if (oldLink != newLink) {
    //                 if (oldLink != nullptr) {
    //                     oldLink->removeApproaching(this);
    //                 }
    //                 setApproaching(newLink);
    //             }
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addRouteStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        let me = Ptr::from_mut(self);
        let seg = self.mySegment;
        let oldLink = if !seg.is_null() { seg.get_link(me, false) } else { Ptr::null() };
        if self.base.replace_route(route, info, onInit, offset, addRouteStops, removeStops, msgReturn) {
            let seg = self.mySegment;
            if !seg.is_null() {
                let newLink = seg.get_link(me, false);
                // update approaching vehicle information
                if oldLink != newLink {
                    if !oldLink.is_null() {
                        oldLink.m().remove_approaching(me.cast::<SUMOVehicle>());
                    }
                    self.set_approaching(newLink);
                }
            }
            return true;
        }
        false
    }

    /// C++: `bool MEVehicle::mayProceed()`
    /// Returns whether the vehicle is allowed to pass the next junction, checks also for triggered stops
    // bool
    // MEVehicle::mayProceed() {
    //     if (mySegment == nullptr) {
    //         return true;
    //     }
    //     if (!endTriggeredStop()) {
    //         return false;
    //     }
    //     return mySegment->isOpen(this);
    // }
    #[allow(unused_variables)]
    pub fn may_proceed(&mut self) -> bool {
        if self.mySegment.is_null() {
            return true;
        }
        if !self.end_triggered_stop() {
            return false;
        }
        let seg = self.mySegment;
        seg.is_open(Ptr::from_mut(self))
    }

    /// C++: `void MEVehicle::updateDetectorForWriting(MSMoveReminder *, SUMOTime, SUMOTime)`
    /// Updates a single vehicle detector if present
    // void
    // MEVehicle::updateDetectorForWriting(MSMoveReminder* rem, SUMOTime currentTime, SUMOTime exitTime) {
    //     for (MoveReminderCont::iterator i = myMoveReminders.begin(); i != myMoveReminders.end(); ++i) {
    //         if (i->first == rem) {
    //             rem->updateDetector(*this, mySegment->getIndex() * mySegment->getLength(),
    //                                 (mySegment->getIndex() + 1) * mySegment->getLength(),
    //                                 getLastEntryTime(), currentTime, exitTime, false);
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("notifyMove", i->first, i->second, true);
    //             }
    // #endif
    //             return;
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_detector_for_writing(&mut self, rem: Ptr<MSMoveReminder>, currentTime: SUMOTime, exitTime: SUMOTime) {
        for i in 0..self.myMoveReminders.len() {
            if self.myMoveReminders[i].0 == rem {
                let seg = self.mySegment;
                let start = (seg.get_index() as f64) * seg.get_length();
                let end = ((seg.get_index() + 1) as f64) * seg.get_length();
                let entry = self.get_last_entry_time();
                rem.m().update_detector(self, start, end, entry, currentTime, exitTime, false);
                // shortcut: traceMoveReminder() is _DEBUG-only and not ported
                return;
            }
        }
    }

    /// C++: `void MEVehicle::updateDetectors(const SUMOTime, const SUMOTime, const bool, const MSMoveReminder::Notification)`
    /// Updates all vehicle detectors
    // void
    // MEVehicle::updateDetectors(const SUMOTime currentTime, const SUMOTime exitTime, const bool isLeave, const MSMoveReminder::Notification reason) {
    //     // segments of the same edge have the same reminder so no cleaning up must take place
    //     const bool cleanUp = isLeave && (reason != MSMoveReminder::NOTIFICATION_SEGMENT);
    //     for (MoveReminderCont::iterator rem = myMoveReminders.begin(); rem != myMoveReminders.end();) {
    //         if (currentTime != getLastEntryTime() && reason < MSMoveReminder::NOTIFICATION_VAPORIZED_CALIBRATOR) {
    //             rem->first->updateDetector(*this, mySegment->getIndex() * mySegment->getLength(),
    //                                        (mySegment->getIndex() + 1) * mySegment->getLength(),
    //                                        getLastEntryTime(), currentTime, exitTime, cleanUp);
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("notifyMove", rem->first, rem->second, true);
    //             }
    // #endif
    //         }
    //         if (!isLeave || rem->first->notifyLeave(*this, mySegment == nullptr ? 0 : mySegment->getLength(), reason)) {
    // #ifdef _DEBUG
    //             if (isLeave && myTraceMoveReminders) {
    //                 traceMoveReminder("notifyLeave", rem->first, rem->second, true);
    //             }
    // #endif
    //
    //             if (isLeave) {
    //                 rem->second += getEdge()->getLength();
    // #ifdef _DEBUG
    //                 if (myTraceMoveReminders) {
    //                     traceMoveReminder("adaptedPos", rem->first, rem->second, true);
    //                 }
    // #endif
    //             }
    //             ++rem;
    //         } else {
    // #ifdef _DEBUG
    //             if (myTraceMoveReminders) {
    //                 traceMoveReminder("remove", rem->first, rem->second, false);
    //             }
    // #endif
    //             rem = myMoveReminders.erase(rem);
    //         }
    //     }
    //     if (reason == MSMoveReminder::NOTIFICATION_JUNCTION || reason == MSMoveReminder::NOTIFICATION_TELEPORT) {
    //         myOdometer += getEdge()->getLength();
    //     }
    // }
    #[allow(unused_variables)]
    pub fn update_detectors(&mut self, currentTime: SUMOTime, exitTime: SUMOTime, isLeave: bool, reason: MSMoveReminder_Notification) {
        // segments of the same edge have the same reminder so no cleaning up must take place
        let cleanUp = isLeave && (reason != MSMoveReminder_Notification::NOTIFICATION_SEGMENT);
        let mut i = 0;
        while i < self.myMoveReminders.len() {
            let rem = self.myMoveReminders[i].0;
            if currentTime != self.get_last_entry_time() && reason < MSMoveReminder_Notification::NOTIFICATION_VAPORIZED_CALIBRATOR {
                let seg = self.mySegment;
                let start = (seg.get_index() as f64) * seg.get_length();
                let end = ((seg.get_index() + 1) as f64) * seg.get_length();
                let entry = self.get_last_entry_time();
                rem.m().update_detector(self, start, end, entry, currentTime, exitTime, cleanUp);
                // shortcut: traceMoveReminder() is _DEBUG-only and not ported
            }
            let keep = if !isLeave {
                true
            } else {
                let seg = self.mySegment;
                let last_pos = if seg.is_null() { 0. } else { seg.get_length() };
                rem.m().notify_leave(self, last_pos, reason, Ptr::null())
            };
            if keep {
                if isLeave {
                    let edge_len = self.get_edge().get_length();
                    self.myMoveReminders[i].1 += edge_len;
                    // shortcut: traceMoveReminder("adaptedPos") is _DEBUG-only and not ported
                }
                i += 1;
            } else {
                // shortcut: traceMoveReminder("remove") is _DEBUG-only and not ported
                self.myMoveReminders.remove(i);
            }
        }
        if reason == MSMoveReminder_Notification::NOTIFICATION_JUNCTION || reason == MSMoveReminder_Notification::NOTIFICATION_TELEPORT {
            self.myOdometer += self.get_edge().get_length();
        }
    }

    /// C++: `BaseInfluencer & MEVehicle::getBaseInfluencer()` [virtual]
    /// Returns the velocity/lane influencer
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_base_influencer(&mut self) -> &mut MSBaseVehicle_BaseInfluencer {
        self.base.get_base_influencer()
    }

    /// C++: `const BaseInfluencer * MEVehicle::getBaseInfluencer() const` [const, virtual]
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_base_influencer_2(&self) -> Ptr<MSBaseVehicle_BaseInfluencer> {
        self.base.get_base_influencer_2()
    }

    /// C++: `bool MEVehicle::hasInfluencer() const` [const, virtual]
    // bool hasInfluencer() const {
    //         return myInfluencer != nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        !self.myInfluencer.is_null()
    }

    /// C++: `void MEVehicle::markJammed(bool)`
    // void markJammed(bool jammed) {
    //         myWasJammed = jammed;
    //     }
    #[allow(unused_variables)]
    pub fn mark_jammed(&mut self, jammed: bool) {
        self.myWasJammed = jammed;
    }

    /// C++: `bool MEVehicle::wasJammed() const` [const]
    // bool wasJammed() const {
    //         return myWasJammed;
    //     }
    #[allow(unused_variables)]
    pub fn was_jammed(&self) -> bool {
        self.myWasJammed
    }

    /// C++: `void MEVehicle::saveState(OutputDevice &)` [virtual]
    /// Saves the states of a vehicle
    // void
    // MEVehicle::saveState(OutputDevice& out) {
    //     if (mySegment != nullptr && MESegment::isInvalid(mySegment)) {
    //         // segment is vaporization target, do not write this vehicle
    //         return;
    //     }
    //     MSBaseVehicle::saveState(out);
    //     assert(mySegment == nullptr || *myCurrEdge == &mySegment->getEdge() || mySegment->getEdge().isInternal());
    //     std::vector<SUMOTime> internals;
    //     internals.push_back(myParameter->parametersSet);
    //     internals.push_back(myDeparture);
    //     internals.push_back((SUMOTime)distance(myRoute->begin(), myCurrEdge));
    //     internals.push_back((SUMOTime)myDepartPos * 1000); // store as mm
    //     internals.push_back(mySegment == nullptr ? (SUMOTime) - 1 : (SUMOTime)mySegment->getIndex());
    //     internals.push_back((SUMOTime)getQueIndex());
    //     internals.push_back(myEventTime);
    //     internals.push_back(myLastEntryTime);
    //     internals.push_back(myBlockTime);
    //     internals.push_back(isStopped());
    //     internals.push_back(myPastStops.size());
    //     internals.push_back(myWasJammed);
    //     internals.push_back(myEventIndex);
    //     internals.push_back(myUnqueuedEventTime);
    //     out.writeAttr(SUMO_ATTR_STATE, toString(internals));
    //     // save past stops
    //     for (SUMOVehicleParameter::Stop stop : myPastStops) {
    //         stop.write(out, false);
    //         // do not write started and ended twice
    //         if ((stop.parametersSet & STOP_STARTED_SET) == 0) {
    //             out.writeAttr(SUMO_ATTR_STARTED, time2string(stop.started));
    //         }
    //         if ((stop.parametersSet & STOP_ENDED_SET) == 0) {
    //             out.writeAttr(SUMO_ATTR_ENDED, time2string(stop.ended));
    //         }
    //         out.closeTag();
    //     }
    //     // save upcoming stops
    //     for (const MSStop& stop : myStops) {
    //         stop.write(out);
    //     }
    //     // save parameters
    //     myParameter->writeParams(out);
    //     for (MSDevice* dev : myDevices) {
    //         dev->saveState(out);
    //     }
    //     for (const auto& item : myMoveReminders) {
    //         item.first->saveReminderState(out, *this);
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        if !self.mySegment.is_null() && MESegment::is_invalid(self.mySegment) {
            // segment is vaporization target, do not write this vehicle
            return;
        }
        self.base.save_state(out);
        // assert(mySegment == nullptr || *myCurrEdge == &mySegment->getEdge() || mySegment->getEdge().isInternal());
        let internals: Vec<SUMOTime> = vec![
            self.myParameter.parametersSet,
            self.myDeparture,
            self.myCurrEdge as SUMOTime,
            (self.myDepartPos as SUMOTime) * 1000, // store as mm
            if self.mySegment.is_null() { -1 } else { self.mySegment.get_index() as SUMOTime },
            self.get_que_index() as SUMOTime,
            self.myEventTime,
            self.myLastEntryTime,
            self.myBlockTime,
            self.is_stopped() as SUMOTime,
            self.myPastStops.len() as SUMOTime,
            self.myWasJammed as SUMOTime,
            self.myEventIndex,
            self.myUnqueuedEventTime,
        ];
        let state = internals.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ");
        out.write_attr(SUMO_ATTR_STATE, &state);

        // save past stops
        for stop in self.myPastStops.clone() {
            stop.write(out, false, true);
            // do not write started and ended twice
            if (stop.parametersSet & STOP_STARTED_SET) == 0 {
                out.write_attr(SUMO_ATTR_STARTED, &utils::common::time::time2string(stop.started));
            }
            if (stop.parametersSet & STOP_ENDED_SET) == 0 {
                out.write_attr(SUMO_ATTR_ENDED, &utils::common::time::time2string(stop.ended));
            }
            out.close_tag("");
        }
        // save upcoming stops
        for stop in self.myStops.iter() {
            stop.write(out);
        }
        // save parameters
        self.myParameter.write_params(out);
        for dev in self.myDevices.iter() {
            dev.m().base2.save_state(out);
        }
        for item in self.myMoveReminders.iter() {
            item.0.m().save_reminder_state(out, &*self);
        }
        out.close_tag("");
    }

    /// C++: `void MEVehicle::loadState(const SUMOSAXAttributes &, const SUMOTime)` [virtual]
    /// Loads the state of this vehicle from the given description
    // void
    // MEVehicle::loadState(const SUMOSAXAttributes& attrs, const SUMOTime offset) {
    //     if (attrs.hasAttribute(SUMO_ATTR_POSITION)) {
    //         throw ProcessError(TL("Error: Invalid vehicles in state (may be a micro state)!"));
    //     }
    //     int routeOffset;
    //     bool stopped;
    //     int pastStops;
    //     int segIndex;
    //     int queIndex;
    //     std::istringstream bis(attrs.getString(SUMO_ATTR_STATE));
    //     bis >> myParameter->parametersSet;
    //     bis >> myDeparture;
    //     bis >> routeOffset;
    //     bis >> myDepartPos;
    //     bis >> segIndex;
    //     bis >> queIndex;
    //     bis >> myEventTime;
    //     bis >> myLastEntryTime;
    //     bis >> myBlockTime;
    //     bis >> stopped;
    //     bis >> pastStops;
    //     bis >> myWasJammed;
    //     bis >> myEventIndex;
    //     bis >> myUnqueuedEventTime;
    //     myDepartPos /= 1000.; // was stored as mm
    //
    //     if (attrs.hasAttribute(SUMO_ATTR_ARRIVALPOS_RANDOMIZED)) {
    //         bool ok;
    //         myArrivalPos = attrs.get<double>(SUMO_ATTR_ARRIVALPOS_RANDOMIZED, getID().c_str(), ok);
    //     }
    //
    //     // load stops
    //     myStops.clear();
    //     addStops(!MSGlobals::gCheckRoutes, &myCurrEdge, false);
    //
    //     if (hasDeparted()) {
    //         myDeparture -= offset;
    //         myEventTime -= offset;
    //         myLastEntryTime -= offset;
    //         myCurrEdge = myRoute->begin() + routeOffset;
    //         // fix stops
    //         while (pastStops > 0) {
    //             for (const auto& rem : myMoveReminders) {
    //                 rem.first->notifyStopEnded();
    //             }
    //             myPastStops.push_back(myStops.front().pars);
    //             myPastStops.back().routeIndex = (int)(myStops.front().edge - myRoute->begin());
    //             myStops.pop_front();
    //             pastStops--;
    //         }
    //         if (segIndex >= 0) {
    //             MESegment* seg = MSGlobals::gMesoNet->getSegmentForEdge(**myCurrEdge);
    //             while (seg->getIndex() != (int)segIndex) {
    //                 seg = seg->getNextSegment();
    //                 if (seg == nullptr) {
    //                     throw ProcessError(TLF("Unknown segment '%:%' for vehicle '%' in loaded state.", (*myCurrEdge)->getID(), segIndex, getID()));
    //                 }
    //             }
    //             setSegment(seg, queIndex);
    //             if (queIndex == MESegment::PARKING_QUEUE) {
    //                 MSGlobals::gMesoNet->addLeaderCar(this, nullptr, true);
    //                 getCurrentEdge()->getLanes()[0]->addParking(this);
    //             }
    //         } else {
    //             // on teleport
    //             setSegment(nullptr, 0);
    //             assert(myEventTime != SUMOTime_MIN);
    //             MSGlobals::gMesoNet->addLeaderCar(this, nullptr, true);
    //         }
    //         // see MSBaseVehicle constructor
    //         if (myParameter->wasSet(VEHPARS_FORCE_REROUTE)) {
    //             calculateArrivalParams(true);
    //         }
    //     }
    //     if (myBlockTime != SUMOTime_MAX) {
    //         myBlockTime -= offset;
    //     }
    //     if (myUnqueuedEventTime != SUMOTime_MIN) {
    //         myUnqueuedEventTime -= offset;
    //     }
    //     std::istringstream dis(attrs.getString(SUMO_ATTR_DISTANCE));
    //     dis >> myOdometer >> myNumberReroutes;
    //     if (stopped) {
    //         myStops.front().startedFromState = true;
    //         myStops.front().reached = true;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, attrs: &SUMOSAXAttributes, offset: SUMOTime) {
        if attrs.has_attribute(SUMO_ATTR_POSITION) {
            panic!("{}", ProcessError::new("Error: Invalid vehicles in state (may be a micro state)!"));
        }
        let me = Ptr::from_mut(self);
        let state = attrs.get_string(SUMO_ATTR_STATE);
        let mut bis = state.split_whitespace();
        self.myParameter.m().parametersSet = next_token::<i64>(&mut bis);
        self.myDeparture = next_token(&mut bis);
        let routeOffset: i32 = next_token(&mut bis);
        self.myDepartPos = next_token(&mut bis);
        let segIndex: i32 = next_token(&mut bis);
        let queIndex: i32 = next_token(&mut bis);
        self.myEventTime = next_token(&mut bis);
        self.myLastEntryTime = next_token(&mut bis);
        self.myBlockTime = next_token(&mut bis);
        let stopped = next_token::<i32>(&mut bis) != 0;
        let mut pastStops: i32 = next_token(&mut bis);
        self.myWasJammed = next_token::<i32>(&mut bis) != 0;
        self.myEventIndex = next_token(&mut bis);
        self.myUnqueuedEventTime = next_token(&mut bis);
        self.myDepartPos /= 1000.; // was stored as mm

        if attrs.has_attribute(SUMO_ATTR_ARRIVALPOS_RANDOMIZED) {
            let mut ok = false;
            let objectid = self.get_id().to_string();
            self.myArrivalPos = attrs.get::<f64>(SUMO_ATTR_ARRIVALPOS_RANDOMIZED, &objectid, &mut ok, true);
        }

        // load stops
        self.myStops.clear();
        // shortcut: addStops(!MSGlobals::gCheckRoutes, &myCurrEdge, false) needs an MSRouteIterator (Unmapped); stops are not re-added from the saved state until it is mapped

        if self.has_departed() {
            self.myDeparture -= offset;
            self.myEventTime -= offset;
            self.myLastEntryTime -= offset;
            self.myCurrEdge = routeOffset as usize;
            // fix stops
            while pastStops > 0 {
                for rem in self.myMoveReminders.iter() {
                    rem.0.m().notify_stop_ended();
                }
                let pars = self.myStops.front().unwrap().pars.clone();
                self.myPastStops.push(pars);
                // shortcut: routeIndex is (int)(stop.edge - myRoute->begin()) on an Unmapped iterator; uses the current route position
                self.myPastStops.last_mut().unwrap().routeIndex = self.myCurrEdge as i32;
                self.myStops.pop_front();
                pastStops -= 1;
            }
            if segIndex >= 0 {
                let curEdge = self.myRoute.index_op(self.myCurrEdge as i32);
                let meso = meso_net();
                let mut seg = meso.m().get_segment_for_edge(&curEdge, 0.);
                while seg.get_index() != segIndex {
                    seg = seg.get_next_segment();
                    if seg.is_null() {
                        panic!("{}", ProcessError::new(utils::tlf!("Unknown segment '%:%' for vehicle '%' in loaded state.", curEdge.get_id(), segIndex, self.get_id())));
                    }
                }
                self.set_segment(seg, queIndex);
                if queIndex == PARKING_QUEUE {
                    meso.m().add_leader_car(me, Ptr::null(), true);
                    let lane0 = self.get_current_edge().get_lanes()[0];
                    lane0.m().add_parking(me.cast::<MSBaseVehicle>());
                }
            } else {
                // on teleport
                self.set_segment(Ptr::null(), 0);
                debug_assert!(self.myEventTime != SUMOTIME_MIN);
                meso_net().m().add_leader_car(me, Ptr::null(), true);
            }
            // see MSBaseVehicle constructor
            if self.myParameter.was_set(VEHPARS_FORCE_REROUTE) {
                self.calculate_arrival_params(true);
            }
        }
        if self.myBlockTime != SUMOTIME_MAX {
            self.myBlockTime -= offset;
        }
        if self.myUnqueuedEventTime != SUMOTIME_MIN {
            self.myUnqueuedEventTime -= offset;
        }
        let dis = attrs.get_string(SUMO_ATTR_DISTANCE);
        let mut dis_tokens = dis.split_whitespace();
        self.myOdometer = next_token(&mut dis_tokens);
        self.myNumberReroutes = next_token(&mut dis_tokens);
        if stopped {
            let front = self.myStops.front_mut().unwrap();
            front.startedFromState = true;
            front.reached = true;
        }
    }

}

