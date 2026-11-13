//! Port of utils/vehicle/SUMOTrafficObject.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use utils::tlf;

/// C++ typedef `std::shared_ptr<const MSRoute> ConstMSRoutePtr`
pub type ConstMSRoutePtr = Rc<MSRoute>;

/// C++ typedef `long long SUMOTrafficObject::NumericalID`
pub type SUMOTrafficObject_NumericalID = i64;

/// Virtual dispatch table for the polymorphic methods of SUMOTrafficObject.
/// Each subclass constructor installs its own table in `vt`; the base methods forward through it.
pub struct SUMOTrafficObjectVt {
    pub is_vehicle: fn(&SUMOTrafficObject) -> bool,
    pub is_person: fn(&SUMOTrafficObject) -> bool,
    pub is_container: fn(&SUMOTrafficObject) -> bool,
    pub get_parameter: for<'a> fn(&'a SUMOTrafficObject) -> &'a SUMOVehicleParameter,
    pub get_edge: fn(&SUMOTrafficObject) -> Ptr<MSEdge>,
    pub get_lane: fn(&SUMOTrafficObject) -> Ptr<MSLane>,
    pub get_speed: fn(&SUMOTrafficObject) -> f64,
    pub get_position: fn(&SUMOTrafficObject, f64) -> Position,
    pub get_numerical_id: fn(&SUMOTrafficObject) -> SUMOTrafficObject_NumericalID,
    pub get_random_seed: fn(&SUMOTrafficObject) -> i64,
    pub get_vehicle_type: for<'a> fn(&'a SUMOTrafficObject) -> &'a MSVehicleType,
    pub get_v_type_parameter: for<'a> fn(&'a SUMOTrafficObject) -> &'a SUMOVTypeParameter,
    pub replace_vehicle_type: fn(&mut SUMOTrafficObject, Ptr<MSVehicleType>),
    pub get_rng: fn(&SUMOTrafficObject) -> Ptr<SumoRNG>,
    pub get_rng_index: fn(&SUMOTrafficObject) -> i32,
    pub is_stopped: fn(&SUMOTrafficObject) -> bool,
    pub get_current_edge: fn(&SUMOTrafficObject) -> Ptr<MSEdge>,
    pub get_next_edge_ptr: fn(&SUMOTrafficObject) -> Ptr<MSEdge>,
    pub get_upcoming_edge_i_ds: fn(&SUMOTrafficObject) -> BTreeSet<SUMOTrafficObject_NumericalID>,
    pub get_back_lane: fn(&SUMOTrafficObject) -> Ptr<MSLane>,
    pub get_route_position: fn(&SUMOTrafficObject) -> i32,
    pub get_reroute_destination: fn(&SUMOTrafficObject) -> Ptr<MSEdge>,
    pub replace_route: fn(&mut SUMOTrafficObject, ConstMSRoutePtr, &str, bool, i32, bool, bool, Ptr<String>) -> bool,
    pub get_slope: fn(&SUMOTrafficObject) -> f64,
    pub get_chosen_speed_factor: fn(&SUMOTrafficObject) -> f64,
    pub get_v_class: fn(&SUMOTrafficObject) -> SUMOVehicleClass,
    pub get_routing_mode: fn(&SUMOTrafficObject) -> i32,
    pub set_routing_mode: fn(&mut SUMOTrafficObject, i32),
    pub get_max_speed: fn(&SUMOTrafficObject) -> f64,
    pub get_waiting_time: fn(&SUMOTrafficObject, bool) -> SUMOTime,
    pub get_previous_speed: fn(&SUMOTrafficObject) -> f64,
    pub get_acceleration: fn(&SUMOTrafficObject) -> f64,
    pub get_position_on_lane: fn(&SUMOTrafficObject) -> f64,
    pub get_back_position_on_lane: fn(&SUMOTrafficObject, Ptr<MSLane>) -> f64,
    pub get_angle: fn(&SUMOTrafficObject) -> f64,
    pub has_arrived: fn(&SUMOTrafficObject) -> bool,
    pub has_influencer: fn(&SUMOTrafficObject) -> bool,
    pub is_selected: fn(&SUMOTrafficObject) -> bool,
    pub get_device: fn(&SUMOTrafficObject, &Unmapped) -> Ptr<MSDevice>,
}

fn default_is_false(_: &SUMOTrafficObject) -> bool {
    false
}

pub(crate) fn default_get_parameter(_: &SUMOTrafficObject) -> &SUMOVehicleParameter {
    panic!("SUMOTrafficObject::getParameter is pure virtual")
}
pub(crate) fn default_get_edge(_: &SUMOTrafficObject) -> Ptr<MSEdge> {
    panic!("SUMOTrafficObject::getEdge is pure virtual")
}
pub(crate) fn default_get_lane(_: &SUMOTrafficObject) -> Ptr<MSLane> {
    panic!("SUMOTrafficObject::getLane is pure virtual")
}
pub(crate) fn default_get_speed(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getSpeed is pure virtual")
}
pub(crate) fn default_get_position(_: &SUMOTrafficObject, _: f64) -> Position {
    panic!("SUMOTrafficObject::getPosition is pure virtual")
}

pub(crate) fn default_get_numerical_id(_: &SUMOTrafficObject) -> SUMOTrafficObject_NumericalID {
    panic!("SUMOTrafficObject::getNumericalID is pure virtual")
}
pub(crate) fn default_get_random_seed(_: &SUMOTrafficObject) -> i64 {
    panic!("SUMOTrafficObject::getRandomSeed is pure virtual")
}
pub(crate) fn default_get_vehicle_type(_: &SUMOTrafficObject) -> &MSVehicleType {
    panic!("SUMOTrafficObject::getVehicleType is pure virtual")
}
pub(crate) fn default_get_v_type_parameter(_: &SUMOTrafficObject) -> &SUMOVTypeParameter {
    panic!("SUMOTrafficObject::getVTypeParameter is pure virtual")
}
pub(crate) fn default_replace_vehicle_type(_: &mut SUMOTrafficObject, _: Ptr<MSVehicleType>) {
    panic!("SUMOTrafficObject::replaceVehicleType is pure virtual")
}
pub(crate) fn default_get_rng(_: &SUMOTrafficObject) -> Ptr<SumoRNG> {
    panic!("SUMOTrafficObject::getRNG is pure virtual")
}
pub(crate) fn default_get_rng_index(_: &SUMOTrafficObject) -> i32 {
    panic!("SUMOTrafficObject::getRNGIndex is pure virtual")
}
pub(crate) fn default_is_stopped(_: &SUMOTrafficObject) -> bool {
    panic!("SUMOTrafficObject::isStopped is pure virtual")
}
pub(crate) fn default_get_current_edge(_: &SUMOTrafficObject) -> Ptr<MSEdge> {
    panic!("SUMOTrafficObject::getCurrentEdge is pure virtual")
}
pub(crate) fn default_get_next_edge_ptr(_: &SUMOTrafficObject) -> Ptr<MSEdge> {
    panic!("SUMOTrafficObject::getNextEdgePtr is pure virtual")
}
pub(crate) fn default_get_upcoming_edge_i_ds(_: &SUMOTrafficObject) -> BTreeSet<SUMOTrafficObject_NumericalID> {
    panic!("SUMOTrafficObject::getUpcomingEdgeIDs is pure virtual")
}
pub(crate) fn default_get_back_lane(_: &SUMOTrafficObject) -> Ptr<MSLane> {
    panic!("SUMOTrafficObject::getBackLane is pure virtual")
}
pub(crate) fn default_get_route_position(_: &SUMOTrafficObject) -> i32 {
    panic!("SUMOTrafficObject::getRoutePosition is pure virtual")
}
pub(crate) fn default_get_reroute_destination(_: &SUMOTrafficObject) -> Ptr<MSEdge> {
    panic!("SUMOTrafficObject::getRerouteDestination is pure virtual")
}
pub(crate) fn default_replace_route(_: &mut SUMOTrafficObject, _: ConstMSRoutePtr, _: &str, _: bool, _: i32, _: bool, _: bool, _: Ptr<String>) -> bool {
    panic!("SUMOTrafficObject::replaceRoute is pure virtual")
}
pub(crate) fn default_get_slope(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getSlope is pure virtual")
}
pub(crate) fn default_get_chosen_speed_factor(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getChosenSpeedFactor is pure virtual")
}
pub(crate) fn default_get_v_class(_: &SUMOTrafficObject) -> SUMOVehicleClass {
    panic!("SUMOTrafficObject::getVClass is pure virtual")
}
pub(crate) fn default_get_routing_mode(_: &SUMOTrafficObject) -> i32 {
    panic!("SUMOTrafficObject::getRoutingMode is pure virtual")
}
pub(crate) fn default_set_routing_mode(_: &mut SUMOTrafficObject, _: i32) {
    panic!("SUMOTrafficObject::setRoutingMode is pure virtual")
}
pub(crate) fn default_get_max_speed(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getMaxSpeed is pure virtual")
}
pub(crate) fn default_get_waiting_time(_: &SUMOTrafficObject, _: bool) -> SUMOTime {
    panic!("SUMOTrafficObject::getWaitingTime is pure virtual")
}
pub(crate) fn default_get_previous_speed(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getPreviousSpeed is pure virtual")
}
pub(crate) fn default_get_acceleration(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getAcceleration is pure virtual")
}
pub(crate) fn default_get_position_on_lane(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getPositionOnLane is pure virtual")
}
pub(crate) fn default_get_back_position_on_lane(_: &SUMOTrafficObject, _: Ptr<MSLane>) -> f64 {
    panic!("SUMOTrafficObject::getBackPositionOnLane is pure virtual")
}
pub(crate) fn default_get_angle(_: &SUMOTrafficObject) -> f64 {
    panic!("SUMOTrafficObject::getAngle is pure virtual")
}
pub(crate) fn default_has_arrived(_: &SUMOTrafficObject) -> bool {
    panic!("SUMOTrafficObject::hasArrived is pure virtual")
}
pub(crate) fn default_has_influencer(_: &SUMOTrafficObject) -> bool {
    panic!("SUMOTrafficObject::hasInfluencer is pure virtual")
}
pub(crate) fn default_is_selected(_: &SUMOTrafficObject) -> bool {
    panic!("SUMOTrafficObject::isSelected is pure virtual")
}
pub(crate) fn default_get_device(_: &SUMOTrafficObject, _: &Unmapped) -> Ptr<MSDevice> {
    panic!("SUMOTrafficObject::getDevice is pure virtual")
}

static DEFAULT_SUMO_TRAFFIC_OBJECT_VT: SUMOTrafficObjectVt = SUMOTrafficObjectVt {
    is_vehicle: default_is_false,
    is_person: default_is_false,
    is_container: default_is_false,
    get_parameter: default_get_parameter,
    get_edge: default_get_edge,
    get_numerical_id: default_get_numerical_id,
    get_v_class: default_get_v_class,
    get_lane: default_get_lane,
    get_speed: default_get_speed,
    get_position: default_get_position,
    get_random_seed: default_get_random_seed,
    get_vehicle_type: default_get_vehicle_type,
    get_v_type_parameter: default_get_v_type_parameter,
    replace_vehicle_type: default_replace_vehicle_type,
    get_rng: default_get_rng,
    get_rng_index: default_get_rng_index,
    is_stopped: default_is_stopped,
    get_current_edge: default_get_current_edge,
    get_next_edge_ptr: default_get_next_edge_ptr,
    get_upcoming_edge_i_ds: default_get_upcoming_edge_i_ds,
    get_back_lane: default_get_back_lane,
    get_route_position: default_get_route_position,
    get_reroute_destination: default_get_reroute_destination,
    replace_route: default_replace_route,
    get_slope: default_get_slope,
    get_chosen_speed_factor: default_get_chosen_speed_factor,
    get_routing_mode: default_get_routing_mode,
    set_routing_mode: default_set_routing_mode,
    get_max_speed: default_get_max_speed,
    get_waiting_time: default_get_waiting_time,
    get_previous_speed: default_get_previous_speed,
    get_acceleration: default_get_acceleration,
    get_position_on_lane: default_get_position_on_lane,
    get_back_position_on_lane: default_get_back_position_on_lane,
    get_angle: default_get_angle,
    has_arrived: default_has_arrived,
    has_influencer: default_has_influencer,
    is_selected: default_is_selected,
    get_device: default_get_device,
};

/// Representation of a vehicle, person, or container
/// C++ class `SUMOTrafficObject` (utils/vehicle/SUMOTrafficObject.h)
#[repr(C)]
pub struct SUMOTrafficObject {
    /// C++ base class `Named`
    pub base: Named,
    /// Virtual dispatch table; must stay after `base`
    pub vt: &'static SUMOTrafficObjectVt,
}

impl std::ops::Deref for SUMOTrafficObject {
    type Target = Named;
    fn deref(&self) -> &Named {
        &self.base
    }
}
impl std::ops::DerefMut for SUMOTrafficObject {
    fn deref_mut(&mut self) -> &mut Named {
        &mut self.base
    }
}

impl SUMOTrafficObject {
    /// C++: `void SUMOTrafficObject::SUMOTrafficObject(const std::string &)`
    /// Constructor
    // SUMOTrafficObject(const std::string& id) : Named(id) {}
    #[allow(unused_variables)]
    pub fn new(id: &str) -> Self {
        SUMOTrafficObject { base: Named::new(id), vt: &DEFAULT_SUMO_TRAFFIC_OBJECT_VT }
    }

    /// C++: `void SUMOTrafficObject::~SUMOTrafficObject() noexcept` [virtual]
    /// Destructor
    // virtual ~SUMOTrafficObject() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `bool SUMOTrafficObject::isVehicle() const` [const, virtual]
    /// Whether it is a vehicle
    // virtual bool isVehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle(&self) -> bool {
        (self.vt.is_vehicle)(self)
    }

    /// C++: `bool SUMOTrafficObject::isPerson() const` [const, virtual]
    /// Whether it is a person
    // virtual bool isPerson() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_person(&self) -> bool {
        (self.vt.is_person)(self)
    }

    /// C++: `bool SUMOTrafficObject::isContainer() const` [const, virtual]
    /// Whether it is a container
    // virtual bool isContainer() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_container(&self) -> bool {
        (self.vt.is_container)(self)
    }

    /// C++: `NumericalID SUMOTrafficObject::getNumericalID() const` [const, virtual, pure virtual]
    /// return the numerical ID which is only for internal usage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        (self.vt.get_numerical_id)(self)
    }

    /// C++: `long long SUMOTrafficObject::getRandomSeed() const` [const, virtual, pure virtual]
    /// return an object-specific random constant
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        (self.vt.get_random_seed)(self)
    }

    /// C++: `const MSVehicleType & SUMOTrafficObject::getVehicleType() const` [const, virtual, pure virtual]
    /// Returns the object's "vehicle" type
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> &MSVehicleType {
        (self.vt.get_vehicle_type)(self)
    }

    /// C++: `const SUMOVTypeParameter & SUMOTrafficObject::getVTypeParameter() const` [const, virtual, pure virtual]
    /// Returns the object's "vehicle" type parameter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        (self.vt.get_v_type_parameter)(self)
    }

    /// C++: `void SUMOTrafficObject::replaceVehicleType(const MSVehicleType *)` [virtual, pure virtual]
    /// Replaces the current vehicle type by the one given
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_vehicle_type(&mut self, type_: Ptr<MSVehicleType>) {
        let vt = self.vt;
                (vt.replace_vehicle_type)(self, type_)
    }

    /// C++: `const SUMOVehicleParameter & SUMOTrafficObject::getParameter() const` [const, virtual, pure virtual]
    /// Returns the vehicle's parameter (including departure definition)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        (self.vt.get_parameter)(self)
    }

    /// C++: `SumoRNG * SUMOTrafficObject::getRNG() const` [const, virtual, pure virtual]
    /// Returns the associated RNG for this object
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        (self.vt.get_rng)(self)
    }

    /// C++: `int SUMOTrafficObject::getRNGIndex() const` [const, virtual, pure virtual]
    /// Returns The index of the associated RNG
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        (self.vt.get_rng_index)(self)
    }

    /// C++: `bool SUMOTrafficObject::isStopped() const` [const, virtual, pure virtual]
    /// Returns whether the object is at a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        (self.vt.is_stopped)(self)
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getEdge() const` [const, virtual, pure virtual]
    /// Returns the (normal) route edge the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        (self.vt.get_edge)(self)
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getCurrentEdge() const` [const, virtual, pure virtual]
    /// Returns the edge (normal or internal) the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        (self.vt.get_current_edge)(self)
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getNextEdgePtr() const` [const, virtual, pure virtual]
    /// returns the next edge (possibly an internal edge)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        (self.vt.get_next_edge_ptr)(self)
    }

    /// C++: `const std::set<NumericalID> SUMOTrafficObject::getUpcomingEdgeIDs() const` [const, virtual, pure virtual]
    /// returns the numerical IDs of edges to be used (possibly of future stages)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_upcoming_edge_i_ds(&self) -> BTreeSet<SUMOTrafficObject_NumericalID> {
        (self.vt.get_upcoming_edge_i_ds)(self)
    }

    /// C++: `const MSLane * SUMOTrafficObject::getLane() const` [const, virtual, pure virtual]
    /// Returns the lane the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        (self.vt.get_lane)(self)
    }

    /// C++: `const MSLane * SUMOTrafficObject::getBackLane() const` [const, virtual, pure virtual]
    /// Returns the lane the where the rear of the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_back_lane(&self) -> Ptr<MSLane> {
        (self.vt.get_back_lane)(self)
    }

    /// C++: `int SUMOTrafficObject::getRoutePosition() const` [const, virtual, pure virtual]
    /// return index of edge within route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        (self.vt.get_route_position)(self)
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getRerouteDestination() const` [const, virtual, pure virtual]
    /// Returns the end point for reroutes (usually the last edge of the route)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_reroute_destination(&self) -> Ptr<MSEdge> {
        (self.vt.get_reroute_destination)(self)
    }

    /// C++: `bool SUMOTrafficObject::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual, pure virtual]
    /// Replaces the current route by the given one
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        let vt = self.vt;
                (vt.replace_route)(self, route, info, onInit, offset, addStops, removeStops, msgReturn)
    }

    /// C++: `double SUMOTrafficObject::getSlope() const` [const, virtual, pure virtual]
    /// Returns the slope of the road at object's position in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_slope(&self) -> f64 {
        (self.vt.get_slope)(self)
    }

    /// C++: `double SUMOTrafficObject::getChosenSpeedFactor() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        (self.vt.get_chosen_speed_factor)(self)
    }

    /// C++: `SUMOVehicleClass SUMOTrafficObject::getVClass() const` [const, virtual, pure virtual]
    /// Returns the object's access class
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        (self.vt.get_v_class)(self)
    }

    /// C++: `bool SUMOTrafficObject::ignoreTransientPermissions() const` [const, virtual]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // virtual bool ignoreTransientPermissions() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        false
    }

    /// C++: `int SUMOTrafficObject::getRoutingMode() const` [const, virtual, pure virtual]
    /// return routing mode (configures router choice but also handling of transient permission changes)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_routing_mode(&self) -> i32 {
        (self.vt.get_routing_mode)(self)
    }

    /// C++: `void SUMOTrafficObject::setRoutingMode(int)` [virtual, pure virtual]
    /// Sets routing behavior
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_routing_mode(&mut self, value: i32) {
        let vt = self.vt;
                (vt.set_routing_mode)(self, value)
    }

    /// C++: `double SUMOTrafficObject::getMaxSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's maximum speed (minimum of technical and desired maximum speed)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        (self.vt.get_max_speed)(self)
    }

    /// C++: `SUMOTime SUMOTrafficObject::getWaitingTime(const bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, accumulated: bool) -> SUMOTime {
        (self.vt.get_waiting_time)(self, accumulated)
    }

    /// C++: `double SUMOTrafficObject::getSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's current speed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        (self.vt.get_speed)(self)
    }

    /// C++: `double SUMOTrafficObject::getPreviousSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's previous speed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_previous_speed(&self) -> f64 {
        (self.vt.get_previous_speed)(self)
    }

    /// C++: `double SUMOTrafficObject::getAcceleration() const` [const, virtual, pure virtual]
    /// Returns the object's acceleration
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_acceleration(&self) -> f64 {
        (self.vt.get_acceleration)(self)
    }

    /// C++: `double SUMOTrafficObject::getPositionOnLane() const` [const, virtual, pure virtual]
    /// Get the object's position along the lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position_on_lane(&self) -> f64 {
        (self.vt.get_position_on_lane)(self)
    }

    /// C++: `double SUMOTrafficObject::getBackPositionOnLane(const MSLane *) const` [const, virtual, pure virtual]
    /// Get the object's back position along the given lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_back_position_on_lane(&self, lane: Ptr<MSLane>) -> f64 {
        (self.vt.get_back_position_on_lane)(self, lane)
    }

    /// C++: `Position SUMOTrafficObject::getPosition(const double) const` [const, virtual, pure virtual]
    /// Return current position (x/y, cartesian)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, offset: f64) -> Position {
        (self.vt.get_position)(self, offset)
    }

    /// C++: `double SUMOTrafficObject::getAngle() const` [const, virtual, pure virtual]
    /// Returns the object's angle in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        (self.vt.get_angle)(self)
    }

    /// C++: `bool SUMOTrafficObject::hasArrived() const` [const, virtual, pure virtual]
    /// Returns whether this object has arrived
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        (self.vt.has_arrived)(self)
    }

    /// C++: `bool SUMOTrafficObject::hasInfluencer() const` [const, virtual, pure virtual]
    /// whether the vehicle is individually influenced (via TraCI or special parameters)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        (self.vt.has_influencer)(self)
    }

    /// C++: `bool SUMOTrafficObject::isSelected() const` [const, virtual, pure virtual]
    /// whether this object is selected in the GUI
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        (self.vt.is_selected)(self)
    }

    /// C++: `MSDevice * SUMOTrafficObject::getDevice(const std::type_info &) const` [const, virtual, pure virtual]
    /// Returns a device of the given type if it exists or nullptr if not
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_device(&self, type_: &Unmapped /* std::type_info */) -> Ptr<MSDevice> {
        (self.vt.get_device)(self, type_)
    }

    /// C++: `std::string SUMOTrafficObject::getStringParam(const std::string &, const bool, const std::string &) const` [const]
    /// Retrieve a string parameter for the traffic object.
    // std::string
    // SUMOTrafficObject::getStringParam(const std::string& paramName, const bool required, const std::string& deflt) const {
    //     if (getParameter().hasParameter(paramName)) {
    //         return getParameter().getParameter(paramName, "");
    //     } else if (getVTypeParameter().hasParameter(paramName)) {
    //         return getVTypeParameter().getParameter(paramName, "");
    //     } else {
    //         const OptionsCont& oc = OptionsCont::getOptions();
    //         if (oc.exists(paramName) && oc.isSet(paramName)) {
    //             return oc.getValueString(paramName);
    //         } else {
    //             if (required) {
    //                 const std::string type = isVehicle() ? "vehicle" : (isPerson() ? "person" : "container");
    //                 throw ProcessError(TLF("Missing parameter '%' for % '%'.", paramName, type, getID()));
    //             } else {
    //                 if (oc.exists(paramName)) {
    //                     return oc.getValueString(paramName);
    //                 }
    //                 return deflt;
    //             }
    //         }
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_string_param(&self, paramName: &str, required: bool, deflt: &str) -> String {
        if self.get_parameter().has_parameter(paramName) {
            return self.get_parameter().get_parameter(paramName, "");
        } else if self.get_v_type_parameter().has_parameter(paramName) {
            return self.get_v_type_parameter().get_parameter(paramName, "");
        }
        if utils::options::options_cont::exists(paramName) && utils::options::options_cont::is_set(paramName) {
            return utils::options::options_cont::with_options(|oc| oc.get_value_string(paramName)).unwrap_or_else(|e| panic!("{e}"));
        }
        if required {
            let type_ = if self.is_vehicle() { "vehicle" } else if self.is_person() { "person" } else { "container" };
            // C++ throws ProcessError here; the signature has no Result, so panic with the same text
            panic!("{}", tlf!("Missing parameter '%' for % '%'.", paramName, type_, self.get_id()));
        }
        if utils::options::options_cont::exists(paramName) {
            return utils::options::options_cont::with_options(|oc| oc.get_value_string(paramName)).unwrap_or_else(|e| panic!("{e}"));
        }
        deflt.to_string()
    }

    /// C++: `double SUMOTrafficObject::getFloatParam(const std::string &, const bool, const double, bool) const` [const]
    /// Retrieve a floating point parameter for the traffic object.
    // double
    // SUMOTrafficObject::getFloatParam(const std::string& paramName, const bool required, const double deflt, bool checkDist) const {
    //     const std::string val = getStringParam(paramName, required, toString(deflt, 16));
    //     if (!checkDist) {
    //         try {
    //             return StringUtils::toDouble(val);
    //         } catch (NumberFormatException& e) {
    //             const std::string type = isVehicle() ? "vehicle" : (isPerson() ? "person" : "container");
    //             WRITE_ERRORF(TL("Invalid float value '%' for parameter '%' in % '%' (%)."), val, paramName, type, getID(), e.what());
    //             return deflt;
    //         }
    //     }
    //     try {
    //         Distribution_Parameterized dist(val);
    //         const std::string& error = dist.isValid();
    //         if (error != "") {
    //             throw ProcessError(error);
    //         }
    //         return dist.sample();
    //     } catch (const ProcessError& e) {
    //         const std::string type = isVehicle() ? "vehicle" : (isPerson() ? "person" : "container");
    //         WRITE_ERRORF(TL("Invalid distribution / float value '%' for parameter '%' in % '%' (%)."), val, paramName, type, getID(), e.what());
    //         return deflt;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_float_param(&self, paramName: &str, required: bool, deflt: f64, checkDist: bool) -> f64 {
        let val = self.get_string_param(paramName, required, &utils::common::to_string::f64_to_string_prec(deflt, 16));
        if !checkDist {
            return match utils::common::string_utils::to_double(&val) {
                Ok(v) => v,
                Err(e) if e.is_format() => {
                    let type_ = if self.is_vehicle() { "vehicle" } else if self.is_person() { "person" } else { "container" };
                    write_errorf!("Invalid float value '%' for parameter '%' in % '%' (%).", val, paramName, type_, self.get_id(), e.to_string());
                    deflt
                }
                Err(e) => panic!("{e}"),
            };
        }
        let parsed = DistributionParameterized::from_description(&val).and_then(|dist| {
            let error = dist.is_valid();
            if !error.is_empty() {
                return Err(ProcessError::new(error));
            }
            dist.sample(None)
        });
        match parsed {
            Ok(v) => v,
            Err(e) => {
                let type_ = if self.is_vehicle() { "vehicle" } else if self.is_person() { "person" } else { "container" };
                write_errorf!("Invalid distribution / float value '%' for parameter '%' in % '%' (%).", val, paramName, type_, self.get_id(), e.to_string());
                deflt
            }
        }
    }

    /// C++: `bool SUMOTrafficObject::getBoolParam(const std::string &, const bool, const bool) const` [const]
    /// Retrieve a boolean parameter for the traffic object.
    // bool
    // SUMOTrafficObject::getBoolParam(const std::string& paramName, const bool required, const bool deflt) const {
    //     const std::string val = getStringParam(paramName, required, toString(deflt));
    //     try {
    //         return StringUtils::toBool(val);
    //     } catch (const ProcessError&) {
    //         const std::string type = isVehicle() ? "vehicle" : (isPerson() ? "person" : "container");
    //         WRITE_ERRORF(TL("Invalid boolean value '%' for parameter '%' in % '%'."), val, paramName, type, getID());
    //         return deflt;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_bool_param(&self, paramName: &str, required: bool, deflt: bool) -> bool {
        let val = self.get_string_param(paramName, required, &utils::common::to_string::to_string(&deflt));
        match utils::common::string_utils::to_bool(&val) {
            Ok(v) => v,
            Err(_) => {
                let type_ = if self.is_vehicle() { "vehicle" } else if self.is_person() { "person" } else { "container" };
                write_errorf!("Invalid boolean value '%' for parameter '%' in % '%'.", val, paramName, type_, self.get_id());
                deflt
            }
        }
    }

    /// C++: `SUMOTime SUMOTrafficObject::getTimeParam(const std::string &, const bool, const SUMOTime) const` [const]
    /// Retrieve a time parameter for the traffic object.
    // SUMOTime
    // SUMOTrafficObject::getTimeParam(const std::string& paramName, const bool required, const SUMOTime deflt) const {
    //     const std::string val = getStringParam(paramName, required, time2string(deflt));
    //     try {
    //         return string2time(val);
    //     } catch (const ProcessError&) {
    //         const std::string type = isVehicle() ? "vehicle" : (isPerson() ? "person" : "container");
    //         WRITE_ERRORF(TL("Invalid time value '%' for parameter '%' in % '%'."), val, paramName, type, getID());
    //         return deflt;
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_time_param(&self, paramName: &str, required: bool, deflt: SUMOTime) -> SUMOTime {
        let val = self.get_string_param(paramName, required, &utils::common::time::time2string(deflt));
        match utils::common::time::string2time(&val) {
            Ok(v) => v,
            Err(_) => {
                let type_ = if self.is_vehicle() { "vehicle" } else if self.is_person() { "person" } else { "container" };
                write_errorf!("Invalid time value '%' for parameter '%' in % '%'.", val, paramName, type_, self.get_id());
                deflt
            }
        }
    }

    /// C++: `std::string SUMOTrafficObject::getCategory() const` [const]
    /// return description for suitable for warnings and messages
    // std::string
    // SUMOTrafficObject::getCategory() const {
    //     if (isPerson()) {
    //         return "Person";
    //     } else if (isContainer()) {
    //         return "Container";
    //     } else {
    //         return "Vehicle";
    //     }
    // }
    #[allow(unused_variables)]
    pub fn get_category(&self) -> String {
        if self.is_person() {
            "Person".to_string()
        } else if self.is_container() {
            "Container".to_string()
        } else {
            "Vehicle".to_string()
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_table_is_all_false() {
        let obj = SUMOTrafficObject::new("bare");
        let p = Ptr::new(&obj);
        assert!(!p.is_vehicle());
        assert!(!p.is_person());
        assert!(!p.is_container());
    }
}

