//! Port of utils/router/IntermodalTrip.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;
use crate::utils::router::router_edge::RouterVehicle;

/// the "vehicle" type that is given to the internal router (SUMOAbstractRouter)
/// C++ class `IntermodalTrip` (utils/router/IntermodalTrip.h)
pub struct IntermodalTrip<E, N, V> {
    pub from: Ptr<E>,
    pub to: Ptr<E>,
    pub departPos: f64,
    pub arrivalPos: f64,
    pub speed: f64,
    pub departTime: SUMOTime,
    pub node: Ptr<N>,
    pub pars: Ptr<SUMOVTypeParameter>,
    pub vehicle: Ptr<V>,
    pub modeSet: SVCPermissions,
    pub calc: Ptr<EffortCalculator>,
    pub externalFactor: f64,
    _p: std::marker::PhantomData<(E, N, V,)>,
}

impl<E, N, V> RouterVehicle for IntermodalTrip<E, N, V> {
    fn v_class(&self) -> SUMOVehicleClass {
        self.get_v_class()
    }
}

impl<E, N, V> IntermodalTrip<E, N, V> {
    /// C++: `void IntermodalTrip::IntermodalTrip<E, N, V>(const E *, const E *, double, double, double, SUMOTime, const N *, const SUMOVTypeParameter &, const V *, const SVCPermissions, const EffortCalculator *const, const double)`
    // IntermodalTrip(const E* _from, const E* _to, double _departPos, double _arrivalPos,
    //                    double _speed, SUMOTime _departTime, const N* _node,
    //                    const SUMOVTypeParameter& _pars,
    //                    const V* _vehicle = 0, const SVCPermissions _modeSet = SVC_PEDESTRIAN,
    //                    const EffortCalculator* const _calc = nullptr, const double _externalFactor = 0.) :
    //         from(_from),
    //         to(_to),
    //         departPos(_departPos < 0 ? _from->getLength() + _departPos : _departPos),
    //         arrivalPos(_arrivalPos < 0 ? _to->getLength() + _arrivalPos : _arrivalPos),
    //         speed(_speed),
    //         departTime(_departTime),
    //         node(_node),
    //         pars(_pars),
    //         vehicle(_vehicle),
    //         modeSet(_modeSet),
    //         calc(_calc),
    //         externalFactor(_externalFactor) {
    //     }
    #[allow(unused_variables)]
    pub fn new(_from: Ptr<E>, _to: Ptr<E>, _departPos: f64, _arrivalPos: f64, _speed: f64, _departTime: SUMOTime, _node: Ptr<N>, _pars: &SUMOVTypeParameter, _vehicle: Ptr<V>, _modeSet: SVCPermissions, _calc: Ptr<EffortCalculator>, _externalFactor: f64) -> Self {
        // shortcut: needs router trait bounds (lead), E::getLength is not callable on generic E
        let departPos = if _departPos < 0. { panic!("shortcut: needs router trait bounds (lead), E::getLength") } else { _departPos };
        let arrivalPos = if _arrivalPos < 0. { panic!("shortcut: needs router trait bounds (lead), E::getLength") } else { _arrivalPos };
        Self {
            from: _from,
            to: _to,
            departPos,
            arrivalPos,
            speed: _speed,
            departTime: _departTime,
            node: _node,
            pars: Ptr::new(_pars),
            vehicle: _vehicle,
            modeSet: _modeSet,
            calc: _calc,
            externalFactor: _externalFactor,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `std::string IntermodalTrip::getID() const` [const]
    // std::string getID() const {
    //         return from->getID() + ":" + to->getID() + ":" + time2string(departTime);
    //     }
    #[allow(unused_variables)]
    pub fn get_id(&self) -> String {
        // shortcut: needs router trait bounds (lead), E::getID is not callable on generic E
        panic!("shortcut: needs router trait bounds (lead), E::getID")
    }

    /// C++: `SUMOVehicleClass IntermodalTrip::getVClass() const` [const]
    // inline SUMOVehicleClass getVClass() const {
    //         return vehicle != 0 ? vehicle->getVClass() : SVC_PEDESTRIAN;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_class(&self) -> SUMOVehicleClass {
        if self.vehicle.is_null() {
            SVC_PEDESTRIAN
        } else {
            // shortcut: needs router trait bounds (lead), V::getVClass is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::getVClass")
        }
    }

    /// C++: `const SUMOVTypeParameter & IntermodalTrip::getVTypeParameter() const` [const]
    // inline const SUMOVTypeParameter& getVTypeParameter() const {
    //         return vehicle != 0 ? vehicle->getVTypeParameter() : pars;
    //     }
    #[allow(unused_variables)]
    pub fn get_v_type_parameter(&self) -> &SUMOVTypeParameter {
        if self.vehicle.is_null() {
            &*self.pars
        } else {
            // shortcut: needs router trait bounds (lead), V::getVTypeParameter is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::getVTypeParameter")
        }
    }

    /// C++: `bool IntermodalTrip::ignoreTransientPermissions() const` [const]
    /// Returns whether this object is ignoring transient permission changes (during routing)
    // bool ignoreTransientPermissions() const {
    //         return vehicle != 0 ? vehicle->ignoreTransientPermissions() : false;
    //     }
    #[allow(unused_variables)]
    pub fn ignore_transient_permissions(&self) -> bool {
        if self.vehicle.is_null() {
            false
        } else {
            // shortcut: needs router trait bounds (lead), V::ignoreTransientPermissions is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::ignoreTransientPermissions")
        }
    }

    /// C++: `double IntermodalTrip::getLength() const` [const]
    // inline double getLength() const {
    //         // person length is arbitrary (only used in the context of rail-reversal validity
    //         return vehicle != 0 ? vehicle->getVehicleType().getLength() : 1;
    //     }
    #[allow(unused_variables)]
    pub fn get_length(&self) -> f64 {
        if self.vehicle.is_null() {
            1.
        } else {
            // shortcut: needs router trait bounds (lead), V::getVehicleType().getLength is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::getVehicleType")
        }
    }

    /// C++: `double IntermodalTrip::getMaxSpeed() const` [const]
    // inline double getMaxSpeed() const {
    //         return vehicle != nullptr ? vehicle->getMaxSpeed() : speed;
    //     }
    #[allow(unused_variables)]
    pub fn get_max_speed(&self) -> f64 {
        if self.vehicle.is_null() {
            self.speed
        } else {
            // shortcut: needs router trait bounds (lead), V::getMaxSpeed is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::getMaxSpeed")
        }
    }

    /// C++: `double IntermodalTrip::getChosenSpeedFactor() const` [const]
    // inline double getChosenSpeedFactor() const {
    //         return vehicle != nullptr ? vehicle->getChosenSpeedFactor() : 1.0;
    //     }
    #[allow(unused_variables)]
    pub fn get_chosen_speed_factor(&self) -> f64 {
        if self.vehicle.is_null() {
            1.0
        } else {
            // shortcut: needs router trait bounds (lead), V::getChosenSpeedFactor is not callable on generic V
            panic!("shortcut: needs router trait bounds (lead), V::getChosenSpeedFactor")
        }
    }

    /// C++: `IntermodalTrip<E, N, V> & IntermodalTrip::operator=(const IntermodalTrip<E, N, V> &)` [private]
    /// Invalidated assignment operator.
    // (no C++ body found: implicit/defaulted or defined elsewhere)
    #[allow(unused_variables)]
    pub fn assign_op(&mut self, arg0: &IntermodalTrip<E, N, V>) -> &mut IntermodalTrip<E, N, V> {
        panic!("IntermodalTrip assignment operator is private and invalidated in C++")
    }

}

