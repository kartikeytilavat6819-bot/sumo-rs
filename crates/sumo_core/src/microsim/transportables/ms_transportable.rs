//! Port of microsim/transportables/MSTransportable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::common::msg_handler::write_warning;
use utils::geom::POSITION_EPS;

/// C++ typedef `std::vector<MSStage *> MSTransportable::MSTransportablePlan`
pub type MSTransportable_MSTransportablePlan = Vec<Ptr<MSStage>>;

/// The class holds a simulated moveable object
/// C++ class `MSTransportable` (microsim/transportables/MSTransportable.h)
#[repr(C)]
pub struct MSTransportable {
    /// C++ base class `SUMOTrafficObject`
    pub base: SUMOTrafficObject,
    /// the plan of the transportable
    pub myParameter: Ptr<SUMOVehicleParameter>,
    /// This transportable's type. (mainly used for drawing related information Note sure if it is really necessary
    pub myVType: Ptr<MSVehicleType>,
    /// Whether events shall be written
    pub myWriteEvents: bool,
    /// the plan of the transportable
    pub myPlan: Ptr<MSTransportable_MSTransportablePlan>,
    /// the iterator over the route
    pub myStep: usize /* index into *myPlan (C++ MSTransportablePlan::iterator) */,
    /// The devices this transportable has
    pub myDevices: Vec<Ptr<MSTransportableDevice>>,
    pub myAmPerson: bool,
    pub myNumericalID: SUMOTrafficObject_NumericalID,
    pub myRandomSeed: i64,
    pub myAbortCommand: Ptr<crate::utils::wrapping_command::WrappingCommand<MSTransportable>>,
}

impl std::ops::Deref for MSTransportable {
    type Target = SUMOTrafficObject;
    fn deref(&self) -> &SUMOTrafficObject {
        &self.base
    }
}
impl std::ops::DerefMut for MSTransportable {
    fn deref_mut(&mut self) -> &mut SUMOTrafficObject {
        &mut self.base
    }
}

// C++ static members of MSTransportable (port as module-level statics, see AGENTS.md):
//   static NumericalID MSTransportable::myCurrentNumericalIndex;
static MY_CURRENT_NUMERICAL_INDEX: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);
// C++ constants from SUMOVehicleParameter.h / SUMOVTypeParameter.h (headers not ported yet)
const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
const VEHPARS_JUNCTIONMODEL_PARAMS_SET: i64 = 2 << 28;
const VTYPEPARS_VEHICLECLASS_SET: i64 = 1 << 7;

// C++ overrides of SUMOTrafficObject in MSTransportable, used by containers (myAmPerson == false); persons use MS_PERSON_VT
fn ms_container_of(o: &SUMOTrafficObject) -> &MSTransportable {
    Ptr::new(o).cast::<MSTransportable>().get().unwrap()
}
fn ms_container_of_mut(o: &mut SUMOTrafficObject) -> &mut MSTransportable {
    Ptr::from_mut(o).cast::<MSTransportable>().get_mut().unwrap()
}
fn ms_container_is_vehicle(_: &SUMOTrafficObject) -> bool {
    false
}
fn ms_container_is_person(_: &SUMOTrafficObject) -> bool {
    false
}
fn ms_container_is_container(_: &SUMOTrafficObject) -> bool {
    true
}
fn ms_container_get_parameter(o: &SUMOTrafficObject) -> &SUMOVehicleParameter {
    ms_container_of(o).get_parameter()
}
fn ms_container_get_edge(o: &SUMOTrafficObject) -> Ptr<MSEdge> {
    ms_container_of(o).get_edge()
}
fn ms_container_get_lane(o: &SUMOTrafficObject) -> Ptr<MSLane> {
    ms_container_of(o).get_lane()
}
fn ms_container_get_speed(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_speed()
}
fn ms_container_get_position(o: &SUMOTrafficObject, offset: f64) -> Position {
    ms_container_of(o).get_position(offset)
}
fn ms_container_get_numerical_id(o: &SUMOTrafficObject) -> SUMOTrafficObject_NumericalID {
    ms_container_of(o).get_numerical_id()
}
fn ms_container_get_random_seed(o: &SUMOTrafficObject) -> i64 {
    ms_container_of(o).get_random_seed()
}
fn ms_container_get_vehicle_type(o: &SUMOTrafficObject) -> &MSVehicleType {
    ms_container_of(o).get_vehicle_type()
}
fn ms_container_get_v_type_parameter(o: &SUMOTrafficObject) -> &SUMOVTypeParameter {
    ms_container_of(o).get_v_type_parameter()
}
fn ms_container_replace_vehicle_type(o: &mut SUMOTrafficObject, type_: Ptr<MSVehicleType>) {
    ms_container_of_mut(o).replace_vehicle_type(type_)
}
fn ms_container_get_rng(o: &SUMOTrafficObject) -> Ptr<SumoRNG> {
    ms_container_of(o).get_rng()
}
fn ms_container_get_rng_index(o: &SUMOTrafficObject) -> i32 {
    ms_container_of(o).get_rng_index()
}
fn ms_container_is_stopped(o: &SUMOTrafficObject) -> bool {
    ms_container_of(o).is_stopped()
}
fn ms_container_get_current_edge(o: &SUMOTrafficObject) -> Ptr<MSEdge> {
    ms_container_of(o).get_current_edge()
}
fn ms_container_get_next_edge_ptr(o: &SUMOTrafficObject) -> Ptr<MSEdge> {
    ms_container_of(o).get_next_edge_ptr()
}
fn ms_container_get_upcoming_edge_i_ds(o: &SUMOTrafficObject) -> BTreeSet<SUMOTrafficObject_NumericalID> {
    ms_container_of(o).get_upcoming_edge_i_ds()
}
fn ms_container_get_back_lane(o: &SUMOTrafficObject) -> Ptr<MSLane> {
    ms_container_of(o).get_back_lane()
}
fn ms_container_get_route_position(o: &SUMOTrafficObject) -> i32 {
    ms_container_of(o).get_route_position()
}
fn ms_container_get_reroute_destination(o: &SUMOTrafficObject) -> Ptr<MSEdge> {
    ms_container_of(o).get_reroute_destination()
}
fn ms_container_get_slope(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_slope()
}
fn ms_container_get_chosen_speed_factor(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_chosen_speed_factor()
}
fn ms_container_get_v_class(o: &SUMOTrafficObject) -> SUMOVehicleClass {
    ms_container_of(o).get_v_class()
}
fn ms_container_get_max_speed(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_max_speed()
}
fn ms_container_get_waiting_time(o: &SUMOTrafficObject, accumulated: bool) -> SUMOTime {
    ms_container_of(o).get_waiting_time(accumulated)
}
fn ms_container_get_acceleration(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_acceleration()
}
fn ms_container_get_position_on_lane(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_position_on_lane()
}
fn ms_container_get_back_position_on_lane(o: &SUMOTrafficObject, lane: Ptr<MSLane>) -> f64 {
    ms_container_of(o).get_back_position_on_lane(lane)
}
fn ms_container_get_angle(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_angle()
}
fn ms_container_has_arrived(o: &SUMOTrafficObject) -> bool {
    ms_container_of(o).has_arrived()
}
fn ms_container_get_device(o: &SUMOTrafficObject, kind: &str) -> Ptr<MSDevice> {
    ms_container_of(o).get_device(kind)
}
fn ms_container_replace_route(o: &mut SUMOTrafficObject, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
    ms_container_of_mut(o).replace_route(route, info, onInit, offset, addStops, removeStops, msgReturn)
}
fn ms_container_get_routing_mode(o: &SUMOTrafficObject) -> i32 {
    ms_container_of(o).get_routing_mode()
}
fn ms_container_set_routing_mode(o: &mut SUMOTrafficObject, value: i32) {
    ms_container_of_mut(o).set_routing_mode(value)
}
fn ms_container_get_previous_speed(o: &SUMOTrafficObject) -> f64 {
    ms_container_of(o).get_previous_speed()
}
fn ms_container_has_influencer(o: &SUMOTrafficObject) -> bool {
    ms_container_of(o).has_influencer()
}
fn ms_container_is_selected(o: &SUMOTrafficObject) -> bool {
    ms_container_of(o).is_selected()
}

static MS_CONTAINER_VT: SUMOTrafficObjectVt = SUMOTrafficObjectVt {
    is_vehicle: ms_container_is_vehicle,
    is_person: ms_container_is_person,
    is_container: ms_container_is_container,
    get_parameter: ms_container_get_parameter,
    get_edge: ms_container_get_edge,
    get_lane: ms_container_get_lane,
    get_speed: ms_container_get_speed,
    get_position: ms_container_get_position,
    get_numerical_id: ms_container_get_numerical_id,
    get_random_seed: ms_container_get_random_seed,
    get_vehicle_type: ms_container_get_vehicle_type,
    get_v_type_parameter: ms_container_get_v_type_parameter,
    replace_vehicle_type: ms_container_replace_vehicle_type,
    get_rng: ms_container_get_rng,
    get_rng_index: ms_container_get_rng_index,
    is_stopped: ms_container_is_stopped,
    get_current_edge: ms_container_get_current_edge,
    get_next_edge_ptr: ms_container_get_next_edge_ptr,
    get_upcoming_edge_i_ds: ms_container_get_upcoming_edge_i_ds,
    get_back_lane: ms_container_get_back_lane,
    get_route_position: ms_container_get_route_position,
    get_reroute_destination: ms_container_get_reroute_destination,
    replace_route: ms_container_replace_route,
    get_slope: ms_container_get_slope,
    get_chosen_speed_factor: ms_container_get_chosen_speed_factor,
    get_v_class: ms_container_get_v_class,
    get_routing_mode: ms_container_get_routing_mode,
    set_routing_mode: ms_container_set_routing_mode,
    get_max_speed: ms_container_get_max_speed,
    get_waiting_time: ms_container_get_waiting_time,
    get_previous_speed: ms_container_get_previous_speed,
    get_acceleration: ms_container_get_acceleration,
    get_position_on_lane: ms_container_get_position_on_lane,
    get_back_position_on_lane: ms_container_get_back_position_on_lane,
    get_angle: ms_container_get_angle,
    has_arrived: ms_container_has_arrived,
    has_influencer: ms_container_has_influencer,
    is_selected: ms_container_is_selected,
    get_device: ms_container_get_device,
};

impl MSTransportable {
    /// C++: `bool MSTransportable::isPerson() const` [const, virtual]
    /// @{
    // inline bool isPerson() const override {
    //         return myAmPerson;
    //     }
    #[allow(unused_variables)]
    pub fn is_person(&self) -> bool {
        self.myAmPerson
    }

    /// C++: `bool MSTransportable::isContainer() const` [const, virtual]
    // inline bool isContainer() const override {
    //         return !myAmPerson;
    //     }
    #[allow(unused_variables)]
    pub fn is_container(&self) -> bool {
        !self.myAmPerson
    }

    /// C++: `std::string MSTransportable::getObjectType()`
    // inline std::string getObjectType() {
    //         return myAmPerson ? "Person" : "Container";
    //     }
    #[allow(unused_variables)]
    pub fn get_object_type(&mut self) -> String {
        if self.myAmPerson {
            "Person".to_string()
        } else {
            "Container".to_string()
        }
    }

    /// C++: `NumericalID MSTransportable::getNumericalID() const` [const, virtual]
    // inline NumericalID getNumericalID() const override {
    //         return myNumericalID;
    //     }
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        self.myNumericalID
    }

    /// C++: `long long MSTransportable::getRandomSeed() const` [const, virtual]
    /// return transportable-specific random number
    // long long int getRandomSeed() const override {
    //         return myRandomSeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        self.myRandomSeed
    }

    /// C++: `bool MSTransportable::isStopped() const` [const, virtual]
    // inline bool isStopped() const override {
    //         return getCurrentStageType() == MSStageType::WAITING;
    //     }
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        self.get_current_stage_type() == MSStageType::WAITING
    }

    /// C++: `double MSTransportable::getSlope() const` [const, virtual]
    // double
    // MSTransportable::getSlope() const {
    //     const MSEdge* edge = getEdge();
    //     const double ep = getEdgePos();
    //     const double gp = edge->getLanes()[0]->interpolateLanePosToGeometryPos(ep);
    //     return edge->getLanes()[0]->getShape().slopeDegreeAtOffset(gp);
    // }
    #[allow(unused_variables)]
    pub fn get_slope(&self) -> f64 {
        let edge = self.get_edge();
        let ep = self.get_edge_pos();
        let gp = edge.get_lanes()[0].interpolate_lane_pos_to_geometry_pos(ep);
        edge.get_lanes()[0].get_shape_2().slope_degree_at_offset(gp)
    }

    /// C++: `SUMOVehicleClass MSTransportable::getVClass() const` [const, virtual]
    // SUMOVehicleClass
    // MSTransportable::getVClass() const {
    //     return getVehicleType().getVehicleClass();
    // }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        self.get_vehicle_type().get_vehicle_class()
    }

    /// C++: `bool MSTransportable::isJammed() const` [const, virtual]
    /// whether the transportable (persons) is jammed as defined by the current pedestrian model
    // virtual bool isJammed() const {
    //         return false;
    //     }
    pub fn is_jammed(&self) -> bool {
        // C++ MSTransportable::isJammed is virtual; MSPerson delegates to its
        // walking stage PState, containers keep the base (false) behaviour.
        if self.is_person() {
            Ptr::new(self).cast::<crate::microsim::transportables::ms_person::MSPerson>().is_jammed()
        } else {
            false
        }
    }

    /// C++: `double MSTransportable::getMaxSpeed() const` [const, virtual]
    /// Returns the maximum speed (the minimum of desired and physical maximum speed)
    // double
    // MSTransportable::getMaxSpeed() const {
    //     return MIN2(getVehicleType().getMaxSpeed(), getVehicleType().getDesiredMaxSpeed() * getChosenSpeedFactor());
    // }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        let a = self.get_vehicle_type().get_max_speed();
        let b = self.get_vehicle_type().get_desired_max_speed() * self.get_chosen_speed_factor();
        if b < a { b } else { a }
    }

    /// C++: `SUMOTime MSTransportable::getWaitingTime(const bool) const` [const, virtual]
    // SUMOTime
    // MSTransportable::getWaitingTime(const bool /* accumulated */) const {
    //     return (*myStep)->getWaitingTime();
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, accumulated: bool) -> SUMOTime {
        self.get_current_stage().get_waiting_time()
    }

    /// C++: `double MSTransportable::getPreviousSpeed() const` [const, virtual]
    // double getPreviousSpeed() const override {
    //         return getSpeed();
    //     }
    #[allow(unused_variables)]
    pub fn get_previous_speed(&self) -> f64 {
        self.get_speed()
    }

    /// C++: `double MSTransportable::getAcceleration() const` [const, virtual]
    // double getAcceleration() const override {
    //         return 0.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_acceleration(&self) -> f64 {
        0.0
    }

    /// C++: `double MSTransportable::getPositionOnLane() const` [const, virtual]
    // double getPositionOnLane() const override {
    //         return getEdgePos();
    //     }
    #[allow(unused_variables)]
    pub fn get_position_on_lane(&self) -> f64 {
        self.get_edge_pos()
    }

    /// C++: `double MSTransportable::getBackPositionOnLane(const MSLane *) const` [const, virtual]
    // double
    // MSTransportable::getBackPositionOnLane(const MSLane* /*lane*/) const {
    //     return getEdgePos() - getVehicleType().getLength();
    // }
    #[allow(unused_variables)]
    pub fn get_back_position_on_lane(&self, lane: Ptr<MSLane>) -> f64 {
        self.get_edge_pos() - self.get_vehicle_type().get_length()
    }

    /// C++: `Position MSTransportable::getPosition(const double) const` [const, virtual]
    // Position getPosition(const double /*offset*/) const override {
    //         return getPosition();
    //     }
    #[allow(unused_variables)]
    pub fn get_position(&self, arg0: f64) -> Position {
        self.get_position_2()
    }

    /// C++: `void MSTransportable::MSTransportable(const SUMOVehicleParameter *, MSVehicleType *, MSTransportablePlan *, const bool)`
    /// constructor
    // MSTransportable::MSTransportable(const SUMOVehicleParameter* pars, MSVehicleType* vtype, MSTransportablePlan* plan, const bool isPerson) :
    //     SUMOTrafficObject(pars->id),
    //     myParameter(pars), myVType(vtype), myPlan(plan),
    //     myAmPerson(isPerson),
    //     myNumericalID(myCurrentNumericalIndex++),
    //     myRandomSeed(RandHelper::murmur3_32(pars->id, RandHelper::getSeed())) {
    //     myStep = myPlan->begin();
    //     // init devices
    //     MSDevice::buildTransportableDevices(*this, myDevices);
    //     for (MSStage* const stage : * myPlan) {
    //         stage->init(this);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn new(pars: Ptr<SUMOVehicleParameter>, vtype: Ptr<MSVehicleType>, plan: Ptr<MSTransportable_MSTransportablePlan>, isPerson: bool) -> Self {
        let numerical_id = MY_CURRENT_NUMERICAL_INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        // Devices and stage->init(this) need the final address: the builders (MSTransportableControl::build_person/build_container) run them after boxing
        let mut t = MSTransportable {
            base: SUMOTrafficObject::new(&pars.id),
            myParameter: pars,
            myVType: vtype,
            myWriteEvents: false,
            myPlan: plan,
            myStep: 0,
            myDevices: Vec::new(),
            myAmPerson: isPerson,
            myNumericalID: numerical_id,
            myRandomSeed: utils::common::rand_helper::murmur3_32(&pars.id, utils::common::rand_helper::with_default(|r| r.orig_seed)) as i64,
            myAbortCommand: Ptr::null(),
        };
        if !isPerson {
            t.base.vt = &MS_CONTAINER_VT;
        }
        t
    }

    /// C++: `void MSTransportable::~MSTransportable() noexcept` [virtual]
    /// destructor
    // MSTransportable::~MSTransportable() {
    //     if (myStep != myPlan->end() && getCurrentStageType() == MSStageType::DRIVING) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(*myStep);
    //         if (stage->getVehicle() != nullptr) {
    //             stage->getVehicle()->removeTransportable(this);
    //         } else if (stage->getOriginStop() != nullptr)  {
    //             stage->getOriginStop()->removeTransportable(this);
    //         }
    //     }
    //     if (myPlan != nullptr) {
    //         for (MSTransportablePlan::const_iterator i = myPlan->begin(); i != myPlan->end(); ++i) {
    //             delete *i;
    //         }
    //         delete myPlan;
    //         myPlan = nullptr;
    //     }
    //     for (MSTransportableDevice* dev : myDevices) {
    //         delete dev;
    //     }
    //     delete myParameter;
    //     if (myVType->isVehicleSpecific()) {
    //         MSNet::getInstance()->getVehicleControl().removeVType(myVType);
    //     }
    // }
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        let plan = self.myPlan;
        if self.myStep != plan.len() && self.get_current_stage_type() == MSStageType::DRIVING {
            let stage: Ptr<MSStageDriving> = self.get_current_stage().cast::<MSStageDriving>();
            if !stage.get_vehicle().is_null() {
                stage.get_vehicle().m().remove_transportable(Ptr::from_mut(self));
            } else if !stage.get_origin_stop().is_null() {
                stage.get_origin_stop().m().remove_transportable(Ptr::from_mut(self));
            }
        }
        // shortcut: C++ also deletes the plan stages, devices and myParameter here; ownership of those objects is not modelled yet
        if self.myVType.is_vehicle_specific() {
            MSNet::get_instance().m().get_vehicle_control().remove_v_type(self.myVType);
        }
    }

    /// C++: `bool MSTransportable::proceed(MSNet *, SUMOTime, const bool)` [virtual]
    // bool
    // MSTransportable::proceed(MSNet* net, SUMOTime time, const bool vehicleArrived) {
    //     MSStage* const prior = *myStep;
    //     const std::string& error = prior->setArrived(net, this, time, vehicleArrived);
    //     // must be done before increasing myStep to avoid invalid state for rendering
    //     prior->getEdge()->removeTransportable(this);
    //     myStep++;
    //     if (error != "") {
    //         throw ProcessError(error);
    //     }
    //     /* We need to check whether an access stage is needed (or maybe even two).
    //        The general scheme is: If the prior stage ended at a stop and the next stage
    //        starts at an edge which is not the one the stop is at, but the stop has an access to it
    //        we need an access stage. The same is true if prior ends at an edge, the next stage
    //        is allowed to start at any stop the edge has access to.
    //        If we start at a stop or end at a stop no access is needed.
    //     */
    //     bool accessToStop = false;
    //     if (prior->getStageType() == MSStageType::WALKING || prior->getStageType() == MSStageType::DRIVING) {
    //         accessToStop = checkAccess(prior);
    //     } else if (prior->getStageType() == MSStageType::WAITING_FOR_DEPART) {
    //         for (MSTransportableDevice* const dev : myDevices) {
    //             dev->notifyEnter(*this, MSMoveReminder::NOTIFICATION_DEPARTED, nullptr);
    //         }
    //     }
    //     if (!accessToStop && (myStep == myPlan->end()
    //                           || ((*myStep)->getStageType() != MSStageType::DRIVING
    //                               && (*myStep)->getStageType() != MSStageType::TRIP))) {
    //         MSStoppingPlace* priorStop = prior->getStageType() == MSStageType::TRIP ? prior->getOriginStop() : prior->getDestinationStop();
    //         // a trip might resolve to DRIVING so we would have to stay at the stop
    //         // if a trip resolves to something else, this step will do stop removal
    //         if (priorStop != nullptr) {
    //             priorStop->removeTransportable(this);
    //         }
    //     }
    //     MSTransportableControl& tc = isPerson() ? MSNet::getInstance()->getPersonControl() : MSNet::getInstance()->getContainerControl();
    //     if (myStep != myPlan->end()) {
    //         if ((*myStep)->getStageType() == MSStageType::WALKING && (prior->getStageType() != MSStageType::ACCESS || prior->getDestination() != (*myStep)->getFromEdge())) {
    //             checkAccess(prior, false);
    //         } else if ((*myStep)->getStageType() == MSStageType::WAITING && prior->getStageType() == MSStageType::WAITING && prior->getDestination() != (*myStep)->getFromEdge()) {
    //             checkAccess(prior, false);
    //         }
    //         if ((*myStep)->proceed(net, this, time, prior)) {
    //             return true;
    //         } else {
    //             // some problem was encountered
    //             tc.addArrived();
    //             return false;
    //         }
    //     } else {
    //         tc.addArrived();
    //         return false;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn proceed(&mut self, net: Ptr<MSNet>, time: SUMOTime, vehicleArrived: bool) -> bool {
        let prior = self.get_current_stage();
        let error = prior.m().set_arrived(net, Ptr::from_mut(self), time, vehicleArrived);
        // must be done before increasing myStep to avoid invalid state for rendering
        prior.get_edge().remove_transportable(Ptr::from_mut(self));
        self.myStep += 1;
        if error != "" {
            panic!("{}", ProcessError::Process(error));
        }
        // We need to check whether an access stage is needed (or maybe even two).
        // The general scheme is: If the prior stage ended at a stop and the next stage
        // starts at an edge which is not the one the stop is at, but the stop has an access to it
        // we need an access stage. The same is true if prior ends at an edge, the next stage
        // is allowed to start at any stop the edge has access to.
        // If we start at a stop or end at a stop no access is needed.
        let mut accessToStop = false;
        if prior.get_stage_type() == MSStageType::WALKING || prior.get_stage_type() == MSStageType::DRIVING {
            accessToStop = self.check_access(prior, true);
        } else if prior.get_stage_type() == MSStageType::WAITING_FOR_DEPART {
            for dev in self.myDevices.clone() {
                dev.m().base2.notify_enter(&mut self.base, MSMoveReminder_Notification::NOTIFICATION_DEPARTED, Ptr::null());
            }
        }
        if !accessToStop && (self.myStep == self.myPlan.len() || (self.myPlan[self.myStep].get_stage_type() != MSStageType::DRIVING && self.myPlan[self.myStep].get_stage_type() != MSStageType::TRIP)) {
            let priorStop = if prior.get_stage_type() == MSStageType::TRIP { prior.get_origin_stop() } else { prior.get_destination_stop() };
            // a trip might resolve to DRIVING so we would have to stay at the stop
            // if a trip resolves to something else, this step will do stop removal
            if !priorStop.is_null() {
                priorStop.m().remove_transportable(Ptr::from_mut(self));
            }
        }
        let mut net_ptr = net;
        let tc: &mut MSTransportableControl = if self.is_person() { net_ptr.m().get_person_control() } else { net_ptr.m().get_container_control() };
        if self.myStep != self.myPlan.len() {
            let next = self.myPlan[self.myStep];
            if next.get_stage_type() == MSStageType::WALKING && (prior.get_stage_type() != MSStageType::ACCESS || prior.get_destination() != next.get_from_edge()) {
                self.check_access(prior, false);
            } else if next.get_stage_type() == MSStageType::WAITING && prior.get_stage_type() == MSStageType::WAITING && prior.get_destination() != next.get_from_edge() {
                self.check_access(prior, false);
            }
            // C++ re-reads *myStep after checkAccess: it may have inserted an access stage
            // at myStep, which is then the stage that proceeds (not the stale `next`).
            let next = self.myPlan[self.myStep];
            if next.m().proceed(net, Ptr::from_mut(self), time, prior) {
                return true;
            } else {
                // some problem was encountered
                tc.add_arrived();
                return false;
            }
        } else {
            tc.add_arrived();
            return false;
        }
    }

    /// C++: `bool MSTransportable::checkAccess(const MSStage *const, const bool)` [virtual]
    // virtual bool checkAccess(const MSStage* const prior, const bool waitAtStop = true) {
    //         UNUSED_PARAMETER(prior);
    //         UNUSED_PARAMETER(waitAtStop);
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn check_access(&mut self, prior: Ptr<MSStage>, waitAtStop: bool) -> bool {
        if self.myAmPerson {
            // MSPerson overrides checkAccess; dispatched by the person flag until vtables exist
            return Ptr::from_mut(self).cast::<MSPerson>().m().check_access(prior, waitAtStop);
        }
        false
    }

    /// C++: `void MSTransportable::setID(const std::string &)` [virtual]
    /// set the id (inherited from Named but forbidden for transportables)
    // void
    // MSTransportable::setID(const std::string& /*newID*/) {
    //     throw ProcessError(TL("Changing a transportable ID is not permitted"));
    // }
    #[allow(unused_variables)]
    pub fn set_id(&mut self, newID: &str) {
        panic!("{}", ProcessError::Process("Changing a transportable ID is not permitted".to_string()));
    }

    /// C++: `const SUMOVehicleParameter & MSTransportable::getParameter() const` [const, virtual]
    // inline const SUMOVehicleParameter& getParameter() const override {
    //         return *myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        &*self.myParameter
    }

    /// C++ `mutable` parameter members are written through the owner's `&mut`
    pub fn get_parameter_mutable(&mut self) -> &mut SUMOVehicleParameter {
        self.myParameter.m()
    }

    /// C++: `const MSVehicleType & MSTransportable::getVehicleType() const` [const, virtual]
    // inline const MSVehicleType& getVehicleType() const override {
    //         return *myVType;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> &MSVehicleType {
        &*self.myVType
    }

    /// C++: `const SUMOVTypeParameter & MSTransportable::getVTypeParameter() const` [const, virtual]
    /// Returns the object's "vehicle" type parameter
    // inline const SUMOVTypeParameter& getVTypeParameter() const override {
    //         return myVType->getParameter();
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        self.myVType.get_parameter()
    }

    /// C++: `SumoRNG * MSTransportable::getRNG() const` [const, virtual]
    /// returns the associated RNG
    // SumoRNG*
    // MSTransportable::getRNG() const {
    //     return getEdge()->getLanes()[0]->getRNG();
    // }
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        let edge = self.get_edge();
        let rng = edge.get_lanes()[0].get_rng();
        rng
    }

    /// C++: `int MSTransportable::getRNGIndex() const` [const, virtual]
    /// returns the index of the associated RNG
    // int
    // MSTransportable::getRNGIndex() const {
    //     const std::vector<MSLane*>& lanes = getEdge()->getLanes();
    //     return lanes.empty() ? 0 : lanes[0]->getRNGIndex();
    // }
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        let edge = self.get_edge();
        let lanes = edge.get_lanes();
        if lanes.is_empty() { 0 } else { lanes[0].get_rng_index() }
    }

    /// C++: `SUMOTime MSTransportable::getDesiredDepart() const` [const]
    /// Returns the desired departure time.
    // SUMOTime
    // MSTransportable::getDesiredDepart() const {
    //     return myParameter->depart;
    // }
    #[allow(unused_variables)]
    pub fn get_desired_depart(&self) -> SUMOTime {
        self.myParameter.depart
    }

    /// C++: `void MSTransportable::setDeparted(SUMOTime)`
    /// logs depart time of the current stage
    // void
    // MSTransportable::setDeparted(SUMOTime now) {
    //     (*myStep)->setDeparted(now);
    // }
    #[allow(unused_variables)]
    pub fn set_departed(&mut self, now: SUMOTime) {
        self.get_current_stage().m().set_departed(now);
    }

    /// C++: `SUMOTime MSTransportable::getDeparture() const` [const]
    /// logs depart time of the current stage
    // SUMOTime
    // MSTransportable::getDeparture() const {
    //     for (const MSStage* const stage : *myPlan) {
    //         if (stage->getDeparted() >= 0) {
    //             return stage->getDeparted();
    //         }
    //     }
    //     return -1;
    // }
    #[allow(unused_variables)]
    pub fn get_departure(&self) -> SUMOTime {
        for stage in self.myPlan.iter() {
            if stage.get_departed() >= 0 {
                return stage.get_departed();
            }
        }
        -1
    }

    /// C++: `const MSEdge * MSTransportable::getDestination() const` [const]
    /// Returns the current destination.
    // const MSEdge* getDestination() const {
    //         return (*myStep)->getDestination();
    //     }
    #[allow(unused_variables)]
    pub fn get_destination(&self) -> Ptr<MSEdge> {
        self.get_current_stage().get_destination()
    }

    /// C++: `const MSEdge * MSTransportable::getNextDestination() const` [const]
    /// Returns the destination after the current destination.
    // const MSEdge* getNextDestination() const {
    //         return (*(myStep + 1))->getDestination();
    //     }
    #[allow(unused_variables)]
    pub fn get_next_destination(&self) -> Ptr<MSEdge> {
        self.myPlan[self.myStep + 1].get_destination()
    }

    /// C++: `const MSEdge * MSTransportable::getEdge() const` [const, virtual]
    /// Returns the current edge
    // const MSEdge* getEdge() const override {
    //         return (*myStep)->getEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        self.get_current_stage().get_edge()
    }

    /// C++: `const MSEdge * MSTransportable::getCurrentEdge() const` [const, virtual]
    // const MSEdge*
    // MSTransportable::getCurrentEdge() const {
    //     const MSLane* lane = getLane();
    //     return lane == nullptr ? getEdge() : &lane->getEdge();
    // }
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        let lane = self.get_lane();
        if lane.is_null() {
            self.get_edge()
        } else {
            Ptr::from_mut(lane.get_edge())
        }
    }

    /// C++: `const MSLane * MSTransportable::getLane() const` [const, virtual]
    /// Returns the current lane (may be nullptr)
    // const MSLane* getLane() const override {
    //         return (*myStep)->getLane();
    //     }
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        self.get_current_stage().get_lane()
    }

    /// C++: `const MSLane * MSTransportable::getBackLane() const` [const, virtual]
    // const MSLane* getBackLane() const override {
    //         return getLane();
    //     }
    #[allow(unused_variables)]
    pub fn get_back_lane(&self) -> Ptr<MSLane> {
        self.get_lane()
    }

    /// C++: `const MSEdge * MSTransportable::getFromEdge() const` [const]
    /// Returns the departure edge
    // const MSEdge* getFromEdge() const {
    //         return (*myStep)->getFromEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_from_edge(&self) -> Ptr<MSEdge> {
        self.get_current_stage().get_from_edge()
    }

    /// C++: `double MSTransportable::getEdgePos() const` [const, virtual]
    /// Return the position on the edge
    // double
    // MSTransportable::getEdgePos() const {
    //     return (*myStep)->getEdgePos(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_edge_pos(&self) -> f64 {
        let now = MSNet::get_instance().get_current_time_step();
        self.get_current_stage().get_edge_pos(now)
    }

    /// C++: `int MSTransportable::getDirection() const` [const, virtual]
    /// Return the movement directon on the edge
    // int
    // MSTransportable::getDirection() const {
    //     return (*myStep)->getDirection();
    // }
    #[allow(unused_variables)]
    pub fn get_direction(&self) -> i32 {
        self.get_current_stage().get_direction()
    }

    /// C++: `Position MSTransportable::getPosition() const` [const, virtual]
    /// Return the Network coordinate of the transportable
    // Position
    // MSTransportable::getPosition() const {
    //     return (*myStep)->getPosition(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_position_2(&self) -> Position {
        let now = MSNet::get_instance().get_current_time_step();
        self.get_current_stage().get_position(now)
    }

    /// C++: `double MSTransportable::getAngle() const` [const, virtual]
    /// return the current angle of the transportable
    // double
    // MSTransportable::getAngle() const {
    //     return (*myStep)->getAngle(MSNet::getInstance()->getCurrentTimeStep());
    // }
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        let now = MSNet::get_instance().get_current_time_step();
        self.get_current_stage().get_angle(now)
    }

    /// C++: `double MSTransportable::getWaitingSeconds() const` [const, virtual]
    /// the time this transportable spent waiting in seconds
    // double
    // MSTransportable::getWaitingSeconds() const {
    //     return STEPS2TIME((*myStep)->getWaitingTime());
    // }
    #[allow(unused_variables)]
    pub fn get_waiting_seconds(&self) -> f64 {
        steps2time(self.get_current_stage().get_waiting_time())
    }

    /// C++: `double MSTransportable::getSpeed() const` [const, virtual]
    /// the current speed of the transportable
    // double
    // MSTransportable::getSpeed() const {
    //     return (*myStep)->getSpeed();
    // }
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        self.get_current_stage().get_speed()
    }

    /// C++: `double MSTransportable::getChosenSpeedFactor() const` [const, virtual]
    /// the current speed factor of the transportable (where applicable)
    // virtual double getChosenSpeedFactor() const override {
    //         return 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        // MSPerson overrides getChosenSpeedFactor(); containers keep the base value 1
        if self.is_person() { self.base.get_chosen_speed_factor() } else { 1. }
    }

    /// C++: `MSStageType MSTransportable::getCurrentStageType() const` [const]
    /// the current stage type of the transportable
    // MSStageType getCurrentStageType() const {
    //         return (*myStep)->getStageType();
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_type(&self) -> MSStageType {
        self.get_current_stage().get_stage_type()
    }

    /// C++: `MSStageType MSTransportable::getStageType(int) const` [const]
    /// the stage type for the nth next stage
    // MSStageType getStageType(int next) const {
    //         assert(myStep + next < myPlan->end());
    //         assert(myStep + next >= myPlan->begin());
    //         return (*(myStep + next))->getStageType();
    //     }
    #[allow(unused_variables)]
    pub fn get_stage_type(&self, next: i32) -> MSStageType {
        self.myPlan[(self.myStep as i64 + next as i64) as usize].get_stage_type()
    }

    /// C++: `std::string MSTransportable::getStageSummary(int) const` [const]
    /// return textual summary for the given stage
    // std::string
    // MSTransportable::getStageSummary(int stageIndex) const {
    //     assert(stageIndex < (int)myPlan->size());
    //     assert(stageIndex >= 0);
    //     return (*myPlan)[stageIndex]->getStageSummary(myAmPerson);
    // }
    #[allow(unused_variables)]
    pub fn get_stage_summary(&self, stageIndex: i32) -> String {
        self.myPlan[stageIndex as usize].get_stage_summary(self.myAmPerson)
    }

    /// C++: `std::string MSTransportable::getCurrentStageDescription() const` [const]
    /// Returns the current stage description as a string
    // std::string getCurrentStageDescription() const {
    //         return (*myStep)->getStageDescription(myAmPerson);
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_description(&self) -> String {
        self.get_current_stage().get_stage_description(self.myAmPerson)
    }

    /// C++: `MSStage * MSTransportable::getCurrentStage() const` [const]
    /// Return the current stage
    // MSStage* getCurrentStage() const {
    //         return *myStep;
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage(&self) -> Ptr<MSStage> {
        self.myPlan[self.myStep]
    }

    /// C++: `MSStage * MSTransportable::getNextStage(int) const` [const]
    /// Return the next (or previous) stage denoted by the offset
    // inline MSStage* getNextStage(int offset) const {
    //         assert(myStep + offset >= myPlan->begin());
    //         assert(myStep + offset < myPlan->end());
    //         return *(myStep + offset);
    //     }
    #[allow(unused_variables)]
    pub fn get_next_stage(&self, offset: i32) -> Ptr<MSStage> {
        self.myPlan[(self.myStep as i64 + offset as i64) as usize]
    }

    /// C++: `const std::set<NumericalID> MSTransportable::getUpcomingEdgeIDs() const` [const, virtual]
    /// returns the numerical IDs of edges to be used (possibly of future stages)
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn get_upcoming_edge_i_ds(&self) -> BTreeSet<SUMOTrafficObject_NumericalID> {
        let mut result = BTreeSet::new();
        for step in self.myPlan[self.myStep..].iter() {
            for e in step.get_edges() {
                if !e.is_null() {
                    // catch incomplete rides
                    result.insert(e.get_numerical_id() as SUMOTrafficObject_NumericalID);
                }
            }
        }
        result
    }

    /// C++: `int MSTransportable::getNumStages() const` [const]
    /// Return the total number stages in this person's plan
    // inline int getNumStages() const {
    //         return (int)myPlan->size();
    //     }
    #[allow(unused_variables)]
    pub fn get_num_stages(&self) -> i32 {
        self.myPlan.len() as i32
    }

    /// C++: `int MSTransportable::getNumRemainingStages() const` [const]
    /// Return the number of remaining stages (including the current)
    // inline int getNumRemainingStages() const {
    //         return (int)(myPlan->end() - myStep);
    //     }
    #[allow(unused_variables)]
    pub fn get_num_remaining_stages(&self) -> i32 {
        (self.myPlan.len() - self.myStep) as i32
    }

    /// C++: `int MSTransportable::getCurrentStageIndex() const` [const]
    /// Return the index of the current stage
    // inline int getCurrentStageIndex() const {
    //         return (int)(myStep - myPlan->begin());
    //     }
    #[allow(unused_variables)]
    pub fn get_current_stage_index(&self) -> i32 {
        self.myStep as i32
    }

    /// C++: `int MSTransportable::getRoutePosition() const` [const, virtual]
    /// return the index of the edge within the route
    // inline int getRoutePosition() const override {
    //         return (*myStep)->getRoutePosition();
    //     }
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        self.get_current_stage().get_route_position()
    }

    /// C++: `const MSEdge * MSTransportable::getNextEdgePtr() const` [const, virtual]
    /// returns the next edge ptr (used by walking persons)
    // virtual const MSEdge* getNextEdgePtr() const override {
    //         return nullptr;
    //     }
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        Ptr::null()
    }

    /// C++: `void MSTransportable::tripInfoOutput(OutputDevice &) const` [const]
    /// Called on writing tripinfo output
    // void
    // MSTransportable::tripInfoOutput(OutputDevice& os) const {
    //     SUMOTime departure = myPlan->front()->getDeparted();
    //     os.openTag(isPerson() ? "personinfo" : "containerinfo");
    //     os.writeAttr(SUMO_ATTR_ID, getID());
    //     os.writeAttr(SUMO_ATTR_DEPART, departure >= 0 ? time2string(departure) : "-1");
    //     os.writeAttr(SUMO_ATTR_TYPE, getVehicleType().getID());
    //     if (isPerson()) {
    //         os.writeAttr(SUMO_ATTR_SPEEDFACTOR, getChosenSpeedFactor());
    //     }
    //     SUMOTime duration = 0;
    //     SUMOTime waitingTime = 0;
    //     SUMOTime timeLoss = 0;
    //     SUMOTime travelTime = 0;
    //     bool durationOK = true;
    //     bool waitingTimeOK = true;
    //     bool timeLossOK = true;
    //     bool travelTimeOK = true;
    //     for (MSStage* const i : *myPlan) {
    //         SUMOTime t = i->getDuration();
    //         if (t != SUMOTime_MAX) {
    //             duration += t;
    //         } else {
    //             durationOK = false;
    //         }
    //         t = i->getTotalWaitingTime();
    //         if (t != SUMOTime_MAX) {
    //             waitingTime += t;
    //         } else {
    //             waitingTimeOK = false;
    //         }
    //         t = i->getTimeLoss(this);
    //         if (t != SUMOTime_MAX) {
    //             timeLoss += t;
    //         } else {
    //             timeLossOK = false;
    //         }
    //         t = i->getTravelTime();
    //         if (t != SUMOTime_MAX) {
    //             travelTime += t;
    //         } else {
    //             travelTimeOK = false;
    //         }
    //     }
    //     os.writeAttr(SUMO_ATTR_DURATION, durationOK ? time2string(duration) : "-1");
    //     os.writeAttr(SUMO_ATTR_WAITINGTIME, waitingTimeOK ? time2string(waitingTime) : "-1");
    //     os.writeAttr(SUMO_ATTR_TIMELOSS, timeLossOK ? time2string(timeLoss) : "-1");
    //     os.writeAttr(SUMO_ATTR_TRAVELTIME, travelTimeOK ? time2string(travelTime) : "-1");
    //     for (MSStage* const i : *myPlan) {
    //         i->tripInfoOutput(os, this);
    //     }
    //     os.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn trip_info_output(&self, os: &mut OutputDevice) {
        let departure = self.myPlan[0].get_departed();
        os.open_tag(if self.is_person() { "personinfo" } else { "containerinfo" });
        os.write_attr(SUMO_ATTR_ID, self.get_id());
        let departStr = if departure >= 0 { time2string(departure) } else { "-1".to_string() };
        os.write_attr(SUMO_ATTR_DEPART, &departStr);
        os.write_attr(SUMO_ATTR_TYPE, self.get_vehicle_type().get_id());
        if self.is_person() {
            os.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.get_chosen_speed_factor());
        }
        let mut duration: SUMOTime = 0;
        let mut waitingTime: SUMOTime = 0;
        let mut timeLoss: SUMOTime = 0;
        let mut travelTime: SUMOTime = 0;
        let mut durationOK = true;
        let mut waitingTimeOK = true;
        let mut timeLossOK = true;
        let mut travelTimeOK = true;
        // C++ accumulates SUMOTime (int) with +=; the official NDEBUG binary wraps on overflow.
        for i in self.myPlan.iter() {
            let mut t = i.get_duration();
            if t != SUMOTIME_MAX {
                duration = duration.wrapping_add(t);
            } else {
                durationOK = false;
            }
            t = i.get_total_waiting_time();
            if t != SUMOTIME_MAX {
                waitingTime = waitingTime.wrapping_add(t);
            } else {
                waitingTimeOK = false;
            }
            t = i.get_time_loss(Ptr::new(self));
            if t != SUMOTIME_MAX {
                timeLoss = timeLoss.wrapping_add(t);
            } else {
                timeLossOK = false;
            }
            t = i.get_travel_time();
            if t != SUMOTIME_MAX {
                travelTime = travelTime.wrapping_add(t);
            } else {
                travelTimeOK = false;
            }
        }
        let durStr = if durationOK { time2string(duration) } else { "-1".to_string() };
        os.write_attr(SUMO_ATTR_DURATION, &durStr);
        let waitStr = if waitingTimeOK { time2string(waitingTime) } else { "-1".to_string() };
        os.write_attr(SUMO_ATTR_WAITINGTIME, &waitStr);
        let lossStr = if timeLossOK { time2string(timeLoss) } else { "-1".to_string() };
        os.write_attr(SUMO_ATTR_TIMELOSS, &lossStr);
        let travelStr = if travelTimeOK { time2string(travelTime) } else { "-1".to_string() };
        os.write_attr(SUMO_ATTR_TRAVELTIME, &travelStr);
        for i in self.myPlan.iter() {
            i.trip_info_output(os, Ptr::new(self));
        }
        os.close_tag("");
    }

    /// C++: `void MSTransportable::routeOutput(OutputDevice &, const bool) const` [const]
    /// Called on writing vehroute output
    // void
    // MSTransportable::routeOutput(OutputDevice& os, const bool withRouteLength) const {
    //     const std::string typeID = (
    //                                    (isPerson() && getVehicleType().getID() == DEFAULT_PEDTYPE_ID)
    //                                    || (isContainer() && getVehicleType().getID() == DEFAULT_CONTAINERTYPE_ID)) ? "" : getVehicleType().getID();
    //     myParameter->write(os, OptionsCont::getOptions(), isPerson() ? SUMO_TAG_PERSON : SUMO_TAG_CONTAINER, typeID);
    //     if (hasArrived()) {
    //         os.writeAttr("arrival", time2string(MSNet::getInstance()->getCurrentTimeStep()));
    //     }
    //     const MSStage* previous = nullptr;
    //     const bool withTiming = OptionsCont::getOptions().getBool("vehroute-output.exit-times");
    //     for (const MSStage* const stage : *myPlan) {
    //         stage->routeOutput(myAmPerson, os, withRouteLength, previous, withTiming);
    //         previous = stage;
    //     }
    //     myParameter->writeParams(os);
    //     os.closeTag();
    //     os.lf();
    // }
    #[allow(unused_variables)]
    pub fn route_output(&self, os: &mut OutputDevice, withRouteLength: bool) {
        let typeID: String = if (self.is_person() && self.get_vehicle_type().get_id() == DEFAULT_PEDTYPE_ID) || (self.is_container() && self.get_vehicle_type().get_id() == DEFAULT_CONTAINERTYPE_ID) {
            String::new()
        } else {
            self.get_vehicle_type().get_id().to_string()
        };
        let altTag = if self.is_person() { SUMO_TAG_PERSON } else { SUMO_TAG_CONTAINER };
        let myParameter = self.myParameter;
        utils::options::options_cont::with_options(|oc| myParameter.write(os, oc, altTag, &typeID));
        if self.has_arrived() {
            let now = MSNet::get_instance().get_current_time_step();
            os.write_attr("arrival", &time2string(now));
        }
        let mut previous: Ptr<MSStage> = Ptr::null();
        let withTiming = utils::options::options_cont::get_bool("vehroute-output.exit-times");
        for stage in self.myPlan.iter() {
            stage.route_output(self.myAmPerson, os, withRouteLength, previous, withTiming, false);
            previous = *stage;
        }
        myParameter.write_params(os);
        os.close_tag("");
        os.lf();
    }

    /// C++: `bool MSTransportable::isWaitingFor(const SUMOVehicle *) const` [const]
    /// Whether the transportable waits for the given vehicle in the current step
    // bool isWaitingFor(const SUMOVehicle* vehicle) const {
    //         return (*myStep)->isWaitingFor(vehicle);
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting_for(&self, vehicle: Ptr<SUMOVehicle>) -> bool {
        self.get_current_stage().is_waiting_for(vehicle)
    }

    /// C++: `bool MSTransportable::isWaiting4Vehicle() const` [const]
    /// Whether the transportable waits for a vehicle
    // bool isWaiting4Vehicle() const {
    //         return (*myStep)->isWaiting4Vehicle();
    //     }
    #[allow(unused_variables)]
    pub fn is_waiting4_vehicle(&self) -> bool {
        self.get_current_stage().is_waiting4_vehicle()
    }

    /// C++: `void MSTransportable::setAbortWaiting(const SUMOTime)`
    // void
    // MSTransportable::setAbortWaiting(const SUMOTime timeout) {
    //     if (timeout < 0 && myAbortCommand != nullptr) {
    //         myAbortCommand->deschedule();
    //         myAbortCommand = nullptr;
    //         return;
    //     }
    //     myAbortCommand = new WrappingCommand<MSTransportable>(this, &MSTransportable::abortStage);
    //     MSNet::getInstance()->getBeginOfTimestepEvents()->addEvent(myAbortCommand, SIMSTEP + timeout);
    // }
    #[allow(unused_variables)]
    pub fn set_abort_waiting(&mut self, timeout: SUMOTime) {
        if timeout < 0 && !self.myAbortCommand.is_null() {
            self.myAbortCommand.m().deschedule();
            self.myAbortCommand = Ptr::null();
            return;
        }
        let cmd = Box::into_raw(Box::new(crate::utils::wrapping_command::WrappingCommand::new(Ptr::from_mut(self), MSTransportable::abort_stage)));
        self.myAbortCommand = Ptr::from_raw(cmd);
        MSNet::get_instance().m().get_begin_of_timestep_events().m().add_event(Ptr::from_raw(cmd as *mut dyn crate::microsim::ms_event_control::Command), MSNet::get_instance().get_current_time_step() + timeout);
    }

    /// C++: `SUMOTime MSTransportable::abortStage(SUMOTime)`
    /// Abort current stage (used for aborting waiting for a vehicle)
    // SUMOTime
    // MSTransportable::abortStage(SUMOTime step) {
    //     WRITE_WARNINGF(TL("Teleporting % '%'; waited too long, from edge '%', time=%."),
    //                    isPerson() ? "person" : "container", getID(), (*myStep)->getEdge()->getID(), time2string(step));
    //     MSTransportableControl& tc = isPerson() ? MSNet::getInstance()->getPersonControl() : MSNet::getInstance()->getContainerControl();
    //     tc.registerTeleportAbortWait();
    //     (*myStep)->abort(this);
    //     if (!proceed(MSNet::getInstance(), step)) {
    //         tc.erase(this);
    //     }
    //     return 0;
    // }
    #[allow(unused_variables)]
    pub fn abort_stage(&mut self, step: SUMOTime) -> SUMOTime {
        write_warningf!("Teleporting % '%'; waited too long, from edge '%', time=%.", (if self.is_person() { "person" } else { "container" }).to_string(), self.get_id().to_string(), self.get_current_stage().get_edge().get_id().to_string(), time2string(step));
        let net = MSNet::get_instance();
        let tc: &mut MSTransportableControl = if self.is_person() { net.m().get_person_control() } else { net.m().get_container_control() };
        tc.register_teleport_abort_wait();
        self.get_current_stage().m().abort(Ptr::from_mut(self));
        if !self.proceed(net, step, false) {
            tc.erase(Ptr::from_mut(self));
        }
        0
    }

    /// C++: `SUMOVehicle * MSTransportable::getVehicle() const` [const]
    /// The vehicle associated with this transportable
    // SUMOVehicle* getVehicle() const {
    //         return (*myStep)->getVehicle();
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle(&self) -> Ptr<SUMOVehicle> {
        self.get_current_stage().get_vehicle()
    }

    /// C++: `void MSTransportable::appendStage(MSStage *, int)`
    /// Appends the given stage to the current plan
    // void
    // MSTransportable::appendStage(MSStage* stage, int next) {
    //     // myStep is invalidated upon modifying myPlan
    //     const int stepIndex = (int)(myStep - myPlan->begin());
    //     if (next < 0) {
    //         myPlan->push_back(stage);
    //     } else {
    //         if (stepIndex + next > (int)myPlan->size()) {
    //             throw ProcessError("invalid index '" + toString(next) + "' for inserting new stage into plan of '" + getID() + "'");
    //         }
    //         myPlan->insert(myPlan->begin() + stepIndex + next, stage);
    //     }
    //     myStep = myPlan->begin() + stepIndex;
    // }
    #[allow(unused_variables)]
    pub fn append_stage(&mut self, stage: Ptr<MSStage>, next: i32) {
        // myStep is invalidated upon modifying myPlan
        let stepIndex = self.myStep as i32;
        if next < 0 {
            self.myPlan.m().push(stage);
        } else {
            if stepIndex + next > self.myPlan.len() as i32 {
                panic!("{}", ProcessError::Process(format!("invalid index '{}' for inserting new stage into plan of '{}'", next, self.get_id())));
            }
            self.myPlan.m().insert((stepIndex + next) as usize, stage);
        }
        self.myStep = stepIndex as usize;
    }

    /// C++: `void MSTransportable::removeStage(int, bool)`
    /// removes the nth next stage
    // void
    // MSTransportable::removeStage(int next, bool stayInSim) {
    //     assert(myStep + next < myPlan->end());
    //     assert(next >= 0);
    //     if (next > 0) {
    //         // myStep is invalidated upon modifying myPlan
    //         int stepIndex = (int)(myStep - myPlan->begin());
    //         delete *(myStep + next);
    //         myPlan->erase(myStep + next);
    //         myStep = myPlan->begin() + stepIndex;
    //     } else {
    //         if (myStep + 1 == myPlan->end() && stayInSim) {
    //             // stay in the simulation until the start of simStep to allow appending new stages (at the correct position)
    //             appendStage(new MSStageWaiting(getEdge(), nullptr, 0, 0, getEdgePos(), "last stage removed", false));
    //         } else if (myStep + 1 != myPlan->end()) {
    //             (*(myStep + 1))->setOrigin(getEdge(), getEdge() == getFromEdge() ? (*myStep)->getOriginStop() : nullptr, getEdgePos());
    //         }
    //         (*myStep)->abort(this);
    //         if (!proceed(MSNet::getInstance(), SIMSTEP)) {
    //             MSNet::getInstance()->getPersonControl().erase(this);
    //         } else if (myPlan->front()->getDeparted() < 0) {
    //             myPlan->front()->setDeparted(SIMSTEP);
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn remove_stage(&mut self, next: i32, stayInSim: bool) {
        assert!(next >= 0);
        if next > 0 {
            // myStep is invalidated upon modifying myPlan
            let stepIndex = self.myStep;
            // shortcut: the removed stage is not freed, ownership of plan stages is not modelled yet
            self.myPlan.m().remove(self.myStep + next as usize);
            self.myStep = stepIndex;
        } else {
            if self.myStep + 1 == self.myPlan.len() && stayInSim {
                // stay in the simulation until the start of simStep to allow appending new stages (at the correct position)
                let stage = MSStageWaiting::new(self.get_edge(), Ptr::null(), 0, 0, self.get_edge_pos(), "last stage removed", false, -1);
                let stagePtr = Ptr::from_mut(Box::leak(Box::new(stage))).cast::<MSStage>();
                self.append_stage(stagePtr, -1);
            } else if self.myStep + 1 != self.myPlan.len() {
                let originStop = if self.get_edge() == self.get_from_edge() { self.get_current_stage().get_origin_stop() } else { Ptr::null() };
                self.myPlan[self.myStep + 1].m().set_origin(self.get_edge(), originStop, self.get_edge_pos());
            }
            self.get_current_stage().m().abort(Ptr::from_mut(self));
            let simstep = MSNet::get_instance().get_current_time_step();
            if !self.proceed(MSNet::get_instance(), simstep, false) {
                MSNet::get_instance().m().get_person_control().erase(Ptr::from_mut(self));
            } else if self.myPlan[0].get_departed() < 0 {
                self.myPlan[0].m().set_departed(simstep);
            }
        }
    }

    /// C++: `void MSTransportable::setSpeed(double)`
    /// set the speed for all present and future (walking) stages and modify the vType so that stages added later are also affected
    // void
    // MSTransportable::setSpeed(double speed) {
    //     for (MSTransportablePlan::const_iterator i = myStep; i != myPlan->end(); ++i) {
    //         (*i)->setSpeed(speed);
    //     }
    //     getSingularType().setMaxSpeed(speed);
    // }
    #[allow(unused_variables)]
    pub fn set_speed(&mut self, speed: f64) {
        for i in self.myStep..self.myPlan.len() {
            self.myPlan[i].m().set_speed(speed);
        }
        self.get_singular_type().set_max_speed(&speed);
    }

    /// C++: `double MSTransportable::getArrivalPos() const` [const]
    /// returns the final arrival pos
    // double getArrivalPos() const {
    //         return myPlan->back()->getArrivalPos();
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_pos(&self) -> f64 {
        self.myPlan[self.myPlan.len() - 1].get_arrival_pos()
    }

    /// C++: `const MSEdge * MSTransportable::getArrivalEdge() const` [const]
    /// returns the final arrival edge
    // const MSEdge* getArrivalEdge() const {
    //         return myPlan->back()->getEdges().back();
    //     }
    #[allow(unused_variables)]
    pub fn get_arrival_edge(&self) -> Ptr<MSEdge> {
        self.myPlan[self.myPlan.len() - 1].get_edges().last().copied().unwrap()
    }

    /// C++: `const MSEdge * MSTransportable::getRerouteDestination() const` [const, virtual]
    /// Returns the end point for reroutes (usually the last edge of the route)
    // const MSEdge* getRerouteDestination() const override {
    //         return getArrivalEdge();
    //     }
    #[allow(unused_variables)]
    pub fn get_reroute_destination(&self) -> Ptr<MSEdge> {
        self.get_arrival_edge()
    }

    /// C++: `bool MSTransportable::reroute(SUMOTime, const std::string &, MSTransportableRouter &, const bool, const bool, const bool, const MSEdge *)`
    // bool
    // MSTransportable::reroute(SUMOTime t, const std::string& /* info */, MSTransportableRouter& router, const bool /* onInit */, const bool /* withTaz */, const bool /* silent */, const MSEdge* /* sink */) {
    //     MSStageTrip* trip = getCurrentStage()->getTrip();
    //     if (trip == nullptr) {
    //         // TODO this should be possible after factoring out MSStageTrip::reroute
    //         return false;
    //     }
    //     if (getCurrentStage()->getVehicle() != nullptr) {
    //         // TODO rerouting during a ride still needs to be implemented
    //         return false;
    //     }
    //     // find the final stage of the trip
    //     int tripEndOffset = -1;
    //     for (int i = getNumRemainingStages() - 1; i >= 0; i--) {
    //         if (getNextStage(i)->getTrip() == trip) {
    //             tripEndOffset = i;
    //             break;
    //         }
    //     }
    //     std::vector<MSStage*> stages;
    //     MSStageWaiting start(getEdge(), getCurrentStage()->getOriginStop(), -1, t, getEdgePos(), "start", true);
    //     if (trip->reroute(t, router, this, &start, getEdge(), getRerouteDestination(), stages) == "") {
    //         // check whether the new plan actually differs
    //         while (tripEndOffset >= 0 && !stages.empty() && stages.back()->equals(*getNextStage(tripEndOffset))) {
    //             delete stages.back();
    //             stages.pop_back();
    //             tripEndOffset--;
    //         }
    //         bool abortCurrent = true;
    //         // check whether the future route of the current stage is identical to the route
    //         if (!stages.empty() && stages.front()->isWalk() && getCurrentStage()->isWalk()) {
    //             // TODO this check should be done for rides as well
    //             MSStageMoving* s = static_cast<MSStageMoving*>(getCurrentStage());
    //             int routeIndex = (int)(s->getRouteStep() - s->getRoute().begin());
    //             ConstMSEdgeVector oldEdges = s->getEdges();
    //             oldEdges.erase(oldEdges.begin(), oldEdges.begin() + routeIndex);
    //             ConstMSEdgeVector newEdges = stages.front()->getEdges();
    //             if (newEdges == oldEdges) {
    //                 delete stages.front();
    //                 stages.erase(stages.begin());
    //                 abortCurrent = false;
    //             }
    //         }
    //         if (stages.empty()) {
    //             return false;
    //         }
    //         // remove future stages of the trip
    //         for (int i = tripEndOffset; i >= 1; i--) {
    //             removeStage(i);
    //         }
    //         // insert new stages of the rerouting
    //         int idx = 1;
    //         for (MSStage* stage : stages) {
    //             appendStage(stage, idx++);
    //         }
    //         if (abortCurrent) {
    //             removeStage(0);
    //         }
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn reroute(&mut self, t: SUMOTime, info: &str, router: &mut MSTransportableRouter, onInit: bool, withTaz: bool, silent: bool, sink: Ptr<MSEdge>) -> bool {
        let trip: Ptr<MSStageTrip> = self.get_current_stage().get_trip();
        if trip.is_null() {
            // TODO this should be possible after factoring out MSStageTrip::reroute
            return false;
        }
        if !self.get_current_stage().get_vehicle().is_null() {
            // TODO rerouting during a ride still needs to be implemented
            return false;
        }
        // find the final stage of the trip
        let mut tripEndOffset: i32 = -1;
        for i in (0..self.get_num_remaining_stages()).rev() {
            if self.get_next_stage(i).get_trip() == trip {
                tripEndOffset = i;
                break;
            }
        }
        let mut stages: Vec<Ptr<MSStage>> = Vec::new();
        let mut start = MSStageWaiting::new(self.get_edge(), self.get_current_stage().get_origin_stop(), -1, t, self.get_edge_pos(), "start", true, -1);
        let startPtr = Ptr::from_mut(&mut start).cast::<MSStage>();
        let origin = self.get_edge();
        let destination = self.get_reroute_destination();
        if trip.m().reroute(t, router, Ptr::from_mut(self), startPtr, origin, destination, &mut stages).is_empty() {
            // check whether the new plan actually differs
            // shortcut: deleted stages are not freed, ownership of stages is not modelled yet
            while tripEndOffset >= 0 && !stages.is_empty() && stages.last().unwrap().equals(&*self.get_next_stage(tripEndOffset)) {
                stages.pop();
                tripEndOffset -= 1;
            }
            let mut abortCurrent = true;
            // check whether the future route of the current stage is identical to the route
            if !stages.is_empty() && stages.first().unwrap().is_walk() && self.get_current_stage().is_walk() {
                let s: Ptr<MSStageMoving> = self.get_current_stage().cast::<MSStageMoving>();
                let routeIndex = s.get_route_step();
                let mut oldEdges = s.get_edges();
                oldEdges.drain(0..routeIndex);
                let newEdges = stages.first().unwrap().get_edges();
                if newEdges == oldEdges {
                    stages.remove(0);
                    abortCurrent = false;
                }
            }
            if stages.is_empty() {
                return false;
            }
            // remove future stages of the trip
            let mut i = tripEndOffset;
            while i >= 1 {
                self.remove_stage(i, true);
                i -= 1;
            }
            // insert new stages of the rerouting
            let mut idx = 1;
            for stage in stages.iter() {
                self.append_stage(*stage, idx);
                idx += 1;
            }
            if abortCurrent {
                self.remove_stage(0, true);
            }
            return true;
        }
        false
    }

    /// C++: `bool MSTransportable::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual]
    /// Replaces the current route by the given one
    // bool
    // MSTransportable::replaceRoute(ConstMSRoutePtr newRoute, const std::string& /* info */, bool /* onInit */, int /* offset */, bool /* addRouteStops */, bool /* removeStops */, std::string* /* msgReturn */) {
    //     if (isPerson()) {
    //         static_cast<MSPerson*>(this)->replaceWalk(newRoute->getEdges(), getPositionOnLane(), 0, 1);
    //         return true;
    //     }
    //     return false;
    // }
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        if self.is_person() {
            Ptr::from_mut(self).cast::<MSPerson>().m().replace_walk(route.get_edges(), self.get_position_on_lane(), 0, 1);
            return true;
        }
        false
    }

    /// C++: `void MSTransportable::replaceVehicleType(const MSVehicleType *)` [virtual]
    /// Replaces the current vehicle type by the one given
    // void
    // MSTransportable::replaceVehicleType(const MSVehicleType* type) {
    //     const SUMOVehicleClass oldVClass = myVType->getVehicleClass();
    //     if (myVType->isVehicleSpecific()) {
    //         MSNet::getInstance()->getVehicleControl().removeVType(myVType);
    //     }
    //     if (isPerson()
    //             && type->getVehicleClass() != oldVClass
    //             && type->getVehicleClass() != SVC_PEDESTRIAN
    //             && !type->getParameter().wasSet(VTYPEPARS_VEHICLECLASS_SET)) {
    //         WRITE_WARNINGF(TL("Person '%' receives type '%' which implicitly uses unsuitable vClass '%'."), getID(), type->getID(), toString(type->getVehicleClass()));
    //     }
    //     myVType = type;
    // }
    #[allow(unused_variables)]
    pub fn replace_vehicle_type(&mut self, type_: Ptr<MSVehicleType>) {
        let oldVClass = self.myVType.get_vehicle_class();
        if self.myVType.is_vehicle_specific() {
            MSNet::get_instance().m().get_vehicle_control().remove_v_type(self.myVType);
        }
        if self.is_person() && type_.get_vehicle_class() != oldVClass && type_.get_vehicle_class() != SVC_PEDESTRIAN && !type_.get_parameter().was_set(VTYPEPARS_VEHICLECLASS_SET) {
            write_warningf!("Person '%' receives type '%' which implicitly uses unsuitable vClass '%'.", self.get_id().to_string(), type_.get_id().to_string(), utils::common::sumo_vehicle_class::get_vehicle_class_names(type_.get_vehicle_class(), false));
        }
        self.myVType = type_;
    }

    /// C++: `MSVehicleType & MSTransportable::getSingularType()`
    /// Replaces the current vehicle type with a new one used by this vehicle only
    // MSVehicleType&
    // MSTransportable::getSingularType() {
    //     if (myVType->isVehicleSpecific()) {
    //         return *const_cast<MSVehicleType*>(myVType);
    //     }
    //     MSVehicleType* type = myVType->buildSingularType(myVType->getID() + "@" + getID());
    //     replaceVehicleType(type);
    //     return *type;
    // }
    #[allow(unused_variables)]
    pub fn get_singular_type(&mut self) -> &mut MSVehicleType {
        if self.myVType.is_vehicle_specific() {
            return self.myVType.get_mut().unwrap();
        }
        let typ = self.myVType.build_singular_type(&format!("{}@{}", self.myVType.get_id(), self.get_id()));
        self.replace_vehicle_type(typ);
        typ.get_mut().unwrap()
    }

    /// C++: `PositionVector MSTransportable::getBoundingBox() const` [const]
    /// return the bounding box of the person
    // PositionVector
    // MSTransportable::getBoundingBox() const {
    //     PositionVector centerLine;
    //     const Position p = getPosition();
    //     const double angle = getAngle();
    //     const double length = getVehicleType().getLength();
    //     const Position back = p + Position(-cos(angle) * length, -sin(angle) * length);
    //     centerLine.push_back(p);
    //     centerLine.push_back(back);
    //     centerLine.move2side(0.5 * getVehicleType().getWidth());
    //     PositionVector result = centerLine;
    //     centerLine.move2side(-getVehicleType().getWidth());
    //     result.append(centerLine.reverse(), POSITION_EPS);
    //     //std::cout << " transp=" << getID() << " p=" << p << " angle=" << GeomHelper::naviDegree(angle) << " back=" << back << " result=" << result << "\n";
    //     return result;
    // }
    #[allow(unused_variables)]
    pub fn get_bounding_box(&self) -> PositionVector {
        let mut centerLine = PositionVector(Vec::new());
        let p = self.get_position_2();
        let angle = self.get_angle();
        let length = self.get_vehicle_type().get_length();
        let back = p + Position::new(-angle.cos() * length, -angle.sin() * length);
        centerLine.push(p);
        centerLine.push(back);
        centerLine.move2side(0.5 * self.get_vehicle_type().get_width(), 100.);
        let mut result = centerLine.clone();
        centerLine.move2side(-self.get_vehicle_type().get_width(), 100.);
        result.append(&centerLine.reverse(), POSITION_EPS);
        result
    }

    /// C++: `bool MSTransportable::hasArrived() const` [const, virtual]
    /// return whether the person has reached the end of its plan
    // bool
    // MSTransportable::hasArrived() const {
    //     return myStep == myPlan->end();
    // }
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        self.myStep == self.myPlan.len()
    }

    /// C++: `bool MSTransportable::hasDeparted() const` [const]
    /// return whether the transportable has started its plan
    // bool
    // MSTransportable::hasDeparted() const {
    //     return myPlan->size() > 0 && (myPlan->front()->getDeparted() >= 0 || myStep > myPlan->begin());
    // }
    #[allow(unused_variables)]
    pub fn has_departed(&self) -> bool {
        self.myPlan.len() > 0 && (self.myPlan[0].get_departed() >= 0 || self.myStep > 0)
    }

    /// C++: `void MSTransportable::rerouteParkingArea(MSStoppingPlace *, MSStoppingPlace *)`
    /// adapt plan when the vehicle reroutes and now stops at replacement instead of orig
    // void
    // MSTransportable::rerouteParkingArea(MSStoppingPlace* orig, MSStoppingPlace* replacement) {
    //     // check whether the transportable was riding to the orignal stop
    //     // @note: parkingArea can currently not be set as myDestinationStop so we
    //     // check for stops on the edge instead
    // #ifdef DEBUG_PARKING
    //     std::cout << SIMTIME << " person=" << getID() << " rerouteParkingArea orig=" << orig->getID() << " replacement=" << replacement->getID() << "\n";
    // #endif
    //     assert(getCurrentStageType() == MSStageType::DRIVING);
    //     if (!myAmPerson) {
    //         WRITE_WARNING(TL("parkingAreaReroute not supported for containers"));
    //         return;
    //     }
    //     if (getDestination() == &orig->getLane().getEdge()) {
    //         MSStageDriving* const stage = dynamic_cast<MSStageDriving*>(*myStep);
    //         assert(stage != 0);
    //         assert(stage->getVehicle() != 0);
    //         // adapt plan
    //         stage->setDestination(&replacement->getLane().getEdge(), replacement);
    //         stage->setArrivalPos((replacement->getBeginLanePosition() + replacement->getEndLanePosition()) / 2);
    // #ifdef DEBUG_PARKING
    //         std::cout << " set ride destination\n";
    // #endif
    //         if (myStep + 1 == myPlan->end()) {
    //             return;
    //         }
    //         // if the next step is a walk, adapt the route
    //         MSStage* nextStage = *(myStep + 1);
    //         if (nextStage->getStageType() == MSStageType::TRIP) {
    //             dynamic_cast<MSStageTrip*>(nextStage)->setOrigin(stage->getDestination(), stage->getDestinationStop(), stage->getArrivalPos());
    // #ifdef DEBUG_PARKING
    //             std::cout << " set subsequent trip origin\n";
    // #endif
    //         } else if (nextStage->getStageType() == MSStageType::WALKING) {
    // #ifdef DEBUG_PARKING
    //             std::cout << " replace subsequent walk with a trip\n";
    // #endif
    //             MSStageTrip* newStage = new MSStageTrip(stage->getDestination(), nullptr, nextStage->getDestination(),
    //                                                     nextStage->getDestinationStop(), -1, 0, "", -1, 1, getID(), 0, true, nextStage->getArrivalPos());
    //             removeStage(1);
    //             appendStage(newStage, 1);
    //         } else if (nextStage->getStageType() == MSStageType::WAITING) {
    // #ifdef DEBUG_PARKING
    //             std::cout << " add subsequent walk to reach stop\n";
    //             std::cout << "   arrivalPos=" << nextStage->getArrivalPos() << "\n";
    // #endif
    //             MSStageTrip* newStage = new MSStageTrip(stage->getDestination(), nullptr, nextStage->getDestination(),
    //                                                     nextStage->getDestinationStop(), -1, 0, "", -1, 1, getID(), 0, true, nextStage->getArrivalPos());
    //             appendStage(newStage, 1);
    //         }
    //         // if the plan contains another ride with the same vehicle from the same
    //         // parking area, adapt the preceeding walk to end at the replacement
    //         for (auto it = myStep + 2; it != myPlan->end(); it++) {
    //             MSStage* const futureStage = *it;
    //             MSStage* const prevStage = *(it - 1);
    //             if (futureStage->getStageType() == MSStageType::DRIVING) {
    //                 MSStageDriving* const ds = static_cast<MSStageDriving*>(futureStage);
    //                 // ride origin is set implicitly from the walk destination
    //                 ds->setOrigin(nullptr, nullptr, -1);
    //                 if (ds->getLines() == stage->getLines()
    //                         && prevStage->getDestination() == &orig->getLane().getEdge()) {
    //                     if (prevStage->getStageType() == MSStageType::TRIP) {
    //                         dynamic_cast<MSStageTrip*>(prevStage)->setDestination(stage->getDestination(), replacement);
    // #ifdef DEBUG_PARKING
    //                         std::cout << " replace later trip before ride (" << (it - myPlan->begin()) << ")\n";
    // #endif
    //                     } else if (prevStage->getStageType() == MSStageType::WALKING) {
    // #ifdef DEBUG_PARKING
    //                         std::cout << " replace later walk before ride (" << (it - myPlan->begin()) << ")\n";
    // #endif
    //                         MSStageTrip* newStage = new MSStageTrip(prevStage->getFromEdge(), nullptr, stage->getDestination(),
    //                                                                 replacement, -1, 0, "", -1, 1, getID(), 0, true, stage->getArrivalPos());
    //                         int prevStageRelIndex = (int)(it - 1 - myStep);
    //                         removeStage(prevStageRelIndex);
    //                         appendStage(newStage, prevStageRelIndex);
    //                     }
    //                     break;
    //                 }
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn reroute_parking_area(&mut self, orig: Ptr<MSStoppingPlace>, replacement: Ptr<MSStoppingPlace>) {
        assert!(self.get_current_stage_type() == MSStageType::DRIVING);
        if !self.myAmPerson {
            write_warning("parkingAreaReroute not supported for containers");
            return;
        }
        let origEdge = Ptr::from_mut(orig.get_lane().get_edge());
        if self.get_destination() == origEdge {
            let stage: Ptr<MSStageDriving> = self.get_current_stage().cast::<MSStageDriving>();
            assert!(!stage.get_vehicle().is_null());
            // adapt plan
            let replEdge = Ptr::from_mut(replacement.get_lane().get_edge());
            stage.m().set_destination(replEdge, replacement);
            stage.m().set_arrival_pos((replacement.get_begin_lane_position() + replacement.get_end_lane_position()) / 2.);
            if self.myStep + 1 == self.myPlan.len() {
                return;
            }
            // if the next step is a walk, adapt the route
            let nextStage = self.myPlan[self.myStep + 1];
            if nextStage.get_stage_type() == MSStageType::TRIP {
                nextStage.cast::<MSStageTrip>().m().set_origin(stage.get_destination(), stage.get_destination_stop(), stage.get_arrival_pos());
            } else if nextStage.get_stage_type() == MSStageType::WALKING {
                let newStage = MSStageTrip::new(stage.get_destination(), Ptr::null(), nextStage.get_destination(), nextStage.get_destination_stop(), -1, 0, "", -1., 1., &self.get_id().to_string(), 0., true, nextStage.get_arrival_pos());
                let newPtr = Ptr::from_mut(Box::leak(Box::new(newStage))).cast::<MSStage>();
                self.remove_stage(1, true);
                self.append_stage(newPtr, 1);
            } else if nextStage.get_stage_type() == MSStageType::WAITING {
                let newStage = MSStageTrip::new(stage.get_destination(), Ptr::null(), nextStage.get_destination(), nextStage.get_destination_stop(), -1, 0, "", -1., 1., &self.get_id().to_string(), 0., true, nextStage.get_arrival_pos());
                let newPtr = Ptr::from_mut(Box::leak(Box::new(newStage))).cast::<MSStage>();
                self.append_stage(newPtr, 1);
            }
            // if the plan contains another ride with the same vehicle from the same
            // parking area, adapt the preceeding walk to end at the replacement
            let mut it = self.myStep + 2;
            while it != self.myPlan.len() {
                let futureStage = self.myPlan[it];
                let prevStage = self.myPlan[it - 1];
                if futureStage.get_stage_type() == MSStageType::DRIVING {
                    let ds: Ptr<MSStageDriving> = futureStage.cast::<MSStageDriving>();
                    // ride origin is set implicitly from the walk destination
                    ds.m().set_origin(Ptr::null(), Ptr::null(), -1.);
                    if *ds.get_lines() == *stage.get_lines() && prevStage.get_destination() == origEdge {
                        if prevStage.get_stage_type() == MSStageType::TRIP {
                            prevStage.cast::<MSStageTrip>().m().set_destination(stage.get_destination(), replacement);
                        } else if prevStage.get_stage_type() == MSStageType::WALKING {
                            let newStage = MSStageTrip::new(prevStage.get_from_edge(), Ptr::null(), stage.get_destination(), replacement, -1, 0, "", -1., 1., &self.get_id().to_string(), 0., true, stage.get_arrival_pos());
                            let newPtr = Ptr::from_mut(Box::leak(Box::new(newStage))).cast::<MSStage>();
                            let prevStageRelIndex = (it as i32) - 1 - (self.myStep as i32);
                            self.remove_stage(prevStageRelIndex, true);
                            self.append_stage(newPtr, prevStageRelIndex);
                        }
                        break;
                    }
                }
                it += 1;
            }
        }
    }

    /// C++: `MSDevice * MSTransportable::getDevice(const std::type_info &) const` [const, virtual]
    /// Returns a device of the given type if it exists or nullptr if not
    // MSDevice*
    // MSTransportable::getDevice(const std::type_info& type) const {
    //     for (MSTransportableDevice* const dev : myDevices) {
    //         if (typeid(*dev) == type) {
    //             return dev;
    //         }
    //     }
    //     return nullptr;
    // }
    #[allow(unused_variables)]
    pub fn get_device(&self, kind: &str) -> Ptr<MSDevice> {
        for dev in self.myDevices.iter() {
            if dev.base.device_name() == kind {
                // MSDevice is the first base of MSTransportableDevice, so the upcast keeps the address
                return dev.cast::<MSDevice>();
            }
        }
        Ptr::null()
    }

    /// C++: `void MSTransportable::setJunctionModelParameter(const std::string &, const std::string &)`
    /// set individual junction model paramete (not type related)
    // void
    // MSTransportable::setJunctionModelParameter(const std::string& key, const std::string& value) {
    //     if (key == toString(SUMO_ATTR_JM_IGNORE_IDS) || key == toString(SUMO_ATTR_JM_IGNORE_TYPES)) {
    //         getParameter().parametersSet |= VEHPARS_JUNCTIONMODEL_PARAMS_SET;
    //         const_cast<SUMOVehicleParameter&>(getParameter()).setParameter(key, value);
    //         // checked in MSLink::ignoreFoe
    //     } else {
    //         throw InvalidArgument(getObjectType() + " '" + getID() + "' does not support junctionModel parameter '" + key + "'");
    //     }
    // }
    #[allow(unused_variables)]
    pub fn set_junction_model_parameter(&mut self, key: &str, value: &str) {
        if key == utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_JM_IGNORE_IDS) || key == utils::xml::sumo_xml_definitions::Attrs.get_string(SUMO_ATTR_JM_IGNORE_TYPES) {
            self.myParameter.m().parametersSet |= VEHPARS_JUNCTIONMODEL_PARAMS_SET;
            self.myParameter.m().base.set_parameter(key, value);
        } else {
            panic!("{}", InvalidArgument(format!("{} '{}' does not support junctionModel parameter '{}'", self.get_object_type(), self.get_id(), key)).0);
        }
    }

    /// C++: `const std::vector<MSTransportableDevice *> & MSTransportable::getDevices() const` [const]
    /// Returns this vehicle's devices
    // inline const std::vector<MSTransportableDevice*>& getDevices() const {
    //         return myDevices;
    //     }
    #[allow(unused_variables)]
    pub fn get_devices(&self) -> &Vec<Ptr<MSTransportableDevice>> {
        &self.myDevices
    }

    /// C++: `bool MSTransportable::hasInfluencer() const` [const, virtual]
    // virtual bool hasInfluencer() const override {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        false
    }

    /// C++: `bool MSTransportable::isSelected() const` [const, virtual]
    /// whether this transportable is selected in the GUI
    // virtual bool isSelected() const override {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        false
    }

    /// C++: `int MSTransportable::getRoutingMode() const` [const, virtual]
    /// return routing mode (configures router choice but also handling of transient permission changes)
    // int
    // MSTransportable::getRoutingMode() const {
    //     /// @todo: allow configuring routing mode
    //     return libsumo::ROUTING_MODE_DEFAULT;
    // }
    #[allow(unused_variables)]
    pub fn get_routing_mode(&self) -> i32 {
        ROUTING_MODE_DEFAULT
    }

    /// C++: `void MSTransportable::setRoutingMode(int)` [virtual]
    /// Sets routing behavior
    // void
    // MSTransportable::setRoutingMode(int value) {
    //     /// @todo: allow configuring routing mode
    //     UNUSED_PARAMETER(value);
    // }
    #[allow(unused_variables)]
    pub fn set_routing_mode(&mut self, value: i32) {
        // the routing mode is not configurable yet, value is ignored
    }

    /// C++: `void MSTransportable::saveState(OutputDevice &)`
    /// Saves the current state into the given stream
    // void
    // MSTransportable::saveState(OutputDevice& out) {
    //     // this saves lots of departParameters which are only needed for transportables that did not yet depart
    //     // the parameters may hold the name of a vTypeDistribution but we are interested in the actual type
    //     const SUMOTime desiredDepart = myParameter->depart;
    //     if (myPlan->front()->getDeparted() >= 0) {
    //         // this is only relevant in the context of delayed departure (max-num-persons)
    //         const_cast<SUMOVehicleParameter*>(myParameter)->depart = myPlan->front()->getDeparted();
    //     }
    //     myParameter->write(out, OptionsCont::getOptions(), myAmPerson ? SUMO_TAG_PERSON : SUMO_TAG_CONTAINER, getVehicleType().getID());
    //     const_cast<SUMOVehicleParameter*>(myParameter)->depart = desiredDepart;
    //     if (!myParameter->wasSet(VEHPARS_SPEEDFACTOR_SET)) {
    //         out.setPrecision(MAX2(gPrecisionRandom, gPrecision));
    //         out.writeAttr(SUMO_ATTR_SPEEDFACTOR, getChosenSpeedFactor());
    //         out.setPrecision(gPrecision);
    //     }
    //     int stepIdx = (int)(myStep - myPlan->begin());
    //     for (auto it = myPlan->begin(); it != myStep; ++it) {
    //         const MSStageType st = (*it)->getStageType();
    //         if (st == MSStageType::TRIP || st == MSStageType::ACCESS) {
    //             stepIdx--;
    //         }
    //     }
    //     const bool isAccess = (*myStep)->getStageType() == MSStageType::ACCESS;
    //     std::ostringstream state;
    //     state << myParameter->parametersSet << " " << (isAccess ? toString(stepIdx - 0.5, 1) : toString(stepIdx));
    //     (*myStep)->saveState(state, this);
    //     out.writeAttr(SUMO_ATTR_STATE, state.str());
    //     const MSStage* previous = nullptr;
    //     for (const MSStage* const stage : *myPlan) {
    //         const bool routeLength = stage->getDeparted() >= 0 && stage->getStageType() == MSStageType::DRIVING;
    //         const bool withTiming = stage->getDeparted() >= 0;
    //         stage->routeOutput(myAmPerson, out, routeLength, previous, withTiming, true);
    //         previous = stage;
    //     }
    //     out.closeTag();
    // }
    #[allow(unused_variables)]
    pub fn save_state(&mut self, out: &mut OutputDevice) {
        // this saves lots of departParameters which are only needed for transportables that did not yet depart
        // the parameters may hold the name of a vTypeDistribution but we are interested in the actual type
        let desiredDepart = self.myParameter.depart;
        if self.myPlan[0].get_departed() >= 0 {
            // this is only relevant in the context of delayed departure (max-num-persons)
            self.myParameter.m().depart = self.myPlan[0].get_departed();
        }
        let typeID = self.get_vehicle_type().get_id().to_string();
        let altTag = if self.myAmPerson { SUMO_TAG_PERSON } else { SUMO_TAG_CONTAINER };
        let myParameter = self.myParameter;
        utils::options::options_cont::with_options(|oc| myParameter.write(out, oc, altTag, &typeID));
        self.myParameter.m().depart = desiredDepart;
        if !self.myParameter.was_set(VEHPARS_SPEEDFACTOR_SET) {
            out.set_precision(std::cmp::max(utils::common::std_defs::g_precision_random(), utils::common::std_defs::g_precision()));
            out.write_attr(SUMO_ATTR_SPEEDFACTOR, &self.get_chosen_speed_factor());
            out.set_precision(utils::common::std_defs::g_precision());
        }
        let mut stepIdx = self.myStep as i32;
        for it in 0..self.myStep {
            let st = self.myPlan[it].get_stage_type();
            if st == MSStageType::TRIP || st == MSStageType::ACCESS {
                stepIdx -= 1;
            }
        }
        let isAccess = self.get_current_stage().get_stage_type() == MSStageType::ACCESS;
        let stepText = if isAccess { utils::common::to_string::f64_to_string_prec(stepIdx as f64 - 0.5, 1) } else { stepIdx.to_string() };
        let mut state = format!("{} {}", self.myParameter.parametersSet, stepText);
        self.get_current_stage().m().save_state(&mut state, Ptr::from_mut(self));
        out.write_attr(SUMO_ATTR_STATE, &state);
        let mut previous: Ptr<MSStage> = Ptr::null();
        for stage in self.myPlan.iter() {
            let routeLength = stage.get_departed() >= 0 && stage.get_stage_type() == MSStageType::DRIVING;
            let withTiming = stage.get_departed() >= 0;
            stage.route_output(self.myAmPerson, out, routeLength, previous, withTiming, true);
            previous = *stage;
        }
        out.close_tag("");
    }

    /// C++: `void MSTransportable::loadState(const std::string &)`
    /// Reconstructs the current state
    // void
    // MSTransportable::loadState(const std::string& state) {
    //     std::istringstream iss(state);
    //     double step;
    //     iss >> myParameter->parametersSet >> step;
    //     myPlan->front()->setDeparted(myParameter->depart);
    //     if (step != floor(step)) {
    //         // we are in an access stage
    //         int priorIndex = (int)(step - 0.5);
    //         MSStage* prior = *(myPlan->begin() + priorIndex);
    //         myStep = myPlan->begin() + priorIndex + 1;
    //         bool waitAtStop = prior->getDestinationStop() != nullptr
    //                           && &prior->getDestinationStop()->getLane().getEdge() != prior->getDestination();
    //         checkAccess(prior, waitAtStop);
    //         //std::cout << " step=" << step << " i=" << getCurrentStageIndex() << " stage=" << getStageSummary(true) << "\n";
    //     } else {
    //         myStep = myPlan->begin() + (int)step;
    //     }
    //     (*myStep)->loadState(this, iss);
    //     for (int i = 1; i < step; i++) {
    //         if ((*myPlan)[i]->getStageType() == MSStageType::DRIVING) {
    //             dynamic_cast<MSStageDriving*>((*myPlan)[i])->setWaitingSince((*myPlan)[i - 1]->getArrived());
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn load_state(&mut self, state: &str) {
        let mut iss = state.split_whitespace();
        let parametersSet: i64 = iss.next().and_then(|s| s.parse().ok()).unwrap_or_default();
        self.myParameter.m().parametersSet = parametersSet;
        let step: f64 = iss.next().and_then(|s| s.parse().ok()).unwrap_or_default();
        self.myPlan[0].m().set_departed(self.myParameter.depart);
        if step != step.floor() {
            // we are in an access stage
            let priorIndex = (step - 0.5) as i32;
            let prior = self.myPlan[priorIndex as usize];
            self.myStep = priorIndex as usize + 1;
            let waitAtStop = !prior.get_destination_stop().is_null() && Ptr::from_mut(prior.get_destination_stop().get_lane().get_edge()) != prior.get_destination();
            self.check_access(prior, waitAtStop);
        } else {
            self.myStep = step as usize;
        }
        self.get_current_stage().m().load_state(Ptr::from_mut(self), &mut iss);
        let mut i: usize = 1;
        while (i as f64) < step {
            if self.myPlan[i].get_stage_type() == MSStageType::DRIVING {
                let prevArrived = self.myPlan[i - 1].get_arrived();
                self.myPlan[i].cast::<MSStageDriving>().m().set_waiting_since(prevArrived);
            }
            i += 1;
        }
    }

    /// C++: `void MSTransportable::MSTransportable(const MSTransportable &)` [private]
    /// Invalidated copy constructor.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(arg0: &MSTransportable) -> Self {
        panic!("MSTransportable copy constructor is invalidated in C++ (private)");
    }

    /// C++: `MSTransportable & MSTransportable::operator=(const MSTransportable &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &MSTransportable) -> &mut MSTransportable {
        panic!("MSTransportable assignment operator is invalidated in C++ (private)");
    }

}

