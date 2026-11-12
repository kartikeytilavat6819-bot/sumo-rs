//! Port of utils/vehicle/SUMOTrafficObject.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::shared_ptr<const MSRoute> ConstMSRoutePtr`
pub type ConstMSRoutePtr = Rc<MSRoute>;

/// C++ typedef `long long SUMOTrafficObject::NumericalID`
pub type SUMOTrafficObject_NumericalID = i64;

/// Representation of a vehicle, person, or container
/// C++ class `SUMOTrafficObject` (utils/vehicle/SUMOTrafficObject.h)
#[repr(C)]
pub struct SUMOTrafficObject {
    /// C++ base class `Named`
    pub base: Named,
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
        todo!("port SUMOTrafficObject::SUMOTrafficObject")
    }

    /// C++: `void SUMOTrafficObject::~SUMOTrafficObject() noexcept` [virtual]
    /// Destructor
    // virtual ~SUMOTrafficObject() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        todo!("port SUMOTrafficObject::~SUMOTrafficObject")
    }

    /// C++: `bool SUMOTrafficObject::isVehicle() const` [const, virtual]
    /// Whether it is a vehicle
    // virtual bool isVehicle() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_vehicle(&self) -> bool {
        todo!("port SUMOTrafficObject::isVehicle")
    }

    /// C++: `bool SUMOTrafficObject::isPerson() const` [const, virtual]
    /// Whether it is a person
    // virtual bool isPerson() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_person(&self) -> bool {
        todo!("port SUMOTrafficObject::isPerson")
    }

    /// C++: `bool SUMOTrafficObject::isContainer() const` [const, virtual]
    /// Whether it is a container
    // virtual bool isContainer() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn is_container(&self) -> bool {
        todo!("port SUMOTrafficObject::isContainer")
    }

    /// C++: `NumericalID SUMOTrafficObject::getNumericalID() const` [const, virtual, pure virtual]
    /// return the numerical ID which is only for internal usage
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_numerical_id(&self) -> SUMOTrafficObject_NumericalID {
        todo!("port SUMOTrafficObject::getNumericalID")
    }

    /// C++: `long long SUMOTrafficObject::getRandomSeed() const` [const, virtual, pure virtual]
    /// return an object-specific random constant
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        todo!("port SUMOTrafficObject::getRandomSeed")
    }

    /// C++: `const MSVehicleType & SUMOTrafficObject::getVehicleType() const` [const, virtual, pure virtual]
    /// Returns the object's "vehicle" type
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_vehicle_type(&self) -> &MSVehicleType {
        todo!("port SUMOTrafficObject::getVehicleType")
    }

    /// C++: `const SUMOVTypeParameter & SUMOTrafficObject::getVTypeParameter() const` [const, virtual, pure virtual]
    /// Returns the object's "vehicle" type parameter
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        todo!("port SUMOTrafficObject::getVTypeParameter")
    }

    /// C++: `void SUMOTrafficObject::replaceVehicleType(const MSVehicleType *)` [virtual, pure virtual]
    /// Replaces the current vehicle type by the one given
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_vehicle_type(&mut self, type_: Ptr<MSVehicleType>) {
        todo!("port SUMOTrafficObject::replaceVehicleType")
    }

    /// C++: `const SUMOVehicleParameter & SUMOTrafficObject::getParameter() const` [const, virtual, pure virtual]
    /// Returns the vehicle's parameter (including departure definition)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        todo!("port SUMOTrafficObject::getParameter")
    }

    /// C++: `SumoRNG * SUMOTrafficObject::getRNG() const` [const, virtual, pure virtual]
    /// Returns the associated RNG for this object
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_rng(&self) -> Ptr<SumoRNG> {
        todo!("port SUMOTrafficObject::getRNG")
    }

    /// C++: `int SUMOTrafficObject::getRNGIndex() const` [const, virtual, pure virtual]
    /// Returns The index of the associated RNG
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_rng_index(&self) -> i32 {
        todo!("port SUMOTrafficObject::getRNGIndex")
    }

    /// C++: `bool SUMOTrafficObject::isStopped() const` [const, virtual, pure virtual]
    /// Returns whether the object is at a stop
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_stopped(&self) -> bool {
        todo!("port SUMOTrafficObject::isStopped")
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getEdge() const` [const, virtual, pure virtual]
    /// Returns the (normal) route edge the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_edge(&self) -> Ptr<MSEdge> {
        todo!("port SUMOTrafficObject::getEdge")
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getCurrentEdge() const` [const, virtual, pure virtual]
    /// Returns the edge (normal or internal) the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_current_edge(&self) -> Ptr<MSEdge> {
        todo!("port SUMOTrafficObject::getCurrentEdge")
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getNextEdgePtr() const` [const, virtual, pure virtual]
    /// returns the next edge (possibly an internal edge)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_next_edge_ptr(&self) -> Ptr<MSEdge> {
        todo!("port SUMOTrafficObject::getNextEdgePtr")
    }

    /// C++: `const std::set<NumericalID> SUMOTrafficObject::getUpcomingEdgeIDs() const` [const, virtual, pure virtual]
    /// returns the numerical IDs of edges to be used (possibly of future stages)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_upcoming_edge_i_ds(&self) -> BTreeSet<SUMOTrafficObject_NumericalID> {
        todo!("port SUMOTrafficObject::getUpcomingEdgeIDs")
    }

    /// C++: `const MSLane * SUMOTrafficObject::getLane() const` [const, virtual, pure virtual]
    /// Returns the lane the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_lane(&self) -> Ptr<MSLane> {
        todo!("port SUMOTrafficObject::getLane")
    }

    /// C++: `const MSLane * SUMOTrafficObject::getBackLane() const` [const, virtual, pure virtual]
    /// Returns the lane the where the rear of the object is currently at
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_back_lane(&self) -> Ptr<MSLane> {
        todo!("port SUMOTrafficObject::getBackLane")
    }

    /// C++: `int SUMOTrafficObject::getRoutePosition() const` [const, virtual, pure virtual]
    /// return index of edge within route
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_route_position(&self) -> i32 {
        todo!("port SUMOTrafficObject::getRoutePosition")
    }

    /// C++: `const MSEdge * SUMOTrafficObject::getRerouteDestination() const` [const, virtual, pure virtual]
    /// Returns the end point for reroutes (usually the last edge of the route)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_reroute_destination(&self) -> Ptr<MSEdge> {
        todo!("port SUMOTrafficObject::getRerouteDestination")
    }

    /// C++: `bool SUMOTrafficObject::replaceRoute(ConstMSRoutePtr, const std::string &, bool, int, bool, bool, std::string *)` [virtual, pure virtual]
    /// Replaces the current route by the given one
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn replace_route(&mut self, route: ConstMSRoutePtr, info: &str, onInit: bool, offset: i32, addStops: bool, removeStops: bool, msgReturn: Ptr<String>) -> bool {
        todo!("port SUMOTrafficObject::replaceRoute")
    }

    /// C++: `double SUMOTrafficObject::getSlope() const` [const, virtual, pure virtual]
    /// Returns the slope of the road at object's position in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_slope(&self) -> f64 {
        todo!("port SUMOTrafficObject::getSlope")
    }

    /// C++: `double SUMOTrafficObject::getChosenSpeedFactor() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        todo!("port SUMOTrafficObject::getChosenSpeedFactor")
    }

    /// C++: `SUMOVehicleClass SUMOTrafficObject::getVClass() const` [const, virtual, pure virtual]
    /// Returns the object's access class
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        todo!("port SUMOTrafficObject::getVClass")
    }

    /// C++: `bool SUMOTrafficObject::ignoreTransientPermissions() const` [const, virtual]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // virtual bool ignoreTransientPermissions() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        todo!("port SUMOTrafficObject::ignoreTransientPermissions")
    }

    /// C++: `int SUMOTrafficObject::getRoutingMode() const` [const, virtual, pure virtual]
    /// return routing mode (configures router choice but also handling of transient permission changes)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_routing_mode(&self) -> i32 {
        todo!("port SUMOTrafficObject::getRoutingMode")
    }

    /// C++: `void SUMOTrafficObject::setRoutingMode(int)` [virtual, pure virtual]
    /// Sets routing behavior
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn set_routing_mode(&mut self, value: i32) {
        todo!("port SUMOTrafficObject::setRoutingMode")
    }

    /// C++: `double SUMOTrafficObject::getMaxSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's maximum speed (minimum of technical and desired maximum speed)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        todo!("port SUMOTrafficObject::getMaxSpeed")
    }

    /// C++: `SUMOTime SUMOTrafficObject::getWaitingTime(const bool) const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_waiting_time(&self, accumulated: bool) -> SUMOTime {
        todo!("port SUMOTrafficObject::getWaitingTime")
    }

    /// C++: `double SUMOTrafficObject::getSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's current speed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_speed(&self) -> f64 {
        todo!("port SUMOTrafficObject::getSpeed")
    }

    /// C++: `double SUMOTrafficObject::getPreviousSpeed() const` [const, virtual, pure virtual]
    /// Returns the object's previous speed
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_previous_speed(&self) -> f64 {
        todo!("port SUMOTrafficObject::getPreviousSpeed")
    }

    /// C++: `double SUMOTrafficObject::getAcceleration() const` [const, virtual, pure virtual]
    /// Returns the object's acceleration
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_acceleration(&self) -> f64 {
        todo!("port SUMOTrafficObject::getAcceleration")
    }

    /// C++: `double SUMOTrafficObject::getPositionOnLane() const` [const, virtual, pure virtual]
    /// Get the object's position along the lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position_on_lane(&self) -> f64 {
        todo!("port SUMOTrafficObject::getPositionOnLane")
    }

    /// C++: `double SUMOTrafficObject::getBackPositionOnLane(const MSLane *) const` [const, virtual, pure virtual]
    /// Get the object's back position along the given lane
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_back_position_on_lane(&self, lane: Ptr<MSLane>) -> f64 {
        todo!("port SUMOTrafficObject::getBackPositionOnLane")
    }

    /// C++: `Position SUMOTrafficObject::getPosition(const double) const` [const, virtual, pure virtual]
    /// Return current position (x/y, cartesian)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_position(&self, offset: f64) -> Position {
        todo!("port SUMOTrafficObject::getPosition")
    }

    /// C++: `double SUMOTrafficObject::getAngle() const` [const, virtual, pure virtual]
    /// Returns the object's angle in degrees
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_angle(&self) -> f64 {
        todo!("port SUMOTrafficObject::getAngle")
    }

    /// C++: `bool SUMOTrafficObject::hasArrived() const` [const, virtual, pure virtual]
    /// Returns whether this object has arrived
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_arrived(&self) -> bool {
        todo!("port SUMOTrafficObject::hasArrived")
    }

    /// C++: `bool SUMOTrafficObject::hasInfluencer() const` [const, virtual, pure virtual]
    /// whether the vehicle is individually influenced (via TraCI or special parameters)
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn has_influencer(&self) -> bool {
        todo!("port SUMOTrafficObject::hasInfluencer")
    }

    /// C++: `bool SUMOTrafficObject::isSelected() const` [const, virtual, pure virtual]
    /// whether this object is selected in the GUI
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn is_selected(&self) -> bool {
        todo!("port SUMOTrafficObject::isSelected")
    }

    /// C++: `MSDevice * SUMOTrafficObject::getDevice(const std::type_info &) const` [const, virtual, pure virtual]
    /// Returns a device of the given type if it exists or nullptr if not
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_device(&self, type_: &Unmapped /* std::type_info */) -> Ptr<MSDevice> {
        todo!("port SUMOTrafficObject::getDevice")
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
        todo!("port SUMOTrafficObject::getStringParam")
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
        todo!("port SUMOTrafficObject::getFloatParam")
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
        todo!("port SUMOTrafficObject::getBoolParam")
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
        todo!("port SUMOTrafficObject::getTimeParam")
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
        todo!("port SUMOTrafficObject::getCategory")
    }

}

