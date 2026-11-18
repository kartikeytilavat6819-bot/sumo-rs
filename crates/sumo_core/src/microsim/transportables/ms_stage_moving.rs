//! Port of microsim/transportables/MSStageMoving.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

use utils::common::msg_handler::write_warning;
use utils::options::options_cont;

// C++ constant MSPModel::UNDEFINED_DIRECTION (MSPModel.h)
const MSPMODEL_UNDEFINED_DIRECTION: i32 = 0;

/// C++ template getSidewalk<MSEdge, MSLane>() (utils/router/IntermodalEdge.h)
pub(crate) fn get_sidewalk_lane(edge: Ptr<MSEdge>, svc: SUMOVehicleClass) -> Ptr<MSLane> {
    crate::utils::router::intermodal_edge::get_sidewalk::<MSEdge, MSLane, SUMOVehicle>(edge, svc)
}

/// An abstract stage providing additional interface for the movement models
/// C++ class `MSStageMoving` (microsim/transportables/MSStageMoving.h)
#[repr(C)]
pub struct MSStageMoving {
    /// C++ base class `MSStage`
    pub base: MSStage,
    /// state that is to be manipulated by MSPModel
    pub myPState: Ptr<MSTransportableStateAdapter>,
    /// The route of the container
    pub myRoute: Vec<Ptr<MSEdge>>,
    /// The original route id
    pub myRouteID: String,
    /// current step
    pub myRouteStep: usize, // C++ iterator into myRoute, stored as an index (== myRoute.len() means end())
    /// The current internal edge this transportable is on or nullptr
    pub myCurrentInternalEdge: Ptr<MSEdge>,
    /// the speed of the transportable
    pub mySpeed: f64,
    /// the depart position
    pub myDepartPos: f64,
    /// the lateral depart position
    pub myDepartPosLat: f64,
    /// the depart lane or -1
    pub myDepartLane: i32,
    pub myLoadedWaitingTime: SUMOTime,
}

impl std::ops::Deref for MSStageMoving {
    type Target = MSStage;
    fn deref(&self) -> &MSStage {
        &self.base
    }
}
impl std::ops::DerefMut for MSStageMoving {
    fn deref_mut(&mut self) -> &mut MSStage {
        &mut self.base
    }
}

impl MSStageMoving {
    /// C++: `void MSStageMoving::MSStageMoving(const MSStageType, const std::vector<const MSEdge *> &, const std::string &, MSStoppingPlace *, const double, const double, const double, const double, const int)`
    /// constructor
    // MSStageMoving(const MSStageType type, const std::vector<const MSEdge*>& route, const std::string& routeID, MSStoppingPlace* toStop, const double speed,
    //                   const double departPos, const double arrivalPos, const double departPosLat, const int departLane) :
    //         MSStage(type, route.back(), toStop, arrivalPos),
    //         myPState(nullptr), myRoute(route), myRouteID(routeID), myRouteStep(myRoute.begin()),
    //         mySpeed(speed), myDepartPos(departPos),
    //         myDepartPosLat(departPosLat), myDepartLane(departLane),
    //         myLoadedWaitingTime(0)
    //     {}
    #[allow(unused_variables)]
    pub fn new(type_: MSStageType, route: &Vec<Ptr<MSEdge>>, routeID: &str, toStop: Ptr<MSStoppingPlace>, speed: f64, departPos: f64, arrivalPos: f64, departPosLat: f64, departLane: i32) -> Self {
        let m = MSStageMoving {
            base: MSStage::new(type_, *route.last().unwrap(), toStop, arrivalPos, 0., ""),
            myPState: Ptr::null(),
            myRoute: route.clone(),
            myRouteID: routeID.to_string(),
            myRouteStep: 0,
            myCurrentInternalEdge: Ptr::null(),
            mySpeed: speed,
            myDepartPos: departPos,
            myDepartPosLat: departPosLat,
            myDepartLane: departLane,
            myLoadedWaitingTime: 0,
        };
        m
    }

    /// C++: `void MSStageMoving::~MSStageMoving() noexcept` [virtual]
    /// destructor
    // MSStageMoving::~MSStageMoving() {
    //     if (myPState != nullptr && myPState->isFinished()) {
    //         delete myPState;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // shortcut: C++ deletes a finished myPState; the state adapter is owned by MSPModel here, so it is not freed
    }

    /// C++: `const MSEdge * MSStageMoving::getNextRouteEdge() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_route_edge(&self) -> Ptr<MSEdge> {
        match self.myType {
            MSStageType::WALKING => Ptr::new(self).cast::<MSStageWalking>().get().unwrap().get_next_route_edge(),
            MSStageType::TRANSHIP => Ptr::new(self).cast::<MSStageTranship>().get().unwrap().get_next_route_edge(),
            _ => panic!("MSStageMoving::getNextRouteEdge is pure virtual; subclasses must provide it"),
        }
    }

    /// C++: `MSTransportableStateAdapter * MSStageMoving::getPState() const` [const]
    // inline MSTransportableStateAdapter* getPState() const {
    //         return myPState;
    //     }
    #[allow(unused_variables)]
    pub fn get_p_state(&self) -> Ptr<MSTransportableStateAdapter> {
        self.myPState
    }

    /// C++: `void MSStageMoving::setPState(MSTransportableStateAdapter *)`
    // inline void setPState(MSTransportableStateAdapter* pstate) {
    //         myPState = pstate;
    //     }
    #[allow(unused_variables)]
    pub fn set_p_state(&mut self, pstate: Ptr<MSTransportableStateAdapter>) {
        self.myPState = pstate;
    }

    /// C++: `const MSEdge * MSStageMoving::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge*
    // MSStageMoving::getEdge() const {
    //     if (myCurrentInternalEdge != nullptr) {
    //         return myCurrentInternalEdge;
    //     } else {
    //         return myRouteStep == myRoute.end() ? nullptr : *myRouteStep;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        if !self.myCurrentInternalEdge.is_null() {
            self.myCurrentInternalEdge
        } else if self.myRouteStep >= self.myRoute.len() {
            Ptr::null()
        } else {
            self.myRoute[self.myRouteStep]
        }
    }

    /// C++: `const MSLane * MSStageMoving::getLane() const` [const, virtual]
    /// Returns the current lane
    // const MSLane*
    // MSStageMoving::getLane() const {
    //     return myPState == nullptr ? nullptr : myPState->getLane();
    // }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        if self.myPState.is_null() {
            Ptr::null()
        } else {
            self.myPState.get_lane()
        }
    }

    /// C++: `const MSEdge * MSStageMoving::getFromEdge() const` [const, virtual]
    /// Returns first edge of the containers route
    // const MSEdge*
    // MSStageMoving::getFromEdge() const {
    //     return myRoute.front();
    // }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        self.myRoute[0]
    }

    /// C++: `ConstMSEdgeVector MSStageMoving::getEdges() const` [const, virtual]
    /// the edges of the current stage
    // ConstMSEdgeVector
    // MSStageMoving::getEdges() const {
    //     return myRoute;
    // }
    #[allow(unused_variables)]
    pub fn get_edges(&self) -> ConstMSEdgeVector {
        self.myRoute.clone()
    }

    /// C++: `double MSStageMoving::getEdgePos(SUMOTime) const` [const, virtual]
    /// Returns the offset from the start of the current edge measured in its natural direction
    // double
    // MSStageMoving::getEdgePos(SUMOTime now) const {
    //     return myPState == nullptr ? myDepartPos : myPState->getEdgePos(now);
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self, now: SUMOTime) -> f64 {
        if self.myPState.is_null() {
            self.myDepartPos
        } else {
            self.myPState.get_edge_pos(now)
        }
    }

    /// C++: `int MSStageMoving::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSStageMoving::getDirection() const {
    //     return myPState == nullptr ? MSPModel::UNDEFINED_DIRECTION : myPState->getDirection();
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        if self.myPState.is_null() {
            MSPMODEL_UNDEFINED_DIRECTION
        } else {
            self.myPState.get_direction()
        }
    }

    /// C++: `Position MSStageMoving::getPosition(SUMOTime) const` [const, virtual]
    /// Returns the position of the container
    // Position
    // MSStageMoving::getPosition(SUMOTime now) const {
    //     return myPState == nullptr ? Position::INVALID : myPState->getPosition(*this, now);
    // }
    #[allow(unused_variables)]
    pub fn get_position(&self, now: SUMOTime) -> Position {
        if self.myPState.is_null() {
            Position::INVALID
        } else {
            self.myPState.get_position(self, now)
        }
    }

    /// C++: `double MSStageMoving::getAngle(SUMOTime) const` [const, virtual]
    /// Returns the angle of the transportable
    // double
    // MSStageMoving::getAngle(SUMOTime now) const {
    //     return myPState == nullptr ? 0. : myPState->getAngle(*this, now);
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self, now: SUMOTime) -> f64 {
        if self.myPState.is_null() {
            0.
        } else {
            self.myPState.get_angle(self, now)
        }
    }

    /// C++: `SUMOTime MSStageMoving::getWaitingTime() const` [const, virtual]
    /// Returns the time the transportable spent waiting
    // SUMOTime
    // MSStageMoving::getWaitingTime() const {
    //     return myPState == nullptr ? 0 : myPState->getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self) -> SUMOTime {
        if self.myPState.is_null() {
            0
        } else {
            self.myPState.get_waiting_time()
        }
    }

    /// C++: `SUMOTime MSStageMoving::getTotalWaitingTime() const` [const, virtual]
    /// Returns the cumulative time the transportable spent waiting
    // SUMOTime
    // MSStageMoving::getTotalWaitingTime() const {
    //     return myPState == nullptr ? myLoadedWaitingTime : myPState->getTotalWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_total_waiting_time(&self) -> SUMOTime {
        if self.myPState.is_null() {
            self.myLoadedWaitingTime
        } else {
            self.myPState.get_total_waiting_time()
        }
    }

    /// C++: `void MSStageMoving::setTotalWaitingTime(SUMOTime)`
    /// Restore waiting time when loading state
    // void setTotalWaitingTime(SUMOTime t) {
    //         myLoadedWaitingTime = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_total_waiting_time(&mut self, t: SUMOTime) {
        self.myLoadedWaitingTime = t;
    }

    /// C++: `double MSStageMoving::getSpeed() const` [const, virtual]
    /// Returns the speed of the transportable
    // double
    // MSStageMoving::getSpeed() const {
    //     return myPState == nullptr ? 0. : myPState->getSpeed(*this);
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        if self.myPState.is_null() {
            0.
        } else {
            self.myPState.get_speed(self)
        }
    }

    /// C++: `double MSStageMoving::getConfiguredSpeed() const` [const]
    /// Returns the configured speed in this stage
    // double getConfiguredSpeed() const {
    //         return mySpeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_configured_speed(&self) -> f64 {
        self.mySpeed
    }

    /// C++: `double MSStageMoving::getMaxSpeed(const MSTransportable *const) const` [const, virtual, pure virtual]
    /// the maximum speed of the transportable
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_max_speed(&self, transportable: Ptr<MSTransportable>) -> f64 {
        match self.myType {
            MSStageType::WALKING => Ptr::new(self).cast::<MSStageWalking>().get().unwrap().get_max_speed(transportable),
            MSStageType::TRANSHIP => Ptr::new(self).cast::<MSStageTranship>().get().unwrap().get_max_speed(transportable),
            _ => panic!("MSStageMoving::getMaxSpeed is pure virtual; subclasses must provide it"),
        }
    }

    /// C++: `bool MSStageMoving::moveToNextEdge(MSTransportable *, SUMOTime, int, MSEdge *, const bool)` [virtual, pure virtual]
    /// move forward and return whether the transportable arrived
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn move_to_next_edge(&mut self, transportable: Ptr<MSTransportable>, currentTime: SUMOTime, prevDir: i32, nextInternal: Ptr<MSEdge>, isReplay: bool) -> bool {
        match self.myType {
            MSStageType::WALKING => Ptr::from_mut(self).cast::<MSStageWalking>().m().move_to_next_edge(transportable, currentTime, prevDir, nextInternal, isReplay),
            MSStageType::TRANSHIP => Ptr::from_mut(self).cast::<MSStageTranship>().m().move_to_next_edge(transportable, currentTime, prevDir, nextInternal, isReplay),
            _ => panic!("MSStageMoving::moveToNextEdge is pure virtual; subclasses must provide it"),
        }
    }

    /// C++: `void MSStageMoving::activateEntryReminders(MSTransportable *, const bool)` [virtual]
    /// add the move reminders for the current lane on entry
    // virtual void activateEntryReminders(MSTransportable* person, const bool isDepart = false) {
    //         UNUSED_PARAMETER(person);
    //         UNUSED_PARAMETER(isDepart);
    //     }
    #[allow(unused_variables)]
    pub fn activate_entry_reminders(&mut self, person: Ptr<MSTransportable>, isDepart: bool) {
        // virtual: only MSStageWalking overrides it; the base body is empty
        match self.myType {
            MSStageType::WALKING => Ptr::from_mut(self).cast::<MSStageWalking>().m().activate_entry_reminders(person, isDepart),
            _ => {}
        }
    }

    /// C++: `void MSStageMoving::setRouteIndex(MSTransportable *const, int)` [virtual]
    /// place transportable on a previously passed edge
    // void
    // MSStageMoving::setRouteIndex(MSTransportable* const transportable, int routeOffset) {
    //     assert(routeOffset >= 0);
    //     assert(routeOffset < (int)myRoute.size());
    //     getEdge()->removeTransportable(transportable);
    //     myRouteStep = myRoute.begin() + routeOffset;
    //     getEdge()->addTransportable(transportable);
    // }
    #[allow(unused_variables)]
    pub fn set_route_index(&mut self, transportable: Ptr<MSTransportable>, routeOffset: i32) {
        assert!(routeOffset >= 0);
        assert!((routeOffset as usize) < self.myRoute.len());
        self.get_edge().remove_transportable(transportable);
        self.myRouteStep = routeOffset as usize;
        self.get_edge().add_transportable(transportable);
    }

    /// C++: `void MSStageMoving::replaceRoute(MSTransportable *const, const ConstMSEdgeVector &, int)` [virtual]
    // void
    // MSStageMoving::replaceRoute(MSTransportable* const transportable, const ConstMSEdgeVector& edges, int routeOffset) {
    //     assert(routeOffset >= 0);
    //     assert(routeOffset < (int)edges.size());
    //     getEdge()->removeTransportable(transportable);
    //     myRoute = edges;
    //     myRouteStep = myRoute.begin() + routeOffset;
    //     getEdge()->addTransportable(transportable);
    // }
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, transportable: Ptr<MSTransportable>, edges: &ConstMSEdgeVector, routeOffset: i32) {
        assert!(routeOffset >= 0);
        assert!((routeOffset as usize) < edges.len());
        self.get_edge().remove_transportable(transportable);
        self.myRoute = edges.clone();
        self.myRouteStep = routeOffset as usize;
        self.get_edge().add_transportable(transportable);
    }

    /// C++: `const std::vector<const MSEdge *> & MSStageMoving::getRoute() const` [const]
    // inline const std::vector<const MSEdge*>& getRoute() const {
    //         return myRoute;
    //     }
    #[allow(unused_variables)]
    pub fn get_route(&self) -> &Vec<Ptr<MSEdge>> {
        &self.myRoute
    }

    /// C++: `const std::vector<const MSEdge *>::iterator MSStageMoving::getRouteStep() const` [const]
    // inline const std::vector<const MSEdge*>::iterator getRouteStep() const {
    //         return myRouteStep;
    //     }
    #[allow(unused_variables)]
    pub fn get_route_step(&self) -> usize {
        self.myRouteStep
    }

    /// C++: `double MSStageMoving::getDepartPos() const` [const]
    // inline double getDepartPos() const {
    //         return myDepartPos;
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_pos(&self) -> f64 {
        self.myDepartPos
    }

    /// C++: `void MSStageMoving::setDepartPos(const double)`
    // inline void setDepartPos(const double pos) {
    //         myDepartPos = pos;
    //     }
    #[allow(unused_variables)]
    pub fn set_depart_pos(&mut self, pos: f64) {
        self.myDepartPos = pos;
    }

    /// C++: `double MSStageMoving::getDepartPosLat() const` [const]
    // inline double getDepartPosLat() const {
    //         return myDepartPosLat;
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_pos_lat(&self) -> f64 {
        self.myDepartPosLat
    }

    /// C++: `int MSStageMoving::getDepartLane() const` [const]
    // inline int getDepartLane() const {
    //         return myDepartLane;
    //     }
    #[allow(unused_variables)]
    pub fn get_depart_lane(&self) -> i32 {
        self.myDepartLane
    }

    /// C++: `const MSLane * MSStageMoving::checkDepartLane(const MSEdge *, SUMOVehicleClass, int, const std::string &)` [static]
    /// interpret custom depart lane
    // const MSLane*
    // MSStageMoving::checkDepartLane(const MSEdge* edge, SUMOVehicleClass svc, int laneIndex, const std::string& id) {
    //     const MSLane* lane = getSidewalk<MSEdge, MSLane>(edge, svc);
    //     if (laneIndex > 0) {
    //         const std::vector<MSLane*>& departLanes = edge->getLanes();
    //         if ((int)departLanes.size() <= laneIndex || !departLanes[laneIndex]->allowsVehicleClass(svc)) {
    //             std::string error = "Invalid departLane '" + toString(laneIndex) + "' for person '" + id + "'";
    //             if (OptionsCont::getOptions().getBool("ignore-route-errors")) {
    //                 WRITE_WARNING(error);
    //                 return nullptr;
    //             } else {
    //                 throw ProcessError(error);
    //             }
    //         } else {
    //             lane = departLanes[laneIndex];
    //         }
    //     }
    //     return lane;
    // }
    #[allow(unused_variables)]
    pub fn check_depart_lane(edge: Ptr<MSEdge>, svc: SUMOVehicleClass, laneIndex: i32, id: &str) -> Ptr<MSLane> {
        let mut lane = get_sidewalk_lane(edge, svc);
        if laneIndex > 0 {
            let departLanes = edge.get_lanes();
            if departLanes.len() as i32 <= laneIndex || !departLanes[laneIndex as usize].allows_vehicle_class(svc) {
                let error = format!("Invalid departLane '{}' for person '{}'", laneIndex, id);
                if options_cont::get_bool("ignore-route-errors") {
                    write_warning(&error);
                    return Ptr::null();
                } else {
                    panic!("{}", ProcessError::new(&error));
                }
            } else {
                lane = departLanes[laneIndex as usize];
            }
        }
        lane
    }

    /// C++: `bool MSStageMoving::equals(const MSStage &) const` [const, virtual]
    // bool equals(const MSStage& s) const {
    //         if (!MSStage::equals(s)) {
    //             return false;
    //         }
    //         // this is safe because MSStage already checked that the type fits
    //         const MSStageMoving& sm = static_cast<const MSStageMoving&>(s);
    //         return myRoute == sm.myRoute &&
    //                myRouteID == sm.myRouteID &&
    //                mySpeed == sm.mySpeed &&
    //                myDepartPos == sm.myDepartPos &&
    //                myDepartPosLat == sm.myDepartPosLat &&
    //                myDepartLane == sm.myDepartLane;
    //     }
    #[allow(unused_variables)]
    pub fn equals(&self, s: &MSStage) -> bool {
        if !self.base.equals_base(s) {
            return false;
        }
        // this is safe because MSStage already checked that the type fits
        let sm = Ptr::new(s).cast::<MSStageMoving>();
        self.myRoute == sm.myRoute
            && self.myRouteID == sm.myRouteID
            && self.mySpeed == sm.mySpeed
            && self.myDepartPos == sm.myDepartPos
            && self.myDepartPosLat == sm.myDepartPosLat
            && self.myDepartLane == sm.myDepartLane
    }

}

