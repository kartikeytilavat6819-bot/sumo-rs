//! Port of utils/router/AccessEdge.{h,cpp} — GENERATED SKELETON (tools/skeleton/gen.py). Replace every todo!() with a faithful translation of the C++ shown above it.
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_imports, unused_variables, unreachable_code, clippy::all)]
use crate::prelude::*;

/// C++ typedef `IntermodalEdge<E, L, N, V> AccessEdge::_IntermodalEdge`
pub type AccessEdge__IntermodalEdge = Unmapped /* IntermodalEdge<E, L, N, V> */;

/// the access edge connecting different modes that is given to the internal router (SUMOAbstractRouter)
/// C++ class `AccessEdge` (utils/router/AccessEdge.h)
pub struct AccessEdge<E, L, N, V> {
    /// C++ base class `IntermodalEdge<E, L, N, V>`
    pub base: IntermodalEdge<E, L, N, V>,
    /// travel time (alternative to length)
    pub myTraveltime: f64,
    /// only allow using this edge if the modeSet matches (i.e. entering a taxi)
    pub myModeRestrictions: SVCPermissions,
    /// only allow using this edge if the vehicle class matches (i.e. exiting a taxi)
    pub myVehicleRestriction: SVCPermissions,
    _p: std::marker::PhantomData<(E, L, N, V,)>,
}

impl<E, L, N, V> std::ops::Deref for AccessEdge<E, L, N, V> {
    type Target = IntermodalEdge<E, L, N, V>;
    fn deref(&self) -> &IntermodalEdge<E, L, N, V> {
        &self.base
    }
}
impl<E, L, N, V> std::ops::DerefMut for AccessEdge<E, L, N, V> {
    fn deref_mut(&mut self) -> &mut IntermodalEdge<E, L, N, V> {
        &mut self.base
    }
}

impl<E, L, N, V> AccessEdge<E, L, N, V> {
    /// C++: `void AccessEdge::AccessEdge<E, L, N, V>(int, const _IntermodalEdge *, const _IntermodalEdge *, const double, SVCPermissions, SVCPermissions, double)`
    // AccessEdge(int numericalID, const _IntermodalEdge* inEdge, const _IntermodalEdge* outEdge, const double length,
    //                SVCPermissions modeRestriction = SVC_IGNORING,
    //                SVCPermissions vehicleRestriction = SVC_IGNORING,
    //                double traveltime = -1) :
    //         _IntermodalEdge(inEdge->getID() + ":" + outEdge->getID() + (modeRestriction == SVC_TAXI ? ":taxi" : ""),
    //                         numericalID, outEdge->getEdge(), "!access", length > 0. ? length : NUMERICAL_EPS),
    //         myTraveltime(traveltime),
    //         myModeRestrictions(modeRestriction),
    //         myVehicleRestriction(vehicleRestriction)
    //     { }
    #[allow(unused_variables)]
    pub fn new(numericalID: i32, inEdge: Ptr<IntermodalEdge<E, L, N, V>>, outEdge: Ptr<IntermodalEdge<E, L, N, V>>, length: f64, modeRestriction: SVCPermissions, vehicleRestriction: SVCPermissions, traveltime: f64) -> Self {
        let id = format!("{}:{}{}", inEdge.get_id(), outEdge.get_id(), if modeRestriction == SVC_TAXI { ":taxi" } else { "" });
        Self {
            base: IntermodalEdge::new(id, numericalID, outEdge.get_edge(), "!access", if length > 0. { length } else { utils::geom::NUMERICAL_EPS }),
            myTraveltime: traveltime,
            myModeRestrictions: modeRestriction,
            myVehicleRestriction: vehicleRestriction,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `void AccessEdge::AccessEdge<E, L, N, V>(int, const std::string &, const E *, const double, SVCPermissions, SVCPermissions)`
    // AccessEdge(int numericalID, const std::string& id, const E* edge, const double length = 0,
    //                SVCPermissions modeRestriction = SVC_IGNORING,
    //                SVCPermissions vehicleRestriction = SVC_IGNORING) :
    //         _IntermodalEdge(id, numericalID, edge, "!access", length > 0. ? length : NUMERICAL_EPS),
    //         myTraveltime(-1),
    //         myModeRestrictions(modeRestriction),
    //         myVehicleRestriction(vehicleRestriction)
    //     { }
    #[allow(unused_variables)]
    pub fn new_2(numericalID: i32, id: &str, edge: Ptr<E>, length: f64, modeRestriction: SVCPermissions, vehicleRestriction: SVCPermissions) -> Self {
        Self {
            base: IntermodalEdge::new(id.to_string(), numericalID, edge, "!access", if length > 0. { length } else { utils::geom::NUMERICAL_EPS }),
            myTraveltime: -1.,
            myModeRestrictions: modeRestriction,
            myVehicleRestriction: vehicleRestriction,
            _p: std::marker::PhantomData,
        }
    }

    /// C++: `double AccessEdge::getTravelTime(const IntermodalTrip<E, N, V> *const, double) const` [const]
    // double getTravelTime(const IntermodalTrip<E, N, V>* const trip, double /* time */) const {
    //         return myTraveltime > 0 ? myTraveltime : this->getLength() / trip->speed;
    //     }
    #[allow(unused_variables)]
    pub fn get_travel_time(&self, trip: Ptr<IntermodalTrip<E, N, V>>, arg1: f64) -> f64 {
        if self.myTraveltime > 0. { self.myTraveltime } else { self.get_length() / trip.speed }
    }

    /// C++: `bool AccessEdge::prohibits(const IntermodalTrip<E, N, V> *const) const` [const]
    // bool prohibits(const IntermodalTrip<E, N, V>* const trip) const {
    //         return ((myModeRestrictions != SVC_IGNORING && (trip->modeSet & myModeRestrictions) == 0)
    //                 || (myVehicleRestriction != SVC_IGNORING &&
    //                     ((trip->vehicle == nullptr ? SVC_PEDESTRIAN : trip->vehicle->getVClass()) & myVehicleRestriction) == 0));
    //     }
    #[allow(unused_variables)]
    pub fn prohibits(&self, trip: Ptr<IntermodalTrip<E, N, V>>) -> bool {
        (self.myModeRestrictions != SVC_IGNORING && (trip.modeSet & self.myModeRestrictions) == 0)
            || (self.myVehicleRestriction != SVC_IGNORING && (trip.get_v_class() & self.myVehicleRestriction) == 0)
    }

    /// C++: `SVCPermissions AccessEdge::getVehicleRetriction() const` [const]
    // SVCPermissions getVehicleRetriction() const {
    //         return myVehicleRestriction;
    //     }
    #[allow(unused_variables)]
    pub fn get_vehicle_retriction(&self) -> SVCPermissions {
        self.myVehicleRestriction
    }

}

