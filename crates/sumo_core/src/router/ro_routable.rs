//! Port of router/RORoutable.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `std::vector<const ROEdge *> ConstROEdgeVector`
pub type ConstROEdgeVector = Vec<Ptr<ROEdge>>;

/// C++ typedef `IntermodalRouter<ROEdge, ROLane, RONode, ROVehicle> ROIntermodalRouter`
pub type ROIntermodalRouter = IntermodalRouter<ROEdge, ROLane, RONode, ROVehicle>;

/// C++ typedef `RouterProvider<ROEdge, ROLane, RONode, ROVehicle> RORouterProvider`
pub type RORouterProvider = RouterProvider<ROEdge, ROLane, RONode, ROVehicle>;

/// A routable thing such as a vehicle or person
/// C++ class `RORoutable` (router/RORoutable.h)
pub struct RORoutable {
    /// The vehicle's parameter
    pub myParameter: SUMOVehicleParameter,
    /// The type of the vehicle
    pub myType: Ptr<SUMOVTypeParameter>,
    /// object-specific random constant
    pub myRandomSeed: i64,
    /// Whether the last routing was successful
    pub myRoutingSuccess: bool,
}

impl RORoutable {
    /// C++: `void RORoutable::RORoutable(const SUMOVehicleParameter &, const SUMOVTypeParameter *)`
    /// Constructor
    // RORoutable(const SUMOVehicleParameter& pars, const SUMOVTypeParameter* type) :
    //         myParameter(pars),
    //         myType(type),
    //         myRandomSeed(RandHelper::murmur3_32(pars.id, RandHelper::getSeed())),
    //         myRoutingSuccess(false) {}
    #[allow(unused_variables)]
    pub fn new(pars: &SUMOVehicleParameter, type_: Ptr<SUMOVTypeParameter>) -> Self {
        RORoutable {
            myParameter: pars.clone(),
            myType: type_,
            myRandomSeed: utils::common::rand_helper::murmur3_32(&pars.id, utils::common::rand_helper::with_default(|r| r.orig_seed)) as i64,
            myRoutingSuccess: false,
        }
    }

    /// C++: `void RORoutable::~RORoutable() noexcept` [virtual]
    /// Destructor
    // virtual ~RORoutable() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `const SUMOVehicleParameter & RORoutable::getParameter() const` [const]
    /// Returns the definition of the vehicle / person parameter
    // inline const SUMOVehicleParameter& getParameter() const {
    //         return myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter(&self) -> &SUMOVehicleParameter {
        &self.myParameter
    }

    /// C++: `SUMOVehicleParameter & RORoutable::getParameterMutable()`
    // inline SUMOVehicleParameter& getParameterMutable() {
    //         return myParameter;
    //     }
    #[allow(unused_variables)]
    pub fn get_parameter_mutable(&mut self) -> &mut SUMOVehicleParameter {
        &mut self.myParameter
    }

    /// C++: `const SUMOVTypeParameter * RORoutable::getType() const` [const]
    /// Returns the type of the routable
    // inline const SUMOVTypeParameter* getType() const {
    //         return myType;
    //     }
    #[allow(unused_variables)]
    pub fn get_type(&self) -> Ptr<SUMOVTypeParameter> {
        self.myType
    }

    /// C++: `const std::string & RORoutable::getID() const` [const]
    /// Returns the id of the routable
    // inline const std::string& getID() const {
    //         return myParameter.id;
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> &str {
        &self.myParameter.id
    }

    /// C++: `long long RORoutable::getRandomSeed() const` [const]
    /// return vehicle-specific random number
    // long long int getRandomSeed() const {
    //         return myRandomSeed;
    //     }
    #[allow(unused_variables)]
    pub fn get_random_seed(&self) -> i64 {
        self.myRandomSeed
    }

    /// C++: `double RORoutable::getChosenSpeedFactor() const` [const]
    /// Returns an upper bound for the speed factor of this vehicle
    // inline double getChosenSpeedFactor() const {
    //         return getParameter().wasSet(VEHPARS_SPEEDFACTOR_SET) ? getParameter().speedFactor :  getType()->speedFactor.getParameter(0);
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        // C++ VEHPARS_SPEEDFACTOR_SET (SUMOVehicleParameter.h), not ported as a named constant yet
        const VEHPARS_SPEEDFACTOR_SET: i64 = 2 << 24;
        if self.get_parameter().was_set(VEHPARS_SPEEDFACTOR_SET) {
            self.get_parameter().speedFactor
        } else {
            self.get_type().speedFactor.get_parameter(0)
        }
    }

    /// C++: `SUMOTime RORoutable::getDepart() const` [const]
    /// Returns the time the vehicle starts at, -1 for triggered vehicles
    // inline SUMOTime getDepart() const {
    //         return myParameter.depart;
    //     }
    #[allow(unused_variables)]
    pub fn get_depart(&self) -> SUMOTime {
        self.myParameter.depart
    }

    /// C++: `void RORoutable::setDepart(SUMOTime)`
    /// update depart time (for triggered persons)
    // inline void setDepart(SUMOTime t) {
    //         myParameter.depart = t;
    //     }
    #[allow(unused_variables)]
    pub fn set_depart(&mut self, t: SUMOTime) {
        self.myParameter.depart = t;
    }

    /// C++: `SUMOVehicleClass RORoutable::getVClass() const` [const]
    // inline SUMOVehicleClass getVClass() const {
    //         return getType() != 0 ? getType()->vehicleClass : SVC_IGNORING;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        if !self.myType.is_null() { self.myType.vehicleClass } else { SVC_IGNORING }
    }

    /// C++: `bool RORoutable::ignoreTransientPermissions() const` [const]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // bool ignoreTransientPermissions() const {
    //         return false;
    //     }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        false
    }

    /// C++: `double RORoutable::getMaxSpeed() const` [const]
    /// Returns the vehicle's maximum speed
    // inline double getMaxSpeed() const {
    //         return MIN2(getType()->maxSpeed,
    //                     getType()->desiredMaxSpeed * getChosenSpeedFactor());
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        let a = self.get_type().maxSpeed;
        let b = self.get_type().desiredMaxSpeed * self.get_chosen_speed_factor();
        if a < b { a } else { b }
    }

    /// C++: `const ROEdge * RORoutable::getDepartEdge() const` [const, virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn get_depart_edge(&self) -> Ptr<ROEdge> {
        // C++ pure virtual: only subclasses define it
        panic!("RORoutable::getDepartEdge is pure virtual; subclasses must implement it")
    }

    /// C++: `bool RORoutable::isPublicTransport() const` [const]
    // inline bool isPublicTransport() const {
    //         return myParameter.line != "";
    //     }
    #[allow(unused_variables)]
    pub fn is_public_transport(&self) -> bool {
        !self.myParameter.line.is_empty()
    }

    /// C++: `bool RORoutable::isPartOfFlow() const` [const]
    // inline bool isPartOfFlow() const {
    //         return myParameter.repetitionNumber >= 0;
    //     }
    #[allow(unused_variables)]
    pub fn is_part_of_flow(&self) -> bool {
        self.myParameter.repetitionNumber >= 0
    }

    /// C++: `void RORoutable::computeRoute(const RORouterProvider &, const bool, MsgHandler *)` [virtual, pure virtual]
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn compute_route(&mut self, provider: &RORouterProvider, removeLoops: bool, errorHandler: Ptr<MsgHandler>) {
        // C++ pure virtual: only subclasses define it
        panic!("RORoutable::computeRoute is pure virtual; subclasses must implement it")
    }

    /// C++: `void RORoutable::write(OutputDevice *, OutputDevice *const, OutputDevice *const, OptionsCont &, int) const` [const]
    /// Saves the routable including the vehicle type (if it was not saved before).
    // void write(OutputDevice* os, OutputDevice* const altos,
    //                OutputDevice* const typeos, OptionsCont& options, int quota) const {
    //         for (int i = 0; i < quota; i++) {
    //             if (os != nullptr) {
    //                 if (altos == nullptr && typeos == nullptr) {
    //                     saveAsXML(*os, os, false, options, i);
    //                 } else {
    //                     saveAsXML(*os, typeos, false, options, i);
    //                 }
    //             }
    //             if (altos != nullptr) {
    //                 saveAsXML(*altos, typeos, true, options, i);
    //             }
    //         }
    //     }
    #[allow(unused_variables)]
    pub fn write(&self, os: Ptr<OutputDevice>, altos: Ptr<OutputDevice>, typeos: Ptr<OutputDevice>, options: &mut OptionsCont, quota: i32) {
        for i in 0..quota {
            if !os.is_null() {
                if altos.is_null() && typeos.is_null() {
                    self.save_as_xml(os.m(), os, false, options, i);
                } else {
                    self.save_as_xml(os.m(), typeos, false, options, i);
                }
            }
            if !altos.is_null() {
                self.save_as_xml(altos.m(), typeos, true, options, i);
            }
        }
    }

    /// C++: `bool RORoutable::getRoutingSuccess() const` [const]
    // inline bool getRoutingSuccess() const {
    //         return myRoutingSuccess;
    //     }
    #[allow(unused_variables)]
    pub fn get_routing_success(&self) -> bool {
        self.myRoutingSuccess
    }

    /// C++: `void RORoutable::saveAsXML(OutputDevice &, OutputDevice *const, bool, OptionsCont &, int) const` [const, virtual, pure virtual, protected]
    /// Saves the complete routable description.
    // (pure virtual: no C++ body; subclasses implement it)
    #[allow(unused_variables)]
    pub fn save_as_xml(&self, os: &mut OutputDevice, typeos: Ptr<OutputDevice>, asAlternatives: bool, options: &mut OptionsCont, cloneIndex: i32) {
        // C++ pure virtual: only subclasses define it
        panic!("RORoutable::saveAsXML is pure virtual; subclasses must implement it")
    }

    /// C++: `void RORoutable::RORoutable(const RORoutable &)` [private]
    /// Invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(src: &RORoutable) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("RORoutable is not copyable (private copy constructor in C++)")
    }

    /// C++: `RORoutable & RORoutable::operator=(const RORoutable &)` [private]
    /// Invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, src: &RORoutable) -> &mut RORoutable {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("RORoutable is not assignable (private operator= in C++)")
    }

}

