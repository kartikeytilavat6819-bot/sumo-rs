//! Port of netbuild/NBVehicle.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// A vehicle as used by router
/// C++ class `NBVehicle` (netbuild/NBVehicle.h)
pub struct NBVehicle {
    /// vehicle ID for error reporting
    pub myID: String,
    /// The vehicle class of the vehicle
    pub myVClass: SUMOVehicleClass,
    /// The length of the vehicle (for rail-routing)
    pub myLength: f64,
}

impl NBVehicle {
    /// C++: `void NBVehicle::NBVehicle(const std::string &, SUMOVehicleClass)`
    /// Constructor
    // NBVehicle(const std::string& id, SUMOVehicleClass vClass):
    //         myID(id), myVClass(vClass), myLength(getDefaultVehicleLength(vClass)) {}
    #[allow(unused_variables)]
    pub fn new(id: &str, vClass: SUMOVehicleClass) -> Self {
        Self {
            myID: id.to_string(),
            myVClass: vClass,
            myLength: get_default_vehicle_length(vClass),
        }
    }

    /// C++: `const std::string & NBVehicle::getID() const` [const]
    // const std::string& getID() const {
    //         return myID;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        &self.myID
    }

    /// C++: `SUMOVehicleClass NBVehicle::getVClass() const` [const]
    // SUMOVehicleClass getVClass() const {
    //         return myVClass;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        self.myVClass
    }

    /// C++: `bool NBVehicle::ignoreTransientPermissions() const` [const]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // bool ignoreTransientPermissions() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        false
    }

    /// C++: `double NBVehicle::getLength() const` [const]
    // double getLength() const {
    //         return myLength;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        self.myLength
    }

    /// C++: `void NBVehicle::~NBVehicle() noexcept` [virtual]
    /// Destructor
    // virtual ~NBVehicle() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {}

    /// C++: `void NBVehicle::NBVehicle(const NBVehicle &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &NBVehicle) -> Self {
        panic!("NBVehicle copy constructor is private and invalidated in C++")
    }

    /// C++: `NBVehicle & NBVehicle::operator=(const NBVehicle &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &NBVehicle) -> &mut NBVehicle {
        panic!("NBVehicle assignment operator is private and invalidated in C++")
    }

}

impl crate::utils::router::router_edge::RouterVehicle for NBVehicle {
    fn v_class(&self) -> SUMOVehicleClass {
        self.myVClass
    }
    fn get_id(&self) -> String {
        self.myID.clone()
    }
    // shortcut: C++ NBVehicle has no getMaxSpeed, only the af router's speed effort reads it, upgrade if NBVehicle is routed by that router
    fn get_max_speed(&self) -> f64 {
        panic!("shortcut: NBVehicle has no getMaxSpeed in C++")
    }
    // shortcut: C++ NBVehicle has no getChosenSpeedFactor, upgrade if NBVehicle is routed by the af router
    fn get_chosen_speed_factor(&self) -> f64 {
        panic!("shortcut: NBVehicle has no getChosenSpeedFactor in C++")
    }
    fn ignore_transient_permissions(&self) -> bool {
        self.ignore_transient_permissions()
    }
    fn length(&self) -> f64 {
        self.get_length()
    }
}

