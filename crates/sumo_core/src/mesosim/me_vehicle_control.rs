//! Port of mesosim/MEVehicleControl.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// The class responsible for building and deletion of vehicles (gui-version)
/// C++ class `MEVehicleControl` (mesosim/MEVehicleControl.h)
#[repr(C)]
pub struct MEVehicleControl {
    /// C++ base class `MSVehicleControl`
    pub base: MSVehicleControl,
}

impl std::ops::Deref for MEVehicleControl {
    type Target = MSVehicleControl;
    fn deref(&self) -> &MSVehicleControl {
        &self.base
    }
}
impl std::ops::DerefMut for MEVehicleControl {
    fn deref_mut(&mut self) -> &mut MSVehicleControl {
        &mut self.base
    }
}

use crate::microsim::ms_vehicle_control::MSVehicleControlVt;

/// Virtual method table of MEVehicleControl: overrides `buildVehicle`
static ME_VEHICLE_CONTROL_VT: MSVehicleControlVt = MSVehicleControlVt {
    build_vehicle: me_vehicle_control_build_vehicle,
};

fn me_vehicle_control_build_vehicle(vc: &mut MSVehicleControl, defs: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, ignoreStopErrors: bool, source: MSVehicleControl_VehicleDefinitionSource, addRouteStops: bool) -> Ptr<SUMOVehicle> {
    Ptr::from_mut(vc).cast::<MEVehicleControl>().m().build_vehicle(defs, route, type_, ignoreStopErrors, source, addRouteStops)
}

impl MEVehicleControl {
    /// C++: `void MEVehicleControl::MEVehicleControl()`
    /// Constructor
    // MEVehicleControl::MEVehicleControl()
    //     : MSVehicleControl() {}
    #[allow(unused_variables)]
    pub fn new() -> Self {
        let mut s = MEVehicleControl { base: MSVehicleControl::new() };
        s.base.vt = &ME_VEHICLE_CONTROL_VT;
        s
    }

    /// C++: `void MEVehicleControl::~MEVehicleControl() noexcept` [virtual]
    /// Destructor
    // MEVehicleControl::~MEVehicleControl() {}
    #[allow(unused_variables)]
    pub fn drop_(&mut self) {
        // C++ destructor body is empty
    }

    /// C++: `SUMOVehicle * MEVehicleControl::buildVehicle(SUMOVehicleParameter *, ConstMSRoutePtr, MSVehicleType *, const bool, const VehicleDefinitionSource, bool)` [virtual]
    /// Builds a vehicle, increases the number of built vehicles
    // SUMOVehicle*
    // MEVehicleControl::buildVehicle(SUMOVehicleParameter* defs,
    //                                ConstMSRoutePtr route, MSVehicleType* type,
    //                                const bool ignoreStopErrors, const VehicleDefinitionSource source,
    //                                bool addRouteStops) {
    //     const double speedFactor = type->computeChosenSpeedDeviation(defs->speedFactor, source == VehicleDefinitionSource::ROUTEFILE ? MSRouteHandler::getParsingRNG() : nullptr);
    //     MEVehicle* built = new MEVehicle(defs, route, type, speedFactor);
    //     initVehicle(built, ignoreStopErrors, addRouteStops, source);
    //     return built;
    // }
    #[allow(unused_variables)]
    pub fn build_vehicle(&mut self, defs: Ptr<SUMOVehicleParameter>, route: ConstMSRoutePtr, type_: Ptr<MSVehicleType>, ignoreStopErrors: bool, source: MSVehicleControl_VehicleDefinitionSource, addRouteStops: bool) -> Ptr<SUMOVehicle> {
        let speedFactor = type_.compute_chosen_speed_deviation(
            defs.speedFactor,
            if source == MSVehicleControl_VehicleDefinitionSource::ROUTEFILE { crate::utils::vehicle::sumo_route_handler::SUMORouteHandler::get_parsing_rng() } else { Ptr::null() },
            -1.,
        );
        let built: Ptr<MEVehicle> = Ptr::from_raw(Box::into_raw(Box::new(MEVehicle::new(defs, route, type_, speedFactor))));
        self.init_vehicle(Ptr::from_mut(&mut built.m().base), ignoreStopErrors, addRouteStops, source);
        Ptr::from_mut(&mut built.m().base.base)
    }

    /// C++: `void MEVehicleControl::MEVehicleControl(const MEVehicleControl &)` [private]
    /// invalidated copy constructor
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn new_2(s: &MEVehicleControl) -> Self {
        // C++ copy constructor is private and never defined: copying is disallowed
        panic!("MEVehicleControl is not copyable (private copy constructor in C++)")
    }

    /// C++: `MEVehicleControl & MEVehicleControl::operator=(const MEVehicleControl &)` [private]
    /// invalidated assignment operator
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, s: &MEVehicleControl) -> &mut MEVehicleControl {
        // C++ assignment operator is private and never defined: assignment is disallowed
        panic!("MEVehicleControl is not assignable (private operator= in C++)")
    }

}

